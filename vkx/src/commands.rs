// WARNING: AUTO GENERATED MODULE
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::manual::*;
use crate::platform::*;
use std::ffi::{c_char, c_int, c_uint, c_void};

use crate::bitmasks::*;
use crate::consts_inner::*;
use crate::enums::*;
use crate::flags::*;
use crate::fn_ptrs::*;
use crate::handles::*;
use crate::structs::*;
/// `vkCreateInstance`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateInstance.html>
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
/// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
/// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
/// - [`LAYER_NOT_PRESENT`](ResultCode::ERROR_LAYER_NOT_PRESENT)
/// - [`EXTENSION_NOT_PRESENT`](ResultCode::ERROR_EXTENSION_NOT_PRESENT)
/// - [`INCOMPATIBLE_DRIVER`](ResultCode::ERROR_INCOMPATIBLE_DRIVER)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkCreateInstance")]
pub unsafe fn create_instance(
    create_info: *const InstanceCreateInfo,
    allocator: *const AllocationCallbacks,
    instance: *mut Instance,
) -> ResultCode {
    todo!()
}

impl Instance {
    /// `vkDestroyInstance`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyInstance.html>
    ///
    #[doc(alias = "vkDestroyInstance")]
    pub unsafe fn destroy_instance(self, allocator: *const AllocationCallbacks) {
        todo!()
    }
}

