//! # vkx
//! Auto-generated Vulkan bindings with utilities and a loader. Unlike most Vulkan crates in Rust,
//! `vkx` does not parse `vk.xml` directly - it uses Khronos Group's [`vulkan-object`] Python library
//! instead.
//!
//! `vkx` tries to add as much documentation as possible to the generated code, reducing the need to
//! open Vulkan documentation to know some basics regarding an item. It also aliases every generated
//! item to their raw Vulkan API name, so searching `vkx`'s documentation for something like
//! `vkGetBufferDeviceAddressKHR` will get you to [`Device::get_buffer_device_address_khr`].
//!
//! `vkx` does not try to "rustify" the Vulkan API too much - arguments are kept as pointers, and
//! return values through out parameters are kept like that. It also does _not_ introduce lifetimes
//! into structures to add some pseudo-safety to structure chains.
//!
//! This version of `vkx` has been built against Vulkan
#![doc = concat!(include_str!("../version.md"), ".")]
//!
//! # Quickstart
//! Call [`setup`]: this will dynamically load the Vulkan library using [`libloading`] and build a
//! vtable with all global commands. From there, you can create an instance using either
//! [`create_instance`] (if you want to build vtables yourself) or (the recommended approach)
//! [`Instance::create`].
//!
//! ```rust
//! // always run setup before using vkx!
//! unsafe { vkx::setup().unwrap() };
//!
//! let app_info = vkx::ApplicationInfo {
//!     p_application_name: c"example".as_ptr(),
//!     application_version: vkx::Version::V1_0.get(),
//!     p_engine_name: c"vkx".as_ptr(),
//!     api_version: vkx::Version::V1_3.get(),
//!     ..Default::default()
//! };
//!
//! let instance_layers = &[c"VK_LAYER_KHRONOS_validation".as_ptr()];
//! let instance = vkx::Instance::create(
//!     &vkx::InstanceCreateInfo {
//!         p_application_info: &app_info,
//!         enabled_layer_count: instance_layers.len() as u32,
//!         pp_enabled_layer_names: instance_layers.as_ptr(),
//!         ..Default::default()
//!     },
//!     std::ptr::null(),
//! )
//! .unwrap();
//!
//! // have fun with your instance
//! ```
//!
//! # Dispatchable handles
//! All dispatchable handles are suffixed with `Handle` to indicate they're the just the handle
//! itself: [`InstanceHandle`], [`PhysicalDeviceHandle`], [`DeviceHandle`], [`QueueHandle`] and
//! [`CommandBufferHandle`].
//!
//! However, `vkx` also provides a loader for instance and device level commands that can be used
//! through the special dispatchable handle wrappers: [`Instance`], [`PhysicalDevice`], [`Device`],
//! [`Queue`] and [`CommandBuffer`].
//!
//! These special wrappers can only be created from special methods on their respective parents,
//! e.g. to create a [`Device`], use [`PhysicalDevice::create_device`]. The only exception is
//! [`Instance`] since it is the "entrypoint" and uses [`Instance::create`].
//!
//! These handles do _not_ destroy themselves - you still need to destroy them properly when needed.
//!
//! # Special items
//! These items are not in the Vulkan API - they're purely `vkx` utilities:
//! - [`Version`]
//! - [`Extension`]
//! - [`SuccessCode`]
//! - [`ErrorCode`]
//! - Dispatchable handle wrappers
//!
//! # Safety
//! Almost everything is `unsafe`: the Vulkan library is dynamically loaded, so there's no guarantee
//! whatsoever about what it might do. In practice, however, it's just going to be a valid vulkan
//! implementation, and the safety precautions are now on you.
//!
//! In general, you're responsible for guaranteeing pointers and structure-chain lifetimes are
//! valid. You also need to ensure dispatchable handles outlive their parents, otherwise they might
//! cause a use-after-free (the vtable is deallocated by the instance/device on drop).
//!
//! # Windowing
//! `vkx` has support for [`raw_window_handle`] through the `window` module. Enable the `window`
//! feature flag to use it (it is enabled by default).
//!
//! [`vulkan-object`]: https://github.com/KhronosGroup/vulkan-object

#![feature(doc_cfg)]

mod internal;
mod loader;
mod platform;

#[cfg(feature = "window")]
pub mod window;

// auto generated modules
mod commands;
mod consts;
mod consts_inner;
mod enums;
mod flags;
mod fn_ptrs;
mod handles;
mod structs;

pub use commands::*;
pub use consts::*;
pub use enums::*;
pub use flags::*;
pub use fn_ptrs::*;
pub use handles::*;
pub use internal::*;
pub use loader::*;
pub use structs::*;

pub use libloading;
