//! Manually implemented items.

use std::sync::OnceLock;

use libloading::Library;

pub(crate) type VTable<const N: usize> = [crate::vkVoidFunction; N];

type GlobalVTable = VTable<{ crate::GlobalCommand::VARIANTS.len() }>;
type InstanceVTable = VTable<{ crate::InstanceCommand::VARIANTS.len() }>;

#[inline(always)]
pub(crate) fn vtable_get<const N: usize>(table: &VTable<N>, index: usize) -> crate::vkVoidFunction {
    let func = table[index];
    let ptr =
        unsafe { std::mem::transmute::<crate::vkVoidFunction, *const std::ffi::c_void>(func) };
    assert!(!ptr.is_null(), "command should not be null");
    func
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

    let mut global_commands = Vec::with_capacity(crate::GlobalCommand::VARIANTS.len());
    for command in crate::GlobalCommand::VARIANTS {
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
/// Child handles of this instance carry a reference to the same vtable, but calling any
/// function on them _after_ this instance is destroyed is _undefined behaviour_.
pub struct Instance {
    pub(crate) handle: crate::InstanceHandle,
    pub(crate) vtable: *const InstanceVTable,
}

impl Instance {
    #[inline(always)]
    pub fn handle(&self) -> crate::InstanceHandle {
        self.handle
    }

    #[inline(always)]
    pub(crate) fn vtable(&self) -> &InstanceVTable {
        // SAFETY: the vtable is alive as long as the instance is
        unsafe { self.vtable.as_ref_unchecked() }
    }

    /// Creates a new [`Instance`]. This is a wrapper around [`create_instance`](crate::create_instance).
    pub fn create(
        create_info: *const crate::InstanceCreateInfo,
        allocator: *const crate::AllocationCallbacks,
    ) -> Result<Self, crate::ErrorCode> {
        let mut instance = crate::InstanceHandle::default();
        unsafe { crate::create_instance(create_info, allocator, &mut instance).success()? };

        let get_instance_proc_addr = GLOBAL
            .get()
            .expect("vkx setup should have been run")
            .get_instance_proc_addr;

        let mut instance_commands = Vec::with_capacity(crate::InstanceCommand::VARIANTS.len());
        for command in crate::InstanceCommand::VARIANTS {
            let command = unsafe { get_instance_proc_addr(instance, command.name().as_ptr()) };
            instance_commands.push(command);
        }

        let boxed_array: Box<[_; _]> = instance_commands.into_boxed_slice().try_into().unwrap();
        let instance_commands = Box::leak(boxed_array);

        Ok(Self {
            handle: instance,
            vtable: instance_commands,
        })
    }

    /// Enumerates physical devices - a wrapper around [`Self::raw_enumerate_physical_devices`].
    #[inline(always)]
    pub unsafe fn enumerate_physical_devices(
        &self,
    ) -> Result<Vec<PhysicalDevice>, crate::ErrorCode> {
        let mut count = 0;
        unsafe {
            self.raw_enumerate_physical_devices(&mut count, std::ptr::null_mut())
                .success()?;
        }

        let mut devices = vec![crate::PhysicalDeviceHandle::default(); count as usize];
        unsafe {
            self.raw_enumerate_physical_devices(&mut count, devices.as_mut_ptr())
                .success()?;
        }

        Ok(devices
            .into_iter()
            .map(|handle| PhysicalDevice {
                handle,
                vtable: self.vtable,
            })
            .collect())
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            self.destroy(std::ptr::null());
            std::mem::drop(Box::from_raw(self.vtable.cast_mut()))
        }
    }
}

pub struct PhysicalDevice {
    pub(crate) handle: crate::PhysicalDeviceHandle,
    pub(crate) vtable: *const VTable<{ crate::InstanceCommand::VARIANTS.len() }>,
}

impl PhysicalDevice {
    #[inline(always)]
    pub fn handle(&self) -> crate::PhysicalDeviceHandle {
        self.handle
    }

    #[track_caller]
    #[inline(always)]
    pub(crate) fn vtable(&self) -> &InstanceVTable {
        // SAFETY: user contract - parent instance must be alive
        unsafe { self.vtable.as_ref_unchecked() }
    }

    /// Creates a [`Device`] - a wrapper around [`Self::raw_create_device`].
    #[inline(always)]
    pub fn create_device(
        &self,
        create_info: *const crate::DeviceCreateInfo,
        allocator: *const crate::AllocationCallbacks,
    ) -> Result<Device, crate::ErrorCode> {
        let mut device = crate::DeviceHandle::default();
        unsafe {
            self.raw_create_device(create_info, allocator, &mut device)
                .success()?;
        }

        Ok(Device {
            handle: device,
            vtable: self.vtable,
        })
    }
}

pub struct Device {
    pub(crate) handle: crate::DeviceHandle,
    pub(crate) vtable: *const VTable<{ crate::InstanceCommand::VARIANTS.len() }>,
}

impl Device {
    #[inline(always)]
    pub fn handle(&self) -> crate::DeviceHandle {
        self.handle
    }

    #[inline(always)]
    pub(crate) fn vtable(&self) -> &InstanceVTable {
        // SAFETY: user contract - parent instance must be alive
        unsafe { self.vtable.as_ref_unchecked() }
    }

    /// Gets a [`Queue`] - a wrapper around [`Self::raw_get_device_queue`].
    pub unsafe fn get_device_queue(&self, queue_family_index: u32, queue_index: u32) -> Queue {
        let mut queue = crate::QueueHandle::null();
        unsafe { self.raw_get_device_queue(queue_family_index, queue_index, &mut queue) };

        Queue {
            handle: queue,
            vtable: self.vtable,
        }
    }

    /// Gets a [`Queue`] - a wrapper around [`Self::raw_get_device_queue_2`].
    pub unsafe fn get_device_queue_2(&self, p_queue_info: *const crate::DeviceQueueInfo2) -> Queue {
        let mut queue = crate::QueueHandle::null();
        unsafe { self.raw_get_device_queue_2(p_queue_info, &mut queue) };

        Queue {
            handle: queue,
            vtable: self.vtable,
        }
    }

    /// Allocates command buffers - a wrapper around [`Self::allocate_command_buffers`].
    pub unsafe fn allocate_command_buffers(
        &self,
        p_allocate_info: *const crate::CommandBufferAllocateInfo,
    ) -> Result<Vec<CommandBuffer>, crate::ErrorCode> {
        let count = unsafe { (*p_allocate_info).command_buffer_count as usize };
        let mut command_buffers = vec![crate::CommandBufferHandle::null(); count];
        unsafe {
            self.raw_allocate_command_buffers(p_allocate_info, command_buffers.as_mut_ptr())
                .success()?
        };

        Ok(command_buffers
            .into_iter()
            .map(|c| CommandBuffer {
                handle: c,
                vtable: self.vtable,
            })
            .collect())
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { self.destroy_device(std::ptr::null()) };
    }
}

pub struct Queue {
    pub(crate) handle: crate::QueueHandle,
    pub(crate) vtable: *const VTable<{ crate::InstanceCommand::VARIANTS.len() }>,
}

impl Queue {
    #[inline(always)]
    pub fn handle(&self) -> crate::QueueHandle {
        self.handle
    }

    #[track_caller]
    #[inline(always)]
    pub(crate) fn vtable(&self) -> &InstanceVTable {
        // SAFETY: user contract - parent device (and instance) must be alive
        unsafe { self.vtable.as_ref_unchecked() }
    }
}

pub struct CommandBuffer {
    pub(crate) handle: crate::CommandBufferHandle,
    pub(crate) vtable: *const VTable<{ crate::InstanceCommand::VARIANTS.len() }>,
}

impl CommandBuffer {
    #[inline(always)]
    pub fn handle(&self) -> crate::CommandBufferHandle {
        self.handle
    }

    #[track_caller]
    #[inline(always)]
    pub(crate) fn vtable(&self) -> &InstanceVTable {
        // SAFETY: user contract - parent instance must be alive
        unsafe { self.vtable.as_ref_unchecked() }
    }
}
