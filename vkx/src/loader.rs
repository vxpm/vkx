//! Manually implemented items.

use std::sync::OnceLock;

use libloading::Library;

pub(crate) type VTable<const N: usize> = [crate::FnVoidFunction; N];

type GlobalVTable = VTable<{ crate::GlobalCommand::VARIANTS.len() }>;
type InstanceVTable = VTable<{ crate::InstanceCommand::VARIANTS.len() }>;
type DeviceVTable = VTable<{ crate::DeviceCommand::VARIANTS.len() }>;

#[inline(always)]
pub(crate) fn vtable_get<const N: usize>(table: &VTable<N>, index: usize) -> crate::FnVoidFunction {
    let func = table[index];

    if cfg!(debug_assertions) {
        let ptr = func as *const crate::FnVoidFunction;
        assert!(!ptr.is_null(), "command should not be null");
    }

    func
}

pub(crate) struct Global {
    pub _lib: Library,
    pub get_instance_proc_addr: crate::FnGetInstanceProcAddr,
    pub get_device_proc_addr: crate::FnGetDeviceProcAddr,
    pub commands: GlobalVTable,
}

pub(crate) static GLOBAL: OnceLock<Global> = OnceLock::new();

/// Setups the loader. This needs to be called before calling anything else in `vkx`, but only once.
///
/// # Panics
/// Panics if the function is called multiple times.
///
/// # Safety
/// This function will dynamically load the Vulkan library and build a global command vtable. This
/// is inherently unsafe (see [`Library::new`]).
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
        *unsafe { lib.get::<crate::FnGetInstanceProcAddr>("vkGetInstanceProcAddr") }?;
    let get_device_proc_addr =
        *unsafe { lib.get::<crate::FnGetDeviceProcAddr>("vkGetDeviceProcAddr") }?;

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
            get_device_proc_addr,
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
/// to call every instance function.
///
/// Child handles of this instance carry a reference to the same vtable, but calling any
/// function on them _after_ this instance is [destroyed](Self::destroy) is _undefined behaviour_.
#[derive(Clone)]
pub struct Instance {
    pub(crate) handle: crate::InstanceHandle,
    pub(crate) vtable: *const InstanceVTable,
}

impl Instance {
    /// Returns the raw [`InstanceHandle`](crate::InstanceHandle) backing this [`Instance`]. Unlike
    /// other methods, this is _always_ safe to call - even if the underlying object has been
    /// destroyed.
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
    ///
    /// # Safety
    /// Same as all other Vulkan functions.
    pub unsafe fn create(
        create_info: *const crate::InstanceCreateInfo,
        allocator: Option<*const crate::AllocationCallbacks>,
    ) -> Result<Self, crate::ErrorCode> {
        let mut instance = crate::InstanceHandle::default();
        unsafe { crate::create_instance(create_info, allocator, &mut instance)? };

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
    ///
    /// # Safety
    /// Same as all other Vulkan functions.
    #[inline(always)]
    pub unsafe fn enumerate_physical_devices(
        &self,
    ) -> Result<Vec<PhysicalDevice>, crate::ErrorCode> {
        let mut count = 0;
        unsafe { self.raw_enumerate_physical_devices(&mut count, None)? };

        let mut devices = vec![crate::PhysicalDeviceHandle::default(); count as usize];
        unsafe { self.raw_enumerate_physical_devices(&mut count, Some(devices.as_mut_ptr()))? };

        Ok(devices
            .into_iter()
            .map(|handle| PhysicalDevice {
                handle,
                vtable: self.vtable,
            })
            .collect())
    }

    /// Destroys this instance. This will destroy the handle _and_ deallocate the vtable.
    ///
    /// # Safety
    /// You must not call any method on a child of this instance or on another handle to this same
    /// instance after this method is called (otherwise it will cause an use-after-free).
    pub unsafe fn destroy(self, allocator: Option<*const crate::AllocationCallbacks>) {
        unsafe {
            self.raw_destroy(allocator);
            std::mem::drop(Box::from_raw(self.vtable.cast_mut()))
        }
    }
}

/// A [`PhysicalDeviceHandle`](crate::PhysicalDeviceHandle) wrapper that carries it's parent
/// [`Instance`]'s vtable.
///
/// This type is like a smart version of a [`PhysicalDeviceHandle`](crate::PhysicalDeviceHandle): it
/// knows how to call every physical device function.
#[derive(Clone)]
pub struct PhysicalDevice {
    pub(crate) handle: crate::PhysicalDeviceHandle,
    pub(crate) vtable: *const InstanceVTable,
}

impl PhysicalDevice {
    /// Returns the raw [`PhysicalDeviceHandle`](crate::PhysicalDeviceHandle) backing this
    /// [`PhysicalDevice`]. Unlike other methods, this is _always_ safe to call - even if the
    /// underlying object has been destroyed.
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
    ///
    /// # Safety
    /// Same as all other Vulkan functions.
    #[inline(always)]
    pub unsafe fn create_device(
        &self,
        create_info: *const crate::DeviceCreateInfo,
        allocator: Option<*const crate::AllocationCallbacks>,
    ) -> Result<Device, crate::ErrorCode> {
        let mut device = crate::DeviceHandle::default();
        unsafe { self.raw_create_device(create_info, allocator, &mut device)? };

        let get_device_proc_addr = GLOBAL
            .get()
            .expect("vkx setup should have been run")
            .get_device_proc_addr;

        let mut device_commands = Vec::with_capacity(crate::DeviceCommand::VARIANTS.len());
        for command in crate::DeviceCommand::VARIANTS {
            let command = unsafe { get_device_proc_addr(device, command.name().as_ptr()) };
            device_commands.push(command);
        }

        let boxed_array: Box<[_; _]> = device_commands.into_boxed_slice().try_into().unwrap();
        let device_commands = Box::leak(boxed_array);

        Ok(Device {
            handle: device,
            vtable: device_commands,
        })
    }
}

/// A [`DeviceHandle`](crate::DeviceHandle) wrapper that carries a vtable generated at creation
/// time.
///
/// This type is like a smart version of a [`DeviceHandle`](crate::DeviceHandle): it knows how
/// to call every device function.
///
/// Child handles of this instance carry a reference to the same vtable, but calling any
/// function on them _after_ this device is destroyed is an use-after-free (UB).
#[derive(Clone)]
pub struct Device {
    pub(crate) handle: crate::DeviceHandle,
    pub(crate) vtable: *const DeviceVTable,
}

impl Device {
    /// Returns the raw [`DeviceHandle`](crate::DeviceHandle) backing this [`Device`]. Unlike other
    /// methods, this is _always_ safe to call - even if the underlying object has been destroyed.
    #[inline(always)]
    pub fn handle(&self) -> crate::DeviceHandle {
        self.handle
    }

