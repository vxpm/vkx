use core::ffi::CStr;
use std::collections::HashSet;

use vkx::Extendable;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

fn vertex_shader() -> Vec<u32> {
    let compiler = shaderc::Compiler::new().unwrap();
    let artifact = compiler
        .compile_into_spirv(
            include_str!("./triangle.vert"),
            shaderc::ShaderKind::Vertex,
            "triangle.vert",
            "main",
            Default::default(),
        )
        .unwrap();

    artifact.as_binary().to_vec()
}

fn fragment_shader() -> Vec<u32> {
    let compiler = shaderc::Compiler::new().unwrap();
    let artifact = compiler
        .compile_into_spirv(
            include_str!("./triangle.frag"),
            shaderc::ShaderKind::Fragment,
            "triangle.frag",
            "main",
            Default::default(),
        )
        .unwrap();

    artifact.as_binary().to_vec()
}

struct WindowState {
    window: Window,
    surface: vkx::SurfaceKHR,
    surface_caps: vkx::SurfaceCapabilitiesKHR,
}

struct SwapchainState {
    handle: vkx::SwapchainKHR,
    images: Vec<vkx::Image>,
    views: Vec<vkx::ImageView>,
}

struct App {
    instance: vkx::Instance,
    physical_device: vkx::PhysicalDevice,
    device: vkx::Device,
    queue: vkx::Queue,
    pipeline: vkx::Pipeline,
    cmdbuf: vkx::CommandBuffer,
    window: Option<WindowState>,
    swapchain: Option<SwapchainState>,
    swapchain_fence: vkx::Fence,
    swapchain_acquire_semaphore: vkx::Semaphore,
    swapchain_finished_semaphore: vkx::Semaphore,
    recreate_swapchain: bool,
}

