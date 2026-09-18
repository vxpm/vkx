//! Manually implemented items.

use std::sync::OnceLock;

use libloading::Library;

pub(crate) type VTable<const N: usize> = [crate::vkVoidFunction; N];

type GlobalVTable = VTable<{ crate::GlobalCommands::VARIANTS.len() }>;
type InstanceVTable = VTable<{ crate::InstanceCommands::VARIANTS.len() }>;

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
    pub commands: GlobalVTable,
}

pub(crate) static GLOBAL: OnceLock<Global> = OnceLock::new();

/// Setups the loader. This needs to be called before anything else in the crate can be used.
pub unsafe fn setup() -> Result<(), libloading::Error> {
    const PATH: &str = cfg_select! {
        any(target_os = "android", target_os = "fuchsia") => "libvulkan.so",
        any(target_os = "macos", target_os = "ios") => "libvulkan.dylib",
        unix => "libvulkan.so.1",
        windows => "vulkan-1.dll",
        _ => std::compile_error!("unsupported platform"),
    };

    let lib = unsafe { Library::new(PATH) }?;
    let get_instance_proc_addr =
        *unsafe { lib.get::<crate::FUN_GetInstanceProcAddr>("vkGetInstanceProcAddr") }?;

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

/// An [`InstanceHandle`](crate::InstanceHandle) wrapper that carries a vtable generated at
/// creation time.
///
/// This type is like a smart version of an [`InstanceHandle`](crate::InstanceHandle): it knows how
/// to call every instance function and will also destroy itself at drop time.
///
/// Children handles of this instance carry a reference to the same vtable, but calling any
/// function on them _after_ this instance is destroyed is _undefined behaviour_.
pub struct Instance {
    pub(crate) handle: crate::InstanceHandle,
    pub(crate) vtable: &'static InstanceVTable,
}

impl Instance {
    #[inline(always)]
    pub(crate) fn vtable(&self) -> &InstanceVTable {
        &self.vtable
    }

    /// Creates a new [`Instance`]. This is a wrapper around [`create_instance`](crate::create_instance).
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
        let instance_commands = Box::leak(boxed_array);

        Self {
            handle: instance,
            vtable: instance_commands,
        }
    }

    pub fn enumerate_physical_devices(&self) -> Vec<PhysicalDevice> {
        let mut count = 0;
        let result =
            unsafe { self.raw_enumerate_physical_devices(&mut count, std::ptr::null_mut()) };
        assert_eq!(result, crate::ResultCode::SUCCESS);

        let mut devices = vec![crate::PhysicalDeviceHandle::default(); count as usize];
        let result =
            unsafe { self.raw_enumerate_physical_devices(&mut count, devices.as_mut_ptr()) };
        assert_eq!(result, crate::ResultCode::SUCCESS);

        devices
            .into_iter()
            .map(|handle| PhysicalDevice {
                handle,
                vtable: self.vtable,
            })
            .collect()
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe { self.destroy(std::ptr::null()) };
    }
}

pub struct PhysicalDevice {
    pub(crate) handle: crate::PhysicalDeviceHandle,
    pub(crate) vtable: &'static VTable<{ crate::InstanceCommands::VARIANTS.len() }>,
}

impl PhysicalDevice {
    #[track_caller]
    #[inline(always)]
    pub(crate) fn vtable(&self) -> &InstanceVTable {
        self.vtable
    }
}

pub struct Device {
    pub(crate) handle: crate::DeviceHandle,
    pub(crate) vtable: &'static VTable<{ crate::InstanceCommands::VARIANTS.len() }>,
}

impl Device {
    #[track_caller]
    #[inline(always)]
    pub(crate) fn vtable(&self) -> &InstanceVTable {
        self.vtable
    }
}

pub struct Queue {
    pub(crate) handle: crate::QueueHandle,
    pub(crate) vtable: &'static VTable<{ crate::InstanceCommands::VARIANTS.len() }>,
}

impl Queue {
    #[track_caller]
    #[inline(always)]
    pub(crate) fn vtable(&self) -> &InstanceVTable {
        self.vtable
    }
}

pub struct CommandBuffer {
    pub(crate) handle: crate::CommandBufferHandle,
    pub(crate) vtable: &'static VTable<{ crate::InstanceCommands::VARIANTS.len() }>,
}

impl CommandBuffer {
    #[track_caller]
    #[inline(always)]
    pub(crate) fn vtable(&self) -> &InstanceVTable {
        self.vtable
    }
}