    #[inline(always)]
    pub(crate) fn vtable(&self) -> &DeviceVTable {
        // SAFETY: user contract - parent instance must be alive
        unsafe { self.vtable.as_ref_unchecked() }
    }

    /// Gets a [`Queue`] - a wrapper around [`Self::raw_get_device_queue`].
    ///
    /// # Safety
    /// Same as all other Vulkan functions.
    pub unsafe fn get_device_queue(&self, queue_family_index: u32, queue_index: u32) -> Queue {
        let mut queue = crate::QueueHandle::null();
        unsafe { self.raw_get_device_queue(queue_family_index, queue_index, &mut queue) };

        Queue {
            handle: queue,
            vtable: self.vtable,
        }
    }

    /// Gets a [`Queue`] - a wrapper around [`Self::raw_get_device_queue_2`].
    ///
    /// # Safety
    /// Same as all other Vulkan functions.
    pub unsafe fn get_device_queue_2(&self, p_queue_info: *const crate::DeviceQueueInfo2) -> Queue {
        let mut queue = crate::QueueHandle::null();
        unsafe { self.raw_get_device_queue_2(p_queue_info, &mut queue) };

        Queue {
            handle: queue,
            vtable: self.vtable,
        }
    }

    /// Allocates command buffers - a wrapper around [`Self::allocate_command_buffers`].
    ///
    /// # Safety
    /// Same as all other Vulkan functions.
    pub unsafe fn allocate_command_buffers(
        &self,
        p_allocate_info: *const crate::CommandBufferAllocateInfo,
    ) -> Result<Vec<CommandBuffer>, crate::ErrorCode> {
        let count = unsafe { (*p_allocate_info).command_buffer_count as usize };
        let mut command_buffers = vec![crate::CommandBufferHandle::null(); count];
        unsafe {
            self.raw_allocate_command_buffers(p_allocate_info, command_buffers.as_mut_ptr())?
        };

        Ok(command_buffers
            .into_iter()
            .map(|c| CommandBuffer {
                handle: c,
                vtable: self.vtable,
            })
            .collect())
    }

    /// Destroys this device. This will destroy the handle _and_ deallocate the vtable.
    ///
    /// # Safety
    /// You must not call any method on a child of this device or on another handle to this same
    /// device after this method is called (otherwise it will cause an use-after-free).
    pub unsafe fn destroy(self, allocator: Option<*const crate::AllocationCallbacks>) {
        unsafe {
            self.raw_destroy_device(allocator);
            std::mem::drop(Box::from_raw(self.vtable.cast_mut()))
        }
    }
}

/// A [`QueueHandle`](crate::QueueHandle) wrapper that carries it's parent [`Device`]'s vtable.
///
/// This type is like a smart version of a [`QueueHandle`](crate::QueueHandle): it knows how
/// to call every queue function.
#[derive(Clone)]
pub struct Queue {
    pub(crate) handle: crate::QueueHandle,
    pub(crate) vtable: *const DeviceVTable,
}

impl Queue {
    /// Returns the raw [`QueueHandle`](crate::QueueHandle) backing this [`Queue`]. Unlike other
    /// methods, this is _always_ safe to call - even if the underlying object has been destroyed.
    #[inline(always)]
    pub fn handle(&self) -> crate::QueueHandle {
        self.handle
    }

    #[track_caller]
    #[inline(always)]
    pub(crate) fn vtable(&self) -> &DeviceVTable {
        // SAFETY: user contract - parent device (and instance) must be alive
        unsafe { self.vtable.as_ref_unchecked() }
    }
}

/// A [`CommandBufferHandle`](crate::CommandBufferHandle) wrapper that carries it's parent
/// [`Device`]'s vtable.
///
/// This type is like a smart version of a [`CommandBufferHandle`](crate::CommandBufferHandle): it
/// knows how to call every command buffer function.
#[derive(Clone)]
pub struct CommandBuffer {
    pub(crate) handle: crate::CommandBufferHandle,
    pub(crate) vtable: *const DeviceVTable,
}

impl CommandBuffer {
    /// Returns the raw [`CommandBufferHandle`](crate::CommandBufferHandle) backing this
    /// [`CommandBuffer`]. Unlike other methods, this is _always_ safe to call - even if the
    /// underlying object has been destroyed.
    #[inline(always)]
    pub fn handle(&self) -> crate::CommandBufferHandle {
        self.handle
    }

    #[track_caller]
    #[inline(always)]
    pub(crate) fn vtable(&self) -> &DeviceVTable {
        // SAFETY: user contract - parent instance must be alive
        unsafe { self.vtable.as_ref_unchecked() }
    }
}
