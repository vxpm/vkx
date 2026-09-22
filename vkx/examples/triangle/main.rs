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

struct SwapchainState {
    surface: vkx::SurfaceKHR,
    surface_caps: vkx::SurfaceCapabilitiesKHR,
    swapchain: vkx::SwapchainKHR,
    images: Vec<vkx::Image>,
    views: Vec<vkx::ImageView>,
}

struct App {
    instance: vkx::Instance,
    physical_device: vkx::PhysicalDevice,
    device: vkx::Device,
    queue: vkx::Queue,
    pipeline: vkx::Pipeline,
    window: Option<Window>,
    swapchain: Option<SwapchainState>,
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
                    let extensions = unsafe {
                        vkx::auto_count!(|count, vec| dev
                            .enumerate_device_extension_properties(None, &mut count, vec))
                    };

                    let extensions = vkx::Extension::from_ext_properties(extensions);

                    // we want a device that supports at least vulkan 1.3 and has all the device
                    // extensions we need
                    properties.api_version >= vkx::Version::V1_3.get()
                        && extensions.is_superset(&device_extensions)
                })
                .filter_map(|(dev, properties)| {
                    let queue_families = unsafe {
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

        Self {
            instance,
            physical_device,
            device,
            queue,
            pipeline,
            window: None,
            swapchain: None,
        }
    }

    fn create_swapchain(&mut self, event_loop: &ActiveEventLoop) {
        let window = self.window.as_ref().unwrap();

        // create a surface
        let surface =
            unsafe { vkx::window::create_surface(&self.instance, event_loop, &window).unwrap() };

        let mut surface_caps = vkx::SurfaceCapabilitiesKHR::default();
        unsafe {
            self.physical_device
                .get_surface_capabilities_khr(surface, &mut surface_caps)
                .success()
                .unwrap()
        };

        // create the swapchain
        let window_size = window.inner_size();
        let mut swapchain = vkx::SwapchainKHR::null();
        unsafe {
            self.device.create_swapchain_khr(
                &vkx::SwapchainCreateInfoKHR {
                    surface: surface,
                    min_image_count: surface_caps.min_image_count,
                    image_format: vkx::Format::B8G8R8A8_SRGB,
                    image_color_space: vkx::ColorSpaceKHR::COLOR_SPACE_SRGB_NONLINEAR_KHR,
                    image_extent: vkx::Extent2D {
                        width: window_size.width,
                        height: window_size.height,
                    },
                    image_array_layers: 1,
                    image_usage: vkx::ImageUsageFlag::COLOR_ATTACHMENT.into(),
                    image_sharing_mode: vkx::SharingMode::EXCLUSIVE,
                    pre_transform: surface_caps.current_transform,
                    composite_alpha: vkx::CompositeAlphaFlagKHR::OPAQUE_KHR.into(),
                    present_mode: vkx::PresentModeKHR::PRESENT_MODE_FIFO_KHR,
                    ..Default::default()
                },
                None,
                &mut swapchain,
            )
        };

        // get the swapchain images and create views for them
        let swapchain_images = unsafe {
            vkx::auto_count!(|count, vec| self
                .device
                .get_swapchain_images_khr(swapchain, &mut count, vec))
        };

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
                        .success()
                        .unwrap()
                };

                img_view
            })
            .collect::<Vec<_>>();

        self.swapchain = Some(SwapchainState {
            surface,
            surface_caps,
            swapchain,
            images: swapchain_images,
            views: swapchain_image_views,
        })
    }

    fn create_pipeline(device: &vkx::Device) -> Result<vkx::Pipeline, vkx::ErrorCode> {
        // pipeline layout
        let mut layout = vkx::PipelineLayout::null();
        unsafe {
            device
                .create_pipeline_layout(
                    &vkx::PipelineLayoutCreateInfo::default(),
                    None,
                    &mut layout,
                )
                .success()?
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
            device
                .create_shader_module(&vertex_shader_create_info, None, &mut vertex_shader_mod)
                .success()?
        };

        let mut fragment_shader_mod = vkx::ShaderModule::null();
        unsafe {
            device
                .create_shader_module(&fragment_shader_create_info, None, &mut fragment_shader_mod)
                .success()?
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
            device
                .create_graphics_pipelines(None, 1, &pipeline_create_info, None, &mut pipeline)
                .success()?
        };

        Ok(pipeline)
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // create a window if we havent yet
        if self.window.is_none() {
            self.window = Some(
                event_loop
                    .create_window(Window::default_attributes())
                    .unwrap(),
            );
        }

        // (re)create the swapchain
        self.create_swapchain(event_loop);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                todo!()
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