impl Instance {
    /// `vkEnumeratePhysicalDevices`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDevices.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkEnumeratePhysicalDevices")]
    pub unsafe fn enumerate_physical_devices(
        self,
        physical_device_count: *mut u32,
        physical_devices: *mut PhysicalDevice,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceFeatures`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceFeatures")]
    pub unsafe fn get_physical_device_features(self, features: *mut PhysicalDeviceFeatures) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceFormatProperties`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceFormatProperties")]
    pub unsafe fn get_physical_device_format_properties(
        self,
        format: Format,
        format_properties: *mut FormatProperties,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceImageFormatProperties`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`FORMAT_NOT_SUPPORTED`](ResultCode::ERROR_FORMAT_NOT_SUPPORTED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceImageFormatProperties")]
    pub unsafe fn get_physical_device_image_format_properties(
        self,
        format: Format,
        type_: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        image_format_properties: *mut ImageFormatProperties,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceProperties`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceProperties")]
    pub unsafe fn get_physical_device_properties(self, properties: *mut PhysicalDeviceProperties) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceQueueFamilyProperties`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties")]
    pub unsafe fn get_physical_device_queue_family_properties(
        self,
        queue_family_property_count: *mut u32,
        queue_family_properties: *mut QueueFamilyProperties,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceMemoryProperties`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceMemoryProperties")]
    pub unsafe fn get_physical_device_memory_properties(
        self,
        memory_properties: *mut PhysicalDeviceMemoryProperties,
    ) {
        todo!()
    }
}

impl Instance {
    /// `vkGetInstanceProcAddr`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetInstanceProcAddr.html>
    ///
    #[doc(alias = "vkGetInstanceProcAddr")]
    pub unsafe fn get_instance_proc_addr(self, name: *const c_char) -> vkVoidFunction {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceProcAddr`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceProcAddr.html>
    ///
    #[doc(alias = "vkGetDeviceProcAddr")]
    pub unsafe fn get_device_proc_addr(self, name: *const c_char) -> vkVoidFunction {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkCreateDevice`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDevice.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`EXTENSION_NOT_PRESENT`](ResultCode::ERROR_EXTENSION_NOT_PRESENT)
    /// - [`FEATURE_NOT_PRESENT`](ResultCode::ERROR_FEATURE_NOT_PRESENT)
    /// - [`TOO_MANY_OBJECTS`](ResultCode::ERROR_TOO_MANY_OBJECTS)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateDevice")]
    pub unsafe fn create_device(
        self,
        create_info: *const DeviceCreateInfo,
        allocator: *const AllocationCallbacks,
        device: *mut Device,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyDevice`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDevice.html>
    ///
    #[doc(alias = "vkDestroyDevice")]
    pub unsafe fn destroy_device(self, allocator: *const AllocationCallbacks) {
        todo!()
    }
}

/// `vkEnumerateInstanceExtensionProperties`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceExtensionProperties.html>
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
/// ## Error
/// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
/// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
/// - [`LAYER_NOT_PRESENT`](ResultCode::ERROR_LAYER_NOT_PRESENT)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkEnumerateInstanceExtensionProperties")]
pub unsafe fn enumerate_instance_extension_properties(
    layer_name: *const c_char,
    property_count: *mut u32,
    properties: *mut ExtensionProperties,
) -> ResultCode {
    todo!()
}

impl PhysicalDevice {
    /// `vkEnumerateDeviceExtensionProperties`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceExtensionProperties.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`LAYER_NOT_PRESENT`](ResultCode::ERROR_LAYER_NOT_PRESENT)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkEnumerateDeviceExtensionProperties")]
    pub unsafe fn enumerate_device_extension_properties(
        self,
        layer_name: *const c_char,
        property_count: *mut u32,
        properties: *mut ExtensionProperties,
    ) -> ResultCode {
        todo!()
    }
}

/// `vkEnumerateInstanceLayerProperties`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceLayerProperties.html>
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
/// ## Error
/// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
/// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkEnumerateInstanceLayerProperties")]
pub unsafe fn enumerate_instance_layer_properties(
    property_count: *mut u32,
    properties: *mut LayerProperties,
) -> ResultCode {
    todo!()
}

impl PhysicalDevice {
    /// `vkEnumerateDeviceLayerProperties`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceLayerProperties.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkEnumerateDeviceLayerProperties")]
    pub unsafe fn enumerate_device_layer_properties(
        self,
        property_count: *mut u32,
        properties: *mut LayerProperties,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceQueue`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue.html>
    ///
    #[doc(alias = "vkGetDeviceQueue")]
    pub unsafe fn get_device_queue(
        self,
        queue_family_index: u32,
        queue_index: u32,
        queue: *mut Queue,
    ) {
        todo!()
    }
}

impl Queue {
    /// `vkQueueSubmit`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkQueueSubmit")]
    pub unsafe fn queue_submit(
        self,
        submit_count: u32,
        submits: *const SubmitInfo,
        fence: Fence,
    ) -> ResultCode {
        todo!()
    }
}

impl Queue {
    /// `vkQueueWaitIdle`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueWaitIdle.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkQueueWaitIdle")]
    pub unsafe fn queue_wait_idle(self) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDeviceWaitIdle`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDeviceWaitIdle.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkDeviceWaitIdle")]
    pub unsafe fn device_wait_idle(self) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkAllocateMemory`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateMemory.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INVALID_EXTERNAL_HANDLE`](ResultCode::ERROR_INVALID_EXTERNAL_HANDLE)
    /// - [`INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`](ResultCode::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkAllocateMemory")]
    pub unsafe fn allocate_memory(
        self,
        allocate_info: *const MemoryAllocateInfo,
        allocator: *const AllocationCallbacks,
        memory: *mut DeviceMemory,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkFreeMemory`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeMemory.html>
    ///
    #[doc(alias = "vkFreeMemory")]
    pub unsafe fn free_memory(self, memory: DeviceMemory, allocator: *const AllocationCallbacks) {
        todo!()
    }
}

impl Device {
    /// `vkMapMemory`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`MEMORY_MAP_FAILED`](ResultCode::ERROR_MEMORY_MAP_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkMapMemory")]
    pub unsafe fn map_memory(
        self,
        memory: DeviceMemory,
        offset: DeviceSize,
        size: DeviceSize,
        flags: MemoryMapFlags,
        data: *mut *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkUnmapMemory`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory.html>
    ///
    #[doc(alias = "vkUnmapMemory")]
    pub unsafe fn unmap_memory(self, memory: DeviceMemory) {
        todo!()
    }
}

impl Device {
    /// `vkFlushMappedMemoryRanges`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkFlushMappedMemoryRanges.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkFlushMappedMemoryRanges")]
    pub unsafe fn flush_mapped_memory_ranges(
        self,
        memory_range_count: u32,
        memory_ranges: *const MappedMemoryRange,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkInvalidateMappedMemoryRanges`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkInvalidateMappedMemoryRanges.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkInvalidateMappedMemoryRanges")]
    pub unsafe fn invalidate_mapped_memory_ranges(
        self,
        memory_range_count: u32,
        memory_ranges: *const MappedMemoryRange,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceMemoryCommitment`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryCommitment.html>
    ///
    #[doc(alias = "vkGetDeviceMemoryCommitment")]
    pub unsafe fn get_device_memory_commitment(
        self,
        memory: DeviceMemory,
        committed_memory_in_bytes: *mut DeviceSize,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkBindBufferMemory`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`](ResultCode::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkBindBufferMemory")]
    pub unsafe fn bind_buffer_memory(
        self,
        buffer: Buffer,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkBindImageMemory`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkBindImageMemory")]
    pub unsafe fn bind_image_memory(
        self,
        image: Image,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetBufferMemoryRequirements`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements.html>
    ///
    #[doc(alias = "vkGetBufferMemoryRequirements")]
    pub unsafe fn get_buffer_memory_requirements(
        self,
        buffer: Buffer,
        memory_requirements: *mut MemoryRequirements,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetImageMemoryRequirements`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements.html>
    ///
    #[doc(alias = "vkGetImageMemoryRequirements")]
    pub unsafe fn get_image_memory_requirements(
        self,
        image: Image,
        memory_requirements: *mut MemoryRequirements,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetImageSparseMemoryRequirements`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements.html>
    ///
    #[doc(alias = "vkGetImageSparseMemoryRequirements")]
    pub unsafe fn get_image_sparse_memory_requirements(
        self,
        image: Image,
        sparse_memory_requirement_count: *mut u32,
        sparse_memory_requirements: *mut SparseImageMemoryRequirements,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceSparseImageFormatProperties`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties")]
    pub unsafe fn get_physical_device_sparse_image_format_properties(
        self,
        format: Format,
        type_: ImageType,
        samples: SampleCountFlags,
        usage: ImageUsageFlags,
        tiling: ImageTiling,
        property_count: *mut u32,
        properties: *mut SparseImageFormatProperties,
    ) {
        todo!()
    }
}

impl Queue {
    /// `vkQueueBindSparse`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBindSparse.html>
    ///
    /// ## Allowed queues
    /// - [`SPARSE_BINDING`](QueueFlags::SPARSE_BINDING)
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkQueueBindSparse")]
    pub unsafe fn queue_bind_sparse(
        self,
        bind_info_count: u32,
        bind_info: *const BindSparseInfo,
        fence: Fence,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCreateFence`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateFence.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateFence")]
    pub unsafe fn create_fence(
        self,
        create_info: *const FenceCreateInfo,
        allocator: *const AllocationCallbacks,
        fence: *mut Fence,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyFence`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFence.html>
    ///
    #[doc(alias = "vkDestroyFence")]
    pub unsafe fn destroy_fence(self, fence: Fence, allocator: *const AllocationCallbacks) {
        todo!()
    }
}

impl Device {
    /// `vkResetFences`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkResetFences.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkResetFences")]
    pub unsafe fn reset_fences(self, fence_count: u32, fences: *const Fence) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetFenceStatus`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceStatus.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`NOT_READY`](ResultCode::NOT_READY)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetFenceStatus")]
    pub unsafe fn get_fence_status(self, fence: Fence) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkWaitForFences`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForFences.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`TIMEOUT`](ResultCode::TIMEOUT)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkWaitForFences")]
    pub unsafe fn wait_for_fences(
        self,
        fence_count: u32,
        fences: *const Fence,
        wait_all: Bool32,
        timeout: u64,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCreateSemaphore`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSemaphore.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateSemaphore")]
    pub unsafe fn create_semaphore(
        self,
        create_info: *const SemaphoreCreateInfo,
        allocator: *const AllocationCallbacks,
        semaphore: *mut Semaphore,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroySemaphore`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySemaphore.html>
    ///
    #[doc(alias = "vkDestroySemaphore")]
    pub unsafe fn destroy_semaphore(
        self,
        semaphore: Semaphore,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateQueryPool`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateQueryPool.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateQueryPool")]
    pub unsafe fn create_query_pool(
        self,
        create_info: *const QueryPoolCreateInfo,
        allocator: *const AllocationCallbacks,
        query_pool: *mut QueryPool,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyQueryPool`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyQueryPool.html>
    ///
    #[doc(alias = "vkDestroyQueryPool")]
    pub unsafe fn destroy_query_pool(
        self,
        query_pool: QueryPool,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetQueryPoolResults`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueryPoolResults.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`NOT_READY`](ResultCode::NOT_READY)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetQueryPoolResults")]
    pub unsafe fn get_query_pool_results(
        self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        data_size: usize,
        data: *mut c_void,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCreateBuffer`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBuffer.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`](ResultCode::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateBuffer")]
    pub unsafe fn create_buffer(
        self,
        create_info: *const BufferCreateInfo,
        allocator: *const AllocationCallbacks,
        buffer: *mut Buffer,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyBuffer`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBuffer.html>
    ///
    #[doc(alias = "vkDestroyBuffer")]
    pub unsafe fn destroy_buffer(self, buffer: Buffer, allocator: *const AllocationCallbacks) {
        todo!()
    }
}

impl Device {
    /// `vkCreateImage`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImage.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`COMPRESSION_EXHAUSTED_EXT`](ResultCode::ERROR_COMPRESSION_EXHAUSTED_EXT)
    /// - [`INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`](ResultCode::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateImage")]
    pub unsafe fn create_image(
        self,
        create_info: *const ImageCreateInfo,
        allocator: *const AllocationCallbacks,
        image: *mut Image,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyImage`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImage.html>
    ///
    #[doc(alias = "vkDestroyImage")]
    pub unsafe fn destroy_image(self, image: Image, allocator: *const AllocationCallbacks) {
        todo!()
    }
}

impl Device {
    /// `vkGetImageSubresourceLayout`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout.html>
    ///
    #[doc(alias = "vkGetImageSubresourceLayout")]
    pub unsafe fn get_image_subresource_layout(
        self,
        image: Image,
        subresource: *const ImageSubresource,
        layout: *mut SubresourceLayout,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateImageView`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImageView.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`](ResultCode::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateImageView")]
    pub unsafe fn create_image_view(
        self,
        create_info: *const ImageViewCreateInfo,
        allocator: *const AllocationCallbacks,
        view: *mut ImageView,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyImageView`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImageView.html>
    ///
    #[doc(alias = "vkDestroyImageView")]
    pub unsafe fn destroy_image_view(
        self,
        image_view: ImageView,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateCommandPool`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCommandPool.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateCommandPool")]
    pub unsafe fn create_command_pool(
        self,
        create_info: *const CommandPoolCreateInfo,
        allocator: *const AllocationCallbacks,
        command_pool: *mut CommandPool,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyCommandPool`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCommandPool.html>
    ///
    #[doc(alias = "vkDestroyCommandPool")]
    pub unsafe fn destroy_command_pool(
        self,
        command_pool: CommandPool,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkResetCommandPool`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandPool.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkResetCommandPool")]
    pub unsafe fn reset_command_pool(
        self,
        command_pool: CommandPool,
        flags: CommandPoolResetFlags,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkAllocateCommandBuffers`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateCommandBuffers.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkAllocateCommandBuffers")]
    pub unsafe fn allocate_command_buffers(
        self,
        allocate_info: *const CommandBufferAllocateInfo,
        command_buffers: *mut CommandBuffer,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkFreeCommandBuffers`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeCommandBuffers.html>
    ///
    #[doc(alias = "vkFreeCommandBuffers")]
    pub unsafe fn free_command_buffers(
        self,
        command_pool: CommandPool,
        command_buffer_count: u32,
        command_buffers: *const CommandBuffer,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkBeginCommandBuffer`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBeginCommandBuffer.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkBeginCommandBuffer")]
    pub unsafe fn begin_command_buffer(
        self,
        begin_info: *const CommandBufferBeginInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkEndCommandBuffer`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEndCommandBuffer.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INVALID_VIDEO_STD_PARAMETERS_KHR`](ResultCode::ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkEndCommandBuffer")]
    pub unsafe fn end_command_buffer(self) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkResetCommandBuffer`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandBuffer.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkResetCommandBuffer")]
    pub unsafe fn reset_command_buffer(self, flags: CommandBufferResetFlags) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyBuffer`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyBuffer")]
    pub unsafe fn cmd_copy_buffer(
        self,
        src_buffer: Buffer,
        dst_buffer: Buffer,
        region_count: u32,
        regions: *const BufferCopy,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyImage`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyImage")]
    pub unsafe fn cmd_copy_image(
        self,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        regions: *const ImageCopy,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyBufferToImage`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyBufferToImage")]
    pub unsafe fn cmd_copy_buffer_to_image(
        self,
        src_buffer: Buffer,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        regions: *const BufferImageCopy,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyImageToBuffer`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyImageToBuffer")]
    pub unsafe fn cmd_copy_image_to_buffer(
        self,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_buffer: Buffer,
        region_count: u32,
        regions: *const BufferImageCopy,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdUpdateBuffer`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdateBuffer.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdUpdateBuffer")]
    pub unsafe fn cmd_update_buffer(
        self,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        data_size: DeviceSize,
        data: *const c_void,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdFillBuffer`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdFillBuffer.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdFillBuffer")]
    pub unsafe fn cmd_fill_buffer(
        self,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        size: DeviceSize,
        data: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdPipelineBarrier`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier.html>
    ///
    /// ## Performed tasks
    /// - `synchronization`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    ///
    #[doc(alias = "vkCmdPipelineBarrier")]
    pub unsafe fn cmd_pipeline_barrier(
        self,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        dependency_flags: DependencyFlags,
        memory_barrier_count: u32,
        memory_barriers: *const MemoryBarrier,
        buffer_memory_barrier_count: u32,
        buffer_memory_barriers: *const BufferMemoryBarrier,
        image_memory_barrier_count: u32,
        image_memory_barriers: *const ImageMemoryBarrier,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBeginQuery`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginQuery.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    ///
    #[doc(alias = "vkCmdBeginQuery")]
    pub unsafe fn cmd_begin_query(
        self,
        query_pool: QueryPool,
        query: u32,
        flags: QueryControlFlags,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdEndQuery`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndQuery.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    ///
    #[doc(alias = "vkCmdEndQuery")]
    pub unsafe fn cmd_end_query(self, query_pool: QueryPool, query: u32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdResetQueryPool`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetQueryPool.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    /// - [`OPTICAL_FLOWNV`](QueueFlags::OPTICAL_FLOWNV)
    ///
    #[doc(alias = "vkCmdResetQueryPool")]
    pub unsafe fn cmd_reset_query_pool(
        self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdWriteTimestamp`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    /// - [`OPTICAL_FLOWNV`](QueueFlags::OPTICAL_FLOWNV)
    ///
    #[doc(alias = "vkCmdWriteTimestamp")]
    pub unsafe fn cmd_write_timestamp(
        self,
        pipeline_stage: PipelineStageFlags,
        query_pool: QueryPool,
        query: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyQueryPoolResults`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyQueryPoolResults.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyQueryPoolResults")]
    pub unsafe fn cmd_copy_query_pool_results(
        self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdExecuteCommands`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteCommands.html>
    ///
    /// ## Performed tasks
    /// - `indirection`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdExecuteCommands")]
    pub unsafe fn cmd_execute_commands(
        self,
        command_buffer_count: u32,
        command_buffers: *const CommandBuffer,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateEvent`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateEvent.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateEvent")]
    pub unsafe fn create_event(
        self,
        create_info: *const EventCreateInfo,
        allocator: *const AllocationCallbacks,
        event: *mut Event,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyEvent`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyEvent.html>
    ///
    #[doc(alias = "vkDestroyEvent")]
    pub unsafe fn destroy_event(self, event: Event, allocator: *const AllocationCallbacks) {
        todo!()
    }
}

impl Device {
    /// `vkGetEventStatus`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetEventStatus.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`EVENT_SET`](ResultCode::EVENT_SET)
    /// - [`EVENT_RESET`](ResultCode::EVENT_RESET)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetEventStatus")]
    pub unsafe fn get_event_status(self, event: Event) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkSetEvent`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetEvent.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkSetEvent")]
    pub unsafe fn set_event(self, event: Event) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkResetEvent`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkResetEvent.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkResetEvent")]
    pub unsafe fn reset_event(self, event: Event) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCreateBufferView`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBufferView.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateBufferView")]
    pub unsafe fn create_buffer_view(
        self,
        create_info: *const BufferViewCreateInfo,
        allocator: *const AllocationCallbacks,
        view: *mut BufferView,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyBufferView`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferView.html>
    ///
    #[doc(alias = "vkDestroyBufferView")]
    pub unsafe fn destroy_buffer_view(
        self,
        buffer_view: BufferView,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateShaderModule`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShaderModule.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INVALID_SHADER_NV`](ResultCode::ERROR_INVALID_SHADER_NV)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateShaderModule")]
    pub unsafe fn create_shader_module(
        self,
        create_info: *const ShaderModuleCreateInfo,
        allocator: *const AllocationCallbacks,
        shader_module: *mut ShaderModule,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyShaderModule`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderModule.html>
    ///
    #[doc(alias = "vkDestroyShaderModule")]
    pub unsafe fn destroy_shader_module(
        self,
        shader_module: ShaderModule,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreatePipelineCache`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineCache.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreatePipelineCache")]
    pub unsafe fn create_pipeline_cache(
        self,
        create_info: *const PipelineCacheCreateInfo,
        allocator: *const AllocationCallbacks,
        pipeline_cache: *mut PipelineCache,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyPipelineCache`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineCache.html>
    ///
    #[doc(alias = "vkDestroyPipelineCache")]
    pub unsafe fn destroy_pipeline_cache(
        self,
        pipeline_cache: PipelineCache,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetPipelineCacheData`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineCacheData.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPipelineCacheData")]
    pub unsafe fn get_pipeline_cache_data(
        self,
        pipeline_cache: PipelineCache,
        data_size: *mut usize,
        data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkMergePipelineCaches`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkMergePipelineCaches.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkMergePipelineCaches")]
    pub unsafe fn merge_pipeline_caches(
        self,
        dst_cache: PipelineCache,
        src_cache_count: u32,
        src_caches: *const PipelineCache,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCreateComputePipelines`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateComputePipelines.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`PIPELINE_COMPILE_REQUIRED_EXT`](ResultCode::PIPELINE_COMPILE_REQUIRED_EXT)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INVALID_SHADER_NV`](ResultCode::ERROR_INVALID_SHADER_NV)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateComputePipelines")]
    pub unsafe fn create_compute_pipelines(
        self,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        create_infos: *const ComputePipelineCreateInfo,
        allocator: *const AllocationCallbacks,
        pipelines: *mut Pipeline,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyPipeline`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipeline.html>
    ///
    #[doc(alias = "vkDestroyPipeline")]
    pub unsafe fn destroy_pipeline(
        self,
        pipeline: Pipeline,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreatePipelineLayout`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineLayout.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreatePipelineLayout")]
    pub unsafe fn create_pipeline_layout(
        self,
        create_info: *const PipelineLayoutCreateInfo,
        allocator: *const AllocationCallbacks,
        pipeline_layout: *mut PipelineLayout,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyPipelineLayout`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineLayout.html>
    ///
    #[doc(alias = "vkDestroyPipelineLayout")]
    pub unsafe fn destroy_pipeline_layout(
        self,
        pipeline_layout: PipelineLayout,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateSampler`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSampler.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`](ResultCode::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateSampler")]
    pub unsafe fn create_sampler(
        self,
        create_info: *const SamplerCreateInfo,
        allocator: *const AllocationCallbacks,
        sampler: *mut Sampler,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroySampler`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySampler.html>
    ///
    #[doc(alias = "vkDestroySampler")]
    pub unsafe fn destroy_sampler(self, sampler: Sampler, allocator: *const AllocationCallbacks) {
        todo!()
    }
}

impl Device {
    /// `vkCreateDescriptorSetLayout`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorSetLayout.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateDescriptorSetLayout")]
    pub unsafe fn create_descriptor_set_layout(
        self,
        create_info: *const DescriptorSetLayoutCreateInfo,
        allocator: *const AllocationCallbacks,
        set_layout: *mut DescriptorSetLayout,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyDescriptorSetLayout`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorSetLayout.html>
    ///
    #[doc(alias = "vkDestroyDescriptorSetLayout")]
    pub unsafe fn destroy_descriptor_set_layout(
        self,
        descriptor_set_layout: DescriptorSetLayout,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateDescriptorPool`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorPool.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`FRAGMENTATION_EXT`](ResultCode::ERROR_FRAGMENTATION_EXT)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateDescriptorPool")]
    pub unsafe fn create_descriptor_pool(
        self,
        create_info: *const DescriptorPoolCreateInfo,
        allocator: *const AllocationCallbacks,
        descriptor_pool: *mut DescriptorPool,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyDescriptorPool`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorPool.html>
    ///
    #[doc(alias = "vkDestroyDescriptorPool")]
    pub unsafe fn destroy_descriptor_pool(
        self,
        descriptor_pool: DescriptorPool,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkResetDescriptorPool`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkResetDescriptorPool.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkResetDescriptorPool")]
    pub unsafe fn reset_descriptor_pool(
        self,
        descriptor_pool: DescriptorPool,
        flags: DescriptorPoolResetFlags,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkAllocateDescriptorSets`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateDescriptorSets.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`FRAGMENTED_POOL`](ResultCode::ERROR_FRAGMENTED_POOL)
    /// - [`OUT_OF_POOL_MEMORY`](ResultCode::ERROR_OUT_OF_POOL_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkAllocateDescriptorSets")]
    pub unsafe fn allocate_descriptor_sets(
        self,
        allocate_info: *const DescriptorSetAllocateInfo,
        descriptor_sets: *mut DescriptorSet,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkFreeDescriptorSets`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeDescriptorSets.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkFreeDescriptorSets")]
    pub unsafe fn free_descriptor_sets(
        self,
        descriptor_pool: DescriptorPool,
        descriptor_set_count: u32,
        descriptor_sets: *const DescriptorSet,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkUpdateDescriptorSets`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSets.html>
    ///
    #[doc(alias = "vkUpdateDescriptorSets")]
    pub unsafe fn update_descriptor_sets(
        self,
        descriptor_write_count: u32,
        descriptor_writes: *const WriteDescriptorSet,
        descriptor_copy_count: u32,
        descriptor_copies: *const CopyDescriptorSet,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBindPipeline`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindPipeline.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`DATA_GRAPHARM`](QueueFlags::DATA_GRAPHARM)
    ///
    #[doc(alias = "vkCmdBindPipeline")]
    pub unsafe fn cmd_bind_pipeline(
        self,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBindDescriptorSets`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`DATA_GRAPHARM`](QueueFlags::DATA_GRAPHARM)
    ///
    #[doc(alias = "vkCmdBindDescriptorSets")]
    pub unsafe fn cmd_bind_descriptor_sets(
        self,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        descriptor_set_count: u32,
        descriptor_sets: *const DescriptorSet,
        dynamic_offset_count: u32,
        dynamic_offsets: *const u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdClearColorImage`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearColorImage.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdClearColorImage")]
    pub unsafe fn cmd_clear_color_image(
        self,
        image: Image,
        image_layout: ImageLayout,
        color: *const ClearColorValue,
        range_count: u32,
        ranges: *const ImageSubresourceRange,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDispatch`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatch.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdDispatch")]
    pub unsafe fn cmd_dispatch(self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDispatchIndirect`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchIndirect.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdDispatchIndirect")]
    pub unsafe fn cmd_dispatch_indirect(self, buffer: Buffer, offset: DeviceSize) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetEvent`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent.html>
    ///
    /// ## Performed tasks
    /// - `synchronization`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    ///
    #[doc(alias = "vkCmdSetEvent")]
    pub unsafe fn cmd_set_event(self, event: Event, stage_mask: PipelineStageFlags) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdResetEvent`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent.html>
    ///
    /// ## Performed tasks
    /// - `synchronization`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    ///
    #[doc(alias = "vkCmdResetEvent")]
    pub unsafe fn cmd_reset_event(self, event: Event, stage_mask: PipelineStageFlags) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdWaitEvents`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents.html>
    ///
    /// ## Performed tasks
    /// - `synchronization`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    ///
    #[doc(alias = "vkCmdWaitEvents")]
    pub unsafe fn cmd_wait_events(
        self,
        event_count: u32,
        events: *const Event,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        memory_barrier_count: u32,
        memory_barriers: *const MemoryBarrier,
        buffer_memory_barrier_count: u32,
        buffer_memory_barriers: *const BufferMemoryBarrier,
        image_memory_barrier_count: u32,
        image_memory_barriers: *const ImageMemoryBarrier,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdPushConstants`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdPushConstants")]
    pub unsafe fn cmd_push_constants(
        self,
        layout: PipelineLayout,
        stage_flags: ShaderStageFlags,
        offset: u32,
        size: u32,
        values: *const c_void,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateGraphicsPipelines`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateGraphicsPipelines.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`PIPELINE_COMPILE_REQUIRED_EXT`](ResultCode::PIPELINE_COMPILE_REQUIRED_EXT)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INVALID_SHADER_NV`](ResultCode::ERROR_INVALID_SHADER_NV)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateGraphicsPipelines")]
    pub unsafe fn create_graphics_pipelines(
        self,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        create_infos: *const GraphicsPipelineCreateInfo,
        allocator: *const AllocationCallbacks,
        pipelines: *mut Pipeline,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCreateFramebuffer`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateFramebuffer.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateFramebuffer")]
    pub unsafe fn create_framebuffer(
        self,
        create_info: *const FramebufferCreateInfo,
        allocator: *const AllocationCallbacks,
        framebuffer: *mut Framebuffer,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyFramebuffer`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFramebuffer.html>
    ///
    #[doc(alias = "vkDestroyFramebuffer")]
    pub unsafe fn destroy_framebuffer(
        self,
        framebuffer: Framebuffer,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateRenderPass`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateRenderPass")]
    pub unsafe fn create_render_pass(
        self,
        create_info: *const RenderPassCreateInfo,
        allocator: *const AllocationCallbacks,
        render_pass: *mut RenderPass,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyRenderPass`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyRenderPass.html>
    ///
    #[doc(alias = "vkDestroyRenderPass")]
    pub unsafe fn destroy_render_pass(
        self,
        render_pass: RenderPass,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetRenderAreaGranularity`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderAreaGranularity.html>
    ///
    #[doc(alias = "vkGetRenderAreaGranularity")]
    pub unsafe fn get_render_area_granularity(
        self,
        render_pass: RenderPass,
        granularity: *mut Extent2D,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetViewport`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewport.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetViewport")]
    pub unsafe fn cmd_set_viewport(
        self,
        first_viewport: u32,
        viewport_count: u32,
        viewports: *const Viewport,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetScissor`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissor.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetScissor")]
    pub unsafe fn cmd_set_scissor(
        self,
        first_scissor: u32,
        scissor_count: u32,
        scissors: *const Rect2D,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetLineWidth`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineWidth.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetLineWidth")]
    pub unsafe fn cmd_set_line_width(self, line_width: f32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDepthBias`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetDepthBias")]
    pub unsafe fn cmd_set_depth_bias(
        self,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetBlendConstants`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetBlendConstants.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetBlendConstants")]
    pub unsafe fn cmd_set_blend_constants(self, blend_constants: *const [f32; 4 as usize]) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDepthBounds`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBounds.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetDepthBounds")]
    pub unsafe fn cmd_set_depth_bounds(self, min_depth_bounds: f32, max_depth_bounds: f32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetStencilCompareMask`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilCompareMask.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetStencilCompareMask")]
    pub unsafe fn cmd_set_stencil_compare_mask(
        self,
        face_mask: StencilFaceFlags,
        compare_mask: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetStencilWriteMask`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilWriteMask.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetStencilWriteMask")]
    pub unsafe fn cmd_set_stencil_write_mask(self, face_mask: StencilFaceFlags, write_mask: u32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetStencilReference`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilReference.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetStencilReference")]
    pub unsafe fn cmd_set_stencil_reference(self, face_mask: StencilFaceFlags, reference: u32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBindIndexBuffer`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdBindIndexBuffer")]
    pub unsafe fn cmd_bind_index_buffer(
        self,
        buffer: Buffer,
        offset: DeviceSize,
        index_type: IndexType,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBindVertexBuffers`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdBindVertexBuffers")]
    pub unsafe fn cmd_bind_vertex_buffers(
        self,
        first_binding: u32,
        binding_count: u32,
        buffers: *const Buffer,
        offsets: *const DeviceSize,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDraw`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDraw.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDraw")]
    pub unsafe fn cmd_draw(
        self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawIndexed`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexed.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawIndexed")]
    pub unsafe fn cmd_draw_indexed(
        self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawIndirect`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirect.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawIndirect")]
    pub unsafe fn cmd_draw_indirect(
        self,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawIndexedIndirect`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirect.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawIndexedIndirect")]
    pub unsafe fn cmd_draw_indexed_indirect(
        self,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBlitImage`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdBlitImage")]
    pub unsafe fn cmd_blit_image(
        self,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        regions: *const ImageBlit,
        filter: Filter,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdClearDepthStencilImage`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearDepthStencilImage.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdClearDepthStencilImage")]
    pub unsafe fn cmd_clear_depth_stencil_image(
        self,
        image: Image,
        image_layout: ImageLayout,
        depth_stencil: *const ClearDepthStencilValue,
        range_count: u32,
        ranges: *const ImageSubresourceRange,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdClearAttachments`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearAttachments.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdClearAttachments")]
    pub unsafe fn cmd_clear_attachments(
        self,
        attachment_count: u32,
        attachments: *const ClearAttachment,
        rect_count: u32,
        rects: *const ClearRect,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdResolveImage`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdResolveImage")]
    pub unsafe fn cmd_resolve_image(
        self,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        regions: *const ImageResolve,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBeginRenderPass`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    /// - `synchronization`
    ///
    /// ## Allowed command buffers
    /// - Primary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdBeginRenderPass")]
    pub unsafe fn cmd_begin_render_pass(
        self,
        render_pass_begin: *const RenderPassBeginInfo,
        contents: SubpassContents,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdNextSubpass`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    /// - `synchronization`
    ///
    /// ## Allowed command buffers
    /// - Primary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdNextSubpass")]
    pub unsafe fn cmd_next_subpass(self, contents: SubpassContents) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdEndRenderPass`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    /// - `synchronization`
    ///
    /// ## Allowed command buffers
    /// - Primary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdEndRenderPass")]
    pub unsafe fn cmd_end_render_pass(self) {
        todo!()
    }
}

/// `vkEnumerateInstanceVersion`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceVersion.html>
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkEnumerateInstanceVersion")]
pub unsafe fn enumerate_instance_version(api_version: *mut u32) -> ResultCode {
    todo!()
}

impl Device {
    /// `vkBindBufferMemory2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory2.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`](ResultCode::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkBindBufferMemory2")]
    pub unsafe fn bind_buffer_memory_2(
        self,
        bind_info_count: u32,
        bind_infos: *const BindBufferMemoryInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkBindImageMemory2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory2.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkBindImageMemory2")]
    pub unsafe fn bind_image_memory_2(
        self,
        bind_info_count: u32,
        bind_infos: *const BindImageMemoryInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceGroupPeerMemoryFeatures`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPeerMemoryFeatures.html>
    ///
    #[doc(alias = "vkGetDeviceGroupPeerMemoryFeatures")]
    pub unsafe fn get_device_group_peer_memory_features(
        self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
        peer_memory_features: *mut PeerMemoryFeatureFlags,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDeviceMask`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDeviceMask.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    ///
    #[doc(alias = "vkCmdSetDeviceMask")]
    pub unsafe fn cmd_set_device_mask(self, device_mask: u32) {
        todo!()
    }
}

impl Instance {
    /// `vkEnumeratePhysicalDeviceGroups`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroups.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkEnumeratePhysicalDeviceGroups")]
    pub unsafe fn enumerate_physical_device_groups(
        self,
        physical_device_group_count: *mut u32,
        physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetImageMemoryRequirements2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements2.html>
    ///
    #[doc(alias = "vkGetImageMemoryRequirements2")]
    pub unsafe fn get_image_memory_requirements_2(
        self,
        info: *const ImageMemoryRequirementsInfo2,
        memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetBufferMemoryRequirements2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements2.html>
    ///
    #[doc(alias = "vkGetBufferMemoryRequirements2")]
    pub unsafe fn get_buffer_memory_requirements_2(
        self,
        info: *const BufferMemoryRequirementsInfo2,
        memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetImageSparseMemoryRequirements2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2.html>
    ///
    #[doc(alias = "vkGetImageSparseMemoryRequirements2")]
    pub unsafe fn get_image_sparse_memory_requirements_2(
        self,
        info: *const ImageSparseMemoryRequirementsInfo2,
        sparse_memory_requirement_count: *mut u32,
        sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceFeatures2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures2.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceFeatures2")]
    pub unsafe fn get_physical_device_features_2(self, features: *mut PhysicalDeviceFeatures2) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceProperties2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties2.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceProperties2")]
    pub unsafe fn get_physical_device_properties_2(
        self,
        properties: *mut PhysicalDeviceProperties2,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceFormatProperties2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties2.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceFormatProperties2")]
    pub unsafe fn get_physical_device_format_properties_2(
        self,
        format: Format,
        format_properties: *mut FormatProperties2,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceImageFormatProperties2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties2.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`FORMAT_NOT_SUPPORTED`](ResultCode::ERROR_FORMAT_NOT_SUPPORTED)
    /// - [`IMAGE_USAGE_NOT_SUPPORTED_KHR`](ResultCode::ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR)
    /// - [`VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR`](ResultCode::ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR)
    /// - [`VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR`](ResultCode::ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR)
    /// - [`VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR`](ResultCode::ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR)
    /// - [`VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR`](ResultCode::ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceImageFormatProperties2")]
    pub unsafe fn get_physical_device_image_format_properties_2(
        self,
        image_format_info: *const PhysicalDeviceImageFormatInfo2,
        image_format_properties: *mut ImageFormatProperties2,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceQueueFamilyProperties2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2")]
    pub unsafe fn get_physical_device_queue_family_properties_2(
        self,
        queue_family_property_count: *mut u32,
        queue_family_properties: *mut QueueFamilyProperties2,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceMemoryProperties2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties2.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceMemoryProperties2")]
    pub unsafe fn get_physical_device_memory_properties_2(
        self,
        memory_properties: *mut PhysicalDeviceMemoryProperties2,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceSparseImageFormatProperties2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2")]
    pub unsafe fn get_physical_device_sparse_image_format_properties_2(
        self,
        format_info: *const PhysicalDeviceSparseImageFormatInfo2,
        property_count: *mut u32,
        properties: *mut SparseImageFormatProperties2,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkTrimCommandPool`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkTrimCommandPool.html>
    ///
    #[doc(alias = "vkTrimCommandPool")]
    pub unsafe fn trim_command_pool(self, command_pool: CommandPool, flags: CommandPoolTrimFlags) {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceQueue2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue2.html>
    ///
    #[doc(alias = "vkGetDeviceQueue2")]
    pub unsafe fn get_device_queue_2(self, queue_info: *const DeviceQueueInfo2, queue: *mut Queue) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceExternalBufferProperties`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalBufferProperties.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceExternalBufferProperties")]
    pub unsafe fn get_physical_device_external_buffer_properties(
        self,
        external_buffer_info: *const PhysicalDeviceExternalBufferInfo,
        external_buffer_properties: *mut ExternalBufferProperties,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceExternalFenceProperties`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalFenceProperties.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceExternalFenceProperties")]
    pub unsafe fn get_physical_device_external_fence_properties(
        self,
        external_fence_info: *const PhysicalDeviceExternalFenceInfo,
        external_fence_properties: *mut ExternalFenceProperties,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceExternalSemaphoreProperties`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalSemaphoreProperties.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceExternalSemaphoreProperties")]
    pub unsafe fn get_physical_device_external_semaphore_properties(
        self,
        external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo,
        external_semaphore_properties: *mut ExternalSemaphoreProperties,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDispatchBase`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchBase.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdDispatchBase")]
    pub unsafe fn cmd_dispatch_base(
        self,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateDescriptorUpdateTemplate`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorUpdateTemplate.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateDescriptorUpdateTemplate")]
    pub unsafe fn create_descriptor_update_template(
        self,
        create_info: *const DescriptorUpdateTemplateCreateInfo,
        allocator: *const AllocationCallbacks,
        descriptor_update_template: *mut DescriptorUpdateTemplate,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyDescriptorUpdateTemplate`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplate.html>
    ///
    #[doc(alias = "vkDestroyDescriptorUpdateTemplate")]
    pub unsafe fn destroy_descriptor_update_template(
        self,
        descriptor_update_template: DescriptorUpdateTemplate,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkUpdateDescriptorSetWithTemplate`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplate.html>
    ///
    #[doc(alias = "vkUpdateDescriptorSetWithTemplate")]
    pub unsafe fn update_descriptor_set_with_template(
        self,
        descriptor_set: DescriptorSet,
        descriptor_update_template: DescriptorUpdateTemplate,
        data: *const c_void,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetDescriptorSetLayoutSupport`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSupport.html>
    ///
    #[doc(alias = "vkGetDescriptorSetLayoutSupport")]
    pub unsafe fn get_descriptor_set_layout_support(
        self,
        create_info: *const DescriptorSetLayoutCreateInfo,
        support: *mut DescriptorSetLayoutSupport,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateSamplerYcbcrConversion`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSamplerYcbcrConversion.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateSamplerYcbcrConversion")]
    pub unsafe fn create_sampler_ycbcr_conversion(
        self,
        create_info: *const SamplerYcbcrConversionCreateInfo,
        allocator: *const AllocationCallbacks,
        ycbcr_conversion: *mut SamplerYcbcrConversion,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroySamplerYcbcrConversion`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversion.html>
    ///
    #[doc(alias = "vkDestroySamplerYcbcrConversion")]
    pub unsafe fn destroy_sampler_ycbcr_conversion(
        self,
        ycbcr_conversion: SamplerYcbcrConversion,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkResetQueryPool`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkResetQueryPool.html>
    ///
    #[doc(alias = "vkResetQueryPool")]
    pub unsafe fn reset_query_pool(
        self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetSemaphoreCounterValue`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValue.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetSemaphoreCounterValue")]
    pub unsafe fn get_semaphore_counter_value(
        self,
        semaphore: Semaphore,
        value: *mut u64,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkWaitSemaphores`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitSemaphores.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`TIMEOUT`](ResultCode::TIMEOUT)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkWaitSemaphores")]
    pub unsafe fn wait_semaphores(
        self,
        wait_info: *const SemaphoreWaitInfo,
        timeout: u64,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkSignalSemaphore`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSignalSemaphore.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkSignalSemaphore")]
    pub unsafe fn signal_semaphore(self, signal_info: *const SemaphoreSignalInfo) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetBufferDeviceAddress`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddress.html>
    ///
    #[doc(alias = "vkGetBufferDeviceAddress")]
    pub unsafe fn get_buffer_device_address(
        self,
        info: *const BufferDeviceAddressInfo,
    ) -> DeviceAddress {
        todo!()
    }
}

impl Device {
    /// `vkGetBufferOpaqueCaptureAddress`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureAddress.html>
    ///
    #[doc(alias = "vkGetBufferOpaqueCaptureAddress")]
    pub unsafe fn get_buffer_opaque_capture_address(
        self,
        info: *const BufferDeviceAddressInfo,
    ) -> u64 {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceMemoryOpaqueCaptureAddress`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryOpaqueCaptureAddress.html>
    ///
    #[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddress")]
    pub unsafe fn get_device_memory_opaque_capture_address(
        self,
        info: *const DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> u64 {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawIndirectCount`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawIndirectCount")]
    pub unsafe fn cmd_draw_indirect_count(
        self,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawIndexedIndirectCount`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawIndexedIndirectCount")]
    pub unsafe fn cmd_draw_indexed_indirect_count(
        self,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateRenderPass2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass2.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateRenderPass2")]
    pub unsafe fn create_render_pass_2(
        self,
        create_info: *const RenderPassCreateInfo2,
        allocator: *const AllocationCallbacks,
        render_pass: *mut RenderPass,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBeginRenderPass2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass2.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    /// - `synchronization`
    ///
    /// ## Allowed command buffers
    /// - Primary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdBeginRenderPass2")]
    pub unsafe fn cmd_begin_render_pass_2(
        self,
        render_pass_begin: *const RenderPassBeginInfo,
        subpass_begin_info: *const SubpassBeginInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdNextSubpass2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass2.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    /// - `synchronization`
    ///
    /// ## Allowed command buffers
    /// - Primary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdNextSubpass2")]
    pub unsafe fn cmd_next_subpass_2(
        self,
        subpass_begin_info: *const SubpassBeginInfo,
        subpass_end_info: *const SubpassEndInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdEndRenderPass2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass2.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    /// - `synchronization`
    ///
    /// ## Allowed command buffers
    /// - Primary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdEndRenderPass2")]
    pub unsafe fn cmd_end_render_pass_2(self, subpass_end_info: *const SubpassEndInfo) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceToolProperties`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolProperties.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceToolProperties")]
    pub unsafe fn get_physical_device_tool_properties(
        self,
        tool_count: *mut u32,
        tool_properties: *mut PhysicalDeviceToolProperties,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCreatePrivateDataSlot`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePrivateDataSlot.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreatePrivateDataSlot")]
    pub unsafe fn create_private_data_slot(
        self,
        create_info: *const PrivateDataSlotCreateInfo,
        allocator: *const AllocationCallbacks,
        private_data_slot: *mut PrivateDataSlot,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyPrivateDataSlot`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlot.html>
    ///
    #[doc(alias = "vkDestroyPrivateDataSlot")]
    pub unsafe fn destroy_private_data_slot(
        self,
        private_data_slot: PrivateDataSlot,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkSetPrivateData`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetPrivateData.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkSetPrivateData")]
    pub unsafe fn set_private_data(
        self,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: u64,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetPrivateData`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPrivateData.html>
    ///
    #[doc(alias = "vkGetPrivateData")]
    pub unsafe fn get_private_data(
        self,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: *mut u64,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdPipelineBarrier2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier2.html>
    ///
    /// ## Performed tasks
    /// - `synchronization`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    ///
    #[doc(alias = "vkCmdPipelineBarrier2")]
    pub unsafe fn cmd_pipeline_barrier_2(self, dependency_info: *const DependencyInfo) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdWriteTimestamp2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp2.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    ///
    #[doc(alias = "vkCmdWriteTimestamp2")]
    pub unsafe fn cmd_write_timestamp_2(
        self,
        stage: PipelineStageFlags2,
        query_pool: QueryPool,
        query: u32,
    ) {
        todo!()
    }
}

impl Queue {
    /// `vkQueueSubmit2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkQueueSubmit2")]
    pub unsafe fn queue_submit_2(
        self,
        submit_count: u32,
        submits: *const SubmitInfo2,
        fence: Fence,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyBuffer2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyBuffer2")]
    pub unsafe fn cmd_copy_buffer_2(self, copy_buffer_info: *const CopyBufferInfo2) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyImage2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyImage2")]
    pub unsafe fn cmd_copy_image_2(self, copy_image_info: *const CopyImageInfo2) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyBufferToImage2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyBufferToImage2")]
    pub unsafe fn cmd_copy_buffer_to_image_2(
        self,
        copy_buffer_to_image_info: *const CopyBufferToImageInfo2,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyImageToBuffer2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyImageToBuffer2")]
    pub unsafe fn cmd_copy_image_to_buffer_2(
        self,
        copy_image_to_buffer_info: *const CopyImageToBufferInfo2,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceBufferMemoryRequirements`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceBufferMemoryRequirements.html>
    ///
    #[doc(alias = "vkGetDeviceBufferMemoryRequirements")]
    pub unsafe fn get_device_buffer_memory_requirements(
        self,
        info: *const DeviceBufferMemoryRequirements,
        memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceImageMemoryRequirements`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageMemoryRequirements.html>
    ///
    #[doc(alias = "vkGetDeviceImageMemoryRequirements")]
    pub unsafe fn get_device_image_memory_requirements(
        self,
        info: *const DeviceImageMemoryRequirements,
        memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceImageSparseMemoryRequirements`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirements.html>
    ///
    #[doc(alias = "vkGetDeviceImageSparseMemoryRequirements")]
    pub unsafe fn get_device_image_sparse_memory_requirements(
        self,
        info: *const DeviceImageMemoryRequirements,
        sparse_memory_requirement_count: *mut u32,
        sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetEvent2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent2.html>
    ///
    /// ## Performed tasks
    /// - `synchronization`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    ///
    #[doc(alias = "vkCmdSetEvent2")]
    pub unsafe fn cmd_set_event_2(self, event: Event, dependency_info: *const DependencyInfo) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdResetEvent2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent2.html>
    ///
    /// ## Performed tasks
    /// - `synchronization`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    ///
    #[doc(alias = "vkCmdResetEvent2")]
    pub unsafe fn cmd_reset_event_2(self, event: Event, stage_mask: PipelineStageFlags2) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdWaitEvents2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents2.html>
    ///
    /// ## Performed tasks
    /// - `synchronization`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    ///
    #[doc(alias = "vkCmdWaitEvents2")]
    pub unsafe fn cmd_wait_events_2(
        self,
        event_count: u32,
        events: *const Event,
        dependency_infos: *const DependencyInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBlitImage2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdBlitImage2")]
    pub unsafe fn cmd_blit_image_2(self, blit_image_info: *const BlitImageInfo2) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdResolveImage2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage2.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdResolveImage2")]
    pub unsafe fn cmd_resolve_image_2(self, resolve_image_info: *const ResolveImageInfo2) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBeginRendering`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRendering.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdBeginRendering")]
    pub unsafe fn cmd_begin_rendering(self, rendering_info: *const RenderingInfo) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdEndRendering`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdEndRendering")]
    pub unsafe fn cmd_end_rendering(self) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetCullMode`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCullMode.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetCullMode")]
    pub unsafe fn cmd_set_cull_mode(self, cull_mode: CullModeFlags) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetFrontFace`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFrontFace.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetFrontFace")]
    pub unsafe fn cmd_set_front_face(self, front_face: FrontFace) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetPrimitiveTopology`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveTopology.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetPrimitiveTopology")]
    pub unsafe fn cmd_set_primitive_topology(self, primitive_topology: PrimitiveTopology) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetViewportWithCount`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCount.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetViewportWithCount")]
    pub unsafe fn cmd_set_viewport_with_count(
        self,
        viewport_count: u32,
        viewports: *const Viewport,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetScissorWithCount`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCount.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetScissorWithCount")]
    pub unsafe fn cmd_set_scissor_with_count(self, scissor_count: u32, scissors: *const Rect2D) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBindVertexBuffers2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdBindVertexBuffers2")]
    pub unsafe fn cmd_bind_vertex_buffers_2(
        self,
        first_binding: u32,
        binding_count: u32,
        buffers: *const Buffer,
        offsets: *const DeviceSize,
        sizes: *const DeviceSize,
        strides: *const DeviceSize,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDepthTestEnable`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthTestEnable.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetDepthTestEnable")]
    pub unsafe fn cmd_set_depth_test_enable(self, depth_test_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDepthWriteEnable`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthWriteEnable.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetDepthWriteEnable")]
    pub unsafe fn cmd_set_depth_write_enable(self, depth_write_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDepthCompareOp`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthCompareOp.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetDepthCompareOp")]
    pub unsafe fn cmd_set_depth_compare_op(self, depth_compare_op: CompareOp) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDepthBoundsTestEnable`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBoundsTestEnable.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetDepthBoundsTestEnable")]
    pub unsafe fn cmd_set_depth_bounds_test_enable(self, depth_bounds_test_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetStencilTestEnable`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilTestEnable.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetStencilTestEnable")]
    pub unsafe fn cmd_set_stencil_test_enable(self, stencil_test_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetStencilOp`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilOp.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetStencilOp")]
    pub unsafe fn cmd_set_stencil_op(
        self,
        face_mask: StencilFaceFlags,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetRasterizerDiscardEnable`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizerDiscardEnable.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetRasterizerDiscardEnable")]
    pub unsafe fn cmd_set_rasterizer_discard_enable(self, rasterizer_discard_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDepthBiasEnable`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBiasEnable.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetDepthBiasEnable")]
    pub unsafe fn cmd_set_depth_bias_enable(self, depth_bias_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetPrimitiveRestartEnable`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartEnable.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetPrimitiveRestartEnable")]
    pub unsafe fn cmd_set_primitive_restart_enable(self, primitive_restart_enable: Bool32) {
        todo!()
    }
}

impl Device {
    /// `vkMapMemory2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`MEMORY_MAP_FAILED`](ResultCode::ERROR_MEMORY_MAP_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkMapMemory2")]
    pub unsafe fn map_memory_2(
        self,
        memory_map_info: *const MemoryMapInfo,
        data: *mut *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkUnmapMemory2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`MEMORY_MAP_FAILED`](ResultCode::ERROR_MEMORY_MAP_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkUnmapMemory2")]
    pub unsafe fn unmap_memory_2(self, memory_unmap_info: *const MemoryUnmapInfo) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceImageSubresourceLayout`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSubresourceLayout.html>
    ///
    #[doc(alias = "vkGetDeviceImageSubresourceLayout")]
    pub unsafe fn get_device_image_subresource_layout(
        self,
        info: *const DeviceImageSubresourceInfo,
        layout: *mut SubresourceLayout2,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetImageSubresourceLayout2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2.html>
    ///
    #[doc(alias = "vkGetImageSubresourceLayout2")]
    pub unsafe fn get_image_subresource_layout_2(
        self,
        image: Image,
        subresource: *const ImageSubresource2,
        layout: *mut SubresourceLayout2,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCopyMemoryToImage`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToImage.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`MEMORY_MAP_FAILED`](ResultCode::ERROR_MEMORY_MAP_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCopyMemoryToImage")]
    pub unsafe fn copy_memory_to_image(
        self,
        copy_memory_to_image_info: *const CopyMemoryToImageInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCopyImageToMemory`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToMemory.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`MEMORY_MAP_FAILED`](ResultCode::ERROR_MEMORY_MAP_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCopyImageToMemory")]
    pub unsafe fn copy_image_to_memory(
        self,
        copy_image_to_memory_info: *const CopyImageToMemoryInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCopyImageToImage`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToImage.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`MEMORY_MAP_FAILED`](ResultCode::ERROR_MEMORY_MAP_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCopyImageToImage")]
    pub unsafe fn copy_image_to_image(
        self,
        copy_image_to_image_info: *const CopyImageToImageInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkTransitionImageLayout`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkTransitionImageLayout.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`MEMORY_MAP_FAILED`](ResultCode::ERROR_MEMORY_MAP_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkTransitionImageLayout")]
    pub unsafe fn transition_image_layout(
        self,
        transition_count: u32,
        transitions: *const HostImageLayoutTransitionInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdPushDescriptorSet`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdPushDescriptorSet")]
    pub unsafe fn cmd_push_descriptor_set(
        self,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
        descriptor_write_count: u32,
        descriptor_writes: *const WriteDescriptorSet,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdPushDescriptorSetWithTemplate`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdPushDescriptorSetWithTemplate")]
    pub unsafe fn cmd_push_descriptor_set_with_template(
        self,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: u32,
        data: *const c_void,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBindDescriptorSets2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdBindDescriptorSets2")]
    pub unsafe fn cmd_bind_descriptor_sets_2(
        self,
        bind_descriptor_sets_info: *const BindDescriptorSetsInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdPushConstants2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdPushConstants2")]
    pub unsafe fn cmd_push_constants_2(self, push_constants_info: *const PushConstantsInfo) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdPushDescriptorSet2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdPushDescriptorSet2")]
    pub unsafe fn cmd_push_descriptor_set_2(
        self,
        push_descriptor_set_info: *const PushDescriptorSetInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdPushDescriptorSetWithTemplate2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate2.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdPushDescriptorSetWithTemplate2")]
    pub unsafe fn cmd_push_descriptor_set_with_template_2(
        self,
        push_descriptor_set_with_template_info: *const PushDescriptorSetWithTemplateInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetLineStipple`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStipple.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetLineStipple")]
    pub unsafe fn cmd_set_line_stipple(self, line_stipple_factor: u32, line_stipple_pattern: u16) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBindIndexBuffer2`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer2.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdBindIndexBuffer2")]
    pub unsafe fn cmd_bind_index_buffer_2(
        self,
        buffer: Buffer,
        offset: DeviceSize,
        size: DeviceSize,
        index_type: IndexType,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetRenderingAreaGranularity`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderingAreaGranularity.html>
    ///
    #[doc(alias = "vkGetRenderingAreaGranularity")]
    pub unsafe fn get_rendering_area_granularity(
        self,
        rendering_area_info: *const RenderingAreaInfo,
        granularity: *mut Extent2D,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetRenderingAttachmentLocations`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocations.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetRenderingAttachmentLocations")]
    pub unsafe fn cmd_set_rendering_attachment_locations(
        self,
        location_info: *const RenderingAttachmentLocationInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetRenderingInputAttachmentIndices`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndices.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetRenderingInputAttachmentIndices")]
    pub unsafe fn cmd_set_rendering_input_attachment_indices(
        self,
        input_attachment_index_info: *const RenderingInputAttachmentIndexInfo,
    ) {
        todo!()
    }
}

impl Instance {
    /// `vkDestroySurfaceKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySurfaceKHR.html>
    ///
    #[doc(alias = "vkDestroySurfaceKHR")]
    pub unsafe fn destroy_surface_khr(
        self,
        surface: SurfaceKHR,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceSurfaceSupportKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceSupportKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceSurfaceSupportKHR")]
    pub unsafe fn get_physical_device_surface_support_khr(
        self,
        queue_family_index: u32,
        surface: SurfaceKHR,
        supported: *mut Bool32,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceSurfaceCapabilitiesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilitiesKHR")]
    pub unsafe fn get_physical_device_surface_capabilities_khr(
        self,
        surface: SurfaceKHR,
        surface_capabilities: *mut SurfaceCapabilitiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceSurfaceFormatsKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceFormatsKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceSurfaceFormatsKHR")]
    pub unsafe fn get_physical_device_surface_formats_khr(
        self,
        surface: SurfaceKHR,
        surface_format_count: *mut u32,
        surface_formats: *mut SurfaceFormatKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceSurfacePresentModesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfacePresentModesKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceSurfacePresentModesKHR")]
    pub unsafe fn get_physical_device_surface_present_modes_khr(
        self,
        surface: SurfaceKHR,
        present_mode_count: *mut u32,
        present_modes: *mut PresentModeKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCreateSwapchainKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSwapchainKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`NATIVE_WINDOW_IN_USE_KHR`](ResultCode::ERROR_NATIVE_WINDOW_IN_USE_KHR)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`COMPRESSION_EXHAUSTED_EXT`](ResultCode::ERROR_COMPRESSION_EXHAUSTED_EXT)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateSwapchainKHR")]
    pub unsafe fn create_swapchain_khr(
        self,
        create_info: *const SwapchainCreateInfoKHR,
        allocator: *const AllocationCallbacks,
        swapchain: *mut SwapchainKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroySwapchainKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySwapchainKHR.html>
    ///
    #[doc(alias = "vkDestroySwapchainKHR")]
    pub unsafe fn destroy_swapchain_khr(
        self,
        swapchain: SwapchainKHR,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetSwapchainImagesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainImagesKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetSwapchainImagesKHR")]
    pub unsafe fn get_swapchain_images_khr(
        self,
        swapchain: SwapchainKHR,
        swapchain_image_count: *mut u32,
        swapchain_images: *mut Image,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkAcquireNextImageKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImageKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`TIMEOUT`](ResultCode::TIMEOUT)
    /// - [`NOT_READY`](ResultCode::NOT_READY)
    /// - [`SUBOPTIMAL_KHR`](ResultCode::SUBOPTIMAL_KHR)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`OUT_OF_DATE_KHR`](ResultCode::ERROR_OUT_OF_DATE_KHR)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`](ResultCode::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkAcquireNextImageKHR")]
    pub unsafe fn acquire_next_image_khr(
        self,
        swapchain: SwapchainKHR,
        timeout: u64,
        semaphore: Semaphore,
        fence: Fence,
        image_index: *mut u32,
    ) -> ResultCode {
        todo!()
    }
}

impl Queue {
    /// `vkQueuePresentKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueuePresentKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`SUBOPTIMAL_KHR`](ResultCode::SUBOPTIMAL_KHR)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`OUT_OF_DATE_KHR`](ResultCode::ERROR_OUT_OF_DATE_KHR)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`](ResultCode::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    /// - [`PRESENT_TIMING_QUEUE_FULL_EXT`](ResultCode::ERROR_PRESENT_TIMING_QUEUE_FULL_EXT)
    #[doc(alias = "vkQueuePresentKHR")]
    pub unsafe fn queue_present_khr(self, present_info: *const PresentInfoKHR) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceGroupPresentCapabilitiesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPresentCapabilitiesKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetDeviceGroupPresentCapabilitiesKHR")]
    pub unsafe fn get_device_group_present_capabilities_khr(
        self,
        device_group_present_capabilities: *mut DeviceGroupPresentCapabilitiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceGroupSurfacePresentModesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModesKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetDeviceGroupSurfacePresentModesKHR")]
    pub unsafe fn get_device_group_surface_present_modes_khr(
        self,
        surface: SurfaceKHR,
        modes: *mut DeviceGroupPresentModeFlagsKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDevicePresentRectanglesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDevicePresentRectanglesKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDevicePresentRectanglesKHR")]
    pub unsafe fn get_physical_device_present_rectangles_khr(
        self,
        surface: SurfaceKHR,
        rect_count: *mut u32,
        rects: *mut Rect2D,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkAcquireNextImage2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImage2KHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`TIMEOUT`](ResultCode::TIMEOUT)
    /// - [`NOT_READY`](ResultCode::NOT_READY)
    /// - [`SUBOPTIMAL_KHR`](ResultCode::SUBOPTIMAL_KHR)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`OUT_OF_DATE_KHR`](ResultCode::ERROR_OUT_OF_DATE_KHR)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`](ResultCode::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkAcquireNextImage2KHR")]
    pub unsafe fn acquire_next_image_2_khr(
        self,
        acquire_info: *const AcquireNextImageInfoKHR,
        image_index: *mut u32,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceDisplayPropertiesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPropertiesKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceDisplayPropertiesKHR")]
    pub unsafe fn get_physical_device_display_properties_khr(
        self,
        property_count: *mut u32,
        properties: *mut DisplayPropertiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceDisplayPlanePropertiesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceDisplayPlanePropertiesKHR")]
    pub unsafe fn get_physical_device_display_plane_properties_khr(
        self,
        property_count: *mut u32,
        properties: *mut DisplayPlanePropertiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetDisplayPlaneSupportedDisplaysKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneSupportedDisplaysKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetDisplayPlaneSupportedDisplaysKHR")]
    pub unsafe fn get_display_plane_supported_displays_khr(
        self,
        plane_index: u32,
        display_count: *mut u32,
        displays: *mut DisplayKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetDisplayModePropertiesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayModePropertiesKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetDisplayModePropertiesKHR")]
    pub unsafe fn get_display_mode_properties_khr(
        self,
        display: DisplayKHR,
        property_count: *mut u32,
        properties: *mut DisplayModePropertiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkCreateDisplayModeKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayModeKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateDisplayModeKHR")]
    pub unsafe fn create_display_mode_khr(
        self,
        display: DisplayKHR,
        create_info: *const DisplayModeCreateInfoKHR,
        allocator: *const AllocationCallbacks,
        mode: *mut DisplayModeKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetDisplayPlaneCapabilitiesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneCapabilitiesKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetDisplayPlaneCapabilitiesKHR")]
    pub unsafe fn get_display_plane_capabilities_khr(
        self,
        mode: DisplayModeKHR,
        plane_index: u32,
        capabilities: *mut DisplayPlaneCapabilitiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Instance {
    /// `vkCreateDisplayPlaneSurfaceKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayPlaneSurfaceKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateDisplayPlaneSurfaceKHR")]
    pub unsafe fn create_display_plane_surface_khr(
        self,
        create_info: *const DisplaySurfaceCreateInfoKHR,
        allocator: *const AllocationCallbacks,
        surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCreateSharedSwapchainsKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSharedSwapchainsKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INCOMPATIBLE_DISPLAY_KHR`](ResultCode::ERROR_INCOMPATIBLE_DISPLAY_KHR)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateSharedSwapchainsKHR")]
    pub unsafe fn create_shared_swapchains_khr(
        self,
        swapchain_count: u32,
        create_infos: *const SwapchainCreateInfoKHR,
        allocator: *const AllocationCallbacks,
        swapchains: *mut SwapchainKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Instance {
    /// `vkCreateXlibSurfaceKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXlibSurfaceKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateXlibSurfaceKHR")]
    pub unsafe fn create_xlib_surface_khr(
        self,
        create_info: *const XlibSurfaceCreateInfoKHR,
        allocator: *const AllocationCallbacks,
        surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceXlibPresentationSupportKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceXlibPresentationSupportKHR.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceXlibPresentationSupportKHR")]
    pub unsafe fn get_physical_device_xlib_presentation_support_khr(
        self,
        queue_family_index: u32,
        dpy: *mut Display,
        visual_id: VisualID,
    ) -> Bool32 {
        todo!()
    }
}

impl Instance {
    /// `vkCreateXcbSurfaceKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXcbSurfaceKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateXcbSurfaceKHR")]
    pub unsafe fn create_xcb_surface_khr(
        self,
        create_info: *const XcbSurfaceCreateInfoKHR,
        allocator: *const AllocationCallbacks,
        surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceXcbPresentationSupportKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceXcbPresentationSupportKHR.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceXcbPresentationSupportKHR")]
    pub unsafe fn get_physical_device_xcb_presentation_support_khr(
        self,
        queue_family_index: u32,
        connection: *mut xcb_connection_t,
        visual_id: xcb_visualid_t,
    ) -> Bool32 {
        todo!()
    }
}

impl Instance {
    /// `vkCreateWaylandSurfaceKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWaylandSurfaceKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateWaylandSurfaceKHR")]
    pub unsafe fn create_wayland_surface_khr(
        self,
        create_info: *const WaylandSurfaceCreateInfoKHR,
        allocator: *const AllocationCallbacks,
        surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceWaylandPresentationSupportKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceWaylandPresentationSupportKHR")]
    pub unsafe fn get_physical_device_wayland_presentation_support_khr(
        self,
        queue_family_index: u32,
        display: *mut wl_display,
    ) -> Bool32 {
        todo!()
    }
}

impl Instance {
    /// `vkCreateAndroidSurfaceKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAndroidSurfaceKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`NATIVE_WINDOW_IN_USE_KHR`](ResultCode::ERROR_NATIVE_WINDOW_IN_USE_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateAndroidSurfaceKHR")]
    pub unsafe fn create_android_surface_khr(
        self,
        create_info: *const AndroidSurfaceCreateInfoKHR,
        allocator: *const AllocationCallbacks,
        surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Instance {
    /// `vkCreateWin32SurfaceKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWin32SurfaceKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateWin32SurfaceKHR")]
    pub unsafe fn create_win_32_surface_khr(
        self,
        create_info: *const Win32SurfaceCreateInfoKHR,
        allocator: *const AllocationCallbacks,
        surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceWin32PresentationSupportKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceWin32PresentationSupportKHR.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceWin32PresentationSupportKHR")]
    pub unsafe fn get_physical_device_win_32_presentation_support_khr(
        self,
        queue_family_index: u32,
    ) -> Bool32 {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceVideoCapabilitiesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoCapabilitiesKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR`](ResultCode::ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR)
    /// - [`VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR`](ResultCode::ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR)
    /// - [`VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR`](ResultCode::ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR)
    /// - [`VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR`](ResultCode::ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceVideoCapabilitiesKHR")]
    pub unsafe fn get_physical_device_video_capabilities_khr(
        self,
        video_profile: *const VideoProfileInfoKHR,
        capabilities: *mut VideoCapabilitiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceVideoFormatPropertiesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`IMAGE_USAGE_NOT_SUPPORTED_KHR`](ResultCode::ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR)
    /// - [`VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR`](ResultCode::ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR)
    /// - [`VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR`](ResultCode::ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR)
    /// - [`VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR`](ResultCode::ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR)
    /// - [`VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR`](ResultCode::ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceVideoFormatPropertiesKHR")]
    pub unsafe fn get_physical_device_video_format_properties_khr(
        self,
        video_format_info: *const PhysicalDeviceVideoFormatInfoKHR,
        video_format_property_count: *mut u32,
        video_format_properties: *mut VideoFormatPropertiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCreateVideoSessionKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateVideoSessionKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`VIDEO_STD_VERSION_NOT_SUPPORTED_KHR`](ResultCode::ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR)
    /// - [`INVALID_VIDEO_STD_PARAMETERS_KHR`](ResultCode::ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateVideoSessionKHR")]
    pub unsafe fn create_video_session_khr(
        self,
        create_info: *const VideoSessionCreateInfoKHR,
        allocator: *const AllocationCallbacks,
        video_session: *mut VideoSessionKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyVideoSessionKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyVideoSessionKHR.html>
    ///
    #[doc(alias = "vkDestroyVideoSessionKHR")]
    pub unsafe fn destroy_video_session_khr(
        self,
        video_session: VideoSessionKHR,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetVideoSessionMemoryRequirementsKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetVideoSessionMemoryRequirementsKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetVideoSessionMemoryRequirementsKHR")]
    pub unsafe fn get_video_session_memory_requirements_khr(
        self,
        video_session: VideoSessionKHR,
        memory_requirements_count: *mut u32,
        memory_requirements: *mut VideoSessionMemoryRequirementsKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkBindVideoSessionMemoryKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindVideoSessionMemoryKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkBindVideoSessionMemoryKHR")]
    pub unsafe fn bind_video_session_memory_khr(
        self,
        video_session: VideoSessionKHR,
        bind_session_memory_info_count: u32,
        bind_session_memory_infos: *const BindVideoSessionMemoryInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCreateVideoSessionParametersKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateVideoSessionParametersKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`INVALID_VIDEO_STD_PARAMETERS_KHR`](ResultCode::ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateVideoSessionParametersKHR")]
    pub unsafe fn create_video_session_parameters_khr(
        self,
        create_info: *const VideoSessionParametersCreateInfoKHR,
        allocator: *const AllocationCallbacks,
        video_session_parameters: *mut VideoSessionParametersKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkUpdateVideoSessionParametersKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateVideoSessionParametersKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INVALID_VIDEO_STD_PARAMETERS_KHR`](ResultCode::ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkUpdateVideoSessionParametersKHR")]
    pub unsafe fn update_video_session_parameters_khr(
        self,
        video_session_parameters: VideoSessionParametersKHR,
        update_info: *const VideoSessionParametersUpdateInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyVideoSessionParametersKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyVideoSessionParametersKHR.html>
    ///
    #[doc(alias = "vkDestroyVideoSessionParametersKHR")]
    pub unsafe fn destroy_video_session_parameters_khr(
        self,
        video_session_parameters: VideoSessionParametersKHR,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBeginVideoCodingKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginVideoCodingKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    ///
    /// ## Allowed queues
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    ///
    #[doc(alias = "vkCmdBeginVideoCodingKHR")]
    pub unsafe fn cmd_begin_video_coding_khr(self, begin_info: *const VideoBeginCodingInfoKHR) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdEndVideoCodingKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndVideoCodingKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    ///
    /// ## Allowed queues
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    ///
    #[doc(alias = "vkCmdEndVideoCodingKHR")]
    pub unsafe fn cmd_end_video_coding_khr(self, end_coding_info: *const VideoEndCodingInfoKHR) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdControlVideoCodingKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdControlVideoCodingKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    ///
    /// ## Allowed queues
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    ///
    #[doc(alias = "vkCmdControlVideoCodingKHR")]
    pub unsafe fn cmd_control_video_coding_khr(
        self,
        coding_control_info: *const VideoCodingControlInfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDecodeVideoKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecodeVideoKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    ///
    /// ## Allowed queues
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    ///
    #[doc(alias = "vkCmdDecodeVideoKHR")]
    pub unsafe fn cmd_decode_video_khr(self, decode_info: *const VideoDecodeInfoKHR) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBeginRenderingKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderingKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdBeginRenderingKHR")]
    pub unsafe fn cmd_begin_rendering_khr(self, rendering_info: *const RenderingInfo) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdEndRenderingKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderingKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdEndRenderingKHR")]
    pub unsafe fn cmd_end_rendering_khr(self) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceFeatures2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures2KHR.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceFeatures2KHR")]
    pub unsafe fn get_physical_device_features_2_khr(self, features: *mut PhysicalDeviceFeatures2) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceProperties2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties2KHR.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceProperties2KHR")]
    pub unsafe fn get_physical_device_properties_2_khr(
        self,
        properties: *mut PhysicalDeviceProperties2,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceFormatProperties2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties2KHR.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceFormatProperties2KHR")]
    pub unsafe fn get_physical_device_format_properties_2_khr(
        self,
        format: Format,
        format_properties: *mut FormatProperties2,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceImageFormatProperties2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties2KHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`FORMAT_NOT_SUPPORTED`](ResultCode::ERROR_FORMAT_NOT_SUPPORTED)
    /// - [`IMAGE_USAGE_NOT_SUPPORTED_KHR`](ResultCode::ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR)
    /// - [`VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR`](ResultCode::ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR)
    /// - [`VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR`](ResultCode::ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR)
    /// - [`VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR`](ResultCode::ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR)
    /// - [`VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR`](ResultCode::ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceImageFormatProperties2KHR")]
    pub unsafe fn get_physical_device_image_format_properties_2_khr(
        self,
        image_format_info: *const PhysicalDeviceImageFormatInfo2,
        image_format_properties: *mut ImageFormatProperties2,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceQueueFamilyProperties2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2KHR.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2KHR")]
    pub unsafe fn get_physical_device_queue_family_properties_2_khr(
        self,
        queue_family_property_count: *mut u32,
        queue_family_properties: *mut QueueFamilyProperties2,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceMemoryProperties2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties2KHR.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceMemoryProperties2KHR")]
    pub unsafe fn get_physical_device_memory_properties_2_khr(
        self,
        memory_properties: *mut PhysicalDeviceMemoryProperties2,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceSparseImageFormatProperties2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2KHR.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2KHR")]
    pub unsafe fn get_physical_device_sparse_image_format_properties_2_khr(
        self,
        format_info: *const PhysicalDeviceSparseImageFormatInfo2,
        property_count: *mut u32,
        properties: *mut SparseImageFormatProperties2,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceGroupPeerMemoryFeaturesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPeerMemoryFeaturesKHR.html>
    ///
    #[doc(alias = "vkGetDeviceGroupPeerMemoryFeaturesKHR")]
    pub unsafe fn get_device_group_peer_memory_features_khr(
        self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
        peer_memory_features: *mut PeerMemoryFeatureFlags,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDeviceMaskKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDeviceMaskKHR.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    ///
    #[doc(alias = "vkCmdSetDeviceMaskKHR")]
    pub unsafe fn cmd_set_device_mask_khr(self, device_mask: u32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDispatchBaseKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchBaseKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdDispatchBaseKHR")]
    pub unsafe fn cmd_dispatch_base_khr(
        self,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkTrimCommandPoolKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkTrimCommandPoolKHR.html>
    ///
    #[doc(alias = "vkTrimCommandPoolKHR")]
    pub unsafe fn trim_command_pool_khr(
        self,
        command_pool: CommandPool,
        flags: CommandPoolTrimFlags,
    ) {
        todo!()
    }
}

impl Instance {
    /// `vkEnumeratePhysicalDeviceGroupsKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroupsKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkEnumeratePhysicalDeviceGroupsKHR")]
    pub unsafe fn enumerate_physical_device_groups_khr(
        self,
        physical_device_group_count: *mut u32,
        physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceExternalBufferPropertiesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalBufferPropertiesKHR.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceExternalBufferPropertiesKHR")]
    pub unsafe fn get_physical_device_external_buffer_properties_khr(
        self,
        external_buffer_info: *const PhysicalDeviceExternalBufferInfo,
        external_buffer_properties: *mut ExternalBufferProperties,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetMemoryWin32HandleKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`TOO_MANY_OBJECTS`](ResultCode::ERROR_TOO_MANY_OBJECTS)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetMemoryWin32HandleKHR")]
    pub unsafe fn get_memory_win_32_handle_khr(
        self,
        get_win_32_handle_info: *const MemoryGetWin32HandleInfoKHR,
        handle: *mut HANDLE,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetMemoryWin32HandlePropertiesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandlePropertiesKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`INVALID_EXTERNAL_HANDLE`](ResultCode::ERROR_INVALID_EXTERNAL_HANDLE)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetMemoryWin32HandlePropertiesKHR")]
    pub unsafe fn get_memory_win_32_handle_properties_khr(
        self,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: HANDLE,
        memory_win_32_handle_properties: *mut MemoryWin32HandlePropertiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetMemoryFdKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryFdKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`TOO_MANY_OBJECTS`](ResultCode::ERROR_TOO_MANY_OBJECTS)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetMemoryFdKHR")]
    pub unsafe fn get_memory_fd_khr(
        self,
        get_fd_info: *const MemoryGetFdInfoKHR,
        fd: *mut c_int,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetMemoryFdPropertiesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryFdPropertiesKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`INVALID_EXTERNAL_HANDLE`](ResultCode::ERROR_INVALID_EXTERNAL_HANDLE)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetMemoryFdPropertiesKHR")]
    pub unsafe fn get_memory_fd_properties_khr(
        self,
        handle_type: ExternalMemoryHandleTypeFlags,
        fd: c_int,
        memory_fd_properties: *mut MemoryFdPropertiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceExternalSemaphorePropertiesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalSemaphorePropertiesKHR.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceExternalSemaphorePropertiesKHR")]
    pub unsafe fn get_physical_device_external_semaphore_properties_khr(
        self,
        external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo,
        external_semaphore_properties: *mut ExternalSemaphoreProperties,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkImportSemaphoreWin32HandleKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreWin32HandleKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`INVALID_EXTERNAL_HANDLE`](ResultCode::ERROR_INVALID_EXTERNAL_HANDLE)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkImportSemaphoreWin32HandleKHR")]
    pub unsafe fn import_semaphore_win_32_handle_khr(
        self,
        import_semaphore_win_32_handle_info: *const ImportSemaphoreWin32HandleInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetSemaphoreWin32HandleKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreWin32HandleKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`TOO_MANY_OBJECTS`](ResultCode::ERROR_TOO_MANY_OBJECTS)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetSemaphoreWin32HandleKHR")]
    pub unsafe fn get_semaphore_win_32_handle_khr(
        self,
        get_win_32_handle_info: *const SemaphoreGetWin32HandleInfoKHR,
        handle: *mut HANDLE,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkImportSemaphoreFdKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreFdKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`INVALID_EXTERNAL_HANDLE`](ResultCode::ERROR_INVALID_EXTERNAL_HANDLE)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkImportSemaphoreFdKHR")]
    pub unsafe fn import_semaphore_fd_khr(
        self,
        import_semaphore_fd_info: *const ImportSemaphoreFdInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetSemaphoreFdKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreFdKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`TOO_MANY_OBJECTS`](ResultCode::ERROR_TOO_MANY_OBJECTS)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetSemaphoreFdKHR")]
    pub unsafe fn get_semaphore_fd_khr(
        self,
        get_fd_info: *const SemaphoreGetFdInfoKHR,
        fd: *mut c_int,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdPushDescriptorSetKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetKHR.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdPushDescriptorSetKHR")]
    pub unsafe fn cmd_push_descriptor_set_khr(
        self,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
        descriptor_write_count: u32,
        descriptor_writes: *const WriteDescriptorSet,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdPushDescriptorSetWithTemplateKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplateKHR.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdPushDescriptorSetWithTemplateKHR")]
    pub unsafe fn cmd_push_descriptor_set_with_template_khr(
        self,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: u32,
        data: *const c_void,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateDescriptorUpdateTemplateKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorUpdateTemplateKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateDescriptorUpdateTemplateKHR")]
    pub unsafe fn create_descriptor_update_template_khr(
        self,
        create_info: *const DescriptorUpdateTemplateCreateInfo,
        allocator: *const AllocationCallbacks,
        descriptor_update_template: *mut DescriptorUpdateTemplate,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyDescriptorUpdateTemplateKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplateKHR.html>
    ///
    #[doc(alias = "vkDestroyDescriptorUpdateTemplateKHR")]
    pub unsafe fn destroy_descriptor_update_template_khr(
        self,
        descriptor_update_template: DescriptorUpdateTemplate,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkUpdateDescriptorSetWithTemplateKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplateKHR.html>
    ///
    #[doc(alias = "vkUpdateDescriptorSetWithTemplateKHR")]
    pub unsafe fn update_descriptor_set_with_template_khr(
        self,
        descriptor_set: DescriptorSet,
        descriptor_update_template: DescriptorUpdateTemplate,
        data: *const c_void,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateRenderPass2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass2KHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateRenderPass2KHR")]
    pub unsafe fn create_render_pass_2_khr(
        self,
        create_info: *const RenderPassCreateInfo2,
        allocator: *const AllocationCallbacks,
        render_pass: *mut RenderPass,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBeginRenderPass2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass2KHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    /// - `synchronization`
    ///
    /// ## Allowed command buffers
    /// - Primary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdBeginRenderPass2KHR")]
    pub unsafe fn cmd_begin_render_pass_2_khr(
        self,
        render_pass_begin: *const RenderPassBeginInfo,
        subpass_begin_info: *const SubpassBeginInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdNextSubpass2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass2KHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    /// - `synchronization`
    ///
    /// ## Allowed command buffers
    /// - Primary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdNextSubpass2KHR")]
    pub unsafe fn cmd_next_subpass_2_khr(
        self,
        subpass_begin_info: *const SubpassBeginInfo,
        subpass_end_info: *const SubpassEndInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdEndRenderPass2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass2KHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    /// - `synchronization`
    ///
    /// ## Allowed command buffers
    /// - Primary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdEndRenderPass2KHR")]
    pub unsafe fn cmd_end_render_pass_2_khr(self, subpass_end_info: *const SubpassEndInfo) {
        todo!()
    }
}

impl Device {
    /// `vkGetSwapchainStatusKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainStatusKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`SUBOPTIMAL_KHR`](ResultCode::SUBOPTIMAL_KHR)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`OUT_OF_DATE_KHR`](ResultCode::ERROR_OUT_OF_DATE_KHR)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`](ResultCode::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetSwapchainStatusKHR")]
    pub unsafe fn get_swapchain_status_khr(self, swapchain: SwapchainKHR) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceExternalFencePropertiesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalFencePropertiesKHR.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceExternalFencePropertiesKHR")]
    pub unsafe fn get_physical_device_external_fence_properties_khr(
        self,
        external_fence_info: *const PhysicalDeviceExternalFenceInfo,
        external_fence_properties: *mut ExternalFenceProperties,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkImportFenceWin32HandleKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceWin32HandleKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`INVALID_EXTERNAL_HANDLE`](ResultCode::ERROR_INVALID_EXTERNAL_HANDLE)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkImportFenceWin32HandleKHR")]
    pub unsafe fn import_fence_win_32_handle_khr(
        self,
        import_fence_win_32_handle_info: *const ImportFenceWin32HandleInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetFenceWin32HandleKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceWin32HandleKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`TOO_MANY_OBJECTS`](ResultCode::ERROR_TOO_MANY_OBJECTS)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetFenceWin32HandleKHR")]
    pub unsafe fn get_fence_win_32_handle_khr(
        self,
        get_win_32_handle_info: *const FenceGetWin32HandleInfoKHR,
        handle: *mut HANDLE,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkImportFenceFdKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceFdKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`INVALID_EXTERNAL_HANDLE`](ResultCode::ERROR_INVALID_EXTERNAL_HANDLE)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkImportFenceFdKHR")]
    pub unsafe fn import_fence_fd_khr(
        self,
        import_fence_fd_info: *const ImportFenceFdInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetFenceFdKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceFdKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`TOO_MANY_OBJECTS`](ResultCode::ERROR_TOO_MANY_OBJECTS)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetFenceFdKHR")]
    pub unsafe fn get_fence_fd_khr(
        self,
        get_fd_info: *const FenceGetFdInfoKHR,
        fd: *mut c_int,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR")]
    pub unsafe fn enumerate_physical_device_queue_family_performance_query_counters_khr(
        self,
        queue_family_index: u32,
        counter_count: *mut u32,
        counters: *mut PerformanceCounterKHR,
        counter_descriptions: *mut PerformanceCounterDescriptionKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR")]
    pub unsafe fn get_physical_device_queue_family_performance_query_passes_khr(
        self,
        performance_query_create_info: *const QueryPoolPerformanceCreateInfoKHR,
        num_passes: *mut u32,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkAcquireProfilingLockKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireProfilingLockKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`TIMEOUT`](ResultCode::TIMEOUT)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkAcquireProfilingLockKHR")]
    pub unsafe fn acquire_profiling_lock_khr(
        self,
        info: *const AcquireProfilingLockInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkReleaseProfilingLockKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseProfilingLockKHR.html>
    ///
    #[doc(alias = "vkReleaseProfilingLockKHR")]
    pub unsafe fn release_profiling_lock_khr(self) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceSurfaceCapabilities2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2KHR")]
    pub unsafe fn get_physical_device_surface_capabilities_2_khr(
        self,
        surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
        surface_capabilities: *mut SurfaceCapabilities2KHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceSurfaceFormats2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceFormats2KHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceSurfaceFormats2KHR")]
    pub unsafe fn get_physical_device_surface_formats_2_khr(
        self,
        surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
        surface_format_count: *mut u32,
        surface_formats: *mut SurfaceFormat2KHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceDisplayProperties2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayProperties2KHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceDisplayProperties2KHR")]
    pub unsafe fn get_physical_device_display_properties_2_khr(
        self,
        property_count: *mut u32,
        properties: *mut DisplayProperties2KHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceDisplayPlaneProperties2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceDisplayPlaneProperties2KHR")]
    pub unsafe fn get_physical_device_display_plane_properties_2_khr(
        self,
        property_count: *mut u32,
        properties: *mut DisplayPlaneProperties2KHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetDisplayModeProperties2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayModeProperties2KHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetDisplayModeProperties2KHR")]
    pub unsafe fn get_display_mode_properties_2_khr(
        self,
        display: DisplayKHR,
        property_count: *mut u32,
        properties: *mut DisplayModeProperties2KHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetDisplayPlaneCapabilities2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneCapabilities2KHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetDisplayPlaneCapabilities2KHR")]
    pub unsafe fn get_display_plane_capabilities_2_khr(
        self,
        display_plane_info: *const DisplayPlaneInfo2KHR,
        capabilities: *mut DisplayPlaneCapabilities2KHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetImageMemoryRequirements2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements2KHR.html>
    ///
    #[doc(alias = "vkGetImageMemoryRequirements2KHR")]
    pub unsafe fn get_image_memory_requirements_2_khr(
        self,
        info: *const ImageMemoryRequirementsInfo2,
        memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetBufferMemoryRequirements2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements2KHR.html>
    ///
    #[doc(alias = "vkGetBufferMemoryRequirements2KHR")]
    pub unsafe fn get_buffer_memory_requirements_2_khr(
        self,
        info: *const BufferMemoryRequirementsInfo2,
        memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetImageSparseMemoryRequirements2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2KHR.html>
    ///
    #[doc(alias = "vkGetImageSparseMemoryRequirements2KHR")]
    pub unsafe fn get_image_sparse_memory_requirements_2_khr(
        self,
        info: *const ImageSparseMemoryRequirementsInfo2,
        sparse_memory_requirement_count: *mut u32,
        sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateSamplerYcbcrConversionKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSamplerYcbcrConversionKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateSamplerYcbcrConversionKHR")]
    pub unsafe fn create_sampler_ycbcr_conversion_khr(
        self,
        create_info: *const SamplerYcbcrConversionCreateInfo,
        allocator: *const AllocationCallbacks,
        ycbcr_conversion: *mut SamplerYcbcrConversion,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroySamplerYcbcrConversionKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversionKHR.html>
    ///
    #[doc(alias = "vkDestroySamplerYcbcrConversionKHR")]
    pub unsafe fn destroy_sampler_ycbcr_conversion_khr(
        self,
        ycbcr_conversion: SamplerYcbcrConversion,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkBindBufferMemory2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory2KHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`](ResultCode::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkBindBufferMemory2KHR")]
    pub unsafe fn bind_buffer_memory_2_khr(
        self,
        bind_info_count: u32,
        bind_infos: *const BindBufferMemoryInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkBindImageMemory2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory2KHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkBindImageMemory2KHR")]
    pub unsafe fn bind_image_memory_2_khr(
        self,
        bind_info_count: u32,
        bind_infos: *const BindImageMemoryInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetDescriptorSetLayoutSupportKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSupportKHR.html>
    ///
    #[doc(alias = "vkGetDescriptorSetLayoutSupportKHR")]
    pub unsafe fn get_descriptor_set_layout_support_khr(
        self,
        create_info: *const DescriptorSetLayoutCreateInfo,
        support: *mut DescriptorSetLayoutSupport,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawIndirectCountKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCountKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawIndirectCountKHR")]
    pub unsafe fn cmd_draw_indirect_count_khr(
        self,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawIndexedIndirectCountKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCountKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawIndexedIndirectCountKHR")]
    pub unsafe fn cmd_draw_indexed_indirect_count_khr(
        self,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetSemaphoreCounterValueKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValueKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetSemaphoreCounterValueKHR")]
    pub unsafe fn get_semaphore_counter_value_khr(
        self,
        semaphore: Semaphore,
        value: *mut u64,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkWaitSemaphoresKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitSemaphoresKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`TIMEOUT`](ResultCode::TIMEOUT)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkWaitSemaphoresKHR")]
    pub unsafe fn wait_semaphores_khr(
        self,
        wait_info: *const SemaphoreWaitInfo,
        timeout: u64,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkSignalSemaphoreKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSignalSemaphoreKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkSignalSemaphoreKHR")]
    pub unsafe fn signal_semaphore_khr(
        self,
        signal_info: *const SemaphoreSignalInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceFragmentShadingRatesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFragmentShadingRatesKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceFragmentShadingRatesKHR")]
    pub unsafe fn get_physical_device_fragment_shading_rates_khr(
        self,
        fragment_shading_rate_count: *mut u32,
        fragment_shading_rates: *mut PhysicalDeviceFragmentShadingRateKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetFragmentShadingRateKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFragmentShadingRateKHR.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetFragmentShadingRateKHR")]
    pub unsafe fn cmd_set_fragment_shading_rate_khr(
        self,
        fragment_size: *const Extent2D,
        combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2 as usize],
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetRenderingAttachmentLocationsKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocationsKHR.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetRenderingAttachmentLocationsKHR")]
    pub unsafe fn cmd_set_rendering_attachment_locations_khr(
        self,
        location_info: *const RenderingAttachmentLocationInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetRenderingInputAttachmentIndicesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndicesKHR.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetRenderingInputAttachmentIndicesKHR")]
    pub unsafe fn cmd_set_rendering_input_attachment_indices_khr(
        self,
        input_attachment_index_info: *const RenderingInputAttachmentIndexInfo,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkWaitForPresentKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresentKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`TIMEOUT`](ResultCode::TIMEOUT)
    /// - [`SUBOPTIMAL_KHR`](ResultCode::SUBOPTIMAL_KHR)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`OUT_OF_DATE_KHR`](ResultCode::ERROR_OUT_OF_DATE_KHR)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`](ResultCode::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkWaitForPresentKHR")]
    pub unsafe fn wait_for_present_khr(
        self,
        swapchain: SwapchainKHR,
        present_id: u64,
        timeout: u64,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetBufferDeviceAddressKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddressKHR.html>
    ///
    #[doc(alias = "vkGetBufferDeviceAddressKHR")]
    pub unsafe fn get_buffer_device_address_khr(
        self,
        info: *const BufferDeviceAddressInfo,
    ) -> DeviceAddress {
        todo!()
    }
}

impl Device {
    /// `vkGetBufferOpaqueCaptureAddressKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureAddressKHR.html>
    ///
    #[doc(alias = "vkGetBufferOpaqueCaptureAddressKHR")]
    pub unsafe fn get_buffer_opaque_capture_address_khr(
        self,
        info: *const BufferDeviceAddressInfo,
    ) -> u64 {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceMemoryOpaqueCaptureAddressKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryOpaqueCaptureAddressKHR.html>
    ///
    #[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddressKHR")]
    pub unsafe fn get_device_memory_opaque_capture_address_khr(
        self,
        info: *const DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> u64 {
        todo!()
    }
}

impl Device {
    /// `vkCreateDeferredOperationKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDeferredOperationKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateDeferredOperationKHR")]
    pub unsafe fn create_deferred_operation_khr(
        self,
        allocator: *const AllocationCallbacks,
        deferred_operation: *mut DeferredOperationKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyDeferredOperationKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDeferredOperationKHR.html>
    ///
    #[doc(alias = "vkDestroyDeferredOperationKHR")]
    pub unsafe fn destroy_deferred_operation_khr(
        self,
        operation: DeferredOperationKHR,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetDeferredOperationMaxConcurrencyKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeferredOperationMaxConcurrencyKHR.html>
    ///
    #[doc(alias = "vkGetDeferredOperationMaxConcurrencyKHR")]
    pub unsafe fn get_deferred_operation_max_concurrency_khr(
        self,
        operation: DeferredOperationKHR,
    ) -> u32 {
        todo!()
    }
}

impl Device {
    /// `vkGetDeferredOperationResultKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeferredOperationResultKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`NOT_READY`](ResultCode::NOT_READY)
    /// ## Error
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetDeferredOperationResultKHR")]
    pub unsafe fn get_deferred_operation_result_khr(
        self,
        operation: DeferredOperationKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDeferredOperationJoinKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDeferredOperationJoinKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`THREAD_DONE_KHR`](ResultCode::THREAD_DONE_KHR)
    /// - [`THREAD_IDLE_KHR`](ResultCode::THREAD_IDLE_KHR)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkDeferredOperationJoinKHR")]
    pub unsafe fn deferred_operation_join_khr(self, operation: DeferredOperationKHR) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetPipelineExecutablePropertiesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutablePropertiesKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPipelineExecutablePropertiesKHR")]
    pub unsafe fn get_pipeline_executable_properties_khr(
        self,
        pipeline_info: *const PipelineInfoKHR,
        executable_count: *mut u32,
        properties: *mut PipelineExecutablePropertiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetPipelineExecutableStatisticsKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutableStatisticsKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPipelineExecutableStatisticsKHR")]
    pub unsafe fn get_pipeline_executable_statistics_khr(
        self,
        executable_info: *const PipelineExecutableInfoKHR,
        statistic_count: *mut u32,
        statistics: *mut PipelineExecutableStatisticKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetPipelineExecutableInternalRepresentationsKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutableInternalRepresentationsKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPipelineExecutableInternalRepresentationsKHR")]
    pub unsafe fn get_pipeline_executable_internal_representations_khr(
        self,
        executable_info: *const PipelineExecutableInfoKHR,
        internal_representation_count: *mut u32,
        internal_representations: *mut PipelineExecutableInternalRepresentationKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkMapMemory2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2KHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`MEMORY_MAP_FAILED`](ResultCode::ERROR_MEMORY_MAP_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkMapMemory2KHR")]
    pub unsafe fn map_memory_2_khr(
        self,
        memory_map_info: *const MemoryMapInfo,
        data: *mut *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkUnmapMemory2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2KHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`MEMORY_MAP_FAILED`](ResultCode::ERROR_MEMORY_MAP_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkUnmapMemory2KHR")]
    pub unsafe fn unmap_memory_2_khr(
        self,
        memory_unmap_info: *const MemoryUnmapInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR`](ResultCode::ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR)
    /// - [`VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR`](ResultCode::ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR)
    /// - [`VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR`](ResultCode::ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR)
    /// - [`VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR`](ResultCode::ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR")]
    pub unsafe fn get_physical_device_video_encode_quality_level_properties_khr(
        self,
        quality_level_info: *const PhysicalDeviceVideoEncodeQualityLevelInfoKHR,
        quality_level_properties: *mut VideoEncodeQualityLevelPropertiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetEncodedVideoSessionParametersKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetEncodedVideoSessionParametersKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetEncodedVideoSessionParametersKHR")]
    pub unsafe fn get_encoded_video_session_parameters_khr(
        self,
        video_session_parameters_info: *const VideoEncodeSessionParametersGetInfoKHR,
        feedback_info: *mut VideoEncodeSessionParametersFeedbackInfoKHR,
        data_size: *mut usize,
        data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdEncodeVideoKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEncodeVideoKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    ///
    /// ## Allowed queues
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    ///
    #[doc(alias = "vkCmdEncodeVideoKHR")]
    pub unsafe fn cmd_encode_video_khr(self, encode_info: *const VideoEncodeInfoKHR) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetEvent2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent2KHR.html>
    ///
    /// ## Performed tasks
    /// - `synchronization`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    ///
    #[doc(alias = "vkCmdSetEvent2KHR")]
    pub unsafe fn cmd_set_event_2_khr(self, event: Event, dependency_info: *const DependencyInfo) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdResetEvent2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent2KHR.html>
    ///
    /// ## Performed tasks
    /// - `synchronization`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    ///
    #[doc(alias = "vkCmdResetEvent2KHR")]
    pub unsafe fn cmd_reset_event_2_khr(self, event: Event, stage_mask: PipelineStageFlags2) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdWaitEvents2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents2KHR.html>
    ///
    /// ## Performed tasks
    /// - `synchronization`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    ///
    #[doc(alias = "vkCmdWaitEvents2KHR")]
    pub unsafe fn cmd_wait_events_2_khr(
        self,
        event_count: u32,
        events: *const Event,
        dependency_infos: *const DependencyInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdPipelineBarrier2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier2KHR.html>
    ///
    /// ## Performed tasks
    /// - `synchronization`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    ///
    #[doc(alias = "vkCmdPipelineBarrier2KHR")]
    pub unsafe fn cmd_pipeline_barrier_2_khr(self, dependency_info: *const DependencyInfo) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdWriteTimestamp2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp2KHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    ///
    #[doc(alias = "vkCmdWriteTimestamp2KHR")]
    pub unsafe fn cmd_write_timestamp_2_khr(
        self,
        stage: PipelineStageFlags2,
        query_pool: QueryPool,
        query: u32,
    ) {
        todo!()
    }
}

impl Queue {
    /// `vkQueueSubmit2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2KHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkQueueSubmit2KHR")]
    pub unsafe fn queue_submit_2_khr(
        self,
        submit_count: u32,
        submits: *const SubmitInfo2,
        fence: Fence,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBindIndexBuffer3KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer3KHR.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdBindIndexBuffer3KHR")]
    pub unsafe fn cmd_bind_index_buffer_3_khr(self, info: *const BindIndexBuffer3InfoKHR) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBindVertexBuffers3KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers3KHR.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdBindVertexBuffers3KHR")]
    pub unsafe fn cmd_bind_vertex_buffers_3_khr(
        self,
        first_binding: u32,
        binding_count: u32,
        binding_infos: *const BindVertexBuffer3InfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawIndirect2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirect2KHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawIndirect2KHR")]
    pub unsafe fn cmd_draw_indirect_2_khr(self, info: *const DrawIndirect2InfoKHR) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawIndexedIndirect2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirect2KHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawIndexedIndirect2KHR")]
    pub unsafe fn cmd_draw_indexed_indirect_2_khr(self, info: *const DrawIndirect2InfoKHR) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDispatchIndirect2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchIndirect2KHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdDispatchIndirect2KHR")]
    pub unsafe fn cmd_dispatch_indirect_2_khr(self, info: *const DispatchIndirect2InfoKHR) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyMemoryKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    ///
    #[doc(alias = "vkCmdCopyMemoryKHR")]
    pub unsafe fn cmd_copy_memory_khr(self, copy_memory_info: *const CopyDeviceMemoryInfoKHR) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyMemoryToImageKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    ///
    #[doc(alias = "vkCmdCopyMemoryToImageKHR")]
    pub unsafe fn cmd_copy_memory_to_image_khr(
        self,
        copy_memory_info: *const CopyDeviceMemoryImageInfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyImageToMemoryKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToMemoryKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    ///
    #[doc(alias = "vkCmdCopyImageToMemoryKHR")]
    pub unsafe fn cmd_copy_image_to_memory_khr(
        self,
        copy_memory_info: *const CopyDeviceMemoryImageInfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdUpdateMemoryKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdateMemoryKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    ///
    #[doc(alias = "vkCmdUpdateMemoryKHR")]
    pub unsafe fn cmd_update_memory_khr(
        self,
        dst_range: *const DeviceAddressRangeKHR,
        dst_flags: AddressCommandFlagsKHR,
        data_size: DeviceSize,
        data: *const c_void,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdFillMemoryKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdFillMemoryKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    ///
    #[doc(alias = "vkCmdFillMemoryKHR")]
    pub unsafe fn cmd_fill_memory_khr(
        self,
        dst_range: *const DeviceAddressRangeKHR,
        dst_flags: AddressCommandFlagsKHR,
        data: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyQueryPoolResultsToMemoryKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyQueryPoolResultsToMemoryKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    ///
    #[doc(alias = "vkCmdCopyQueryPoolResultsToMemoryKHR")]
    pub unsafe fn cmd_copy_query_pool_results_to_memory_khr(
        self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        dst_range: *const StridedDeviceAddressRangeKHR,
        dst_flags: AddressCommandFlagsKHR,
        query_result_flags: QueryResultFlags,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawIndirectCount2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount2KHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawIndirectCount2KHR")]
    pub unsafe fn cmd_draw_indirect_count_2_khr(self, info: *const DrawIndirectCount2InfoKHR) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawIndexedIndirectCount2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount2KHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawIndexedIndirectCount2KHR")]
    pub unsafe fn cmd_draw_indexed_indirect_count_2_khr(
        self,
        info: *const DrawIndirectCount2InfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBeginConditionalRendering2EXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginConditionalRendering2EXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdBeginConditionalRendering2EXT")]
    pub unsafe fn cmd_begin_conditional_rendering_2_ext(
        self,
        conditional_rendering_begin: *const ConditionalRenderingBeginInfo2EXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBindTransformFeedbackBuffers2EXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTransformFeedbackBuffers2EXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdBindTransformFeedbackBuffers2EXT")]
    pub unsafe fn cmd_bind_transform_feedback_buffers_2_ext(
        self,
        first_binding: u32,
        binding_count: u32,
        binding_infos: *const BindTransformFeedbackBuffer2InfoEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBeginTransformFeedback2EXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginTransformFeedback2EXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdBeginTransformFeedback2EXT")]
    pub unsafe fn cmd_begin_transform_feedback_2_ext(
        self,
        first_counter_range: u32,
        counter_range_count: u32,
        counter_infos: *const BindTransformFeedbackBuffer2InfoEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdEndTransformFeedback2EXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndTransformFeedback2EXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdEndTransformFeedback2EXT")]
    pub unsafe fn cmd_end_transform_feedback_2_ext(
        self,
        first_counter_range: u32,
        counter_range_count: u32,
        counter_infos: *const BindTransformFeedbackBuffer2InfoEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawIndirectByteCount2EXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectByteCount2EXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawIndirectByteCount2EXT")]
    pub unsafe fn cmd_draw_indirect_byte_count_2_ext(
        self,
        instance_count: u32,
        first_instance: u32,
        counter_info: *const BindTransformFeedbackBuffer2InfoEXT,
        counter_offset: u32,
        vertex_stride: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawMeshTasksIndirect2EXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirect2EXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawMeshTasksIndirect2EXT")]
    pub unsafe fn cmd_draw_mesh_tasks_indirect_2_ext(self, info: *const DrawIndirect2InfoKHR) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawMeshTasksIndirectCount2EXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCount2EXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawMeshTasksIndirectCount2EXT")]
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count_2_ext(
        self,
        info: *const DrawIndirectCount2InfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdWriteMarkerToMemoryAMD`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteMarkerToMemoryAMD.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    ///
    #[doc(alias = "vkCmdWriteMarkerToMemoryAMD")]
    pub unsafe fn cmd_write_marker_to_memory_amd(self, info: *const MemoryMarkerInfoAMD) {
        todo!()
    }
}

impl Device {
    /// `vkCreateAccelerationStructure2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructure2KHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`](ResultCode::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    #[doc(alias = "vkCreateAccelerationStructure2KHR")]
    pub unsafe fn create_acceleration_structure_2_khr(
        self,
        create_info: *const AccelerationStructureCreateInfo2KHR,
        allocator: *const AllocationCallbacks,
        acceleration_structure: *mut AccelerationStructureKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyBuffer2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2KHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyBuffer2KHR")]
    pub unsafe fn cmd_copy_buffer_2_khr(self, copy_buffer_info: *const CopyBufferInfo2) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyImage2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2KHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyImage2KHR")]
    pub unsafe fn cmd_copy_image_2_khr(self, copy_image_info: *const CopyImageInfo2) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyBufferToImage2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2KHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyBufferToImage2KHR")]
    pub unsafe fn cmd_copy_buffer_to_image_2_khr(
        self,
        copy_buffer_to_image_info: *const CopyBufferToImageInfo2,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyImageToBuffer2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2KHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyImageToBuffer2KHR")]
    pub unsafe fn cmd_copy_image_to_buffer_2_khr(
        self,
        copy_image_to_buffer_info: *const CopyImageToBufferInfo2,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBlitImage2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2KHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdBlitImage2KHR")]
    pub unsafe fn cmd_blit_image_2_khr(self, blit_image_info: *const BlitImageInfo2) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdResolveImage2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage2KHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdResolveImage2KHR")]
    pub unsafe fn cmd_resolve_image_2_khr(self, resolve_image_info: *const ResolveImageInfo2) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdTraceRaysIndirect2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysIndirect2KHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdTraceRaysIndirect2KHR")]
    pub unsafe fn cmd_trace_rays_indirect_2_khr(self, indirect_device_address: DeviceAddress) {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceBufferMemoryRequirementsKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceBufferMemoryRequirementsKHR.html>
    ///
    #[doc(alias = "vkGetDeviceBufferMemoryRequirementsKHR")]
    pub unsafe fn get_device_buffer_memory_requirements_khr(
        self,
        info: *const DeviceBufferMemoryRequirements,
        memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceImageMemoryRequirementsKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageMemoryRequirementsKHR.html>
    ///
    #[doc(alias = "vkGetDeviceImageMemoryRequirementsKHR")]
    pub unsafe fn get_device_image_memory_requirements_khr(
        self,
        info: *const DeviceImageMemoryRequirements,
        memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceImageSparseMemoryRequirementsKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirementsKHR.html>
    ///
    #[doc(alias = "vkGetDeviceImageSparseMemoryRequirementsKHR")]
    pub unsafe fn get_device_image_sparse_memory_requirements_khr(
        self,
        info: *const DeviceImageMemoryRequirements,
        sparse_memory_requirement_count: *mut u32,
        sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBindIndexBuffer2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer2KHR.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdBindIndexBuffer2KHR")]
    pub unsafe fn cmd_bind_index_buffer_2_khr(
        self,
        buffer: Buffer,
        offset: DeviceSize,
        size: DeviceSize,
        index_type: IndexType,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetRenderingAreaGranularityKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderingAreaGranularityKHR.html>
    ///
    #[doc(alias = "vkGetRenderingAreaGranularityKHR")]
    pub unsafe fn get_rendering_area_granularity_khr(
        self,
        rendering_area_info: *const RenderingAreaInfo,
        granularity: *mut Extent2D,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceImageSubresourceLayoutKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSubresourceLayoutKHR.html>
    ///
    #[doc(alias = "vkGetDeviceImageSubresourceLayoutKHR")]
    pub unsafe fn get_device_image_subresource_layout_khr(
        self,
        info: *const DeviceImageSubresourceInfo,
        layout: *mut SubresourceLayout2,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetImageSubresourceLayout2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2KHR.html>
    ///
    #[doc(alias = "vkGetImageSubresourceLayout2KHR")]
    pub unsafe fn get_image_subresource_layout_2_khr(
        self,
        image: Image,
        subresource: *const ImageSubresource2,
        layout: *mut SubresourceLayout2,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkWaitForPresent2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresent2KHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`TIMEOUT`](ResultCode::TIMEOUT)
    /// - [`SUBOPTIMAL_KHR`](ResultCode::SUBOPTIMAL_KHR)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`OUT_OF_DATE_KHR`](ResultCode::ERROR_OUT_OF_DATE_KHR)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`](ResultCode::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkWaitForPresent2KHR")]
    pub unsafe fn wait_for_present_2_khr(
        self,
        swapchain: SwapchainKHR,
        present_wait_2_info: *const PresentWait2InfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCreatePipelineBinariesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineBinariesKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// - [`PIPELINE_BINARY_MISSING_KHR`](ResultCode::PIPELINE_BINARY_MISSING_KHR)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreatePipelineBinariesKHR")]
    pub unsafe fn create_pipeline_binaries_khr(
        self,
        create_info: *const PipelineBinaryCreateInfoKHR,
        allocator: *const AllocationCallbacks,
        binaries: *mut PipelineBinaryHandlesInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyPipelineBinaryKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineBinaryKHR.html>
    ///
    #[doc(alias = "vkDestroyPipelineBinaryKHR")]
    pub unsafe fn destroy_pipeline_binary_khr(
        self,
        pipeline_binary: PipelineBinaryKHR,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetPipelineKeyKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineKeyKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPipelineKeyKHR")]
    pub unsafe fn get_pipeline_key_khr(
        self,
        pipeline_create_info: *const PipelineCreateInfoKHR,
        pipeline_key: *mut PipelineBinaryKeyKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetPipelineBinaryDataKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineBinaryDataKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`NOT_ENOUGH_SPACE_KHR`](ResultCode::ERROR_NOT_ENOUGH_SPACE_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPipelineBinaryDataKHR")]
    pub unsafe fn get_pipeline_binary_data_khr(
        self,
        info: *const PipelineBinaryDataInfoKHR,
        pipeline_binary_key: *mut PipelineBinaryKeyKHR,
        pipeline_binary_data_size: *mut usize,
        pipeline_binary_data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkReleaseCapturedPipelineDataKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseCapturedPipelineDataKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkReleaseCapturedPipelineDataKHR")]
    pub unsafe fn release_captured_pipeline_data_khr(
        self,
        info: *const ReleaseCapturedPipelineDataInfoKHR,
        allocator: *const AllocationCallbacks,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkReleaseSwapchainImagesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseSwapchainImagesKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkReleaseSwapchainImagesKHR")]
    pub unsafe fn release_swapchain_images_khr(
        self,
        release_info: *const ReleaseSwapchainImagesInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR")]
    pub unsafe fn get_physical_device_cooperative_matrix_properties_khr(
        self,
        property_count: *mut u32,
        properties: *mut CooperativeMatrixPropertiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetLineStippleKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleKHR.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetLineStippleKHR")]
    pub unsafe fn cmd_set_line_stipple_khr(
        self,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceCalibrateableTimeDomainsKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCalibrateableTimeDomainsKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceCalibrateableTimeDomainsKHR")]
    pub unsafe fn get_physical_device_calibrateable_time_domains_khr(
        self,
        time_domain_count: *mut u32,
        time_domains: *mut TimeDomainKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetCalibratedTimestampsKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCalibratedTimestampsKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetCalibratedTimestampsKHR")]
    pub unsafe fn get_calibrated_timestamps_khr(
        self,
        timestamp_count: u32,
        timestamp_infos: *const CalibratedTimestampInfoKHR,
        timestamps: *mut u64,
        max_deviation: *mut u64,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBindDescriptorSets2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2KHR.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdBindDescriptorSets2KHR")]
    pub unsafe fn cmd_bind_descriptor_sets_2_khr(
        self,
        bind_descriptor_sets_info: *const BindDescriptorSetsInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdPushConstants2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2KHR.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdPushConstants2KHR")]
    pub unsafe fn cmd_push_constants_2_khr(self, push_constants_info: *const PushConstantsInfo) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdPushDescriptorSet2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2KHR.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdPushDescriptorSet2KHR")]
    pub unsafe fn cmd_push_descriptor_set_2_khr(
        self,
        push_descriptor_set_info: *const PushDescriptorSetInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdPushDescriptorSetWithTemplate2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate2KHR.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdPushDescriptorSetWithTemplate2KHR")]
    pub unsafe fn cmd_push_descriptor_set_with_template_2_khr(
        self,
        push_descriptor_set_with_template_info: *const PushDescriptorSetWithTemplateInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDescriptorBufferOffsets2EXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDescriptorBufferOffsets2EXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`DATA_GRAPHARM`](QueueFlags::DATA_GRAPHARM)
    ///
    #[doc(alias = "vkCmdSetDescriptorBufferOffsets2EXT")]
    pub unsafe fn cmd_set_descriptor_buffer_offsets_2_ext(
        self,
        set_descriptor_buffer_offsets_info: *const SetDescriptorBufferOffsetsInfoEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBindDescriptorBufferEmbeddedSamplers2EXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBufferEmbeddedSamplers2EXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdBindDescriptorBufferEmbeddedSamplers2EXT")]
    pub unsafe fn cmd_bind_descriptor_buffer_embedded_samplers_2_ext(
        self,
        bind_descriptor_buffer_embedded_samplers_info: *const BindDescriptorBufferEmbeddedSamplersInfoEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyMemoryIndirectKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryIndirectKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyMemoryIndirectKHR")]
    pub unsafe fn cmd_copy_memory_indirect_khr(
        self,
        copy_memory_indirect_info: *const CopyMemoryIndirectInfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyMemoryToImageIndirectKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageIndirectKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyMemoryToImageIndirectKHR")]
    pub unsafe fn cmd_copy_memory_to_image_indirect_khr(
        self,
        copy_memory_to_image_indirect_info: *const CopyMemoryToImageIndirectInfoKHR,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceFaultReportsKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultReportsKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// - [`TIMEOUT`](ResultCode::TIMEOUT)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetDeviceFaultReportsKHR")]
    pub unsafe fn get_device_fault_reports_khr(
        self,
        timeout: u64,
        fault_counts: *mut u32,
        fault_info: *mut DeviceFaultInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceFaultDebugInfoKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultDebugInfoKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`NOT_ENOUGH_SPACE_KHR`](ResultCode::ERROR_NOT_ENOUGH_SPACE_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetDeviceFaultDebugInfoKHR")]
    pub unsafe fn get_device_fault_debug_info_khr(
        self,
        debug_info: *mut DeviceFaultDebugInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdEndRendering2KHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering2KHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdEndRendering2KHR")]
    pub unsafe fn cmd_end_rendering_2_khr(self, rendering_end_info: *const RenderingEndInfoKHR) {
        todo!()
    }
}

impl Instance {
    /// `vkCreateDebugReportCallbackEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDebugReportCallbackEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateDebugReportCallbackEXT")]
    pub unsafe fn create_debug_report_callback_ext(
        self,
        create_info: *const DebugReportCallbackCreateInfoEXT,
        allocator: *const AllocationCallbacks,
        callback: *mut DebugReportCallbackEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Instance {
    /// `vkDestroyDebugReportCallbackEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugReportCallbackEXT.html>
    ///
    #[doc(alias = "vkDestroyDebugReportCallbackEXT")]
    pub unsafe fn destroy_debug_report_callback_ext(
        self,
        callback: DebugReportCallbackEXT,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Instance {
    /// `vkDebugReportMessageEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugReportMessageEXT.html>
    ///
    #[doc(alias = "vkDebugReportMessageEXT")]
    pub unsafe fn debug_report_message_ext(
        self,
        flags: DebugReportFlagsEXT,
        object_type: DebugReportObjectTypeEXT,
        object: u64,
        location: usize,
        message_code: i32,
        layer_prefix: *const c_char,
        message: *const c_char,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkDebugMarkerSetObjectTagEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugMarkerSetObjectTagEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkDebugMarkerSetObjectTagEXT")]
    pub unsafe fn debug_marker_set_object_tag_ext(
        self,
        tag_info: *const DebugMarkerObjectTagInfoEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDebugMarkerSetObjectNameEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugMarkerSetObjectNameEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkDebugMarkerSetObjectNameEXT")]
    pub unsafe fn debug_marker_set_object_name_ext(
        self,
        name_info: *const DebugMarkerObjectNameInfoEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDebugMarkerBeginEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerBeginEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    /// - [`OPTICAL_FLOWNV`](QueueFlags::OPTICAL_FLOWNV)
    ///
    #[doc(alias = "vkCmdDebugMarkerBeginEXT")]
    pub unsafe fn cmd_debug_marker_begin_ext(self, marker_info: *const DebugMarkerMarkerInfoEXT) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDebugMarkerEndEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerEndEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    /// - [`OPTICAL_FLOWNV`](QueueFlags::OPTICAL_FLOWNV)
    ///
    #[doc(alias = "vkCmdDebugMarkerEndEXT")]
    pub unsafe fn cmd_debug_marker_end_ext(self) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDebugMarkerInsertEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerInsertEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    /// - [`OPTICAL_FLOWNV`](QueueFlags::OPTICAL_FLOWNV)
    ///
    #[doc(alias = "vkCmdDebugMarkerInsertEXT")]
    pub unsafe fn cmd_debug_marker_insert_ext(self, marker_info: *const DebugMarkerMarkerInfoEXT) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBindTransformFeedbackBuffersEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTransformFeedbackBuffersEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdBindTransformFeedbackBuffersEXT")]
    pub unsafe fn cmd_bind_transform_feedback_buffers_ext(
        self,
        first_binding: u32,
        binding_count: u32,
        buffers: *const Buffer,
        offsets: *const DeviceSize,
        sizes: *const DeviceSize,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBeginTransformFeedbackEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginTransformFeedbackEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdBeginTransformFeedbackEXT")]
    pub unsafe fn cmd_begin_transform_feedback_ext(
        self,
        first_counter_buffer: u32,
        counter_buffer_count: u32,
        counter_buffers: *const Buffer,
        counter_buffer_offsets: *const DeviceSize,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdEndTransformFeedbackEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndTransformFeedbackEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdEndTransformFeedbackEXT")]
    pub unsafe fn cmd_end_transform_feedback_ext(
        self,
        first_counter_buffer: u32,
        counter_buffer_count: u32,
        counter_buffers: *const Buffer,
        counter_buffer_offsets: *const DeviceSize,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBeginQueryIndexedEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginQueryIndexedEXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    ///
    #[doc(alias = "vkCmdBeginQueryIndexedEXT")]
    pub unsafe fn cmd_begin_query_indexed_ext(
        self,
        query_pool: QueryPool,
        query: u32,
        flags: QueryControlFlags,
        index: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdEndQueryIndexedEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndQueryIndexedEXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    ///
    #[doc(alias = "vkCmdEndQueryIndexedEXT")]
    pub unsafe fn cmd_end_query_indexed_ext(self, query_pool: QueryPool, query: u32, index: u32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawIndirectByteCountEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectByteCountEXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawIndirectByteCountEXT")]
    pub unsafe fn cmd_draw_indirect_byte_count_ext(
        self,
        instance_count: u32,
        first_instance: u32,
        counter_buffer: Buffer,
        counter_buffer_offset: DeviceSize,
        counter_offset: u32,
        vertex_stride: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateCuModuleNVX`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCuModuleNVX.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateCuModuleNVX")]
    pub unsafe fn create_cu_module_nvx(
        self,
        create_info: *const CuModuleCreateInfoNVX,
        allocator: *const AllocationCallbacks,
        module: *mut CuModuleNVX,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCreateCuFunctionNVX`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCuFunctionNVX.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateCuFunctionNVX")]
    pub unsafe fn create_cu_function_nvx(
        self,
        create_info: *const CuFunctionCreateInfoNVX,
        allocator: *const AllocationCallbacks,
        function: *mut CuFunctionNVX,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyCuModuleNVX`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuModuleNVX.html>
    ///
    #[doc(alias = "vkDestroyCuModuleNVX")]
    pub unsafe fn destroy_cu_module_nvx(
        self,
        module: CuModuleNVX,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkDestroyCuFunctionNVX`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuFunctionNVX.html>
    ///
    #[doc(alias = "vkDestroyCuFunctionNVX")]
    pub unsafe fn destroy_cu_function_nvx(
        self,
        function: CuFunctionNVX,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCuLaunchKernelNVX`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCuLaunchKernelNVX.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCuLaunchKernelNVX")]
    pub unsafe fn cmd_cu_launch_kernel_nvx(self, launch_info: *const CuLaunchInfoNVX) {
        todo!()
    }
}

impl Device {
    /// `vkGetImageViewHandleNVX`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewHandleNVX.html>
    ///
    #[doc(alias = "vkGetImageViewHandleNVX")]
    pub unsafe fn get_image_view_handle_nvx(self, info: *const ImageViewHandleInfoNVX) -> u32 {
        todo!()
    }
}

impl Device {
    /// `vkGetImageViewHandle64NVX`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewHandle64NVX.html>
    ///
    #[doc(alias = "vkGetImageViewHandle64NVX")]
    pub unsafe fn get_image_view_handle_64_nvx(self, info: *const ImageViewHandleInfoNVX) -> u64 {
        todo!()
    }
}

impl Device {
    /// `vkGetImageViewAddressNVX`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewAddressNVX.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetImageViewAddressNVX")]
    pub unsafe fn get_image_view_address_nvx(
        self,
        image_view: ImageView,
        properties: *mut ImageViewAddressPropertiesNVX,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceCombinedImageSamplerIndexNVX`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceCombinedImageSamplerIndexNVX.html>
    ///
    #[doc(alias = "vkGetDeviceCombinedImageSamplerIndexNVX")]
    pub unsafe fn get_device_combined_image_sampler_index_nvx(
        self,
        image_view_index: u64,
        sampler_index: u64,
    ) -> u64 {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawIndirectCountAMD`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCountAMD.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawIndirectCountAMD")]
    pub unsafe fn cmd_draw_indirect_count_amd(
        self,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawIndexedIndirectCountAMD`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCountAMD.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawIndexedIndirectCountAMD")]
    pub unsafe fn cmd_draw_indexed_indirect_count_amd(
        self,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetShaderInfoAMD`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderInfoAMD.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`FEATURE_NOT_PRESENT`](ResultCode::ERROR_FEATURE_NOT_PRESENT)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetShaderInfoAMD")]
    pub unsafe fn get_shader_info_amd(
        self,
        pipeline: Pipeline,
        shader_stage: ShaderStageFlags,
        info_type: ShaderInfoTypeAMD,
        info_size: *mut usize,
        info: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Instance {
    /// `vkCreateStreamDescriptorSurfaceGGP`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateStreamDescriptorSurfaceGGP.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`NATIVE_WINDOW_IN_USE_KHR`](ResultCode::ERROR_NATIVE_WINDOW_IN_USE_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateStreamDescriptorSurfaceGGP")]
    pub unsafe fn create_stream_descriptor_surface_ggp(
        self,
        create_info: *const StreamDescriptorSurfaceCreateInfoGGP,
        allocator: *const AllocationCallbacks,
        surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceExternalImageFormatPropertiesNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`FORMAT_NOT_SUPPORTED`](ResultCode::ERROR_FORMAT_NOT_SUPPORTED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceExternalImageFormatPropertiesNV")]
    pub unsafe fn get_physical_device_external_image_format_properties_nv(
        self,
        format: Format,
        type_: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        external_handle_type: ExternalMemoryHandleTypeFlagsNV,
        external_image_format_properties: *mut ExternalImageFormatPropertiesNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetMemoryWin32HandleNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`TOO_MANY_OBJECTS`](ResultCode::ERROR_TOO_MANY_OBJECTS)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetMemoryWin32HandleNV")]
    pub unsafe fn get_memory_win_32_handle_nv(
        self,
        memory: DeviceMemory,
        handle_type: ExternalMemoryHandleTypeFlagsNV,
        handle: *mut HANDLE,
    ) -> ResultCode {
        todo!()
    }
}

impl Instance {
    /// `vkCreateViSurfaceNN`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateViSurfaceNN.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`NATIVE_WINDOW_IN_USE_KHR`](ResultCode::ERROR_NATIVE_WINDOW_IN_USE_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateViSurfaceNN")]
    pub unsafe fn create_vi_surface_nn(
        self,
        create_info: *const ViSurfaceCreateInfoNN,
        allocator: *const AllocationCallbacks,
        surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBeginConditionalRenderingEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginConditionalRenderingEXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdBeginConditionalRenderingEXT")]
    pub unsafe fn cmd_begin_conditional_rendering_ext(
        self,
        conditional_rendering_begin: *const ConditionalRenderingBeginInfoEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdEndConditionalRenderingEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndConditionalRenderingEXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdEndConditionalRenderingEXT")]
    pub unsafe fn cmd_end_conditional_rendering_ext(self) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetViewportWScalingNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWScalingNV.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetViewportWScalingNV")]
    pub unsafe fn cmd_set_viewport_w_scaling_nv(
        self,
        first_viewport: u32,
        viewport_count: u32,
        viewport_w_scalings: *const ViewportWScalingNV,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkReleaseDisplayEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseDisplayEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkReleaseDisplayEXT")]
    pub unsafe fn release_display_ext(self, display: DisplayKHR) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkAcquireXlibDisplayEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireXlibDisplayEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkAcquireXlibDisplayEXT")]
    pub unsafe fn acquire_xlib_display_ext(
        self,
        dpy: *mut Display,
        display: DisplayKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetRandROutputDisplayEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRandROutputDisplayEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetRandROutputDisplayEXT")]
    pub unsafe fn get_rand_r_output_display_ext(
        self,
        dpy: *mut Display,
        rr_output: RROutput,
        display: *mut DisplayKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceSurfaceCapabilities2EXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2EXT")]
    pub unsafe fn get_physical_device_surface_capabilities_2_ext(
        self,
        surface: SurfaceKHR,
        surface_capabilities: *mut SurfaceCapabilities2EXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDisplayPowerControlEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDisplayPowerControlEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkDisplayPowerControlEXT")]
    pub unsafe fn display_power_control_ext(
        self,
        display: DisplayKHR,
        display_power_info: *const DisplayPowerInfoEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkRegisterDeviceEventEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterDeviceEventEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkRegisterDeviceEventEXT")]
    pub unsafe fn register_device_event_ext(
        self,
        device_event_info: *const DeviceEventInfoEXT,
        allocator: *const AllocationCallbacks,
        fence: *mut Fence,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkRegisterDisplayEventEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterDisplayEventEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkRegisterDisplayEventEXT")]
    pub unsafe fn register_display_event_ext(
        self,
        display: DisplayKHR,
        display_event_info: *const DisplayEventInfoEXT,
        allocator: *const AllocationCallbacks,
        fence: *mut Fence,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetSwapchainCounterEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainCounterEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`OUT_OF_DATE_KHR`](ResultCode::ERROR_OUT_OF_DATE_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetSwapchainCounterEXT")]
    pub unsafe fn get_swapchain_counter_ext(
        self,
        swapchain: SwapchainKHR,
        counter: SurfaceCounterFlagsEXT,
        counter_value: *mut u64,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetRefreshCycleDurationGOOGLE`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRefreshCycleDurationGOOGLE.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetRefreshCycleDurationGOOGLE")]
    pub unsafe fn get_refresh_cycle_duration_google(
        self,
        swapchain: SwapchainKHR,
        display_timing_properties: *mut RefreshCycleDurationGOOGLE,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetPastPresentationTimingGOOGLE`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingGOOGLE.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`OUT_OF_DATE_KHR`](ResultCode::ERROR_OUT_OF_DATE_KHR)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPastPresentationTimingGOOGLE")]
    pub unsafe fn get_past_presentation_timing_google(
        self,
        swapchain: SwapchainKHR,
        presentation_timing_count: *mut u32,
        presentation_timings: *mut PastPresentationTimingGOOGLE,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDiscardRectangleEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetDiscardRectangleEXT")]
    pub unsafe fn cmd_set_discard_rectangle_ext(
        self,
        first_discard_rectangle: u32,
        discard_rectangle_count: u32,
        discard_rectangles: *const Rect2D,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDiscardRectangleEnableEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleEnableEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetDiscardRectangleEnableEXT")]
    pub unsafe fn cmd_set_discard_rectangle_enable_ext(self, discard_rectangle_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDiscardRectangleModeEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleModeEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetDiscardRectangleModeEXT")]
    pub unsafe fn cmd_set_discard_rectangle_mode_ext(
        self,
        discard_rectangle_mode: DiscardRectangleModeEXT,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkSetHdrMetadataEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetHdrMetadataEXT.html>
    ///
    #[doc(alias = "vkSetHdrMetadataEXT")]
    pub unsafe fn set_hdr_metadata_ext(
        self,
        swapchain_count: u32,
        swapchains: *const SwapchainKHR,
        metadata: *const HdrMetadataEXT,
    ) {
        todo!()
    }
}

impl Instance {
    /// `vkCreateIOSSurfaceMVK`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIOSSurfaceMVK.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`NATIVE_WINDOW_IN_USE_KHR`](ResultCode::ERROR_NATIVE_WINDOW_IN_USE_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateIOSSurfaceMVK")]
    pub unsafe fn create_ios_surface_mvk(
        self,
        create_info: *const IOSSurfaceCreateInfoMVK,
        allocator: *const AllocationCallbacks,
        surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Instance {
    /// `vkCreateMacOSSurfaceMVK`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMacOSSurfaceMVK.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`NATIVE_WINDOW_IN_USE_KHR`](ResultCode::ERROR_NATIVE_WINDOW_IN_USE_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateMacOSSurfaceMVK")]
    pub unsafe fn create_mac_os_surface_mvk(
        self,
        create_info: *const MacOSSurfaceCreateInfoMVK,
        allocator: *const AllocationCallbacks,
        surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkSetDebugUtilsObjectNameEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDebugUtilsObjectNameEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkSetDebugUtilsObjectNameEXT")]
    pub unsafe fn set_debug_utils_object_name_ext(
        self,
        name_info: *const DebugUtilsObjectNameInfoEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkSetDebugUtilsObjectTagEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDebugUtilsObjectTagEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkSetDebugUtilsObjectTagEXT")]
    pub unsafe fn set_debug_utils_object_tag_ext(
        self,
        tag_info: *const DebugUtilsObjectTagInfoEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Queue {
    /// `vkQueueBeginDebugUtilsLabelEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBeginDebugUtilsLabelEXT.html>
    ///
    #[doc(alias = "vkQueueBeginDebugUtilsLabelEXT")]
    pub unsafe fn queue_begin_debug_utils_label_ext(self, label_info: *const DebugUtilsLabelEXT) {
        todo!()
    }
}

impl Queue {
    /// `vkQueueEndDebugUtilsLabelEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueEndDebugUtilsLabelEXT.html>
    ///
    #[doc(alias = "vkQueueEndDebugUtilsLabelEXT")]
    pub unsafe fn queue_end_debug_utils_label_ext(self) {
        todo!()
    }
}

impl Queue {
    /// `vkQueueInsertDebugUtilsLabelEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueInsertDebugUtilsLabelEXT.html>
    ///
    #[doc(alias = "vkQueueInsertDebugUtilsLabelEXT")]
    pub unsafe fn queue_insert_debug_utils_label_ext(self, label_info: *const DebugUtilsLabelEXT) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBeginDebugUtilsLabelEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginDebugUtilsLabelEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    /// - [`OPTICAL_FLOWNV`](QueueFlags::OPTICAL_FLOWNV)
    ///
    #[doc(alias = "vkCmdBeginDebugUtilsLabelEXT")]
    pub unsafe fn cmd_begin_debug_utils_label_ext(self, label_info: *const DebugUtilsLabelEXT) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdEndDebugUtilsLabelEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndDebugUtilsLabelEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    /// - [`OPTICAL_FLOWNV`](QueueFlags::OPTICAL_FLOWNV)
    ///
    #[doc(alias = "vkCmdEndDebugUtilsLabelEXT")]
    pub unsafe fn cmd_end_debug_utils_label_ext(self) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdInsertDebugUtilsLabelEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdInsertDebugUtilsLabelEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
    /// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
    /// - [`OPTICAL_FLOWNV`](QueueFlags::OPTICAL_FLOWNV)
    ///
    #[doc(alias = "vkCmdInsertDebugUtilsLabelEXT")]
    pub unsafe fn cmd_insert_debug_utils_label_ext(self, label_info: *const DebugUtilsLabelEXT) {
        todo!()
    }
}

impl Instance {
    /// `vkCreateDebugUtilsMessengerEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDebugUtilsMessengerEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateDebugUtilsMessengerEXT")]
    pub unsafe fn create_debug_utils_messenger_ext(
        self,
        create_info: *const DebugUtilsMessengerCreateInfoEXT,
        allocator: *const AllocationCallbacks,
        messenger: *mut DebugUtilsMessengerEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Instance {
    /// `vkDestroyDebugUtilsMessengerEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugUtilsMessengerEXT.html>
    ///
    #[doc(alias = "vkDestroyDebugUtilsMessengerEXT")]
    pub unsafe fn destroy_debug_utils_messenger_ext(
        self,
        messenger: DebugUtilsMessengerEXT,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Instance {
    /// `vkSubmitDebugUtilsMessageEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSubmitDebugUtilsMessageEXT.html>
    ///
    #[doc(alias = "vkSubmitDebugUtilsMessageEXT")]
    pub unsafe fn submit_debug_utils_message_ext(
        self,
        message_severity: DebugUtilsMessageSeverityFlagsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        callback_data: *const DebugUtilsMessengerCallbackDataEXT,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetAndroidHardwareBufferPropertiesANDROID`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAndroidHardwareBufferPropertiesANDROID.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`INVALID_EXTERNAL_HANDLE_KHR`](ResultCode::ERROR_INVALID_EXTERNAL_HANDLE_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetAndroidHardwareBufferPropertiesANDROID")]
    pub unsafe fn get_android_hardware_buffer_properties_android(
        self,
        buffer: *const AHardwareBuffer,
        properties: *mut AndroidHardwareBufferPropertiesANDROID,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetMemoryAndroidHardwareBufferANDROID`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryAndroidHardwareBufferANDROID.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`TOO_MANY_OBJECTS`](ResultCode::ERROR_TOO_MANY_OBJECTS)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetMemoryAndroidHardwareBufferANDROID")]
    pub unsafe fn get_memory_android_hardware_buffer_android(
        self,
        info: *const MemoryGetAndroidHardwareBufferInfoANDROID,
        buffer: *mut *mut AHardwareBuffer,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCreateGpaSessionAMD`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateGpaSessionAMD.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateGpaSessionAMD")]
    pub unsafe fn create_gpa_session_amd(
        self,
        create_info: *const GpaSessionCreateInfoAMD,
        allocator: *const AllocationCallbacks,
        gpa_session: *mut GpaSessionAMD,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyGpaSessionAMD`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyGpaSessionAMD.html>
    ///
    #[doc(alias = "vkDestroyGpaSessionAMD")]
    pub unsafe fn destroy_gpa_session_amd(
        self,
        gpa_session: GpaSessionAMD,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkSetGpaDeviceClockModeAMD`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetGpaDeviceClockModeAMD.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkSetGpaDeviceClockModeAMD")]
    pub unsafe fn set_gpa_device_clock_mode_amd(
        self,
        info: *mut GpaDeviceClockModeInfoAMD,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetGpaDeviceClockInfoAMD`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaDeviceClockInfoAMD.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetGpaDeviceClockInfoAMD")]
    pub unsafe fn get_gpa_device_clock_info_amd(
        self,
        info: *mut GpaDeviceGetClockInfoAMD,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBeginGpaSessionAMD`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginGpaSessionAMD.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCmdBeginGpaSessionAMD")]
    pub unsafe fn cmd_begin_gpa_session_amd(self, gpa_session: GpaSessionAMD) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdEndGpaSessionAMD`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndGpaSessionAMD.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCmdEndGpaSessionAMD")]
    pub unsafe fn cmd_end_gpa_session_amd(self, gpa_session: GpaSessionAMD) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBeginGpaSampleAMD`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginGpaSampleAMD.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCmdBeginGpaSampleAMD")]
    pub unsafe fn cmd_begin_gpa_sample_amd(
        self,
        gpa_session: GpaSessionAMD,
        gpa_sample_begin_info: *const GpaSampleBeginInfoAMD,
        sample_id: *mut u32,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdEndGpaSampleAMD`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndGpaSampleAMD.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdEndGpaSampleAMD")]
    pub unsafe fn cmd_end_gpa_sample_amd(self, gpa_session: GpaSessionAMD, sample_id: u32) {
        todo!()
    }
}

impl Device {
    /// `vkGetGpaSessionStatusAMD`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaSessionStatusAMD.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetGpaSessionStatusAMD")]
    pub unsafe fn get_gpa_session_status_amd(self, gpa_session: GpaSessionAMD) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetGpaSessionResultsAMD`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaSessionResultsAMD.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetGpaSessionResultsAMD")]
    pub unsafe fn get_gpa_session_results_amd(
        self,
        gpa_session: GpaSessionAMD,
        sample_id: u32,
        size_in_bytes: *mut usize,
        data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkResetGpaSessionAMD`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkResetGpaSessionAMD.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkResetGpaSessionAMD")]
    pub unsafe fn reset_gpa_session_amd(self, gpa_session: GpaSessionAMD) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyGpaSessionResultsAMD`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyGpaSessionResultsAMD.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    ///
    #[doc(alias = "vkCmdCopyGpaSessionResultsAMD")]
    pub unsafe fn cmd_copy_gpa_session_results_amd(self, gpa_session: GpaSessionAMD) {
        todo!()
    }
}

impl Device {
    /// `vkCreateExecutionGraphPipelinesAMDX`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateExecutionGraphPipelinesAMDX.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`PIPELINE_COMPILE_REQUIRED_EXT`](ResultCode::PIPELINE_COMPILE_REQUIRED_EXT)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateExecutionGraphPipelinesAMDX")]
    pub unsafe fn create_execution_graph_pipelines_amdx(
        self,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        create_infos: *const ExecutionGraphPipelineCreateInfoAMDX,
        allocator: *const AllocationCallbacks,
        pipelines: *mut Pipeline,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetExecutionGraphPipelineScratchSizeAMDX`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExecutionGraphPipelineScratchSizeAMDX.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetExecutionGraphPipelineScratchSizeAMDX")]
    pub unsafe fn get_execution_graph_pipeline_scratch_size_amdx(
        self,
        execution_graph: Pipeline,
        size_info: *mut ExecutionGraphPipelineScratchSizeAMDX,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetExecutionGraphPipelineNodeIndexAMDX`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExecutionGraphPipelineNodeIndexAMDX.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetExecutionGraphPipelineNodeIndexAMDX")]
    pub unsafe fn get_execution_graph_pipeline_node_index_amdx(
        self,
        execution_graph: Pipeline,
        node_info: *const PipelineShaderStageNodeCreateInfoAMDX,
        node_index: *mut u32,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdInitializeGraphScratchMemoryAMDX`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdInitializeGraphScratchMemoryAMDX.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdInitializeGraphScratchMemoryAMDX")]
    pub unsafe fn cmd_initialize_graph_scratch_memory_amdx(
        self,
        execution_graph: Pipeline,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDispatchGraphAMDX`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphAMDX.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdDispatchGraphAMDX")]
    pub unsafe fn cmd_dispatch_graph_amdx(
        self,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: *const DispatchGraphCountInfoAMDX,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDispatchGraphIndirectAMDX`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphIndirectAMDX.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdDispatchGraphIndirectAMDX")]
    pub unsafe fn cmd_dispatch_graph_indirect_amdx(
        self,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: *const DispatchGraphCountInfoAMDX,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDispatchGraphIndirectCountAMDX`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphIndirectCountAMDX.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdDispatchGraphIndirectCountAMDX")]
    pub unsafe fn cmd_dispatch_graph_indirect_count_amdx(
        self,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: DeviceAddress,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkWriteSamplerDescriptorsEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteSamplerDescriptorsEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkWriteSamplerDescriptorsEXT")]
    pub unsafe fn write_sampler_descriptors_ext(
        self,
        sampler_count: u32,
        samplers: *const SamplerCreateInfo,
        descriptors: *const HostAddressRangeEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkWriteResourceDescriptorsEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteResourceDescriptorsEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkWriteResourceDescriptorsEXT")]
    pub unsafe fn write_resource_descriptors_ext(
        self,
        resource_count: u32,
        resources: *const ResourceDescriptorInfoEXT,
        descriptors: *const HostAddressRangeEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBindSamplerHeapEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindSamplerHeapEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdBindSamplerHeapEXT")]
    pub unsafe fn cmd_bind_sampler_heap_ext(self, bind_info: *const BindHeapInfoEXT) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBindResourceHeapEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindResourceHeapEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdBindResourceHeapEXT")]
    pub unsafe fn cmd_bind_resource_heap_ext(self, bind_info: *const BindHeapInfoEXT) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdPushDataEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDataEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdPushDataEXT")]
    pub unsafe fn cmd_push_data_ext(self, push_data_info: *const PushDataInfoEXT) {
        todo!()
    }
}

impl Device {
    /// `vkGetImageOpaqueCaptureDataEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageOpaqueCaptureDataEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetImageOpaqueCaptureDataEXT")]
    pub unsafe fn get_image_opaque_capture_data_ext(
        self,
        image_count: u32,
        images: *const Image,
        datas: *mut HostAddressRangeEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceDescriptorSizeEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDescriptorSizeEXT.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceDescriptorSizeEXT")]
    pub unsafe fn get_physical_device_descriptor_size_ext(
        self,
        descriptor_type: DescriptorType,
    ) -> DeviceSize {
        todo!()
    }
}

impl Device {
    /// `vkRegisterCustomBorderColorEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterCustomBorderColorEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`TOO_MANY_OBJECTS`](ResultCode::ERROR_TOO_MANY_OBJECTS)
    /// - [`INVALID_OPAQUE_CAPTURE_ADDRESS`](ResultCode::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkRegisterCustomBorderColorEXT")]
    pub unsafe fn register_custom_border_color_ext(
        self,
        border_color: *const SamplerCustomBorderColorCreateInfoEXT,
        request_index: Bool32,
        index: *mut u32,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkUnregisterCustomBorderColorEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUnregisterCustomBorderColorEXT.html>
    ///
    #[doc(alias = "vkUnregisterCustomBorderColorEXT")]
    pub unsafe fn unregister_custom_border_color_ext(self, index: u32) {
        todo!()
    }
}

impl Device {
    /// `vkGetTensorOpaqueCaptureDataARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDataARM.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetTensorOpaqueCaptureDataARM")]
    pub unsafe fn get_tensor_opaque_capture_data_arm(
        self,
        tensor_count: u32,
        tensors: *const TensorARM,
        datas: *mut HostAddressRangeEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetSampleLocationsEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleLocationsEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetSampleLocationsEXT")]
    pub unsafe fn cmd_set_sample_locations_ext(
        self,
        sample_locations_info: *const SampleLocationsInfoEXT,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceMultisamplePropertiesEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMultisamplePropertiesEXT.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceMultisamplePropertiesEXT")]
    pub unsafe fn get_physical_device_multisample_properties_ext(
        self,
        samples: SampleCountFlags,
        multisample_properties: *mut MultisamplePropertiesEXT,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetImageDrmFormatModifierPropertiesEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageDrmFormatModifierPropertiesEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetImageDrmFormatModifierPropertiesEXT")]
    pub unsafe fn get_image_drm_format_modifier_properties_ext(
        self,
        image: Image,
        properties: *mut ImageDrmFormatModifierPropertiesEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCreateValidationCacheEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateValidationCacheEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateValidationCacheEXT")]
    pub unsafe fn create_validation_cache_ext(
        self,
        create_info: *const ValidationCacheCreateInfoEXT,
        allocator: *const AllocationCallbacks,
        validation_cache: *mut ValidationCacheEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyValidationCacheEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyValidationCacheEXT.html>
    ///
    #[doc(alias = "vkDestroyValidationCacheEXT")]
    pub unsafe fn destroy_validation_cache_ext(
        self,
        validation_cache: ValidationCacheEXT,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkMergeValidationCachesEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkMergeValidationCachesEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkMergeValidationCachesEXT")]
    pub unsafe fn merge_validation_caches_ext(
        self,
        dst_cache: ValidationCacheEXT,
        src_cache_count: u32,
        src_caches: *const ValidationCacheEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetValidationCacheDataEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetValidationCacheDataEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetValidationCacheDataEXT")]
    pub unsafe fn get_validation_cache_data_ext(
        self,
        validation_cache: ValidationCacheEXT,
        data_size: *mut usize,
        data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBindShadingRateImageNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindShadingRateImageNV.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdBindShadingRateImageNV")]
    pub unsafe fn cmd_bind_shading_rate_image_nv(
        self,
        image_view: ImageView,
        image_layout: ImageLayout,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetViewportShadingRatePaletteNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportShadingRatePaletteNV.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetViewportShadingRatePaletteNV")]
    pub unsafe fn cmd_set_viewport_shading_rate_palette_nv(
        self,
        first_viewport: u32,
        viewport_count: u32,
        shading_rate_palettes: *const ShadingRatePaletteNV,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetCoarseSampleOrderNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoarseSampleOrderNV.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetCoarseSampleOrderNV")]
    pub unsafe fn cmd_set_coarse_sample_order_nv(
        self,
        sample_order_type: CoarseSampleOrderTypeNV,
        custom_sample_order_count: u32,
        custom_sample_orders: *const CoarseSampleOrderCustomNV,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateAccelerationStructureNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructureNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateAccelerationStructureNV")]
    pub unsafe fn create_acceleration_structure_nv(
        self,
        create_info: *const AccelerationStructureCreateInfoNV,
        allocator: *const AllocationCallbacks,
        acceleration_structure: *mut AccelerationStructureNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyAccelerationStructureNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureNV.html>
    ///
    #[doc(alias = "vkDestroyAccelerationStructureNV")]
    pub unsafe fn destroy_acceleration_structure_nv(
        self,
        acceleration_structure: AccelerationStructureNV,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetAccelerationStructureMemoryRequirementsNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureMemoryRequirementsNV.html>
    ///
    #[doc(alias = "vkGetAccelerationStructureMemoryRequirementsNV")]
    pub unsafe fn get_acceleration_structure_memory_requirements_nv(
        self,
        info: *const AccelerationStructureMemoryRequirementsInfoNV,
        memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkBindAccelerationStructureMemoryNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindAccelerationStructureMemoryNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkBindAccelerationStructureMemoryNV")]
    pub unsafe fn bind_acceleration_structure_memory_nv(
        self,
        bind_info_count: u32,
        bind_infos: *const BindAccelerationStructureMemoryInfoNV,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBuildAccelerationStructureNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructureNV.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdBuildAccelerationStructureNV")]
    pub unsafe fn cmd_build_acceleration_structure_nv(
        self,
        info: *const AccelerationStructureInfoNV,
        instance_data: Buffer,
        instance_offset: DeviceSize,
        update: Bool32,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        scratch: Buffer,
        scratch_offset: DeviceSize,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyAccelerationStructureNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureNV.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyAccelerationStructureNV")]
    pub unsafe fn cmd_copy_acceleration_structure_nv(
        self,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        mode: CopyAccelerationStructureModeKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdTraceRaysNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysNV.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdTraceRaysNV")]
    pub unsafe fn cmd_trace_rays_nv(
        self,
        raygen_shader_binding_table_buffer: Buffer,
        raygen_shader_binding_offset: DeviceSize,
        miss_shader_binding_table_buffer: Buffer,
        miss_shader_binding_offset: DeviceSize,
        miss_shader_binding_stride: DeviceSize,
        hit_shader_binding_table_buffer: Buffer,
        hit_shader_binding_offset: DeviceSize,
        hit_shader_binding_stride: DeviceSize,
        callable_shader_binding_table_buffer: Buffer,
        callable_shader_binding_offset: DeviceSize,
        callable_shader_binding_stride: DeviceSize,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateRayTracingPipelinesNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRayTracingPipelinesNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`PIPELINE_COMPILE_REQUIRED_EXT`](ResultCode::PIPELINE_COMPILE_REQUIRED_EXT)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INVALID_SHADER_NV`](ResultCode::ERROR_INVALID_SHADER_NV)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateRayTracingPipelinesNV")]
    pub unsafe fn create_ray_tracing_pipelines_nv(
        self,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        create_infos: *const RayTracingPipelineCreateInfoNV,
        allocator: *const AllocationCallbacks,
        pipelines: *mut Pipeline,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetRayTracingShaderGroupHandlesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupHandlesKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetRayTracingShaderGroupHandlesKHR")]
    pub unsafe fn get_ray_tracing_shader_group_handles_khr(
        self,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetRayTracingShaderGroupHandlesNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupHandlesNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetRayTracingShaderGroupHandlesNV")]
    pub unsafe fn get_ray_tracing_shader_group_handles_nv(
        self,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetAccelerationStructureHandleNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureHandleNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetAccelerationStructureHandleNV")]
    pub unsafe fn get_acceleration_structure_handle_nv(
        self,
        acceleration_structure: AccelerationStructureNV,
        data_size: usize,
        data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdWriteAccelerationStructuresPropertiesNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteAccelerationStructuresPropertiesNV.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesNV")]
    pub unsafe fn cmd_write_acceleration_structures_properties_nv(
        self,
        acceleration_structure_count: u32,
        acceleration_structures: *const AccelerationStructureNV,
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCompileDeferredNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCompileDeferredNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCompileDeferredNV")]
    pub unsafe fn compile_deferred_nv(self, pipeline: Pipeline, shader: u32) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetMemoryHostPointerPropertiesEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryHostPointerPropertiesEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`INVALID_EXTERNAL_HANDLE`](ResultCode::ERROR_INVALID_EXTERNAL_HANDLE)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetMemoryHostPointerPropertiesEXT")]
    pub unsafe fn get_memory_host_pointer_properties_ext(
        self,
        handle_type: ExternalMemoryHandleTypeFlags,
        host_pointer: *const c_void,
        memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdWriteBufferMarkerAMD`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteBufferMarkerAMD.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdWriteBufferMarkerAMD")]
    pub unsafe fn cmd_write_buffer_marker_amd(
        self,
        pipeline_stage: PipelineStageFlags,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        marker: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdWriteBufferMarker2AMD`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteBufferMarker2AMD.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdWriteBufferMarker2AMD")]
    pub unsafe fn cmd_write_buffer_marker_2_amd(
        self,
        stage: PipelineStageFlags2,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        marker: u32,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceCalibrateableTimeDomainsEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceCalibrateableTimeDomainsEXT")]
    pub unsafe fn get_physical_device_calibrateable_time_domains_ext(
        self,
        time_domain_count: *mut u32,
        time_domains: *mut TimeDomainKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetCalibratedTimestampsEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCalibratedTimestampsEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetCalibratedTimestampsEXT")]
    pub unsafe fn get_calibrated_timestamps_ext(
        self,
        timestamp_count: u32,
        timestamp_infos: *const CalibratedTimestampInfoKHR,
        timestamps: *mut u64,
        max_deviation: *mut u64,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawMeshTasksNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksNV.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawMeshTasksNV")]
    pub unsafe fn cmd_draw_mesh_tasks_nv(self, task_count: u32, first_task: u32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawMeshTasksIndirectNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectNV.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawMeshTasksIndirectNV")]
    pub unsafe fn cmd_draw_mesh_tasks_indirect_nv(
        self,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawMeshTasksIndirectCountNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCountNV.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawMeshTasksIndirectCountNV")]
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count_nv(
        self,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetExclusiveScissorEnableNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExclusiveScissorEnableNV.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetExclusiveScissorEnableNV")]
    pub unsafe fn cmd_set_exclusive_scissor_enable_nv(
        self,
        first_exclusive_scissor: u32,
        exclusive_scissor_count: u32,
        exclusive_scissor_enables: *const Bool32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetExclusiveScissorNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExclusiveScissorNV.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetExclusiveScissorNV")]
    pub unsafe fn cmd_set_exclusive_scissor_nv(
        self,
        first_exclusive_scissor: u32,
        exclusive_scissor_count: u32,
        exclusive_scissors: *const Rect2D,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetCheckpointNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCheckpointNV.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    ///
    #[doc(alias = "vkCmdSetCheckpointNV")]
    pub unsafe fn cmd_set_checkpoint_nv(self, checkpoint_marker: *const c_void) {
        todo!()
    }
}

impl Queue {
    /// `vkGetQueueCheckpointDataNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointDataNV.html>
    ///
    #[doc(alias = "vkGetQueueCheckpointDataNV")]
    pub unsafe fn get_queue_checkpoint_data_nv(
        self,
        checkpoint_data_count: *mut u32,
        checkpoint_data: *mut CheckpointDataNV,
    ) {
        todo!()
    }
}

impl Queue {
    /// `vkGetQueueCheckpointData2NV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointData2NV.html>
    ///
    #[doc(alias = "vkGetQueueCheckpointData2NV")]
    pub unsafe fn get_queue_checkpoint_data_2_nv(
        self,
        checkpoint_data_count: *mut u32,
        checkpoint_data: *mut CheckpointData2NV,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkSetSwapchainPresentTimingQueueSizeEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetSwapchainPresentTimingQueueSizeEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`NOT_READY`](ResultCode::NOT_READY)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkSetSwapchainPresentTimingQueueSizeEXT")]
    pub unsafe fn set_swapchain_present_timing_queue_size_ext(
        self,
        swapchain: SwapchainKHR,
        size: u32,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetSwapchainTimingPropertiesEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainTimingPropertiesEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`NOT_READY`](ResultCode::NOT_READY)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetSwapchainTimingPropertiesEXT")]
    pub unsafe fn get_swapchain_timing_properties_ext(
        self,
        swapchain: SwapchainKHR,
        swapchain_timing_properties: *mut SwapchainTimingPropertiesEXT,
        swapchain_timing_properties_counter: *mut u64,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetSwapchainTimeDomainPropertiesEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainTimeDomainPropertiesEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetSwapchainTimeDomainPropertiesEXT")]
    pub unsafe fn get_swapchain_time_domain_properties_ext(
        self,
        swapchain: SwapchainKHR,
        swapchain_time_domain_properties: *mut SwapchainTimeDomainPropertiesEXT,
        time_domains_counter: *mut u64,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetPastPresentationTimingEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`OUT_OF_DATE_KHR`](ResultCode::ERROR_OUT_OF_DATE_KHR)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPastPresentationTimingEXT")]
    pub unsafe fn get_past_presentation_timing_ext(
        self,
        past_presentation_timing_info: *const PastPresentationTimingInfoEXT,
        past_presentation_timing_properties: *mut PastPresentationTimingPropertiesEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkInitializePerformanceApiINTEL`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkInitializePerformanceApiINTEL.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`TOO_MANY_OBJECTS`](ResultCode::ERROR_TOO_MANY_OBJECTS)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkInitializePerformanceApiINTEL")]
    pub unsafe fn initialize_performance_api_intel(
        self,
        initialize_info: *const InitializePerformanceApiInfoINTEL,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkUninitializePerformanceApiINTEL`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUninitializePerformanceApiINTEL.html>
    ///
    #[doc(alias = "vkUninitializePerformanceApiINTEL")]
    pub unsafe fn uninitialize_performance_api_intel(self) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetPerformanceMarkerINTEL`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceMarkerINTEL.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`TOO_MANY_OBJECTS`](ResultCode::ERROR_TOO_MANY_OBJECTS)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCmdSetPerformanceMarkerINTEL")]
    pub unsafe fn cmd_set_performance_marker_intel(
        self,
        marker_info: *const PerformanceMarkerInfoINTEL,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetPerformanceStreamMarkerINTEL`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceStreamMarkerINTEL.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`TOO_MANY_OBJECTS`](ResultCode::ERROR_TOO_MANY_OBJECTS)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCmdSetPerformanceStreamMarkerINTEL")]
    pub unsafe fn cmd_set_performance_stream_marker_intel(
        self,
        marker_info: *const PerformanceStreamMarkerInfoINTEL,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetPerformanceOverrideINTEL`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceOverrideINTEL.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`TOO_MANY_OBJECTS`](ResultCode::ERROR_TOO_MANY_OBJECTS)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCmdSetPerformanceOverrideINTEL")]
    pub unsafe fn cmd_set_performance_override_intel(
        self,
        override_info: *const PerformanceOverrideInfoINTEL,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkAcquirePerformanceConfigurationINTEL`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquirePerformanceConfigurationINTEL.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`TOO_MANY_OBJECTS`](ResultCode::ERROR_TOO_MANY_OBJECTS)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkAcquirePerformanceConfigurationINTEL")]
    pub unsafe fn acquire_performance_configuration_intel(
        self,
        acquire_info: *const PerformanceConfigurationAcquireInfoINTEL,
        configuration: *mut PerformanceConfigurationINTEL,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkReleasePerformanceConfigurationINTEL`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleasePerformanceConfigurationINTEL.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`TOO_MANY_OBJECTS`](ResultCode::ERROR_TOO_MANY_OBJECTS)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkReleasePerformanceConfigurationINTEL")]
    pub unsafe fn release_performance_configuration_intel(
        self,
        configuration: PerformanceConfigurationINTEL,
    ) -> ResultCode {
        todo!()
    }
}

impl Queue {
    /// `vkQueueSetPerformanceConfigurationINTEL`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSetPerformanceConfigurationINTEL.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`TOO_MANY_OBJECTS`](ResultCode::ERROR_TOO_MANY_OBJECTS)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkQueueSetPerformanceConfigurationINTEL")]
    pub unsafe fn queue_set_performance_configuration_intel(
        self,
        configuration: PerformanceConfigurationINTEL,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetPerformanceParameterINTEL`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPerformanceParameterINTEL.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`TOO_MANY_OBJECTS`](ResultCode::ERROR_TOO_MANY_OBJECTS)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPerformanceParameterINTEL")]
    pub unsafe fn get_performance_parameter_intel(
        self,
        parameter: PerformanceParameterTypeINTEL,
        value: *mut PerformanceValueINTEL,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkSetLocalDimmingAMD`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLocalDimmingAMD.html>
    ///
    #[doc(alias = "vkSetLocalDimmingAMD")]
    pub unsafe fn set_local_dimming_amd(
        self,
        swap_chain: SwapchainKHR,
        local_dimming_enable: Bool32,
    ) {
        todo!()
    }
}

impl Instance {
    /// `vkCreateImagePipeSurfaceFUCHSIA`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImagePipeSurfaceFUCHSIA.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateImagePipeSurfaceFUCHSIA")]
    pub unsafe fn create_image_pipe_surface_fuchsia(
        self,
        create_info: *const ImagePipeSurfaceCreateInfoFUCHSIA,
        allocator: *const AllocationCallbacks,
        surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Instance {
    /// `vkCreateMetalSurfaceEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMetalSurfaceEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`NATIVE_WINDOW_IN_USE_KHR`](ResultCode::ERROR_NATIVE_WINDOW_IN_USE_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateMetalSurfaceEXT")]
    pub unsafe fn create_metal_surface_ext(
        self,
        create_info: *const MetalSurfaceCreateInfoEXT,
        allocator: *const AllocationCallbacks,
        surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetBufferDeviceAddressEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddressEXT.html>
    ///
    #[doc(alias = "vkGetBufferDeviceAddressEXT")]
    pub unsafe fn get_buffer_device_address_ext(
        self,
        info: *const BufferDeviceAddressInfo,
    ) -> DeviceAddress {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceToolPropertiesEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolPropertiesEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceToolPropertiesEXT")]
    pub unsafe fn get_physical_device_tool_properties_ext(
        self,
        tool_count: *mut u32,
        tool_properties: *mut PhysicalDeviceToolProperties,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceCooperativeMatrixPropertiesNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixPropertiesNV")]
    pub unsafe fn get_physical_device_cooperative_matrix_properties_nv(
        self,
        property_count: *mut u32,
        properties: *mut CooperativeMatrixPropertiesNV,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV")]
    pub unsafe fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv(
        self,
        combination_count: *mut u32,
        combinations: *mut FramebufferMixedSamplesCombinationNV,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceSurfacePresentModes2EXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfacePresentModes2EXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceSurfacePresentModes2EXT")]
    pub unsafe fn get_physical_device_surface_present_modes_2_ext(
        self,
        surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
        present_mode_count: *mut u32,
        present_modes: *mut PresentModeKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkAcquireFullScreenExclusiveModeEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireFullScreenExclusiveModeEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkAcquireFullScreenExclusiveModeEXT")]
    pub unsafe fn acquire_full_screen_exclusive_mode_ext(
        self,
        swapchain: SwapchainKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkReleaseFullScreenExclusiveModeEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseFullScreenExclusiveModeEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkReleaseFullScreenExclusiveModeEXT")]
    pub unsafe fn release_full_screen_exclusive_mode_ext(
        self,
        swapchain: SwapchainKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceGroupSurfacePresentModes2EXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModes2EXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetDeviceGroupSurfacePresentModes2EXT")]
    pub unsafe fn get_device_group_surface_present_modes_2_ext(
        self,
        surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
        modes: *mut DeviceGroupPresentModeFlagsKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Instance {
    /// `vkCreateHeadlessSurfaceEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateHeadlessSurfaceEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateHeadlessSurfaceEXT")]
    pub unsafe fn create_headless_surface_ext(
        self,
        create_info: *const HeadlessSurfaceCreateInfoEXT,
        allocator: *const AllocationCallbacks,
        surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetLineStippleEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetLineStippleEXT")]
    pub unsafe fn cmd_set_line_stipple_ext(
        self,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkResetQueryPoolEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkResetQueryPoolEXT.html>
    ///
    #[doc(alias = "vkResetQueryPoolEXT")]
    pub unsafe fn reset_query_pool_ext(
        self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetCullModeEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCullModeEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetCullModeEXT")]
    pub unsafe fn cmd_set_cull_mode_ext(self, cull_mode: CullModeFlags) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetFrontFaceEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFrontFaceEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetFrontFaceEXT")]
    pub unsafe fn cmd_set_front_face_ext(self, front_face: FrontFace) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetPrimitiveTopologyEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveTopologyEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetPrimitiveTopologyEXT")]
    pub unsafe fn cmd_set_primitive_topology_ext(self, primitive_topology: PrimitiveTopology) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetViewportWithCountEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCountEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetViewportWithCountEXT")]
    pub unsafe fn cmd_set_viewport_with_count_ext(
        self,
        viewport_count: u32,
        viewports: *const Viewport,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetScissorWithCountEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCountEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetScissorWithCountEXT")]
    pub unsafe fn cmd_set_scissor_with_count_ext(
        self,
        scissor_count: u32,
        scissors: *const Rect2D,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBindVertexBuffers2EXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2EXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdBindVertexBuffers2EXT")]
    pub unsafe fn cmd_bind_vertex_buffers_2_ext(
        self,
        first_binding: u32,
        binding_count: u32,
        buffers: *const Buffer,
        offsets: *const DeviceSize,
        sizes: *const DeviceSize,
        strides: *const DeviceSize,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDepthTestEnableEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthTestEnableEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetDepthTestEnableEXT")]
    pub unsafe fn cmd_set_depth_test_enable_ext(self, depth_test_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDepthWriteEnableEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthWriteEnableEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetDepthWriteEnableEXT")]
    pub unsafe fn cmd_set_depth_write_enable_ext(self, depth_write_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDepthCompareOpEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthCompareOpEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetDepthCompareOpEXT")]
    pub unsafe fn cmd_set_depth_compare_op_ext(self, depth_compare_op: CompareOp) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDepthBoundsTestEnableEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBoundsTestEnableEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetDepthBoundsTestEnableEXT")]
    pub unsafe fn cmd_set_depth_bounds_test_enable_ext(self, depth_bounds_test_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetStencilTestEnableEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilTestEnableEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetStencilTestEnableEXT")]
    pub unsafe fn cmd_set_stencil_test_enable_ext(self, stencil_test_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetStencilOpEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilOpEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetStencilOpEXT")]
    pub unsafe fn cmd_set_stencil_op_ext(
        self,
        face_mask: StencilFaceFlags,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCopyMemoryToImageEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToImageEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`MEMORY_MAP_FAILED`](ResultCode::ERROR_MEMORY_MAP_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCopyMemoryToImageEXT")]
    pub unsafe fn copy_memory_to_image_ext(
        self,
        copy_memory_to_image_info: *const CopyMemoryToImageInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCopyImageToMemoryEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToMemoryEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`MEMORY_MAP_FAILED`](ResultCode::ERROR_MEMORY_MAP_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCopyImageToMemoryEXT")]
    pub unsafe fn copy_image_to_memory_ext(
        self,
        copy_image_to_memory_info: *const CopyImageToMemoryInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCopyImageToImageEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToImageEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`MEMORY_MAP_FAILED`](ResultCode::ERROR_MEMORY_MAP_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCopyImageToImageEXT")]
    pub unsafe fn copy_image_to_image_ext(
        self,
        copy_image_to_image_info: *const CopyImageToImageInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkTransitionImageLayoutEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkTransitionImageLayoutEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`MEMORY_MAP_FAILED`](ResultCode::ERROR_MEMORY_MAP_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkTransitionImageLayoutEXT")]
    pub unsafe fn transition_image_layout_ext(
        self,
        transition_count: u32,
        transitions: *const HostImageLayoutTransitionInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetImageSubresourceLayout2EXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2EXT.html>
    ///
    #[doc(alias = "vkGetImageSubresourceLayout2EXT")]
    pub unsafe fn get_image_subresource_layout_2_ext(
        self,
        image: Image,
        subresource: *const ImageSubresource2,
        layout: *mut SubresourceLayout2,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkReleaseSwapchainImagesEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseSwapchainImagesEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkReleaseSwapchainImagesEXT")]
    pub unsafe fn release_swapchain_images_ext(
        self,
        release_info: *const ReleaseSwapchainImagesInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetGeneratedCommandsMemoryRequirementsNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGeneratedCommandsMemoryRequirementsNV.html>
    ///
    #[doc(alias = "vkGetGeneratedCommandsMemoryRequirementsNV")]
    pub unsafe fn get_generated_commands_memory_requirements_nv(
        self,
        info: *const GeneratedCommandsMemoryRequirementsInfoNV,
        memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdPreprocessGeneratedCommandsNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPreprocessGeneratedCommandsNV.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdPreprocessGeneratedCommandsNV")]
    pub unsafe fn cmd_preprocess_generated_commands_nv(
        self,
        generated_commands_info: *const GeneratedCommandsInfoNV,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdExecuteGeneratedCommandsNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteGeneratedCommandsNV.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `indirection`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdExecuteGeneratedCommandsNV")]
    pub unsafe fn cmd_execute_generated_commands_nv(
        self,
        is_preprocessed: Bool32,
        generated_commands_info: *const GeneratedCommandsInfoNV,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBindPipelineShaderGroupNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindPipelineShaderGroupNV.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdBindPipelineShaderGroupNV")]
    pub unsafe fn cmd_bind_pipeline_shader_group_nv(
        self,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
        group_index: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateIndirectCommandsLayoutNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectCommandsLayoutNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateIndirectCommandsLayoutNV")]
    pub unsafe fn create_indirect_commands_layout_nv(
        self,
        create_info: *const IndirectCommandsLayoutCreateInfoNV,
        allocator: *const AllocationCallbacks,
        indirect_commands_layout: *mut IndirectCommandsLayoutNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyIndirectCommandsLayoutNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectCommandsLayoutNV.html>
    ///
    #[doc(alias = "vkDestroyIndirectCommandsLayoutNV")]
    pub unsafe fn destroy_indirect_commands_layout_nv(
        self,
        indirect_commands_layout: IndirectCommandsLayoutNV,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDepthBias2EXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias2EXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetDepthBias2EXT")]
    pub unsafe fn cmd_set_depth_bias_2_ext(self, depth_bias_info: *const DepthBiasInfoEXT) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkAcquireDrmDisplayEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireDrmDisplayEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkAcquireDrmDisplayEXT")]
    pub unsafe fn acquire_drm_display_ext(self, drm_fd: i32, display: DisplayKHR) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetDrmDisplayEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDrmDisplayEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetDrmDisplayEXT")]
    pub unsafe fn get_drm_display_ext(
        self,
        drm_fd: i32,
        connector_id: u32,
        display: *mut DisplayKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCreatePrivateDataSlotEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePrivateDataSlotEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreatePrivateDataSlotEXT")]
    pub unsafe fn create_private_data_slot_ext(
        self,
        create_info: *const PrivateDataSlotCreateInfo,
        allocator: *const AllocationCallbacks,
        private_data_slot: *mut PrivateDataSlot,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyPrivateDataSlotEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlotEXT.html>
    ///
    #[doc(alias = "vkDestroyPrivateDataSlotEXT")]
    pub unsafe fn destroy_private_data_slot_ext(
        self,
        private_data_slot: PrivateDataSlot,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkSetPrivateDataEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetPrivateDataEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkSetPrivateDataEXT")]
    pub unsafe fn set_private_data_ext(
        self,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: u64,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetPrivateDataEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPrivateDataEXT.html>
    ///
    #[doc(alias = "vkGetPrivateDataEXT")]
    pub unsafe fn get_private_data_ext(
        self,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: *mut u64,
    ) {
        todo!()
    }
}

impl Queue {
    /// `vkQueueSetPerfHintQCOM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSetPerfHintQCOM.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkQueueSetPerfHintQCOM")]
    pub unsafe fn queue_set_perf_hint_qcom(
        self,
        perf_hint_info: *const PerfHintInfoQCOM,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCreateCudaModuleNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCudaModuleNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateCudaModuleNV")]
    pub unsafe fn create_cuda_module_nv(
        self,
        create_info: *const CudaModuleCreateInfoNV,
        allocator: *const AllocationCallbacks,
        module: *mut CudaModuleNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetCudaModuleCacheNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCudaModuleCacheNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetCudaModuleCacheNV")]
    pub unsafe fn get_cuda_module_cache_nv(
        self,
        module: CudaModuleNV,
        cache_size: *mut usize,
        cache_data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCreateCudaFunctionNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCudaFunctionNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateCudaFunctionNV")]
    pub unsafe fn create_cuda_function_nv(
        self,
        create_info: *const CudaFunctionCreateInfoNV,
        allocator: *const AllocationCallbacks,
        function: *mut CudaFunctionNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyCudaModuleNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCudaModuleNV.html>
    ///
    #[doc(alias = "vkDestroyCudaModuleNV")]
    pub unsafe fn destroy_cuda_module_nv(
        self,
        module: CudaModuleNV,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkDestroyCudaFunctionNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCudaFunctionNV.html>
    ///
    #[doc(alias = "vkDestroyCudaFunctionNV")]
    pub unsafe fn destroy_cuda_function_nv(
        self,
        function: CudaFunctionNV,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCudaLaunchKernelNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCudaLaunchKernelNV.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCudaLaunchKernelNV")]
    pub unsafe fn cmd_cuda_launch_kernel_nv(self, launch_info: *const CudaLaunchInfoNV) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDispatchTileQCOM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchTileQCOM.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdDispatchTileQCOM")]
    pub unsafe fn cmd_dispatch_tile_qcom(self, dispatch_tile_info: *const DispatchTileInfoQCOM) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBeginPerTileExecutionQCOM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginPerTileExecutionQCOM.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdBeginPerTileExecutionQCOM")]
    pub unsafe fn cmd_begin_per_tile_execution_qcom(
        self,
        per_tile_begin_info: *const PerTileBeginInfoQCOM,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdEndPerTileExecutionQCOM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndPerTileExecutionQCOM.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdEndPerTileExecutionQCOM")]
    pub unsafe fn cmd_end_per_tile_execution_qcom(
        self,
        per_tile_end_info: *const PerTileEndInfoQCOM,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkSetLatencySleepModeLegacyNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencySleepModeLegacyNV.html>
    ///
    #[doc(alias = "vkSetLatencySleepModeLegacyNV")]
    pub unsafe fn set_latency_sleep_mode_legacy_nv(
        self,
        low_latency_mode: Bool32,
        low_latency_boost: Bool32,
        minimum_interval_us: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkLatencySleepLegacyNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkLatencySleepLegacyNV.html>
    ///
    #[doc(alias = "vkLatencySleepLegacyNV")]
    pub unsafe fn latency_sleep_legacy_nv(self, signal_semaphore: Semaphore, value: u64) {
        todo!()
    }
}

impl Device {
    /// `vkSetLatencyMarkerLegacyNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencyMarkerLegacyNV.html>
    ///
    #[doc(alias = "vkSetLatencyMarkerLegacyNV")]
    pub unsafe fn set_latency_marker_legacy_nv(self, frame_id: u64, marker: u32) {
        todo!()
    }
}

impl Device {
    /// `vkGetLatencyTimingsLegacyNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetLatencyTimingsLegacyNV.html>
    ///
    #[doc(alias = "vkGetLatencyTimingsLegacyNV")]
    pub unsafe fn get_latency_timings_legacy_nv(self, timings: *mut c_void) {
        todo!()
    }
}

impl Queue {
    /// `vkQueueNotifyOutOfBandLegacyNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueNotifyOutOfBandLegacyNV.html>
    ///
    #[doc(alias = "vkQueueNotifyOutOfBandLegacyNV")]
    pub unsafe fn queue_notify_out_of_band_legacy_nv(self, queue_type: u32) {
        todo!()
    }
}

impl Device {
    /// `vkGetSleepStatusLegacyNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSleepStatusLegacyNV.html>
    ///
    #[doc(alias = "vkGetSleepStatusLegacyNV")]
    pub unsafe fn get_sleep_status_legacy_nv(self, low_latency_mode: *mut Bool32) {
        todo!()
    }
}

impl Device {
    /// `vkShutdownLatencyDeviceLegacyNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkShutdownLatencyDeviceLegacyNV.html>
    ///
    #[doc(alias = "vkShutdownLatencyDeviceLegacyNV")]
    pub unsafe fn shutdown_latency_device_legacy_nv(self) {
        todo!()
    }
}

impl Device {
    /// `vkExportMetalObjectsEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkExportMetalObjectsEXT.html>
    ///
    #[doc(alias = "vkExportMetalObjectsEXT")]
    pub unsafe fn export_metal_objects_ext(
        self,
        metal_objects_info: *mut ExportMetalObjectsInfoEXT,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetDescriptorSetLayoutSizeEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSizeEXT.html>
    ///
    #[doc(alias = "vkGetDescriptorSetLayoutSizeEXT")]
    pub unsafe fn get_descriptor_set_layout_size_ext(
        self,
        layout: DescriptorSetLayout,
        layout_size_in_bytes: *mut DeviceSize,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetDescriptorSetLayoutBindingOffsetEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutBindingOffsetEXT.html>
    ///
    #[doc(alias = "vkGetDescriptorSetLayoutBindingOffsetEXT")]
    pub unsafe fn get_descriptor_set_layout_binding_offset_ext(
        self,
        layout: DescriptorSetLayout,
        binding: u32,
        offset: *mut DeviceSize,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetDescriptorEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorEXT.html>
    ///
    #[doc(alias = "vkGetDescriptorEXT")]
    pub unsafe fn get_descriptor_ext(
        self,
        descriptor_info: *const DescriptorGetInfoEXT,
        data_size: usize,
        descriptor: *mut c_void,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBindDescriptorBuffersEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBuffersEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`DATA_GRAPHARM`](QueueFlags::DATA_GRAPHARM)
    ///
    #[doc(alias = "vkCmdBindDescriptorBuffersEXT")]
    pub unsafe fn cmd_bind_descriptor_buffers_ext(
        self,
        buffer_count: u32,
        binding_infos: *const DescriptorBufferBindingInfoEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDescriptorBufferOffsetsEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDescriptorBufferOffsetsEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`DATA_GRAPHARM`](QueueFlags::DATA_GRAPHARM)
    ///
    #[doc(alias = "vkCmdSetDescriptorBufferOffsetsEXT")]
    pub unsafe fn cmd_set_descriptor_buffer_offsets_ext(
        self,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        set_count: u32,
        buffer_indices: *const u32,
        offsets: *const DeviceSize,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBindDescriptorBufferEmbeddedSamplersEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBufferEmbeddedSamplersEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdBindDescriptorBufferEmbeddedSamplersEXT")]
    pub unsafe fn cmd_bind_descriptor_buffer_embedded_samplers_ext(
        self,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetBufferOpaqueCaptureDescriptorDataEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureDescriptorDataEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetBufferOpaqueCaptureDescriptorDataEXT")]
    pub unsafe fn get_buffer_opaque_capture_descriptor_data_ext(
        self,
        info: *const BufferCaptureDescriptorDataInfoEXT,
        data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetImageOpaqueCaptureDescriptorDataEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageOpaqueCaptureDescriptorDataEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetImageOpaqueCaptureDescriptorDataEXT")]
    pub unsafe fn get_image_opaque_capture_descriptor_data_ext(
        self,
        info: *const ImageCaptureDescriptorDataInfoEXT,
        data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetImageViewOpaqueCaptureDescriptorDataEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewOpaqueCaptureDescriptorDataEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetImageViewOpaqueCaptureDescriptorDataEXT")]
    pub unsafe fn get_image_view_opaque_capture_descriptor_data_ext(
        self,
        info: *const ImageViewCaptureDescriptorDataInfoEXT,
        data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetSamplerOpaqueCaptureDescriptorDataEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSamplerOpaqueCaptureDescriptorDataEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetSamplerOpaqueCaptureDescriptorDataEXT")]
    pub unsafe fn get_sampler_opaque_capture_descriptor_data_ext(
        self,
        info: *const SamplerCaptureDescriptorDataInfoEXT,
        data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT")]
    pub unsafe fn get_acceleration_structure_opaque_capture_descriptor_data_ext(
        self,
        info: *const AccelerationStructureCaptureDescriptorDataInfoEXT,
        data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetFragmentShadingRateEnumNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFragmentShadingRateEnumNV.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetFragmentShadingRateEnumNV")]
    pub unsafe fn cmd_set_fragment_shading_rate_enum_nv(
        self,
        shading_rate: FragmentShadingRateNV,
        combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2 as usize],
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceFaultInfoEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultInfoEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetDeviceFaultInfoEXT")]
    pub unsafe fn get_device_fault_info_ext(
        self,
        fault_counts: *mut DeviceFaultCountsEXT,
        fault_info: *mut DeviceFaultInfoEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkAcquireWinrtDisplayNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireWinrtDisplayNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkAcquireWinrtDisplayNV")]
    pub unsafe fn acquire_winrt_display_nv(self, display: DisplayKHR) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetWinrtDisplayNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetWinrtDisplayNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetWinrtDisplayNV")]
    pub unsafe fn get_winrt_display_nv(
        self,
        device_relative_id: u32,
        display: *mut DisplayKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Instance {
    /// `vkCreateDirectFBSurfaceEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDirectFBSurfaceEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateDirectFBSurfaceEXT")]
    pub unsafe fn create_direct_fb_surface_ext(
        self,
        create_info: *const DirectFBSurfaceCreateInfoEXT,
        allocator: *const AllocationCallbacks,
        surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceDirectFBPresentationSupportEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceDirectFBPresentationSupportEXT")]
    pub unsafe fn get_physical_device_direct_fb_presentation_support_ext(
        self,
        queue_family_index: u32,
        dfb: *mut IDirectFB,
    ) -> Bool32 {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetVertexInputEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetVertexInputEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetVertexInputEXT")]
    pub unsafe fn cmd_set_vertex_input_ext(
        self,
        vertex_binding_description_count: u32,
        vertex_binding_descriptions: *const VertexInputBindingDescription2EXT,
        vertex_attribute_description_count: u32,
        vertex_attribute_descriptions: *const VertexInputAttributeDescription2EXT,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetMemoryZirconHandleFUCHSIA`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryZirconHandleFUCHSIA.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`TOO_MANY_OBJECTS`](ResultCode::ERROR_TOO_MANY_OBJECTS)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetMemoryZirconHandleFUCHSIA")]
    pub unsafe fn get_memory_zircon_handle_fuchsia(
        self,
        get_zircon_handle_info: *const MemoryGetZirconHandleInfoFUCHSIA,
        zircon_handle: *mut zx_handle_t,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetMemoryZirconHandlePropertiesFUCHSIA`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryZirconHandlePropertiesFUCHSIA.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`INVALID_EXTERNAL_HANDLE`](ResultCode::ERROR_INVALID_EXTERNAL_HANDLE)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetMemoryZirconHandlePropertiesFUCHSIA")]
    pub unsafe fn get_memory_zircon_handle_properties_fuchsia(
        self,
        handle_type: ExternalMemoryHandleTypeFlags,
        zircon_handle: zx_handle_t,
        memory_zircon_handle_properties: *mut MemoryZirconHandlePropertiesFUCHSIA,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkImportSemaphoreZirconHandleFUCHSIA`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreZirconHandleFUCHSIA.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`INVALID_EXTERNAL_HANDLE`](ResultCode::ERROR_INVALID_EXTERNAL_HANDLE)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkImportSemaphoreZirconHandleFUCHSIA")]
    pub unsafe fn import_semaphore_zircon_handle_fuchsia(
        self,
        import_semaphore_zircon_handle_info: *const ImportSemaphoreZirconHandleInfoFUCHSIA,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetSemaphoreZirconHandleFUCHSIA`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreZirconHandleFUCHSIA.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`TOO_MANY_OBJECTS`](ResultCode::ERROR_TOO_MANY_OBJECTS)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetSemaphoreZirconHandleFUCHSIA")]
    pub unsafe fn get_semaphore_zircon_handle_fuchsia(
        self,
        get_zircon_handle_info: *const SemaphoreGetZirconHandleInfoFUCHSIA,
        zircon_handle: *mut zx_handle_t,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCreateBufferCollectionFUCHSIA`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBufferCollectionFUCHSIA.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`INVALID_EXTERNAL_HANDLE`](ResultCode::ERROR_INVALID_EXTERNAL_HANDLE)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateBufferCollectionFUCHSIA")]
    pub unsafe fn create_buffer_collection_fuchsia(
        self,
        create_info: *const BufferCollectionCreateInfoFUCHSIA,
        allocator: *const AllocationCallbacks,
        collection: *mut BufferCollectionFUCHSIA,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkSetBufferCollectionImageConstraintsFUCHSIA`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionImageConstraintsFUCHSIA.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`FORMAT_NOT_SUPPORTED`](ResultCode::ERROR_FORMAT_NOT_SUPPORTED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkSetBufferCollectionImageConstraintsFUCHSIA")]
    pub unsafe fn set_buffer_collection_image_constraints_fuchsia(
        self,
        collection: BufferCollectionFUCHSIA,
        image_constraints_info: *const ImageConstraintsInfoFUCHSIA,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkSetBufferCollectionBufferConstraintsFUCHSIA`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionBufferConstraintsFUCHSIA.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`FORMAT_NOT_SUPPORTED`](ResultCode::ERROR_FORMAT_NOT_SUPPORTED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkSetBufferCollectionBufferConstraintsFUCHSIA")]
    pub unsafe fn set_buffer_collection_buffer_constraints_fuchsia(
        self,
        collection: BufferCollectionFUCHSIA,
        buffer_constraints_info: *const BufferConstraintsInfoFUCHSIA,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyBufferCollectionFUCHSIA`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferCollectionFUCHSIA.html>
    ///
    #[doc(alias = "vkDestroyBufferCollectionFUCHSIA")]
    pub unsafe fn destroy_buffer_collection_fuchsia(
        self,
        collection: BufferCollectionFUCHSIA,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetBufferCollectionPropertiesFUCHSIA`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferCollectionPropertiesFUCHSIA.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetBufferCollectionPropertiesFUCHSIA")]
    pub unsafe fn get_buffer_collection_properties_fuchsia(
        self,
        collection: BufferCollectionFUCHSIA,
        properties: *mut BufferCollectionPropertiesFUCHSIA,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI")]
    pub unsafe fn get_device_subpass_shading_max_workgroup_size_huawei(
        self,
        renderpass: RenderPass,
        max_workgroup_size: *mut Extent2D,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSubpassShadingHUAWEI`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSubpassShadingHUAWEI.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSubpassShadingHUAWEI")]
    pub unsafe fn cmd_subpass_shading_huawei(self) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBindInvocationMaskHUAWEI`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindInvocationMaskHUAWEI.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdBindInvocationMaskHUAWEI")]
    pub unsafe fn cmd_bind_invocation_mask_huawei(
        self,
        image_view: ImageView,
        image_layout: ImageLayout,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetMemoryRemoteAddressNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryRemoteAddressNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`INVALID_EXTERNAL_HANDLE`](ResultCode::ERROR_INVALID_EXTERNAL_HANDLE)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetMemoryRemoteAddressNV")]
    pub unsafe fn get_memory_remote_address_nv(
        self,
        memory_get_remote_address_info: *const MemoryGetRemoteAddressInfoNV,
        address: *mut RemoteAddressNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetPipelinePropertiesEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelinePropertiesEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPipelinePropertiesEXT")]
    pub unsafe fn get_pipeline_properties_ext(
        self,
        pipeline_info: *const PipelineInfoKHR,
        pipeline_properties: *mut BaseOutStructure,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetPatchControlPointsEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPatchControlPointsEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetPatchControlPointsEXT")]
    pub unsafe fn cmd_set_patch_control_points_ext(self, patch_control_points: u32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetRasterizerDiscardEnableEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizerDiscardEnableEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetRasterizerDiscardEnableEXT")]
    pub unsafe fn cmd_set_rasterizer_discard_enable_ext(self, rasterizer_discard_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDepthBiasEnableEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBiasEnableEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetDepthBiasEnableEXT")]
    pub unsafe fn cmd_set_depth_bias_enable_ext(self, depth_bias_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetLogicOpEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLogicOpEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetLogicOpEXT")]
    pub unsafe fn cmd_set_logic_op_ext(self, logic_op: LogicOp) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetPrimitiveRestartEnableEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartEnableEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetPrimitiveRestartEnableEXT")]
    pub unsafe fn cmd_set_primitive_restart_enable_ext(self, primitive_restart_enable: Bool32) {
        todo!()
    }
}

impl Instance {
    /// `vkCreateScreenSurfaceQNX`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateScreenSurfaceQNX.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateScreenSurfaceQNX")]
    pub unsafe fn create_screen_surface_qnx(
        self,
        create_info: *const ScreenSurfaceCreateInfoQNX,
        allocator: *const AllocationCallbacks,
        surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceScreenPresentationSupportQNX`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceScreenPresentationSupportQNX.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceScreenPresentationSupportQNX")]
    pub unsafe fn get_physical_device_screen_presentation_support_qnx(
        self,
        queue_family_index: u32,
        window: *mut _screen_window,
    ) -> Bool32 {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetColorWriteEnableEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorWriteEnableEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetColorWriteEnableEXT")]
    pub unsafe fn cmd_set_color_write_enable_ext(
        self,
        attachment_count: u32,
        color_write_enables: *const Bool32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawMultiEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMultiEXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawMultiEXT")]
    pub unsafe fn cmd_draw_multi_ext(
        self,
        draw_count: u32,
        vertex_info: *const MultiDrawInfoEXT,
        instance_count: u32,
        first_instance: u32,
        stride: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawMultiIndexedEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMultiIndexedEXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawMultiIndexedEXT")]
    pub unsafe fn cmd_draw_multi_indexed_ext(
        self,
        draw_count: u32,
        index_info: *const MultiDrawIndexedInfoEXT,
        instance_count: u32,
        first_instance: u32,
        stride: u32,
        vertex_offset: *const i32,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateMicromapEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMicromapEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`](ResultCode::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateMicromapEXT")]
    pub unsafe fn create_micromap_ext(
        self,
        create_info: *const MicromapCreateInfoEXT,
        allocator: *const AllocationCallbacks,
        micromap: *mut MicromapEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyMicromapEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyMicromapEXT.html>
    ///
    #[doc(alias = "vkDestroyMicromapEXT")]
    pub unsafe fn destroy_micromap_ext(
        self,
        micromap: MicromapEXT,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBuildMicromapsEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildMicromapsEXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdBuildMicromapsEXT")]
    pub unsafe fn cmd_build_micromaps_ext(
        self,
        info_count: u32,
        infos: *const MicromapBuildInfoEXT,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkBuildMicromapsEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBuildMicromapsEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`OPERATION_DEFERRED_KHR`](ResultCode::OPERATION_DEFERRED_KHR)
    /// - [`OPERATION_NOT_DEFERRED_KHR`](ResultCode::OPERATION_NOT_DEFERRED_KHR)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkBuildMicromapsEXT")]
    pub unsafe fn build_micromaps_ext(
        self,
        deferred_operation: DeferredOperationKHR,
        info_count: u32,
        infos: *const MicromapBuildInfoEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCopyMicromapEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMicromapEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`OPERATION_DEFERRED_KHR`](ResultCode::OPERATION_DEFERRED_KHR)
    /// - [`OPERATION_NOT_DEFERRED_KHR`](ResultCode::OPERATION_NOT_DEFERRED_KHR)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCopyMicromapEXT")]
    pub unsafe fn copy_micromap_ext(
        self,
        deferred_operation: DeferredOperationKHR,
        info: *const CopyMicromapInfoEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCopyMicromapToMemoryEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMicromapToMemoryEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`OPERATION_DEFERRED_KHR`](ResultCode::OPERATION_DEFERRED_KHR)
    /// - [`OPERATION_NOT_DEFERRED_KHR`](ResultCode::OPERATION_NOT_DEFERRED_KHR)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCopyMicromapToMemoryEXT")]
    pub unsafe fn copy_micromap_to_memory_ext(
        self,
        deferred_operation: DeferredOperationKHR,
        info: *const CopyMicromapToMemoryInfoEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCopyMemoryToMicromapEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToMicromapEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`OPERATION_DEFERRED_KHR`](ResultCode::OPERATION_DEFERRED_KHR)
    /// - [`OPERATION_NOT_DEFERRED_KHR`](ResultCode::OPERATION_NOT_DEFERRED_KHR)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCopyMemoryToMicromapEXT")]
    pub unsafe fn copy_memory_to_micromap_ext(
        self,
        deferred_operation: DeferredOperationKHR,
        info: *const CopyMemoryToMicromapInfoEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkWriteMicromapsPropertiesEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteMicromapsPropertiesEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkWriteMicromapsPropertiesEXT")]
    pub unsafe fn write_micromaps_properties_ext(
        self,
        micromap_count: u32,
        micromaps: *const MicromapEXT,
        query_type: QueryType,
        data_size: usize,
        data: *mut c_void,
        stride: usize,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyMicromapEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMicromapEXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyMicromapEXT")]
    pub unsafe fn cmd_copy_micromap_ext(self, info: *const CopyMicromapInfoEXT) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyMicromapToMemoryEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMicromapToMemoryEXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyMicromapToMemoryEXT")]
    pub unsafe fn cmd_copy_micromap_to_memory_ext(self, info: *const CopyMicromapToMemoryInfoEXT) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyMemoryToMicromapEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToMicromapEXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyMemoryToMicromapEXT")]
    pub unsafe fn cmd_copy_memory_to_micromap_ext(self, info: *const CopyMemoryToMicromapInfoEXT) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdWriteMicromapsPropertiesEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteMicromapsPropertiesEXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdWriteMicromapsPropertiesEXT")]
    pub unsafe fn cmd_write_micromaps_properties_ext(
        self,
        micromap_count: u32,
        micromaps: *const MicromapEXT,
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceMicromapCompatibilityEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMicromapCompatibilityEXT.html>
    ///
    #[doc(alias = "vkGetDeviceMicromapCompatibilityEXT")]
    pub unsafe fn get_device_micromap_compatibility_ext(
        self,
        version_info: *const MicromapVersionInfoEXT,
        compatibility: *mut AccelerationStructureCompatibilityKHR,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetMicromapBuildSizesEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMicromapBuildSizesEXT.html>
    ///
    #[doc(alias = "vkGetMicromapBuildSizesEXT")]
    pub unsafe fn get_micromap_build_sizes_ext(
        self,
        build_type: AccelerationStructureBuildTypeKHR,
        build_info: *const MicromapBuildInfoEXT,
        size_info: *mut MicromapBuildSizesInfoEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawClusterHUAWEI`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawClusterHUAWEI.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawClusterHUAWEI")]
    pub unsafe fn cmd_draw_cluster_huawei(
        self,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawClusterIndirectHUAWEI`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawClusterIndirectHUAWEI.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawClusterIndirectHUAWEI")]
    pub unsafe fn cmd_draw_cluster_indirect_huawei(self, buffer: Buffer, offset: DeviceSize) {
        todo!()
    }
}

impl Device {
    /// `vkSetDeviceMemoryPriorityEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDeviceMemoryPriorityEXT.html>
    ///
    #[doc(alias = "vkSetDeviceMemoryPriorityEXT")]
    pub unsafe fn set_device_memory_priority_ext(self, memory: DeviceMemory, priority: f32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDispatchParametersARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDispatchParametersARM.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdSetDispatchParametersARM")]
    pub unsafe fn cmd_set_dispatch_parameters_arm(
        self,
        dispatch_parameters: *const DispatchParametersARM,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetDescriptorSetLayoutHostMappingInfoVALVE`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutHostMappingInfoVALVE.html>
    ///
    #[doc(alias = "vkGetDescriptorSetLayoutHostMappingInfoVALVE")]
    pub unsafe fn get_descriptor_set_layout_host_mapping_info_valve(
        self,
        binding_reference: *const DescriptorSetBindingReferenceVALVE,
        host_mapping: *mut DescriptorSetLayoutHostMappingInfoVALVE,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetDescriptorSetHostMappingVALVE`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetHostMappingVALVE.html>
    ///
    #[doc(alias = "vkGetDescriptorSetHostMappingVALVE")]
    pub unsafe fn get_descriptor_set_host_mapping_valve(
        self,
        descriptor_set: DescriptorSet,
        data: *mut *mut c_void,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyMemoryIndirectNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryIndirectNV.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyMemoryIndirectNV")]
    pub unsafe fn cmd_copy_memory_indirect_nv(
        self,
        copy_buffer_address: DeviceAddress,
        copy_count: u32,
        stride: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyMemoryToImageIndirectNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageIndirectNV.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyMemoryToImageIndirectNV")]
    pub unsafe fn cmd_copy_memory_to_image_indirect_nv(
        self,
        copy_buffer_address: DeviceAddress,
        copy_count: u32,
        stride: u32,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        image_subresources: *const ImageSubresourceLayers,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDecompressMemoryNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryNV.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdDecompressMemoryNV")]
    pub unsafe fn cmd_decompress_memory_nv(
        self,
        decompress_region_count: u32,
        decompress_memory_regions: *const DecompressMemoryRegionNV,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDecompressMemoryIndirectCountNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryIndirectCountNV.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdDecompressMemoryIndirectCountNV")]
    pub unsafe fn cmd_decompress_memory_indirect_count_nv(
        self,
        indirect_commands_address: DeviceAddress,
        indirect_commands_count_address: DeviceAddress,
        stride: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetPipelineIndirectMemoryRequirementsNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineIndirectMemoryRequirementsNV.html>
    ///
    #[doc(alias = "vkGetPipelineIndirectMemoryRequirementsNV")]
    pub unsafe fn get_pipeline_indirect_memory_requirements_nv(
        self,
        create_info: *const ComputePipelineCreateInfo,
        memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdUpdatePipelineIndirectBufferNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdatePipelineIndirectBufferNV.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdUpdatePipelineIndirectBufferNV")]
    pub unsafe fn cmd_update_pipeline_indirect_buffer_nv(
        self,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetPipelineIndirectDeviceAddressNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineIndirectDeviceAddressNV.html>
    ///
    #[doc(alias = "vkGetPipelineIndirectDeviceAddressNV")]
    pub unsafe fn get_pipeline_indirect_device_address_nv(
        self,
        info: *const PipelineIndirectDeviceAddressInfoNV,
    ) -> DeviceAddress {
        todo!()
    }
}

impl Device {
    /// `vkGetNativeBufferPropertiesOHOS`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetNativeBufferPropertiesOHOS.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INVALID_EXTERNAL_HANDLE_KHR`](ResultCode::ERROR_INVALID_EXTERNAL_HANDLE_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetNativeBufferPropertiesOHOS")]
    pub unsafe fn get_native_buffer_properties_ohos(
        self,
        buffer: *const OH_NativeBuffer,
        properties: *mut NativeBufferPropertiesOHOS,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetMemoryNativeBufferOHOS`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryNativeBufferOHOS.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetMemoryNativeBufferOHOS")]
    pub unsafe fn get_memory_native_buffer_ohos(
        self,
        info: *const MemoryGetNativeBufferInfoOHOS,
        buffer: *mut *mut OH_NativeBuffer,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDepthClampEnableEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClampEnableEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetDepthClampEnableEXT")]
    pub unsafe fn cmd_set_depth_clamp_enable_ext(self, depth_clamp_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetPolygonModeEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPolygonModeEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetPolygonModeEXT")]
    pub unsafe fn cmd_set_polygon_mode_ext(self, polygon_mode: PolygonMode) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetRasterizationSamplesEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizationSamplesEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetRasterizationSamplesEXT")]
    pub unsafe fn cmd_set_rasterization_samples_ext(self, rasterization_samples: SampleCountFlags) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetSampleMaskEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleMaskEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetSampleMaskEXT")]
    pub unsafe fn cmd_set_sample_mask_ext(
        self,
        samples: SampleCountFlags,
        sample_mask: *const SampleMask,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetAlphaToCoverageEnableEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAlphaToCoverageEnableEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetAlphaToCoverageEnableEXT")]
    pub unsafe fn cmd_set_alpha_to_coverage_enable_ext(self, alpha_to_coverage_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetAlphaToOneEnableEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAlphaToOneEnableEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetAlphaToOneEnableEXT")]
    pub unsafe fn cmd_set_alpha_to_one_enable_ext(self, alpha_to_one_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetLogicOpEnableEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLogicOpEnableEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetLogicOpEnableEXT")]
    pub unsafe fn cmd_set_logic_op_enable_ext(self, logic_op_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetColorBlendEnableEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendEnableEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetColorBlendEnableEXT")]
    pub unsafe fn cmd_set_color_blend_enable_ext(
        self,
        first_attachment: u32,
        attachment_count: u32,
        color_blend_enables: *const Bool32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetColorBlendEquationEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendEquationEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetColorBlendEquationEXT")]
    pub unsafe fn cmd_set_color_blend_equation_ext(
        self,
        first_attachment: u32,
        attachment_count: u32,
        color_blend_equations: *const ColorBlendEquationEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetColorWriteMaskEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorWriteMaskEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetColorWriteMaskEXT")]
    pub unsafe fn cmd_set_color_write_mask_ext(
        self,
        first_attachment: u32,
        attachment_count: u32,
        color_write_masks: *const ColorComponentFlags,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetTessellationDomainOriginEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetTessellationDomainOriginEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetTessellationDomainOriginEXT")]
    pub unsafe fn cmd_set_tessellation_domain_origin_ext(
        self,
        domain_origin: TessellationDomainOrigin,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetRasterizationStreamEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizationStreamEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetRasterizationStreamEXT")]
    pub unsafe fn cmd_set_rasterization_stream_ext(self, rasterization_stream: u32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetConservativeRasterizationModeEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetConservativeRasterizationModeEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetConservativeRasterizationModeEXT")]
    pub unsafe fn cmd_set_conservative_rasterization_mode_ext(
        self,
        conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetExtraPrimitiveOverestimationSizeEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExtraPrimitiveOverestimationSizeEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetExtraPrimitiveOverestimationSizeEXT")]
    pub unsafe fn cmd_set_extra_primitive_overestimation_size_ext(
        self,
        extra_primitive_overestimation_size: f32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDepthClipEnableEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClipEnableEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetDepthClipEnableEXT")]
    pub unsafe fn cmd_set_depth_clip_enable_ext(self, depth_clip_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetSampleLocationsEnableEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleLocationsEnableEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetSampleLocationsEnableEXT")]
    pub unsafe fn cmd_set_sample_locations_enable_ext(self, sample_locations_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetColorBlendAdvancedEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendAdvancedEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetColorBlendAdvancedEXT")]
    pub unsafe fn cmd_set_color_blend_advanced_ext(
        self,
        first_attachment: u32,
        attachment_count: u32,
        color_blend_advanced: *const ColorBlendAdvancedEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetProvokingVertexModeEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetProvokingVertexModeEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetProvokingVertexModeEXT")]
    pub unsafe fn cmd_set_provoking_vertex_mode_ext(
        self,
        provoking_vertex_mode: ProvokingVertexModeEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetLineRasterizationModeEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineRasterizationModeEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetLineRasterizationModeEXT")]
    pub unsafe fn cmd_set_line_rasterization_mode_ext(
        self,
        line_rasterization_mode: LineRasterizationModeEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetLineStippleEnableEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleEnableEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetLineStippleEnableEXT")]
    pub unsafe fn cmd_set_line_stipple_enable_ext(self, stippled_line_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDepthClipNegativeOneToOneEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClipNegativeOneToOneEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetDepthClipNegativeOneToOneEXT")]
    pub unsafe fn cmd_set_depth_clip_negative_one_to_one_ext(self, negative_one_to_one: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetViewportWScalingEnableNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWScalingEnableNV.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetViewportWScalingEnableNV")]
    pub unsafe fn cmd_set_viewport_w_scaling_enable_nv(self, viewport_w_scaling_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetViewportSwizzleNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportSwizzleNV.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetViewportSwizzleNV")]
    pub unsafe fn cmd_set_viewport_swizzle_nv(
        self,
        first_viewport: u32,
        viewport_count: u32,
        viewport_swizzles: *const ViewportSwizzleNV,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetCoverageToColorEnableNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageToColorEnableNV.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetCoverageToColorEnableNV")]
    pub unsafe fn cmd_set_coverage_to_color_enable_nv(self, coverage_to_color_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetCoverageToColorLocationNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageToColorLocationNV.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetCoverageToColorLocationNV")]
    pub unsafe fn cmd_set_coverage_to_color_location_nv(self, coverage_to_color_location: u32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetCoverageModulationModeNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationModeNV.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetCoverageModulationModeNV")]
    pub unsafe fn cmd_set_coverage_modulation_mode_nv(
        self,
        coverage_modulation_mode: CoverageModulationModeNV,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetCoverageModulationTableEnableNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationTableEnableNV.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetCoverageModulationTableEnableNV")]
    pub unsafe fn cmd_set_coverage_modulation_table_enable_nv(
        self,
        coverage_modulation_table_enable: Bool32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetCoverageModulationTableNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationTableNV.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetCoverageModulationTableNV")]
    pub unsafe fn cmd_set_coverage_modulation_table_nv(
        self,
        coverage_modulation_table_count: u32,
        coverage_modulation_table: *const f32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetShadingRateImageEnableNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetShadingRateImageEnableNV.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetShadingRateImageEnableNV")]
    pub unsafe fn cmd_set_shading_rate_image_enable_nv(self, shading_rate_image_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetRepresentativeFragmentTestEnableNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRepresentativeFragmentTestEnableNV.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetRepresentativeFragmentTestEnableNV")]
    pub unsafe fn cmd_set_representative_fragment_test_enable_nv(
        self,
        representative_fragment_test_enable: Bool32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetCoverageReductionModeNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageReductionModeNV.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetCoverageReductionModeNV")]
    pub unsafe fn cmd_set_coverage_reduction_mode_nv(
        self,
        coverage_reduction_mode: CoverageReductionModeNV,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateTensorARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateTensorARM.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateTensorARM")]
    pub unsafe fn create_tensor_arm(
        self,
        create_info: *const TensorCreateInfoARM,
        allocator: *const AllocationCallbacks,
        tensor: *mut TensorARM,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyTensorARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyTensorARM.html>
    ///
    #[doc(alias = "vkDestroyTensorARM")]
    pub unsafe fn destroy_tensor_arm(
        self,
        tensor: TensorARM,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateTensorViewARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateTensorViewARM.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateTensorViewARM")]
    pub unsafe fn create_tensor_view_arm(
        self,
        create_info: *const TensorViewCreateInfoARM,
        allocator: *const AllocationCallbacks,
        view: *mut TensorViewARM,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyTensorViewARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyTensorViewARM.html>
    ///
    #[doc(alias = "vkDestroyTensorViewARM")]
    pub unsafe fn destroy_tensor_view_arm(
        self,
        tensor_view: TensorViewARM,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetTensorMemoryRequirementsARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorMemoryRequirementsARM.html>
    ///
    #[doc(alias = "vkGetTensorMemoryRequirementsARM")]
    pub unsafe fn get_tensor_memory_requirements_arm(
        self,
        info: *const TensorMemoryRequirementsInfoARM,
        memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkBindTensorMemoryARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindTensorMemoryARM.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkBindTensorMemoryARM")]
    pub unsafe fn bind_tensor_memory_arm(
        self,
        bind_info_count: u32,
        bind_infos: *const BindTensorMemoryInfoARM,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceTensorMemoryRequirementsARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceTensorMemoryRequirementsARM.html>
    ///
    #[doc(alias = "vkGetDeviceTensorMemoryRequirementsARM")]
    pub unsafe fn get_device_tensor_memory_requirements_arm(
        self,
        info: *const DeviceTensorMemoryRequirementsARM,
        memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyTensorARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyTensorARM.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`TRANSFER`](QueueFlags::TRANSFER)
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyTensorARM")]
    pub unsafe fn cmd_copy_tensor_arm(self, copy_tensor_info: *const CopyTensorInfoARM) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceExternalTensorPropertiesARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalTensorPropertiesARM.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceExternalTensorPropertiesARM")]
    pub unsafe fn get_physical_device_external_tensor_properties_arm(
        self,
        external_tensor_info: *const PhysicalDeviceExternalTensorInfoARM,
        external_tensor_properties: *mut ExternalTensorPropertiesARM,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetTensorOpaqueCaptureDescriptorDataARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDescriptorDataARM.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetTensorOpaqueCaptureDescriptorDataARM")]
    pub unsafe fn get_tensor_opaque_capture_descriptor_data_arm(
        self,
        info: *const TensorCaptureDescriptorDataInfoARM,
        data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetTensorViewOpaqueCaptureDescriptorDataARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorViewOpaqueCaptureDescriptorDataARM.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetTensorViewOpaqueCaptureDescriptorDataARM")]
    pub unsafe fn get_tensor_view_opaque_capture_descriptor_data_arm(
        self,
        info: *const TensorViewCaptureDescriptorDataInfoARM,
        data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetShaderModuleIdentifierEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleIdentifierEXT.html>
    ///
    #[doc(alias = "vkGetShaderModuleIdentifierEXT")]
    pub unsafe fn get_shader_module_identifier_ext(
        self,
        shader_module: ShaderModule,
        identifier: *mut ShaderModuleIdentifierEXT,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetShaderModuleCreateInfoIdentifierEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleCreateInfoIdentifierEXT.html>
    ///
    #[doc(alias = "vkGetShaderModuleCreateInfoIdentifierEXT")]
    pub unsafe fn get_shader_module_create_info_identifier_ext(
        self,
        create_info: *const ShaderModuleCreateInfo,
        identifier: *mut ShaderModuleIdentifierEXT,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceOpticalFlowImageFormatsNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceOpticalFlowImageFormatsNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`EXTENSION_NOT_PRESENT`](ResultCode::ERROR_EXTENSION_NOT_PRESENT)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`FORMAT_NOT_SUPPORTED`](ResultCode::ERROR_FORMAT_NOT_SUPPORTED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceOpticalFlowImageFormatsNV")]
    pub unsafe fn get_physical_device_optical_flow_image_formats_nv(
        self,
        optical_flow_image_format_info: *const OpticalFlowImageFormatInfoNV,
        format_count: *mut u32,
        image_format_properties: *mut OpticalFlowImageFormatPropertiesNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCreateOpticalFlowSessionNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateOpticalFlowSessionNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateOpticalFlowSessionNV")]
    pub unsafe fn create_optical_flow_session_nv(
        self,
        create_info: *const OpticalFlowSessionCreateInfoNV,
        allocator: *const AllocationCallbacks,
        session: *mut OpticalFlowSessionNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyOpticalFlowSessionNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyOpticalFlowSessionNV.html>
    ///
    #[doc(alias = "vkDestroyOpticalFlowSessionNV")]
    pub unsafe fn destroy_optical_flow_session_nv(
        self,
        session: OpticalFlowSessionNV,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkBindOpticalFlowSessionImageNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindOpticalFlowSessionImageNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkBindOpticalFlowSessionImageNV")]
    pub unsafe fn bind_optical_flow_session_image_nv(
        self,
        session: OpticalFlowSessionNV,
        binding_point: OpticalFlowSessionBindingPointNV,
        view: ImageView,
        layout: ImageLayout,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdOpticalFlowExecuteNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdOpticalFlowExecuteNV.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`OPTICAL_FLOWNV`](QueueFlags::OPTICAL_FLOWNV)
    ///
    #[doc(alias = "vkCmdOpticalFlowExecuteNV")]
    pub unsafe fn cmd_optical_flow_execute_nv(
        self,
        session: OpticalFlowSessionNV,
        execute_info: *const OpticalFlowExecuteInfoNV,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkAntiLagUpdateAMD`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAntiLagUpdateAMD.html>
    ///
    #[doc(alias = "vkAntiLagUpdateAMD")]
    pub unsafe fn anti_lag_update_amd(self, data: *const AntiLagDataAMD) {
        todo!()
    }
}

impl Device {
    /// `vkCreateShadersEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShadersEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPATIBLE_SHADER_BINARY_EXT`](ResultCode::INCOMPATIBLE_SHADER_BINARY_EXT)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateShadersEXT")]
    pub unsafe fn create_shaders_ext(
        self,
        create_info_count: u32,
        create_infos: *const ShaderCreateInfoEXT,
        allocator: *const AllocationCallbacks,
        shaders: *mut ShaderEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyShaderEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderEXT.html>
    ///
    #[doc(alias = "vkDestroyShaderEXT")]
    pub unsafe fn destroy_shader_ext(
        self,
        shader: ShaderEXT,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetShaderBinaryDataEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderBinaryDataEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetShaderBinaryDataEXT")]
    pub unsafe fn get_shader_binary_data_ext(
        self,
        shader: ShaderEXT,
        data_size: *mut usize,
        data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBindShadersEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindShadersEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdBindShadersEXT")]
    pub unsafe fn cmd_bind_shaders_ext(
        self,
        stage_count: u32,
        stages: *const ShaderStageFlags,
        shaders: *const ShaderEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetDepthClampRangeEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClampRangeEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetDepthClampRangeEXT")]
    pub unsafe fn cmd_set_depth_clamp_range_ext(
        self,
        depth_clamp_mode: DepthClampModeEXT,
        depth_clamp_range: *const DepthClampRangeEXT,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetFramebufferTilePropertiesQCOM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFramebufferTilePropertiesQCOM.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetFramebufferTilePropertiesQCOM")]
    pub unsafe fn get_framebuffer_tile_properties_qcom(
        self,
        framebuffer: Framebuffer,
        properties_count: *mut u32,
        properties: *mut TilePropertiesQCOM,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetDynamicRenderingTilePropertiesQCOM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDynamicRenderingTilePropertiesQCOM.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetDynamicRenderingTilePropertiesQCOM")]
    pub unsafe fn get_dynamic_rendering_tile_properties_qcom(
        self,
        rendering_info: *const RenderingInfo,
        properties: *mut TilePropertiesQCOM,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceCooperativeVectorPropertiesNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeVectorPropertiesNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceCooperativeVectorPropertiesNV")]
    pub unsafe fn get_physical_device_cooperative_vector_properties_nv(
        self,
        property_count: *mut u32,
        properties: *mut CooperativeVectorPropertiesNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkConvertCooperativeVectorMatrixNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkConvertCooperativeVectorMatrixNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkConvertCooperativeVectorMatrixNV")]
    pub unsafe fn convert_cooperative_vector_matrix_nv(
        self,
        info: *const ConvertCooperativeVectorMatrixInfoNV,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdConvertCooperativeVectorMatrixNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdConvertCooperativeVectorMatrixNV.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdConvertCooperativeVectorMatrixNV")]
    pub unsafe fn cmd_convert_cooperative_vector_matrix_nv(
        self,
        info_count: u32,
        infos: *const ConvertCooperativeVectorMatrixInfoNV,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkSetLatencySleepModeNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencySleepModeNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkSetLatencySleepModeNV")]
    pub unsafe fn set_latency_sleep_mode_nv(
        self,
        swapchain: SwapchainKHR,
        sleep_mode_info: *const LatencySleepModeInfoNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkLatencySleepNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkLatencySleepNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkLatencySleepNV")]
    pub unsafe fn latency_sleep_nv(
        self,
        swapchain: SwapchainKHR,
        sleep_info: *const LatencySleepInfoNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkSetLatencyMarkerNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencyMarkerNV.html>
    ///
    #[doc(alias = "vkSetLatencyMarkerNV")]
    pub unsafe fn set_latency_marker_nv(
        self,
        swapchain: SwapchainKHR,
        latency_marker_info: *const SetLatencyMarkerInfoNV,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetLatencyTimingsNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetLatencyTimingsNV.html>
    ///
    #[doc(alias = "vkGetLatencyTimingsNV")]
    pub unsafe fn get_latency_timings_nv(
        self,
        swapchain: SwapchainKHR,
        latency_marker_info: *mut GetLatencyMarkerInfoNV,
    ) {
        todo!()
    }
}

impl Queue {
    /// `vkQueueNotifyOutOfBandNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueNotifyOutOfBandNV.html>
    ///
    #[doc(alias = "vkQueueNotifyOutOfBandNV")]
    pub unsafe fn queue_notify_out_of_band_nv(
        self,
        queue_type_info: *const OutOfBandQueueTypeInfoNV,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateDataGraphPipelinesARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDataGraphPipelinesARM.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`PIPELINE_COMPILE_REQUIRED_EXT`](ResultCode::PIPELINE_COMPILE_REQUIRED_EXT)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateDataGraphPipelinesARM")]
    pub unsafe fn create_data_graph_pipelines_arm(
        self,
        deferred_operation: DeferredOperationKHR,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        create_infos: *const DataGraphPipelineCreateInfoARM,
        allocator: *const AllocationCallbacks,
        pipelines: *mut Pipeline,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCreateDataGraphPipelineSessionARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDataGraphPipelineSessionARM.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateDataGraphPipelineSessionARM")]
    pub unsafe fn create_data_graph_pipeline_session_arm(
        self,
        create_info: *const DataGraphPipelineSessionCreateInfoARM,
        allocator: *const AllocationCallbacks,
        session: *mut DataGraphPipelineSessionARM,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetDataGraphPipelineSessionBindPointRequirementsARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineSessionBindPointRequirementsARM.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetDataGraphPipelineSessionBindPointRequirementsARM")]
    pub unsafe fn get_data_graph_pipeline_session_bind_point_requirements_arm(
        self,
        info: *const DataGraphPipelineSessionBindPointRequirementsInfoARM,
        bind_point_requirement_count: *mut u32,
        bind_point_requirements: *mut DataGraphPipelineSessionBindPointRequirementARM,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetDataGraphPipelineSessionMemoryRequirementsARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineSessionMemoryRequirementsARM.html>
    ///
    #[doc(alias = "vkGetDataGraphPipelineSessionMemoryRequirementsARM")]
    pub unsafe fn get_data_graph_pipeline_session_memory_requirements_arm(
        self,
        info: *const DataGraphPipelineSessionMemoryRequirementsInfoARM,
        memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkBindDataGraphPipelineSessionMemoryARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindDataGraphPipelineSessionMemoryARM.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkBindDataGraphPipelineSessionMemoryARM")]
    pub unsafe fn bind_data_graph_pipeline_session_memory_arm(
        self,
        bind_info_count: u32,
        bind_infos: *const BindDataGraphPipelineSessionMemoryInfoARM,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyDataGraphPipelineSessionARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDataGraphPipelineSessionARM.html>
    ///
    #[doc(alias = "vkDestroyDataGraphPipelineSessionARM")]
    pub unsafe fn destroy_data_graph_pipeline_session_arm(
        self,
        session: DataGraphPipelineSessionARM,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDispatchDataGraphARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchDataGraphARM.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`DATA_GRAPHARM`](QueueFlags::DATA_GRAPHARM)
    ///
    #[doc(alias = "vkCmdDispatchDataGraphARM")]
    pub unsafe fn cmd_dispatch_data_graph_arm(
        self,
        session: DataGraphPipelineSessionARM,
        info: *const DataGraphPipelineDispatchInfoARM,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetDataGraphPipelineAvailablePropertiesARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineAvailablePropertiesARM.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetDataGraphPipelineAvailablePropertiesARM")]
    pub unsafe fn get_data_graph_pipeline_available_properties_arm(
        self,
        pipeline_info: *const DataGraphPipelineInfoARM,
        properties_count: *mut u32,
        properties: *mut DataGraphPipelinePropertyARM,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetDataGraphPipelinePropertiesARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelinePropertiesARM.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetDataGraphPipelinePropertiesARM")]
    pub unsafe fn get_data_graph_pipeline_properties_arm(
        self,
        pipeline_info: *const DataGraphPipelineInfoARM,
        properties_count: u32,
        properties: *mut DataGraphPipelinePropertyQueryResultARM,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM")]
    pub unsafe fn get_physical_device_queue_family_data_graph_properties_arm(
        self,
        queue_family_index: u32,
        queue_family_data_graph_property_count: *mut u32,
        queue_family_data_graph_properties: *mut QueueFamilyDataGraphPropertiesARM,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM")]
    pub unsafe fn get_physical_device_queue_family_data_graph_processing_engine_properties_arm(
        self,
        queue_family_data_graph_processing_engine_info: *const PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM,
        queue_family_data_graph_processing_engine_properties: *mut QueueFamilyDataGraphProcessingEnginePropertiesARM,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM")]
    pub unsafe fn get_physical_device_queue_family_data_graph_engine_operation_properties_arm(
        self,
        queue_family_index: u32,
        queue_family_data_graph_properties: *const QueueFamilyDataGraphPropertiesARM,
        properties: *mut BaseOutStructure,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetAttachmentFeedbackLoopEnableEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAttachmentFeedbackLoopEnableEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetAttachmentFeedbackLoopEnableEXT")]
    pub unsafe fn cmd_set_attachment_feedback_loop_enable_ext(self, aspect_mask: ImageAspectFlags) {
        todo!()
    }
}

impl Device {
    /// `vkGetScreenBufferPropertiesQNX`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetScreenBufferPropertiesQNX.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`INVALID_EXTERNAL_HANDLE_KHR`](ResultCode::ERROR_INVALID_EXTERNAL_HANDLE_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetScreenBufferPropertiesQNX")]
    pub unsafe fn get_screen_buffer_properties_qnx(
        self,
        buffer: *const _screen_buffer,
        properties: *mut ScreenBufferPropertiesQNX,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBindTileMemoryQCOM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTileMemoryQCOM.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdBindTileMemoryQCOM")]
    pub unsafe fn cmd_bind_tile_memory_qcom(
        self,
        tile_memory_bind_info: *const TileMemoryBindInfoQCOM,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDecompressMemoryEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryEXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdDecompressMemoryEXT")]
    pub unsafe fn cmd_decompress_memory_ext(
        self,
        decompress_memory_info_ext: *const DecompressMemoryInfoEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDecompressMemoryIndirectCountEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryIndirectCountEXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdDecompressMemoryIndirectCountEXT")]
    pub unsafe fn cmd_decompress_memory_indirect_count_ext(
        self,
        decompression_method: MemoryDecompressionMethodFlagsEXT,
        indirect_commands_address: DeviceAddress,
        indirect_commands_count_address: DeviceAddress,
        max_decompression_count: u32,
        stride: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateExternalComputeQueueNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateExternalComputeQueueNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`TOO_MANY_OBJECTS`](ResultCode::ERROR_TOO_MANY_OBJECTS)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateExternalComputeQueueNV")]
    pub unsafe fn create_external_compute_queue_nv(
        self,
        create_info: *const ExternalComputeQueueCreateInfoNV,
        allocator: *const AllocationCallbacks,
        external_queue: *mut ExternalComputeQueueNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyExternalComputeQueueNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyExternalComputeQueueNV.html>
    ///
    #[doc(alias = "vkDestroyExternalComputeQueueNV")]
    pub unsafe fn destroy_external_compute_queue_nv(
        self,
        external_queue: ExternalComputeQueueNV,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

/// `vkGetExternalComputeQueueDataNV`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExternalComputeQueueDataNV.html>
///
#[doc(alias = "vkGetExternalComputeQueueDataNV")]
pub unsafe fn get_external_compute_queue_data_nv(
    external_queue: ExternalComputeQueueNV,
    params: *mut ExternalComputeQueueDataParamsNV,
    data: *mut c_void,
) {
    todo!()
}

impl Device {
    /// `vkGetClusterAccelerationStructureBuildSizesNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetClusterAccelerationStructureBuildSizesNV.html>
    ///
    #[doc(alias = "vkGetClusterAccelerationStructureBuildSizesNV")]
    pub unsafe fn get_cluster_acceleration_structure_build_sizes_nv(
        self,
        info: *const ClusterAccelerationStructureInputInfoNV,
        size_info: *mut AccelerationStructureBuildSizesInfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBuildClusterAccelerationStructureIndirectNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildClusterAccelerationStructureIndirectNV.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdBuildClusterAccelerationStructureIndirectNV")]
    pub unsafe fn cmd_build_cluster_acceleration_structure_indirect_nv(
        self,
        command_infos: *const ClusterAccelerationStructureCommandsInfoNV,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetPartitionedAccelerationStructuresBuildSizesNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPartitionedAccelerationStructuresBuildSizesNV.html>
    ///
    #[doc(alias = "vkGetPartitionedAccelerationStructuresBuildSizesNV")]
    pub unsafe fn get_partitioned_acceleration_structures_build_sizes_nv(
        self,
        info: *const PartitionedAccelerationStructureInstancesInputNV,
        size_info: *mut AccelerationStructureBuildSizesInfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBuildPartitionedAccelerationStructuresNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildPartitionedAccelerationStructuresNV.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdBuildPartitionedAccelerationStructuresNV")]
    pub unsafe fn cmd_build_partitioned_acceleration_structures_nv(
        self,
        build_info: *const BuildPartitionedAccelerationStructureInfoNV,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetGeneratedCommandsMemoryRequirementsEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGeneratedCommandsMemoryRequirementsEXT.html>
    ///
    #[doc(alias = "vkGetGeneratedCommandsMemoryRequirementsEXT")]
    pub unsafe fn get_generated_commands_memory_requirements_ext(
        self,
        info: *const GeneratedCommandsMemoryRequirementsInfoEXT,
        memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdPreprocessGeneratedCommandsEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPreprocessGeneratedCommandsEXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdPreprocessGeneratedCommandsEXT")]
    pub unsafe fn cmd_preprocess_generated_commands_ext(
        self,
        generated_commands_info: *const GeneratedCommandsInfoEXT,
        state_command_buffer: CommandBuffer,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdExecuteGeneratedCommandsEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteGeneratedCommandsEXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `indirection`
    ///
    /// ## Allowed command buffers
    /// - Primary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdExecuteGeneratedCommandsEXT")]
    pub unsafe fn cmd_execute_generated_commands_ext(
        self,
        is_preprocessed: Bool32,
        generated_commands_info: *const GeneratedCommandsInfoEXT,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateIndirectCommandsLayoutEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectCommandsLayoutEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateIndirectCommandsLayoutEXT")]
    pub unsafe fn create_indirect_commands_layout_ext(
        self,
        create_info: *const IndirectCommandsLayoutCreateInfoEXT,
        allocator: *const AllocationCallbacks,
        indirect_commands_layout: *mut IndirectCommandsLayoutEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyIndirectCommandsLayoutEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectCommandsLayoutEXT.html>
    ///
    #[doc(alias = "vkDestroyIndirectCommandsLayoutEXT")]
    pub unsafe fn destroy_indirect_commands_layout_ext(
        self,
        indirect_commands_layout: IndirectCommandsLayoutEXT,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateIndirectExecutionSetEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectExecutionSetEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateIndirectExecutionSetEXT")]
    pub unsafe fn create_indirect_execution_set_ext(
        self,
        create_info: *const IndirectExecutionSetCreateInfoEXT,
        allocator: *const AllocationCallbacks,
        indirect_execution_set: *mut IndirectExecutionSetEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyIndirectExecutionSetEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectExecutionSetEXT.html>
    ///
    #[doc(alias = "vkDestroyIndirectExecutionSetEXT")]
    pub unsafe fn destroy_indirect_execution_set_ext(
        self,
        indirect_execution_set: IndirectExecutionSetEXT,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkUpdateIndirectExecutionSetPipelineEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateIndirectExecutionSetPipelineEXT.html>
    ///
    #[doc(alias = "vkUpdateIndirectExecutionSetPipelineEXT")]
    pub unsafe fn update_indirect_execution_set_pipeline_ext(
        self,
        indirect_execution_set: IndirectExecutionSetEXT,
        execution_set_write_count: u32,
        execution_set_writes: *const WriteIndirectExecutionSetPipelineEXT,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkUpdateIndirectExecutionSetShaderEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateIndirectExecutionSetShaderEXT.html>
    ///
    #[doc(alias = "vkUpdateIndirectExecutionSetShaderEXT")]
    pub unsafe fn update_indirect_execution_set_shader_ext(
        self,
        indirect_execution_set: IndirectExecutionSetEXT,
        execution_set_write_count: u32,
        execution_set_writes: *const WriteIndirectExecutionSetShaderEXT,
    ) {
        todo!()
    }
}

impl Instance {
    /// `vkCreateSurfaceOHOS`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSurfaceOHOS.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateSurfaceOHOS")]
    pub unsafe fn create_surface_ohos(
        self,
        create_info: *const SurfaceCreateInfoOHOS,
        allocator: *const AllocationCallbacks,
        surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV")]
    pub unsafe fn get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv(
        self,
        property_count: *mut u32,
        properties: *mut CooperativeMatrixFlexibleDimensionsPropertiesNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetMemoryMetalHandleEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryMetalHandleEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`TOO_MANY_OBJECTS`](ResultCode::ERROR_TOO_MANY_OBJECTS)
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetMemoryMetalHandleEXT")]
    pub unsafe fn get_memory_metal_handle_ext(
        self,
        get_metal_handle_info: *const MemoryGetMetalHandleInfoEXT,
        handle: *mut *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetMemoryMetalHandlePropertiesEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryMetalHandlePropertiesEXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`INVALID_EXTERNAL_HANDLE`](ResultCode::ERROR_INVALID_EXTERNAL_HANDLE)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetMemoryMetalHandlePropertiesEXT")]
    pub unsafe fn get_memory_metal_handle_properties_ext(
        self,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: *const c_void,
        memory_metal_handle_properties: *mut MemoryMetalHandlePropertiesEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM")]
    pub unsafe fn enumerate_physical_device_queue_family_performance_counters_by_region_arm(
        self,
        queue_family_index: u32,
        counter_count: *mut u32,
        counters: *mut PerformanceCounterARM,
        counter_descriptions: *mut PerformanceCounterDescriptionARM,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM")]
    pub unsafe fn enumerate_physical_device_shader_instrumentation_metrics_arm(
        self,
        description_count: *mut u32,
        descriptions: *mut ShaderInstrumentationMetricDescriptionARM,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCreateShaderInstrumentationARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShaderInstrumentationARM.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateShaderInstrumentationARM")]
    pub unsafe fn create_shader_instrumentation_arm(
        self,
        create_info: *const ShaderInstrumentationCreateInfoARM,
        allocator: *const AllocationCallbacks,
        instrumentation: *mut ShaderInstrumentationARM,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyShaderInstrumentationARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderInstrumentationARM.html>
    ///
    #[doc(alias = "vkDestroyShaderInstrumentationARM")]
    pub unsafe fn destroy_shader_instrumentation_arm(
        self,
        instrumentation: ShaderInstrumentationARM,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBeginShaderInstrumentationARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginShaderInstrumentationARM.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`DATA_GRAPHARM`](QueueFlags::DATA_GRAPHARM)
    ///
    #[doc(alias = "vkCmdBeginShaderInstrumentationARM")]
    pub unsafe fn cmd_begin_shader_instrumentation_arm(
        self,
        instrumentation: ShaderInstrumentationARM,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdEndShaderInstrumentationARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndShaderInstrumentationARM.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    /// - [`DATA_GRAPHARM`](QueueFlags::DATA_GRAPHARM)
    ///
    #[doc(alias = "vkCmdEndShaderInstrumentationARM")]
    pub unsafe fn cmd_end_shader_instrumentation_arm(self) {
        todo!()
    }
}

impl Device {
    /// `vkGetShaderInstrumentationValuesARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderInstrumentationValuesARM.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetShaderInstrumentationValuesARM")]
    pub unsafe fn get_shader_instrumentation_values_arm(
        self,
        instrumentation: ShaderInstrumentationARM,
        metric_block_count: *mut u32,
        metric_values: *mut c_void,
        flags: ShaderInstrumentationValuesFlagsARM,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkClearShaderInstrumentationMetricsARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkClearShaderInstrumentationMetricsARM.html>
    ///
    #[doc(alias = "vkClearShaderInstrumentationMetricsARM")]
    pub unsafe fn clear_shader_instrumentation_metrics_arm(
        self,
        instrumentation: ShaderInstrumentationARM,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdEndRendering2EXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering2EXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdEndRendering2EXT")]
    pub unsafe fn cmd_end_rendering_2_ext(self, rendering_end_info: *const RenderingEndInfoKHR) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBeginCustomResolveEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginCustomResolveEXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdBeginCustomResolveEXT")]
    pub unsafe fn cmd_begin_custom_resolve_ext(
        self,
        begin_custom_resolve_info: *const BeginCustomResolveInfoEXT,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`EXTENSION_NOT_PRESENT`](ResultCode::ERROR_EXTENSION_NOT_PRESENT)
    /// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
    /// - [`FORMAT_NOT_SUPPORTED`](ResultCode::ERROR_FORMAT_NOT_SUPPORTED)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM")]
    pub unsafe fn get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm(
        self,
        queue_family_index: u32,
        queue_family_data_graph_properties: *const QueueFamilyDataGraphPropertiesARM,
        optical_flow_image_format_info: *const DataGraphOpticalFlowImageFormatInfoARM,
        format_count: *mut u32,
        image_format_properties: *mut DataGraphOpticalFlowImageFormatPropertiesARM,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetComputeOccupancyPriorityNV`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetComputeOccupancyPriorityNV.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdSetComputeOccupancyPriorityNV")]
    pub unsafe fn cmd_set_compute_occupancy_priority_nv(
        self,
        parameters: *const ComputeOccupancyPriorityParametersNV,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceCooperativeMatrixProperties2EXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixProperties2EXT.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixProperties2EXT")]
    pub unsafe fn get_physical_device_cooperative_matrix_properties_2_ext(
        self,
        cooperative_matrix_info: *const PhysicalDeviceCooperativeMatrixInfo2EXT,
        property_count: *mut u32,
        properties: *mut CooperativeMatrixProperties2EXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Instance {
    /// `vkCreateUbmSurfaceSEC`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateUbmSurfaceSEC.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateUbmSurfaceSEC")]
    pub unsafe fn create_ubm_surface_sec(
        self,
        create_info: *const UbmSurfaceCreateInfoSEC,
        allocator: *const AllocationCallbacks,
        surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// `vkGetPhysicalDeviceUbmPresentationSupportSEC`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceUbmPresentationSupportSEC.html>
    ///
    #[doc(alias = "vkGetPhysicalDeviceUbmPresentationSupportSEC")]
    pub unsafe fn get_physical_device_ubm_presentation_support_sec(
        self,
        queue_family_index: u32,
        device: *mut ubm_device,
    ) -> Bool32 {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetPrimitiveRestartIndexEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartIndexEXT.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdSetPrimitiveRestartIndexEXT")]
    pub unsafe fn cmd_set_primitive_restart_index_ext(self, primitive_restart_index: u32) {
        todo!()
    }
}

impl Device {
    /// `vkCreateAccelerationStructureKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructureKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`](ResultCode::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateAccelerationStructureKHR")]
    pub unsafe fn create_acceleration_structure_khr(
        self,
        create_info: *const AccelerationStructureCreateInfoKHR,
        allocator: *const AllocationCallbacks,
        acceleration_structure: *mut AccelerationStructureKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkDestroyAccelerationStructureKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureKHR.html>
    ///
    #[doc(alias = "vkDestroyAccelerationStructureKHR")]
    pub unsafe fn destroy_acceleration_structure_khr(
        self,
        acceleration_structure: AccelerationStructureKHR,
        allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBuildAccelerationStructuresKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructuresKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdBuildAccelerationStructuresKHR")]
    pub unsafe fn cmd_build_acceleration_structures_khr(
        self,
        info_count: u32,
        infos: *const AccelerationStructureBuildGeometryInfoKHR,
        build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdBuildAccelerationStructuresIndirectKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructuresIndirectKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdBuildAccelerationStructuresIndirectKHR")]
    pub unsafe fn cmd_build_acceleration_structures_indirect_khr(
        self,
        info_count: u32,
        infos: *const AccelerationStructureBuildGeometryInfoKHR,
        indirect_device_addresses: *const DeviceAddress,
        indirect_strides: *const u32,
        max_primitive_counts: *const *const u32,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkBuildAccelerationStructuresKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBuildAccelerationStructuresKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`OPERATION_DEFERRED_KHR`](ResultCode::OPERATION_DEFERRED_KHR)
    /// - [`OPERATION_NOT_DEFERRED_KHR`](ResultCode::OPERATION_NOT_DEFERRED_KHR)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkBuildAccelerationStructuresKHR")]
    pub unsafe fn build_acceleration_structures_khr(
        self,
        deferred_operation: DeferredOperationKHR,
        info_count: u32,
        infos: *const AccelerationStructureBuildGeometryInfoKHR,
        build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCopyAccelerationStructureKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyAccelerationStructureKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`OPERATION_DEFERRED_KHR`](ResultCode::OPERATION_DEFERRED_KHR)
    /// - [`OPERATION_NOT_DEFERRED_KHR`](ResultCode::OPERATION_NOT_DEFERRED_KHR)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCopyAccelerationStructureKHR")]
    pub unsafe fn copy_acceleration_structure_khr(
        self,
        deferred_operation: DeferredOperationKHR,
        info: *const CopyAccelerationStructureInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCopyAccelerationStructureToMemoryKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyAccelerationStructureToMemoryKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`OPERATION_DEFERRED_KHR`](ResultCode::OPERATION_DEFERRED_KHR)
    /// - [`OPERATION_NOT_DEFERRED_KHR`](ResultCode::OPERATION_NOT_DEFERRED_KHR)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCopyAccelerationStructureToMemoryKHR")]
    pub unsafe fn copy_acceleration_structure_to_memory_khr(
        self,
        deferred_operation: DeferredOperationKHR,
        info: *const CopyAccelerationStructureToMemoryInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkCopyMemoryToAccelerationStructureKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToAccelerationStructureKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`OPERATION_DEFERRED_KHR`](ResultCode::OPERATION_DEFERRED_KHR)
    /// - [`OPERATION_NOT_DEFERRED_KHR`](ResultCode::OPERATION_NOT_DEFERRED_KHR)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCopyMemoryToAccelerationStructureKHR")]
    pub unsafe fn copy_memory_to_acceleration_structure_khr(
        self,
        deferred_operation: DeferredOperationKHR,
        info: *const CopyMemoryToAccelerationStructureInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkWriteAccelerationStructuresPropertiesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteAccelerationStructuresPropertiesKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkWriteAccelerationStructuresPropertiesKHR")]
    pub unsafe fn write_acceleration_structures_properties_khr(
        self,
        acceleration_structure_count: u32,
        acceleration_structures: *const AccelerationStructureKHR,
        query_type: QueryType,
        data_size: usize,
        data: *mut c_void,
        stride: usize,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyAccelerationStructureKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyAccelerationStructureKHR")]
    pub unsafe fn cmd_copy_acceleration_structure_khr(
        self,
        info: *const CopyAccelerationStructureInfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyAccelerationStructureToMemoryKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureToMemoryKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyAccelerationStructureToMemoryKHR")]
    pub unsafe fn cmd_copy_acceleration_structure_to_memory_khr(
        self,
        info: *const CopyAccelerationStructureToMemoryInfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdCopyMemoryToAccelerationStructureKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToAccelerationStructureKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdCopyMemoryToAccelerationStructureKHR")]
    pub unsafe fn cmd_copy_memory_to_acceleration_structure_khr(
        self,
        info: *const CopyMemoryToAccelerationStructureInfoKHR,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetAccelerationStructureDeviceAddressKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureDeviceAddressKHR.html>
    ///
    #[doc(alias = "vkGetAccelerationStructureDeviceAddressKHR")]
    pub unsafe fn get_acceleration_structure_device_address_khr(
        self,
        info: *const AccelerationStructureDeviceAddressInfoKHR,
    ) -> DeviceAddress {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdWriteAccelerationStructuresPropertiesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteAccelerationStructuresPropertiesKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesKHR")]
    pub unsafe fn cmd_write_acceleration_structures_properties_khr(
        self,
        acceleration_structure_count: u32,
        acceleration_structures: *const AccelerationStructureKHR,
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetDeviceAccelerationStructureCompatibilityKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceAccelerationStructureCompatibilityKHR.html>
    ///
    #[doc(alias = "vkGetDeviceAccelerationStructureCompatibilityKHR")]
    pub unsafe fn get_device_acceleration_structure_compatibility_khr(
        self,
        version_info: *const AccelerationStructureVersionInfoKHR,
        compatibility: *mut AccelerationStructureCompatibilityKHR,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetAccelerationStructureBuildSizesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureBuildSizesKHR.html>
    ///
    #[doc(alias = "vkGetAccelerationStructureBuildSizesKHR")]
    pub unsafe fn get_acceleration_structure_build_sizes_khr(
        self,
        build_type: AccelerationStructureBuildTypeKHR,
        build_info: *const AccelerationStructureBuildGeometryInfoKHR,
        max_primitive_counts: *const u32,
        size_info: *mut AccelerationStructureBuildSizesInfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdTraceRaysKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdTraceRaysKHR")]
    pub unsafe fn cmd_trace_rays_khr(
        self,
        raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkCreateRayTracingPipelinesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRayTracingPipelinesKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// - [`OPERATION_DEFERRED_KHR`](ResultCode::OPERATION_DEFERRED_KHR)
    /// - [`OPERATION_NOT_DEFERRED_KHR`](ResultCode::OPERATION_NOT_DEFERRED_KHR)
    /// - [`PIPELINE_COMPILE_REQUIRED_EXT`](ResultCode::PIPELINE_COMPILE_REQUIRED_EXT)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`INVALID_OPAQUE_CAPTURE_ADDRESS`](ResultCode::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkCreateRayTracingPipelinesKHR")]
    pub unsafe fn create_ray_tracing_pipelines_khr(
        self,
        deferred_operation: DeferredOperationKHR,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        create_infos: *const RayTracingPipelineCreateInfoKHR,
        allocator: *const AllocationCallbacks,
        pipelines: *mut Pipeline,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// `vkGetRayTracingCaptureReplayShaderGroupHandlesKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html>
    ///
    /// # Result codes
    /// ## Success
    /// - [`SUCCESS`](ResultCode::SUCCESS)
    /// ## Error
    /// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
    /// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
    /// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
    /// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
    #[doc(alias = "vkGetRayTracingCaptureReplayShaderGroupHandlesKHR")]
    pub unsafe fn get_ray_tracing_capture_replay_shader_group_handles_khr(
        self,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdTraceRaysIndirectKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysIndirectKHR.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdTraceRaysIndirectKHR")]
    pub unsafe fn cmd_trace_rays_indirect_khr(
        self,
        raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        indirect_device_address: DeviceAddress,
    ) {
        todo!()
    }
}

impl Device {
    /// `vkGetRayTracingShaderGroupStackSizeKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupStackSizeKHR.html>
    ///
    #[doc(alias = "vkGetRayTracingShaderGroupStackSizeKHR")]
    pub unsafe fn get_ray_tracing_shader_group_stack_size_khr(
        self,
        pipeline: Pipeline,
        group: u32,
        group_shader: ShaderGroupShaderKHR,
    ) -> DeviceSize {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdSetRayTracingPipelineStackSizeKHR`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRayTracingPipelineStackSizeKHR.html>
    ///
    /// ## Performed tasks
    /// - `state`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`COMPUTE`](QueueFlags::COMPUTE)
    ///
    #[doc(alias = "vkCmdSetRayTracingPipelineStackSizeKHR")]
    pub unsafe fn cmd_set_ray_tracing_pipeline_stack_size_khr(self, pipeline_stack_size: u32) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawMeshTasksEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksEXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawMeshTasksEXT")]
    pub unsafe fn cmd_draw_mesh_tasks_ext(
        self,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawMeshTasksIndirectEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectEXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawMeshTasksIndirectEXT")]
    pub unsafe fn cmd_draw_mesh_tasks_indirect_ext(
        self,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// `vkCmdDrawMeshTasksIndirectCountEXT`
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCountEXT.html>
    ///
    /// ## Performed tasks
    /// - `action`
    ///
    /// ## Allowed command buffers
    /// - Primary
    /// - Secondary
    ///
    /// ## Allowed queues
    /// - [`GRAPHICS`](QueueFlags::GRAPHICS)
    ///
    #[doc(alias = "vkCmdDrawMeshTasksIndirectCountEXT")]
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count_ext(
        self,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        todo!()
    }
}
