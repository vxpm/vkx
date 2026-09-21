use core::ffi::CStr;
use std::collections::HashSet;

use vkx::Extendable;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]
struct App {
    window: Option<Window>,
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
            std::ptr::null(),
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
                        vkx::auto_count!(|count, vec| dev.enumerate_device_extension_properties(
                            std::ptr::null(),
                            &mut count,
                            vec
                        ))
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
            .create_device(&device_create_info, std::ptr::null())
            .unwrap();

        // and get the queue
        let queue = unsafe { device.get_device_queue(family_idx, 0) };

        todo!()
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() {
    unsafe { vkx::setup().unwrap() };

    let event_loop = EventLoop::new().unwrap();
    let app = App::new(&event_loop);
}