impl App {
    fn new(event_loop: &EventLoop<()>) -> Self {
        // 01. creating an instance
        let app_info = vkx::ApplicationInfo {
            p_application_name: c"triangle".as_ptr(),
            application_version: vkx::Version::V1_0.get(),
            p_engine_name: c"vkx".as_ptr(),
            api_version: vkx::Version::V1_3.get(),
            ..Default::default()
        };

        // build a list of instance extensions
        let mut instance_extensions = vec![
            vkx::Extension::EXT_DebugUtils,
            vkx::Extension::KHR_PortabilityEnumeration,
        ];

        instance_extensions.extend(vkx::window::get_required_extensions(&event_loop).unwrap());

        // convert it into a vec of C string pointers
        let instance_extensions = vkx::Extension::to_ptrs(instance_extensions);

        // list of instance layers
        let instance_layers = &[c"VK_LAYER_KHRONOS_validation".as_ptr()];

        // create the instance
        let instance = vkx::Instance::create(
            &vkx::InstanceCreateInfo {
                p_application_info: &app_info,
                enabled_layer_count: instance_layers.len() as u32,
                pp_enabled_layer_names: instance_layers.as_ptr(),
                enabled_extension_count: instance_extensions.len() as u32,
                pp_enabled_extension_names: instance_extensions.as_ptr(),
                // this flag is needed to allow enumerating portability devices
                flags: vkx::InstanceCreateFlag::ENUMERATE_PORTABILITY_KHR.into(),
                ..Default::default()
            },
            None,
        )
        .unwrap();

        // 02. creating the logical device
        // enumerate the physical devices
        let physical_devices = unsafe { instance.enumerate_physical_devices().unwrap() };

        // list of device extensions we want
        let device_extensions = HashSet::from_iter([vkx::Extension::KHR_Swapchain]);

        // choose the physical device that fits best
        let (physical_device, properties, family_idx) =
            physical_devices
                .into_iter()
                .map(|dev| {
                    // get the device properties
                    let mut properties = Default::default();
                    unsafe { dev.get_properties(&mut properties) };

                    (dev, properties)
                })
                .filter(|(dev, properties)| {
                    let (extensions, result) = unsafe {
                        vkx::auto_count!(|count, vec| dev
                            .enumerate_device_extension_properties(None, &mut count, vec))
                    };

                    assert!(result.unwrap().is_success());
                    let extensions = vkx::Extension::from_ext_properties(extensions);

                    // we want a device that supports at least vulkan 1.3 and has all the device
                    // extensions we need
                    properties.api_version >= vkx::Version::V1_3.get()
                        && extensions.is_superset(&device_extensions)
                })
                .filter_map(|(dev, properties)| {
                    let (queue_families, _) = unsafe {
                        vkx::auto_count!(
                            |count, vec| dev.get_queue_family_properties_2(&mut count, vec)
                        )
                    };

                    queue_families
                        .iter()
                        .enumerate()
                        .position(|(idx, family)| {
                            // we need a queue that supports graphics
                            let is_graphics = family
                                .queue_family_properties
                                .queue_flags
                                .contains(vkx::QueueFlag::GRAPHICS);

                            // we also need it to support presentation
                            let supports_presentation = unsafe {
                                vkx::window::queue_family_supports_presentation(
                                    &dev, idx as u32, event_loop,
                                )
                                .unwrap()
                            };

                            is_graphics && supports_presentation
                        })
                        .map(|family_idx| (dev, properties, family_idx as u32))
                })
                .min_by_key(|(_, properties, _)| match properties.device_type {
                    vkx::PhysicalDeviceType::DISCRETE_GPU => 0,
                    vkx::PhysicalDeviceType::INTEGRATED_GPU => 1,
                    vkx::PhysicalDeviceType::VIRTUAL_GPU => 2,
                    vkx::PhysicalDeviceType::CPU => 3,
                    vkx::PhysicalDeviceType::OTHER => 4,
                    _ => 5,
                })
                .unwrap();

        let name =
            CStr::from_bytes_until_nul(zerocopy::transmute_ref!(&properties.device_name)).unwrap();

        println!("Device chosen: {}", name.to_string_lossy());

        // setup queue creation info
        let queue_priorities = [0.5];
        let queue_create_info = [vkx::DeviceQueueCreateInfo {
            queue_family_index: family_idx,
            queue_count: 1,
            p_queue_priorities: queue_priorities.as_ptr(),
            ..Default::default()
        }];

        // setup enabled features
        let enabled_features = vkx::PhysicalDeviceFeatures::default();
        let mut enabled_features_1_3 = vkx::PhysicalDeviceVulkan13Features {
            synchronization_2: true.into(),
            dynamic_rendering: true.into(),
            ..Default::default()
        };

        // setup creation info
        let device_extensions_names = vkx::Extension::to_ptrs(device_extensions);
        let mut device_create_info = vkx::DeviceCreateInfo {
            queue_create_info_count: 1,
            p_queue_create_infos: queue_create_info.as_ptr(),
            enabled_extension_count: device_extensions_names.len() as u32,
            pp_enabled_extension_names: device_extensions_names.as_ptr(),
            p_enabled_features: &enabled_features,
            ..Default::default()
        };

        device_create_info.push_next(&mut enabled_features_1_3);

        // create the device
        let device = physical_device
            .create_device(&device_create_info, None)
            .unwrap();

        // and get the queue
        let queue = unsafe { device.get_device_queue(family_idx, 0) };

        // 03. create objects used for rendering
        let pipeline = Self::create_pipeline(&device).unwrap();

        let mut command_pool = vkx::CommandPool::null();
        unsafe {
            device
                .create_command_pool(
                    &vkx::CommandPoolCreateInfo {
                        flags: vkx::CommandPoolCreateFlag::RESET_COMMAND_BUFFER.into(),
                        queue_family_index: family_idx,
                        ..Default::default()
                    },
                    None,
                    &mut command_pool,
                )
                .unwrap()
        };

        let [cmdbuf] = unsafe {
            device
                .allocate_command_buffers(&vkx::CommandBufferAllocateInfo {
                    command_pool,
                    level: vkx::CommandBufferLevel::PRIMARY,
                    command_buffer_count: 1,
                    ..Default::default()
                })
                .unwrap()
                .try_into()
                .ok()
                .unwrap()
        };

        let mut swapchain_fence = vkx::Fence::null();
        let mut swapchain_acquire_semaphore = vkx::Semaphore::null();
        let mut swapchain_finished_semaphore = vkx::Semaphore::null();
        unsafe {
            device
                .create_fence(
                    &vkx::FenceCreateInfo {
                        flags: vkx::FenceCreateFlag::SIGNALED.into(),
                        ..Default::default()
                    },
                    None,
                    &mut swapchain_fence,
                )
                .unwrap();

            device
                .create_semaphore(
                    &vkx::SemaphoreCreateInfo::default(),
                    None,
                    &mut swapchain_acquire_semaphore,
                )
                .unwrap();

            device
                .create_semaphore(
                    &vkx::SemaphoreCreateInfo::default(),
                    None,
                    &mut swapchain_finished_semaphore,
                )
                .unwrap();
        };

        Self {
            instance,
            physical_device,
            device,
            queue,
            pipeline,
            cmdbuf,
            window: None,
            swapchain: None,
            swapchain_fence,
            swapchain_acquire_semaphore,
            swapchain_finished_semaphore,
            recreate_swapchain: false,
        }
    }

