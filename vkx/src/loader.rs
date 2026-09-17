//! Manually implemented items.

use std::sync::{Arc, OnceLock, Weak};

use libloading::Library;

pub(crate) type VTable<const N: usize> = [crate::vkVoidFunction; N];

#[inline(always)]
pub(crate) fn vtable_get<const N: usize>(
    table: &VTable<N>,
    index: usize,
) -> Option<crate::vkVoidFunction> {
    let func = table[index];
    let ptr =
        unsafe { std::mem::transmute::<crate::vkVoidFunction, *const std::ffi::c_void>(func) };
    (!ptr.is_null()).then_some(func)
}

pub(crate) struct Global {
    pub _lib: Library,
    pub get_instance_proc_addr: crate::FUN_GetInstanceProcAddr,
    pub commands: VTable<{ crate::GlobalCommands::VARIANTS.len() }>,
}

pub(crate) static GLOBAL: OnceLock<Global> = OnceLock::new();

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

    let lib = unsafe { Library::new(PATH).map_err(SetupError::Loading) }?;
    let get_instance_proc_addr = *unsafe {
        lib.get::<crate::FUN_GetInstanceProcAddr>("vkGetInstanceProcAddr")
            .map_err(SetupError::Loading)
    }?;

    let mut global_commands = Vec::with_capacity(crate::GlobalCommands::VARIANTS.len());
    for command in crate::GlobalCommands::VARIANTS {
        let command = unsafe {
            get_instance_proc_addr(crate::InstanceHandle::default(), command.name().as_ptr())
        };

        global_commands.push(command);
    }

    let global_commands = global_commands.try_into().unwrap();
    GLOBAL
        .set(Global {
            _lib: lib,
            get_instance_proc_addr,
            commands: global_commands,
        })
        .ok()
        .expect("setup should only be called once");

    Ok(())
}

type InstanceVTable = VTable<{ crate::InstanceCommands::VARIANTS.len() }>;

pub struct Instance {
    pub(crate) handle: crate::InstanceHandle,
    pub(crate) vtable: Arc<VTable<{ crate::InstanceCommands::VARIANTS.len() }>>,
}

impl Instance {
    pub fn create(
        create_info: *const crate::InstanceCreateInfo,
        allocator: *const crate::AllocationCallbacks,
    ) -> Self {
        let mut instance = crate::InstanceHandle::default();
        let result = unsafe { crate::create_instance(create_info, allocator, &mut instance) };
        assert_eq!(result, crate::ResultCode::SUCCESS);

        let get_instance_proc_addr = GLOBAL
            .get()
            .expect("vkx setup should have been run")
            .get_instance_proc_addr;

        let mut instance_commands = Vec::with_capacity(crate::InstanceCommands::VARIANTS.len());
        for command in crate::InstanceCommands::VARIANTS {
            let command = unsafe { get_instance_proc_addr(instance, command.name().as_ptr()) };
            instance_commands.push(command);
        }

        let boxed_array: Box<[_; _]> = instance_commands.into_boxed_slice().try_into().unwrap();
        let instance_commands = boxed_array.into();

        Self {
            handle: instance,
            vtable: instance_commands,
        }
    }

    #[inline(always)]
    pub(crate) fn vtable(&self) -> &InstanceVTable {
        &self.vtable
    }
}

pub struct PhysicalDevice {
    pub(crate) handle: crate::PhysicalDeviceHandle,
    pub(crate) vtable: Weak<VTable<{ crate::InstanceCommands::VARIANTS.len() }>>,
}

impl PhysicalDevice {
    #[track_caller]
    #[inline(always)]
    pub(crate) fn vtable(&self) -> Arc<InstanceVTable> {
        let Some(vtable) = self.vtable.upgrade() else {
            panic!("tried to use a physical device after it's parent instance has been dropped");
        };

        vtable
    }
}

pub struct Device {
    pub(crate) handle: crate::DeviceHandle,
    pub(crate) vtable: Weak<VTable<{ crate::InstanceCommands::VARIANTS.len() }>>,
}

impl Device {
    #[track_caller]
    #[inline(always)]
    pub(crate) fn vtable(&self) -> Arc<InstanceVTable> {
        let Some(vtable) = self.vtable.upgrade() else {
            panic!("tried to use a device after it's parent instance has been dropped");
        };

        vtable
    }
}

pub struct Queue {
    pub(crate) handle: crate::QueueHandle,
    pub(crate) vtable: Weak<VTable<{ crate::InstanceCommands::VARIANTS.len() }>>,
}

impl Queue {
    #[track_caller]
    #[inline(always)]
    pub(crate) fn vtable(&self) -> Arc<InstanceVTable> {
        let Some(vtable) = self.vtable.upgrade() else {
            panic!("tried to use a queue after it's parent instance has been dropped");
        };

        vtable
    }
}

pub struct CommandBuffer {
    pub(crate) handle: crate::CommandBufferHandle,
    pub(crate) vtable: Weak<VTable<{ crate::InstanceCommands::VARIANTS.len() }>>,
}

impl CommandBuffer {
    #[track_caller]
    #[inline(always)]
    pub(crate) fn vtable(&self) -> Arc<InstanceVTable> {
        let Some(vtable) = self.vtable.upgrade() else {
            panic!("tried to use a command buffer after it's parent instance has been dropped");
        };

        vtable
    }
}
