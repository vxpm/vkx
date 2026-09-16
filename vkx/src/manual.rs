//! Manually implemented items.

use std::sync::OnceLock;

use libloading::Library;

use crate::vkVoidFunction;

/// Marker trait indicating a vulkan structure that can be extended.
#[diagnostic::on_unimplemented(
    message = "Vulkan structure `{Self}` cannot be extended",
    note = "It does not have a `next` pointer"
)]
pub unsafe trait Extendable: Sized {
    fn with_next<T: Extends<Self>>(self, next: *mut T) -> Self;
}

/// Marker trait indicating a vulkan structure extends another.
#[diagnostic::on_unimplemented(
    message = "Vulkan structure `{Self}` does not extend structure `{T}`",
    note = "Documentation of `{T}` contains a list of all structures extending it"
)]
pub unsafe trait Extends<T>: Extendable {}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBool32.html>
#[doc(alias = "VkBool32")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum Bool32 {
    #[default]
    False = 0,
    True = 1,
}

impl From<Bool32> for bool {
    #[inline(always)]
    fn from(value: Bool32) -> Self {
        value == Bool32::True
    }
}

impl From<bool> for Bool32 {
    #[inline(always)]
    fn from(value: bool) -> Self {
        if value { Self::True } else { Self::False }
    }
}

impl std::cmp::PartialEq<bool> for Bool32 {
    #[inline(always)]
    fn eq(&self, other: &bool) -> bool {
        bool::from(*self) == *other
    }
}

impl std::cmp::PartialEq<Bool32> for bool {
    #[inline(always)]
    fn eq(&self, other: &Bool32) -> bool {
        bool::from(*other) == *self
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceAddress.html>
#[doc(alias = "VkDeviceAddress")]
pub type DeviceAddress = u64;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceAddress.html>
#[doc(alias = "VkDeviceSize")]
pub type DeviceSize = u64;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSampleMask.html>
#[doc(alias = "VkSampleMask")]
pub type SampleMask = u32;

#[derive(Debug)]
pub(crate) struct LoadedCommands<const N: usize>(Box<[vkVoidFunction; N]>);

impl<const N: usize> LoadedCommands<N> {
    pub fn get(&self, index: usize) -> Option<vkVoidFunction> {
        let func = self.0[index];
        let ptr = unsafe { std::mem::transmute::<vkVoidFunction, *const std::ffi::c_void>(func) };
        (!ptr.is_null()).then_some(func)
    }
}

static _LIBRARY: OnceLock<Library> = OnceLock::new();
static GLOBAL_COMMANDS: OnceLock<LoadedCommands<{ crate::GlobalCommands::VARIANTS.len() }>> =
    OnceLock::new();

pub enum SetupError {
    Loading(libloading::Error),
}

pub unsafe fn setup() -> Result<(), SetupError> {
    const PATH: &str = cfg_select! {
        any(target_os = "android", target_os = "fuchsia") => "libvulkan.so",
        any(target_os = "macos", target_os = "ios") => "libvulkan.dylib",
        unix => "libvulkan.so.1",
        windows => "vulkan-1.dll",
        _ => std::compile_error!("unsupported platform"),
    };

    let library = unsafe { Library::new(PATH).map_err(SetupError::Loading) }?;
    let get_instance_proc_addr = *unsafe {
        library
            .get::<crate::FUN_GetInstanceProcAddr>("vkGetInstanceProcAddr")
            .map_err(SetupError::Loading)
    }?;

    let mut global_commands = vec![];
    for command in crate::GlobalCommands::VARIANTS {
        let command = unsafe {
            get_instance_proc_addr(crate::InstanceHandle::default(), command.name().as_ptr())
        };

        global_commands.push(command);
    }

    GLOBAL_COMMANDS
        .set(LoadedCommands(global_commands.try_into().unwrap()))
        .expect("setup should only be called once");

    Ok(())
}

pub struct Instance {
    handle: crate::InstanceHandle,
}

pub struct PhysicalDevice {
    handle: crate::PhysicalDeviceHandle,
}

pub struct Device {
    handle: crate::DeviceHandle,
}

pub struct Queue {
    handle: crate::QueueHandle,
}

pub struct CommandBuffer {
    handle: crate::CommandBufferHandle,
}