    fn create_swapchain(&mut self) {
        let window = self.window.as_ref().unwrap();

        // create the swapchain
        let window_size = window.window.inner_size();
        let mut swapchain = vkx::SwapchainKHR::null();
        unsafe {
            self.device
                .create_swapchain_khr(
                    &vkx::SwapchainCreateInfoKHR {
                        surface: window.surface,
                        min_image_count: window.surface_caps.min_image_count,
                        image_format: vkx::Format::B8G8R8A8_SRGB,
                        image_color_space: vkx::ColorSpaceKHR::COLOR_SPACE_SRGB_NONLINEAR_KHR,
                        image_extent: vkx::Extent2D {
                            width: window_size.width,
                            height: window_size.height,
                        },
                        image_array_layers: 1,
                        image_usage: vkx::ImageUsageFlag::COLOR_ATTACHMENT.into(),
                        image_sharing_mode: vkx::SharingMode::EXCLUSIVE,
                        pre_transform: window.surface_caps.current_transform,
                        composite_alpha: vkx::CompositeAlphaFlagKHR::OPAQUE_KHR.into(),
                        present_mode: vkx::PresentModeKHR::PRESENT_MODE_FIFO_KHR,
                        old_swapchain: self
                            .swapchain
                            .as_ref()
                            .map(|s| s.handle)
                            .unwrap_or_default(),
                        ..Default::default()
                    },
                    None,
                    &mut swapchain,
                )
                .unwrap()
        };

        // get the swapchain images and create views for them
        let (swapchain_images, result) = unsafe {
            vkx::auto_count!(|count, vec| self
                .device
                .get_swapchain_images_khr(swapchain, &mut count, vec))
        };

        assert!(result.unwrap().is_success());
        let swapchain_image_views = swapchain_images
            .iter()
            .map(|i| {
                let mut img_view = vkx::ImageView::default();
                unsafe {
                    self.device
                        .create_image_view(
                            &vkx::ImageViewCreateInfo {
                                image: *i,
                                view_type: vkx::ImageViewType::_2D,
                                format: vkx::Format::B8G8R8A8_SRGB,
                                subresource_range: vkx::ImageSubresourceRange {
                                    aspect_mask: vkx::ImageAspectFlag::COLOR.into(),
                                    level_count: 1,
                                    layer_count: 1,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            None,
                            &mut img_view,
                        )
                        .unwrap()
                };

                img_view
            })
            .collect::<Vec<_>>();

        let old_swapchain = std::mem::replace(
            &mut self.swapchain,
            Some(SwapchainState {
                handle: swapchain,
                images: swapchain_images,
                views: swapchain_image_views,
            }),
        );

        if let Some(swapchain) = old_swapchain {
            for view in swapchain.views.into_iter() {
                unsafe { self.device.destroy_image_view(Some(view), None) };
            }

            unsafe {
                self.device.device_wait_idle().unwrap();
                self.device
                    .destroy_swapchain_khr(Some(swapchain.handle), None)
            };
        }
    }

    fn create_pipeline(device: &vkx::Device) -> Result<vkx::Pipeline, vkx::ErrorCode> {
        // pipeline layout
        let mut layout = vkx::PipelineLayout::null();
        unsafe {
            device.create_pipeline_layout(
                &vkx::PipelineLayoutCreateInfo::default(),
                None,
                &mut layout,
            )?
        }

        // color blend
        let color_blend_attachment = vkx::PipelineColorBlendAttachmentState {
            blend_enable: false.into(),
            color_write_mask: vkx::ColorComponentFlags::full(),
            ..Default::default()
        };

        // dynamic states
        let dynamic_states = [vkx::DynamicState::VIEWPORT, vkx::DynamicState::SCISSOR];

        // dynamic rendering
        let color_attachment_format = vkx::Format::B8G8R8A8_SRGB;
        let mut dynamic_rendering_info = vkx::PipelineRenderingCreateInfo {
            color_attachment_count: 1,
            p_color_attachment_formats: &color_attachment_format,
            ..Default::default()
        };

        // shaders
        let vertex_shader = vertex_shader();
        let fragment_shader = fragment_shader();

        let vertex_shader_create_info = vkx::ShaderModuleCreateInfo {
            code_size: vertex_shader.len() * 4,
            p_code: vertex_shader.as_ptr(),
            ..Default::default()
        };

        let fragment_shader_create_info = vkx::ShaderModuleCreateInfo {
            code_size: fragment_shader.len() * 4,
            p_code: fragment_shader.as_ptr(),
            ..Default::default()
        };

        let mut vertex_shader_mod = vkx::ShaderModule::null();
        unsafe {
            device.create_shader_module(&vertex_shader_create_info, None, &mut vertex_shader_mod)?
        };

        let mut fragment_shader_mod = vkx::ShaderModule::null();
        unsafe {
            device.create_shader_module(
                &fragment_shader_create_info,
                None,
                &mut fragment_shader_mod,
            )?
        };

        let stages = [
            vkx::PipelineShaderStageCreateInfo {
                stage: vkx::ShaderStageFlag::VERTEX.into(),
                p_name: c"main".as_ptr(),
                module: vertex_shader_mod,
                ..Default::default()
            },
            vkx::PipelineShaderStageCreateInfo {
                stage: vkx::ShaderStageFlag::FRAGMENT.into(),
                p_name: c"main".as_ptr(),
                module: fragment_shader_mod,
                ..Default::default()
            },
        ];

        // create it!
        let mut pipeline_create_info = vkx::GraphicsPipelineCreateInfo {
            stage_count: stages.len() as u32,
            p_stages: stages.as_ptr(),
            p_vertex_input_state: &vkx::PipelineVertexInputStateCreateInfo::default(),
            p_input_assembly_state: &vkx::PipelineInputAssemblyStateCreateInfo {
                topology: vkx::PrimitiveTopology::TRIANGLE_LIST,
                primitive_restart_enable: false.into(),
                ..Default::default()
            },
            p_viewport_state: &vkx::PipelineViewportStateCreateInfo {
                viewport_count: 1,
                scissor_count: 1,
                ..Default::default()
            },
            p_rasterization_state: &vkx::PipelineRasterizationStateCreateInfo {
                line_width: 1.0,
                ..Default::default()
            },
            p_multisample_state: &vkx::PipelineMultisampleStateCreateInfo {
                rasterization_samples: vkx::SampleCountFlag::_1.into(),
                ..Default::default()
            },
            p_depth_stencil_state: &vkx::PipelineDepthStencilStateCreateInfo::default(),
            p_color_blend_state: &vkx::PipelineColorBlendStateCreateInfo {
                attachment_count: 1,
                p_attachments: &color_blend_attachment,
                ..Default::default()
            },
            p_dynamic_state: &vkx::PipelineDynamicStateCreateInfo {
                dynamic_state_count: dynamic_states.len() as u32,
                p_dynamic_states: dynamic_states.as_ptr(),
                ..Default::default()
            },
            layout,
            ..Default::default()
        };
        pipeline_create_info.push_next(&mut dynamic_rendering_info);

        let mut pipeline = vkx::Pipeline::null();
        unsafe {
            device.create_graphics_pipelines(None, 1, &pipeline_create_info, None, &mut pipeline)?
        };

        Ok(pipeline)
    }

    fn draw(&mut self) -> Result<(), vkx::ErrorCode> {
        let window = self.window.as_ref().unwrap();
        let swapchain = self.swapchain.as_ref().unwrap();

        // 00. wait until previous draw is finished
        unsafe {
            self.device
                .wait_for_fences(1, &self.swapchain_fence, true.into(), u64::MAX)
                .unwrap();

            self.device.reset_fences(1, &self.swapchain_fence).unwrap();
        };

        unsafe { self.device.device_wait_idle().unwrap() };

        // 01. acquire swapchain image
        let mut swapchain_img_idx = 0;
        loop {
            let success_code = unsafe {
                self.device.acquire_next_image_khr(
                    swapchain.handle,
                    u64::MAX,
                    Some(self.swapchain_acquire_semaphore),
                    None,
                    &mut swapchain_img_idx,
                )?
            };

            if success_code == vkx::SuccessCode::SUCCESS {
                break;
            }
        }

        // 02. record a command buffer
        // reset the command buffer
        unsafe { self.cmdbuf.reset(None)? };

        // and start recording
        unsafe {
            self.cmdbuf.begin(&vkx::CommandBufferBeginInfo {
                flags: vkx::CommandBufferUsageFlag::ONE_TIME_SUBMIT.into(),
                ..Default::default()
            })?
        };

        // synchronize and transition the swapchain image into an optimal layout for attachments
        let image_barrier = vkx::ImageMemoryBarrier2 {
            // wait until all uses of the image are done
            src_stage_mask: vkx::PipelineStageFlag2::COLOR_ATTACHMENT_OUTPUT.into(),
            dst_stage_mask: vkx::PipelineStageFlag2::COLOR_ATTACHMENT_OUTPUT.into(),
            // we could make previous writes available to the layout transition, but since we are
            // transitioning from UNDEFINED, we dont care
            src_access_mask: Default::default(),
            // signal we plan to write to the image. this makes the layout transition visible
            dst_access_mask: vkx::AccessFlag2::COLOR_ATTACHMENT_WRITE.into(),
            old_layout: vkx::ImageLayout::UNDEFINED,
            new_layout: vkx::ImageLayout::ATTACHMENT_OPTIMAL,
            image: swapchain.images[swapchain_img_idx as usize],
            subresource_range: vkx::ImageSubresourceRange {
                aspect_mask: vkx::ImageAspectFlag::COLOR.into(),
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
            ..Default::default()
        };

        unsafe {
            self.cmdbuf.cmd_pipeline_barrier_2(&vkx::DependencyInfo {
                image_memory_barrier_count: 1,
                p_image_memory_barriers: &image_barrier,
                ..Default::default()
            });
        }

        // begin rendering
        let color_attachment_info = vkx::RenderingAttachmentInfo {
            image_view: swapchain.views[swapchain_img_idx as usize],
            image_layout: vkx::ImageLayout::ATTACHMENT_OPTIMAL,
            load_op: vkx::AttachmentLoadOp::CLEAR,
            store_op: vkx::AttachmentStoreOp::STORE,
            clear_value: vkx::ClearValue {
                color: vkx::ClearColorValue {
                    float_32: [0.0, 0.0, 0.1, 1.0],
                },
            },
            ..Default::default()
        };

        let window_size = window.window.inner_size();
        unsafe {
            self.cmdbuf.cmd_begin_rendering(&vkx::RenderingInfo {
                render_area: vkx::Rect2D {
                    offset: vkx::Offset2D::default(),
                    extent: vkx::Extent2D {
                        width: window_size.width,
                        height: window_size.height,
                    },
                },
                layer_count: 1,
                color_attachment_count: 1,
                p_color_attachments: &color_attachment_info,
                ..Default::default()
            });
        }

        // setup the viewport and scissor
        unsafe {
            self.cmdbuf.cmd_set_viewport(
                0,
                1,
                &vkx::Viewport {
                    x: 0.0,
                    y: 0.0,
                    width: window_size.width as f32,
                    height: window_size.height as f32,
                    min_depth: 0.0,
                    max_depth: 1.0,
                },
            );

            self.cmdbuf.cmd_set_scissor(
                0,
                1,
                &vkx::Rect2D {
                    offset: vkx::Offset2D::default(),
                    extent: vkx::Extent2D {
                        width: window_size.width,
                        height: window_size.height,
                    },
                },
            );
        }

        // bind the pipeline, draw and finish rendering
        unsafe {
            self.cmdbuf
                .cmd_bind_pipeline(vkx::PipelineBindPoint::GRAPHICS, self.pipeline);
            self.cmdbuf.cmd_draw(3, 1, 0, 0);
            self.cmdbuf.cmd_end_rendering();
        }

        // synchronize and transition the swapchain image into an optimal layout for presentation
        let image_barrier = vkx::ImageMemoryBarrier2 {
            // wait until all uses of the image are done
            src_stage_mask: vkx::PipelineStageFlag2::COLOR_ATTACHMENT_OUTPUT.into(),
            // make previous writes available to the layout transition
            src_access_mask: vkx::AccessFlag2::COLOR_ATTACHMENT_WRITE.into(),
            // ensure the 2nd sync scope waits until the layout transition is complete
            dst_stage_mask: vkx::PipelineStageFlag2::COLOR_ATTACHMENT_OUTPUT.into(),
            old_layout: vkx::ImageLayout::ATTACHMENT_OPTIMAL,
            new_layout: vkx::ImageLayout::PRESENT_SRC_KHR,
            image: swapchain.images[swapchain_img_idx as usize],
            subresource_range: vkx::ImageSubresourceRange {
                aspect_mask: vkx::ImageAspectFlag::COLOR.into(),
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
            ..Default::default()
        };

        unsafe {
            self.cmdbuf.cmd_pipeline_barrier_2(&vkx::DependencyInfo {
                image_memory_barrier_count: 1,
                p_image_memory_barriers: &image_barrier,
                ..Default::default()
            });
        }

        // end the command buffer
        unsafe { self.cmdbuf.end()? };

        // 03. submit and present
        unsafe {
            self.queue.submit_2(
                Some(1),
                &vkx::SubmitInfo2 {
                    wait_semaphore_info_count: 1,
                    p_wait_semaphore_infos: &vkx::SemaphoreSubmitInfo {
                        semaphore: self.swapchain_acquire_semaphore,
                        // stage to wait for the sempahore
                        stage_mask: vkx::PipelineStageFlag2::COLOR_ATTACHMENT_OUTPUT.into(),
                        ..Default::default()
                    },
                    command_buffer_info_count: 1,
                    p_command_buffer_infos: &vkx::CommandBufferSubmitInfo {
                        command_buffer: self.cmdbuf.handle(),
                        ..Default::default()
                    },
                    signal_semaphore_info_count: 1,
                    p_signal_semaphore_infos: &vkx::SemaphoreSubmitInfo {
                        semaphore: self.swapchain_finished_semaphore,
                        // stage to signal the semaphore
                        stage_mask: vkx::PipelineStageFlag2::COLOR_ATTACHMENT_OUTPUT.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Some(self.swapchain_fence),
            )?;

            self.queue.present_khr(&vkx::PresentInfoKHR {
                wait_semaphore_count: 1,
                p_wait_semaphores: &self.swapchain_finished_semaphore,
                swapchain_count: 1,
                p_swapchains: &swapchain.handle,
                p_image_indices: &swapchain_img_idx,
                ..Default::default()
            })?;
        }

        Ok(())
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // create a window and a surface if we havent yet
        if self.window.is_none() {
            let window = event_loop
                .create_window(Window::default_attributes())
                .unwrap();

            let surface = unsafe {
                vkx::window::create_surface(&self.instance, event_loop, &window).unwrap()
            };

            let mut surface_caps = vkx::SurfaceCapabilitiesKHR::default();
            unsafe {
                self.physical_device
                    .get_surface_capabilities_khr(surface, &mut surface_caps)
                    .unwrap()
            };

            self.window = Some(WindowState {
                window,
                surface,
                surface_caps,
            });
        }

        // (re)create the swapchain
        self.create_swapchain();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if std::mem::take(&mut self.recreate_swapchain) {
                    self.create_swapchain();
                }

                self.draw().unwrap();
            }
            WindowEvent::Resized(_) => {
                self.recreate_swapchain = true;
            }
            _ => (),
        }
    }
}

fn main() {
    unsafe { vkx::setup().unwrap() };

    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new(&event_loop);
    event_loop.run_app(&mut app).unwrap();
}
