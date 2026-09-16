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
use crate::internal::*;
use crate::structs::*;
pub(crate) type FUN_CreateInstance = unsafe extern "C" fn(
    *const InstanceCreateInfo,
    *const AllocationCallbacks,
    *mut InstanceHandle,
) -> ResultCode;
/// [`vkCreateInstance`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateInstance.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_instance(
    create_info: *const InstanceCreateInfo,
    allocator: *const AllocationCallbacks,
    instance: *mut InstanceHandle,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyInstance =
    unsafe extern "C" fn(InstanceHandle, *const AllocationCallbacks);
/// [`vkDestroyInstance`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyInstance.html)
///
/// # Optional parameters
/// - instance
/// - allocator
///
#[doc(alias = "vkDestroyInstance")]
#[inline(always)]
pub unsafe fn destroy_instance(instance: InstanceHandle, allocator: *const AllocationCallbacks) {
    todo!()
}

pub(crate) type FUN_EnumeratePhysicalDevices =
    unsafe extern "C" fn(InstanceHandle, *mut u32, *mut PhysicalDeviceHandle) -> ResultCode;
/// [`vkEnumeratePhysicalDevices`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDevices.html)
///
/// # Optional parameters
/// - physical_devices
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
#[inline(always)]
pub unsafe fn enumerate_physical_devices(
    instance: InstanceHandle,
    physical_device_count: *mut u32,
    physical_devices: *mut PhysicalDeviceHandle,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceFeatures =
    unsafe extern "C" fn(PhysicalDeviceHandle, *mut PhysicalDeviceFeatures);
/// [`vkGetPhysicalDeviceFeatures`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures.html)
///
#[doc(alias = "vkGetPhysicalDeviceFeatures")]
#[inline(always)]
pub unsafe fn get_physical_device_features(
    physical_device: PhysicalDeviceHandle,
    features: *mut PhysicalDeviceFeatures,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceFormatProperties =
    unsafe extern "C" fn(PhysicalDeviceHandle, Format, *mut FormatProperties);
/// [`vkGetPhysicalDeviceFormatProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties.html)
///
#[doc(alias = "vkGetPhysicalDeviceFormatProperties")]
#[inline(always)]
pub unsafe fn get_physical_device_format_properties(
    physical_device: PhysicalDeviceHandle,
    format: Format,
    format_properties: *mut FormatProperties,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceImageFormatProperties = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    Format,
    ImageType,
    ImageTiling,
    ImageUsageFlags,
    ImageCreateFlags,
    *mut ImageFormatProperties,
) -> ResultCode;
/// [`vkGetPhysicalDeviceImageFormatProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties.html)
///
/// # Optional parameters
/// - flags
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
#[inline(always)]
pub unsafe fn get_physical_device_image_format_properties(
    physical_device: PhysicalDeviceHandle,
    format: Format,
    type_: ImageType,
    tiling: ImageTiling,
    usage: ImageUsageFlags,
    flags: ImageCreateFlags,
    image_format_properties: *mut ImageFormatProperties,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceProperties =
    unsafe extern "C" fn(PhysicalDeviceHandle, *mut PhysicalDeviceProperties);
/// [`vkGetPhysicalDeviceProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties.html)
///
#[doc(alias = "vkGetPhysicalDeviceProperties")]
#[inline(always)]
pub unsafe fn get_physical_device_properties(
    physical_device: PhysicalDeviceHandle,
    properties: *mut PhysicalDeviceProperties,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceQueueFamilyProperties =
    unsafe extern "C" fn(PhysicalDeviceHandle, *mut u32, *mut QueueFamilyProperties);
/// [`vkGetPhysicalDeviceQueueFamilyProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties.html)
///
/// # Optional parameters
/// - queue_family_properties
///
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties")]
#[inline(always)]
pub unsafe fn get_physical_device_queue_family_properties(
    physical_device: PhysicalDeviceHandle,
    queue_family_property_count: *mut u32,
    queue_family_properties: *mut QueueFamilyProperties,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceMemoryProperties =
    unsafe extern "C" fn(PhysicalDeviceHandle, *mut PhysicalDeviceMemoryProperties);
/// [`vkGetPhysicalDeviceMemoryProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties.html)
///
#[doc(alias = "vkGetPhysicalDeviceMemoryProperties")]
#[inline(always)]
pub unsafe fn get_physical_device_memory_properties(
    physical_device: PhysicalDeviceHandle,
    memory_properties: *mut PhysicalDeviceMemoryProperties,
) {
    todo!()
}

pub(crate) type FUN_GetInstanceProcAddr =
    unsafe extern "C" fn(InstanceHandle, *const c_char) -> vkVoidFunction;
/// [`vkGetInstanceProcAddr`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetInstanceProcAddr.html)
///
/// # Optional parameters
/// - instance
///
#[doc(alias = "vkGetInstanceProcAddr")]
#[inline(always)]
pub unsafe fn get_instance_proc_addr(
    instance: InstanceHandle,
    name: *const c_char,
) -> vkVoidFunction {
    todo!()
}

pub(crate) type FUN_GetDeviceProcAddr =
    unsafe extern "C" fn(DeviceHandle, *const c_char) -> vkVoidFunction;
/// [`vkGetDeviceProcAddr`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceProcAddr.html)
///
#[doc(alias = "vkGetDeviceProcAddr")]
#[inline(always)]
pub unsafe fn get_device_proc_addr(device: DeviceHandle, name: *const c_char) -> vkVoidFunction {
    todo!()
}

pub(crate) type FUN_CreateDevice = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *const DeviceCreateInfo,
    *const AllocationCallbacks,
    *mut DeviceHandle,
) -> ResultCode;
/// [`vkCreateDevice`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDevice.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_device(
    physical_device: PhysicalDeviceHandle,
    create_info: *const DeviceCreateInfo,
    allocator: *const AllocationCallbacks,
    device: *mut DeviceHandle,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyDevice = unsafe extern "C" fn(DeviceHandle, *const AllocationCallbacks);
/// [`vkDestroyDevice`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDevice.html)
///
/// # Optional parameters
/// - device
/// - allocator
///
#[doc(alias = "vkDestroyDevice")]
#[inline(always)]
pub unsafe fn destroy_device(device: DeviceHandle, allocator: *const AllocationCallbacks) {
    todo!()
}

pub(crate) type FUN_EnumerateInstanceExtensionProperties =
    unsafe extern "C" fn(*const c_char, *mut u32, *mut ExtensionProperties) -> ResultCode;
/// [`vkEnumerateInstanceExtensionProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceExtensionProperties.html)
///
/// # Optional parameters
/// - layer_name
/// - properties
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
#[inline(always)]
pub unsafe fn enumerate_instance_extension_properties(
    layer_name: *const c_char,
    property_count: *mut u32,
    properties: *mut ExtensionProperties,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_EnumerateDeviceExtensionProperties = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *const c_char,
    *mut u32,
    *mut ExtensionProperties,
) -> ResultCode;
/// [`vkEnumerateDeviceExtensionProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceExtensionProperties.html)
///
/// # Optional parameters
/// - layer_name
/// - properties
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
#[inline(always)]
pub unsafe fn enumerate_device_extension_properties(
    physical_device: PhysicalDeviceHandle,
    layer_name: *const c_char,
    property_count: *mut u32,
    properties: *mut ExtensionProperties,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_EnumerateInstanceLayerProperties =
    unsafe extern "C" fn(*mut u32, *mut LayerProperties) -> ResultCode;
/// [`vkEnumerateInstanceLayerProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceLayerProperties.html)
///
/// # Optional parameters
/// - properties
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
#[inline(always)]
pub unsafe fn enumerate_instance_layer_properties(
    property_count: *mut u32,
    properties: *mut LayerProperties,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_EnumerateDeviceLayerProperties =
    unsafe extern "C" fn(PhysicalDeviceHandle, *mut u32, *mut LayerProperties) -> ResultCode;
/// [`vkEnumerateDeviceLayerProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceLayerProperties.html)
///
/// # Optional parameters
/// - properties
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
#[inline(always)]
pub unsafe fn enumerate_device_layer_properties(
    physical_device: PhysicalDeviceHandle,
    property_count: *mut u32,
    properties: *mut LayerProperties,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetDeviceQueue = unsafe extern "C" fn(DeviceHandle, u32, u32, *mut QueueHandle);
/// [`vkGetDeviceQueue`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue.html)
///
#[doc(alias = "vkGetDeviceQueue")]
#[inline(always)]
pub unsafe fn get_device_queue(
    device: DeviceHandle,
    queue_family_index: u32,
    queue_index: u32,
    queue: *mut QueueHandle,
) {
    todo!()
}

pub(crate) type FUN_QueueSubmit =
    unsafe extern "C" fn(QueueHandle, u32, *const SubmitInfo, Fence) -> ResultCode;
/// [`vkQueueSubmit`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit.html)
///
/// # Optional parameters
/// - submit_count
/// - fence
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
#[inline(always)]
pub unsafe fn queue_submit(
    queue: QueueHandle,
    submit_count: u32,
    submits: *const SubmitInfo,
    fence: Fence,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_QueueWaitIdle = unsafe extern "C" fn(QueueHandle) -> ResultCode;
/// [`vkQueueWaitIdle`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueWaitIdle.html)
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
#[inline(always)]
pub unsafe fn queue_wait_idle(queue: QueueHandle) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DeviceWaitIdle = unsafe extern "C" fn(DeviceHandle) -> ResultCode;
/// [`vkDeviceWaitIdle`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDeviceWaitIdle.html)
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
#[inline(always)]
pub unsafe fn device_wait_idle(device: DeviceHandle) -> ResultCode {
    todo!()
}

pub(crate) type FUN_AllocateMemory = unsafe extern "C" fn(
    DeviceHandle,
    *const MemoryAllocateInfo,
    *const AllocationCallbacks,
    *mut DeviceMemory,
) -> ResultCode;
/// [`vkAllocateMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateMemory.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn allocate_memory(
    device: DeviceHandle,
    allocate_info: *const MemoryAllocateInfo,
    allocator: *const AllocationCallbacks,
    memory: *mut DeviceMemory,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_FreeMemory =
    unsafe extern "C" fn(DeviceHandle, DeviceMemory, *const AllocationCallbacks);
/// [`vkFreeMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeMemory.html)
///
/// # Optional parameters
/// - memory
/// - allocator
///
#[doc(alias = "vkFreeMemory")]
#[inline(always)]
pub unsafe fn free_memory(
    device: DeviceHandle,
    memory: DeviceMemory,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_MapMemory = unsafe extern "C" fn(
    DeviceHandle,
    DeviceMemory,
    DeviceSize,
    DeviceSize,
    MemoryMapFlags,
    *mut *mut c_void,
) -> ResultCode;
/// [`vkMapMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory.html)
///
/// # Optional parameters
/// - flags
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
#[inline(always)]
pub unsafe fn map_memory(
    device: DeviceHandle,
    memory: DeviceMemory,
    offset: DeviceSize,
    size: DeviceSize,
    flags: MemoryMapFlags,
    data: *mut *mut c_void,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_UnmapMemory = unsafe extern "C" fn(DeviceHandle, DeviceMemory);
/// [`vkUnmapMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory.html)
///
#[doc(alias = "vkUnmapMemory")]
#[inline(always)]
pub unsafe fn unmap_memory(device: DeviceHandle, memory: DeviceMemory) {
    todo!()
}

pub(crate) type FUN_FlushMappedMemoryRanges =
    unsafe extern "C" fn(DeviceHandle, u32, *const MappedMemoryRange) -> ResultCode;
/// [`vkFlushMappedMemoryRanges`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFlushMappedMemoryRanges.html)
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
#[inline(always)]
pub unsafe fn flush_mapped_memory_ranges(
    device: DeviceHandle,
    memory_range_count: u32,
    memory_ranges: *const MappedMemoryRange,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_InvalidateMappedMemoryRanges =
    unsafe extern "C" fn(DeviceHandle, u32, *const MappedMemoryRange) -> ResultCode;
/// [`vkInvalidateMappedMemoryRanges`](https://docs.vulkan.org/refpages/latest/refpages/source/vkInvalidateMappedMemoryRanges.html)
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
#[inline(always)]
pub unsafe fn invalidate_mapped_memory_ranges(
    device: DeviceHandle,
    memory_range_count: u32,
    memory_ranges: *const MappedMemoryRange,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetDeviceMemoryCommitment =
    unsafe extern "C" fn(DeviceHandle, DeviceMemory, *mut DeviceSize);
/// [`vkGetDeviceMemoryCommitment`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryCommitment.html)
///
#[doc(alias = "vkGetDeviceMemoryCommitment")]
#[inline(always)]
pub unsafe fn get_device_memory_commitment(
    device: DeviceHandle,
    memory: DeviceMemory,
    committed_memory_in_bytes: *mut DeviceSize,
) {
    todo!()
}

pub(crate) type FUN_BindBufferMemory =
    unsafe extern "C" fn(DeviceHandle, Buffer, DeviceMemory, DeviceSize) -> ResultCode;
/// [`vkBindBufferMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory.html)
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
#[inline(always)]
pub unsafe fn bind_buffer_memory(
    device: DeviceHandle,
    buffer: Buffer,
    memory: DeviceMemory,
    memory_offset: DeviceSize,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_BindImageMemory =
    unsafe extern "C" fn(DeviceHandle, Image, DeviceMemory, DeviceSize) -> ResultCode;
/// [`vkBindImageMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory.html)
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
#[inline(always)]
pub unsafe fn bind_image_memory(
    device: DeviceHandle,
    image: Image,
    memory: DeviceMemory,
    memory_offset: DeviceSize,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetBufferMemoryRequirements =
    unsafe extern "C" fn(DeviceHandle, Buffer, *mut MemoryRequirements);
/// [`vkGetBufferMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements.html)
///
#[doc(alias = "vkGetBufferMemoryRequirements")]
#[inline(always)]
pub unsafe fn get_buffer_memory_requirements(
    device: DeviceHandle,
    buffer: Buffer,
    memory_requirements: *mut MemoryRequirements,
) {
    todo!()
}

pub(crate) type FUN_GetImageMemoryRequirements =
    unsafe extern "C" fn(DeviceHandle, Image, *mut MemoryRequirements);
/// [`vkGetImageMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements.html)
///
#[doc(alias = "vkGetImageMemoryRequirements")]
#[inline(always)]
pub unsafe fn get_image_memory_requirements(
    device: DeviceHandle,
    image: Image,
    memory_requirements: *mut MemoryRequirements,
) {
    todo!()
}

pub(crate) type FUN_GetImageSparseMemoryRequirements =
    unsafe extern "C" fn(DeviceHandle, Image, *mut u32, *mut SparseImageMemoryRequirements);
/// [`vkGetImageSparseMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements.html)
///
/// # Optional parameters
/// - sparse_memory_requirements
///
#[doc(alias = "vkGetImageSparseMemoryRequirements")]
#[inline(always)]
pub unsafe fn get_image_sparse_memory_requirements(
    device: DeviceHandle,
    image: Image,
    sparse_memory_requirement_count: *mut u32,
    sparse_memory_requirements: *mut SparseImageMemoryRequirements,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceSparseImageFormatProperties = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    Format,
    ImageType,
    SampleCountFlags,
    ImageUsageFlags,
    ImageTiling,
    *mut u32,
    *mut SparseImageFormatProperties,
);
/// [`vkGetPhysicalDeviceSparseImageFormatProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties.html)
///
/// # Optional parameters
/// - properties
///
#[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties")]
#[inline(always)]
pub unsafe fn get_physical_device_sparse_image_format_properties(
    physical_device: PhysicalDeviceHandle,
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

pub(crate) type FUN_QueueBindSparse =
    unsafe extern "C" fn(QueueHandle, u32, *const BindSparseInfo, Fence) -> ResultCode;
/// [`vkQueueBindSparse`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBindSparse.html)
///
/// # Optional parameters
/// - bind_info_count
/// - fence
///
/// # Allowed queues
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
#[inline(always)]
pub unsafe fn queue_bind_sparse(
    queue: QueueHandle,
    bind_info_count: u32,
    bind_info: *const BindSparseInfo,
    fence: Fence,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateFence = unsafe extern "C" fn(
    DeviceHandle,
    *const FenceCreateInfo,
    *const AllocationCallbacks,
    *mut Fence,
) -> ResultCode;
/// [`vkCreateFence`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateFence.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_fence(
    device: DeviceHandle,
    create_info: *const FenceCreateInfo,
    allocator: *const AllocationCallbacks,
    fence: *mut Fence,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyFence =
    unsafe extern "C" fn(DeviceHandle, Fence, *const AllocationCallbacks);
/// [`vkDestroyFence`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFence.html)
///
/// # Optional parameters
/// - fence
/// - allocator
///
#[doc(alias = "vkDestroyFence")]
#[inline(always)]
pub unsafe fn destroy_fence(
    device: DeviceHandle,
    fence: Fence,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_ResetFences =
    unsafe extern "C" fn(DeviceHandle, u32, *const Fence) -> ResultCode;
/// [`vkResetFences`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetFences.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkResetFences")]
#[inline(always)]
pub unsafe fn reset_fences(
    device: DeviceHandle,
    fence_count: u32,
    fences: *const Fence,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetFenceStatus = unsafe extern "C" fn(DeviceHandle, Fence) -> ResultCode;
/// [`vkGetFenceStatus`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceStatus.html)
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
#[inline(always)]
pub unsafe fn get_fence_status(device: DeviceHandle, fence: Fence) -> ResultCode {
    todo!()
}

pub(crate) type FUN_WaitForFences =
    unsafe extern "C" fn(DeviceHandle, u32, *const Fence, Bool32, u64) -> ResultCode;
/// [`vkWaitForFences`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForFences.html)
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
#[inline(always)]
pub unsafe fn wait_for_fences(
    device: DeviceHandle,
    fence_count: u32,
    fences: *const Fence,
    wait_all: Bool32,
    timeout: u64,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateSemaphore = unsafe extern "C" fn(
    DeviceHandle,
    *const SemaphoreCreateInfo,
    *const AllocationCallbacks,
    *mut Semaphore,
) -> ResultCode;
/// [`vkCreateSemaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSemaphore.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_semaphore(
    device: DeviceHandle,
    create_info: *const SemaphoreCreateInfo,
    allocator: *const AllocationCallbacks,
    semaphore: *mut Semaphore,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroySemaphore =
    unsafe extern "C" fn(DeviceHandle, Semaphore, *const AllocationCallbacks);
/// [`vkDestroySemaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySemaphore.html)
///
/// # Optional parameters
/// - semaphore
/// - allocator
///
#[doc(alias = "vkDestroySemaphore")]
#[inline(always)]
pub unsafe fn destroy_semaphore(
    device: DeviceHandle,
    semaphore: Semaphore,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_CreateQueryPool = unsafe extern "C" fn(
    DeviceHandle,
    *const QueryPoolCreateInfo,
    *const AllocationCallbacks,
    *mut QueryPool,
) -> ResultCode;
/// [`vkCreateQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateQueryPool.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_query_pool(
    device: DeviceHandle,
    create_info: *const QueryPoolCreateInfo,
    allocator: *const AllocationCallbacks,
    query_pool: *mut QueryPool,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyQueryPool =
    unsafe extern "C" fn(DeviceHandle, QueryPool, *const AllocationCallbacks);
/// [`vkDestroyQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyQueryPool.html)
///
/// # Optional parameters
/// - query_pool
/// - allocator
///
#[doc(alias = "vkDestroyQueryPool")]
#[inline(always)]
pub unsafe fn destroy_query_pool(
    device: DeviceHandle,
    query_pool: QueryPool,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_GetQueryPoolResults = unsafe extern "C" fn(
    DeviceHandle,
    QueryPool,
    u32,
    u32,
    usize,
    *mut c_void,
    DeviceSize,
    QueryResultFlags,
) -> ResultCode;
/// [`vkGetQueryPoolResults`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueryPoolResults.html)
///
/// # Optional parameters
/// - flags
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
#[inline(always)]
pub unsafe fn get_query_pool_results(
    device: DeviceHandle,
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

pub(crate) type FUN_CreateBuffer = unsafe extern "C" fn(
    DeviceHandle,
    *const BufferCreateInfo,
    *const AllocationCallbacks,
    *mut Buffer,
) -> ResultCode;
/// [`vkCreateBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBuffer.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_buffer(
    device: DeviceHandle,
    create_info: *const BufferCreateInfo,
    allocator: *const AllocationCallbacks,
    buffer: *mut Buffer,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyBuffer =
    unsafe extern "C" fn(DeviceHandle, Buffer, *const AllocationCallbacks);
/// [`vkDestroyBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBuffer.html)
///
/// # Optional parameters
/// - buffer
/// - allocator
///
#[doc(alias = "vkDestroyBuffer")]
#[inline(always)]
pub unsafe fn destroy_buffer(
    device: DeviceHandle,
    buffer: Buffer,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_CreateImage = unsafe extern "C" fn(
    DeviceHandle,
    *const ImageCreateInfo,
    *const AllocationCallbacks,
    *mut Image,
) -> ResultCode;
/// [`vkCreateImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImage.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_image(
    device: DeviceHandle,
    create_info: *const ImageCreateInfo,
    allocator: *const AllocationCallbacks,
    image: *mut Image,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyImage =
    unsafe extern "C" fn(DeviceHandle, Image, *const AllocationCallbacks);
/// [`vkDestroyImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImage.html)
///
/// # Optional parameters
/// - image
/// - allocator
///
#[doc(alias = "vkDestroyImage")]
#[inline(always)]
pub unsafe fn destroy_image(
    device: DeviceHandle,
    image: Image,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_GetImageSubresourceLayout =
    unsafe extern "C" fn(DeviceHandle, Image, *const ImageSubresource, *mut SubresourceLayout);
/// [`vkGetImageSubresourceLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout.html)
///
#[doc(alias = "vkGetImageSubresourceLayout")]
#[inline(always)]
pub unsafe fn get_image_subresource_layout(
    device: DeviceHandle,
    image: Image,
    subresource: *const ImageSubresource,
    layout: *mut SubresourceLayout,
) {
    todo!()
}

pub(crate) type FUN_CreateImageView = unsafe extern "C" fn(
    DeviceHandle,
    *const ImageViewCreateInfo,
    *const AllocationCallbacks,
    *mut ImageView,
) -> ResultCode;
/// [`vkCreateImageView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImageView.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_image_view(
    device: DeviceHandle,
    create_info: *const ImageViewCreateInfo,
    allocator: *const AllocationCallbacks,
    view: *mut ImageView,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyImageView =
    unsafe extern "C" fn(DeviceHandle, ImageView, *const AllocationCallbacks);
/// [`vkDestroyImageView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImageView.html)
///
/// # Optional parameters
/// - image_view
/// - allocator
///
#[doc(alias = "vkDestroyImageView")]
#[inline(always)]
pub unsafe fn destroy_image_view(
    device: DeviceHandle,
    image_view: ImageView,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_CreateCommandPool = unsafe extern "C" fn(
    DeviceHandle,
    *const CommandPoolCreateInfo,
    *const AllocationCallbacks,
    *mut CommandPool,
) -> ResultCode;
/// [`vkCreateCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCommandPool.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_command_pool(
    device: DeviceHandle,
    create_info: *const CommandPoolCreateInfo,
    allocator: *const AllocationCallbacks,
    command_pool: *mut CommandPool,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyCommandPool =
    unsafe extern "C" fn(DeviceHandle, CommandPool, *const AllocationCallbacks);
/// [`vkDestroyCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCommandPool.html)
///
/// # Optional parameters
/// - command_pool
/// - allocator
///
#[doc(alias = "vkDestroyCommandPool")]
#[inline(always)]
pub unsafe fn destroy_command_pool(
    device: DeviceHandle,
    command_pool: CommandPool,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_ResetCommandPool =
    unsafe extern "C" fn(DeviceHandle, CommandPool, CommandPoolResetFlags) -> ResultCode;
/// [`vkResetCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandPool.html)
///
/// # Optional parameters
/// - flags
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkResetCommandPool")]
#[inline(always)]
pub unsafe fn reset_command_pool(
    device: DeviceHandle,
    command_pool: CommandPool,
    flags: CommandPoolResetFlags,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_AllocateCommandBuffers = unsafe extern "C" fn(
    DeviceHandle,
    *const CommandBufferAllocateInfo,
    *mut CommandBufferHandle,
) -> ResultCode;
/// [`vkAllocateCommandBuffers`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateCommandBuffers.html)
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
#[inline(always)]
pub unsafe fn allocate_command_buffers(
    device: DeviceHandle,
    allocate_info: *const CommandBufferAllocateInfo,
    command_buffers: *mut CommandBufferHandle,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_FreeCommandBuffers =
    unsafe extern "C" fn(DeviceHandle, CommandPool, u32, *const CommandBufferHandle);
/// [`vkFreeCommandBuffers`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeCommandBuffers.html)
///
#[doc(alias = "vkFreeCommandBuffers")]
#[inline(always)]
pub unsafe fn free_command_buffers(
    device: DeviceHandle,
    command_pool: CommandPool,
    command_buffer_count: u32,
    command_buffers: *const CommandBufferHandle,
) {
    todo!()
}

pub(crate) type FUN_BeginCommandBuffer =
    unsafe extern "C" fn(CommandBufferHandle, *const CommandBufferBeginInfo) -> ResultCode;
/// [`vkBeginCommandBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBeginCommandBuffer.html)
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
#[inline(always)]
pub unsafe fn begin_command_buffer(
    command_buffer: CommandBufferHandle,
    begin_info: *const CommandBufferBeginInfo,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_EndCommandBuffer = unsafe extern "C" fn(CommandBufferHandle) -> ResultCode;
/// [`vkEndCommandBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEndCommandBuffer.html)
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
#[inline(always)]
pub unsafe fn end_command_buffer(command_buffer: CommandBufferHandle) -> ResultCode {
    todo!()
}

pub(crate) type FUN_ResetCommandBuffer =
    unsafe extern "C" fn(CommandBufferHandle, CommandBufferResetFlags) -> ResultCode;
/// [`vkResetCommandBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandBuffer.html)
///
/// # Optional parameters
/// - flags
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkResetCommandBuffer")]
#[inline(always)]
pub unsafe fn reset_command_buffer(
    command_buffer: CommandBufferHandle,
    flags: CommandBufferResetFlags,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdCopyBuffer =
    unsafe extern "C" fn(CommandBufferHandle, Buffer, Buffer, u32, *const BufferCopy);
/// [`vkCmdCopyBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyBuffer")]
#[inline(always)]
pub unsafe fn cmd_copy_buffer(
    command_buffer: CommandBufferHandle,
    src_buffer: Buffer,
    dst_buffer: Buffer,
    region_count: u32,
    regions: *const BufferCopy,
) {
    todo!()
}

pub(crate) type FUN_CmdCopyImage = unsafe extern "C" fn(
    CommandBufferHandle,
    Image,
    ImageLayout,
    Image,
    ImageLayout,
    u32,
    *const ImageCopy,
);
/// [`vkCmdCopyImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyImage")]
#[inline(always)]
pub unsafe fn cmd_copy_image(
    command_buffer: CommandBufferHandle,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    regions: *const ImageCopy,
) {
    todo!()
}

pub(crate) type FUN_CmdCopyBufferToImage = unsafe extern "C" fn(
    CommandBufferHandle,
    Buffer,
    Image,
    ImageLayout,
    u32,
    *const BufferImageCopy,
);
/// [`vkCmdCopyBufferToImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyBufferToImage")]
#[inline(always)]
pub unsafe fn cmd_copy_buffer_to_image(
    command_buffer: CommandBufferHandle,
    src_buffer: Buffer,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    regions: *const BufferImageCopy,
) {
    todo!()
}

pub(crate) type FUN_CmdCopyImageToBuffer = unsafe extern "C" fn(
    CommandBufferHandle,
    Image,
    ImageLayout,
    Buffer,
    u32,
    *const BufferImageCopy,
);
/// [`vkCmdCopyImageToBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyImageToBuffer")]
#[inline(always)]
pub unsafe fn cmd_copy_image_to_buffer(
    command_buffer: CommandBufferHandle,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_buffer: Buffer,
    region_count: u32,
    regions: *const BufferImageCopy,
) {
    todo!()
}

pub(crate) type FUN_CmdUpdateBuffer =
    unsafe extern "C" fn(CommandBufferHandle, Buffer, DeviceSize, DeviceSize, *const c_void);
/// [`vkCmdUpdateBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdateBuffer.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdUpdateBuffer")]
#[inline(always)]
pub unsafe fn cmd_update_buffer(
    command_buffer: CommandBufferHandle,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    data_size: DeviceSize,
    data: *const c_void,
) {
    todo!()
}

pub(crate) type FUN_CmdFillBuffer =
    unsafe extern "C" fn(CommandBufferHandle, Buffer, DeviceSize, DeviceSize, u32);
/// [`vkCmdFillBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdFillBuffer.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdFillBuffer")]
#[inline(always)]
pub unsafe fn cmd_fill_buffer(
    command_buffer: CommandBufferHandle,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    size: DeviceSize,
    data: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdPipelineBarrier = unsafe extern "C" fn(
    CommandBufferHandle,
    PipelineStageFlags,
    PipelineStageFlags,
    DependencyFlags,
    u32,
    *const MemoryBarrier,
    u32,
    *const BufferMemoryBarrier,
    u32,
    *const ImageMemoryBarrier,
);
/// [`vkCmdPipelineBarrier`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier.html)
///
/// # Optional parameters
/// - src_stage_mask
/// - dst_stage_mask
/// - dependency_flags
/// - memory_barrier_count
/// - buffer_memory_barrier_count
/// - image_memory_barrier_count
///
/// # Performed tasks
/// - `synchronization`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
///
#[doc(alias = "vkCmdPipelineBarrier")]
#[inline(always)]
pub unsafe fn cmd_pipeline_barrier(
    command_buffer: CommandBufferHandle,
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

pub(crate) type FUN_CmdBeginQuery =
    unsafe extern "C" fn(CommandBufferHandle, QueryPool, u32, QueryControlFlags);
/// [`vkCmdBeginQuery`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginQuery.html)
///
/// # Optional parameters
/// - flags
///
/// # Performed tasks
/// - `action`
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
///
#[doc(alias = "vkCmdBeginQuery")]
#[inline(always)]
pub unsafe fn cmd_begin_query(
    command_buffer: CommandBufferHandle,
    query_pool: QueryPool,
    query: u32,
    flags: QueryControlFlags,
) {
    todo!()
}

pub(crate) type FUN_CmdEndQuery = unsafe extern "C" fn(CommandBufferHandle, QueryPool, u32);
/// [`vkCmdEndQuery`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndQuery.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
///
#[doc(alias = "vkCmdEndQuery")]
#[inline(always)]
pub unsafe fn cmd_end_query(
    command_buffer: CommandBufferHandle,
    query_pool: QueryPool,
    query: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdResetQueryPool =
    unsafe extern "C" fn(CommandBufferHandle, QueryPool, u32, u32);
/// [`vkCmdResetQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetQueryPool.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
/// - [`OPTICAL_FLOWNV`](QueueFlags::OPTICAL_FLOWNV)
///
#[doc(alias = "vkCmdResetQueryPool")]
#[inline(always)]
pub unsafe fn cmd_reset_query_pool(
    command_buffer: CommandBufferHandle,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdWriteTimestamp =
    unsafe extern "C" fn(CommandBufferHandle, PipelineStageFlags, QueryPool, u32);
/// [`vkCmdWriteTimestamp`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
/// - [`OPTICAL_FLOWNV`](QueueFlags::OPTICAL_FLOWNV)
///
#[doc(alias = "vkCmdWriteTimestamp")]
#[inline(always)]
pub unsafe fn cmd_write_timestamp(
    command_buffer: CommandBufferHandle,
    pipeline_stage: PipelineStageFlags,
    query_pool: QueryPool,
    query: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdCopyQueryPoolResults = unsafe extern "C" fn(
    CommandBufferHandle,
    QueryPool,
    u32,
    u32,
    Buffer,
    DeviceSize,
    DeviceSize,
    QueryResultFlags,
);
/// [`vkCmdCopyQueryPoolResults`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyQueryPoolResults.html)
///
/// # Optional parameters
/// - flags
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyQueryPoolResults")]
#[inline(always)]
pub unsafe fn cmd_copy_query_pool_results(
    command_buffer: CommandBufferHandle,
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

pub(crate) type FUN_CmdExecuteCommands =
    unsafe extern "C" fn(CommandBufferHandle, u32, *const CommandBufferHandle);
/// [`vkCmdExecuteCommands`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteCommands.html)
///
/// # Performed tasks
/// - `indirection`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdExecuteCommands")]
#[inline(always)]
pub unsafe fn cmd_execute_commands(
    command_buffer: CommandBufferHandle,
    command_buffer_count: u32,
    command_buffers: *const CommandBufferHandle,
) {
    todo!()
}

pub(crate) type FUN_CreateEvent = unsafe extern "C" fn(
    DeviceHandle,
    *const EventCreateInfo,
    *const AllocationCallbacks,
    *mut Event,
) -> ResultCode;
/// [`vkCreateEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateEvent.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_event(
    device: DeviceHandle,
    create_info: *const EventCreateInfo,
    allocator: *const AllocationCallbacks,
    event: *mut Event,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyEvent =
    unsafe extern "C" fn(DeviceHandle, Event, *const AllocationCallbacks);
/// [`vkDestroyEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyEvent.html)
///
/// # Optional parameters
/// - event
/// - allocator
///
#[doc(alias = "vkDestroyEvent")]
#[inline(always)]
pub unsafe fn destroy_event(
    device: DeviceHandle,
    event: Event,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_GetEventStatus = unsafe extern "C" fn(DeviceHandle, Event) -> ResultCode;
/// [`vkGetEventStatus`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetEventStatus.html)
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
#[inline(always)]
pub unsafe fn get_event_status(device: DeviceHandle, event: Event) -> ResultCode {
    todo!()
}

pub(crate) type FUN_SetEvent = unsafe extern "C" fn(DeviceHandle, Event) -> ResultCode;
/// [`vkSetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetEvent.html)
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
#[inline(always)]
pub unsafe fn set_event(device: DeviceHandle, event: Event) -> ResultCode {
    todo!()
}

pub(crate) type FUN_ResetEvent = unsafe extern "C" fn(DeviceHandle, Event) -> ResultCode;
/// [`vkResetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetEvent.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_DEVICE_MEMORY`](ResultCode::ERROR_OUT_OF_DEVICE_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkResetEvent")]
#[inline(always)]
pub unsafe fn reset_event(device: DeviceHandle, event: Event) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateBufferView = unsafe extern "C" fn(
    DeviceHandle,
    *const BufferViewCreateInfo,
    *const AllocationCallbacks,
    *mut BufferView,
) -> ResultCode;
/// [`vkCreateBufferView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBufferView.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_buffer_view(
    device: DeviceHandle,
    create_info: *const BufferViewCreateInfo,
    allocator: *const AllocationCallbacks,
    view: *mut BufferView,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyBufferView =
    unsafe extern "C" fn(DeviceHandle, BufferView, *const AllocationCallbacks);
/// [`vkDestroyBufferView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferView.html)
///
/// # Optional parameters
/// - buffer_view
/// - allocator
///
#[doc(alias = "vkDestroyBufferView")]
#[inline(always)]
pub unsafe fn destroy_buffer_view(
    device: DeviceHandle,
    buffer_view: BufferView,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_CreateShaderModule = unsafe extern "C" fn(
    DeviceHandle,
    *const ShaderModuleCreateInfo,
    *const AllocationCallbacks,
    *mut ShaderModule,
) -> ResultCode;
/// [`vkCreateShaderModule`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShaderModule.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_shader_module(
    device: DeviceHandle,
    create_info: *const ShaderModuleCreateInfo,
    allocator: *const AllocationCallbacks,
    shader_module: *mut ShaderModule,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyShaderModule =
    unsafe extern "C" fn(DeviceHandle, ShaderModule, *const AllocationCallbacks);
/// [`vkDestroyShaderModule`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderModule.html)
///
/// # Optional parameters
/// - shader_module
/// - allocator
///
#[doc(alias = "vkDestroyShaderModule")]
#[inline(always)]
pub unsafe fn destroy_shader_module(
    device: DeviceHandle,
    shader_module: ShaderModule,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_CreatePipelineCache = unsafe extern "C" fn(
    DeviceHandle,
    *const PipelineCacheCreateInfo,
    *const AllocationCallbacks,
    *mut PipelineCache,
) -> ResultCode;
/// [`vkCreatePipelineCache`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineCache.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_pipeline_cache(
    device: DeviceHandle,
    create_info: *const PipelineCacheCreateInfo,
    allocator: *const AllocationCallbacks,
    pipeline_cache: *mut PipelineCache,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyPipelineCache =
    unsafe extern "C" fn(DeviceHandle, PipelineCache, *const AllocationCallbacks);
/// [`vkDestroyPipelineCache`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineCache.html)
///
/// # Optional parameters
/// - pipeline_cache
/// - allocator
///
#[doc(alias = "vkDestroyPipelineCache")]
#[inline(always)]
pub unsafe fn destroy_pipeline_cache(
    device: DeviceHandle,
    pipeline_cache: PipelineCache,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_GetPipelineCacheData =
    unsafe extern "C" fn(DeviceHandle, PipelineCache, *mut usize, *mut c_void) -> ResultCode;
/// [`vkGetPipelineCacheData`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineCacheData.html)
///
/// # Optional parameters
/// - data
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
#[inline(always)]
pub unsafe fn get_pipeline_cache_data(
    device: DeviceHandle,
    pipeline_cache: PipelineCache,
    data_size: *mut usize,
    data: *mut c_void,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_MergePipelineCaches =
    unsafe extern "C" fn(DeviceHandle, PipelineCache, u32, *const PipelineCache) -> ResultCode;
/// [`vkMergePipelineCaches`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMergePipelineCaches.html)
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
#[inline(always)]
pub unsafe fn merge_pipeline_caches(
    device: DeviceHandle,
    dst_cache: PipelineCache,
    src_cache_count: u32,
    src_caches: *const PipelineCache,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateComputePipelines = unsafe extern "C" fn(
    DeviceHandle,
    PipelineCache,
    u32,
    *const ComputePipelineCreateInfo,
    *const AllocationCallbacks,
    *mut Pipeline,
) -> ResultCode;
/// [`vkCreateComputePipelines`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateComputePipelines.html)
///
/// # Optional parameters
/// - pipeline_cache
/// - allocator
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
#[inline(always)]
pub unsafe fn create_compute_pipelines(
    device: DeviceHandle,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    create_infos: *const ComputePipelineCreateInfo,
    allocator: *const AllocationCallbacks,
    pipelines: *mut Pipeline,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyPipeline =
    unsafe extern "C" fn(DeviceHandle, Pipeline, *const AllocationCallbacks);
/// [`vkDestroyPipeline`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipeline.html)
///
/// # Optional parameters
/// - pipeline
/// - allocator
///
#[doc(alias = "vkDestroyPipeline")]
#[inline(always)]
pub unsafe fn destroy_pipeline(
    device: DeviceHandle,
    pipeline: Pipeline,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_CreatePipelineLayout = unsafe extern "C" fn(
    DeviceHandle,
    *const PipelineLayoutCreateInfo,
    *const AllocationCallbacks,
    *mut PipelineLayout,
) -> ResultCode;
/// [`vkCreatePipelineLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineLayout.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_pipeline_layout(
    device: DeviceHandle,
    create_info: *const PipelineLayoutCreateInfo,
    allocator: *const AllocationCallbacks,
    pipeline_layout: *mut PipelineLayout,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyPipelineLayout =
    unsafe extern "C" fn(DeviceHandle, PipelineLayout, *const AllocationCallbacks);
/// [`vkDestroyPipelineLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineLayout.html)
///
/// # Optional parameters
/// - pipeline_layout
/// - allocator
///
#[doc(alias = "vkDestroyPipelineLayout")]
#[inline(always)]
pub unsafe fn destroy_pipeline_layout(
    device: DeviceHandle,
    pipeline_layout: PipelineLayout,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_CreateSampler = unsafe extern "C" fn(
    DeviceHandle,
    *const SamplerCreateInfo,
    *const AllocationCallbacks,
    *mut Sampler,
) -> ResultCode;
/// [`vkCreateSampler`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSampler.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_sampler(
    device: DeviceHandle,
    create_info: *const SamplerCreateInfo,
    allocator: *const AllocationCallbacks,
    sampler: *mut Sampler,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroySampler =
    unsafe extern "C" fn(DeviceHandle, Sampler, *const AllocationCallbacks);
/// [`vkDestroySampler`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySampler.html)
///
/// # Optional parameters
/// - sampler
/// - allocator
///
#[doc(alias = "vkDestroySampler")]
#[inline(always)]
pub unsafe fn destroy_sampler(
    device: DeviceHandle,
    sampler: Sampler,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_CreateDescriptorSetLayout = unsafe extern "C" fn(
    DeviceHandle,
    *const DescriptorSetLayoutCreateInfo,
    *const AllocationCallbacks,
    *mut DescriptorSetLayout,
) -> ResultCode;
/// [`vkCreateDescriptorSetLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorSetLayout.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_descriptor_set_layout(
    device: DeviceHandle,
    create_info: *const DescriptorSetLayoutCreateInfo,
    allocator: *const AllocationCallbacks,
    set_layout: *mut DescriptorSetLayout,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyDescriptorSetLayout =
    unsafe extern "C" fn(DeviceHandle, DescriptorSetLayout, *const AllocationCallbacks);
/// [`vkDestroyDescriptorSetLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorSetLayout.html)
///
/// # Optional parameters
/// - descriptor_set_layout
/// - allocator
///
#[doc(alias = "vkDestroyDescriptorSetLayout")]
#[inline(always)]
pub unsafe fn destroy_descriptor_set_layout(
    device: DeviceHandle,
    descriptor_set_layout: DescriptorSetLayout,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_CreateDescriptorPool = unsafe extern "C" fn(
    DeviceHandle,
    *const DescriptorPoolCreateInfo,
    *const AllocationCallbacks,
    *mut DescriptorPool,
) -> ResultCode;
/// [`vkCreateDescriptorPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorPool.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_descriptor_pool(
    device: DeviceHandle,
    create_info: *const DescriptorPoolCreateInfo,
    allocator: *const AllocationCallbacks,
    descriptor_pool: *mut DescriptorPool,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyDescriptorPool =
    unsafe extern "C" fn(DeviceHandle, DescriptorPool, *const AllocationCallbacks);
/// [`vkDestroyDescriptorPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorPool.html)
///
/// # Optional parameters
/// - descriptor_pool
/// - allocator
///
#[doc(alias = "vkDestroyDescriptorPool")]
#[inline(always)]
pub unsafe fn destroy_descriptor_pool(
    device: DeviceHandle,
    descriptor_pool: DescriptorPool,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_ResetDescriptorPool =
    unsafe extern "C" fn(DeviceHandle, DescriptorPool, DescriptorPoolResetFlags) -> ResultCode;
/// [`vkResetDescriptorPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetDescriptorPool.html)
///
/// # Optional parameters
/// - flags
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkResetDescriptorPool")]
#[inline(always)]
pub unsafe fn reset_descriptor_pool(
    device: DeviceHandle,
    descriptor_pool: DescriptorPool,
    flags: DescriptorPoolResetFlags,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_AllocateDescriptorSets = unsafe extern "C" fn(
    DeviceHandle,
    *const DescriptorSetAllocateInfo,
    *mut DescriptorSet,
) -> ResultCode;
/// [`vkAllocateDescriptorSets`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateDescriptorSets.html)
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
#[inline(always)]
pub unsafe fn allocate_descriptor_sets(
    device: DeviceHandle,
    allocate_info: *const DescriptorSetAllocateInfo,
    descriptor_sets: *mut DescriptorSet,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_FreeDescriptorSets =
    unsafe extern "C" fn(DeviceHandle, DescriptorPool, u32, *const DescriptorSet) -> ResultCode;
/// [`vkFreeDescriptorSets`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeDescriptorSets.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkFreeDescriptorSets")]
#[inline(always)]
pub unsafe fn free_descriptor_sets(
    device: DeviceHandle,
    descriptor_pool: DescriptorPool,
    descriptor_set_count: u32,
    descriptor_sets: *const DescriptorSet,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_UpdateDescriptorSets = unsafe extern "C" fn(
    DeviceHandle,
    u32,
    *const WriteDescriptorSet,
    u32,
    *const CopyDescriptorSet,
);
/// [`vkUpdateDescriptorSets`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSets.html)
///
/// # Optional parameters
/// - descriptor_write_count
/// - descriptor_copy_count
///
#[doc(alias = "vkUpdateDescriptorSets")]
#[inline(always)]
pub unsafe fn update_descriptor_sets(
    device: DeviceHandle,
    descriptor_write_count: u32,
    descriptor_writes: *const WriteDescriptorSet,
    descriptor_copy_count: u32,
    descriptor_copies: *const CopyDescriptorSet,
) {
    todo!()
}

pub(crate) type FUN_CmdBindPipeline =
    unsafe extern "C" fn(CommandBufferHandle, PipelineBindPoint, Pipeline);
/// [`vkCmdBindPipeline`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindPipeline.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`DATA_GRAPHARM`](QueueFlags::DATA_GRAPHARM)
///
#[doc(alias = "vkCmdBindPipeline")]
#[inline(always)]
pub unsafe fn cmd_bind_pipeline(
    command_buffer: CommandBufferHandle,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
) {
    todo!()
}

pub(crate) type FUN_CmdBindDescriptorSets = unsafe extern "C" fn(
    CommandBufferHandle,
    PipelineBindPoint,
    PipelineLayout,
    u32,
    u32,
    *const DescriptorSet,
    u32,
    *const u32,
);
/// [`vkCmdBindDescriptorSets`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets.html)
///
/// # Optional parameters
/// - dynamic_offset_count
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`DATA_GRAPHARM`](QueueFlags::DATA_GRAPHARM)
///
#[doc(alias = "vkCmdBindDescriptorSets")]
#[inline(always)]
pub unsafe fn cmd_bind_descriptor_sets(
    command_buffer: CommandBufferHandle,
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

pub(crate) type FUN_CmdClearColorImage = unsafe extern "C" fn(
    CommandBufferHandle,
    Image,
    ImageLayout,
    *const ClearColorValue,
    u32,
    *const ImageSubresourceRange,
);
/// [`vkCmdClearColorImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearColorImage.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdClearColorImage")]
#[inline(always)]
pub unsafe fn cmd_clear_color_image(
    command_buffer: CommandBufferHandle,
    image: Image,
    image_layout: ImageLayout,
    color: *const ClearColorValue,
    range_count: u32,
    ranges: *const ImageSubresourceRange,
) {
    todo!()
}

pub(crate) type FUN_CmdDispatch = unsafe extern "C" fn(CommandBufferHandle, u32, u32, u32);
/// [`vkCmdDispatch`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatch.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdDispatch")]
#[inline(always)]
pub unsafe fn cmd_dispatch(
    command_buffer: CommandBufferHandle,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdDispatchIndirect =
    unsafe extern "C" fn(CommandBufferHandle, Buffer, DeviceSize);
/// [`vkCmdDispatchIndirect`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchIndirect.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdDispatchIndirect")]
#[inline(always)]
pub unsafe fn cmd_dispatch_indirect(
    command_buffer: CommandBufferHandle,
    buffer: Buffer,
    offset: DeviceSize,
) {
    todo!()
}

pub(crate) type FUN_CmdSetEvent =
    unsafe extern "C" fn(CommandBufferHandle, Event, PipelineStageFlags);
/// [`vkCmdSetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent.html)
///
/// # Optional parameters
/// - stage_mask
///
/// # Performed tasks
/// - `synchronization`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
///
#[doc(alias = "vkCmdSetEvent")]
#[inline(always)]
pub unsafe fn cmd_set_event(
    command_buffer: CommandBufferHandle,
    event: Event,
    stage_mask: PipelineStageFlags,
) {
    todo!()
}

pub(crate) type FUN_CmdResetEvent =
    unsafe extern "C" fn(CommandBufferHandle, Event, PipelineStageFlags);
/// [`vkCmdResetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent.html)
///
/// # Optional parameters
/// - stage_mask
///
/// # Performed tasks
/// - `synchronization`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
///
#[doc(alias = "vkCmdResetEvent")]
#[inline(always)]
pub unsafe fn cmd_reset_event(
    command_buffer: CommandBufferHandle,
    event: Event,
    stage_mask: PipelineStageFlags,
) {
    todo!()
}

pub(crate) type FUN_CmdWaitEvents = unsafe extern "C" fn(
    CommandBufferHandle,
    u32,
    *const Event,
    PipelineStageFlags,
    PipelineStageFlags,
    u32,
    *const MemoryBarrier,
    u32,
    *const BufferMemoryBarrier,
    u32,
    *const ImageMemoryBarrier,
);
/// [`vkCmdWaitEvents`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents.html)
///
/// # Optional parameters
/// - src_stage_mask
/// - dst_stage_mask
/// - memory_barrier_count
/// - buffer_memory_barrier_count
/// - image_memory_barrier_count
///
/// # Performed tasks
/// - `synchronization`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
///
#[doc(alias = "vkCmdWaitEvents")]
#[inline(always)]
pub unsafe fn cmd_wait_events(
    command_buffer: CommandBufferHandle,
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

pub(crate) type FUN_CmdPushConstants = unsafe extern "C" fn(
    CommandBufferHandle,
    PipelineLayout,
    ShaderStageFlags,
    u32,
    u32,
    *const c_void,
);
/// [`vkCmdPushConstants`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdPushConstants")]
#[inline(always)]
pub unsafe fn cmd_push_constants(
    command_buffer: CommandBufferHandle,
    layout: PipelineLayout,
    stage_flags: ShaderStageFlags,
    offset: u32,
    size: u32,
    values: *const c_void,
) {
    todo!()
}

pub(crate) type FUN_CreateGraphicsPipelines = unsafe extern "C" fn(
    DeviceHandle,
    PipelineCache,
    u32,
    *const GraphicsPipelineCreateInfo,
    *const AllocationCallbacks,
    *mut Pipeline,
) -> ResultCode;
/// [`vkCreateGraphicsPipelines`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateGraphicsPipelines.html)
///
/// # Optional parameters
/// - pipeline_cache
/// - allocator
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
#[inline(always)]
pub unsafe fn create_graphics_pipelines(
    device: DeviceHandle,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    create_infos: *const GraphicsPipelineCreateInfo,
    allocator: *const AllocationCallbacks,
    pipelines: *mut Pipeline,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateFramebuffer = unsafe extern "C" fn(
    DeviceHandle,
    *const FramebufferCreateInfo,
    *const AllocationCallbacks,
    *mut Framebuffer,
) -> ResultCode;
/// [`vkCreateFramebuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateFramebuffer.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_framebuffer(
    device: DeviceHandle,
    create_info: *const FramebufferCreateInfo,
    allocator: *const AllocationCallbacks,
    framebuffer: *mut Framebuffer,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyFramebuffer =
    unsafe extern "C" fn(DeviceHandle, Framebuffer, *const AllocationCallbacks);
/// [`vkDestroyFramebuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFramebuffer.html)
///
/// # Optional parameters
/// - framebuffer
/// - allocator
///
#[doc(alias = "vkDestroyFramebuffer")]
#[inline(always)]
pub unsafe fn destroy_framebuffer(
    device: DeviceHandle,
    framebuffer: Framebuffer,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_CreateRenderPass = unsafe extern "C" fn(
    DeviceHandle,
    *const RenderPassCreateInfo,
    *const AllocationCallbacks,
    *mut RenderPass,
) -> ResultCode;
/// [`vkCreateRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_render_pass(
    device: DeviceHandle,
    create_info: *const RenderPassCreateInfo,
    allocator: *const AllocationCallbacks,
    render_pass: *mut RenderPass,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyRenderPass =
    unsafe extern "C" fn(DeviceHandle, RenderPass, *const AllocationCallbacks);
/// [`vkDestroyRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyRenderPass.html)
///
/// # Optional parameters
/// - render_pass
/// - allocator
///
#[doc(alias = "vkDestroyRenderPass")]
#[inline(always)]
pub unsafe fn destroy_render_pass(
    device: DeviceHandle,
    render_pass: RenderPass,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_GetRenderAreaGranularity =
    unsafe extern "C" fn(DeviceHandle, RenderPass, *mut Extent2D);
/// [`vkGetRenderAreaGranularity`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderAreaGranularity.html)
///
#[doc(alias = "vkGetRenderAreaGranularity")]
#[inline(always)]
pub unsafe fn get_render_area_granularity(
    device: DeviceHandle,
    render_pass: RenderPass,
    granularity: *mut Extent2D,
) {
    todo!()
}

pub(crate) type FUN_CmdSetViewport =
    unsafe extern "C" fn(CommandBufferHandle, u32, u32, *const Viewport);
/// [`vkCmdSetViewport`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewport.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetViewport")]
#[inline(always)]
pub unsafe fn cmd_set_viewport(
    command_buffer: CommandBufferHandle,
    first_viewport: u32,
    viewport_count: u32,
    viewports: *const Viewport,
) {
    todo!()
}

pub(crate) type FUN_CmdSetScissor =
    unsafe extern "C" fn(CommandBufferHandle, u32, u32, *const Rect2D);
/// [`vkCmdSetScissor`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissor.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetScissor")]
#[inline(always)]
pub unsafe fn cmd_set_scissor(
    command_buffer: CommandBufferHandle,
    first_scissor: u32,
    scissor_count: u32,
    scissors: *const Rect2D,
) {
    todo!()
}

pub(crate) type FUN_CmdSetLineWidth = unsafe extern "C" fn(CommandBufferHandle, f32);
/// [`vkCmdSetLineWidth`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineWidth.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetLineWidth")]
#[inline(always)]
pub unsafe fn cmd_set_line_width(command_buffer: CommandBufferHandle, line_width: f32) {
    todo!()
}

pub(crate) type FUN_CmdSetDepthBias = unsafe extern "C" fn(CommandBufferHandle, f32, f32, f32);
/// [`vkCmdSetDepthBias`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetDepthBias")]
#[inline(always)]
pub unsafe fn cmd_set_depth_bias(
    command_buffer: CommandBufferHandle,
    depth_bias_constant_factor: f32,
    depth_bias_clamp: f32,
    depth_bias_slope_factor: f32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetBlendConstants =
    unsafe extern "C" fn(CommandBufferHandle, *const [f32; 4 as usize]);
/// [`vkCmdSetBlendConstants`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetBlendConstants.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetBlendConstants")]
#[inline(always)]
pub unsafe fn cmd_set_blend_constants(
    command_buffer: CommandBufferHandle,
    blend_constants: *const [f32; 4 as usize],
) {
    todo!()
}

pub(crate) type FUN_CmdSetDepthBounds = unsafe extern "C" fn(CommandBufferHandle, f32, f32);
/// [`vkCmdSetDepthBounds`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBounds.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetDepthBounds")]
#[inline(always)]
pub unsafe fn cmd_set_depth_bounds(
    command_buffer: CommandBufferHandle,
    min_depth_bounds: f32,
    max_depth_bounds: f32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetStencilCompareMask =
    unsafe extern "C" fn(CommandBufferHandle, StencilFaceFlags, u32);
/// [`vkCmdSetStencilCompareMask`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilCompareMask.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetStencilCompareMask")]
#[inline(always)]
pub unsafe fn cmd_set_stencil_compare_mask(
    command_buffer: CommandBufferHandle,
    face_mask: StencilFaceFlags,
    compare_mask: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetStencilWriteMask =
    unsafe extern "C" fn(CommandBufferHandle, StencilFaceFlags, u32);
/// [`vkCmdSetStencilWriteMask`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilWriteMask.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetStencilWriteMask")]
#[inline(always)]
pub unsafe fn cmd_set_stencil_write_mask(
    command_buffer: CommandBufferHandle,
    face_mask: StencilFaceFlags,
    write_mask: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetStencilReference =
    unsafe extern "C" fn(CommandBufferHandle, StencilFaceFlags, u32);
/// [`vkCmdSetStencilReference`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilReference.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetStencilReference")]
#[inline(always)]
pub unsafe fn cmd_set_stencil_reference(
    command_buffer: CommandBufferHandle,
    face_mask: StencilFaceFlags,
    reference: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdBindIndexBuffer =
    unsafe extern "C" fn(CommandBufferHandle, Buffer, DeviceSize, IndexType);
/// [`vkCmdBindIndexBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer.html)
///
/// # Optional parameters
/// - buffer
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdBindIndexBuffer")]
#[inline(always)]
pub unsafe fn cmd_bind_index_buffer(
    command_buffer: CommandBufferHandle,
    buffer: Buffer,
    offset: DeviceSize,
    index_type: IndexType,
) {
    todo!()
}

pub(crate) type FUN_CmdBindVertexBuffers =
    unsafe extern "C" fn(CommandBufferHandle, u32, u32, *const Buffer, *const DeviceSize);
/// [`vkCmdBindVertexBuffers`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdBindVertexBuffers")]
#[inline(always)]
pub unsafe fn cmd_bind_vertex_buffers(
    command_buffer: CommandBufferHandle,
    first_binding: u32,
    binding_count: u32,
    buffers: *const Buffer,
    offsets: *const DeviceSize,
) {
    todo!()
}

pub(crate) type FUN_CmdDraw = unsafe extern "C" fn(CommandBufferHandle, u32, u32, u32, u32);
/// [`vkCmdDraw`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDraw.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDraw")]
#[inline(always)]
pub unsafe fn cmd_draw(
    command_buffer: CommandBufferHandle,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdDrawIndexed =
    unsafe extern "C" fn(CommandBufferHandle, u32, u32, u32, i32, u32);
/// [`vkCmdDrawIndexed`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexed.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawIndexed")]
#[inline(always)]
pub unsafe fn cmd_draw_indexed(
    command_buffer: CommandBufferHandle,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    vertex_offset: i32,
    first_instance: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdDrawIndirect =
    unsafe extern "C" fn(CommandBufferHandle, Buffer, DeviceSize, u32, u32);
/// [`vkCmdDrawIndirect`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirect.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawIndirect")]
#[inline(always)]
pub unsafe fn cmd_draw_indirect(
    command_buffer: CommandBufferHandle,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdDrawIndexedIndirect =
    unsafe extern "C" fn(CommandBufferHandle, Buffer, DeviceSize, u32, u32);
/// [`vkCmdDrawIndexedIndirect`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirect.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawIndexedIndirect")]
#[inline(always)]
pub unsafe fn cmd_draw_indexed_indirect(
    command_buffer: CommandBufferHandle,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdBlitImage = unsafe extern "C" fn(
    CommandBufferHandle,
    Image,
    ImageLayout,
    Image,
    ImageLayout,
    u32,
    *const ImageBlit,
    Filter,
);
/// [`vkCmdBlitImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdBlitImage")]
#[inline(always)]
pub unsafe fn cmd_blit_image(
    command_buffer: CommandBufferHandle,
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

pub(crate) type FUN_CmdClearDepthStencilImage = unsafe extern "C" fn(
    CommandBufferHandle,
    Image,
    ImageLayout,
    *const ClearDepthStencilValue,
    u32,
    *const ImageSubresourceRange,
);
/// [`vkCmdClearDepthStencilImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearDepthStencilImage.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdClearDepthStencilImage")]
#[inline(always)]
pub unsafe fn cmd_clear_depth_stencil_image(
    command_buffer: CommandBufferHandle,
    image: Image,
    image_layout: ImageLayout,
    depth_stencil: *const ClearDepthStencilValue,
    range_count: u32,
    ranges: *const ImageSubresourceRange,
) {
    todo!()
}

pub(crate) type FUN_CmdClearAttachments =
    unsafe extern "C" fn(CommandBufferHandle, u32, *const ClearAttachment, u32, *const ClearRect);
/// [`vkCmdClearAttachments`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearAttachments.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdClearAttachments")]
#[inline(always)]
pub unsafe fn cmd_clear_attachments(
    command_buffer: CommandBufferHandle,
    attachment_count: u32,
    attachments: *const ClearAttachment,
    rect_count: u32,
    rects: *const ClearRect,
) {
    todo!()
}

pub(crate) type FUN_CmdResolveImage = unsafe extern "C" fn(
    CommandBufferHandle,
    Image,
    ImageLayout,
    Image,
    ImageLayout,
    u32,
    *const ImageResolve,
);
/// [`vkCmdResolveImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdResolveImage")]
#[inline(always)]
pub unsafe fn cmd_resolve_image(
    command_buffer: CommandBufferHandle,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    regions: *const ImageResolve,
) {
    todo!()
}

pub(crate) type FUN_CmdBeginRenderPass =
    unsafe extern "C" fn(CommandBufferHandle, *const RenderPassBeginInfo, SubpassContents);
/// [`vkCmdBeginRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
/// - `synchronization`
///
/// # Allowed command buffers
/// - Primary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdBeginRenderPass")]
#[inline(always)]
pub unsafe fn cmd_begin_render_pass(
    command_buffer: CommandBufferHandle,
    render_pass_begin: *const RenderPassBeginInfo,
    contents: SubpassContents,
) {
    todo!()
}

pub(crate) type FUN_CmdNextSubpass = unsafe extern "C" fn(CommandBufferHandle, SubpassContents);
/// [`vkCmdNextSubpass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
/// - `synchronization`
///
/// # Allowed command buffers
/// - Primary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdNextSubpass")]
#[inline(always)]
pub unsafe fn cmd_next_subpass(command_buffer: CommandBufferHandle, contents: SubpassContents) {
    todo!()
}

pub(crate) type FUN_CmdEndRenderPass = unsafe extern "C" fn(CommandBufferHandle);
/// [`vkCmdEndRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
/// - `synchronization`
///
/// # Allowed command buffers
/// - Primary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdEndRenderPass")]
#[inline(always)]
pub unsafe fn cmd_end_render_pass(command_buffer: CommandBufferHandle) {
    todo!()
}

pub(crate) type FUN_EnumerateInstanceVersion = unsafe extern "C" fn(*mut u32) -> ResultCode;
/// [`vkEnumerateInstanceVersion`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceVersion.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkEnumerateInstanceVersion")]
#[inline(always)]
pub unsafe fn enumerate_instance_version(api_version: *mut u32) -> ResultCode {
    todo!()
}

pub(crate) type FUN_BindBufferMemory2 =
    unsafe extern "C" fn(DeviceHandle, u32, *const BindBufferMemoryInfo) -> ResultCode;
/// [`vkBindBufferMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory2.html)
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
#[inline(always)]
pub unsafe fn bind_buffer_memory_2(
    device: DeviceHandle,
    bind_info_count: u32,
    bind_infos: *const BindBufferMemoryInfo,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_BindImageMemory2 =
    unsafe extern "C" fn(DeviceHandle, u32, *const BindImageMemoryInfo) -> ResultCode;
/// [`vkBindImageMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory2.html)
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
#[inline(always)]
pub unsafe fn bind_image_memory_2(
    device: DeviceHandle,
    bind_info_count: u32,
    bind_infos: *const BindImageMemoryInfo,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetDeviceGroupPeerMemoryFeatures =
    unsafe extern "C" fn(DeviceHandle, u32, u32, u32, *mut PeerMemoryFeatureFlags);
/// [`vkGetDeviceGroupPeerMemoryFeatures`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPeerMemoryFeatures.html)
///
#[doc(alias = "vkGetDeviceGroupPeerMemoryFeatures")]
#[inline(always)]
pub unsafe fn get_device_group_peer_memory_features(
    device: DeviceHandle,
    heap_index: u32,
    local_device_index: u32,
    remote_device_index: u32,
    peer_memory_features: *mut PeerMemoryFeatureFlags,
) {
    todo!()
}

pub(crate) type FUN_CmdSetDeviceMask = unsafe extern "C" fn(CommandBufferHandle, u32);
/// [`vkCmdSetDeviceMask`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDeviceMask.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`TRANSFER`](QueueFlags::TRANSFER)
///
#[doc(alias = "vkCmdSetDeviceMask")]
#[inline(always)]
pub unsafe fn cmd_set_device_mask(command_buffer: CommandBufferHandle, device_mask: u32) {
    todo!()
}

pub(crate) type FUN_EnumeratePhysicalDeviceGroups = unsafe extern "C" fn(
    InstanceHandle,
    *mut u32,
    *mut PhysicalDeviceGroupProperties,
) -> ResultCode;
/// [`vkEnumeratePhysicalDeviceGroups`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroups.html)
///
/// # Optional parameters
/// - physical_device_group_properties
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
#[inline(always)]
pub unsafe fn enumerate_physical_device_groups(
    instance: InstanceHandle,
    physical_device_group_count: *mut u32,
    physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetImageMemoryRequirements2 = unsafe extern "C" fn(
    DeviceHandle,
    *const ImageMemoryRequirementsInfo2,
    *mut MemoryRequirements2,
);
/// [`vkGetImageMemoryRequirements2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements2.html)
///
#[doc(alias = "vkGetImageMemoryRequirements2")]
#[inline(always)]
pub unsafe fn get_image_memory_requirements_2(
    device: DeviceHandle,
    info: *const ImageMemoryRequirementsInfo2,
    memory_requirements: *mut MemoryRequirements2,
) {
    todo!()
}

pub(crate) type FUN_GetBufferMemoryRequirements2 = unsafe extern "C" fn(
    DeviceHandle,
    *const BufferMemoryRequirementsInfo2,
    *mut MemoryRequirements2,
);
/// [`vkGetBufferMemoryRequirements2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements2.html)
///
#[doc(alias = "vkGetBufferMemoryRequirements2")]
#[inline(always)]
pub unsafe fn get_buffer_memory_requirements_2(
    device: DeviceHandle,
    info: *const BufferMemoryRequirementsInfo2,
    memory_requirements: *mut MemoryRequirements2,
) {
    todo!()
}

pub(crate) type FUN_GetImageSparseMemoryRequirements2 = unsafe extern "C" fn(
    DeviceHandle,
    *const ImageSparseMemoryRequirementsInfo2,
    *mut u32,
    *mut SparseImageMemoryRequirements2,
);
/// [`vkGetImageSparseMemoryRequirements2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2.html)
///
/// # Optional parameters
/// - sparse_memory_requirements
///
#[doc(alias = "vkGetImageSparseMemoryRequirements2")]
#[inline(always)]
pub unsafe fn get_image_sparse_memory_requirements_2(
    device: DeviceHandle,
    info: *const ImageSparseMemoryRequirementsInfo2,
    sparse_memory_requirement_count: *mut u32,
    sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceFeatures2 =
    unsafe extern "C" fn(PhysicalDeviceHandle, *mut PhysicalDeviceFeatures2);
/// [`vkGetPhysicalDeviceFeatures2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures2.html)
///
#[doc(alias = "vkGetPhysicalDeviceFeatures2")]
#[inline(always)]
pub unsafe fn get_physical_device_features_2(
    physical_device: PhysicalDeviceHandle,
    features: *mut PhysicalDeviceFeatures2,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceProperties2 =
    unsafe extern "C" fn(PhysicalDeviceHandle, *mut PhysicalDeviceProperties2);
/// [`vkGetPhysicalDeviceProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties2.html)
///
#[doc(alias = "vkGetPhysicalDeviceProperties2")]
#[inline(always)]
pub unsafe fn get_physical_device_properties_2(
    physical_device: PhysicalDeviceHandle,
    properties: *mut PhysicalDeviceProperties2,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceFormatProperties2 =
    unsafe extern "C" fn(PhysicalDeviceHandle, Format, *mut FormatProperties2);
/// [`vkGetPhysicalDeviceFormatProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties2.html)
///
#[doc(alias = "vkGetPhysicalDeviceFormatProperties2")]
#[inline(always)]
pub unsafe fn get_physical_device_format_properties_2(
    physical_device: PhysicalDeviceHandle,
    format: Format,
    format_properties: *mut FormatProperties2,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceImageFormatProperties2 = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *const PhysicalDeviceImageFormatInfo2,
    *mut ImageFormatProperties2,
) -> ResultCode;
/// [`vkGetPhysicalDeviceImageFormatProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties2.html)
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
#[inline(always)]
pub unsafe fn get_physical_device_image_format_properties_2(
    physical_device: PhysicalDeviceHandle,
    image_format_info: *const PhysicalDeviceImageFormatInfo2,
    image_format_properties: *mut ImageFormatProperties2,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceQueueFamilyProperties2 =
    unsafe extern "C" fn(PhysicalDeviceHandle, *mut u32, *mut QueueFamilyProperties2);
/// [`vkGetPhysicalDeviceQueueFamilyProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2.html)
///
/// # Optional parameters
/// - queue_family_properties
///
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2")]
#[inline(always)]
pub unsafe fn get_physical_device_queue_family_properties_2(
    physical_device: PhysicalDeviceHandle,
    queue_family_property_count: *mut u32,
    queue_family_properties: *mut QueueFamilyProperties2,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceMemoryProperties2 =
    unsafe extern "C" fn(PhysicalDeviceHandle, *mut PhysicalDeviceMemoryProperties2);
/// [`vkGetPhysicalDeviceMemoryProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties2.html)
///
#[doc(alias = "vkGetPhysicalDeviceMemoryProperties2")]
#[inline(always)]
pub unsafe fn get_physical_device_memory_properties_2(
    physical_device: PhysicalDeviceHandle,
    memory_properties: *mut PhysicalDeviceMemoryProperties2,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceSparseImageFormatProperties2 = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *const PhysicalDeviceSparseImageFormatInfo2,
    *mut u32,
    *mut SparseImageFormatProperties2,
);
/// [`vkGetPhysicalDeviceSparseImageFormatProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2.html)
///
/// # Optional parameters
/// - properties
///
#[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2")]
#[inline(always)]
pub unsafe fn get_physical_device_sparse_image_format_properties_2(
    physical_device: PhysicalDeviceHandle,
    format_info: *const PhysicalDeviceSparseImageFormatInfo2,
    property_count: *mut u32,
    properties: *mut SparseImageFormatProperties2,
) {
    todo!()
}

pub(crate) type FUN_TrimCommandPool =
    unsafe extern "C" fn(DeviceHandle, CommandPool, CommandPoolTrimFlags);
/// [`vkTrimCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTrimCommandPool.html)
///
/// # Optional parameters
/// - flags
///
#[doc(alias = "vkTrimCommandPool")]
#[inline(always)]
pub unsafe fn trim_command_pool(
    device: DeviceHandle,
    command_pool: CommandPool,
    flags: CommandPoolTrimFlags,
) {
    todo!()
}

pub(crate) type FUN_GetDeviceQueue2 =
    unsafe extern "C" fn(DeviceHandle, *const DeviceQueueInfo2, *mut QueueHandle);
/// [`vkGetDeviceQueue2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue2.html)
///
#[doc(alias = "vkGetDeviceQueue2")]
#[inline(always)]
pub unsafe fn get_device_queue_2(
    device: DeviceHandle,
    queue_info: *const DeviceQueueInfo2,
    queue: *mut QueueHandle,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceExternalBufferProperties = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *const PhysicalDeviceExternalBufferInfo,
    *mut ExternalBufferProperties,
);
/// [`vkGetPhysicalDeviceExternalBufferProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalBufferProperties.html)
///
#[doc(alias = "vkGetPhysicalDeviceExternalBufferProperties")]
#[inline(always)]
pub unsafe fn get_physical_device_external_buffer_properties(
    physical_device: PhysicalDeviceHandle,
    external_buffer_info: *const PhysicalDeviceExternalBufferInfo,
    external_buffer_properties: *mut ExternalBufferProperties,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceExternalFenceProperties = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *const PhysicalDeviceExternalFenceInfo,
    *mut ExternalFenceProperties,
);
/// [`vkGetPhysicalDeviceExternalFenceProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalFenceProperties.html)
///
#[doc(alias = "vkGetPhysicalDeviceExternalFenceProperties")]
#[inline(always)]
pub unsafe fn get_physical_device_external_fence_properties(
    physical_device: PhysicalDeviceHandle,
    external_fence_info: *const PhysicalDeviceExternalFenceInfo,
    external_fence_properties: *mut ExternalFenceProperties,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceExternalSemaphoreProperties = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *const PhysicalDeviceExternalSemaphoreInfo,
    *mut ExternalSemaphoreProperties,
);
/// [`vkGetPhysicalDeviceExternalSemaphoreProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalSemaphoreProperties.html)
///
#[doc(alias = "vkGetPhysicalDeviceExternalSemaphoreProperties")]
#[inline(always)]
pub unsafe fn get_physical_device_external_semaphore_properties(
    physical_device: PhysicalDeviceHandle,
    external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo,
    external_semaphore_properties: *mut ExternalSemaphoreProperties,
) {
    todo!()
}

pub(crate) type FUN_CmdDispatchBase =
    unsafe extern "C" fn(CommandBufferHandle, u32, u32, u32, u32, u32, u32);
/// [`vkCmdDispatchBase`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchBase.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdDispatchBase")]
#[inline(always)]
pub unsafe fn cmd_dispatch_base(
    command_buffer: CommandBufferHandle,
    base_group_x: u32,
    base_group_y: u32,
    base_group_z: u32,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
) {
    todo!()
}

pub(crate) type FUN_CreateDescriptorUpdateTemplate = unsafe extern "C" fn(
    DeviceHandle,
    *const DescriptorUpdateTemplateCreateInfo,
    *const AllocationCallbacks,
    *mut DescriptorUpdateTemplate,
) -> ResultCode;
/// [`vkCreateDescriptorUpdateTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorUpdateTemplate.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_descriptor_update_template(
    device: DeviceHandle,
    create_info: *const DescriptorUpdateTemplateCreateInfo,
    allocator: *const AllocationCallbacks,
    descriptor_update_template: *mut DescriptorUpdateTemplate,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyDescriptorUpdateTemplate =
    unsafe extern "C" fn(DeviceHandle, DescriptorUpdateTemplate, *const AllocationCallbacks);
/// [`vkDestroyDescriptorUpdateTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplate.html)
///
/// # Optional parameters
/// - descriptor_update_template
/// - allocator
///
#[doc(alias = "vkDestroyDescriptorUpdateTemplate")]
#[inline(always)]
pub unsafe fn destroy_descriptor_update_template(
    device: DeviceHandle,
    descriptor_update_template: DescriptorUpdateTemplate,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_UpdateDescriptorSetWithTemplate =
    unsafe extern "C" fn(DeviceHandle, DescriptorSet, DescriptorUpdateTemplate, *const c_void);
/// [`vkUpdateDescriptorSetWithTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplate.html)
///
#[doc(alias = "vkUpdateDescriptorSetWithTemplate")]
#[inline(always)]
pub unsafe fn update_descriptor_set_with_template(
    device: DeviceHandle,
    descriptor_set: DescriptorSet,
    descriptor_update_template: DescriptorUpdateTemplate,
    data: *const c_void,
) {
    todo!()
}

pub(crate) type FUN_GetDescriptorSetLayoutSupport = unsafe extern "C" fn(
    DeviceHandle,
    *const DescriptorSetLayoutCreateInfo,
    *mut DescriptorSetLayoutSupport,
);
/// [`vkGetDescriptorSetLayoutSupport`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSupport.html)
///
#[doc(alias = "vkGetDescriptorSetLayoutSupport")]
#[inline(always)]
pub unsafe fn get_descriptor_set_layout_support(
    device: DeviceHandle,
    create_info: *const DescriptorSetLayoutCreateInfo,
    support: *mut DescriptorSetLayoutSupport,
) {
    todo!()
}

pub(crate) type FUN_CreateSamplerYcbcrConversion = unsafe extern "C" fn(
    DeviceHandle,
    *const SamplerYcbcrConversionCreateInfo,
    *const AllocationCallbacks,
    *mut SamplerYcbcrConversion,
) -> ResultCode;
/// [`vkCreateSamplerYcbcrConversion`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSamplerYcbcrConversion.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_sampler_ycbcr_conversion(
    device: DeviceHandle,
    create_info: *const SamplerYcbcrConversionCreateInfo,
    allocator: *const AllocationCallbacks,
    ycbcr_conversion: *mut SamplerYcbcrConversion,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroySamplerYcbcrConversion =
    unsafe extern "C" fn(DeviceHandle, SamplerYcbcrConversion, *const AllocationCallbacks);
/// [`vkDestroySamplerYcbcrConversion`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversion.html)
///
/// # Optional parameters
/// - ycbcr_conversion
/// - allocator
///
#[doc(alias = "vkDestroySamplerYcbcrConversion")]
#[inline(always)]
pub unsafe fn destroy_sampler_ycbcr_conversion(
    device: DeviceHandle,
    ycbcr_conversion: SamplerYcbcrConversion,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_ResetQueryPool = unsafe extern "C" fn(DeviceHandle, QueryPool, u32, u32);
/// [`vkResetQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetQueryPool.html)
///
#[doc(alias = "vkResetQueryPool")]
#[inline(always)]
pub unsafe fn reset_query_pool(
    device: DeviceHandle,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
) {
    todo!()
}

pub(crate) type FUN_GetSemaphoreCounterValue =
    unsafe extern "C" fn(DeviceHandle, Semaphore, *mut u64) -> ResultCode;
/// [`vkGetSemaphoreCounterValue`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValue.html)
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
#[inline(always)]
pub unsafe fn get_semaphore_counter_value(
    device: DeviceHandle,
    semaphore: Semaphore,
    value: *mut u64,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_WaitSemaphores =
    unsafe extern "C" fn(DeviceHandle, *const SemaphoreWaitInfo, u64) -> ResultCode;
/// [`vkWaitSemaphores`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitSemaphores.html)
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
#[inline(always)]
pub unsafe fn wait_semaphores(
    device: DeviceHandle,
    wait_info: *const SemaphoreWaitInfo,
    timeout: u64,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_SignalSemaphore =
    unsafe extern "C" fn(DeviceHandle, *const SemaphoreSignalInfo) -> ResultCode;
/// [`vkSignalSemaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSignalSemaphore.html)
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
#[inline(always)]
pub unsafe fn signal_semaphore(
    device: DeviceHandle,
    signal_info: *const SemaphoreSignalInfo,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetBufferDeviceAddress =
    unsafe extern "C" fn(DeviceHandle, *const BufferDeviceAddressInfo) -> DeviceAddress;
/// [`vkGetBufferDeviceAddress`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddress.html)
///
#[doc(alias = "vkGetBufferDeviceAddress")]
#[inline(always)]
pub unsafe fn get_buffer_device_address(
    device: DeviceHandle,
    info: *const BufferDeviceAddressInfo,
) -> DeviceAddress {
    todo!()
}

pub(crate) type FUN_GetBufferOpaqueCaptureAddress =
    unsafe extern "C" fn(DeviceHandle, *const BufferDeviceAddressInfo) -> u64;
/// [`vkGetBufferOpaqueCaptureAddress`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureAddress.html)
///
#[doc(alias = "vkGetBufferOpaqueCaptureAddress")]
#[inline(always)]
pub unsafe fn get_buffer_opaque_capture_address(
    device: DeviceHandle,
    info: *const BufferDeviceAddressInfo,
) -> u64 {
    todo!()
}

pub(crate) type FUN_GetDeviceMemoryOpaqueCaptureAddress =
    unsafe extern "C" fn(DeviceHandle, *const DeviceMemoryOpaqueCaptureAddressInfo) -> u64;
/// [`vkGetDeviceMemoryOpaqueCaptureAddress`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryOpaqueCaptureAddress.html)
///
#[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddress")]
#[inline(always)]
pub unsafe fn get_device_memory_opaque_capture_address(
    device: DeviceHandle,
    info: *const DeviceMemoryOpaqueCaptureAddressInfo,
) -> u64 {
    todo!()
}

pub(crate) type FUN_CmdDrawIndirectCount =
    unsafe extern "C" fn(CommandBufferHandle, Buffer, DeviceSize, Buffer, DeviceSize, u32, u32);
/// [`vkCmdDrawIndirectCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawIndirectCount")]
#[inline(always)]
pub unsafe fn cmd_draw_indirect_count(
    command_buffer: CommandBufferHandle,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdDrawIndexedIndirectCount =
    unsafe extern "C" fn(CommandBufferHandle, Buffer, DeviceSize, Buffer, DeviceSize, u32, u32);
/// [`vkCmdDrawIndexedIndirectCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawIndexedIndirectCount")]
#[inline(always)]
pub unsafe fn cmd_draw_indexed_indirect_count(
    command_buffer: CommandBufferHandle,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
) {
    todo!()
}

pub(crate) type FUN_CreateRenderPass2 = unsafe extern "C" fn(
    DeviceHandle,
    *const RenderPassCreateInfo2,
    *const AllocationCallbacks,
    *mut RenderPass,
) -> ResultCode;
/// [`vkCreateRenderPass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass2.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_render_pass_2(
    device: DeviceHandle,
    create_info: *const RenderPassCreateInfo2,
    allocator: *const AllocationCallbacks,
    render_pass: *mut RenderPass,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdBeginRenderPass2 =
    unsafe extern "C" fn(CommandBufferHandle, *const RenderPassBeginInfo, *const SubpassBeginInfo);
/// [`vkCmdBeginRenderPass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass2.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
/// - `synchronization`
///
/// # Allowed command buffers
/// - Primary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdBeginRenderPass2")]
#[inline(always)]
pub unsafe fn cmd_begin_render_pass_2(
    command_buffer: CommandBufferHandle,
    render_pass_begin: *const RenderPassBeginInfo,
    subpass_begin_info: *const SubpassBeginInfo,
) {
    todo!()
}

pub(crate) type FUN_CmdNextSubpass2 =
    unsafe extern "C" fn(CommandBufferHandle, *const SubpassBeginInfo, *const SubpassEndInfo);
/// [`vkCmdNextSubpass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass2.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
/// - `synchronization`
///
/// # Allowed command buffers
/// - Primary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdNextSubpass2")]
#[inline(always)]
pub unsafe fn cmd_next_subpass_2(
    command_buffer: CommandBufferHandle,
    subpass_begin_info: *const SubpassBeginInfo,
    subpass_end_info: *const SubpassEndInfo,
) {
    todo!()
}

pub(crate) type FUN_CmdEndRenderPass2 =
    unsafe extern "C" fn(CommandBufferHandle, *const SubpassEndInfo);
/// [`vkCmdEndRenderPass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass2.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
/// - `synchronization`
///
/// # Allowed command buffers
/// - Primary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdEndRenderPass2")]
#[inline(always)]
pub unsafe fn cmd_end_render_pass_2(
    command_buffer: CommandBufferHandle,
    subpass_end_info: *const SubpassEndInfo,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceToolProperties = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *mut u32,
    *mut PhysicalDeviceToolProperties,
) -> ResultCode;
/// [`vkGetPhysicalDeviceToolProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolProperties.html)
///
/// # Optional parameters
/// - tool_properties
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
#[inline(always)]
pub unsafe fn get_physical_device_tool_properties(
    physical_device: PhysicalDeviceHandle,
    tool_count: *mut u32,
    tool_properties: *mut PhysicalDeviceToolProperties,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreatePrivateDataSlot = unsafe extern "C" fn(
    DeviceHandle,
    *const PrivateDataSlotCreateInfo,
    *const AllocationCallbacks,
    *mut PrivateDataSlot,
) -> ResultCode;
/// [`vkCreatePrivateDataSlot`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePrivateDataSlot.html)
///
/// # Optional parameters
/// - allocator
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkCreatePrivateDataSlot")]
#[inline(always)]
pub unsafe fn create_private_data_slot(
    device: DeviceHandle,
    create_info: *const PrivateDataSlotCreateInfo,
    allocator: *const AllocationCallbacks,
    private_data_slot: *mut PrivateDataSlot,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyPrivateDataSlot =
    unsafe extern "C" fn(DeviceHandle, PrivateDataSlot, *const AllocationCallbacks);
/// [`vkDestroyPrivateDataSlot`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlot.html)
///
/// # Optional parameters
/// - private_data_slot
/// - allocator
///
#[doc(alias = "vkDestroyPrivateDataSlot")]
#[inline(always)]
pub unsafe fn destroy_private_data_slot(
    device: DeviceHandle,
    private_data_slot: PrivateDataSlot,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_SetPrivateData =
    unsafe extern "C" fn(DeviceHandle, ObjectType, u64, PrivateDataSlot, u64) -> ResultCode;
/// [`vkSetPrivateData`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetPrivateData.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkSetPrivateData")]
#[inline(always)]
pub unsafe fn set_private_data(
    device: DeviceHandle,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: PrivateDataSlot,
    data: u64,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPrivateData =
    unsafe extern "C" fn(DeviceHandle, ObjectType, u64, PrivateDataSlot, *mut u64);
/// [`vkGetPrivateData`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPrivateData.html)
///
#[doc(alias = "vkGetPrivateData")]
#[inline(always)]
pub unsafe fn get_private_data(
    device: DeviceHandle,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: PrivateDataSlot,
    data: *mut u64,
) {
    todo!()
}

pub(crate) type FUN_CmdPipelineBarrier2 =
    unsafe extern "C" fn(CommandBufferHandle, *const DependencyInfo);
/// [`vkCmdPipelineBarrier2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier2.html)
///
/// # Performed tasks
/// - `synchronization`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
///
#[doc(alias = "vkCmdPipelineBarrier2")]
#[inline(always)]
pub unsafe fn cmd_pipeline_barrier_2(
    command_buffer: CommandBufferHandle,
    dependency_info: *const DependencyInfo,
) {
    todo!()
}

pub(crate) type FUN_CmdWriteTimestamp2 =
    unsafe extern "C" fn(CommandBufferHandle, PipelineStageFlags2, QueryPool, u32);
/// [`vkCmdWriteTimestamp2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp2.html)
///
/// # Optional parameters
/// - stage
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
///
#[doc(alias = "vkCmdWriteTimestamp2")]
#[inline(always)]
pub unsafe fn cmd_write_timestamp_2(
    command_buffer: CommandBufferHandle,
    stage: PipelineStageFlags2,
    query_pool: QueryPool,
    query: u32,
) {
    todo!()
}

pub(crate) type FUN_QueueSubmit2 =
    unsafe extern "C" fn(QueueHandle, u32, *const SubmitInfo2, Fence) -> ResultCode;
/// [`vkQueueSubmit2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2.html)
///
/// # Optional parameters
/// - submit_count
/// - fence
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
#[inline(always)]
pub unsafe fn queue_submit_2(
    queue: QueueHandle,
    submit_count: u32,
    submits: *const SubmitInfo2,
    fence: Fence,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdCopyBuffer2 =
    unsafe extern "C" fn(CommandBufferHandle, *const CopyBufferInfo2);
/// [`vkCmdCopyBuffer2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyBuffer2")]
#[inline(always)]
pub unsafe fn cmd_copy_buffer_2(
    command_buffer: CommandBufferHandle,
    copy_buffer_info: *const CopyBufferInfo2,
) {
    todo!()
}

pub(crate) type FUN_CmdCopyImage2 =
    unsafe extern "C" fn(CommandBufferHandle, *const CopyImageInfo2);
/// [`vkCmdCopyImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyImage2")]
#[inline(always)]
pub unsafe fn cmd_copy_image_2(
    command_buffer: CommandBufferHandle,
    copy_image_info: *const CopyImageInfo2,
) {
    todo!()
}

pub(crate) type FUN_CmdCopyBufferToImage2 =
    unsafe extern "C" fn(CommandBufferHandle, *const CopyBufferToImageInfo2);
/// [`vkCmdCopyBufferToImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyBufferToImage2")]
#[inline(always)]
pub unsafe fn cmd_copy_buffer_to_image_2(
    command_buffer: CommandBufferHandle,
    copy_buffer_to_image_info: *const CopyBufferToImageInfo2,
) {
    todo!()
}

pub(crate) type FUN_CmdCopyImageToBuffer2 =
    unsafe extern "C" fn(CommandBufferHandle, *const CopyImageToBufferInfo2);
/// [`vkCmdCopyImageToBuffer2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyImageToBuffer2")]
#[inline(always)]
pub unsafe fn cmd_copy_image_to_buffer_2(
    command_buffer: CommandBufferHandle,
    copy_image_to_buffer_info: *const CopyImageToBufferInfo2,
) {
    todo!()
}

pub(crate) type FUN_GetDeviceBufferMemoryRequirements = unsafe extern "C" fn(
    DeviceHandle,
    *const DeviceBufferMemoryRequirements,
    *mut MemoryRequirements2,
);
/// [`vkGetDeviceBufferMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceBufferMemoryRequirements.html)
///
#[doc(alias = "vkGetDeviceBufferMemoryRequirements")]
#[inline(always)]
pub unsafe fn get_device_buffer_memory_requirements(
    device: DeviceHandle,
    info: *const DeviceBufferMemoryRequirements,
    memory_requirements: *mut MemoryRequirements2,
) {
    todo!()
}

pub(crate) type FUN_GetDeviceImageMemoryRequirements = unsafe extern "C" fn(
    DeviceHandle,
    *const DeviceImageMemoryRequirements,
    *mut MemoryRequirements2,
);
/// [`vkGetDeviceImageMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageMemoryRequirements.html)
///
#[doc(alias = "vkGetDeviceImageMemoryRequirements")]
#[inline(always)]
pub unsafe fn get_device_image_memory_requirements(
    device: DeviceHandle,
    info: *const DeviceImageMemoryRequirements,
    memory_requirements: *mut MemoryRequirements2,
) {
    todo!()
}

pub(crate) type FUN_GetDeviceImageSparseMemoryRequirements = unsafe extern "C" fn(
    DeviceHandle,
    *const DeviceImageMemoryRequirements,
    *mut u32,
    *mut SparseImageMemoryRequirements2,
);
/// [`vkGetDeviceImageSparseMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirements.html)
///
/// # Optional parameters
/// - sparse_memory_requirements
///
#[doc(alias = "vkGetDeviceImageSparseMemoryRequirements")]
#[inline(always)]
pub unsafe fn get_device_image_sparse_memory_requirements(
    device: DeviceHandle,
    info: *const DeviceImageMemoryRequirements,
    sparse_memory_requirement_count: *mut u32,
    sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
) {
    todo!()
}

pub(crate) type FUN_CmdSetEvent2 =
    unsafe extern "C" fn(CommandBufferHandle, Event, *const DependencyInfo);
/// [`vkCmdSetEvent2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent2.html)
///
/// # Performed tasks
/// - `synchronization`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
///
#[doc(alias = "vkCmdSetEvent2")]
#[inline(always)]
pub unsafe fn cmd_set_event_2(
    command_buffer: CommandBufferHandle,
    event: Event,
    dependency_info: *const DependencyInfo,
) {
    todo!()
}

pub(crate) type FUN_CmdResetEvent2 =
    unsafe extern "C" fn(CommandBufferHandle, Event, PipelineStageFlags2);
/// [`vkCmdResetEvent2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent2.html)
///
/// # Optional parameters
/// - stage_mask
///
/// # Performed tasks
/// - `synchronization`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
///
#[doc(alias = "vkCmdResetEvent2")]
#[inline(always)]
pub unsafe fn cmd_reset_event_2(
    command_buffer: CommandBufferHandle,
    event: Event,
    stage_mask: PipelineStageFlags2,
) {
    todo!()
}

pub(crate) type FUN_CmdWaitEvents2 =
    unsafe extern "C" fn(CommandBufferHandle, u32, *const Event, *const DependencyInfo);
/// [`vkCmdWaitEvents2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents2.html)
///
/// # Performed tasks
/// - `synchronization`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
///
#[doc(alias = "vkCmdWaitEvents2")]
#[inline(always)]
pub unsafe fn cmd_wait_events_2(
    command_buffer: CommandBufferHandle,
    event_count: u32,
    events: *const Event,
    dependency_infos: *const DependencyInfo,
) {
    todo!()
}

pub(crate) type FUN_CmdBlitImage2 =
    unsafe extern "C" fn(CommandBufferHandle, *const BlitImageInfo2);
/// [`vkCmdBlitImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdBlitImage2")]
#[inline(always)]
pub unsafe fn cmd_blit_image_2(
    command_buffer: CommandBufferHandle,
    blit_image_info: *const BlitImageInfo2,
) {
    todo!()
}

pub(crate) type FUN_CmdResolveImage2 =
    unsafe extern "C" fn(CommandBufferHandle, *const ResolveImageInfo2);
/// [`vkCmdResolveImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage2.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdResolveImage2")]
#[inline(always)]
pub unsafe fn cmd_resolve_image_2(
    command_buffer: CommandBufferHandle,
    resolve_image_info: *const ResolveImageInfo2,
) {
    todo!()
}

pub(crate) type FUN_CmdBeginRendering =
    unsafe extern "C" fn(CommandBufferHandle, *const RenderingInfo);
/// [`vkCmdBeginRendering`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRendering.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdBeginRendering")]
#[inline(always)]
pub unsafe fn cmd_begin_rendering(
    command_buffer: CommandBufferHandle,
    rendering_info: *const RenderingInfo,
) {
    todo!()
}

pub(crate) type FUN_CmdEndRendering = unsafe extern "C" fn(CommandBufferHandle);
/// [`vkCmdEndRendering`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdEndRendering")]
#[inline(always)]
pub unsafe fn cmd_end_rendering(command_buffer: CommandBufferHandle) {
    todo!()
}

pub(crate) type FUN_CmdSetCullMode = unsafe extern "C" fn(CommandBufferHandle, CullModeFlags);
/// [`vkCmdSetCullMode`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCullMode.html)
///
/// # Optional parameters
/// - cull_mode
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetCullMode")]
#[inline(always)]
pub unsafe fn cmd_set_cull_mode(command_buffer: CommandBufferHandle, cull_mode: CullModeFlags) {
    todo!()
}

pub(crate) type FUN_CmdSetFrontFace = unsafe extern "C" fn(CommandBufferHandle, FrontFace);
/// [`vkCmdSetFrontFace`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFrontFace.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetFrontFace")]
#[inline(always)]
pub unsafe fn cmd_set_front_face(command_buffer: CommandBufferHandle, front_face: FrontFace) {
    todo!()
}

pub(crate) type FUN_CmdSetPrimitiveTopology =
    unsafe extern "C" fn(CommandBufferHandle, PrimitiveTopology);
/// [`vkCmdSetPrimitiveTopology`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveTopology.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetPrimitiveTopology")]
#[inline(always)]
pub unsafe fn cmd_set_primitive_topology(
    command_buffer: CommandBufferHandle,
    primitive_topology: PrimitiveTopology,
) {
    todo!()
}

pub(crate) type FUN_CmdSetViewportWithCount =
    unsafe extern "C" fn(CommandBufferHandle, u32, *const Viewport);
/// [`vkCmdSetViewportWithCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCount.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetViewportWithCount")]
#[inline(always)]
pub unsafe fn cmd_set_viewport_with_count(
    command_buffer: CommandBufferHandle,
    viewport_count: u32,
    viewports: *const Viewport,
) {
    todo!()
}

pub(crate) type FUN_CmdSetScissorWithCount =
    unsafe extern "C" fn(CommandBufferHandle, u32, *const Rect2D);
/// [`vkCmdSetScissorWithCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCount.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetScissorWithCount")]
#[inline(always)]
pub unsafe fn cmd_set_scissor_with_count(
    command_buffer: CommandBufferHandle,
    scissor_count: u32,
    scissors: *const Rect2D,
) {
    todo!()
}

pub(crate) type FUN_CmdBindVertexBuffers2 = unsafe extern "C" fn(
    CommandBufferHandle,
    u32,
    u32,
    *const Buffer,
    *const DeviceSize,
    *const DeviceSize,
    *const DeviceSize,
);
/// [`vkCmdBindVertexBuffers2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2.html)
///
/// # Optional parameters
/// - sizes
/// - strides
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdBindVertexBuffers2")]
#[inline(always)]
pub unsafe fn cmd_bind_vertex_buffers_2(
    command_buffer: CommandBufferHandle,
    first_binding: u32,
    binding_count: u32,
    buffers: *const Buffer,
    offsets: *const DeviceSize,
    sizes: *const DeviceSize,
    strides: *const DeviceSize,
) {
    todo!()
}

pub(crate) type FUN_CmdSetDepthTestEnable = unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetDepthTestEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthTestEnable.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetDepthTestEnable")]
#[inline(always)]
pub unsafe fn cmd_set_depth_test_enable(
    command_buffer: CommandBufferHandle,
    depth_test_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetDepthWriteEnable = unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetDepthWriteEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthWriteEnable.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetDepthWriteEnable")]
#[inline(always)]
pub unsafe fn cmd_set_depth_write_enable(
    command_buffer: CommandBufferHandle,
    depth_write_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetDepthCompareOp = unsafe extern "C" fn(CommandBufferHandle, CompareOp);
/// [`vkCmdSetDepthCompareOp`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthCompareOp.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetDepthCompareOp")]
#[inline(always)]
pub unsafe fn cmd_set_depth_compare_op(
    command_buffer: CommandBufferHandle,
    depth_compare_op: CompareOp,
) {
    todo!()
}

pub(crate) type FUN_CmdSetDepthBoundsTestEnable = unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetDepthBoundsTestEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBoundsTestEnable.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetDepthBoundsTestEnable")]
#[inline(always)]
pub unsafe fn cmd_set_depth_bounds_test_enable(
    command_buffer: CommandBufferHandle,
    depth_bounds_test_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetStencilTestEnable = unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetStencilTestEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilTestEnable.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetStencilTestEnable")]
#[inline(always)]
pub unsafe fn cmd_set_stencil_test_enable(
    command_buffer: CommandBufferHandle,
    stencil_test_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetStencilOp = unsafe extern "C" fn(
    CommandBufferHandle,
    StencilFaceFlags,
    StencilOp,
    StencilOp,
    StencilOp,
    CompareOp,
);
/// [`vkCmdSetStencilOp`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilOp.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetStencilOp")]
#[inline(always)]
pub unsafe fn cmd_set_stencil_op(
    command_buffer: CommandBufferHandle,
    face_mask: StencilFaceFlags,
    fail_op: StencilOp,
    pass_op: StencilOp,
    depth_fail_op: StencilOp,
    compare_op: CompareOp,
) {
    todo!()
}

pub(crate) type FUN_CmdSetRasterizerDiscardEnable =
    unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetRasterizerDiscardEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizerDiscardEnable.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetRasterizerDiscardEnable")]
#[inline(always)]
pub unsafe fn cmd_set_rasterizer_discard_enable(
    command_buffer: CommandBufferHandle,
    rasterizer_discard_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetDepthBiasEnable = unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetDepthBiasEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBiasEnable.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetDepthBiasEnable")]
#[inline(always)]
pub unsafe fn cmd_set_depth_bias_enable(
    command_buffer: CommandBufferHandle,
    depth_bias_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetPrimitiveRestartEnable =
    unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetPrimitiveRestartEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartEnable.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetPrimitiveRestartEnable")]
#[inline(always)]
pub unsafe fn cmd_set_primitive_restart_enable(
    command_buffer: CommandBufferHandle,
    primitive_restart_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_MapMemory2 =
    unsafe extern "C" fn(DeviceHandle, *const MemoryMapInfo, *mut *mut c_void) -> ResultCode;
/// [`vkMapMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2.html)
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
#[inline(always)]
pub unsafe fn map_memory_2(
    device: DeviceHandle,
    memory_map_info: *const MemoryMapInfo,
    data: *mut *mut c_void,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_UnmapMemory2 =
    unsafe extern "C" fn(DeviceHandle, *const MemoryUnmapInfo) -> ResultCode;
/// [`vkUnmapMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`MEMORY_MAP_FAILED`](ResultCode::ERROR_MEMORY_MAP_FAILED)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkUnmapMemory2")]
#[inline(always)]
pub unsafe fn unmap_memory_2(
    device: DeviceHandle,
    memory_unmap_info: *const MemoryUnmapInfo,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetDeviceImageSubresourceLayout =
    unsafe extern "C" fn(DeviceHandle, *const DeviceImageSubresourceInfo, *mut SubresourceLayout2);
/// [`vkGetDeviceImageSubresourceLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSubresourceLayout.html)
///
#[doc(alias = "vkGetDeviceImageSubresourceLayout")]
#[inline(always)]
pub unsafe fn get_device_image_subresource_layout(
    device: DeviceHandle,
    info: *const DeviceImageSubresourceInfo,
    layout: *mut SubresourceLayout2,
) {
    todo!()
}

pub(crate) type FUN_GetImageSubresourceLayout2 =
    unsafe extern "C" fn(DeviceHandle, Image, *const ImageSubresource2, *mut SubresourceLayout2);
/// [`vkGetImageSubresourceLayout2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2.html)
///
#[doc(alias = "vkGetImageSubresourceLayout2")]
#[inline(always)]
pub unsafe fn get_image_subresource_layout_2(
    device: DeviceHandle,
    image: Image,
    subresource: *const ImageSubresource2,
    layout: *mut SubresourceLayout2,
) {
    todo!()
}

pub(crate) type FUN_CopyMemoryToImage =
    unsafe extern "C" fn(DeviceHandle, *const CopyMemoryToImageInfo) -> ResultCode;
/// [`vkCopyMemoryToImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToImage.html)
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
#[inline(always)]
pub unsafe fn copy_memory_to_image(
    device: DeviceHandle,
    copy_memory_to_image_info: *const CopyMemoryToImageInfo,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CopyImageToMemory =
    unsafe extern "C" fn(DeviceHandle, *const CopyImageToMemoryInfo) -> ResultCode;
/// [`vkCopyImageToMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToMemory.html)
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
#[inline(always)]
pub unsafe fn copy_image_to_memory(
    device: DeviceHandle,
    copy_image_to_memory_info: *const CopyImageToMemoryInfo,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CopyImageToImage =
    unsafe extern "C" fn(DeviceHandle, *const CopyImageToImageInfo) -> ResultCode;
/// [`vkCopyImageToImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToImage.html)
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
#[inline(always)]
pub unsafe fn copy_image_to_image(
    device: DeviceHandle,
    copy_image_to_image_info: *const CopyImageToImageInfo,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_TransitionImageLayout =
    unsafe extern "C" fn(DeviceHandle, u32, *const HostImageLayoutTransitionInfo) -> ResultCode;
/// [`vkTransitionImageLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTransitionImageLayout.html)
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
#[inline(always)]
pub unsafe fn transition_image_layout(
    device: DeviceHandle,
    transition_count: u32,
    transitions: *const HostImageLayoutTransitionInfo,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdPushDescriptorSet = unsafe extern "C" fn(
    CommandBufferHandle,
    PipelineBindPoint,
    PipelineLayout,
    u32,
    u32,
    *const WriteDescriptorSet,
);
/// [`vkCmdPushDescriptorSet`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdPushDescriptorSet")]
#[inline(always)]
pub unsafe fn cmd_push_descriptor_set(
    command_buffer: CommandBufferHandle,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    set: u32,
    descriptor_write_count: u32,
    descriptor_writes: *const WriteDescriptorSet,
) {
    todo!()
}

pub(crate) type FUN_CmdPushDescriptorSetWithTemplate = unsafe extern "C" fn(
    CommandBufferHandle,
    DescriptorUpdateTemplate,
    PipelineLayout,
    u32,
    *const c_void,
);
/// [`vkCmdPushDescriptorSetWithTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdPushDescriptorSetWithTemplate")]
#[inline(always)]
pub unsafe fn cmd_push_descriptor_set_with_template(
    command_buffer: CommandBufferHandle,
    descriptor_update_template: DescriptorUpdateTemplate,
    layout: PipelineLayout,
    set: u32,
    data: *const c_void,
) {
    todo!()
}

pub(crate) type FUN_CmdBindDescriptorSets2 =
    unsafe extern "C" fn(CommandBufferHandle, *const BindDescriptorSetsInfo);
/// [`vkCmdBindDescriptorSets2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdBindDescriptorSets2")]
#[inline(always)]
pub unsafe fn cmd_bind_descriptor_sets_2(
    command_buffer: CommandBufferHandle,
    bind_descriptor_sets_info: *const BindDescriptorSetsInfo,
) {
    todo!()
}

pub(crate) type FUN_CmdPushConstants2 =
    unsafe extern "C" fn(CommandBufferHandle, *const PushConstantsInfo);
/// [`vkCmdPushConstants2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdPushConstants2")]
#[inline(always)]
pub unsafe fn cmd_push_constants_2(
    command_buffer: CommandBufferHandle,
    push_constants_info: *const PushConstantsInfo,
) {
    todo!()
}

pub(crate) type FUN_CmdPushDescriptorSet2 =
    unsafe extern "C" fn(CommandBufferHandle, *const PushDescriptorSetInfo);
/// [`vkCmdPushDescriptorSet2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdPushDescriptorSet2")]
#[inline(always)]
pub unsafe fn cmd_push_descriptor_set_2(
    command_buffer: CommandBufferHandle,
    push_descriptor_set_info: *const PushDescriptorSetInfo,
) {
    todo!()
}

pub(crate) type FUN_CmdPushDescriptorSetWithTemplate2 =
    unsafe extern "C" fn(CommandBufferHandle, *const PushDescriptorSetWithTemplateInfo);
/// [`vkCmdPushDescriptorSetWithTemplate2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate2.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdPushDescriptorSetWithTemplate2")]
#[inline(always)]
pub unsafe fn cmd_push_descriptor_set_with_template_2(
    command_buffer: CommandBufferHandle,
    push_descriptor_set_with_template_info: *const PushDescriptorSetWithTemplateInfo,
) {
    todo!()
}

pub(crate) type FUN_CmdSetLineStipple = unsafe extern "C" fn(CommandBufferHandle, u32, u16);
/// [`vkCmdSetLineStipple`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStipple.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetLineStipple")]
#[inline(always)]
pub unsafe fn cmd_set_line_stipple(
    command_buffer: CommandBufferHandle,
    line_stipple_factor: u32,
    line_stipple_pattern: u16,
) {
    todo!()
}

pub(crate) type FUN_CmdBindIndexBuffer2 =
    unsafe extern "C" fn(CommandBufferHandle, Buffer, DeviceSize, DeviceSize, IndexType);
/// [`vkCmdBindIndexBuffer2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer2.html)
///
/// # Optional parameters
/// - buffer
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdBindIndexBuffer2")]
#[inline(always)]
pub unsafe fn cmd_bind_index_buffer_2(
    command_buffer: CommandBufferHandle,
    buffer: Buffer,
    offset: DeviceSize,
    size: DeviceSize,
    index_type: IndexType,
) {
    todo!()
}

pub(crate) type FUN_GetRenderingAreaGranularity =
    unsafe extern "C" fn(DeviceHandle, *const RenderingAreaInfo, *mut Extent2D);
/// [`vkGetRenderingAreaGranularity`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderingAreaGranularity.html)
///
#[doc(alias = "vkGetRenderingAreaGranularity")]
#[inline(always)]
pub unsafe fn get_rendering_area_granularity(
    device: DeviceHandle,
    rendering_area_info: *const RenderingAreaInfo,
    granularity: *mut Extent2D,
) {
    todo!()
}

pub(crate) type FUN_CmdSetRenderingAttachmentLocations =
    unsafe extern "C" fn(CommandBufferHandle, *const RenderingAttachmentLocationInfo);
/// [`vkCmdSetRenderingAttachmentLocations`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocations.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetRenderingAttachmentLocations")]
#[inline(always)]
pub unsafe fn cmd_set_rendering_attachment_locations(
    command_buffer: CommandBufferHandle,
    location_info: *const RenderingAttachmentLocationInfo,
) {
    todo!()
}

pub(crate) type FUN_CmdSetRenderingInputAttachmentIndices =
    unsafe extern "C" fn(CommandBufferHandle, *const RenderingInputAttachmentIndexInfo);
/// [`vkCmdSetRenderingInputAttachmentIndices`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndices.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetRenderingInputAttachmentIndices")]
#[inline(always)]
pub unsafe fn cmd_set_rendering_input_attachment_indices(
    command_buffer: CommandBufferHandle,
    input_attachment_index_info: *const RenderingInputAttachmentIndexInfo,
) {
    todo!()
}

pub(crate) type FUN_DestroySurfaceKHR =
    unsafe extern "C" fn(InstanceHandle, SurfaceKHR, *const AllocationCallbacks);
/// [`vkDestroySurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySurfaceKHR.html)
///
/// # Optional parameters
/// - surface
/// - allocator
///
#[doc(alias = "vkDestroySurfaceKHR")]
#[inline(always)]
pub unsafe fn destroy_surface_khr(
    instance: InstanceHandle,
    surface: SurfaceKHR,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceSurfaceSupportKHR =
    unsafe extern "C" fn(PhysicalDeviceHandle, u32, SurfaceKHR, *mut Bool32) -> ResultCode;
/// [`vkGetPhysicalDeviceSurfaceSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceSupportKHR.html)
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
#[inline(always)]
pub unsafe fn get_physical_device_surface_support_khr(
    physical_device: PhysicalDeviceHandle,
    queue_family_index: u32,
    surface: SurfaceKHR,
    supported: *mut Bool32,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceSurfaceCapabilitiesKHR = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    SurfaceKHR,
    *mut SurfaceCapabilitiesKHR,
) -> ResultCode;
/// [`vkGetPhysicalDeviceSurfaceCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html)
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
#[inline(always)]
pub unsafe fn get_physical_device_surface_capabilities_khr(
    physical_device: PhysicalDeviceHandle,
    surface: SurfaceKHR,
    surface_capabilities: *mut SurfaceCapabilitiesKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceSurfaceFormatsKHR = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    SurfaceKHR,
    *mut u32,
    *mut SurfaceFormatKHR,
) -> ResultCode;
/// [`vkGetPhysicalDeviceSurfaceFormatsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceFormatsKHR.html)
///
/// # Optional parameters
/// - surface
/// - surface_formats
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
#[inline(always)]
pub unsafe fn get_physical_device_surface_formats_khr(
    physical_device: PhysicalDeviceHandle,
    surface: SurfaceKHR,
    surface_format_count: *mut u32,
    surface_formats: *mut SurfaceFormatKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceSurfacePresentModesKHR = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    SurfaceKHR,
    *mut u32,
    *mut PresentModeKHR,
) -> ResultCode;
/// [`vkGetPhysicalDeviceSurfacePresentModesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfacePresentModesKHR.html)
///
/// # Optional parameters
/// - surface
/// - present_modes
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
#[inline(always)]
pub unsafe fn get_physical_device_surface_present_modes_khr(
    physical_device: PhysicalDeviceHandle,
    surface: SurfaceKHR,
    present_mode_count: *mut u32,
    present_modes: *mut PresentModeKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateSwapchainKHR = unsafe extern "C" fn(
    DeviceHandle,
    *const SwapchainCreateInfoKHR,
    *const AllocationCallbacks,
    *mut SwapchainKHR,
) -> ResultCode;
/// [`vkCreateSwapchainKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSwapchainKHR.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_swapchain_khr(
    device: DeviceHandle,
    create_info: *const SwapchainCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    swapchain: *mut SwapchainKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroySwapchainKHR =
    unsafe extern "C" fn(DeviceHandle, SwapchainKHR, *const AllocationCallbacks);
/// [`vkDestroySwapchainKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySwapchainKHR.html)
///
/// # Optional parameters
/// - swapchain
/// - allocator
///
#[doc(alias = "vkDestroySwapchainKHR")]
#[inline(always)]
pub unsafe fn destroy_swapchain_khr(
    device: DeviceHandle,
    swapchain: SwapchainKHR,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_GetSwapchainImagesKHR =
    unsafe extern "C" fn(DeviceHandle, SwapchainKHR, *mut u32, *mut Image) -> ResultCode;
/// [`vkGetSwapchainImagesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainImagesKHR.html)
///
/// # Optional parameters
/// - swapchain_images
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
#[inline(always)]
pub unsafe fn get_swapchain_images_khr(
    device: DeviceHandle,
    swapchain: SwapchainKHR,
    swapchain_image_count: *mut u32,
    swapchain_images: *mut Image,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_AcquireNextImageKHR =
    unsafe extern "C" fn(DeviceHandle, SwapchainKHR, u64, Semaphore, Fence, *mut u32) -> ResultCode;
/// [`vkAcquireNextImageKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImageKHR.html)
///
/// # Optional parameters
/// - semaphore
/// - fence
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
#[inline(always)]
pub unsafe fn acquire_next_image_khr(
    device: DeviceHandle,
    swapchain: SwapchainKHR,
    timeout: u64,
    semaphore: Semaphore,
    fence: Fence,
    image_index: *mut u32,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_QueuePresentKHR =
    unsafe extern "C" fn(QueueHandle, *const PresentInfoKHR) -> ResultCode;
/// [`vkQueuePresentKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueuePresentKHR.html)
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
#[inline(always)]
pub unsafe fn queue_present_khr(
    queue: QueueHandle,
    present_info: *const PresentInfoKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetDeviceGroupPresentCapabilitiesKHR =
    unsafe extern "C" fn(DeviceHandle, *mut DeviceGroupPresentCapabilitiesKHR) -> ResultCode;
/// [`vkGetDeviceGroupPresentCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPresentCapabilitiesKHR.html)
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
#[inline(always)]
pub unsafe fn get_device_group_present_capabilities_khr(
    device: DeviceHandle,
    device_group_present_capabilities: *mut DeviceGroupPresentCapabilitiesKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetDeviceGroupSurfacePresentModesKHR = unsafe extern "C" fn(
    DeviceHandle,
    SurfaceKHR,
    *mut DeviceGroupPresentModeFlagsKHR,
) -> ResultCode;
/// [`vkGetDeviceGroupSurfacePresentModesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModesKHR.html)
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
#[inline(always)]
pub unsafe fn get_device_group_surface_present_modes_khr(
    device: DeviceHandle,
    surface: SurfaceKHR,
    modes: *mut DeviceGroupPresentModeFlagsKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDevicePresentRectanglesKHR =
    unsafe extern "C" fn(PhysicalDeviceHandle, SurfaceKHR, *mut u32, *mut Rect2D) -> ResultCode;
/// [`vkGetPhysicalDevicePresentRectanglesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDevicePresentRectanglesKHR.html)
///
/// # Optional parameters
/// - rects
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
#[inline(always)]
pub unsafe fn get_physical_device_present_rectangles_khr(
    physical_device: PhysicalDeviceHandle,
    surface: SurfaceKHR,
    rect_count: *mut u32,
    rects: *mut Rect2D,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_AcquireNextImage2KHR =
    unsafe extern "C" fn(DeviceHandle, *const AcquireNextImageInfoKHR, *mut u32) -> ResultCode;
/// [`vkAcquireNextImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImage2KHR.html)
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
#[inline(always)]
pub unsafe fn acquire_next_image_2_khr(
    device: DeviceHandle,
    acquire_info: *const AcquireNextImageInfoKHR,
    image_index: *mut u32,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceDisplayPropertiesKHR =
    unsafe extern "C" fn(PhysicalDeviceHandle, *mut u32, *mut DisplayPropertiesKHR) -> ResultCode;
/// [`vkGetPhysicalDeviceDisplayPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPropertiesKHR.html)
///
/// # Optional parameters
/// - properties
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
#[inline(always)]
pub unsafe fn get_physical_device_display_properties_khr(
    physical_device: PhysicalDeviceHandle,
    property_count: *mut u32,
    properties: *mut DisplayPropertiesKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceDisplayPlanePropertiesKHR = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *mut u32,
    *mut DisplayPlanePropertiesKHR,
)
    -> ResultCode;
/// [`vkGetPhysicalDeviceDisplayPlanePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html)
///
/// # Optional parameters
/// - properties
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
#[inline(always)]
pub unsafe fn get_physical_device_display_plane_properties_khr(
    physical_device: PhysicalDeviceHandle,
    property_count: *mut u32,
    properties: *mut DisplayPlanePropertiesKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetDisplayPlaneSupportedDisplaysKHR =
    unsafe extern "C" fn(PhysicalDeviceHandle, u32, *mut u32, *mut DisplayKHR) -> ResultCode;
/// [`vkGetDisplayPlaneSupportedDisplaysKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneSupportedDisplaysKHR.html)
///
/// # Optional parameters
/// - displays
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
#[inline(always)]
pub unsafe fn get_display_plane_supported_displays_khr(
    physical_device: PhysicalDeviceHandle,
    plane_index: u32,
    display_count: *mut u32,
    displays: *mut DisplayKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetDisplayModePropertiesKHR = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    DisplayKHR,
    *mut u32,
    *mut DisplayModePropertiesKHR,
) -> ResultCode;
/// [`vkGetDisplayModePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayModePropertiesKHR.html)
///
/// # Optional parameters
/// - properties
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
#[inline(always)]
pub unsafe fn get_display_mode_properties_khr(
    physical_device: PhysicalDeviceHandle,
    display: DisplayKHR,
    property_count: *mut u32,
    properties: *mut DisplayModePropertiesKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateDisplayModeKHR = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    DisplayKHR,
    *const DisplayModeCreateInfoKHR,
    *const AllocationCallbacks,
    *mut DisplayModeKHR,
) -> ResultCode;
/// [`vkCreateDisplayModeKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayModeKHR.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_display_mode_khr(
    physical_device: PhysicalDeviceHandle,
    display: DisplayKHR,
    create_info: *const DisplayModeCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    mode: *mut DisplayModeKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetDisplayPlaneCapabilitiesKHR = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    DisplayModeKHR,
    u32,
    *mut DisplayPlaneCapabilitiesKHR,
) -> ResultCode;
/// [`vkGetDisplayPlaneCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneCapabilitiesKHR.html)
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
#[inline(always)]
pub unsafe fn get_display_plane_capabilities_khr(
    physical_device: PhysicalDeviceHandle,
    mode: DisplayModeKHR,
    plane_index: u32,
    capabilities: *mut DisplayPlaneCapabilitiesKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateDisplayPlaneSurfaceKHR = unsafe extern "C" fn(
    InstanceHandle,
    *const DisplaySurfaceCreateInfoKHR,
    *const AllocationCallbacks,
    *mut SurfaceKHR,
) -> ResultCode;
/// [`vkCreateDisplayPlaneSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayPlaneSurfaceKHR.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_display_plane_surface_khr(
    instance: InstanceHandle,
    create_info: *const DisplaySurfaceCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateSharedSwapchainsKHR = unsafe extern "C" fn(
    DeviceHandle,
    u32,
    *const SwapchainCreateInfoKHR,
    *const AllocationCallbacks,
    *mut SwapchainKHR,
) -> ResultCode;
/// [`vkCreateSharedSwapchainsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSharedSwapchainsKHR.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_shared_swapchains_khr(
    device: DeviceHandle,
    swapchain_count: u32,
    create_infos: *const SwapchainCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    swapchains: *mut SwapchainKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateXlibSurfaceKHR = unsafe extern "C" fn(
    InstanceHandle,
    *const XlibSurfaceCreateInfoKHR,
    *const AllocationCallbacks,
    *mut SurfaceKHR,
) -> ResultCode;
/// [`vkCreateXlibSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXlibSurfaceKHR.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_xlib_surface_khr(
    instance: InstanceHandle,
    create_info: *const XlibSurfaceCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceXlibPresentationSupportKHR =
    unsafe extern "C" fn(PhysicalDeviceHandle, u32, *mut Display, VisualID) -> Bool32;
/// [`vkGetPhysicalDeviceXlibPresentationSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceXlibPresentationSupportKHR.html)
///
#[doc(alias = "vkGetPhysicalDeviceXlibPresentationSupportKHR")]
#[inline(always)]
pub unsafe fn get_physical_device_xlib_presentation_support_khr(
    physical_device: PhysicalDeviceHandle,
    queue_family_index: u32,
    dpy: *mut Display,
    visual_id: VisualID,
) -> Bool32 {
    todo!()
}

pub(crate) type FUN_CreateXcbSurfaceKHR = unsafe extern "C" fn(
    InstanceHandle,
    *const XcbSurfaceCreateInfoKHR,
    *const AllocationCallbacks,
    *mut SurfaceKHR,
) -> ResultCode;
/// [`vkCreateXcbSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXcbSurfaceKHR.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_xcb_surface_khr(
    instance: InstanceHandle,
    create_info: *const XcbSurfaceCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceXcbPresentationSupportKHR = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    u32,
    *mut xcb_connection_t,
    xcb_visualid_t,
) -> Bool32;
/// [`vkGetPhysicalDeviceXcbPresentationSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceXcbPresentationSupportKHR.html)
///
#[doc(alias = "vkGetPhysicalDeviceXcbPresentationSupportKHR")]
#[inline(always)]
pub unsafe fn get_physical_device_xcb_presentation_support_khr(
    physical_device: PhysicalDeviceHandle,
    queue_family_index: u32,
    connection: *mut xcb_connection_t,
    visual_id: xcb_visualid_t,
) -> Bool32 {
    todo!()
}

pub(crate) type FUN_CreateWaylandSurfaceKHR = unsafe extern "C" fn(
    InstanceHandle,
    *const WaylandSurfaceCreateInfoKHR,
    *const AllocationCallbacks,
    *mut SurfaceKHR,
) -> ResultCode;
/// [`vkCreateWaylandSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWaylandSurfaceKHR.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_wayland_surface_khr(
    instance: InstanceHandle,
    create_info: *const WaylandSurfaceCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceWaylandPresentationSupportKHR =
    unsafe extern "C" fn(PhysicalDeviceHandle, u32, *mut wl_display) -> Bool32;
/// [`vkGetPhysicalDeviceWaylandPresentationSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html)
///
#[doc(alias = "vkGetPhysicalDeviceWaylandPresentationSupportKHR")]
#[inline(always)]
pub unsafe fn get_physical_device_wayland_presentation_support_khr(
    physical_device: PhysicalDeviceHandle,
    queue_family_index: u32,
    display: *mut wl_display,
) -> Bool32 {
    todo!()
}

pub(crate) type FUN_CreateAndroidSurfaceKHR = unsafe extern "C" fn(
    InstanceHandle,
    *const AndroidSurfaceCreateInfoKHR,
    *const AllocationCallbacks,
    *mut SurfaceKHR,
) -> ResultCode;
/// [`vkCreateAndroidSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAndroidSurfaceKHR.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_android_surface_khr(
    instance: InstanceHandle,
    create_info: *const AndroidSurfaceCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateWin32SurfaceKHR = unsafe extern "C" fn(
    InstanceHandle,
    *const Win32SurfaceCreateInfoKHR,
    *const AllocationCallbacks,
    *mut SurfaceKHR,
) -> ResultCode;
/// [`vkCreateWin32SurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWin32SurfaceKHR.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_win_32_surface_khr(
    instance: InstanceHandle,
    create_info: *const Win32SurfaceCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceWin32PresentationSupportKHR =
    unsafe extern "C" fn(PhysicalDeviceHandle, u32) -> Bool32;
/// [`vkGetPhysicalDeviceWin32PresentationSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceWin32PresentationSupportKHR.html)
///
#[doc(alias = "vkGetPhysicalDeviceWin32PresentationSupportKHR")]
#[inline(always)]
pub unsafe fn get_physical_device_win_32_presentation_support_khr(
    physical_device: PhysicalDeviceHandle,
    queue_family_index: u32,
) -> Bool32 {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceVideoCapabilitiesKHR = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *const VideoProfileInfoKHR,
    *mut VideoCapabilitiesKHR,
) -> ResultCode;
/// [`vkGetPhysicalDeviceVideoCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoCapabilitiesKHR.html)
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
#[inline(always)]
pub unsafe fn get_physical_device_video_capabilities_khr(
    physical_device: PhysicalDeviceHandle,
    video_profile: *const VideoProfileInfoKHR,
    capabilities: *mut VideoCapabilitiesKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceVideoFormatPropertiesKHR = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *const PhysicalDeviceVideoFormatInfoKHR,
    *mut u32,
    *mut VideoFormatPropertiesKHR,
) -> ResultCode;
/// [`vkGetPhysicalDeviceVideoFormatPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html)
///
/// # Optional parameters
/// - video_format_properties
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
#[inline(always)]
pub unsafe fn get_physical_device_video_format_properties_khr(
    physical_device: PhysicalDeviceHandle,
    video_format_info: *const PhysicalDeviceVideoFormatInfoKHR,
    video_format_property_count: *mut u32,
    video_format_properties: *mut VideoFormatPropertiesKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateVideoSessionKHR = unsafe extern "C" fn(
    DeviceHandle,
    *const VideoSessionCreateInfoKHR,
    *const AllocationCallbacks,
    *mut VideoSessionKHR,
) -> ResultCode;
/// [`vkCreateVideoSessionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateVideoSessionKHR.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_video_session_khr(
    device: DeviceHandle,
    create_info: *const VideoSessionCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    video_session: *mut VideoSessionKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyVideoSessionKHR =
    unsafe extern "C" fn(DeviceHandle, VideoSessionKHR, *const AllocationCallbacks);
/// [`vkDestroyVideoSessionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyVideoSessionKHR.html)
///
/// # Optional parameters
/// - video_session
/// - allocator
///
#[doc(alias = "vkDestroyVideoSessionKHR")]
#[inline(always)]
pub unsafe fn destroy_video_session_khr(
    device: DeviceHandle,
    video_session: VideoSessionKHR,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_GetVideoSessionMemoryRequirementsKHR = unsafe extern "C" fn(
    DeviceHandle,
    VideoSessionKHR,
    *mut u32,
    *mut VideoSessionMemoryRequirementsKHR,
) -> ResultCode;
/// [`vkGetVideoSessionMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetVideoSessionMemoryRequirementsKHR.html)
///
/// # Optional parameters
/// - memory_requirements
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
/// ## Error
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkGetVideoSessionMemoryRequirementsKHR")]
#[inline(always)]
pub unsafe fn get_video_session_memory_requirements_khr(
    device: DeviceHandle,
    video_session: VideoSessionKHR,
    memory_requirements_count: *mut u32,
    memory_requirements: *mut VideoSessionMemoryRequirementsKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_BindVideoSessionMemoryKHR = unsafe extern "C" fn(
    DeviceHandle,
    VideoSessionKHR,
    u32,
    *const BindVideoSessionMemoryInfoKHR,
) -> ResultCode;
/// [`vkBindVideoSessionMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindVideoSessionMemoryKHR.html)
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
#[inline(always)]
pub unsafe fn bind_video_session_memory_khr(
    device: DeviceHandle,
    video_session: VideoSessionKHR,
    bind_session_memory_info_count: u32,
    bind_session_memory_infos: *const BindVideoSessionMemoryInfoKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateVideoSessionParametersKHR = unsafe extern "C" fn(
    DeviceHandle,
    *const VideoSessionParametersCreateInfoKHR,
    *const AllocationCallbacks,
    *mut VideoSessionParametersKHR,
) -> ResultCode;
/// [`vkCreateVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateVideoSessionParametersKHR.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_video_session_parameters_khr(
    device: DeviceHandle,
    create_info: *const VideoSessionParametersCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    video_session_parameters: *mut VideoSessionParametersKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_UpdateVideoSessionParametersKHR = unsafe extern "C" fn(
    DeviceHandle,
    VideoSessionParametersKHR,
    *const VideoSessionParametersUpdateInfoKHR,
) -> ResultCode;
/// [`vkUpdateVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateVideoSessionParametersKHR.html)
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
#[inline(always)]
pub unsafe fn update_video_session_parameters_khr(
    device: DeviceHandle,
    video_session_parameters: VideoSessionParametersKHR,
    update_info: *const VideoSessionParametersUpdateInfoKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyVideoSessionParametersKHR =
    unsafe extern "C" fn(DeviceHandle, VideoSessionParametersKHR, *const AllocationCallbacks);
/// [`vkDestroyVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyVideoSessionParametersKHR.html)
///
/// # Optional parameters
/// - video_session_parameters
/// - allocator
///
#[doc(alias = "vkDestroyVideoSessionParametersKHR")]
#[inline(always)]
pub unsafe fn destroy_video_session_parameters_khr(
    device: DeviceHandle,
    video_session_parameters: VideoSessionParametersKHR,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_CmdBeginVideoCodingKHR =
    unsafe extern "C" fn(CommandBufferHandle, *const VideoBeginCodingInfoKHR);
/// [`vkCmdBeginVideoCodingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginVideoCodingKHR.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
///
/// # Allowed command buffers
/// - Primary
///
/// # Allowed queues
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
///
#[doc(alias = "vkCmdBeginVideoCodingKHR")]
#[inline(always)]
pub unsafe fn cmd_begin_video_coding_khr(
    command_buffer: CommandBufferHandle,
    begin_info: *const VideoBeginCodingInfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdEndVideoCodingKHR =
    unsafe extern "C" fn(CommandBufferHandle, *const VideoEndCodingInfoKHR);
/// [`vkCmdEndVideoCodingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndVideoCodingKHR.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
///
/// # Allowed command buffers
/// - Primary
///
/// # Allowed queues
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
///
#[doc(alias = "vkCmdEndVideoCodingKHR")]
#[inline(always)]
pub unsafe fn cmd_end_video_coding_khr(
    command_buffer: CommandBufferHandle,
    end_coding_info: *const VideoEndCodingInfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdControlVideoCodingKHR =
    unsafe extern "C" fn(CommandBufferHandle, *const VideoCodingControlInfoKHR);
/// [`vkCmdControlVideoCodingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdControlVideoCodingKHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
///
/// # Allowed queues
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
///
#[doc(alias = "vkCmdControlVideoCodingKHR")]
#[inline(always)]
pub unsafe fn cmd_control_video_coding_khr(
    command_buffer: CommandBufferHandle,
    coding_control_info: *const VideoCodingControlInfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdDecodeVideoKHR =
    unsafe extern "C" fn(CommandBufferHandle, *const VideoDecodeInfoKHR);
/// [`vkCmdDecodeVideoKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecodeVideoKHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
///
/// # Allowed queues
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
///
#[doc(alias = "vkCmdDecodeVideoKHR")]
#[inline(always)]
pub unsafe fn cmd_decode_video_khr(
    command_buffer: CommandBufferHandle,
    decode_info: *const VideoDecodeInfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdBeginRenderingKHR =
    unsafe extern "C" fn(CommandBufferHandle, *const RenderingInfo);
/// [`vkCmdBeginRenderingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderingKHR.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdBeginRenderingKHR")]
#[inline(always)]
pub unsafe fn cmd_begin_rendering_khr(
    command_buffer: CommandBufferHandle,
    rendering_info: *const RenderingInfo,
) {
    todo!()
}

pub(crate) type FUN_CmdEndRenderingKHR = unsafe extern "C" fn(CommandBufferHandle);
/// [`vkCmdEndRenderingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderingKHR.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdEndRenderingKHR")]
#[inline(always)]
pub unsafe fn cmd_end_rendering_khr(command_buffer: CommandBufferHandle) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceFeatures2KHR =
    unsafe extern "C" fn(PhysicalDeviceHandle, *mut PhysicalDeviceFeatures2);
/// [`vkGetPhysicalDeviceFeatures2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures2KHR.html)
///
#[doc(alias = "vkGetPhysicalDeviceFeatures2KHR")]
#[inline(always)]
pub unsafe fn get_physical_device_features_2_khr(
    physical_device: PhysicalDeviceHandle,
    features: *mut PhysicalDeviceFeatures2,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceProperties2KHR =
    unsafe extern "C" fn(PhysicalDeviceHandle, *mut PhysicalDeviceProperties2);
/// [`vkGetPhysicalDeviceProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties2KHR.html)
///
#[doc(alias = "vkGetPhysicalDeviceProperties2KHR")]
#[inline(always)]
pub unsafe fn get_physical_device_properties_2_khr(
    physical_device: PhysicalDeviceHandle,
    properties: *mut PhysicalDeviceProperties2,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceFormatProperties2KHR =
    unsafe extern "C" fn(PhysicalDeviceHandle, Format, *mut FormatProperties2);
/// [`vkGetPhysicalDeviceFormatProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties2KHR.html)
///
#[doc(alias = "vkGetPhysicalDeviceFormatProperties2KHR")]
#[inline(always)]
pub unsafe fn get_physical_device_format_properties_2_khr(
    physical_device: PhysicalDeviceHandle,
    format: Format,
    format_properties: *mut FormatProperties2,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceImageFormatProperties2KHR = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *const PhysicalDeviceImageFormatInfo2,
    *mut ImageFormatProperties2,
)
    -> ResultCode;
/// [`vkGetPhysicalDeviceImageFormatProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties2KHR.html)
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
#[inline(always)]
pub unsafe fn get_physical_device_image_format_properties_2_khr(
    physical_device: PhysicalDeviceHandle,
    image_format_info: *const PhysicalDeviceImageFormatInfo2,
    image_format_properties: *mut ImageFormatProperties2,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceQueueFamilyProperties2KHR =
    unsafe extern "C" fn(PhysicalDeviceHandle, *mut u32, *mut QueueFamilyProperties2);
/// [`vkGetPhysicalDeviceQueueFamilyProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2KHR.html)
///
/// # Optional parameters
/// - queue_family_properties
///
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2KHR")]
#[inline(always)]
pub unsafe fn get_physical_device_queue_family_properties_2_khr(
    physical_device: PhysicalDeviceHandle,
    queue_family_property_count: *mut u32,
    queue_family_properties: *mut QueueFamilyProperties2,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceMemoryProperties2KHR =
    unsafe extern "C" fn(PhysicalDeviceHandle, *mut PhysicalDeviceMemoryProperties2);
/// [`vkGetPhysicalDeviceMemoryProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties2KHR.html)
///
#[doc(alias = "vkGetPhysicalDeviceMemoryProperties2KHR")]
#[inline(always)]
pub unsafe fn get_physical_device_memory_properties_2_khr(
    physical_device: PhysicalDeviceHandle,
    memory_properties: *mut PhysicalDeviceMemoryProperties2,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceSparseImageFormatProperties2KHR = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *const PhysicalDeviceSparseImageFormatInfo2,
    *mut u32,
    *mut SparseImageFormatProperties2,
);
/// [`vkGetPhysicalDeviceSparseImageFormatProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2KHR.html)
///
/// # Optional parameters
/// - properties
///
#[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2KHR")]
#[inline(always)]
pub unsafe fn get_physical_device_sparse_image_format_properties_2_khr(
    physical_device: PhysicalDeviceHandle,
    format_info: *const PhysicalDeviceSparseImageFormatInfo2,
    property_count: *mut u32,
    properties: *mut SparseImageFormatProperties2,
) {
    todo!()
}

pub(crate) type FUN_GetDeviceGroupPeerMemoryFeaturesKHR =
    unsafe extern "C" fn(DeviceHandle, u32, u32, u32, *mut PeerMemoryFeatureFlags);
/// [`vkGetDeviceGroupPeerMemoryFeaturesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPeerMemoryFeaturesKHR.html)
///
#[doc(alias = "vkGetDeviceGroupPeerMemoryFeaturesKHR")]
#[inline(always)]
pub unsafe fn get_device_group_peer_memory_features_khr(
    device: DeviceHandle,
    heap_index: u32,
    local_device_index: u32,
    remote_device_index: u32,
    peer_memory_features: *mut PeerMemoryFeatureFlags,
) {
    todo!()
}

pub(crate) type FUN_CmdSetDeviceMaskKHR = unsafe extern "C" fn(CommandBufferHandle, u32);
/// [`vkCmdSetDeviceMaskKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDeviceMaskKHR.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`TRANSFER`](QueueFlags::TRANSFER)
///
#[doc(alias = "vkCmdSetDeviceMaskKHR")]
#[inline(always)]
pub unsafe fn cmd_set_device_mask_khr(command_buffer: CommandBufferHandle, device_mask: u32) {
    todo!()
}

pub(crate) type FUN_CmdDispatchBaseKHR =
    unsafe extern "C" fn(CommandBufferHandle, u32, u32, u32, u32, u32, u32);
/// [`vkCmdDispatchBaseKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchBaseKHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdDispatchBaseKHR")]
#[inline(always)]
pub unsafe fn cmd_dispatch_base_khr(
    command_buffer: CommandBufferHandle,
    base_group_x: u32,
    base_group_y: u32,
    base_group_z: u32,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
) {
    todo!()
}

pub(crate) type FUN_TrimCommandPoolKHR =
    unsafe extern "C" fn(DeviceHandle, CommandPool, CommandPoolTrimFlags);
/// [`vkTrimCommandPoolKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTrimCommandPoolKHR.html)
///
/// # Optional parameters
/// - flags
///
#[doc(alias = "vkTrimCommandPoolKHR")]
#[inline(always)]
pub unsafe fn trim_command_pool_khr(
    device: DeviceHandle,
    command_pool: CommandPool,
    flags: CommandPoolTrimFlags,
) {
    todo!()
}

pub(crate) type FUN_EnumeratePhysicalDeviceGroupsKHR = unsafe extern "C" fn(
    InstanceHandle,
    *mut u32,
    *mut PhysicalDeviceGroupProperties,
) -> ResultCode;
/// [`vkEnumeratePhysicalDeviceGroupsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroupsKHR.html)
///
/// # Optional parameters
/// - physical_device_group_properties
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
#[inline(always)]
pub unsafe fn enumerate_physical_device_groups_khr(
    instance: InstanceHandle,
    physical_device_group_count: *mut u32,
    physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceExternalBufferPropertiesKHR = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *const PhysicalDeviceExternalBufferInfo,
    *mut ExternalBufferProperties,
);
/// [`vkGetPhysicalDeviceExternalBufferPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalBufferPropertiesKHR.html)
///
#[doc(alias = "vkGetPhysicalDeviceExternalBufferPropertiesKHR")]
#[inline(always)]
pub unsafe fn get_physical_device_external_buffer_properties_khr(
    physical_device: PhysicalDeviceHandle,
    external_buffer_info: *const PhysicalDeviceExternalBufferInfo,
    external_buffer_properties: *mut ExternalBufferProperties,
) {
    todo!()
}

pub(crate) type FUN_GetMemoryWin32HandleKHR = unsafe extern "C" fn(
    DeviceHandle,
    *const MemoryGetWin32HandleInfoKHR,
    *mut HANDLE,
) -> ResultCode;
/// [`vkGetMemoryWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleKHR.html)
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
#[inline(always)]
pub unsafe fn get_memory_win_32_handle_khr(
    device: DeviceHandle,
    get_win_32_handle_info: *const MemoryGetWin32HandleInfoKHR,
    handle: *mut HANDLE,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetMemoryWin32HandlePropertiesKHR = unsafe extern "C" fn(
    DeviceHandle,
    ExternalMemoryHandleTypeFlags,
    HANDLE,
    *mut MemoryWin32HandlePropertiesKHR,
) -> ResultCode;
/// [`vkGetMemoryWin32HandlePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandlePropertiesKHR.html)
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
#[inline(always)]
pub unsafe fn get_memory_win_32_handle_properties_khr(
    device: DeviceHandle,
    handle_type: ExternalMemoryHandleTypeFlags,
    handle: HANDLE,
    memory_win_32_handle_properties: *mut MemoryWin32HandlePropertiesKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetMemoryFdKHR =
    unsafe extern "C" fn(DeviceHandle, *const MemoryGetFdInfoKHR, *mut c_int) -> ResultCode;
/// [`vkGetMemoryFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryFdKHR.html)
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
#[inline(always)]
pub unsafe fn get_memory_fd_khr(
    device: DeviceHandle,
    get_fd_info: *const MemoryGetFdInfoKHR,
    fd: *mut c_int,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetMemoryFdPropertiesKHR = unsafe extern "C" fn(
    DeviceHandle,
    ExternalMemoryHandleTypeFlags,
    c_int,
    *mut MemoryFdPropertiesKHR,
) -> ResultCode;
/// [`vkGetMemoryFdPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryFdPropertiesKHR.html)
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
#[inline(always)]
pub unsafe fn get_memory_fd_properties_khr(
    device: DeviceHandle,
    handle_type: ExternalMemoryHandleTypeFlags,
    fd: c_int,
    memory_fd_properties: *mut MemoryFdPropertiesKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceExternalSemaphorePropertiesKHR = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *const PhysicalDeviceExternalSemaphoreInfo,
    *mut ExternalSemaphoreProperties,
);
/// [`vkGetPhysicalDeviceExternalSemaphorePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalSemaphorePropertiesKHR.html)
///
#[doc(alias = "vkGetPhysicalDeviceExternalSemaphorePropertiesKHR")]
#[inline(always)]
pub unsafe fn get_physical_device_external_semaphore_properties_khr(
    physical_device: PhysicalDeviceHandle,
    external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo,
    external_semaphore_properties: *mut ExternalSemaphoreProperties,
) {
    todo!()
}

pub(crate) type FUN_ImportSemaphoreWin32HandleKHR =
    unsafe extern "C" fn(DeviceHandle, *const ImportSemaphoreWin32HandleInfoKHR) -> ResultCode;
/// [`vkImportSemaphoreWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreWin32HandleKHR.html)
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
#[inline(always)]
pub unsafe fn import_semaphore_win_32_handle_khr(
    device: DeviceHandle,
    import_semaphore_win_32_handle_info: *const ImportSemaphoreWin32HandleInfoKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetSemaphoreWin32HandleKHR = unsafe extern "C" fn(
    DeviceHandle,
    *const SemaphoreGetWin32HandleInfoKHR,
    *mut HANDLE,
) -> ResultCode;
/// [`vkGetSemaphoreWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreWin32HandleKHR.html)
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
#[inline(always)]
pub unsafe fn get_semaphore_win_32_handle_khr(
    device: DeviceHandle,
    get_win_32_handle_info: *const SemaphoreGetWin32HandleInfoKHR,
    handle: *mut HANDLE,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_ImportSemaphoreFdKHR =
    unsafe extern "C" fn(DeviceHandle, *const ImportSemaphoreFdInfoKHR) -> ResultCode;
/// [`vkImportSemaphoreFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreFdKHR.html)
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
#[inline(always)]
pub unsafe fn import_semaphore_fd_khr(
    device: DeviceHandle,
    import_semaphore_fd_info: *const ImportSemaphoreFdInfoKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetSemaphoreFdKHR =
    unsafe extern "C" fn(DeviceHandle, *const SemaphoreGetFdInfoKHR, *mut c_int) -> ResultCode;
/// [`vkGetSemaphoreFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreFdKHR.html)
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
#[inline(always)]
pub unsafe fn get_semaphore_fd_khr(
    device: DeviceHandle,
    get_fd_info: *const SemaphoreGetFdInfoKHR,
    fd: *mut c_int,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdPushDescriptorSetKHR = unsafe extern "C" fn(
    CommandBufferHandle,
    PipelineBindPoint,
    PipelineLayout,
    u32,
    u32,
    *const WriteDescriptorSet,
);
/// [`vkCmdPushDescriptorSetKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetKHR.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdPushDescriptorSetKHR")]
#[inline(always)]
pub unsafe fn cmd_push_descriptor_set_khr(
    command_buffer: CommandBufferHandle,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    set: u32,
    descriptor_write_count: u32,
    descriptor_writes: *const WriteDescriptorSet,
) {
    todo!()
}

pub(crate) type FUN_CmdPushDescriptorSetWithTemplateKHR = unsafe extern "C" fn(
    CommandBufferHandle,
    DescriptorUpdateTemplate,
    PipelineLayout,
    u32,
    *const c_void,
);
/// [`vkCmdPushDescriptorSetWithTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplateKHR.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdPushDescriptorSetWithTemplateKHR")]
#[inline(always)]
pub unsafe fn cmd_push_descriptor_set_with_template_khr(
    command_buffer: CommandBufferHandle,
    descriptor_update_template: DescriptorUpdateTemplate,
    layout: PipelineLayout,
    set: u32,
    data: *const c_void,
) {
    todo!()
}

pub(crate) type FUN_CreateDescriptorUpdateTemplateKHR = unsafe extern "C" fn(
    DeviceHandle,
    *const DescriptorUpdateTemplateCreateInfo,
    *const AllocationCallbacks,
    *mut DescriptorUpdateTemplate,
) -> ResultCode;
/// [`vkCreateDescriptorUpdateTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorUpdateTemplateKHR.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_descriptor_update_template_khr(
    device: DeviceHandle,
    create_info: *const DescriptorUpdateTemplateCreateInfo,
    allocator: *const AllocationCallbacks,
    descriptor_update_template: *mut DescriptorUpdateTemplate,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyDescriptorUpdateTemplateKHR =
    unsafe extern "C" fn(DeviceHandle, DescriptorUpdateTemplate, *const AllocationCallbacks);
/// [`vkDestroyDescriptorUpdateTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplateKHR.html)
///
/// # Optional parameters
/// - descriptor_update_template
/// - allocator
///
#[doc(alias = "vkDestroyDescriptorUpdateTemplateKHR")]
#[inline(always)]
pub unsafe fn destroy_descriptor_update_template_khr(
    device: DeviceHandle,
    descriptor_update_template: DescriptorUpdateTemplate,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_UpdateDescriptorSetWithTemplateKHR =
    unsafe extern "C" fn(DeviceHandle, DescriptorSet, DescriptorUpdateTemplate, *const c_void);
/// [`vkUpdateDescriptorSetWithTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplateKHR.html)
///
#[doc(alias = "vkUpdateDescriptorSetWithTemplateKHR")]
#[inline(always)]
pub unsafe fn update_descriptor_set_with_template_khr(
    device: DeviceHandle,
    descriptor_set: DescriptorSet,
    descriptor_update_template: DescriptorUpdateTemplate,
    data: *const c_void,
) {
    todo!()
}

pub(crate) type FUN_CreateRenderPass2KHR = unsafe extern "C" fn(
    DeviceHandle,
    *const RenderPassCreateInfo2,
    *const AllocationCallbacks,
    *mut RenderPass,
) -> ResultCode;
/// [`vkCreateRenderPass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass2KHR.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_render_pass_2_khr(
    device: DeviceHandle,
    create_info: *const RenderPassCreateInfo2,
    allocator: *const AllocationCallbacks,
    render_pass: *mut RenderPass,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdBeginRenderPass2KHR =
    unsafe extern "C" fn(CommandBufferHandle, *const RenderPassBeginInfo, *const SubpassBeginInfo);
/// [`vkCmdBeginRenderPass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass2KHR.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
/// - `synchronization`
///
/// # Allowed command buffers
/// - Primary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdBeginRenderPass2KHR")]
#[inline(always)]
pub unsafe fn cmd_begin_render_pass_2_khr(
    command_buffer: CommandBufferHandle,
    render_pass_begin: *const RenderPassBeginInfo,
    subpass_begin_info: *const SubpassBeginInfo,
) {
    todo!()
}

pub(crate) type FUN_CmdNextSubpass2KHR =
    unsafe extern "C" fn(CommandBufferHandle, *const SubpassBeginInfo, *const SubpassEndInfo);
/// [`vkCmdNextSubpass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass2KHR.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
/// - `synchronization`
///
/// # Allowed command buffers
/// - Primary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdNextSubpass2KHR")]
#[inline(always)]
pub unsafe fn cmd_next_subpass_2_khr(
    command_buffer: CommandBufferHandle,
    subpass_begin_info: *const SubpassBeginInfo,
    subpass_end_info: *const SubpassEndInfo,
) {
    todo!()
}

pub(crate) type FUN_CmdEndRenderPass2KHR =
    unsafe extern "C" fn(CommandBufferHandle, *const SubpassEndInfo);
/// [`vkCmdEndRenderPass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass2KHR.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
/// - `synchronization`
///
/// # Allowed command buffers
/// - Primary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdEndRenderPass2KHR")]
#[inline(always)]
pub unsafe fn cmd_end_render_pass_2_khr(
    command_buffer: CommandBufferHandle,
    subpass_end_info: *const SubpassEndInfo,
) {
    todo!()
}

pub(crate) type FUN_GetSwapchainStatusKHR =
    unsafe extern "C" fn(DeviceHandle, SwapchainKHR) -> ResultCode;
/// [`vkGetSwapchainStatusKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainStatusKHR.html)
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
#[inline(always)]
pub unsafe fn get_swapchain_status_khr(
    device: DeviceHandle,
    swapchain: SwapchainKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceExternalFencePropertiesKHR = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *const PhysicalDeviceExternalFenceInfo,
    *mut ExternalFenceProperties,
);
/// [`vkGetPhysicalDeviceExternalFencePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalFencePropertiesKHR.html)
///
#[doc(alias = "vkGetPhysicalDeviceExternalFencePropertiesKHR")]
#[inline(always)]
pub unsafe fn get_physical_device_external_fence_properties_khr(
    physical_device: PhysicalDeviceHandle,
    external_fence_info: *const PhysicalDeviceExternalFenceInfo,
    external_fence_properties: *mut ExternalFenceProperties,
) {
    todo!()
}

pub(crate) type FUN_ImportFenceWin32HandleKHR =
    unsafe extern "C" fn(DeviceHandle, *const ImportFenceWin32HandleInfoKHR) -> ResultCode;
/// [`vkImportFenceWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceWin32HandleKHR.html)
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
#[inline(always)]
pub unsafe fn import_fence_win_32_handle_khr(
    device: DeviceHandle,
    import_fence_win_32_handle_info: *const ImportFenceWin32HandleInfoKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetFenceWin32HandleKHR = unsafe extern "C" fn(
    DeviceHandle,
    *const FenceGetWin32HandleInfoKHR,
    *mut HANDLE,
) -> ResultCode;
/// [`vkGetFenceWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceWin32HandleKHR.html)
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
#[inline(always)]
pub unsafe fn get_fence_win_32_handle_khr(
    device: DeviceHandle,
    get_win_32_handle_info: *const FenceGetWin32HandleInfoKHR,
    handle: *mut HANDLE,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_ImportFenceFdKHR =
    unsafe extern "C" fn(DeviceHandle, *const ImportFenceFdInfoKHR) -> ResultCode;
/// [`vkImportFenceFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceFdKHR.html)
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
#[inline(always)]
pub unsafe fn import_fence_fd_khr(
    device: DeviceHandle,
    import_fence_fd_info: *const ImportFenceFdInfoKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetFenceFdKHR =
    unsafe extern "C" fn(DeviceHandle, *const FenceGetFdInfoKHR, *mut c_int) -> ResultCode;
/// [`vkGetFenceFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceFdKHR.html)
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
#[inline(always)]
pub unsafe fn get_fence_fd_khr(
    device: DeviceHandle,
    get_fd_info: *const FenceGetFdInfoKHR,
    fd: *mut c_int,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_EnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR =
    unsafe extern "C" fn(
        PhysicalDeviceHandle,
        u32,
        *mut u32,
        *mut PerformanceCounterKHR,
        *mut PerformanceCounterDescriptionKHR,
    ) -> ResultCode;
/// [`vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html)
///
/// # Optional parameters
/// - counters
/// - counter_descriptions
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
#[inline(always)]
pub unsafe fn enumerate_physical_device_queue_family_performance_query_counters_khr(
    physical_device: PhysicalDeviceHandle,
    queue_family_index: u32,
    counter_count: *mut u32,
    counters: *mut PerformanceCounterKHR,
    counter_descriptions: *mut PerformanceCounterDescriptionKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR =
    unsafe extern "C" fn(PhysicalDeviceHandle, *const QueryPoolPerformanceCreateInfoKHR, *mut u32);
/// [`vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html)
///
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR")]
#[inline(always)]
pub unsafe fn get_physical_device_queue_family_performance_query_passes_khr(
    physical_device: PhysicalDeviceHandle,
    performance_query_create_info: *const QueryPoolPerformanceCreateInfoKHR,
    num_passes: *mut u32,
) {
    todo!()
}

pub(crate) type FUN_AcquireProfilingLockKHR =
    unsafe extern "C" fn(DeviceHandle, *const AcquireProfilingLockInfoKHR) -> ResultCode;
/// [`vkAcquireProfilingLockKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireProfilingLockKHR.html)
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
#[inline(always)]
pub unsafe fn acquire_profiling_lock_khr(
    device: DeviceHandle,
    info: *const AcquireProfilingLockInfoKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_ReleaseProfilingLockKHR = unsafe extern "C" fn(DeviceHandle);
/// [`vkReleaseProfilingLockKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseProfilingLockKHR.html)
///
#[doc(alias = "vkReleaseProfilingLockKHR")]
#[inline(always)]
pub unsafe fn release_profiling_lock_khr(device: DeviceHandle) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceSurfaceCapabilities2KHR = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *const PhysicalDeviceSurfaceInfo2KHR,
    *mut SurfaceCapabilities2KHR,
) -> ResultCode;
/// [`vkGetPhysicalDeviceSurfaceCapabilities2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html)
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
#[inline(always)]
pub unsafe fn get_physical_device_surface_capabilities_2_khr(
    physical_device: PhysicalDeviceHandle,
    surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    surface_capabilities: *mut SurfaceCapabilities2KHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceSurfaceFormats2KHR = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *const PhysicalDeviceSurfaceInfo2KHR,
    *mut u32,
    *mut SurfaceFormat2KHR,
) -> ResultCode;
/// [`vkGetPhysicalDeviceSurfaceFormats2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceFormats2KHR.html)
///
/// # Optional parameters
/// - surface_formats
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
#[inline(always)]
pub unsafe fn get_physical_device_surface_formats_2_khr(
    physical_device: PhysicalDeviceHandle,
    surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    surface_format_count: *mut u32,
    surface_formats: *mut SurfaceFormat2KHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceDisplayProperties2KHR =
    unsafe extern "C" fn(PhysicalDeviceHandle, *mut u32, *mut DisplayProperties2KHR) -> ResultCode;
/// [`vkGetPhysicalDeviceDisplayProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayProperties2KHR.html)
///
/// # Optional parameters
/// - properties
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
#[inline(always)]
pub unsafe fn get_physical_device_display_properties_2_khr(
    physical_device: PhysicalDeviceHandle,
    property_count: *mut u32,
    properties: *mut DisplayProperties2KHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceDisplayPlaneProperties2KHR =
    unsafe extern "C" fn(
        PhysicalDeviceHandle,
        *mut u32,
        *mut DisplayPlaneProperties2KHR,
    ) -> ResultCode;
/// [`vkGetPhysicalDeviceDisplayPlaneProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html)
///
/// # Optional parameters
/// - properties
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
#[inline(always)]
pub unsafe fn get_physical_device_display_plane_properties_2_khr(
    physical_device: PhysicalDeviceHandle,
    property_count: *mut u32,
    properties: *mut DisplayPlaneProperties2KHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetDisplayModeProperties2KHR = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    DisplayKHR,
    *mut u32,
    *mut DisplayModeProperties2KHR,
) -> ResultCode;
/// [`vkGetDisplayModeProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayModeProperties2KHR.html)
///
/// # Optional parameters
/// - properties
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
#[inline(always)]
pub unsafe fn get_display_mode_properties_2_khr(
    physical_device: PhysicalDeviceHandle,
    display: DisplayKHR,
    property_count: *mut u32,
    properties: *mut DisplayModeProperties2KHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetDisplayPlaneCapabilities2KHR = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *const DisplayPlaneInfo2KHR,
    *mut DisplayPlaneCapabilities2KHR,
) -> ResultCode;
/// [`vkGetDisplayPlaneCapabilities2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneCapabilities2KHR.html)
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
#[inline(always)]
pub unsafe fn get_display_plane_capabilities_2_khr(
    physical_device: PhysicalDeviceHandle,
    display_plane_info: *const DisplayPlaneInfo2KHR,
    capabilities: *mut DisplayPlaneCapabilities2KHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetImageMemoryRequirements2KHR = unsafe extern "C" fn(
    DeviceHandle,
    *const ImageMemoryRequirementsInfo2,
    *mut MemoryRequirements2,
);
/// [`vkGetImageMemoryRequirements2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements2KHR.html)
///
#[doc(alias = "vkGetImageMemoryRequirements2KHR")]
#[inline(always)]
pub unsafe fn get_image_memory_requirements_2_khr(
    device: DeviceHandle,
    info: *const ImageMemoryRequirementsInfo2,
    memory_requirements: *mut MemoryRequirements2,
) {
    todo!()
}

pub(crate) type FUN_GetBufferMemoryRequirements2KHR = unsafe extern "C" fn(
    DeviceHandle,
    *const BufferMemoryRequirementsInfo2,
    *mut MemoryRequirements2,
);
/// [`vkGetBufferMemoryRequirements2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements2KHR.html)
///
#[doc(alias = "vkGetBufferMemoryRequirements2KHR")]
#[inline(always)]
pub unsafe fn get_buffer_memory_requirements_2_khr(
    device: DeviceHandle,
    info: *const BufferMemoryRequirementsInfo2,
    memory_requirements: *mut MemoryRequirements2,
) {
    todo!()
}

pub(crate) type FUN_GetImageSparseMemoryRequirements2KHR = unsafe extern "C" fn(
    DeviceHandle,
    *const ImageSparseMemoryRequirementsInfo2,
    *mut u32,
    *mut SparseImageMemoryRequirements2,
);
/// [`vkGetImageSparseMemoryRequirements2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2KHR.html)
///
/// # Optional parameters
/// - sparse_memory_requirements
///
#[doc(alias = "vkGetImageSparseMemoryRequirements2KHR")]
#[inline(always)]
pub unsafe fn get_image_sparse_memory_requirements_2_khr(
    device: DeviceHandle,
    info: *const ImageSparseMemoryRequirementsInfo2,
    sparse_memory_requirement_count: *mut u32,
    sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
) {
    todo!()
}

pub(crate) type FUN_CreateSamplerYcbcrConversionKHR = unsafe extern "C" fn(
    DeviceHandle,
    *const SamplerYcbcrConversionCreateInfo,
    *const AllocationCallbacks,
    *mut SamplerYcbcrConversion,
) -> ResultCode;
/// [`vkCreateSamplerYcbcrConversionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSamplerYcbcrConversionKHR.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_sampler_ycbcr_conversion_khr(
    device: DeviceHandle,
    create_info: *const SamplerYcbcrConversionCreateInfo,
    allocator: *const AllocationCallbacks,
    ycbcr_conversion: *mut SamplerYcbcrConversion,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroySamplerYcbcrConversionKHR =
    unsafe extern "C" fn(DeviceHandle, SamplerYcbcrConversion, *const AllocationCallbacks);
/// [`vkDestroySamplerYcbcrConversionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversionKHR.html)
///
/// # Optional parameters
/// - ycbcr_conversion
/// - allocator
///
#[doc(alias = "vkDestroySamplerYcbcrConversionKHR")]
#[inline(always)]
pub unsafe fn destroy_sampler_ycbcr_conversion_khr(
    device: DeviceHandle,
    ycbcr_conversion: SamplerYcbcrConversion,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_BindBufferMemory2KHR =
    unsafe extern "C" fn(DeviceHandle, u32, *const BindBufferMemoryInfo) -> ResultCode;
/// [`vkBindBufferMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory2KHR.html)
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
#[inline(always)]
pub unsafe fn bind_buffer_memory_2_khr(
    device: DeviceHandle,
    bind_info_count: u32,
    bind_infos: *const BindBufferMemoryInfo,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_BindImageMemory2KHR =
    unsafe extern "C" fn(DeviceHandle, u32, *const BindImageMemoryInfo) -> ResultCode;
/// [`vkBindImageMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory2KHR.html)
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
#[inline(always)]
pub unsafe fn bind_image_memory_2_khr(
    device: DeviceHandle,
    bind_info_count: u32,
    bind_infos: *const BindImageMemoryInfo,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetDescriptorSetLayoutSupportKHR = unsafe extern "C" fn(
    DeviceHandle,
    *const DescriptorSetLayoutCreateInfo,
    *mut DescriptorSetLayoutSupport,
);
/// [`vkGetDescriptorSetLayoutSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSupportKHR.html)
///
#[doc(alias = "vkGetDescriptorSetLayoutSupportKHR")]
#[inline(always)]
pub unsafe fn get_descriptor_set_layout_support_khr(
    device: DeviceHandle,
    create_info: *const DescriptorSetLayoutCreateInfo,
    support: *mut DescriptorSetLayoutSupport,
) {
    todo!()
}

pub(crate) type FUN_CmdDrawIndirectCountKHR =
    unsafe extern "C" fn(CommandBufferHandle, Buffer, DeviceSize, Buffer, DeviceSize, u32, u32);
/// [`vkCmdDrawIndirectCountKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCountKHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawIndirectCountKHR")]
#[inline(always)]
pub unsafe fn cmd_draw_indirect_count_khr(
    command_buffer: CommandBufferHandle,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdDrawIndexedIndirectCountKHR =
    unsafe extern "C" fn(CommandBufferHandle, Buffer, DeviceSize, Buffer, DeviceSize, u32, u32);
/// [`vkCmdDrawIndexedIndirectCountKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCountKHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawIndexedIndirectCountKHR")]
#[inline(always)]
pub unsafe fn cmd_draw_indexed_indirect_count_khr(
    command_buffer: CommandBufferHandle,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
) {
    todo!()
}

pub(crate) type FUN_GetSemaphoreCounterValueKHR =
    unsafe extern "C" fn(DeviceHandle, Semaphore, *mut u64) -> ResultCode;
/// [`vkGetSemaphoreCounterValueKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValueKHR.html)
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
#[inline(always)]
pub unsafe fn get_semaphore_counter_value_khr(
    device: DeviceHandle,
    semaphore: Semaphore,
    value: *mut u64,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_WaitSemaphoresKHR =
    unsafe extern "C" fn(DeviceHandle, *const SemaphoreWaitInfo, u64) -> ResultCode;
/// [`vkWaitSemaphoresKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitSemaphoresKHR.html)
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
#[inline(always)]
pub unsafe fn wait_semaphores_khr(
    device: DeviceHandle,
    wait_info: *const SemaphoreWaitInfo,
    timeout: u64,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_SignalSemaphoreKHR =
    unsafe extern "C" fn(DeviceHandle, *const SemaphoreSignalInfo) -> ResultCode;
/// [`vkSignalSemaphoreKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSignalSemaphoreKHR.html)
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
#[inline(always)]
pub unsafe fn signal_semaphore_khr(
    device: DeviceHandle,
    signal_info: *const SemaphoreSignalInfo,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceFragmentShadingRatesKHR = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *mut u32,
    *mut PhysicalDeviceFragmentShadingRateKHR,
) -> ResultCode;
/// [`vkGetPhysicalDeviceFragmentShadingRatesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFragmentShadingRatesKHR.html)
///
/// # Optional parameters
/// - fragment_shading_rates
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
#[inline(always)]
pub unsafe fn get_physical_device_fragment_shading_rates_khr(
    physical_device: PhysicalDeviceHandle,
    fragment_shading_rate_count: *mut u32,
    fragment_shading_rates: *mut PhysicalDeviceFragmentShadingRateKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdSetFragmentShadingRateKHR = unsafe extern "C" fn(
    CommandBufferHandle,
    *const Extent2D,
    *const [FragmentShadingRateCombinerOpKHR; 2 as usize],
);
/// [`vkCmdSetFragmentShadingRateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFragmentShadingRateKHR.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetFragmentShadingRateKHR")]
#[inline(always)]
pub unsafe fn cmd_set_fragment_shading_rate_khr(
    command_buffer: CommandBufferHandle,
    fragment_size: *const Extent2D,
    combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2 as usize],
) {
    todo!()
}

pub(crate) type FUN_CmdSetRenderingAttachmentLocationsKHR =
    unsafe extern "C" fn(CommandBufferHandle, *const RenderingAttachmentLocationInfo);
/// [`vkCmdSetRenderingAttachmentLocationsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocationsKHR.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetRenderingAttachmentLocationsKHR")]
#[inline(always)]
pub unsafe fn cmd_set_rendering_attachment_locations_khr(
    command_buffer: CommandBufferHandle,
    location_info: *const RenderingAttachmentLocationInfo,
) {
    todo!()
}

pub(crate) type FUN_CmdSetRenderingInputAttachmentIndicesKHR =
    unsafe extern "C" fn(CommandBufferHandle, *const RenderingInputAttachmentIndexInfo);
/// [`vkCmdSetRenderingInputAttachmentIndicesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndicesKHR.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetRenderingInputAttachmentIndicesKHR")]
#[inline(always)]
pub unsafe fn cmd_set_rendering_input_attachment_indices_khr(
    command_buffer: CommandBufferHandle,
    input_attachment_index_info: *const RenderingInputAttachmentIndexInfo,
) {
    todo!()
}

pub(crate) type FUN_WaitForPresentKHR =
    unsafe extern "C" fn(DeviceHandle, SwapchainKHR, u64, u64) -> ResultCode;
/// [`vkWaitForPresentKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresentKHR.html)
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
#[inline(always)]
pub unsafe fn wait_for_present_khr(
    device: DeviceHandle,
    swapchain: SwapchainKHR,
    present_id: u64,
    timeout: u64,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetBufferDeviceAddressKHR =
    unsafe extern "C" fn(DeviceHandle, *const BufferDeviceAddressInfo) -> DeviceAddress;
/// [`vkGetBufferDeviceAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddressKHR.html)
///
#[doc(alias = "vkGetBufferDeviceAddressKHR")]
#[inline(always)]
pub unsafe fn get_buffer_device_address_khr(
    device: DeviceHandle,
    info: *const BufferDeviceAddressInfo,
) -> DeviceAddress {
    todo!()
}

pub(crate) type FUN_GetBufferOpaqueCaptureAddressKHR =
    unsafe extern "C" fn(DeviceHandle, *const BufferDeviceAddressInfo) -> u64;
/// [`vkGetBufferOpaqueCaptureAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureAddressKHR.html)
///
#[doc(alias = "vkGetBufferOpaqueCaptureAddressKHR")]
#[inline(always)]
pub unsafe fn get_buffer_opaque_capture_address_khr(
    device: DeviceHandle,
    info: *const BufferDeviceAddressInfo,
) -> u64 {
    todo!()
}

pub(crate) type FUN_GetDeviceMemoryOpaqueCaptureAddressKHR =
    unsafe extern "C" fn(DeviceHandle, *const DeviceMemoryOpaqueCaptureAddressInfo) -> u64;
/// [`vkGetDeviceMemoryOpaqueCaptureAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryOpaqueCaptureAddressKHR.html)
///
#[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddressKHR")]
#[inline(always)]
pub unsafe fn get_device_memory_opaque_capture_address_khr(
    device: DeviceHandle,
    info: *const DeviceMemoryOpaqueCaptureAddressInfo,
) -> u64 {
    todo!()
}

pub(crate) type FUN_CreateDeferredOperationKHR = unsafe extern "C" fn(
    DeviceHandle,
    *const AllocationCallbacks,
    *mut DeferredOperationKHR,
) -> ResultCode;
/// [`vkCreateDeferredOperationKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDeferredOperationKHR.html)
///
/// # Optional parameters
/// - allocator
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkCreateDeferredOperationKHR")]
#[inline(always)]
pub unsafe fn create_deferred_operation_khr(
    device: DeviceHandle,
    allocator: *const AllocationCallbacks,
    deferred_operation: *mut DeferredOperationKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyDeferredOperationKHR =
    unsafe extern "C" fn(DeviceHandle, DeferredOperationKHR, *const AllocationCallbacks);
/// [`vkDestroyDeferredOperationKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDeferredOperationKHR.html)
///
/// # Optional parameters
/// - operation
/// - allocator
///
#[doc(alias = "vkDestroyDeferredOperationKHR")]
#[inline(always)]
pub unsafe fn destroy_deferred_operation_khr(
    device: DeviceHandle,
    operation: DeferredOperationKHR,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_GetDeferredOperationMaxConcurrencyKHR =
    unsafe extern "C" fn(DeviceHandle, DeferredOperationKHR) -> u32;
/// [`vkGetDeferredOperationMaxConcurrencyKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeferredOperationMaxConcurrencyKHR.html)
///
#[doc(alias = "vkGetDeferredOperationMaxConcurrencyKHR")]
#[inline(always)]
pub unsafe fn get_deferred_operation_max_concurrency_khr(
    device: DeviceHandle,
    operation: DeferredOperationKHR,
) -> u32 {
    todo!()
}

pub(crate) type FUN_GetDeferredOperationResultKHR =
    unsafe extern "C" fn(DeviceHandle, DeferredOperationKHR) -> ResultCode;
/// [`vkGetDeferredOperationResultKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeferredOperationResultKHR.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// - [`NOT_READY`](ResultCode::NOT_READY)
/// ## Error
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkGetDeferredOperationResultKHR")]
#[inline(always)]
pub unsafe fn get_deferred_operation_result_khr(
    device: DeviceHandle,
    operation: DeferredOperationKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DeferredOperationJoinKHR =
    unsafe extern "C" fn(DeviceHandle, DeferredOperationKHR) -> ResultCode;
/// [`vkDeferredOperationJoinKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDeferredOperationJoinKHR.html)
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
#[inline(always)]
pub unsafe fn deferred_operation_join_khr(
    device: DeviceHandle,
    operation: DeferredOperationKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPipelineExecutablePropertiesKHR = unsafe extern "C" fn(
    DeviceHandle,
    *const PipelineInfoKHR,
    *mut u32,
    *mut PipelineExecutablePropertiesKHR,
) -> ResultCode;
/// [`vkGetPipelineExecutablePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutablePropertiesKHR.html)
///
/// # Optional parameters
/// - properties
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
#[inline(always)]
pub unsafe fn get_pipeline_executable_properties_khr(
    device: DeviceHandle,
    pipeline_info: *const PipelineInfoKHR,
    executable_count: *mut u32,
    properties: *mut PipelineExecutablePropertiesKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPipelineExecutableStatisticsKHR = unsafe extern "C" fn(
    DeviceHandle,
    *const PipelineExecutableInfoKHR,
    *mut u32,
    *mut PipelineExecutableStatisticKHR,
) -> ResultCode;
/// [`vkGetPipelineExecutableStatisticsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutableStatisticsKHR.html)
///
/// # Optional parameters
/// - statistics
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
#[inline(always)]
pub unsafe fn get_pipeline_executable_statistics_khr(
    device: DeviceHandle,
    executable_info: *const PipelineExecutableInfoKHR,
    statistic_count: *mut u32,
    statistics: *mut PipelineExecutableStatisticKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPipelineExecutableInternalRepresentationsKHR =
    unsafe extern "C" fn(
        DeviceHandle,
        *const PipelineExecutableInfoKHR,
        *mut u32,
        *mut PipelineExecutableInternalRepresentationKHR,
    ) -> ResultCode;
/// [`vkGetPipelineExecutableInternalRepresentationsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutableInternalRepresentationsKHR.html)
///
/// # Optional parameters
/// - internal_representations
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
#[inline(always)]
pub unsafe fn get_pipeline_executable_internal_representations_khr(
    device: DeviceHandle,
    executable_info: *const PipelineExecutableInfoKHR,
    internal_representation_count: *mut u32,
    internal_representations: *mut PipelineExecutableInternalRepresentationKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_MapMemory2KHR =
    unsafe extern "C" fn(DeviceHandle, *const MemoryMapInfo, *mut *mut c_void) -> ResultCode;
/// [`vkMapMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2KHR.html)
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
#[inline(always)]
pub unsafe fn map_memory_2_khr(
    device: DeviceHandle,
    memory_map_info: *const MemoryMapInfo,
    data: *mut *mut c_void,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_UnmapMemory2KHR =
    unsafe extern "C" fn(DeviceHandle, *const MemoryUnmapInfo) -> ResultCode;
/// [`vkUnmapMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2KHR.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`MEMORY_MAP_FAILED`](ResultCode::ERROR_MEMORY_MAP_FAILED)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkUnmapMemory2KHR")]
#[inline(always)]
pub unsafe fn unmap_memory_2_khr(
    device: DeviceHandle,
    memory_unmap_info: *const MemoryUnmapInfo,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR =
    unsafe extern "C" fn(
        PhysicalDeviceHandle,
        *const PhysicalDeviceVideoEncodeQualityLevelInfoKHR,
        *mut VideoEncodeQualityLevelPropertiesKHR,
    ) -> ResultCode;
/// [`vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR.html)
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
#[inline(always)]
pub unsafe fn get_physical_device_video_encode_quality_level_properties_khr(
    physical_device: PhysicalDeviceHandle,
    quality_level_info: *const PhysicalDeviceVideoEncodeQualityLevelInfoKHR,
    quality_level_properties: *mut VideoEncodeQualityLevelPropertiesKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetEncodedVideoSessionParametersKHR = unsafe extern "C" fn(
    DeviceHandle,
    *const VideoEncodeSessionParametersGetInfoKHR,
    *mut VideoEncodeSessionParametersFeedbackInfoKHR,
    *mut usize,
    *mut c_void,
) -> ResultCode;
/// [`vkGetEncodedVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetEncodedVideoSessionParametersKHR.html)
///
/// # Optional parameters
/// - feedback_info
/// - data
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
#[inline(always)]
pub unsafe fn get_encoded_video_session_parameters_khr(
    device: DeviceHandle,
    video_session_parameters_info: *const VideoEncodeSessionParametersGetInfoKHR,
    feedback_info: *mut VideoEncodeSessionParametersFeedbackInfoKHR,
    data_size: *mut usize,
    data: *mut c_void,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdEncodeVideoKHR =
    unsafe extern "C" fn(CommandBufferHandle, *const VideoEncodeInfoKHR);
/// [`vkCmdEncodeVideoKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEncodeVideoKHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
///
/// # Allowed queues
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
///
#[doc(alias = "vkCmdEncodeVideoKHR")]
#[inline(always)]
pub unsafe fn cmd_encode_video_khr(
    command_buffer: CommandBufferHandle,
    encode_info: *const VideoEncodeInfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdSetEvent2KHR =
    unsafe extern "C" fn(CommandBufferHandle, Event, *const DependencyInfo);
/// [`vkCmdSetEvent2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent2KHR.html)
///
/// # Performed tasks
/// - `synchronization`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
///
#[doc(alias = "vkCmdSetEvent2KHR")]
#[inline(always)]
pub unsafe fn cmd_set_event_2_khr(
    command_buffer: CommandBufferHandle,
    event: Event,
    dependency_info: *const DependencyInfo,
) {
    todo!()
}

pub(crate) type FUN_CmdResetEvent2KHR =
    unsafe extern "C" fn(CommandBufferHandle, Event, PipelineStageFlags2);
/// [`vkCmdResetEvent2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent2KHR.html)
///
/// # Optional parameters
/// - stage_mask
///
/// # Performed tasks
/// - `synchronization`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
///
#[doc(alias = "vkCmdResetEvent2KHR")]
#[inline(always)]
pub unsafe fn cmd_reset_event_2_khr(
    command_buffer: CommandBufferHandle,
    event: Event,
    stage_mask: PipelineStageFlags2,
) {
    todo!()
}

pub(crate) type FUN_CmdWaitEvents2KHR =
    unsafe extern "C" fn(CommandBufferHandle, u32, *const Event, *const DependencyInfo);
/// [`vkCmdWaitEvents2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents2KHR.html)
///
/// # Performed tasks
/// - `synchronization`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
///
#[doc(alias = "vkCmdWaitEvents2KHR")]
#[inline(always)]
pub unsafe fn cmd_wait_events_2_khr(
    command_buffer: CommandBufferHandle,
    event_count: u32,
    events: *const Event,
    dependency_infos: *const DependencyInfo,
) {
    todo!()
}

pub(crate) type FUN_CmdPipelineBarrier2KHR =
    unsafe extern "C" fn(CommandBufferHandle, *const DependencyInfo);
/// [`vkCmdPipelineBarrier2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier2KHR.html)
///
/// # Performed tasks
/// - `synchronization`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
///
#[doc(alias = "vkCmdPipelineBarrier2KHR")]
#[inline(always)]
pub unsafe fn cmd_pipeline_barrier_2_khr(
    command_buffer: CommandBufferHandle,
    dependency_info: *const DependencyInfo,
) {
    todo!()
}

pub(crate) type FUN_CmdWriteTimestamp2KHR =
    unsafe extern "C" fn(CommandBufferHandle, PipelineStageFlags2, QueryPool, u32);
/// [`vkCmdWriteTimestamp2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp2KHR.html)
///
/// # Optional parameters
/// - stage
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
///
#[doc(alias = "vkCmdWriteTimestamp2KHR")]
#[inline(always)]
pub unsafe fn cmd_write_timestamp_2_khr(
    command_buffer: CommandBufferHandle,
    stage: PipelineStageFlags2,
    query_pool: QueryPool,
    query: u32,
) {
    todo!()
}

pub(crate) type FUN_QueueSubmit2KHR =
    unsafe extern "C" fn(QueueHandle, u32, *const SubmitInfo2, Fence) -> ResultCode;
/// [`vkQueueSubmit2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2KHR.html)
///
/// # Optional parameters
/// - submit_count
/// - fence
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
#[inline(always)]
pub unsafe fn queue_submit_2_khr(
    queue: QueueHandle,
    submit_count: u32,
    submits: *const SubmitInfo2,
    fence: Fence,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdBindIndexBuffer3KHR =
    unsafe extern "C" fn(CommandBufferHandle, *const BindIndexBuffer3InfoKHR);
/// [`vkCmdBindIndexBuffer3KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer3KHR.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdBindIndexBuffer3KHR")]
#[inline(always)]
pub unsafe fn cmd_bind_index_buffer_3_khr(
    command_buffer: CommandBufferHandle,
    info: *const BindIndexBuffer3InfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdBindVertexBuffers3KHR =
    unsafe extern "C" fn(CommandBufferHandle, u32, u32, *const BindVertexBuffer3InfoKHR);
/// [`vkCmdBindVertexBuffers3KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers3KHR.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdBindVertexBuffers3KHR")]
#[inline(always)]
pub unsafe fn cmd_bind_vertex_buffers_3_khr(
    command_buffer: CommandBufferHandle,
    first_binding: u32,
    binding_count: u32,
    binding_infos: *const BindVertexBuffer3InfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdDrawIndirect2KHR =
    unsafe extern "C" fn(CommandBufferHandle, *const DrawIndirect2InfoKHR);
/// [`vkCmdDrawIndirect2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirect2KHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawIndirect2KHR")]
#[inline(always)]
pub unsafe fn cmd_draw_indirect_2_khr(
    command_buffer: CommandBufferHandle,
    info: *const DrawIndirect2InfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdDrawIndexedIndirect2KHR =
    unsafe extern "C" fn(CommandBufferHandle, *const DrawIndirect2InfoKHR);
/// [`vkCmdDrawIndexedIndirect2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirect2KHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawIndexedIndirect2KHR")]
#[inline(always)]
pub unsafe fn cmd_draw_indexed_indirect_2_khr(
    command_buffer: CommandBufferHandle,
    info: *const DrawIndirect2InfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdDispatchIndirect2KHR =
    unsafe extern "C" fn(CommandBufferHandle, *const DispatchIndirect2InfoKHR);
/// [`vkCmdDispatchIndirect2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchIndirect2KHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdDispatchIndirect2KHR")]
#[inline(always)]
pub unsafe fn cmd_dispatch_indirect_2_khr(
    command_buffer: CommandBufferHandle,
    info: *const DispatchIndirect2InfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdCopyMemoryKHR =
    unsafe extern "C" fn(CommandBufferHandle, *const CopyDeviceMemoryInfoKHR);
/// [`vkCmdCopyMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryKHR.html)
///
/// # Optional parameters
/// - copy_memory_info
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
///
#[doc(alias = "vkCmdCopyMemoryKHR")]
#[inline(always)]
pub unsafe fn cmd_copy_memory_khr(
    command_buffer: CommandBufferHandle,
    copy_memory_info: *const CopyDeviceMemoryInfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdCopyMemoryToImageKHR =
    unsafe extern "C" fn(CommandBufferHandle, *const CopyDeviceMemoryImageInfoKHR);
/// [`vkCmdCopyMemoryToImageKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageKHR.html)
///
/// # Optional parameters
/// - copy_memory_info
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
///
#[doc(alias = "vkCmdCopyMemoryToImageKHR")]
#[inline(always)]
pub unsafe fn cmd_copy_memory_to_image_khr(
    command_buffer: CommandBufferHandle,
    copy_memory_info: *const CopyDeviceMemoryImageInfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdCopyImageToMemoryKHR =
    unsafe extern "C" fn(CommandBufferHandle, *const CopyDeviceMemoryImageInfoKHR);
/// [`vkCmdCopyImageToMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToMemoryKHR.html)
///
/// # Optional parameters
/// - copy_memory_info
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
///
#[doc(alias = "vkCmdCopyImageToMemoryKHR")]
#[inline(always)]
pub unsafe fn cmd_copy_image_to_memory_khr(
    command_buffer: CommandBufferHandle,
    copy_memory_info: *const CopyDeviceMemoryImageInfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdUpdateMemoryKHR = unsafe extern "C" fn(
    CommandBufferHandle,
    *const DeviceAddressRangeKHR,
    AddressCommandFlagsKHR,
    DeviceSize,
    *const c_void,
);
/// [`vkCmdUpdateMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdateMemoryKHR.html)
///
/// # Optional parameters
/// - dst_flags
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
///
#[doc(alias = "vkCmdUpdateMemoryKHR")]
#[inline(always)]
pub unsafe fn cmd_update_memory_khr(
    command_buffer: CommandBufferHandle,
    dst_range: *const DeviceAddressRangeKHR,
    dst_flags: AddressCommandFlagsKHR,
    data_size: DeviceSize,
    data: *const c_void,
) {
    todo!()
}

pub(crate) type FUN_CmdFillMemoryKHR = unsafe extern "C" fn(
    CommandBufferHandle,
    *const DeviceAddressRangeKHR,
    AddressCommandFlagsKHR,
    u32,
);
/// [`vkCmdFillMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdFillMemoryKHR.html)
///
/// # Optional parameters
/// - dst_flags
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
///
#[doc(alias = "vkCmdFillMemoryKHR")]
#[inline(always)]
pub unsafe fn cmd_fill_memory_khr(
    command_buffer: CommandBufferHandle,
    dst_range: *const DeviceAddressRangeKHR,
    dst_flags: AddressCommandFlagsKHR,
    data: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdCopyQueryPoolResultsToMemoryKHR = unsafe extern "C" fn(
    CommandBufferHandle,
    QueryPool,
    u32,
    u32,
    *const StridedDeviceAddressRangeKHR,
    AddressCommandFlagsKHR,
    QueryResultFlags,
);
/// [`vkCmdCopyQueryPoolResultsToMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyQueryPoolResultsToMemoryKHR.html)
///
/// # Optional parameters
/// - dst_flags
/// - query_result_flags
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
///
#[doc(alias = "vkCmdCopyQueryPoolResultsToMemoryKHR")]
#[inline(always)]
pub unsafe fn cmd_copy_query_pool_results_to_memory_khr(
    command_buffer: CommandBufferHandle,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
    dst_range: *const StridedDeviceAddressRangeKHR,
    dst_flags: AddressCommandFlagsKHR,
    query_result_flags: QueryResultFlags,
) {
    todo!()
}

pub(crate) type FUN_CmdDrawIndirectCount2KHR =
    unsafe extern "C" fn(CommandBufferHandle, *const DrawIndirectCount2InfoKHR);
/// [`vkCmdDrawIndirectCount2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount2KHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawIndirectCount2KHR")]
#[inline(always)]
pub unsafe fn cmd_draw_indirect_count_2_khr(
    command_buffer: CommandBufferHandle,
    info: *const DrawIndirectCount2InfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdDrawIndexedIndirectCount2KHR =
    unsafe extern "C" fn(CommandBufferHandle, *const DrawIndirectCount2InfoKHR);
/// [`vkCmdDrawIndexedIndirectCount2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount2KHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawIndexedIndirectCount2KHR")]
#[inline(always)]
pub unsafe fn cmd_draw_indexed_indirect_count_2_khr(
    command_buffer: CommandBufferHandle,
    info: *const DrawIndirectCount2InfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdBeginConditionalRendering2EXT =
    unsafe extern "C" fn(CommandBufferHandle, *const ConditionalRenderingBeginInfo2EXT);
/// [`vkCmdBeginConditionalRendering2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginConditionalRendering2EXT.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdBeginConditionalRendering2EXT")]
#[inline(always)]
pub unsafe fn cmd_begin_conditional_rendering_2_ext(
    command_buffer: CommandBufferHandle,
    conditional_rendering_begin: *const ConditionalRenderingBeginInfo2EXT,
) {
    todo!()
}

pub(crate) type FUN_CmdBindTransformFeedbackBuffers2EXT =
    unsafe extern "C" fn(CommandBufferHandle, u32, u32, *const BindTransformFeedbackBuffer2InfoEXT);
/// [`vkCmdBindTransformFeedbackBuffers2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTransformFeedbackBuffers2EXT.html)
///
/// # Optional parameters
/// - binding_infos
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdBindTransformFeedbackBuffers2EXT")]
#[inline(always)]
pub unsafe fn cmd_bind_transform_feedback_buffers_2_ext(
    command_buffer: CommandBufferHandle,
    first_binding: u32,
    binding_count: u32,
    binding_infos: *const BindTransformFeedbackBuffer2InfoEXT,
) {
    todo!()
}

pub(crate) type FUN_CmdBeginTransformFeedback2EXT =
    unsafe extern "C" fn(CommandBufferHandle, u32, u32, *const BindTransformFeedbackBuffer2InfoEXT);
/// [`vkCmdBeginTransformFeedback2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginTransformFeedback2EXT.html)
///
/// # Optional parameters
/// - counter_range_count
/// - counter_infos
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdBeginTransformFeedback2EXT")]
#[inline(always)]
pub unsafe fn cmd_begin_transform_feedback_2_ext(
    command_buffer: CommandBufferHandle,
    first_counter_range: u32,
    counter_range_count: u32,
    counter_infos: *const BindTransformFeedbackBuffer2InfoEXT,
) {
    todo!()
}

pub(crate) type FUN_CmdEndTransformFeedback2EXT =
    unsafe extern "C" fn(CommandBufferHandle, u32, u32, *const BindTransformFeedbackBuffer2InfoEXT);
/// [`vkCmdEndTransformFeedback2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndTransformFeedback2EXT.html)
///
/// # Optional parameters
/// - counter_range_count
/// - counter_infos
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdEndTransformFeedback2EXT")]
#[inline(always)]
pub unsafe fn cmd_end_transform_feedback_2_ext(
    command_buffer: CommandBufferHandle,
    first_counter_range: u32,
    counter_range_count: u32,
    counter_infos: *const BindTransformFeedbackBuffer2InfoEXT,
) {
    todo!()
}

pub(crate) type FUN_CmdDrawIndirectByteCount2EXT = unsafe extern "C" fn(
    CommandBufferHandle,
    u32,
    u32,
    *const BindTransformFeedbackBuffer2InfoEXT,
    u32,
    u32,
);
/// [`vkCmdDrawIndirectByteCount2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectByteCount2EXT.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawIndirectByteCount2EXT")]
#[inline(always)]
pub unsafe fn cmd_draw_indirect_byte_count_2_ext(
    command_buffer: CommandBufferHandle,
    instance_count: u32,
    first_instance: u32,
    counter_info: *const BindTransformFeedbackBuffer2InfoEXT,
    counter_offset: u32,
    vertex_stride: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdDrawMeshTasksIndirect2EXT =
    unsafe extern "C" fn(CommandBufferHandle, *const DrawIndirect2InfoKHR);
/// [`vkCmdDrawMeshTasksIndirect2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirect2EXT.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawMeshTasksIndirect2EXT")]
#[inline(always)]
pub unsafe fn cmd_draw_mesh_tasks_indirect_2_ext(
    command_buffer: CommandBufferHandle,
    info: *const DrawIndirect2InfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdDrawMeshTasksIndirectCount2EXT =
    unsafe extern "C" fn(CommandBufferHandle, *const DrawIndirectCount2InfoKHR);
/// [`vkCmdDrawMeshTasksIndirectCount2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCount2EXT.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawMeshTasksIndirectCount2EXT")]
#[inline(always)]
pub unsafe fn cmd_draw_mesh_tasks_indirect_count_2_ext(
    command_buffer: CommandBufferHandle,
    info: *const DrawIndirectCount2InfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdWriteMarkerToMemoryAMD =
    unsafe extern "C" fn(CommandBufferHandle, *const MemoryMarkerInfoAMD);
/// [`vkCmdWriteMarkerToMemoryAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteMarkerToMemoryAMD.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`TRANSFER`](QueueFlags::TRANSFER)
///
#[doc(alias = "vkCmdWriteMarkerToMemoryAMD")]
#[inline(always)]
pub unsafe fn cmd_write_marker_to_memory_amd(
    command_buffer: CommandBufferHandle,
    info: *const MemoryMarkerInfoAMD,
) {
    todo!()
}

pub(crate) type FUN_CreateAccelerationStructure2KHR = unsafe extern "C" fn(
    DeviceHandle,
    *const AccelerationStructureCreateInfo2KHR,
    *const AllocationCallbacks,
    *mut AccelerationStructureKHR,
) -> ResultCode;
/// [`vkCreateAccelerationStructure2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructure2KHR.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_acceleration_structure_2_khr(
    device: DeviceHandle,
    create_info: *const AccelerationStructureCreateInfo2KHR,
    allocator: *const AllocationCallbacks,
    acceleration_structure: *mut AccelerationStructureKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdCopyBuffer2KHR =
    unsafe extern "C" fn(CommandBufferHandle, *const CopyBufferInfo2);
/// [`vkCmdCopyBuffer2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2KHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyBuffer2KHR")]
#[inline(always)]
pub unsafe fn cmd_copy_buffer_2_khr(
    command_buffer: CommandBufferHandle,
    copy_buffer_info: *const CopyBufferInfo2,
) {
    todo!()
}

pub(crate) type FUN_CmdCopyImage2KHR =
    unsafe extern "C" fn(CommandBufferHandle, *const CopyImageInfo2);
/// [`vkCmdCopyImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2KHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyImage2KHR")]
#[inline(always)]
pub unsafe fn cmd_copy_image_2_khr(
    command_buffer: CommandBufferHandle,
    copy_image_info: *const CopyImageInfo2,
) {
    todo!()
}

pub(crate) type FUN_CmdCopyBufferToImage2KHR =
    unsafe extern "C" fn(CommandBufferHandle, *const CopyBufferToImageInfo2);
/// [`vkCmdCopyBufferToImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2KHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyBufferToImage2KHR")]
#[inline(always)]
pub unsafe fn cmd_copy_buffer_to_image_2_khr(
    command_buffer: CommandBufferHandle,
    copy_buffer_to_image_info: *const CopyBufferToImageInfo2,
) {
    todo!()
}

pub(crate) type FUN_CmdCopyImageToBuffer2KHR =
    unsafe extern "C" fn(CommandBufferHandle, *const CopyImageToBufferInfo2);
/// [`vkCmdCopyImageToBuffer2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2KHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyImageToBuffer2KHR")]
#[inline(always)]
pub unsafe fn cmd_copy_image_to_buffer_2_khr(
    command_buffer: CommandBufferHandle,
    copy_image_to_buffer_info: *const CopyImageToBufferInfo2,
) {
    todo!()
}

pub(crate) type FUN_CmdBlitImage2KHR =
    unsafe extern "C" fn(CommandBufferHandle, *const BlitImageInfo2);
/// [`vkCmdBlitImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2KHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdBlitImage2KHR")]
#[inline(always)]
pub unsafe fn cmd_blit_image_2_khr(
    command_buffer: CommandBufferHandle,
    blit_image_info: *const BlitImageInfo2,
) {
    todo!()
}

pub(crate) type FUN_CmdResolveImage2KHR =
    unsafe extern "C" fn(CommandBufferHandle, *const ResolveImageInfo2);
/// [`vkCmdResolveImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage2KHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdResolveImage2KHR")]
#[inline(always)]
pub unsafe fn cmd_resolve_image_2_khr(
    command_buffer: CommandBufferHandle,
    resolve_image_info: *const ResolveImageInfo2,
) {
    todo!()
}

pub(crate) type FUN_CmdTraceRaysIndirect2KHR =
    unsafe extern "C" fn(CommandBufferHandle, DeviceAddress);
/// [`vkCmdTraceRaysIndirect2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysIndirect2KHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdTraceRaysIndirect2KHR")]
#[inline(always)]
pub unsafe fn cmd_trace_rays_indirect_2_khr(
    command_buffer: CommandBufferHandle,
    indirect_device_address: DeviceAddress,
) {
    todo!()
}

pub(crate) type FUN_GetDeviceBufferMemoryRequirementsKHR = unsafe extern "C" fn(
    DeviceHandle,
    *const DeviceBufferMemoryRequirements,
    *mut MemoryRequirements2,
);
/// [`vkGetDeviceBufferMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceBufferMemoryRequirementsKHR.html)
///
#[doc(alias = "vkGetDeviceBufferMemoryRequirementsKHR")]
#[inline(always)]
pub unsafe fn get_device_buffer_memory_requirements_khr(
    device: DeviceHandle,
    info: *const DeviceBufferMemoryRequirements,
    memory_requirements: *mut MemoryRequirements2,
) {
    todo!()
}

pub(crate) type FUN_GetDeviceImageMemoryRequirementsKHR = unsafe extern "C" fn(
    DeviceHandle,
    *const DeviceImageMemoryRequirements,
    *mut MemoryRequirements2,
);
/// [`vkGetDeviceImageMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageMemoryRequirementsKHR.html)
///
#[doc(alias = "vkGetDeviceImageMemoryRequirementsKHR")]
#[inline(always)]
pub unsafe fn get_device_image_memory_requirements_khr(
    device: DeviceHandle,
    info: *const DeviceImageMemoryRequirements,
    memory_requirements: *mut MemoryRequirements2,
) {
    todo!()
}

pub(crate) type FUN_GetDeviceImageSparseMemoryRequirementsKHR = unsafe extern "C" fn(
    DeviceHandle,
    *const DeviceImageMemoryRequirements,
    *mut u32,
    *mut SparseImageMemoryRequirements2,
);
/// [`vkGetDeviceImageSparseMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirementsKHR.html)
///
/// # Optional parameters
/// - sparse_memory_requirements
///
#[doc(alias = "vkGetDeviceImageSparseMemoryRequirementsKHR")]
#[inline(always)]
pub unsafe fn get_device_image_sparse_memory_requirements_khr(
    device: DeviceHandle,
    info: *const DeviceImageMemoryRequirements,
    sparse_memory_requirement_count: *mut u32,
    sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
) {
    todo!()
}

pub(crate) type FUN_CmdBindIndexBuffer2KHR =
    unsafe extern "C" fn(CommandBufferHandle, Buffer, DeviceSize, DeviceSize, IndexType);
/// [`vkCmdBindIndexBuffer2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer2KHR.html)
///
/// # Optional parameters
/// - buffer
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdBindIndexBuffer2KHR")]
#[inline(always)]
pub unsafe fn cmd_bind_index_buffer_2_khr(
    command_buffer: CommandBufferHandle,
    buffer: Buffer,
    offset: DeviceSize,
    size: DeviceSize,
    index_type: IndexType,
) {
    todo!()
}

pub(crate) type FUN_GetRenderingAreaGranularityKHR =
    unsafe extern "C" fn(DeviceHandle, *const RenderingAreaInfo, *mut Extent2D);
/// [`vkGetRenderingAreaGranularityKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderingAreaGranularityKHR.html)
///
#[doc(alias = "vkGetRenderingAreaGranularityKHR")]
#[inline(always)]
pub unsafe fn get_rendering_area_granularity_khr(
    device: DeviceHandle,
    rendering_area_info: *const RenderingAreaInfo,
    granularity: *mut Extent2D,
) {
    todo!()
}

pub(crate) type FUN_GetDeviceImageSubresourceLayoutKHR =
    unsafe extern "C" fn(DeviceHandle, *const DeviceImageSubresourceInfo, *mut SubresourceLayout2);
/// [`vkGetDeviceImageSubresourceLayoutKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSubresourceLayoutKHR.html)
///
#[doc(alias = "vkGetDeviceImageSubresourceLayoutKHR")]
#[inline(always)]
pub unsafe fn get_device_image_subresource_layout_khr(
    device: DeviceHandle,
    info: *const DeviceImageSubresourceInfo,
    layout: *mut SubresourceLayout2,
) {
    todo!()
}

pub(crate) type FUN_GetImageSubresourceLayout2KHR =
    unsafe extern "C" fn(DeviceHandle, Image, *const ImageSubresource2, *mut SubresourceLayout2);
/// [`vkGetImageSubresourceLayout2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2KHR.html)
///
#[doc(alias = "vkGetImageSubresourceLayout2KHR")]
#[inline(always)]
pub unsafe fn get_image_subresource_layout_2_khr(
    device: DeviceHandle,
    image: Image,
    subresource: *const ImageSubresource2,
    layout: *mut SubresourceLayout2,
) {
    todo!()
}

pub(crate) type FUN_WaitForPresent2KHR =
    unsafe extern "C" fn(DeviceHandle, SwapchainKHR, *const PresentWait2InfoKHR) -> ResultCode;
/// [`vkWaitForPresent2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresent2KHR.html)
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
#[inline(always)]
pub unsafe fn wait_for_present_2_khr(
    device: DeviceHandle,
    swapchain: SwapchainKHR,
    present_wait_2_info: *const PresentWait2InfoKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreatePipelineBinariesKHR = unsafe extern "C" fn(
    DeviceHandle,
    *const PipelineBinaryCreateInfoKHR,
    *const AllocationCallbacks,
    *mut PipelineBinaryHandlesInfoKHR,
) -> ResultCode;
/// [`vkCreatePipelineBinariesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineBinariesKHR.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_pipeline_binaries_khr(
    device: DeviceHandle,
    create_info: *const PipelineBinaryCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    binaries: *mut PipelineBinaryHandlesInfoKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyPipelineBinaryKHR =
    unsafe extern "C" fn(DeviceHandle, PipelineBinaryKHR, *const AllocationCallbacks);
/// [`vkDestroyPipelineBinaryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineBinaryKHR.html)
///
/// # Optional parameters
/// - pipeline_binary
/// - allocator
///
#[doc(alias = "vkDestroyPipelineBinaryKHR")]
#[inline(always)]
pub unsafe fn destroy_pipeline_binary_khr(
    device: DeviceHandle,
    pipeline_binary: PipelineBinaryKHR,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_GetPipelineKeyKHR = unsafe extern "C" fn(
    DeviceHandle,
    *const PipelineCreateInfoKHR,
    *mut PipelineBinaryKeyKHR,
) -> ResultCode;
/// [`vkGetPipelineKeyKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineKeyKHR.html)
///
/// # Optional parameters
/// - pipeline_create_info
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
#[inline(always)]
pub unsafe fn get_pipeline_key_khr(
    device: DeviceHandle,
    pipeline_create_info: *const PipelineCreateInfoKHR,
    pipeline_key: *mut PipelineBinaryKeyKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPipelineBinaryDataKHR = unsafe extern "C" fn(
    DeviceHandle,
    *const PipelineBinaryDataInfoKHR,
    *mut PipelineBinaryKeyKHR,
    *mut usize,
    *mut c_void,
) -> ResultCode;
/// [`vkGetPipelineBinaryDataKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineBinaryDataKHR.html)
///
/// # Optional parameters
/// - pipeline_binary_data
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
#[inline(always)]
pub unsafe fn get_pipeline_binary_data_khr(
    device: DeviceHandle,
    info: *const PipelineBinaryDataInfoKHR,
    pipeline_binary_key: *mut PipelineBinaryKeyKHR,
    pipeline_binary_data_size: *mut usize,
    pipeline_binary_data: *mut c_void,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_ReleaseCapturedPipelineDataKHR = unsafe extern "C" fn(
    DeviceHandle,
    *const ReleaseCapturedPipelineDataInfoKHR,
    *const AllocationCallbacks,
) -> ResultCode;
/// [`vkReleaseCapturedPipelineDataKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseCapturedPipelineDataKHR.html)
///
/// # Optional parameters
/// - allocator
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkReleaseCapturedPipelineDataKHR")]
#[inline(always)]
pub unsafe fn release_captured_pipeline_data_khr(
    device: DeviceHandle,
    info: *const ReleaseCapturedPipelineDataInfoKHR,
    allocator: *const AllocationCallbacks,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_ReleaseSwapchainImagesKHR =
    unsafe extern "C" fn(DeviceHandle, *const ReleaseSwapchainImagesInfoKHR) -> ResultCode;
/// [`vkReleaseSwapchainImagesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseSwapchainImagesKHR.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkReleaseSwapchainImagesKHR")]
#[inline(always)]
pub unsafe fn release_swapchain_images_khr(
    device: DeviceHandle,
    release_info: *const ReleaseSwapchainImagesInfoKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceCooperativeMatrixPropertiesKHR =
    unsafe extern "C" fn(
        PhysicalDeviceHandle,
        *mut u32,
        *mut CooperativeMatrixPropertiesKHR,
    ) -> ResultCode;
/// [`vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR.html)
///
/// # Optional parameters
/// - properties
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
#[inline(always)]
pub unsafe fn get_physical_device_cooperative_matrix_properties_khr(
    physical_device: PhysicalDeviceHandle,
    property_count: *mut u32,
    properties: *mut CooperativeMatrixPropertiesKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdSetLineStippleKHR = unsafe extern "C" fn(CommandBufferHandle, u32, u16);
/// [`vkCmdSetLineStippleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleKHR.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetLineStippleKHR")]
#[inline(always)]
pub unsafe fn cmd_set_line_stipple_khr(
    command_buffer: CommandBufferHandle,
    line_stipple_factor: u32,
    line_stipple_pattern: u16,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceCalibrateableTimeDomainsKHR =
    unsafe extern "C" fn(PhysicalDeviceHandle, *mut u32, *mut TimeDomainKHR) -> ResultCode;
/// [`vkGetPhysicalDeviceCalibrateableTimeDomainsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCalibrateableTimeDomainsKHR.html)
///
/// # Optional parameters
/// - time_domains
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
#[inline(always)]
pub unsafe fn get_physical_device_calibrateable_time_domains_khr(
    physical_device: PhysicalDeviceHandle,
    time_domain_count: *mut u32,
    time_domains: *mut TimeDomainKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetCalibratedTimestampsKHR = unsafe extern "C" fn(
    DeviceHandle,
    u32,
    *const CalibratedTimestampInfoKHR,
    *mut u64,
    *mut u64,
) -> ResultCode;
/// [`vkGetCalibratedTimestampsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCalibratedTimestampsKHR.html)
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
#[inline(always)]
pub unsafe fn get_calibrated_timestamps_khr(
    device: DeviceHandle,
    timestamp_count: u32,
    timestamp_infos: *const CalibratedTimestampInfoKHR,
    timestamps: *mut u64,
    max_deviation: *mut u64,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdBindDescriptorSets2KHR =
    unsafe extern "C" fn(CommandBufferHandle, *const BindDescriptorSetsInfo);
/// [`vkCmdBindDescriptorSets2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2KHR.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdBindDescriptorSets2KHR")]
#[inline(always)]
pub unsafe fn cmd_bind_descriptor_sets_2_khr(
    command_buffer: CommandBufferHandle,
    bind_descriptor_sets_info: *const BindDescriptorSetsInfo,
) {
    todo!()
}

pub(crate) type FUN_CmdPushConstants2KHR =
    unsafe extern "C" fn(CommandBufferHandle, *const PushConstantsInfo);
/// [`vkCmdPushConstants2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2KHR.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdPushConstants2KHR")]
#[inline(always)]
pub unsafe fn cmd_push_constants_2_khr(
    command_buffer: CommandBufferHandle,
    push_constants_info: *const PushConstantsInfo,
) {
    todo!()
}

pub(crate) type FUN_CmdPushDescriptorSet2KHR =
    unsafe extern "C" fn(CommandBufferHandle, *const PushDescriptorSetInfo);
/// [`vkCmdPushDescriptorSet2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2KHR.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdPushDescriptorSet2KHR")]
#[inline(always)]
pub unsafe fn cmd_push_descriptor_set_2_khr(
    command_buffer: CommandBufferHandle,
    push_descriptor_set_info: *const PushDescriptorSetInfo,
) {
    todo!()
}

pub(crate) type FUN_CmdPushDescriptorSetWithTemplate2KHR =
    unsafe extern "C" fn(CommandBufferHandle, *const PushDescriptorSetWithTemplateInfo);
/// [`vkCmdPushDescriptorSetWithTemplate2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate2KHR.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdPushDescriptorSetWithTemplate2KHR")]
#[inline(always)]
pub unsafe fn cmd_push_descriptor_set_with_template_2_khr(
    command_buffer: CommandBufferHandle,
    push_descriptor_set_with_template_info: *const PushDescriptorSetWithTemplateInfo,
) {
    todo!()
}

pub(crate) type FUN_CmdSetDescriptorBufferOffsets2EXT =
    unsafe extern "C" fn(CommandBufferHandle, *const SetDescriptorBufferOffsetsInfoEXT);
/// [`vkCmdSetDescriptorBufferOffsets2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDescriptorBufferOffsets2EXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`DATA_GRAPHARM`](QueueFlags::DATA_GRAPHARM)
///
#[doc(alias = "vkCmdSetDescriptorBufferOffsets2EXT")]
#[inline(always)]
pub unsafe fn cmd_set_descriptor_buffer_offsets_2_ext(
    command_buffer: CommandBufferHandle,
    set_descriptor_buffer_offsets_info: *const SetDescriptorBufferOffsetsInfoEXT,
) {
    todo!()
}

pub(crate) type FUN_CmdBindDescriptorBufferEmbeddedSamplers2EXT =
    unsafe extern "C" fn(CommandBufferHandle, *const BindDescriptorBufferEmbeddedSamplersInfoEXT);
/// [`vkCmdBindDescriptorBufferEmbeddedSamplers2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBufferEmbeddedSamplers2EXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdBindDescriptorBufferEmbeddedSamplers2EXT")]
#[inline(always)]
pub unsafe fn cmd_bind_descriptor_buffer_embedded_samplers_2_ext(
    command_buffer: CommandBufferHandle,
    bind_descriptor_buffer_embedded_samplers_info: *const BindDescriptorBufferEmbeddedSamplersInfoEXT,
) {
    todo!()
}

pub(crate) type FUN_CmdCopyMemoryIndirectKHR =
    unsafe extern "C" fn(CommandBufferHandle, *const CopyMemoryIndirectInfoKHR);
/// [`vkCmdCopyMemoryIndirectKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryIndirectKHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyMemoryIndirectKHR")]
#[inline(always)]
pub unsafe fn cmd_copy_memory_indirect_khr(
    command_buffer: CommandBufferHandle,
    copy_memory_indirect_info: *const CopyMemoryIndirectInfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdCopyMemoryToImageIndirectKHR =
    unsafe extern "C" fn(CommandBufferHandle, *const CopyMemoryToImageIndirectInfoKHR);
/// [`vkCmdCopyMemoryToImageIndirectKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageIndirectKHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyMemoryToImageIndirectKHR")]
#[inline(always)]
pub unsafe fn cmd_copy_memory_to_image_indirect_khr(
    command_buffer: CommandBufferHandle,
    copy_memory_to_image_indirect_info: *const CopyMemoryToImageIndirectInfoKHR,
) {
    todo!()
}

pub(crate) type FUN_GetDeviceFaultReportsKHR =
    unsafe extern "C" fn(DeviceHandle, u64, *mut u32, *mut DeviceFaultInfoKHR) -> ResultCode;
/// [`vkGetDeviceFaultReportsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultReportsKHR.html)
///
/// # Optional parameters
/// - fault_info
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
#[inline(always)]
pub unsafe fn get_device_fault_reports_khr(
    device: DeviceHandle,
    timeout: u64,
    fault_counts: *mut u32,
    fault_info: *mut DeviceFaultInfoKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetDeviceFaultDebugInfoKHR =
    unsafe extern "C" fn(DeviceHandle, *mut DeviceFaultDebugInfoKHR) -> ResultCode;
/// [`vkGetDeviceFaultDebugInfoKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultDebugInfoKHR.html)
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
#[inline(always)]
pub unsafe fn get_device_fault_debug_info_khr(
    device: DeviceHandle,
    debug_info: *mut DeviceFaultDebugInfoKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdEndRendering2KHR =
    unsafe extern "C" fn(CommandBufferHandle, *const RenderingEndInfoKHR);
/// [`vkCmdEndRendering2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering2KHR.html)
///
/// # Optional parameters
/// - rendering_end_info
///
/// # Performed tasks
/// - `action`
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdEndRendering2KHR")]
#[inline(always)]
pub unsafe fn cmd_end_rendering_2_khr(
    command_buffer: CommandBufferHandle,
    rendering_end_info: *const RenderingEndInfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CreateDebugReportCallbackEXT = unsafe extern "C" fn(
    InstanceHandle,
    *const DebugReportCallbackCreateInfoEXT,
    *const AllocationCallbacks,
    *mut DebugReportCallbackEXT,
) -> ResultCode;
/// [`vkCreateDebugReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDebugReportCallbackEXT.html)
///
/// # Optional parameters
/// - allocator
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkCreateDebugReportCallbackEXT")]
#[inline(always)]
pub unsafe fn create_debug_report_callback_ext(
    instance: InstanceHandle,
    create_info: *const DebugReportCallbackCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    callback: *mut DebugReportCallbackEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyDebugReportCallbackEXT =
    unsafe extern "C" fn(InstanceHandle, DebugReportCallbackEXT, *const AllocationCallbacks);
/// [`vkDestroyDebugReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugReportCallbackEXT.html)
///
/// # Optional parameters
/// - callback
/// - allocator
///
#[doc(alias = "vkDestroyDebugReportCallbackEXT")]
#[inline(always)]
pub unsafe fn destroy_debug_report_callback_ext(
    instance: InstanceHandle,
    callback: DebugReportCallbackEXT,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_DebugReportMessageEXT = unsafe extern "C" fn(
    InstanceHandle,
    DebugReportFlagsEXT,
    DebugReportObjectTypeEXT,
    u64,
    usize,
    i32,
    *const c_char,
    *const c_char,
);
/// [`vkDebugReportMessageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugReportMessageEXT.html)
///
#[doc(alias = "vkDebugReportMessageEXT")]
#[inline(always)]
pub unsafe fn debug_report_message_ext(
    instance: InstanceHandle,
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

pub(crate) type FUN_DebugMarkerSetObjectTagEXT =
    unsafe extern "C" fn(DeviceHandle, *const DebugMarkerObjectTagInfoEXT) -> ResultCode;
/// [`vkDebugMarkerSetObjectTagEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugMarkerSetObjectTagEXT.html)
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
#[inline(always)]
pub unsafe fn debug_marker_set_object_tag_ext(
    device: DeviceHandle,
    tag_info: *const DebugMarkerObjectTagInfoEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DebugMarkerSetObjectNameEXT =
    unsafe extern "C" fn(DeviceHandle, *const DebugMarkerObjectNameInfoEXT) -> ResultCode;
/// [`vkDebugMarkerSetObjectNameEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugMarkerSetObjectNameEXT.html)
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
#[inline(always)]
pub unsafe fn debug_marker_set_object_name_ext(
    device: DeviceHandle,
    name_info: *const DebugMarkerObjectNameInfoEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdDebugMarkerBeginEXT =
    unsafe extern "C" fn(CommandBufferHandle, *const DebugMarkerMarkerInfoEXT);
/// [`vkCmdDebugMarkerBeginEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerBeginEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
/// - [`OPTICAL_FLOWNV`](QueueFlags::OPTICAL_FLOWNV)
///
#[doc(alias = "vkCmdDebugMarkerBeginEXT")]
#[inline(always)]
pub unsafe fn cmd_debug_marker_begin_ext(
    command_buffer: CommandBufferHandle,
    marker_info: *const DebugMarkerMarkerInfoEXT,
) {
    todo!()
}

pub(crate) type FUN_CmdDebugMarkerEndEXT = unsafe extern "C" fn(CommandBufferHandle);
/// [`vkCmdDebugMarkerEndEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerEndEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
/// - [`OPTICAL_FLOWNV`](QueueFlags::OPTICAL_FLOWNV)
///
#[doc(alias = "vkCmdDebugMarkerEndEXT")]
#[inline(always)]
pub unsafe fn cmd_debug_marker_end_ext(command_buffer: CommandBufferHandle) {
    todo!()
}

pub(crate) type FUN_CmdDebugMarkerInsertEXT =
    unsafe extern "C" fn(CommandBufferHandle, *const DebugMarkerMarkerInfoEXT);
/// [`vkCmdDebugMarkerInsertEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerInsertEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
/// - [`OPTICAL_FLOWNV`](QueueFlags::OPTICAL_FLOWNV)
///
#[doc(alias = "vkCmdDebugMarkerInsertEXT")]
#[inline(always)]
pub unsafe fn cmd_debug_marker_insert_ext(
    command_buffer: CommandBufferHandle,
    marker_info: *const DebugMarkerMarkerInfoEXT,
) {
    todo!()
}

pub(crate) type FUN_CmdBindTransformFeedbackBuffersEXT = unsafe extern "C" fn(
    CommandBufferHandle,
    u32,
    u32,
    *const Buffer,
    *const DeviceSize,
    *const DeviceSize,
);
/// [`vkCmdBindTransformFeedbackBuffersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTransformFeedbackBuffersEXT.html)
///
/// # Optional parameters
/// - sizes
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdBindTransformFeedbackBuffersEXT")]
#[inline(always)]
pub unsafe fn cmd_bind_transform_feedback_buffers_ext(
    command_buffer: CommandBufferHandle,
    first_binding: u32,
    binding_count: u32,
    buffers: *const Buffer,
    offsets: *const DeviceSize,
    sizes: *const DeviceSize,
) {
    todo!()
}

pub(crate) type FUN_CmdBeginTransformFeedbackEXT =
    unsafe extern "C" fn(CommandBufferHandle, u32, u32, *const Buffer, *const DeviceSize);
/// [`vkCmdBeginTransformFeedbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginTransformFeedbackEXT.html)
///
/// # Optional parameters
/// - counter_buffer_count
/// - counter_buffer_offsets
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdBeginTransformFeedbackEXT")]
#[inline(always)]
pub unsafe fn cmd_begin_transform_feedback_ext(
    command_buffer: CommandBufferHandle,
    first_counter_buffer: u32,
    counter_buffer_count: u32,
    counter_buffers: *const Buffer,
    counter_buffer_offsets: *const DeviceSize,
) {
    todo!()
}

pub(crate) type FUN_CmdEndTransformFeedbackEXT =
    unsafe extern "C" fn(CommandBufferHandle, u32, u32, *const Buffer, *const DeviceSize);
/// [`vkCmdEndTransformFeedbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndTransformFeedbackEXT.html)
///
/// # Optional parameters
/// - counter_buffer_count
/// - counter_buffer_offsets
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdEndTransformFeedbackEXT")]
#[inline(always)]
pub unsafe fn cmd_end_transform_feedback_ext(
    command_buffer: CommandBufferHandle,
    first_counter_buffer: u32,
    counter_buffer_count: u32,
    counter_buffers: *const Buffer,
    counter_buffer_offsets: *const DeviceSize,
) {
    todo!()
}

pub(crate) type FUN_CmdBeginQueryIndexedEXT =
    unsafe extern "C" fn(CommandBufferHandle, QueryPool, u32, QueryControlFlags, u32);
/// [`vkCmdBeginQueryIndexedEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginQueryIndexedEXT.html)
///
/// # Optional parameters
/// - flags
///
/// # Performed tasks
/// - `action`
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
///
#[doc(alias = "vkCmdBeginQueryIndexedEXT")]
#[inline(always)]
pub unsafe fn cmd_begin_query_indexed_ext(
    command_buffer: CommandBufferHandle,
    query_pool: QueryPool,
    query: u32,
    flags: QueryControlFlags,
    index: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdEndQueryIndexedEXT =
    unsafe extern "C" fn(CommandBufferHandle, QueryPool, u32, u32);
/// [`vkCmdEndQueryIndexedEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndQueryIndexedEXT.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
///
#[doc(alias = "vkCmdEndQueryIndexedEXT")]
#[inline(always)]
pub unsafe fn cmd_end_query_indexed_ext(
    command_buffer: CommandBufferHandle,
    query_pool: QueryPool,
    query: u32,
    index: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdDrawIndirectByteCountEXT =
    unsafe extern "C" fn(CommandBufferHandle, u32, u32, Buffer, DeviceSize, u32, u32);
/// [`vkCmdDrawIndirectByteCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectByteCountEXT.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawIndirectByteCountEXT")]
#[inline(always)]
pub unsafe fn cmd_draw_indirect_byte_count_ext(
    command_buffer: CommandBufferHandle,
    instance_count: u32,
    first_instance: u32,
    counter_buffer: Buffer,
    counter_buffer_offset: DeviceSize,
    counter_offset: u32,
    vertex_stride: u32,
) {
    todo!()
}

pub(crate) type FUN_CreateCuModuleNVX = unsafe extern "C" fn(
    DeviceHandle,
    *const CuModuleCreateInfoNVX,
    *const AllocationCallbacks,
    *mut CuModuleNVX,
) -> ResultCode;
/// [`vkCreateCuModuleNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCuModuleNVX.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_cu_module_nvx(
    device: DeviceHandle,
    create_info: *const CuModuleCreateInfoNVX,
    allocator: *const AllocationCallbacks,
    module: *mut CuModuleNVX,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateCuFunctionNVX = unsafe extern "C" fn(
    DeviceHandle,
    *const CuFunctionCreateInfoNVX,
    *const AllocationCallbacks,
    *mut CuFunctionNVX,
) -> ResultCode;
/// [`vkCreateCuFunctionNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCuFunctionNVX.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_cu_function_nvx(
    device: DeviceHandle,
    create_info: *const CuFunctionCreateInfoNVX,
    allocator: *const AllocationCallbacks,
    function: *mut CuFunctionNVX,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyCuModuleNVX =
    unsafe extern "C" fn(DeviceHandle, CuModuleNVX, *const AllocationCallbacks);
/// [`vkDestroyCuModuleNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuModuleNVX.html)
///
/// # Optional parameters
/// - allocator
///
#[doc(alias = "vkDestroyCuModuleNVX")]
#[inline(always)]
pub unsafe fn destroy_cu_module_nvx(
    device: DeviceHandle,
    module: CuModuleNVX,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_DestroyCuFunctionNVX =
    unsafe extern "C" fn(DeviceHandle, CuFunctionNVX, *const AllocationCallbacks);
/// [`vkDestroyCuFunctionNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuFunctionNVX.html)
///
/// # Optional parameters
/// - allocator
///
#[doc(alias = "vkDestroyCuFunctionNVX")]
#[inline(always)]
pub unsafe fn destroy_cu_function_nvx(
    device: DeviceHandle,
    function: CuFunctionNVX,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_CmdCuLaunchKernelNVX =
    unsafe extern "C" fn(CommandBufferHandle, *const CuLaunchInfoNVX);
/// [`vkCmdCuLaunchKernelNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCuLaunchKernelNVX.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCuLaunchKernelNVX")]
#[inline(always)]
pub unsafe fn cmd_cu_launch_kernel_nvx(
    command_buffer: CommandBufferHandle,
    launch_info: *const CuLaunchInfoNVX,
) {
    todo!()
}

pub(crate) type FUN_GetImageViewHandleNVX =
    unsafe extern "C" fn(DeviceHandle, *const ImageViewHandleInfoNVX) -> u32;
/// [`vkGetImageViewHandleNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewHandleNVX.html)
///
#[doc(alias = "vkGetImageViewHandleNVX")]
#[inline(always)]
pub unsafe fn get_image_view_handle_nvx(
    device: DeviceHandle,
    info: *const ImageViewHandleInfoNVX,
) -> u32 {
    todo!()
}

pub(crate) type FUN_GetImageViewHandle64NVX =
    unsafe extern "C" fn(DeviceHandle, *const ImageViewHandleInfoNVX) -> u64;
/// [`vkGetImageViewHandle64NVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewHandle64NVX.html)
///
#[doc(alias = "vkGetImageViewHandle64NVX")]
#[inline(always)]
pub unsafe fn get_image_view_handle_64_nvx(
    device: DeviceHandle,
    info: *const ImageViewHandleInfoNVX,
) -> u64 {
    todo!()
}

pub(crate) type FUN_GetImageViewAddressNVX =
    unsafe extern "C" fn(DeviceHandle, ImageView, *mut ImageViewAddressPropertiesNVX) -> ResultCode;
/// [`vkGetImageViewAddressNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewAddressNVX.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkGetImageViewAddressNVX")]
#[inline(always)]
pub unsafe fn get_image_view_address_nvx(
    device: DeviceHandle,
    image_view: ImageView,
    properties: *mut ImageViewAddressPropertiesNVX,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetDeviceCombinedImageSamplerIndexNVX =
    unsafe extern "C" fn(DeviceHandle, u64, u64) -> u64;
/// [`vkGetDeviceCombinedImageSamplerIndexNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceCombinedImageSamplerIndexNVX.html)
///
#[doc(alias = "vkGetDeviceCombinedImageSamplerIndexNVX")]
#[inline(always)]
pub unsafe fn get_device_combined_image_sampler_index_nvx(
    device: DeviceHandle,
    image_view_index: u64,
    sampler_index: u64,
) -> u64 {
    todo!()
}

pub(crate) type FUN_CmdDrawIndirectCountAMD =
    unsafe extern "C" fn(CommandBufferHandle, Buffer, DeviceSize, Buffer, DeviceSize, u32, u32);
/// [`vkCmdDrawIndirectCountAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCountAMD.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawIndirectCountAMD")]
#[inline(always)]
pub unsafe fn cmd_draw_indirect_count_amd(
    command_buffer: CommandBufferHandle,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdDrawIndexedIndirectCountAMD =
    unsafe extern "C" fn(CommandBufferHandle, Buffer, DeviceSize, Buffer, DeviceSize, u32, u32);
/// [`vkCmdDrawIndexedIndirectCountAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCountAMD.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawIndexedIndirectCountAMD")]
#[inline(always)]
pub unsafe fn cmd_draw_indexed_indirect_count_amd(
    command_buffer: CommandBufferHandle,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
) {
    todo!()
}

pub(crate) type FUN_GetShaderInfoAMD = unsafe extern "C" fn(
    DeviceHandle,
    Pipeline,
    ShaderStageFlags,
    ShaderInfoTypeAMD,
    *mut usize,
    *mut c_void,
) -> ResultCode;
/// [`vkGetShaderInfoAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderInfoAMD.html)
///
/// # Optional parameters
/// - info
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
#[inline(always)]
pub unsafe fn get_shader_info_amd(
    device: DeviceHandle,
    pipeline: Pipeline,
    shader_stage: ShaderStageFlags,
    info_type: ShaderInfoTypeAMD,
    info_size: *mut usize,
    info: *mut c_void,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateStreamDescriptorSurfaceGGP = unsafe extern "C" fn(
    InstanceHandle,
    *const StreamDescriptorSurfaceCreateInfoGGP,
    *const AllocationCallbacks,
    *mut SurfaceKHR,
) -> ResultCode;
/// [`vkCreateStreamDescriptorSurfaceGGP`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateStreamDescriptorSurfaceGGP.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_stream_descriptor_surface_ggp(
    instance: InstanceHandle,
    create_info: *const StreamDescriptorSurfaceCreateInfoGGP,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceExternalImageFormatPropertiesNV =
    unsafe extern "C" fn(
        PhysicalDeviceHandle,
        Format,
        ImageType,
        ImageTiling,
        ImageUsageFlags,
        ImageCreateFlags,
        ExternalMemoryHandleTypeFlagsNV,
        *mut ExternalImageFormatPropertiesNV,
    ) -> ResultCode;
/// [`vkGetPhysicalDeviceExternalImageFormatPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html)
///
/// # Optional parameters
/// - flags
/// - external_handle_type
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
#[inline(always)]
pub unsafe fn get_physical_device_external_image_format_properties_nv(
    physical_device: PhysicalDeviceHandle,
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

pub(crate) type FUN_GetMemoryWin32HandleNV = unsafe extern "C" fn(
    DeviceHandle,
    DeviceMemory,
    ExternalMemoryHandleTypeFlagsNV,
    *mut HANDLE,
) -> ResultCode;
/// [`vkGetMemoryWin32HandleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleNV.html)
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
#[inline(always)]
pub unsafe fn get_memory_win_32_handle_nv(
    device: DeviceHandle,
    memory: DeviceMemory,
    handle_type: ExternalMemoryHandleTypeFlagsNV,
    handle: *mut HANDLE,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateViSurfaceNN = unsafe extern "C" fn(
    InstanceHandle,
    *const ViSurfaceCreateInfoNN,
    *const AllocationCallbacks,
    *mut SurfaceKHR,
) -> ResultCode;
/// [`vkCreateViSurfaceNN`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateViSurfaceNN.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_vi_surface_nn(
    instance: InstanceHandle,
    create_info: *const ViSurfaceCreateInfoNN,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdBeginConditionalRenderingEXT =
    unsafe extern "C" fn(CommandBufferHandle, *const ConditionalRenderingBeginInfoEXT);
/// [`vkCmdBeginConditionalRenderingEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginConditionalRenderingEXT.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdBeginConditionalRenderingEXT")]
#[inline(always)]
pub unsafe fn cmd_begin_conditional_rendering_ext(
    command_buffer: CommandBufferHandle,
    conditional_rendering_begin: *const ConditionalRenderingBeginInfoEXT,
) {
    todo!()
}

pub(crate) type FUN_CmdEndConditionalRenderingEXT = unsafe extern "C" fn(CommandBufferHandle);
/// [`vkCmdEndConditionalRenderingEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndConditionalRenderingEXT.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdEndConditionalRenderingEXT")]
#[inline(always)]
pub unsafe fn cmd_end_conditional_rendering_ext(command_buffer: CommandBufferHandle) {
    todo!()
}

pub(crate) type FUN_CmdSetViewportWScalingNV =
    unsafe extern "C" fn(CommandBufferHandle, u32, u32, *const ViewportWScalingNV);
/// [`vkCmdSetViewportWScalingNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWScalingNV.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetViewportWScalingNV")]
#[inline(always)]
pub unsafe fn cmd_set_viewport_w_scaling_nv(
    command_buffer: CommandBufferHandle,
    first_viewport: u32,
    viewport_count: u32,
    viewport_w_scalings: *const ViewportWScalingNV,
) {
    todo!()
}

pub(crate) type FUN_ReleaseDisplayEXT =
    unsafe extern "C" fn(PhysicalDeviceHandle, DisplayKHR) -> ResultCode;
/// [`vkReleaseDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseDisplayEXT.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkReleaseDisplayEXT")]
#[inline(always)]
pub unsafe fn release_display_ext(
    physical_device: PhysicalDeviceHandle,
    display: DisplayKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_AcquireXlibDisplayEXT =
    unsafe extern "C" fn(PhysicalDeviceHandle, *mut Display, DisplayKHR) -> ResultCode;
/// [`vkAcquireXlibDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireXlibDisplayEXT.html)
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
#[inline(always)]
pub unsafe fn acquire_xlib_display_ext(
    physical_device: PhysicalDeviceHandle,
    dpy: *mut Display,
    display: DisplayKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetRandROutputDisplayEXT = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *mut Display,
    RROutput,
    *mut DisplayKHR,
) -> ResultCode;
/// [`vkGetRandROutputDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRandROutputDisplayEXT.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkGetRandROutputDisplayEXT")]
#[inline(always)]
pub unsafe fn get_rand_r_output_display_ext(
    physical_device: PhysicalDeviceHandle,
    dpy: *mut Display,
    rr_output: RROutput,
    display: *mut DisplayKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceSurfaceCapabilities2EXT = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    SurfaceKHR,
    *mut SurfaceCapabilities2EXT,
) -> ResultCode;
/// [`vkGetPhysicalDeviceSurfaceCapabilities2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html)
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
#[inline(always)]
pub unsafe fn get_physical_device_surface_capabilities_2_ext(
    physical_device: PhysicalDeviceHandle,
    surface: SurfaceKHR,
    surface_capabilities: *mut SurfaceCapabilities2EXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DisplayPowerControlEXT =
    unsafe extern "C" fn(DeviceHandle, DisplayKHR, *const DisplayPowerInfoEXT) -> ResultCode;
/// [`vkDisplayPowerControlEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDisplayPowerControlEXT.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkDisplayPowerControlEXT")]
#[inline(always)]
pub unsafe fn display_power_control_ext(
    device: DeviceHandle,
    display: DisplayKHR,
    display_power_info: *const DisplayPowerInfoEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_RegisterDeviceEventEXT = unsafe extern "C" fn(
    DeviceHandle,
    *const DeviceEventInfoEXT,
    *const AllocationCallbacks,
    *mut Fence,
) -> ResultCode;
/// [`vkRegisterDeviceEventEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterDeviceEventEXT.html)
///
/// # Optional parameters
/// - allocator
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkRegisterDeviceEventEXT")]
#[inline(always)]
pub unsafe fn register_device_event_ext(
    device: DeviceHandle,
    device_event_info: *const DeviceEventInfoEXT,
    allocator: *const AllocationCallbacks,
    fence: *mut Fence,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_RegisterDisplayEventEXT = unsafe extern "C" fn(
    DeviceHandle,
    DisplayKHR,
    *const DisplayEventInfoEXT,
    *const AllocationCallbacks,
    *mut Fence,
) -> ResultCode;
/// [`vkRegisterDisplayEventEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterDisplayEventEXT.html)
///
/// # Optional parameters
/// - allocator
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkRegisterDisplayEventEXT")]
#[inline(always)]
pub unsafe fn register_display_event_ext(
    device: DeviceHandle,
    display: DisplayKHR,
    display_event_info: *const DisplayEventInfoEXT,
    allocator: *const AllocationCallbacks,
    fence: *mut Fence,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetSwapchainCounterEXT = unsafe extern "C" fn(
    DeviceHandle,
    SwapchainKHR,
    SurfaceCounterFlagsEXT,
    *mut u64,
) -> ResultCode;
/// [`vkGetSwapchainCounterEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainCounterEXT.html)
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
#[inline(always)]
pub unsafe fn get_swapchain_counter_ext(
    device: DeviceHandle,
    swapchain: SwapchainKHR,
    counter: SurfaceCounterFlagsEXT,
    counter_value: *mut u64,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetRefreshCycleDurationGOOGLE =
    unsafe extern "C" fn(DeviceHandle, SwapchainKHR, *mut RefreshCycleDurationGOOGLE) -> ResultCode;
/// [`vkGetRefreshCycleDurationGOOGLE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRefreshCycleDurationGOOGLE.html)
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
#[inline(always)]
pub unsafe fn get_refresh_cycle_duration_google(
    device: DeviceHandle,
    swapchain: SwapchainKHR,
    display_timing_properties: *mut RefreshCycleDurationGOOGLE,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPastPresentationTimingGOOGLE = unsafe extern "C" fn(
    DeviceHandle,
    SwapchainKHR,
    *mut u32,
    *mut PastPresentationTimingGOOGLE,
) -> ResultCode;
/// [`vkGetPastPresentationTimingGOOGLE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingGOOGLE.html)
///
/// # Optional parameters
/// - presentation_timings
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
#[inline(always)]
pub unsafe fn get_past_presentation_timing_google(
    device: DeviceHandle,
    swapchain: SwapchainKHR,
    presentation_timing_count: *mut u32,
    presentation_timings: *mut PastPresentationTimingGOOGLE,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdSetDiscardRectangleEXT =
    unsafe extern "C" fn(CommandBufferHandle, u32, u32, *const Rect2D);
/// [`vkCmdSetDiscardRectangleEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetDiscardRectangleEXT")]
#[inline(always)]
pub unsafe fn cmd_set_discard_rectangle_ext(
    command_buffer: CommandBufferHandle,
    first_discard_rectangle: u32,
    discard_rectangle_count: u32,
    discard_rectangles: *const Rect2D,
) {
    todo!()
}

pub(crate) type FUN_CmdSetDiscardRectangleEnableEXT =
    unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetDiscardRectangleEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleEnableEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetDiscardRectangleEnableEXT")]
#[inline(always)]
pub unsafe fn cmd_set_discard_rectangle_enable_ext(
    command_buffer: CommandBufferHandle,
    discard_rectangle_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetDiscardRectangleModeEXT =
    unsafe extern "C" fn(CommandBufferHandle, DiscardRectangleModeEXT);
/// [`vkCmdSetDiscardRectangleModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleModeEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetDiscardRectangleModeEXT")]
#[inline(always)]
pub unsafe fn cmd_set_discard_rectangle_mode_ext(
    command_buffer: CommandBufferHandle,
    discard_rectangle_mode: DiscardRectangleModeEXT,
) {
    todo!()
}

pub(crate) type FUN_SetHdrMetadataEXT =
    unsafe extern "C" fn(DeviceHandle, u32, *const SwapchainKHR, *const HdrMetadataEXT);
/// [`vkSetHdrMetadataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetHdrMetadataEXT.html)
///
#[doc(alias = "vkSetHdrMetadataEXT")]
#[inline(always)]
pub unsafe fn set_hdr_metadata_ext(
    device: DeviceHandle,
    swapchain_count: u32,
    swapchains: *const SwapchainKHR,
    metadata: *const HdrMetadataEXT,
) {
    todo!()
}

pub(crate) type FUN_CreateIOSSurfaceMVK = unsafe extern "C" fn(
    InstanceHandle,
    *const IOSSurfaceCreateInfoMVK,
    *const AllocationCallbacks,
    *mut SurfaceKHR,
) -> ResultCode;
/// [`vkCreateIOSSurfaceMVK`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIOSSurfaceMVK.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_ios_surface_mvk(
    instance: InstanceHandle,
    create_info: *const IOSSurfaceCreateInfoMVK,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateMacOSSurfaceMVK = unsafe extern "C" fn(
    InstanceHandle,
    *const MacOSSurfaceCreateInfoMVK,
    *const AllocationCallbacks,
    *mut SurfaceKHR,
) -> ResultCode;
/// [`vkCreateMacOSSurfaceMVK`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMacOSSurfaceMVK.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_mac_os_surface_mvk(
    instance: InstanceHandle,
    create_info: *const MacOSSurfaceCreateInfoMVK,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_SetDebugUtilsObjectNameEXT =
    unsafe extern "C" fn(DeviceHandle, *const DebugUtilsObjectNameInfoEXT) -> ResultCode;
/// [`vkSetDebugUtilsObjectNameEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDebugUtilsObjectNameEXT.html)
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
#[inline(always)]
pub unsafe fn set_debug_utils_object_name_ext(
    device: DeviceHandle,
    name_info: *const DebugUtilsObjectNameInfoEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_SetDebugUtilsObjectTagEXT =
    unsafe extern "C" fn(DeviceHandle, *const DebugUtilsObjectTagInfoEXT) -> ResultCode;
/// [`vkSetDebugUtilsObjectTagEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDebugUtilsObjectTagEXT.html)
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
#[inline(always)]
pub unsafe fn set_debug_utils_object_tag_ext(
    device: DeviceHandle,
    tag_info: *const DebugUtilsObjectTagInfoEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_QueueBeginDebugUtilsLabelEXT =
    unsafe extern "C" fn(QueueHandle, *const DebugUtilsLabelEXT);
/// [`vkQueueBeginDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBeginDebugUtilsLabelEXT.html)
///
#[doc(alias = "vkQueueBeginDebugUtilsLabelEXT")]
#[inline(always)]
pub unsafe fn queue_begin_debug_utils_label_ext(
    queue: QueueHandle,
    label_info: *const DebugUtilsLabelEXT,
) {
    todo!()
}

pub(crate) type FUN_QueueEndDebugUtilsLabelEXT = unsafe extern "C" fn(QueueHandle);
/// [`vkQueueEndDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueEndDebugUtilsLabelEXT.html)
///
#[doc(alias = "vkQueueEndDebugUtilsLabelEXT")]
#[inline(always)]
pub unsafe fn queue_end_debug_utils_label_ext(queue: QueueHandle) {
    todo!()
}

pub(crate) type FUN_QueueInsertDebugUtilsLabelEXT =
    unsafe extern "C" fn(QueueHandle, *const DebugUtilsLabelEXT);
/// [`vkQueueInsertDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueInsertDebugUtilsLabelEXT.html)
///
#[doc(alias = "vkQueueInsertDebugUtilsLabelEXT")]
#[inline(always)]
pub unsafe fn queue_insert_debug_utils_label_ext(
    queue: QueueHandle,
    label_info: *const DebugUtilsLabelEXT,
) {
    todo!()
}

pub(crate) type FUN_CmdBeginDebugUtilsLabelEXT =
    unsafe extern "C" fn(CommandBufferHandle, *const DebugUtilsLabelEXT);
/// [`vkCmdBeginDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginDebugUtilsLabelEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
/// - [`OPTICAL_FLOWNV`](QueueFlags::OPTICAL_FLOWNV)
///
#[doc(alias = "vkCmdBeginDebugUtilsLabelEXT")]
#[inline(always)]
pub unsafe fn cmd_begin_debug_utils_label_ext(
    command_buffer: CommandBufferHandle,
    label_info: *const DebugUtilsLabelEXT,
) {
    todo!()
}

pub(crate) type FUN_CmdEndDebugUtilsLabelEXT = unsafe extern "C" fn(CommandBufferHandle);
/// [`vkCmdEndDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndDebugUtilsLabelEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
/// - [`OPTICAL_FLOWNV`](QueueFlags::OPTICAL_FLOWNV)
///
#[doc(alias = "vkCmdEndDebugUtilsLabelEXT")]
#[inline(always)]
pub unsafe fn cmd_end_debug_utils_label_ext(command_buffer: CommandBufferHandle) {
    todo!()
}

pub(crate) type FUN_CmdInsertDebugUtilsLabelEXT =
    unsafe extern "C" fn(CommandBufferHandle, *const DebugUtilsLabelEXT);
/// [`vkCmdInsertDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdInsertDebugUtilsLabelEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`VIDEO_DECODEKHR`](QueueFlags::VIDEO_DECODEKHR)
/// - [`VIDEO_ENCODEKHR`](QueueFlags::VIDEO_ENCODEKHR)
/// - [`OPTICAL_FLOWNV`](QueueFlags::OPTICAL_FLOWNV)
///
#[doc(alias = "vkCmdInsertDebugUtilsLabelEXT")]
#[inline(always)]
pub unsafe fn cmd_insert_debug_utils_label_ext(
    command_buffer: CommandBufferHandle,
    label_info: *const DebugUtilsLabelEXT,
) {
    todo!()
}

pub(crate) type FUN_CreateDebugUtilsMessengerEXT = unsafe extern "C" fn(
    InstanceHandle,
    *const DebugUtilsMessengerCreateInfoEXT,
    *const AllocationCallbacks,
    *mut DebugUtilsMessengerEXT,
) -> ResultCode;
/// [`vkCreateDebugUtilsMessengerEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDebugUtilsMessengerEXT.html)
///
/// # Optional parameters
/// - allocator
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkCreateDebugUtilsMessengerEXT")]
#[inline(always)]
pub unsafe fn create_debug_utils_messenger_ext(
    instance: InstanceHandle,
    create_info: *const DebugUtilsMessengerCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    messenger: *mut DebugUtilsMessengerEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyDebugUtilsMessengerEXT =
    unsafe extern "C" fn(InstanceHandle, DebugUtilsMessengerEXT, *const AllocationCallbacks);
/// [`vkDestroyDebugUtilsMessengerEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugUtilsMessengerEXT.html)
///
/// # Optional parameters
/// - messenger
/// - allocator
///
#[doc(alias = "vkDestroyDebugUtilsMessengerEXT")]
#[inline(always)]
pub unsafe fn destroy_debug_utils_messenger_ext(
    instance: InstanceHandle,
    messenger: DebugUtilsMessengerEXT,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_SubmitDebugUtilsMessageEXT = unsafe extern "C" fn(
    InstanceHandle,
    DebugUtilsMessageSeverityFlagsEXT,
    DebugUtilsMessageTypeFlagsEXT,
    *const DebugUtilsMessengerCallbackDataEXT,
);
/// [`vkSubmitDebugUtilsMessageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSubmitDebugUtilsMessageEXT.html)
///
#[doc(alias = "vkSubmitDebugUtilsMessageEXT")]
#[inline(always)]
pub unsafe fn submit_debug_utils_message_ext(
    instance: InstanceHandle,
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    message_types: DebugUtilsMessageTypeFlagsEXT,
    callback_data: *const DebugUtilsMessengerCallbackDataEXT,
) {
    todo!()
}

pub(crate) type FUN_GetAndroidHardwareBufferPropertiesANDROID = unsafe extern "C" fn(
    DeviceHandle,
    *const AHardwareBuffer,
    *mut AndroidHardwareBufferPropertiesANDROID,
) -> ResultCode;
/// [`vkGetAndroidHardwareBufferPropertiesANDROID`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAndroidHardwareBufferPropertiesANDROID.html)
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
#[inline(always)]
pub unsafe fn get_android_hardware_buffer_properties_android(
    device: DeviceHandle,
    buffer: *const AHardwareBuffer,
    properties: *mut AndroidHardwareBufferPropertiesANDROID,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetMemoryAndroidHardwareBufferANDROID = unsafe extern "C" fn(
    DeviceHandle,
    *const MemoryGetAndroidHardwareBufferInfoANDROID,
    *mut *mut AHardwareBuffer,
) -> ResultCode;
/// [`vkGetMemoryAndroidHardwareBufferANDROID`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryAndroidHardwareBufferANDROID.html)
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
#[inline(always)]
pub unsafe fn get_memory_android_hardware_buffer_android(
    device: DeviceHandle,
    info: *const MemoryGetAndroidHardwareBufferInfoANDROID,
    buffer: *mut *mut AHardwareBuffer,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateGpaSessionAMD = unsafe extern "C" fn(
    DeviceHandle,
    *const GpaSessionCreateInfoAMD,
    *const AllocationCallbacks,
    *mut GpaSessionAMD,
) -> ResultCode;
/// [`vkCreateGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateGpaSessionAMD.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_gpa_session_amd(
    device: DeviceHandle,
    create_info: *const GpaSessionCreateInfoAMD,
    allocator: *const AllocationCallbacks,
    gpa_session: *mut GpaSessionAMD,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyGpaSessionAMD =
    unsafe extern "C" fn(DeviceHandle, GpaSessionAMD, *const AllocationCallbacks);
/// [`vkDestroyGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyGpaSessionAMD.html)
///
/// # Optional parameters
/// - gpa_session
/// - allocator
///
#[doc(alias = "vkDestroyGpaSessionAMD")]
#[inline(always)]
pub unsafe fn destroy_gpa_session_amd(
    device: DeviceHandle,
    gpa_session: GpaSessionAMD,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_SetGpaDeviceClockModeAMD =
    unsafe extern "C" fn(DeviceHandle, *mut GpaDeviceClockModeInfoAMD) -> ResultCode;
/// [`vkSetGpaDeviceClockModeAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetGpaDeviceClockModeAMD.html)
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
#[inline(always)]
pub unsafe fn set_gpa_device_clock_mode_amd(
    device: DeviceHandle,
    info: *mut GpaDeviceClockModeInfoAMD,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetGpaDeviceClockInfoAMD =
    unsafe extern "C" fn(DeviceHandle, *mut GpaDeviceGetClockInfoAMD) -> ResultCode;
/// [`vkGetGpaDeviceClockInfoAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaDeviceClockInfoAMD.html)
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
#[inline(always)]
pub unsafe fn get_gpa_device_clock_info_amd(
    device: DeviceHandle,
    info: *mut GpaDeviceGetClockInfoAMD,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdBeginGpaSessionAMD =
    unsafe extern "C" fn(CommandBufferHandle, GpaSessionAMD) -> ResultCode;
/// [`vkCmdBeginGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginGpaSessionAMD.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
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
#[inline(always)]
pub unsafe fn cmd_begin_gpa_session_amd(
    command_buffer: CommandBufferHandle,
    gpa_session: GpaSessionAMD,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdEndGpaSessionAMD =
    unsafe extern "C" fn(CommandBufferHandle, GpaSessionAMD) -> ResultCode;
/// [`vkCmdEndGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndGpaSessionAMD.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
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
#[inline(always)]
pub unsafe fn cmd_end_gpa_session_amd(
    command_buffer: CommandBufferHandle,
    gpa_session: GpaSessionAMD,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdBeginGpaSampleAMD = unsafe extern "C" fn(
    CommandBufferHandle,
    GpaSessionAMD,
    *const GpaSampleBeginInfoAMD,
    *mut u32,
) -> ResultCode;
/// [`vkCmdBeginGpaSampleAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginGpaSampleAMD.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
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
#[inline(always)]
pub unsafe fn cmd_begin_gpa_sample_amd(
    command_buffer: CommandBufferHandle,
    gpa_session: GpaSessionAMD,
    gpa_sample_begin_info: *const GpaSampleBeginInfoAMD,
    sample_id: *mut u32,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdEndGpaSampleAMD =
    unsafe extern "C" fn(CommandBufferHandle, GpaSessionAMD, u32);
/// [`vkCmdEndGpaSampleAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndGpaSampleAMD.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdEndGpaSampleAMD")]
#[inline(always)]
pub unsafe fn cmd_end_gpa_sample_amd(
    command_buffer: CommandBufferHandle,
    gpa_session: GpaSessionAMD,
    sample_id: u32,
) {
    todo!()
}

pub(crate) type FUN_GetGpaSessionStatusAMD =
    unsafe extern "C" fn(DeviceHandle, GpaSessionAMD) -> ResultCode;
/// [`vkGetGpaSessionStatusAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaSessionStatusAMD.html)
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
#[inline(always)]
pub unsafe fn get_gpa_session_status_amd(
    device: DeviceHandle,
    gpa_session: GpaSessionAMD,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetGpaSessionResultsAMD =
    unsafe extern "C" fn(DeviceHandle, GpaSessionAMD, u32, *mut usize, *mut c_void) -> ResultCode;
/// [`vkGetGpaSessionResultsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaSessionResultsAMD.html)
///
/// # Optional parameters
/// - data
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
#[inline(always)]
pub unsafe fn get_gpa_session_results_amd(
    device: DeviceHandle,
    gpa_session: GpaSessionAMD,
    sample_id: u32,
    size_in_bytes: *mut usize,
    data: *mut c_void,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_ResetGpaSessionAMD =
    unsafe extern "C" fn(DeviceHandle, GpaSessionAMD) -> ResultCode;
/// [`vkResetGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetGpaSessionAMD.html)
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
#[inline(always)]
pub unsafe fn reset_gpa_session_amd(
    device: DeviceHandle,
    gpa_session: GpaSessionAMD,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdCopyGpaSessionResultsAMD =
    unsafe extern "C" fn(CommandBufferHandle, GpaSessionAMD);
/// [`vkCmdCopyGpaSessionResultsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyGpaSessionResultsAMD.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`TRANSFER`](QueueFlags::TRANSFER)
///
#[doc(alias = "vkCmdCopyGpaSessionResultsAMD")]
#[inline(always)]
pub unsafe fn cmd_copy_gpa_session_results_amd(
    command_buffer: CommandBufferHandle,
    gpa_session: GpaSessionAMD,
) {
    todo!()
}

pub(crate) type FUN_CreateExecutionGraphPipelinesAMDX = unsafe extern "C" fn(
    DeviceHandle,
    PipelineCache,
    u32,
    *const ExecutionGraphPipelineCreateInfoAMDX,
    *const AllocationCallbacks,
    *mut Pipeline,
) -> ResultCode;
/// [`vkCreateExecutionGraphPipelinesAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateExecutionGraphPipelinesAMDX.html)
///
/// # Optional parameters
/// - pipeline_cache
/// - allocator
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
#[inline(always)]
pub unsafe fn create_execution_graph_pipelines_amdx(
    device: DeviceHandle,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    create_infos: *const ExecutionGraphPipelineCreateInfoAMDX,
    allocator: *const AllocationCallbacks,
    pipelines: *mut Pipeline,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetExecutionGraphPipelineScratchSizeAMDX = unsafe extern "C" fn(
    DeviceHandle,
    Pipeline,
    *mut ExecutionGraphPipelineScratchSizeAMDX,
) -> ResultCode;
/// [`vkGetExecutionGraphPipelineScratchSizeAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExecutionGraphPipelineScratchSizeAMDX.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkGetExecutionGraphPipelineScratchSizeAMDX")]
#[inline(always)]
pub unsafe fn get_execution_graph_pipeline_scratch_size_amdx(
    device: DeviceHandle,
    execution_graph: Pipeline,
    size_info: *mut ExecutionGraphPipelineScratchSizeAMDX,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetExecutionGraphPipelineNodeIndexAMDX = unsafe extern "C" fn(
    DeviceHandle,
    Pipeline,
    *const PipelineShaderStageNodeCreateInfoAMDX,
    *mut u32,
) -> ResultCode;
/// [`vkGetExecutionGraphPipelineNodeIndexAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExecutionGraphPipelineNodeIndexAMDX.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkGetExecutionGraphPipelineNodeIndexAMDX")]
#[inline(always)]
pub unsafe fn get_execution_graph_pipeline_node_index_amdx(
    device: DeviceHandle,
    execution_graph: Pipeline,
    node_info: *const PipelineShaderStageNodeCreateInfoAMDX,
    node_index: *mut u32,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdInitializeGraphScratchMemoryAMDX =
    unsafe extern "C" fn(CommandBufferHandle, Pipeline, DeviceAddress, DeviceSize);
/// [`vkCmdInitializeGraphScratchMemoryAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdInitializeGraphScratchMemoryAMDX.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdInitializeGraphScratchMemoryAMDX")]
#[inline(always)]
pub unsafe fn cmd_initialize_graph_scratch_memory_amdx(
    command_buffer: CommandBufferHandle,
    execution_graph: Pipeline,
    scratch: DeviceAddress,
    scratch_size: DeviceSize,
) {
    todo!()
}

pub(crate) type FUN_CmdDispatchGraphAMDX = unsafe extern "C" fn(
    CommandBufferHandle,
    DeviceAddress,
    DeviceSize,
    *const DispatchGraphCountInfoAMDX,
);
/// [`vkCmdDispatchGraphAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphAMDX.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdDispatchGraphAMDX")]
#[inline(always)]
pub unsafe fn cmd_dispatch_graph_amdx(
    command_buffer: CommandBufferHandle,
    scratch: DeviceAddress,
    scratch_size: DeviceSize,
    count_info: *const DispatchGraphCountInfoAMDX,
) {
    todo!()
}

pub(crate) type FUN_CmdDispatchGraphIndirectAMDX = unsafe extern "C" fn(
    CommandBufferHandle,
    DeviceAddress,
    DeviceSize,
    *const DispatchGraphCountInfoAMDX,
);
/// [`vkCmdDispatchGraphIndirectAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphIndirectAMDX.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdDispatchGraphIndirectAMDX")]
#[inline(always)]
pub unsafe fn cmd_dispatch_graph_indirect_amdx(
    command_buffer: CommandBufferHandle,
    scratch: DeviceAddress,
    scratch_size: DeviceSize,
    count_info: *const DispatchGraphCountInfoAMDX,
) {
    todo!()
}

pub(crate) type FUN_CmdDispatchGraphIndirectCountAMDX =
    unsafe extern "C" fn(CommandBufferHandle, DeviceAddress, DeviceSize, DeviceAddress);
/// [`vkCmdDispatchGraphIndirectCountAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphIndirectCountAMDX.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdDispatchGraphIndirectCountAMDX")]
#[inline(always)]
pub unsafe fn cmd_dispatch_graph_indirect_count_amdx(
    command_buffer: CommandBufferHandle,
    scratch: DeviceAddress,
    scratch_size: DeviceSize,
    count_info: DeviceAddress,
) {
    todo!()
}

pub(crate) type FUN_WriteSamplerDescriptorsEXT = unsafe extern "C" fn(
    DeviceHandle,
    u32,
    *const SamplerCreateInfo,
    *const HostAddressRangeEXT,
) -> ResultCode;
/// [`vkWriteSamplerDescriptorsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteSamplerDescriptorsEXT.html)
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
#[inline(always)]
pub unsafe fn write_sampler_descriptors_ext(
    device: DeviceHandle,
    sampler_count: u32,
    samplers: *const SamplerCreateInfo,
    descriptors: *const HostAddressRangeEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_WriteResourceDescriptorsEXT = unsafe extern "C" fn(
    DeviceHandle,
    u32,
    *const ResourceDescriptorInfoEXT,
    *const HostAddressRangeEXT,
) -> ResultCode;
/// [`vkWriteResourceDescriptorsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteResourceDescriptorsEXT.html)
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
#[inline(always)]
pub unsafe fn write_resource_descriptors_ext(
    device: DeviceHandle,
    resource_count: u32,
    resources: *const ResourceDescriptorInfoEXT,
    descriptors: *const HostAddressRangeEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdBindSamplerHeapEXT =
    unsafe extern "C" fn(CommandBufferHandle, *const BindHeapInfoEXT);
/// [`vkCmdBindSamplerHeapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindSamplerHeapEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdBindSamplerHeapEXT")]
#[inline(always)]
pub unsafe fn cmd_bind_sampler_heap_ext(
    command_buffer: CommandBufferHandle,
    bind_info: *const BindHeapInfoEXT,
) {
    todo!()
}

pub(crate) type FUN_CmdBindResourceHeapEXT =
    unsafe extern "C" fn(CommandBufferHandle, *const BindHeapInfoEXT);
/// [`vkCmdBindResourceHeapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindResourceHeapEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdBindResourceHeapEXT")]
#[inline(always)]
pub unsafe fn cmd_bind_resource_heap_ext(
    command_buffer: CommandBufferHandle,
    bind_info: *const BindHeapInfoEXT,
) {
    todo!()
}

pub(crate) type FUN_CmdPushDataEXT =
    unsafe extern "C" fn(CommandBufferHandle, *const PushDataInfoEXT);
/// [`vkCmdPushDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDataEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdPushDataEXT")]
#[inline(always)]
pub unsafe fn cmd_push_data_ext(
    command_buffer: CommandBufferHandle,
    push_data_info: *const PushDataInfoEXT,
) {
    todo!()
}

pub(crate) type FUN_GetImageOpaqueCaptureDataEXT =
    unsafe extern "C" fn(DeviceHandle, u32, *const Image, *mut HostAddressRangeEXT) -> ResultCode;
/// [`vkGetImageOpaqueCaptureDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageOpaqueCaptureDataEXT.html)
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
#[inline(always)]
pub unsafe fn get_image_opaque_capture_data_ext(
    device: DeviceHandle,
    image_count: u32,
    images: *const Image,
    datas: *mut HostAddressRangeEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceDescriptorSizeEXT =
    unsafe extern "C" fn(PhysicalDeviceHandle, DescriptorType) -> DeviceSize;
/// [`vkGetPhysicalDeviceDescriptorSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDescriptorSizeEXT.html)
///
#[doc(alias = "vkGetPhysicalDeviceDescriptorSizeEXT")]
#[inline(always)]
pub unsafe fn get_physical_device_descriptor_size_ext(
    physical_device: PhysicalDeviceHandle,
    descriptor_type: DescriptorType,
) -> DeviceSize {
    todo!()
}

pub(crate) type FUN_RegisterCustomBorderColorEXT = unsafe extern "C" fn(
    DeviceHandle,
    *const SamplerCustomBorderColorCreateInfoEXT,
    Bool32,
    *mut u32,
) -> ResultCode;
/// [`vkRegisterCustomBorderColorEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterCustomBorderColorEXT.html)
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
#[inline(always)]
pub unsafe fn register_custom_border_color_ext(
    device: DeviceHandle,
    border_color: *const SamplerCustomBorderColorCreateInfoEXT,
    request_index: Bool32,
    index: *mut u32,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_UnregisterCustomBorderColorEXT = unsafe extern "C" fn(DeviceHandle, u32);
/// [`vkUnregisterCustomBorderColorEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnregisterCustomBorderColorEXT.html)
///
#[doc(alias = "vkUnregisterCustomBorderColorEXT")]
#[inline(always)]
pub unsafe fn unregister_custom_border_color_ext(device: DeviceHandle, index: u32) {
    todo!()
}

pub(crate) type FUN_GetTensorOpaqueCaptureDataARM = unsafe extern "C" fn(
    DeviceHandle,
    u32,
    *const TensorARM,
    *mut HostAddressRangeEXT,
) -> ResultCode;
/// [`vkGetTensorOpaqueCaptureDataARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDataARM.html)
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
#[inline(always)]
pub unsafe fn get_tensor_opaque_capture_data_arm(
    device: DeviceHandle,
    tensor_count: u32,
    tensors: *const TensorARM,
    datas: *mut HostAddressRangeEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdSetSampleLocationsEXT =
    unsafe extern "C" fn(CommandBufferHandle, *const SampleLocationsInfoEXT);
/// [`vkCmdSetSampleLocationsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleLocationsEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetSampleLocationsEXT")]
#[inline(always)]
pub unsafe fn cmd_set_sample_locations_ext(
    command_buffer: CommandBufferHandle,
    sample_locations_info: *const SampleLocationsInfoEXT,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceMultisamplePropertiesEXT =
    unsafe extern "C" fn(PhysicalDeviceHandle, SampleCountFlags, *mut MultisamplePropertiesEXT);
/// [`vkGetPhysicalDeviceMultisamplePropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMultisamplePropertiesEXT.html)
///
#[doc(alias = "vkGetPhysicalDeviceMultisamplePropertiesEXT")]
#[inline(always)]
pub unsafe fn get_physical_device_multisample_properties_ext(
    physical_device: PhysicalDeviceHandle,
    samples: SampleCountFlags,
    multisample_properties: *mut MultisamplePropertiesEXT,
) {
    todo!()
}

pub(crate) type FUN_GetImageDrmFormatModifierPropertiesEXT = unsafe extern "C" fn(
    DeviceHandle,
    Image,
    *mut ImageDrmFormatModifierPropertiesEXT,
) -> ResultCode;
/// [`vkGetImageDrmFormatModifierPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageDrmFormatModifierPropertiesEXT.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkGetImageDrmFormatModifierPropertiesEXT")]
#[inline(always)]
pub unsafe fn get_image_drm_format_modifier_properties_ext(
    device: DeviceHandle,
    image: Image,
    properties: *mut ImageDrmFormatModifierPropertiesEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateValidationCacheEXT = unsafe extern "C" fn(
    DeviceHandle,
    *const ValidationCacheCreateInfoEXT,
    *const AllocationCallbacks,
    *mut ValidationCacheEXT,
) -> ResultCode;
/// [`vkCreateValidationCacheEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateValidationCacheEXT.html)
///
/// # Optional parameters
/// - allocator
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkCreateValidationCacheEXT")]
#[inline(always)]
pub unsafe fn create_validation_cache_ext(
    device: DeviceHandle,
    create_info: *const ValidationCacheCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    validation_cache: *mut ValidationCacheEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyValidationCacheEXT =
    unsafe extern "C" fn(DeviceHandle, ValidationCacheEXT, *const AllocationCallbacks);
/// [`vkDestroyValidationCacheEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyValidationCacheEXT.html)
///
/// # Optional parameters
/// - validation_cache
/// - allocator
///
#[doc(alias = "vkDestroyValidationCacheEXT")]
#[inline(always)]
pub unsafe fn destroy_validation_cache_ext(
    device: DeviceHandle,
    validation_cache: ValidationCacheEXT,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_MergeValidationCachesEXT = unsafe extern "C" fn(
    DeviceHandle,
    ValidationCacheEXT,
    u32,
    *const ValidationCacheEXT,
) -> ResultCode;
/// [`vkMergeValidationCachesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMergeValidationCachesEXT.html)
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
#[inline(always)]
pub unsafe fn merge_validation_caches_ext(
    device: DeviceHandle,
    dst_cache: ValidationCacheEXT,
    src_cache_count: u32,
    src_caches: *const ValidationCacheEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetValidationCacheDataEXT =
    unsafe extern "C" fn(DeviceHandle, ValidationCacheEXT, *mut usize, *mut c_void) -> ResultCode;
/// [`vkGetValidationCacheDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetValidationCacheDataEXT.html)
///
/// # Optional parameters
/// - data
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
#[inline(always)]
pub unsafe fn get_validation_cache_data_ext(
    device: DeviceHandle,
    validation_cache: ValidationCacheEXT,
    data_size: *mut usize,
    data: *mut c_void,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdBindShadingRateImageNV =
    unsafe extern "C" fn(CommandBufferHandle, ImageView, ImageLayout);
/// [`vkCmdBindShadingRateImageNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindShadingRateImageNV.html)
///
/// # Optional parameters
/// - image_view
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdBindShadingRateImageNV")]
#[inline(always)]
pub unsafe fn cmd_bind_shading_rate_image_nv(
    command_buffer: CommandBufferHandle,
    image_view: ImageView,
    image_layout: ImageLayout,
) {
    todo!()
}

pub(crate) type FUN_CmdSetViewportShadingRatePaletteNV =
    unsafe extern "C" fn(CommandBufferHandle, u32, u32, *const ShadingRatePaletteNV);
/// [`vkCmdSetViewportShadingRatePaletteNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportShadingRatePaletteNV.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetViewportShadingRatePaletteNV")]
#[inline(always)]
pub unsafe fn cmd_set_viewport_shading_rate_palette_nv(
    command_buffer: CommandBufferHandle,
    first_viewport: u32,
    viewport_count: u32,
    shading_rate_palettes: *const ShadingRatePaletteNV,
) {
    todo!()
}

pub(crate) type FUN_CmdSetCoarseSampleOrderNV = unsafe extern "C" fn(
    CommandBufferHandle,
    CoarseSampleOrderTypeNV,
    u32,
    *const CoarseSampleOrderCustomNV,
);
/// [`vkCmdSetCoarseSampleOrderNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoarseSampleOrderNV.html)
///
/// # Optional parameters
/// - custom_sample_order_count
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetCoarseSampleOrderNV")]
#[inline(always)]
pub unsafe fn cmd_set_coarse_sample_order_nv(
    command_buffer: CommandBufferHandle,
    sample_order_type: CoarseSampleOrderTypeNV,
    custom_sample_order_count: u32,
    custom_sample_orders: *const CoarseSampleOrderCustomNV,
) {
    todo!()
}

pub(crate) type FUN_CreateAccelerationStructureNV = unsafe extern "C" fn(
    DeviceHandle,
    *const AccelerationStructureCreateInfoNV,
    *const AllocationCallbacks,
    *mut AccelerationStructureNV,
) -> ResultCode;
/// [`vkCreateAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructureNV.html)
///
/// # Optional parameters
/// - allocator
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkCreateAccelerationStructureNV")]
#[inline(always)]
pub unsafe fn create_acceleration_structure_nv(
    device: DeviceHandle,
    create_info: *const AccelerationStructureCreateInfoNV,
    allocator: *const AllocationCallbacks,
    acceleration_structure: *mut AccelerationStructureNV,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyAccelerationStructureNV =
    unsafe extern "C" fn(DeviceHandle, AccelerationStructureNV, *const AllocationCallbacks);
/// [`vkDestroyAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureNV.html)
///
/// # Optional parameters
/// - acceleration_structure
/// - allocator
///
#[doc(alias = "vkDestroyAccelerationStructureNV")]
#[inline(always)]
pub unsafe fn destroy_acceleration_structure_nv(
    device: DeviceHandle,
    acceleration_structure: AccelerationStructureNV,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_GetAccelerationStructureMemoryRequirementsNV = unsafe extern "C" fn(
    DeviceHandle,
    *const AccelerationStructureMemoryRequirementsInfoNV,
    *mut MemoryRequirements2,
);
/// [`vkGetAccelerationStructureMemoryRequirementsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureMemoryRequirementsNV.html)
///
#[doc(alias = "vkGetAccelerationStructureMemoryRequirementsNV")]
#[inline(always)]
pub unsafe fn get_acceleration_structure_memory_requirements_nv(
    device: DeviceHandle,
    info: *const AccelerationStructureMemoryRequirementsInfoNV,
    memory_requirements: *mut MemoryRequirements2,
) {
    todo!()
}

pub(crate) type FUN_BindAccelerationStructureMemoryNV = unsafe extern "C" fn(
    DeviceHandle,
    u32,
    *const BindAccelerationStructureMemoryInfoNV,
) -> ResultCode;
/// [`vkBindAccelerationStructureMemoryNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindAccelerationStructureMemoryNV.html)
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
#[inline(always)]
pub unsafe fn bind_acceleration_structure_memory_nv(
    device: DeviceHandle,
    bind_info_count: u32,
    bind_infos: *const BindAccelerationStructureMemoryInfoNV,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdBuildAccelerationStructureNV = unsafe extern "C" fn(
    CommandBufferHandle,
    *const AccelerationStructureInfoNV,
    Buffer,
    DeviceSize,
    Bool32,
    AccelerationStructureNV,
    AccelerationStructureNV,
    Buffer,
    DeviceSize,
);
/// [`vkCmdBuildAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructureNV.html)
///
/// # Optional parameters
/// - instance_data
/// - src
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdBuildAccelerationStructureNV")]
#[inline(always)]
pub unsafe fn cmd_build_acceleration_structure_nv(
    command_buffer: CommandBufferHandle,
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

pub(crate) type FUN_CmdCopyAccelerationStructureNV = unsafe extern "C" fn(
    CommandBufferHandle,
    AccelerationStructureNV,
    AccelerationStructureNV,
    CopyAccelerationStructureModeKHR,
);
/// [`vkCmdCopyAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureNV.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyAccelerationStructureNV")]
#[inline(always)]
pub unsafe fn cmd_copy_acceleration_structure_nv(
    command_buffer: CommandBufferHandle,
    dst: AccelerationStructureNV,
    src: AccelerationStructureNV,
    mode: CopyAccelerationStructureModeKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdTraceRaysNV = unsafe extern "C" fn(
    CommandBufferHandle,
    Buffer,
    DeviceSize,
    Buffer,
    DeviceSize,
    DeviceSize,
    Buffer,
    DeviceSize,
    DeviceSize,
    Buffer,
    DeviceSize,
    DeviceSize,
    u32,
    u32,
    u32,
);
/// [`vkCmdTraceRaysNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysNV.html)
///
/// # Optional parameters
/// - miss_shader_binding_table_buffer
/// - hit_shader_binding_table_buffer
/// - callable_shader_binding_table_buffer
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdTraceRaysNV")]
#[inline(always)]
pub unsafe fn cmd_trace_rays_nv(
    command_buffer: CommandBufferHandle,
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

pub(crate) type FUN_CreateRayTracingPipelinesNV = unsafe extern "C" fn(
    DeviceHandle,
    PipelineCache,
    u32,
    *const RayTracingPipelineCreateInfoNV,
    *const AllocationCallbacks,
    *mut Pipeline,
) -> ResultCode;
/// [`vkCreateRayTracingPipelinesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRayTracingPipelinesNV.html)
///
/// # Optional parameters
/// - pipeline_cache
/// - allocator
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
#[inline(always)]
pub unsafe fn create_ray_tracing_pipelines_nv(
    device: DeviceHandle,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    create_infos: *const RayTracingPipelineCreateInfoNV,
    allocator: *const AllocationCallbacks,
    pipelines: *mut Pipeline,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetRayTracingShaderGroupHandlesKHR =
    unsafe extern "C" fn(DeviceHandle, Pipeline, u32, u32, usize, *mut c_void) -> ResultCode;
/// [`vkGetRayTracingShaderGroupHandlesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupHandlesKHR.html)
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
#[inline(always)]
pub unsafe fn get_ray_tracing_shader_group_handles_khr(
    device: DeviceHandle,
    pipeline: Pipeline,
    first_group: u32,
    group_count: u32,
    data_size: usize,
    data: *mut c_void,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetRayTracingShaderGroupHandlesNV =
    unsafe extern "C" fn(DeviceHandle, Pipeline, u32, u32, usize, *mut c_void) -> ResultCode;
/// [`vkGetRayTracingShaderGroupHandlesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupHandlesNV.html)
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
#[inline(always)]
pub unsafe fn get_ray_tracing_shader_group_handles_nv(
    device: DeviceHandle,
    pipeline: Pipeline,
    first_group: u32,
    group_count: u32,
    data_size: usize,
    data: *mut c_void,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetAccelerationStructureHandleNV =
    unsafe extern "C" fn(DeviceHandle, AccelerationStructureNV, usize, *mut c_void) -> ResultCode;
/// [`vkGetAccelerationStructureHandleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureHandleNV.html)
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
#[inline(always)]
pub unsafe fn get_acceleration_structure_handle_nv(
    device: DeviceHandle,
    acceleration_structure: AccelerationStructureNV,
    data_size: usize,
    data: *mut c_void,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdWriteAccelerationStructuresPropertiesNV = unsafe extern "C" fn(
    CommandBufferHandle,
    u32,
    *const AccelerationStructureNV,
    QueryType,
    QueryPool,
    u32,
);
/// [`vkCmdWriteAccelerationStructuresPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteAccelerationStructuresPropertiesNV.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesNV")]
#[inline(always)]
pub unsafe fn cmd_write_acceleration_structures_properties_nv(
    command_buffer: CommandBufferHandle,
    acceleration_structure_count: u32,
    acceleration_structures: *const AccelerationStructureNV,
    query_type: QueryType,
    query_pool: QueryPool,
    first_query: u32,
) {
    todo!()
}

pub(crate) type FUN_CompileDeferredNV =
    unsafe extern "C" fn(DeviceHandle, Pipeline, u32) -> ResultCode;
/// [`vkCompileDeferredNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCompileDeferredNV.html)
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
#[inline(always)]
pub unsafe fn compile_deferred_nv(
    device: DeviceHandle,
    pipeline: Pipeline,
    shader: u32,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetMemoryHostPointerPropertiesEXT = unsafe extern "C" fn(
    DeviceHandle,
    ExternalMemoryHandleTypeFlags,
    *const c_void,
    *mut MemoryHostPointerPropertiesEXT,
) -> ResultCode;
/// [`vkGetMemoryHostPointerPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryHostPointerPropertiesEXT.html)
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
#[inline(always)]
pub unsafe fn get_memory_host_pointer_properties_ext(
    device: DeviceHandle,
    handle_type: ExternalMemoryHandleTypeFlags,
    host_pointer: *const c_void,
    memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdWriteBufferMarkerAMD =
    unsafe extern "C" fn(CommandBufferHandle, PipelineStageFlags, Buffer, DeviceSize, u32);
/// [`vkCmdWriteBufferMarkerAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteBufferMarkerAMD.html)
///
/// # Optional parameters
/// - pipeline_stage
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdWriteBufferMarkerAMD")]
#[inline(always)]
pub unsafe fn cmd_write_buffer_marker_amd(
    command_buffer: CommandBufferHandle,
    pipeline_stage: PipelineStageFlags,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    marker: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdWriteBufferMarker2AMD =
    unsafe extern "C" fn(CommandBufferHandle, PipelineStageFlags2, Buffer, DeviceSize, u32);
/// [`vkCmdWriteBufferMarker2AMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteBufferMarker2AMD.html)
///
/// # Optional parameters
/// - stage
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdWriteBufferMarker2AMD")]
#[inline(always)]
pub unsafe fn cmd_write_buffer_marker_2_amd(
    command_buffer: CommandBufferHandle,
    stage: PipelineStageFlags2,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    marker: u32,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceCalibrateableTimeDomainsEXT =
    unsafe extern "C" fn(PhysicalDeviceHandle, *mut u32, *mut TimeDomainKHR) -> ResultCode;
/// [`vkGetPhysicalDeviceCalibrateableTimeDomainsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html)
///
/// # Optional parameters
/// - time_domains
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
#[inline(always)]
pub unsafe fn get_physical_device_calibrateable_time_domains_ext(
    physical_device: PhysicalDeviceHandle,
    time_domain_count: *mut u32,
    time_domains: *mut TimeDomainKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetCalibratedTimestampsEXT = unsafe extern "C" fn(
    DeviceHandle,
    u32,
    *const CalibratedTimestampInfoKHR,
    *mut u64,
    *mut u64,
) -> ResultCode;
/// [`vkGetCalibratedTimestampsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCalibratedTimestampsEXT.html)
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
#[inline(always)]
pub unsafe fn get_calibrated_timestamps_ext(
    device: DeviceHandle,
    timestamp_count: u32,
    timestamp_infos: *const CalibratedTimestampInfoKHR,
    timestamps: *mut u64,
    max_deviation: *mut u64,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdDrawMeshTasksNV = unsafe extern "C" fn(CommandBufferHandle, u32, u32);
/// [`vkCmdDrawMeshTasksNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksNV.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawMeshTasksNV")]
#[inline(always)]
pub unsafe fn cmd_draw_mesh_tasks_nv(
    command_buffer: CommandBufferHandle,
    task_count: u32,
    first_task: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdDrawMeshTasksIndirectNV =
    unsafe extern "C" fn(CommandBufferHandle, Buffer, DeviceSize, u32, u32);
/// [`vkCmdDrawMeshTasksIndirectNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectNV.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawMeshTasksIndirectNV")]
#[inline(always)]
pub unsafe fn cmd_draw_mesh_tasks_indirect_nv(
    command_buffer: CommandBufferHandle,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdDrawMeshTasksIndirectCountNV =
    unsafe extern "C" fn(CommandBufferHandle, Buffer, DeviceSize, Buffer, DeviceSize, u32, u32);
/// [`vkCmdDrawMeshTasksIndirectCountNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCountNV.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawMeshTasksIndirectCountNV")]
#[inline(always)]
pub unsafe fn cmd_draw_mesh_tasks_indirect_count_nv(
    command_buffer: CommandBufferHandle,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetExclusiveScissorEnableNV =
    unsafe extern "C" fn(CommandBufferHandle, u32, u32, *const Bool32);
/// [`vkCmdSetExclusiveScissorEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExclusiveScissorEnableNV.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetExclusiveScissorEnableNV")]
#[inline(always)]
pub unsafe fn cmd_set_exclusive_scissor_enable_nv(
    command_buffer: CommandBufferHandle,
    first_exclusive_scissor: u32,
    exclusive_scissor_count: u32,
    exclusive_scissor_enables: *const Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetExclusiveScissorNV =
    unsafe extern "C" fn(CommandBufferHandle, u32, u32, *const Rect2D);
/// [`vkCmdSetExclusiveScissorNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExclusiveScissorNV.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetExclusiveScissorNV")]
#[inline(always)]
pub unsafe fn cmd_set_exclusive_scissor_nv(
    command_buffer: CommandBufferHandle,
    first_exclusive_scissor: u32,
    exclusive_scissor_count: u32,
    exclusive_scissors: *const Rect2D,
) {
    todo!()
}

pub(crate) type FUN_CmdSetCheckpointNV = unsafe extern "C" fn(CommandBufferHandle, *const c_void);
/// [`vkCmdSetCheckpointNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCheckpointNV.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`TRANSFER`](QueueFlags::TRANSFER)
///
#[doc(alias = "vkCmdSetCheckpointNV")]
#[inline(always)]
pub unsafe fn cmd_set_checkpoint_nv(
    command_buffer: CommandBufferHandle,
    checkpoint_marker: *const c_void,
) {
    todo!()
}

pub(crate) type FUN_GetQueueCheckpointDataNV =
    unsafe extern "C" fn(QueueHandle, *mut u32, *mut CheckpointDataNV);
/// [`vkGetQueueCheckpointDataNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointDataNV.html)
///
/// # Optional parameters
/// - checkpoint_data
///
#[doc(alias = "vkGetQueueCheckpointDataNV")]
#[inline(always)]
pub unsafe fn get_queue_checkpoint_data_nv(
    queue: QueueHandle,
    checkpoint_data_count: *mut u32,
    checkpoint_data: *mut CheckpointDataNV,
) {
    todo!()
}

pub(crate) type FUN_GetQueueCheckpointData2NV =
    unsafe extern "C" fn(QueueHandle, *mut u32, *mut CheckpointData2NV);
/// [`vkGetQueueCheckpointData2NV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointData2NV.html)
///
/// # Optional parameters
/// - checkpoint_data
///
#[doc(alias = "vkGetQueueCheckpointData2NV")]
#[inline(always)]
pub unsafe fn get_queue_checkpoint_data_2_nv(
    queue: QueueHandle,
    checkpoint_data_count: *mut u32,
    checkpoint_data: *mut CheckpointData2NV,
) {
    todo!()
}

pub(crate) type FUN_SetSwapchainPresentTimingQueueSizeEXT =
    unsafe extern "C" fn(DeviceHandle, SwapchainKHR, u32) -> ResultCode;
/// [`vkSetSwapchainPresentTimingQueueSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetSwapchainPresentTimingQueueSizeEXT.html)
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
#[inline(always)]
pub unsafe fn set_swapchain_present_timing_queue_size_ext(
    device: DeviceHandle,
    swapchain: SwapchainKHR,
    size: u32,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetSwapchainTimingPropertiesEXT = unsafe extern "C" fn(
    DeviceHandle,
    SwapchainKHR,
    *mut SwapchainTimingPropertiesEXT,
    *mut u64,
) -> ResultCode;
/// [`vkGetSwapchainTimingPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainTimingPropertiesEXT.html)
///
/// # Optional parameters
/// - swapchain_timing_properties_counter
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
#[inline(always)]
pub unsafe fn get_swapchain_timing_properties_ext(
    device: DeviceHandle,
    swapchain: SwapchainKHR,
    swapchain_timing_properties: *mut SwapchainTimingPropertiesEXT,
    swapchain_timing_properties_counter: *mut u64,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetSwapchainTimeDomainPropertiesEXT = unsafe extern "C" fn(
    DeviceHandle,
    SwapchainKHR,
    *mut SwapchainTimeDomainPropertiesEXT,
    *mut u64,
) -> ResultCode;
/// [`vkGetSwapchainTimeDomainPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainTimeDomainPropertiesEXT.html)
///
/// # Optional parameters
/// - time_domains_counter
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
#[inline(always)]
pub unsafe fn get_swapchain_time_domain_properties_ext(
    device: DeviceHandle,
    swapchain: SwapchainKHR,
    swapchain_time_domain_properties: *mut SwapchainTimeDomainPropertiesEXT,
    time_domains_counter: *mut u64,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPastPresentationTimingEXT = unsafe extern "C" fn(
    DeviceHandle,
    *const PastPresentationTimingInfoEXT,
    *mut PastPresentationTimingPropertiesEXT,
) -> ResultCode;
/// [`vkGetPastPresentationTimingEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingEXT.html)
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
#[inline(always)]
pub unsafe fn get_past_presentation_timing_ext(
    device: DeviceHandle,
    past_presentation_timing_info: *const PastPresentationTimingInfoEXT,
    past_presentation_timing_properties: *mut PastPresentationTimingPropertiesEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_InitializePerformanceApiINTEL =
    unsafe extern "C" fn(DeviceHandle, *const InitializePerformanceApiInfoINTEL) -> ResultCode;
/// [`vkInitializePerformanceApiINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkInitializePerformanceApiINTEL.html)
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
#[inline(always)]
pub unsafe fn initialize_performance_api_intel(
    device: DeviceHandle,
    initialize_info: *const InitializePerformanceApiInfoINTEL,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_UninitializePerformanceApiINTEL = unsafe extern "C" fn(DeviceHandle);
/// [`vkUninitializePerformanceApiINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUninitializePerformanceApiINTEL.html)
///
#[doc(alias = "vkUninitializePerformanceApiINTEL")]
#[inline(always)]
pub unsafe fn uninitialize_performance_api_intel(device: DeviceHandle) {
    todo!()
}

pub(crate) type FUN_CmdSetPerformanceMarkerINTEL =
    unsafe extern "C" fn(CommandBufferHandle, *const PerformanceMarkerInfoINTEL) -> ResultCode;
/// [`vkCmdSetPerformanceMarkerINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceMarkerINTEL.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
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
#[inline(always)]
pub unsafe fn cmd_set_performance_marker_intel(
    command_buffer: CommandBufferHandle,
    marker_info: *const PerformanceMarkerInfoINTEL,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdSetPerformanceStreamMarkerINTEL = unsafe extern "C" fn(
    CommandBufferHandle,
    *const PerformanceStreamMarkerInfoINTEL,
) -> ResultCode;
/// [`vkCmdSetPerformanceStreamMarkerINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceStreamMarkerINTEL.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
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
#[inline(always)]
pub unsafe fn cmd_set_performance_stream_marker_intel(
    command_buffer: CommandBufferHandle,
    marker_info: *const PerformanceStreamMarkerInfoINTEL,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdSetPerformanceOverrideINTEL =
    unsafe extern "C" fn(CommandBufferHandle, *const PerformanceOverrideInfoINTEL) -> ResultCode;
/// [`vkCmdSetPerformanceOverrideINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceOverrideINTEL.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
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
#[inline(always)]
pub unsafe fn cmd_set_performance_override_intel(
    command_buffer: CommandBufferHandle,
    override_info: *const PerformanceOverrideInfoINTEL,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_AcquirePerformanceConfigurationINTEL = unsafe extern "C" fn(
    DeviceHandle,
    *const PerformanceConfigurationAcquireInfoINTEL,
    *mut PerformanceConfigurationINTEL,
) -> ResultCode;
/// [`vkAcquirePerformanceConfigurationINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquirePerformanceConfigurationINTEL.html)
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
#[inline(always)]
pub unsafe fn acquire_performance_configuration_intel(
    device: DeviceHandle,
    acquire_info: *const PerformanceConfigurationAcquireInfoINTEL,
    configuration: *mut PerformanceConfigurationINTEL,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_ReleasePerformanceConfigurationINTEL =
    unsafe extern "C" fn(DeviceHandle, PerformanceConfigurationINTEL) -> ResultCode;
/// [`vkReleasePerformanceConfigurationINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleasePerformanceConfigurationINTEL.html)
///
/// # Optional parameters
/// - configuration
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
#[inline(always)]
pub unsafe fn release_performance_configuration_intel(
    device: DeviceHandle,
    configuration: PerformanceConfigurationINTEL,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_QueueSetPerformanceConfigurationINTEL =
    unsafe extern "C" fn(QueueHandle, PerformanceConfigurationINTEL) -> ResultCode;
/// [`vkQueueSetPerformanceConfigurationINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSetPerformanceConfigurationINTEL.html)
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
#[inline(always)]
pub unsafe fn queue_set_performance_configuration_intel(
    queue: QueueHandle,
    configuration: PerformanceConfigurationINTEL,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPerformanceParameterINTEL = unsafe extern "C" fn(
    DeviceHandle,
    PerformanceParameterTypeINTEL,
    *mut PerformanceValueINTEL,
) -> ResultCode;
/// [`vkGetPerformanceParameterINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPerformanceParameterINTEL.html)
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
#[inline(always)]
pub unsafe fn get_performance_parameter_intel(
    device: DeviceHandle,
    parameter: PerformanceParameterTypeINTEL,
    value: *mut PerformanceValueINTEL,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_SetLocalDimmingAMD = unsafe extern "C" fn(DeviceHandle, SwapchainKHR, Bool32);
/// [`vkSetLocalDimmingAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLocalDimmingAMD.html)
///
#[doc(alias = "vkSetLocalDimmingAMD")]
#[inline(always)]
pub unsafe fn set_local_dimming_amd(
    device: DeviceHandle,
    swap_chain: SwapchainKHR,
    local_dimming_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CreateImagePipeSurfaceFUCHSIA = unsafe extern "C" fn(
    InstanceHandle,
    *const ImagePipeSurfaceCreateInfoFUCHSIA,
    *const AllocationCallbacks,
    *mut SurfaceKHR,
) -> ResultCode;
/// [`vkCreateImagePipeSurfaceFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImagePipeSurfaceFUCHSIA.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_image_pipe_surface_fuchsia(
    instance: InstanceHandle,
    create_info: *const ImagePipeSurfaceCreateInfoFUCHSIA,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateMetalSurfaceEXT = unsafe extern "C" fn(
    InstanceHandle,
    *const MetalSurfaceCreateInfoEXT,
    *const AllocationCallbacks,
    *mut SurfaceKHR,
) -> ResultCode;
/// [`vkCreateMetalSurfaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMetalSurfaceEXT.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_metal_surface_ext(
    instance: InstanceHandle,
    create_info: *const MetalSurfaceCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetBufferDeviceAddressEXT =
    unsafe extern "C" fn(DeviceHandle, *const BufferDeviceAddressInfo) -> DeviceAddress;
/// [`vkGetBufferDeviceAddressEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddressEXT.html)
///
#[doc(alias = "vkGetBufferDeviceAddressEXT")]
#[inline(always)]
pub unsafe fn get_buffer_device_address_ext(
    device: DeviceHandle,
    info: *const BufferDeviceAddressInfo,
) -> DeviceAddress {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceToolPropertiesEXT = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *mut u32,
    *mut PhysicalDeviceToolProperties,
) -> ResultCode;
/// [`vkGetPhysicalDeviceToolPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolPropertiesEXT.html)
///
/// # Optional parameters
/// - tool_properties
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
#[inline(always)]
pub unsafe fn get_physical_device_tool_properties_ext(
    physical_device: PhysicalDeviceHandle,
    tool_count: *mut u32,
    tool_properties: *mut PhysicalDeviceToolProperties,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceCooperativeMatrixPropertiesNV =
    unsafe extern "C" fn(
        PhysicalDeviceHandle,
        *mut u32,
        *mut CooperativeMatrixPropertiesNV,
    ) -> ResultCode;
/// [`vkGetPhysicalDeviceCooperativeMatrixPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html)
///
/// # Optional parameters
/// - properties
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
#[inline(always)]
pub unsafe fn get_physical_device_cooperative_matrix_properties_nv(
    physical_device: PhysicalDeviceHandle,
    property_count: *mut u32,
    properties: *mut CooperativeMatrixPropertiesNV,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV =
    unsafe extern "C" fn(
        PhysicalDeviceHandle,
        *mut u32,
        *mut FramebufferMixedSamplesCombinationNV,
    ) -> ResultCode;
/// [`vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html)
///
/// # Optional parameters
/// - combinations
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
#[inline(always)]
pub unsafe fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv(
    physical_device: PhysicalDeviceHandle,
    combination_count: *mut u32,
    combinations: *mut FramebufferMixedSamplesCombinationNV,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceSurfacePresentModes2EXT = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *const PhysicalDeviceSurfaceInfo2KHR,
    *mut u32,
    *mut PresentModeKHR,
) -> ResultCode;
/// [`vkGetPhysicalDeviceSurfacePresentModes2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfacePresentModes2EXT.html)
///
/// # Optional parameters
/// - present_modes
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
#[inline(always)]
pub unsafe fn get_physical_device_surface_present_modes_2_ext(
    physical_device: PhysicalDeviceHandle,
    surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    present_mode_count: *mut u32,
    present_modes: *mut PresentModeKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_AcquireFullScreenExclusiveModeEXT =
    unsafe extern "C" fn(DeviceHandle, SwapchainKHR) -> ResultCode;
/// [`vkAcquireFullScreenExclusiveModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireFullScreenExclusiveModeEXT.html)
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
#[inline(always)]
pub unsafe fn acquire_full_screen_exclusive_mode_ext(
    device: DeviceHandle,
    swapchain: SwapchainKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_ReleaseFullScreenExclusiveModeEXT =
    unsafe extern "C" fn(DeviceHandle, SwapchainKHR) -> ResultCode;
/// [`vkReleaseFullScreenExclusiveModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseFullScreenExclusiveModeEXT.html)
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
#[inline(always)]
pub unsafe fn release_full_screen_exclusive_mode_ext(
    device: DeviceHandle,
    swapchain: SwapchainKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetDeviceGroupSurfacePresentModes2EXT = unsafe extern "C" fn(
    DeviceHandle,
    *const PhysicalDeviceSurfaceInfo2KHR,
    *mut DeviceGroupPresentModeFlagsKHR,
) -> ResultCode;
/// [`vkGetDeviceGroupSurfacePresentModes2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModes2EXT.html)
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
#[inline(always)]
pub unsafe fn get_device_group_surface_present_modes_2_ext(
    device: DeviceHandle,
    surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    modes: *mut DeviceGroupPresentModeFlagsKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateHeadlessSurfaceEXT = unsafe extern "C" fn(
    InstanceHandle,
    *const HeadlessSurfaceCreateInfoEXT,
    *const AllocationCallbacks,
    *mut SurfaceKHR,
) -> ResultCode;
/// [`vkCreateHeadlessSurfaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateHeadlessSurfaceEXT.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_headless_surface_ext(
    instance: InstanceHandle,
    create_info: *const HeadlessSurfaceCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdSetLineStippleEXT = unsafe extern "C" fn(CommandBufferHandle, u32, u16);
/// [`vkCmdSetLineStippleEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetLineStippleEXT")]
#[inline(always)]
pub unsafe fn cmd_set_line_stipple_ext(
    command_buffer: CommandBufferHandle,
    line_stipple_factor: u32,
    line_stipple_pattern: u16,
) {
    todo!()
}

pub(crate) type FUN_ResetQueryPoolEXT = unsafe extern "C" fn(DeviceHandle, QueryPool, u32, u32);
/// [`vkResetQueryPoolEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetQueryPoolEXT.html)
///
#[doc(alias = "vkResetQueryPoolEXT")]
#[inline(always)]
pub unsafe fn reset_query_pool_ext(
    device: DeviceHandle,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetCullModeEXT = unsafe extern "C" fn(CommandBufferHandle, CullModeFlags);
/// [`vkCmdSetCullModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCullModeEXT.html)
///
/// # Optional parameters
/// - cull_mode
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetCullModeEXT")]
#[inline(always)]
pub unsafe fn cmd_set_cull_mode_ext(command_buffer: CommandBufferHandle, cull_mode: CullModeFlags) {
    todo!()
}

pub(crate) type FUN_CmdSetFrontFaceEXT = unsafe extern "C" fn(CommandBufferHandle, FrontFace);
/// [`vkCmdSetFrontFaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFrontFaceEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetFrontFaceEXT")]
#[inline(always)]
pub unsafe fn cmd_set_front_face_ext(command_buffer: CommandBufferHandle, front_face: FrontFace) {
    todo!()
}

pub(crate) type FUN_CmdSetPrimitiveTopologyEXT =
    unsafe extern "C" fn(CommandBufferHandle, PrimitiveTopology);
/// [`vkCmdSetPrimitiveTopologyEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveTopologyEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetPrimitiveTopologyEXT")]
#[inline(always)]
pub unsafe fn cmd_set_primitive_topology_ext(
    command_buffer: CommandBufferHandle,
    primitive_topology: PrimitiveTopology,
) {
    todo!()
}

pub(crate) type FUN_CmdSetViewportWithCountEXT =
    unsafe extern "C" fn(CommandBufferHandle, u32, *const Viewport);
/// [`vkCmdSetViewportWithCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCountEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetViewportWithCountEXT")]
#[inline(always)]
pub unsafe fn cmd_set_viewport_with_count_ext(
    command_buffer: CommandBufferHandle,
    viewport_count: u32,
    viewports: *const Viewport,
) {
    todo!()
}

pub(crate) type FUN_CmdSetScissorWithCountEXT =
    unsafe extern "C" fn(CommandBufferHandle, u32, *const Rect2D);
/// [`vkCmdSetScissorWithCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCountEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetScissorWithCountEXT")]
#[inline(always)]
pub unsafe fn cmd_set_scissor_with_count_ext(
    command_buffer: CommandBufferHandle,
    scissor_count: u32,
    scissors: *const Rect2D,
) {
    todo!()
}

pub(crate) type FUN_CmdBindVertexBuffers2EXT = unsafe extern "C" fn(
    CommandBufferHandle,
    u32,
    u32,
    *const Buffer,
    *const DeviceSize,
    *const DeviceSize,
    *const DeviceSize,
);
/// [`vkCmdBindVertexBuffers2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2EXT.html)
///
/// # Optional parameters
/// - sizes
/// - strides
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdBindVertexBuffers2EXT")]
#[inline(always)]
pub unsafe fn cmd_bind_vertex_buffers_2_ext(
    command_buffer: CommandBufferHandle,
    first_binding: u32,
    binding_count: u32,
    buffers: *const Buffer,
    offsets: *const DeviceSize,
    sizes: *const DeviceSize,
    strides: *const DeviceSize,
) {
    todo!()
}

pub(crate) type FUN_CmdSetDepthTestEnableEXT = unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetDepthTestEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthTestEnableEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetDepthTestEnableEXT")]
#[inline(always)]
pub unsafe fn cmd_set_depth_test_enable_ext(
    command_buffer: CommandBufferHandle,
    depth_test_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetDepthWriteEnableEXT = unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetDepthWriteEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthWriteEnableEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetDepthWriteEnableEXT")]
#[inline(always)]
pub unsafe fn cmd_set_depth_write_enable_ext(
    command_buffer: CommandBufferHandle,
    depth_write_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetDepthCompareOpEXT = unsafe extern "C" fn(CommandBufferHandle, CompareOp);
/// [`vkCmdSetDepthCompareOpEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthCompareOpEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetDepthCompareOpEXT")]
#[inline(always)]
pub unsafe fn cmd_set_depth_compare_op_ext(
    command_buffer: CommandBufferHandle,
    depth_compare_op: CompareOp,
) {
    todo!()
}

pub(crate) type FUN_CmdSetDepthBoundsTestEnableEXT =
    unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetDepthBoundsTestEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBoundsTestEnableEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetDepthBoundsTestEnableEXT")]
#[inline(always)]
pub unsafe fn cmd_set_depth_bounds_test_enable_ext(
    command_buffer: CommandBufferHandle,
    depth_bounds_test_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetStencilTestEnableEXT = unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetStencilTestEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilTestEnableEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetStencilTestEnableEXT")]
#[inline(always)]
pub unsafe fn cmd_set_stencil_test_enable_ext(
    command_buffer: CommandBufferHandle,
    stencil_test_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetStencilOpEXT = unsafe extern "C" fn(
    CommandBufferHandle,
    StencilFaceFlags,
    StencilOp,
    StencilOp,
    StencilOp,
    CompareOp,
);
/// [`vkCmdSetStencilOpEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilOpEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetStencilOpEXT")]
#[inline(always)]
pub unsafe fn cmd_set_stencil_op_ext(
    command_buffer: CommandBufferHandle,
    face_mask: StencilFaceFlags,
    fail_op: StencilOp,
    pass_op: StencilOp,
    depth_fail_op: StencilOp,
    compare_op: CompareOp,
) {
    todo!()
}

pub(crate) type FUN_CopyMemoryToImageEXT =
    unsafe extern "C" fn(DeviceHandle, *const CopyMemoryToImageInfo) -> ResultCode;
/// [`vkCopyMemoryToImageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToImageEXT.html)
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
#[inline(always)]
pub unsafe fn copy_memory_to_image_ext(
    device: DeviceHandle,
    copy_memory_to_image_info: *const CopyMemoryToImageInfo,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CopyImageToMemoryEXT =
    unsafe extern "C" fn(DeviceHandle, *const CopyImageToMemoryInfo) -> ResultCode;
/// [`vkCopyImageToMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToMemoryEXT.html)
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
#[inline(always)]
pub unsafe fn copy_image_to_memory_ext(
    device: DeviceHandle,
    copy_image_to_memory_info: *const CopyImageToMemoryInfo,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CopyImageToImageEXT =
    unsafe extern "C" fn(DeviceHandle, *const CopyImageToImageInfo) -> ResultCode;
/// [`vkCopyImageToImageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToImageEXT.html)
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
#[inline(always)]
pub unsafe fn copy_image_to_image_ext(
    device: DeviceHandle,
    copy_image_to_image_info: *const CopyImageToImageInfo,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_TransitionImageLayoutEXT =
    unsafe extern "C" fn(DeviceHandle, u32, *const HostImageLayoutTransitionInfo) -> ResultCode;
/// [`vkTransitionImageLayoutEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTransitionImageLayoutEXT.html)
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
#[inline(always)]
pub unsafe fn transition_image_layout_ext(
    device: DeviceHandle,
    transition_count: u32,
    transitions: *const HostImageLayoutTransitionInfo,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetImageSubresourceLayout2EXT =
    unsafe extern "C" fn(DeviceHandle, Image, *const ImageSubresource2, *mut SubresourceLayout2);
/// [`vkGetImageSubresourceLayout2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2EXT.html)
///
#[doc(alias = "vkGetImageSubresourceLayout2EXT")]
#[inline(always)]
pub unsafe fn get_image_subresource_layout_2_ext(
    device: DeviceHandle,
    image: Image,
    subresource: *const ImageSubresource2,
    layout: *mut SubresourceLayout2,
) {
    todo!()
}

pub(crate) type FUN_ReleaseSwapchainImagesEXT =
    unsafe extern "C" fn(DeviceHandle, *const ReleaseSwapchainImagesInfoKHR) -> ResultCode;
/// [`vkReleaseSwapchainImagesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseSwapchainImagesEXT.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`SURFACE_LOST_KHR`](ResultCode::ERROR_SURFACE_LOST_KHR)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkReleaseSwapchainImagesEXT")]
#[inline(always)]
pub unsafe fn release_swapchain_images_ext(
    device: DeviceHandle,
    release_info: *const ReleaseSwapchainImagesInfoKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetGeneratedCommandsMemoryRequirementsNV = unsafe extern "C" fn(
    DeviceHandle,
    *const GeneratedCommandsMemoryRequirementsInfoNV,
    *mut MemoryRequirements2,
);
/// [`vkGetGeneratedCommandsMemoryRequirementsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGeneratedCommandsMemoryRequirementsNV.html)
///
#[doc(alias = "vkGetGeneratedCommandsMemoryRequirementsNV")]
#[inline(always)]
pub unsafe fn get_generated_commands_memory_requirements_nv(
    device: DeviceHandle,
    info: *const GeneratedCommandsMemoryRequirementsInfoNV,
    memory_requirements: *mut MemoryRequirements2,
) {
    todo!()
}

pub(crate) type FUN_CmdPreprocessGeneratedCommandsNV =
    unsafe extern "C" fn(CommandBufferHandle, *const GeneratedCommandsInfoNV);
/// [`vkCmdPreprocessGeneratedCommandsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPreprocessGeneratedCommandsNV.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdPreprocessGeneratedCommandsNV")]
#[inline(always)]
pub unsafe fn cmd_preprocess_generated_commands_nv(
    command_buffer: CommandBufferHandle,
    generated_commands_info: *const GeneratedCommandsInfoNV,
) {
    todo!()
}

pub(crate) type FUN_CmdExecuteGeneratedCommandsNV =
    unsafe extern "C" fn(CommandBufferHandle, Bool32, *const GeneratedCommandsInfoNV);
/// [`vkCmdExecuteGeneratedCommandsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteGeneratedCommandsNV.html)
///
/// # Performed tasks
/// - `action`
/// - `indirection`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdExecuteGeneratedCommandsNV")]
#[inline(always)]
pub unsafe fn cmd_execute_generated_commands_nv(
    command_buffer: CommandBufferHandle,
    is_preprocessed: Bool32,
    generated_commands_info: *const GeneratedCommandsInfoNV,
) {
    todo!()
}

pub(crate) type FUN_CmdBindPipelineShaderGroupNV =
    unsafe extern "C" fn(CommandBufferHandle, PipelineBindPoint, Pipeline, u32);
/// [`vkCmdBindPipelineShaderGroupNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindPipelineShaderGroupNV.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdBindPipelineShaderGroupNV")]
#[inline(always)]
pub unsafe fn cmd_bind_pipeline_shader_group_nv(
    command_buffer: CommandBufferHandle,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
    group_index: u32,
) {
    todo!()
}

pub(crate) type FUN_CreateIndirectCommandsLayoutNV = unsafe extern "C" fn(
    DeviceHandle,
    *const IndirectCommandsLayoutCreateInfoNV,
    *const AllocationCallbacks,
    *mut IndirectCommandsLayoutNV,
) -> ResultCode;
/// [`vkCreateIndirectCommandsLayoutNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectCommandsLayoutNV.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_indirect_commands_layout_nv(
    device: DeviceHandle,
    create_info: *const IndirectCommandsLayoutCreateInfoNV,
    allocator: *const AllocationCallbacks,
    indirect_commands_layout: *mut IndirectCommandsLayoutNV,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyIndirectCommandsLayoutNV =
    unsafe extern "C" fn(DeviceHandle, IndirectCommandsLayoutNV, *const AllocationCallbacks);
/// [`vkDestroyIndirectCommandsLayoutNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectCommandsLayoutNV.html)
///
/// # Optional parameters
/// - indirect_commands_layout
/// - allocator
///
#[doc(alias = "vkDestroyIndirectCommandsLayoutNV")]
#[inline(always)]
pub unsafe fn destroy_indirect_commands_layout_nv(
    device: DeviceHandle,
    indirect_commands_layout: IndirectCommandsLayoutNV,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_CmdSetDepthBias2EXT =
    unsafe extern "C" fn(CommandBufferHandle, *const DepthBiasInfoEXT);
/// [`vkCmdSetDepthBias2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias2EXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetDepthBias2EXT")]
#[inline(always)]
pub unsafe fn cmd_set_depth_bias_2_ext(
    command_buffer: CommandBufferHandle,
    depth_bias_info: *const DepthBiasInfoEXT,
) {
    todo!()
}

pub(crate) type FUN_AcquireDrmDisplayEXT =
    unsafe extern "C" fn(PhysicalDeviceHandle, i32, DisplayKHR) -> ResultCode;
/// [`vkAcquireDrmDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireDrmDisplayEXT.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkAcquireDrmDisplayEXT")]
#[inline(always)]
pub unsafe fn acquire_drm_display_ext(
    physical_device: PhysicalDeviceHandle,
    drm_fd: i32,
    display: DisplayKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetDrmDisplayEXT =
    unsafe extern "C" fn(PhysicalDeviceHandle, i32, u32, *mut DisplayKHR) -> ResultCode;
/// [`vkGetDrmDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDrmDisplayEXT.html)
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
#[inline(always)]
pub unsafe fn get_drm_display_ext(
    physical_device: PhysicalDeviceHandle,
    drm_fd: i32,
    connector_id: u32,
    display: *mut DisplayKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreatePrivateDataSlotEXT = unsafe extern "C" fn(
    DeviceHandle,
    *const PrivateDataSlotCreateInfo,
    *const AllocationCallbacks,
    *mut PrivateDataSlot,
) -> ResultCode;
/// [`vkCreatePrivateDataSlotEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePrivateDataSlotEXT.html)
///
/// # Optional parameters
/// - allocator
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkCreatePrivateDataSlotEXT")]
#[inline(always)]
pub unsafe fn create_private_data_slot_ext(
    device: DeviceHandle,
    create_info: *const PrivateDataSlotCreateInfo,
    allocator: *const AllocationCallbacks,
    private_data_slot: *mut PrivateDataSlot,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyPrivateDataSlotEXT =
    unsafe extern "C" fn(DeviceHandle, PrivateDataSlot, *const AllocationCallbacks);
/// [`vkDestroyPrivateDataSlotEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlotEXT.html)
///
/// # Optional parameters
/// - private_data_slot
/// - allocator
///
#[doc(alias = "vkDestroyPrivateDataSlotEXT")]
#[inline(always)]
pub unsafe fn destroy_private_data_slot_ext(
    device: DeviceHandle,
    private_data_slot: PrivateDataSlot,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_SetPrivateDataEXT =
    unsafe extern "C" fn(DeviceHandle, ObjectType, u64, PrivateDataSlot, u64) -> ResultCode;
/// [`vkSetPrivateDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetPrivateDataEXT.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkSetPrivateDataEXT")]
#[inline(always)]
pub unsafe fn set_private_data_ext(
    device: DeviceHandle,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: PrivateDataSlot,
    data: u64,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPrivateDataEXT =
    unsafe extern "C" fn(DeviceHandle, ObjectType, u64, PrivateDataSlot, *mut u64);
/// [`vkGetPrivateDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPrivateDataEXT.html)
///
#[doc(alias = "vkGetPrivateDataEXT")]
#[inline(always)]
pub unsafe fn get_private_data_ext(
    device: DeviceHandle,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: PrivateDataSlot,
    data: *mut u64,
) {
    todo!()
}

pub(crate) type FUN_QueueSetPerfHintQCOM =
    unsafe extern "C" fn(QueueHandle, *const PerfHintInfoQCOM) -> ResultCode;
/// [`vkQueueSetPerfHintQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSetPerfHintQCOM.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`DEVICE_LOST`](ResultCode::ERROR_DEVICE_LOST)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkQueueSetPerfHintQCOM")]
#[inline(always)]
pub unsafe fn queue_set_perf_hint_qcom(
    queue: QueueHandle,
    perf_hint_info: *const PerfHintInfoQCOM,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateCudaModuleNV = unsafe extern "C" fn(
    DeviceHandle,
    *const CudaModuleCreateInfoNV,
    *const AllocationCallbacks,
    *mut CudaModuleNV,
) -> ResultCode;
/// [`vkCreateCudaModuleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCudaModuleNV.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_cuda_module_nv(
    device: DeviceHandle,
    create_info: *const CudaModuleCreateInfoNV,
    allocator: *const AllocationCallbacks,
    module: *mut CudaModuleNV,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetCudaModuleCacheNV =
    unsafe extern "C" fn(DeviceHandle, CudaModuleNV, *mut usize, *mut c_void) -> ResultCode;
/// [`vkGetCudaModuleCacheNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCudaModuleCacheNV.html)
///
/// # Optional parameters
/// - cache_data
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
#[inline(always)]
pub unsafe fn get_cuda_module_cache_nv(
    device: DeviceHandle,
    module: CudaModuleNV,
    cache_size: *mut usize,
    cache_data: *mut c_void,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateCudaFunctionNV = unsafe extern "C" fn(
    DeviceHandle,
    *const CudaFunctionCreateInfoNV,
    *const AllocationCallbacks,
    *mut CudaFunctionNV,
) -> ResultCode;
/// [`vkCreateCudaFunctionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCudaFunctionNV.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_cuda_function_nv(
    device: DeviceHandle,
    create_info: *const CudaFunctionCreateInfoNV,
    allocator: *const AllocationCallbacks,
    function: *mut CudaFunctionNV,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyCudaModuleNV =
    unsafe extern "C" fn(DeviceHandle, CudaModuleNV, *const AllocationCallbacks);
/// [`vkDestroyCudaModuleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCudaModuleNV.html)
///
/// # Optional parameters
/// - allocator
///
#[doc(alias = "vkDestroyCudaModuleNV")]
#[inline(always)]
pub unsafe fn destroy_cuda_module_nv(
    device: DeviceHandle,
    module: CudaModuleNV,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_DestroyCudaFunctionNV =
    unsafe extern "C" fn(DeviceHandle, CudaFunctionNV, *const AllocationCallbacks);
/// [`vkDestroyCudaFunctionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCudaFunctionNV.html)
///
/// # Optional parameters
/// - allocator
///
#[doc(alias = "vkDestroyCudaFunctionNV")]
#[inline(always)]
pub unsafe fn destroy_cuda_function_nv(
    device: DeviceHandle,
    function: CudaFunctionNV,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_CmdCudaLaunchKernelNV =
    unsafe extern "C" fn(CommandBufferHandle, *const CudaLaunchInfoNV);
/// [`vkCmdCudaLaunchKernelNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCudaLaunchKernelNV.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCudaLaunchKernelNV")]
#[inline(always)]
pub unsafe fn cmd_cuda_launch_kernel_nv(
    command_buffer: CommandBufferHandle,
    launch_info: *const CudaLaunchInfoNV,
) {
    todo!()
}

pub(crate) type FUN_CmdDispatchTileQCOM =
    unsafe extern "C" fn(CommandBufferHandle, *const DispatchTileInfoQCOM);
/// [`vkCmdDispatchTileQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchTileQCOM.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdDispatchTileQCOM")]
#[inline(always)]
pub unsafe fn cmd_dispatch_tile_qcom(
    command_buffer: CommandBufferHandle,
    dispatch_tile_info: *const DispatchTileInfoQCOM,
) {
    todo!()
}

pub(crate) type FUN_CmdBeginPerTileExecutionQCOM =
    unsafe extern "C" fn(CommandBufferHandle, *const PerTileBeginInfoQCOM);
/// [`vkCmdBeginPerTileExecutionQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginPerTileExecutionQCOM.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdBeginPerTileExecutionQCOM")]
#[inline(always)]
pub unsafe fn cmd_begin_per_tile_execution_qcom(
    command_buffer: CommandBufferHandle,
    per_tile_begin_info: *const PerTileBeginInfoQCOM,
) {
    todo!()
}

pub(crate) type FUN_CmdEndPerTileExecutionQCOM =
    unsafe extern "C" fn(CommandBufferHandle, *const PerTileEndInfoQCOM);
/// [`vkCmdEndPerTileExecutionQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndPerTileExecutionQCOM.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdEndPerTileExecutionQCOM")]
#[inline(always)]
pub unsafe fn cmd_end_per_tile_execution_qcom(
    command_buffer: CommandBufferHandle,
    per_tile_end_info: *const PerTileEndInfoQCOM,
) {
    todo!()
}

pub(crate) type FUN_SetLatencySleepModeLegacyNV =
    unsafe extern "C" fn(DeviceHandle, Bool32, Bool32, u32);
/// [`vkSetLatencySleepModeLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencySleepModeLegacyNV.html)
///
#[doc(alias = "vkSetLatencySleepModeLegacyNV")]
#[inline(always)]
pub unsafe fn set_latency_sleep_mode_legacy_nv(
    device: DeviceHandle,
    low_latency_mode: Bool32,
    low_latency_boost: Bool32,
    minimum_interval_us: u32,
) {
    todo!()
}

pub(crate) type FUN_LatencySleepLegacyNV = unsafe extern "C" fn(DeviceHandle, Semaphore, u64);
/// [`vkLatencySleepLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkLatencySleepLegacyNV.html)
///
#[doc(alias = "vkLatencySleepLegacyNV")]
#[inline(always)]
pub unsafe fn latency_sleep_legacy_nv(
    device: DeviceHandle,
    signal_semaphore: Semaphore,
    value: u64,
) {
    todo!()
}

pub(crate) type FUN_SetLatencyMarkerLegacyNV = unsafe extern "C" fn(DeviceHandle, u64, u32);
/// [`vkSetLatencyMarkerLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencyMarkerLegacyNV.html)
///
#[doc(alias = "vkSetLatencyMarkerLegacyNV")]
#[inline(always)]
pub unsafe fn set_latency_marker_legacy_nv(device: DeviceHandle, frame_id: u64, marker: u32) {
    todo!()
}

pub(crate) type FUN_GetLatencyTimingsLegacyNV = unsafe extern "C" fn(DeviceHandle, *mut c_void);
/// [`vkGetLatencyTimingsLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetLatencyTimingsLegacyNV.html)
///
#[doc(alias = "vkGetLatencyTimingsLegacyNV")]
#[inline(always)]
pub unsafe fn get_latency_timings_legacy_nv(device: DeviceHandle, timings: *mut c_void) {
    todo!()
}

pub(crate) type FUN_QueueNotifyOutOfBandLegacyNV = unsafe extern "C" fn(QueueHandle, u32);
/// [`vkQueueNotifyOutOfBandLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueNotifyOutOfBandLegacyNV.html)
///
#[doc(alias = "vkQueueNotifyOutOfBandLegacyNV")]
#[inline(always)]
pub unsafe fn queue_notify_out_of_band_legacy_nv(queue: QueueHandle, queue_type: u32) {
    todo!()
}

pub(crate) type FUN_GetSleepStatusLegacyNV = unsafe extern "C" fn(DeviceHandle, *mut Bool32);
/// [`vkGetSleepStatusLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSleepStatusLegacyNV.html)
///
#[doc(alias = "vkGetSleepStatusLegacyNV")]
#[inline(always)]
pub unsafe fn get_sleep_status_legacy_nv(device: DeviceHandle, low_latency_mode: *mut Bool32) {
    todo!()
}

pub(crate) type FUN_ShutdownLatencyDeviceLegacyNV = unsafe extern "C" fn(DeviceHandle);
/// [`vkShutdownLatencyDeviceLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkShutdownLatencyDeviceLegacyNV.html)
///
#[doc(alias = "vkShutdownLatencyDeviceLegacyNV")]
#[inline(always)]
pub unsafe fn shutdown_latency_device_legacy_nv(device: DeviceHandle) {
    todo!()
}

pub(crate) type FUN_ExportMetalObjectsEXT =
    unsafe extern "C" fn(DeviceHandle, *mut ExportMetalObjectsInfoEXT);
/// [`vkExportMetalObjectsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkExportMetalObjectsEXT.html)
///
#[doc(alias = "vkExportMetalObjectsEXT")]
#[inline(always)]
pub unsafe fn export_metal_objects_ext(
    device: DeviceHandle,
    metal_objects_info: *mut ExportMetalObjectsInfoEXT,
) {
    todo!()
}

pub(crate) type FUN_GetDescriptorSetLayoutSizeEXT =
    unsafe extern "C" fn(DeviceHandle, DescriptorSetLayout, *mut DeviceSize);
/// [`vkGetDescriptorSetLayoutSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSizeEXT.html)
///
#[doc(alias = "vkGetDescriptorSetLayoutSizeEXT")]
#[inline(always)]
pub unsafe fn get_descriptor_set_layout_size_ext(
    device: DeviceHandle,
    layout: DescriptorSetLayout,
    layout_size_in_bytes: *mut DeviceSize,
) {
    todo!()
}

pub(crate) type FUN_GetDescriptorSetLayoutBindingOffsetEXT =
    unsafe extern "C" fn(DeviceHandle, DescriptorSetLayout, u32, *mut DeviceSize);
/// [`vkGetDescriptorSetLayoutBindingOffsetEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutBindingOffsetEXT.html)
///
#[doc(alias = "vkGetDescriptorSetLayoutBindingOffsetEXT")]
#[inline(always)]
pub unsafe fn get_descriptor_set_layout_binding_offset_ext(
    device: DeviceHandle,
    layout: DescriptorSetLayout,
    binding: u32,
    offset: *mut DeviceSize,
) {
    todo!()
}

pub(crate) type FUN_GetDescriptorEXT =
    unsafe extern "C" fn(DeviceHandle, *const DescriptorGetInfoEXT, usize, *mut c_void);
/// [`vkGetDescriptorEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorEXT.html)
///
#[doc(alias = "vkGetDescriptorEXT")]
#[inline(always)]
pub unsafe fn get_descriptor_ext(
    device: DeviceHandle,
    descriptor_info: *const DescriptorGetInfoEXT,
    data_size: usize,
    descriptor: *mut c_void,
) {
    todo!()
}

pub(crate) type FUN_CmdBindDescriptorBuffersEXT =
    unsafe extern "C" fn(CommandBufferHandle, u32, *const DescriptorBufferBindingInfoEXT);
/// [`vkCmdBindDescriptorBuffersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBuffersEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`DATA_GRAPHARM`](QueueFlags::DATA_GRAPHARM)
///
#[doc(alias = "vkCmdBindDescriptorBuffersEXT")]
#[inline(always)]
pub unsafe fn cmd_bind_descriptor_buffers_ext(
    command_buffer: CommandBufferHandle,
    buffer_count: u32,
    binding_infos: *const DescriptorBufferBindingInfoEXT,
) {
    todo!()
}

pub(crate) type FUN_CmdSetDescriptorBufferOffsetsEXT = unsafe extern "C" fn(
    CommandBufferHandle,
    PipelineBindPoint,
    PipelineLayout,
    u32,
    u32,
    *const u32,
    *const DeviceSize,
);
/// [`vkCmdSetDescriptorBufferOffsetsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDescriptorBufferOffsetsEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`DATA_GRAPHARM`](QueueFlags::DATA_GRAPHARM)
///
#[doc(alias = "vkCmdSetDescriptorBufferOffsetsEXT")]
#[inline(always)]
pub unsafe fn cmd_set_descriptor_buffer_offsets_ext(
    command_buffer: CommandBufferHandle,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    first_set: u32,
    set_count: u32,
    buffer_indices: *const u32,
    offsets: *const DeviceSize,
) {
    todo!()
}

pub(crate) type FUN_CmdBindDescriptorBufferEmbeddedSamplersEXT =
    unsafe extern "C" fn(CommandBufferHandle, PipelineBindPoint, PipelineLayout, u32);
/// [`vkCmdBindDescriptorBufferEmbeddedSamplersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBufferEmbeddedSamplersEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdBindDescriptorBufferEmbeddedSamplersEXT")]
#[inline(always)]
pub unsafe fn cmd_bind_descriptor_buffer_embedded_samplers_ext(
    command_buffer: CommandBufferHandle,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    set: u32,
) {
    todo!()
}

pub(crate) type FUN_GetBufferOpaqueCaptureDescriptorDataEXT = unsafe extern "C" fn(
    DeviceHandle,
    *const BufferCaptureDescriptorDataInfoEXT,
    *mut c_void,
) -> ResultCode;
/// [`vkGetBufferOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureDescriptorDataEXT.html)
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
#[inline(always)]
pub unsafe fn get_buffer_opaque_capture_descriptor_data_ext(
    device: DeviceHandle,
    info: *const BufferCaptureDescriptorDataInfoEXT,
    data: *mut c_void,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetImageOpaqueCaptureDescriptorDataEXT = unsafe extern "C" fn(
    DeviceHandle,
    *const ImageCaptureDescriptorDataInfoEXT,
    *mut c_void,
) -> ResultCode;
/// [`vkGetImageOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageOpaqueCaptureDescriptorDataEXT.html)
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
#[inline(always)]
pub unsafe fn get_image_opaque_capture_descriptor_data_ext(
    device: DeviceHandle,
    info: *const ImageCaptureDescriptorDataInfoEXT,
    data: *mut c_void,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetImageViewOpaqueCaptureDescriptorDataEXT = unsafe extern "C" fn(
    DeviceHandle,
    *const ImageViewCaptureDescriptorDataInfoEXT,
    *mut c_void,
)
    -> ResultCode;
/// [`vkGetImageViewOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewOpaqueCaptureDescriptorDataEXT.html)
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
#[inline(always)]
pub unsafe fn get_image_view_opaque_capture_descriptor_data_ext(
    device: DeviceHandle,
    info: *const ImageViewCaptureDescriptorDataInfoEXT,
    data: *mut c_void,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetSamplerOpaqueCaptureDescriptorDataEXT = unsafe extern "C" fn(
    DeviceHandle,
    *const SamplerCaptureDescriptorDataInfoEXT,
    *mut c_void,
) -> ResultCode;
/// [`vkGetSamplerOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSamplerOpaqueCaptureDescriptorDataEXT.html)
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
#[inline(always)]
pub unsafe fn get_sampler_opaque_capture_descriptor_data_ext(
    device: DeviceHandle,
    info: *const SamplerCaptureDescriptorDataInfoEXT,
    data: *mut c_void,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetAccelerationStructureOpaqueCaptureDescriptorDataEXT =
    unsafe extern "C" fn(
        DeviceHandle,
        *const AccelerationStructureCaptureDescriptorDataInfoEXT,
        *mut c_void,
    ) -> ResultCode;
/// [`vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT.html)
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
#[inline(always)]
pub unsafe fn get_acceleration_structure_opaque_capture_descriptor_data_ext(
    device: DeviceHandle,
    info: *const AccelerationStructureCaptureDescriptorDataInfoEXT,
    data: *mut c_void,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdSetFragmentShadingRateEnumNV = unsafe extern "C" fn(
    CommandBufferHandle,
    FragmentShadingRateNV,
    *const [FragmentShadingRateCombinerOpKHR; 2 as usize],
);
/// [`vkCmdSetFragmentShadingRateEnumNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFragmentShadingRateEnumNV.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetFragmentShadingRateEnumNV")]
#[inline(always)]
pub unsafe fn cmd_set_fragment_shading_rate_enum_nv(
    command_buffer: CommandBufferHandle,
    shading_rate: FragmentShadingRateNV,
    combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2 as usize],
) {
    todo!()
}

pub(crate) type FUN_GetDeviceFaultInfoEXT = unsafe extern "C" fn(
    DeviceHandle,
    *mut DeviceFaultCountsEXT,
    *mut DeviceFaultInfoEXT,
) -> ResultCode;
/// [`vkGetDeviceFaultInfoEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultInfoEXT.html)
///
/// # Optional parameters
/// - fault_info
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
#[inline(always)]
pub unsafe fn get_device_fault_info_ext(
    device: DeviceHandle,
    fault_counts: *mut DeviceFaultCountsEXT,
    fault_info: *mut DeviceFaultInfoEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_AcquireWinrtDisplayNV =
    unsafe extern "C" fn(PhysicalDeviceHandle, DisplayKHR) -> ResultCode;
/// [`vkAcquireWinrtDisplayNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireWinrtDisplayNV.html)
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
#[inline(always)]
pub unsafe fn acquire_winrt_display_nv(
    physical_device: PhysicalDeviceHandle,
    display: DisplayKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetWinrtDisplayNV =
    unsafe extern "C" fn(PhysicalDeviceHandle, u32, *mut DisplayKHR) -> ResultCode;
/// [`vkGetWinrtDisplayNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetWinrtDisplayNV.html)
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
#[inline(always)]
pub unsafe fn get_winrt_display_nv(
    physical_device: PhysicalDeviceHandle,
    device_relative_id: u32,
    display: *mut DisplayKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateDirectFBSurfaceEXT = unsafe extern "C" fn(
    InstanceHandle,
    *const DirectFBSurfaceCreateInfoEXT,
    *const AllocationCallbacks,
    *mut SurfaceKHR,
) -> ResultCode;
/// [`vkCreateDirectFBSurfaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDirectFBSurfaceEXT.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_direct_fb_surface_ext(
    instance: InstanceHandle,
    create_info: *const DirectFBSurfaceCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceDirectFBPresentationSupportEXT =
    unsafe extern "C" fn(PhysicalDeviceHandle, u32, *mut IDirectFB) -> Bool32;
/// [`vkGetPhysicalDeviceDirectFBPresentationSupportEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html)
///
#[doc(alias = "vkGetPhysicalDeviceDirectFBPresentationSupportEXT")]
#[inline(always)]
pub unsafe fn get_physical_device_direct_fb_presentation_support_ext(
    physical_device: PhysicalDeviceHandle,
    queue_family_index: u32,
    dfb: *mut IDirectFB,
) -> Bool32 {
    todo!()
}

pub(crate) type FUN_CmdSetVertexInputEXT = unsafe extern "C" fn(
    CommandBufferHandle,
    u32,
    *const VertexInputBindingDescription2EXT,
    u32,
    *const VertexInputAttributeDescription2EXT,
);
/// [`vkCmdSetVertexInputEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetVertexInputEXT.html)
///
/// # Optional parameters
/// - vertex_binding_description_count
/// - vertex_attribute_description_count
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetVertexInputEXT")]
#[inline(always)]
pub unsafe fn cmd_set_vertex_input_ext(
    command_buffer: CommandBufferHandle,
    vertex_binding_description_count: u32,
    vertex_binding_descriptions: *const VertexInputBindingDescription2EXT,
    vertex_attribute_description_count: u32,
    vertex_attribute_descriptions: *const VertexInputAttributeDescription2EXT,
) {
    todo!()
}

pub(crate) type FUN_GetMemoryZirconHandleFUCHSIA = unsafe extern "C" fn(
    DeviceHandle,
    *const MemoryGetZirconHandleInfoFUCHSIA,
    *mut zx_handle_t,
) -> ResultCode;
/// [`vkGetMemoryZirconHandleFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryZirconHandleFUCHSIA.html)
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
#[inline(always)]
pub unsafe fn get_memory_zircon_handle_fuchsia(
    device: DeviceHandle,
    get_zircon_handle_info: *const MemoryGetZirconHandleInfoFUCHSIA,
    zircon_handle: *mut zx_handle_t,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetMemoryZirconHandlePropertiesFUCHSIA = unsafe extern "C" fn(
    DeviceHandle,
    ExternalMemoryHandleTypeFlags,
    zx_handle_t,
    *mut MemoryZirconHandlePropertiesFUCHSIA,
) -> ResultCode;
/// [`vkGetMemoryZirconHandlePropertiesFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryZirconHandlePropertiesFUCHSIA.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`INVALID_EXTERNAL_HANDLE`](ResultCode::ERROR_INVALID_EXTERNAL_HANDLE)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkGetMemoryZirconHandlePropertiesFUCHSIA")]
#[inline(always)]
pub unsafe fn get_memory_zircon_handle_properties_fuchsia(
    device: DeviceHandle,
    handle_type: ExternalMemoryHandleTypeFlags,
    zircon_handle: zx_handle_t,
    memory_zircon_handle_properties: *mut MemoryZirconHandlePropertiesFUCHSIA,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_ImportSemaphoreZirconHandleFUCHSIA =
    unsafe extern "C" fn(DeviceHandle, *const ImportSemaphoreZirconHandleInfoFUCHSIA) -> ResultCode;
/// [`vkImportSemaphoreZirconHandleFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreZirconHandleFUCHSIA.html)
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
#[inline(always)]
pub unsafe fn import_semaphore_zircon_handle_fuchsia(
    device: DeviceHandle,
    import_semaphore_zircon_handle_info: *const ImportSemaphoreZirconHandleInfoFUCHSIA,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetSemaphoreZirconHandleFUCHSIA = unsafe extern "C" fn(
    DeviceHandle,
    *const SemaphoreGetZirconHandleInfoFUCHSIA,
    *mut zx_handle_t,
) -> ResultCode;
/// [`vkGetSemaphoreZirconHandleFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreZirconHandleFUCHSIA.html)
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
#[inline(always)]
pub unsafe fn get_semaphore_zircon_handle_fuchsia(
    device: DeviceHandle,
    get_zircon_handle_info: *const SemaphoreGetZirconHandleInfoFUCHSIA,
    zircon_handle: *mut zx_handle_t,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateBufferCollectionFUCHSIA = unsafe extern "C" fn(
    DeviceHandle,
    *const BufferCollectionCreateInfoFUCHSIA,
    *const AllocationCallbacks,
    *mut BufferCollectionFUCHSIA,
) -> ResultCode;
/// [`vkCreateBufferCollectionFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBufferCollectionFUCHSIA.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_buffer_collection_fuchsia(
    device: DeviceHandle,
    create_info: *const BufferCollectionCreateInfoFUCHSIA,
    allocator: *const AllocationCallbacks,
    collection: *mut BufferCollectionFUCHSIA,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_SetBufferCollectionImageConstraintsFUCHSIA = unsafe extern "C" fn(
    DeviceHandle,
    BufferCollectionFUCHSIA,
    *const ImageConstraintsInfoFUCHSIA,
)
    -> ResultCode;
/// [`vkSetBufferCollectionImageConstraintsFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionImageConstraintsFUCHSIA.html)
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
#[inline(always)]
pub unsafe fn set_buffer_collection_image_constraints_fuchsia(
    device: DeviceHandle,
    collection: BufferCollectionFUCHSIA,
    image_constraints_info: *const ImageConstraintsInfoFUCHSIA,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_SetBufferCollectionBufferConstraintsFUCHSIA =
    unsafe extern "C" fn(
        DeviceHandle,
        BufferCollectionFUCHSIA,
        *const BufferConstraintsInfoFUCHSIA,
    ) -> ResultCode;
/// [`vkSetBufferCollectionBufferConstraintsFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionBufferConstraintsFUCHSIA.html)
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
#[inline(always)]
pub unsafe fn set_buffer_collection_buffer_constraints_fuchsia(
    device: DeviceHandle,
    collection: BufferCollectionFUCHSIA,
    buffer_constraints_info: *const BufferConstraintsInfoFUCHSIA,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyBufferCollectionFUCHSIA =
    unsafe extern "C" fn(DeviceHandle, BufferCollectionFUCHSIA, *const AllocationCallbacks);
/// [`vkDestroyBufferCollectionFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferCollectionFUCHSIA.html)
///
/// # Optional parameters
/// - allocator
///
#[doc(alias = "vkDestroyBufferCollectionFUCHSIA")]
#[inline(always)]
pub unsafe fn destroy_buffer_collection_fuchsia(
    device: DeviceHandle,
    collection: BufferCollectionFUCHSIA,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_GetBufferCollectionPropertiesFUCHSIA = unsafe extern "C" fn(
    DeviceHandle,
    BufferCollectionFUCHSIA,
    *mut BufferCollectionPropertiesFUCHSIA,
) -> ResultCode;
/// [`vkGetBufferCollectionPropertiesFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferCollectionPropertiesFUCHSIA.html)
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
#[inline(always)]
pub unsafe fn get_buffer_collection_properties_fuchsia(
    device: DeviceHandle,
    collection: BufferCollectionFUCHSIA,
    properties: *mut BufferCollectionPropertiesFUCHSIA,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI =
    unsafe extern "C" fn(DeviceHandle, RenderPass, *mut Extent2D) -> ResultCode;
/// [`vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html)
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
#[inline(always)]
pub unsafe fn get_device_subpass_shading_max_workgroup_size_huawei(
    device: DeviceHandle,
    renderpass: RenderPass,
    max_workgroup_size: *mut Extent2D,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdSubpassShadingHUAWEI = unsafe extern "C" fn(CommandBufferHandle);
/// [`vkCmdSubpassShadingHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSubpassShadingHUAWEI.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSubpassShadingHUAWEI")]
#[inline(always)]
pub unsafe fn cmd_subpass_shading_huawei(command_buffer: CommandBufferHandle) {
    todo!()
}

pub(crate) type FUN_CmdBindInvocationMaskHUAWEI =
    unsafe extern "C" fn(CommandBufferHandle, ImageView, ImageLayout);
/// [`vkCmdBindInvocationMaskHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindInvocationMaskHUAWEI.html)
///
/// # Optional parameters
/// - image_view
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdBindInvocationMaskHUAWEI")]
#[inline(always)]
pub unsafe fn cmd_bind_invocation_mask_huawei(
    command_buffer: CommandBufferHandle,
    image_view: ImageView,
    image_layout: ImageLayout,
) {
    todo!()
}

pub(crate) type FUN_GetMemoryRemoteAddressNV = unsafe extern "C" fn(
    DeviceHandle,
    *const MemoryGetRemoteAddressInfoNV,
    *mut RemoteAddressNV,
) -> ResultCode;
/// [`vkGetMemoryRemoteAddressNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryRemoteAddressNV.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`INVALID_EXTERNAL_HANDLE`](ResultCode::ERROR_INVALID_EXTERNAL_HANDLE)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkGetMemoryRemoteAddressNV")]
#[inline(always)]
pub unsafe fn get_memory_remote_address_nv(
    device: DeviceHandle,
    memory_get_remote_address_info: *const MemoryGetRemoteAddressInfoNV,
    address: *mut RemoteAddressNV,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPipelinePropertiesEXT =
    unsafe extern "C" fn(DeviceHandle, *const PipelineInfoKHR, *mut BaseOutStructure) -> ResultCode;
/// [`vkGetPipelinePropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelinePropertiesEXT.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkGetPipelinePropertiesEXT")]
#[inline(always)]
pub unsafe fn get_pipeline_properties_ext(
    device: DeviceHandle,
    pipeline_info: *const PipelineInfoKHR,
    pipeline_properties: *mut BaseOutStructure,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdSetPatchControlPointsEXT = unsafe extern "C" fn(CommandBufferHandle, u32);
/// [`vkCmdSetPatchControlPointsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPatchControlPointsEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetPatchControlPointsEXT")]
#[inline(always)]
pub unsafe fn cmd_set_patch_control_points_ext(
    command_buffer: CommandBufferHandle,
    patch_control_points: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetRasterizerDiscardEnableEXT =
    unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetRasterizerDiscardEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizerDiscardEnableEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetRasterizerDiscardEnableEXT")]
#[inline(always)]
pub unsafe fn cmd_set_rasterizer_discard_enable_ext(
    command_buffer: CommandBufferHandle,
    rasterizer_discard_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetDepthBiasEnableEXT = unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetDepthBiasEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBiasEnableEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetDepthBiasEnableEXT")]
#[inline(always)]
pub unsafe fn cmd_set_depth_bias_enable_ext(
    command_buffer: CommandBufferHandle,
    depth_bias_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetLogicOpEXT = unsafe extern "C" fn(CommandBufferHandle, LogicOp);
/// [`vkCmdSetLogicOpEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLogicOpEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetLogicOpEXT")]
#[inline(always)]
pub unsafe fn cmd_set_logic_op_ext(command_buffer: CommandBufferHandle, logic_op: LogicOp) {
    todo!()
}

pub(crate) type FUN_CmdSetPrimitiveRestartEnableEXT =
    unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetPrimitiveRestartEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartEnableEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetPrimitiveRestartEnableEXT")]
#[inline(always)]
pub unsafe fn cmd_set_primitive_restart_enable_ext(
    command_buffer: CommandBufferHandle,
    primitive_restart_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CreateScreenSurfaceQNX = unsafe extern "C" fn(
    InstanceHandle,
    *const ScreenSurfaceCreateInfoQNX,
    *const AllocationCallbacks,
    *mut SurfaceKHR,
) -> ResultCode;
/// [`vkCreateScreenSurfaceQNX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateScreenSurfaceQNX.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_screen_surface_qnx(
    instance: InstanceHandle,
    create_info: *const ScreenSurfaceCreateInfoQNX,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceScreenPresentationSupportQNX =
    unsafe extern "C" fn(PhysicalDeviceHandle, u32, *mut _screen_window) -> Bool32;
/// [`vkGetPhysicalDeviceScreenPresentationSupportQNX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceScreenPresentationSupportQNX.html)
///
#[doc(alias = "vkGetPhysicalDeviceScreenPresentationSupportQNX")]
#[inline(always)]
pub unsafe fn get_physical_device_screen_presentation_support_qnx(
    physical_device: PhysicalDeviceHandle,
    queue_family_index: u32,
    window: *mut _screen_window,
) -> Bool32 {
    todo!()
}

pub(crate) type FUN_CmdSetColorWriteEnableEXT =
    unsafe extern "C" fn(CommandBufferHandle, u32, *const Bool32);
/// [`vkCmdSetColorWriteEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorWriteEnableEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetColorWriteEnableEXT")]
#[inline(always)]
pub unsafe fn cmd_set_color_write_enable_ext(
    command_buffer: CommandBufferHandle,
    attachment_count: u32,
    color_write_enables: *const Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdDrawMultiEXT =
    unsafe extern "C" fn(CommandBufferHandle, u32, *const MultiDrawInfoEXT, u32, u32, u32);
/// [`vkCmdDrawMultiEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMultiEXT.html)
///
/// # Optional parameters
/// - draw_count
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawMultiEXT")]
#[inline(always)]
pub unsafe fn cmd_draw_multi_ext(
    command_buffer: CommandBufferHandle,
    draw_count: u32,
    vertex_info: *const MultiDrawInfoEXT,
    instance_count: u32,
    first_instance: u32,
    stride: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdDrawMultiIndexedEXT = unsafe extern "C" fn(
    CommandBufferHandle,
    u32,
    *const MultiDrawIndexedInfoEXT,
    u32,
    u32,
    u32,
    *const i32,
);
/// [`vkCmdDrawMultiIndexedEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMultiIndexedEXT.html)
///
/// # Optional parameters
/// - draw_count
/// - vertex_offset
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawMultiIndexedEXT")]
#[inline(always)]
pub unsafe fn cmd_draw_multi_indexed_ext(
    command_buffer: CommandBufferHandle,
    draw_count: u32,
    index_info: *const MultiDrawIndexedInfoEXT,
    instance_count: u32,
    first_instance: u32,
    stride: u32,
    vertex_offset: *const i32,
) {
    todo!()
}

pub(crate) type FUN_CreateMicromapEXT = unsafe extern "C" fn(
    DeviceHandle,
    *const MicromapCreateInfoEXT,
    *const AllocationCallbacks,
    *mut MicromapEXT,
) -> ResultCode;
/// [`vkCreateMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMicromapEXT.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_micromap_ext(
    device: DeviceHandle,
    create_info: *const MicromapCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    micromap: *mut MicromapEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyMicromapEXT =
    unsafe extern "C" fn(DeviceHandle, MicromapEXT, *const AllocationCallbacks);
/// [`vkDestroyMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyMicromapEXT.html)
///
/// # Optional parameters
/// - micromap
/// - allocator
///
#[doc(alias = "vkDestroyMicromapEXT")]
#[inline(always)]
pub unsafe fn destroy_micromap_ext(
    device: DeviceHandle,
    micromap: MicromapEXT,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_CmdBuildMicromapsEXT =
    unsafe extern "C" fn(CommandBufferHandle, u32, *const MicromapBuildInfoEXT);
/// [`vkCmdBuildMicromapsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildMicromapsEXT.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdBuildMicromapsEXT")]
#[inline(always)]
pub unsafe fn cmd_build_micromaps_ext(
    command_buffer: CommandBufferHandle,
    info_count: u32,
    infos: *const MicromapBuildInfoEXT,
) {
    todo!()
}

pub(crate) type FUN_BuildMicromapsEXT = unsafe extern "C" fn(
    DeviceHandle,
    DeferredOperationKHR,
    u32,
    *const MicromapBuildInfoEXT,
) -> ResultCode;
/// [`vkBuildMicromapsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBuildMicromapsEXT.html)
///
/// # Optional parameters
/// - deferred_operation
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
#[inline(always)]
pub unsafe fn build_micromaps_ext(
    device: DeviceHandle,
    deferred_operation: DeferredOperationKHR,
    info_count: u32,
    infos: *const MicromapBuildInfoEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CopyMicromapEXT = unsafe extern "C" fn(
    DeviceHandle,
    DeferredOperationKHR,
    *const CopyMicromapInfoEXT,
) -> ResultCode;
/// [`vkCopyMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMicromapEXT.html)
///
/// # Optional parameters
/// - deferred_operation
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
#[inline(always)]
pub unsafe fn copy_micromap_ext(
    device: DeviceHandle,
    deferred_operation: DeferredOperationKHR,
    info: *const CopyMicromapInfoEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CopyMicromapToMemoryEXT = unsafe extern "C" fn(
    DeviceHandle,
    DeferredOperationKHR,
    *const CopyMicromapToMemoryInfoEXT,
) -> ResultCode;
/// [`vkCopyMicromapToMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMicromapToMemoryEXT.html)
///
/// # Optional parameters
/// - deferred_operation
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
#[inline(always)]
pub unsafe fn copy_micromap_to_memory_ext(
    device: DeviceHandle,
    deferred_operation: DeferredOperationKHR,
    info: *const CopyMicromapToMemoryInfoEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CopyMemoryToMicromapEXT = unsafe extern "C" fn(
    DeviceHandle,
    DeferredOperationKHR,
    *const CopyMemoryToMicromapInfoEXT,
) -> ResultCode;
/// [`vkCopyMemoryToMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToMicromapEXT.html)
///
/// # Optional parameters
/// - deferred_operation
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
#[inline(always)]
pub unsafe fn copy_memory_to_micromap_ext(
    device: DeviceHandle,
    deferred_operation: DeferredOperationKHR,
    info: *const CopyMemoryToMicromapInfoEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_WriteMicromapsPropertiesEXT = unsafe extern "C" fn(
    DeviceHandle,
    u32,
    *const MicromapEXT,
    QueryType,
    usize,
    *mut c_void,
    usize,
) -> ResultCode;
/// [`vkWriteMicromapsPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteMicromapsPropertiesEXT.html)
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
#[inline(always)]
pub unsafe fn write_micromaps_properties_ext(
    device: DeviceHandle,
    micromap_count: u32,
    micromaps: *const MicromapEXT,
    query_type: QueryType,
    data_size: usize,
    data: *mut c_void,
    stride: usize,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdCopyMicromapEXT =
    unsafe extern "C" fn(CommandBufferHandle, *const CopyMicromapInfoEXT);
/// [`vkCmdCopyMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMicromapEXT.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyMicromapEXT")]
#[inline(always)]
pub unsafe fn cmd_copy_micromap_ext(
    command_buffer: CommandBufferHandle,
    info: *const CopyMicromapInfoEXT,
) {
    todo!()
}

pub(crate) type FUN_CmdCopyMicromapToMemoryEXT =
    unsafe extern "C" fn(CommandBufferHandle, *const CopyMicromapToMemoryInfoEXT);
/// [`vkCmdCopyMicromapToMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMicromapToMemoryEXT.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyMicromapToMemoryEXT")]
#[inline(always)]
pub unsafe fn cmd_copy_micromap_to_memory_ext(
    command_buffer: CommandBufferHandle,
    info: *const CopyMicromapToMemoryInfoEXT,
) {
    todo!()
}

pub(crate) type FUN_CmdCopyMemoryToMicromapEXT =
    unsafe extern "C" fn(CommandBufferHandle, *const CopyMemoryToMicromapInfoEXT);
/// [`vkCmdCopyMemoryToMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToMicromapEXT.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyMemoryToMicromapEXT")]
#[inline(always)]
pub unsafe fn cmd_copy_memory_to_micromap_ext(
    command_buffer: CommandBufferHandle,
    info: *const CopyMemoryToMicromapInfoEXT,
) {
    todo!()
}

pub(crate) type FUN_CmdWriteMicromapsPropertiesEXT =
    unsafe extern "C" fn(CommandBufferHandle, u32, *const MicromapEXT, QueryType, QueryPool, u32);
/// [`vkCmdWriteMicromapsPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteMicromapsPropertiesEXT.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdWriteMicromapsPropertiesEXT")]
#[inline(always)]
pub unsafe fn cmd_write_micromaps_properties_ext(
    command_buffer: CommandBufferHandle,
    micromap_count: u32,
    micromaps: *const MicromapEXT,
    query_type: QueryType,
    query_pool: QueryPool,
    first_query: u32,
) {
    todo!()
}

pub(crate) type FUN_GetDeviceMicromapCompatibilityEXT = unsafe extern "C" fn(
    DeviceHandle,
    *const MicromapVersionInfoEXT,
    *mut AccelerationStructureCompatibilityKHR,
);
/// [`vkGetDeviceMicromapCompatibilityEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMicromapCompatibilityEXT.html)
///
#[doc(alias = "vkGetDeviceMicromapCompatibilityEXT")]
#[inline(always)]
pub unsafe fn get_device_micromap_compatibility_ext(
    device: DeviceHandle,
    version_info: *const MicromapVersionInfoEXT,
    compatibility: *mut AccelerationStructureCompatibilityKHR,
) {
    todo!()
}

pub(crate) type FUN_GetMicromapBuildSizesEXT = unsafe extern "C" fn(
    DeviceHandle,
    AccelerationStructureBuildTypeKHR,
    *const MicromapBuildInfoEXT,
    *mut MicromapBuildSizesInfoEXT,
);
/// [`vkGetMicromapBuildSizesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMicromapBuildSizesEXT.html)
///
#[doc(alias = "vkGetMicromapBuildSizesEXT")]
#[inline(always)]
pub unsafe fn get_micromap_build_sizes_ext(
    device: DeviceHandle,
    build_type: AccelerationStructureBuildTypeKHR,
    build_info: *const MicromapBuildInfoEXT,
    size_info: *mut MicromapBuildSizesInfoEXT,
) {
    todo!()
}

pub(crate) type FUN_CmdDrawClusterHUAWEI = unsafe extern "C" fn(CommandBufferHandle, u32, u32, u32);
/// [`vkCmdDrawClusterHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawClusterHUAWEI.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawClusterHUAWEI")]
#[inline(always)]
pub unsafe fn cmd_draw_cluster_huawei(
    command_buffer: CommandBufferHandle,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdDrawClusterIndirectHUAWEI =
    unsafe extern "C" fn(CommandBufferHandle, Buffer, DeviceSize);
/// [`vkCmdDrawClusterIndirectHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawClusterIndirectHUAWEI.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawClusterIndirectHUAWEI")]
#[inline(always)]
pub unsafe fn cmd_draw_cluster_indirect_huawei(
    command_buffer: CommandBufferHandle,
    buffer: Buffer,
    offset: DeviceSize,
) {
    todo!()
}

pub(crate) type FUN_SetDeviceMemoryPriorityEXT =
    unsafe extern "C" fn(DeviceHandle, DeviceMemory, f32);
/// [`vkSetDeviceMemoryPriorityEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDeviceMemoryPriorityEXT.html)
///
#[doc(alias = "vkSetDeviceMemoryPriorityEXT")]
#[inline(always)]
pub unsafe fn set_device_memory_priority_ext(
    device: DeviceHandle,
    memory: DeviceMemory,
    priority: f32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetDispatchParametersARM =
    unsafe extern "C" fn(CommandBufferHandle, *const DispatchParametersARM);
/// [`vkCmdSetDispatchParametersARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDispatchParametersARM.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdSetDispatchParametersARM")]
#[inline(always)]
pub unsafe fn cmd_set_dispatch_parameters_arm(
    command_buffer: CommandBufferHandle,
    dispatch_parameters: *const DispatchParametersARM,
) {
    todo!()
}

pub(crate) type FUN_GetDescriptorSetLayoutHostMappingInfoVALVE = unsafe extern "C" fn(
    DeviceHandle,
    *const DescriptorSetBindingReferenceVALVE,
    *mut DescriptorSetLayoutHostMappingInfoVALVE,
);
/// [`vkGetDescriptorSetLayoutHostMappingInfoVALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutHostMappingInfoVALVE.html)
///
#[doc(alias = "vkGetDescriptorSetLayoutHostMappingInfoVALVE")]
#[inline(always)]
pub unsafe fn get_descriptor_set_layout_host_mapping_info_valve(
    device: DeviceHandle,
    binding_reference: *const DescriptorSetBindingReferenceVALVE,
    host_mapping: *mut DescriptorSetLayoutHostMappingInfoVALVE,
) {
    todo!()
}

pub(crate) type FUN_GetDescriptorSetHostMappingVALVE =
    unsafe extern "C" fn(DeviceHandle, DescriptorSet, *mut *mut c_void);
/// [`vkGetDescriptorSetHostMappingVALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetHostMappingVALVE.html)
///
#[doc(alias = "vkGetDescriptorSetHostMappingVALVE")]
#[inline(always)]
pub unsafe fn get_descriptor_set_host_mapping_valve(
    device: DeviceHandle,
    descriptor_set: DescriptorSet,
    data: *mut *mut c_void,
) {
    todo!()
}

pub(crate) type FUN_CmdCopyMemoryIndirectNV =
    unsafe extern "C" fn(CommandBufferHandle, DeviceAddress, u32, u32);
/// [`vkCmdCopyMemoryIndirectNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryIndirectNV.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyMemoryIndirectNV")]
#[inline(always)]
pub unsafe fn cmd_copy_memory_indirect_nv(
    command_buffer: CommandBufferHandle,
    copy_buffer_address: DeviceAddress,
    copy_count: u32,
    stride: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdCopyMemoryToImageIndirectNV = unsafe extern "C" fn(
    CommandBufferHandle,
    DeviceAddress,
    u32,
    u32,
    Image,
    ImageLayout,
    *const ImageSubresourceLayers,
);
/// [`vkCmdCopyMemoryToImageIndirectNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageIndirectNV.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyMemoryToImageIndirectNV")]
#[inline(always)]
pub unsafe fn cmd_copy_memory_to_image_indirect_nv(
    command_buffer: CommandBufferHandle,
    copy_buffer_address: DeviceAddress,
    copy_count: u32,
    stride: u32,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    image_subresources: *const ImageSubresourceLayers,
) {
    todo!()
}

pub(crate) type FUN_CmdDecompressMemoryNV =
    unsafe extern "C" fn(CommandBufferHandle, u32, *const DecompressMemoryRegionNV);
/// [`vkCmdDecompressMemoryNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryNV.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdDecompressMemoryNV")]
#[inline(always)]
pub unsafe fn cmd_decompress_memory_nv(
    command_buffer: CommandBufferHandle,
    decompress_region_count: u32,
    decompress_memory_regions: *const DecompressMemoryRegionNV,
) {
    todo!()
}

pub(crate) type FUN_CmdDecompressMemoryIndirectCountNV =
    unsafe extern "C" fn(CommandBufferHandle, DeviceAddress, DeviceAddress, u32);
/// [`vkCmdDecompressMemoryIndirectCountNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryIndirectCountNV.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdDecompressMemoryIndirectCountNV")]
#[inline(always)]
pub unsafe fn cmd_decompress_memory_indirect_count_nv(
    command_buffer: CommandBufferHandle,
    indirect_commands_address: DeviceAddress,
    indirect_commands_count_address: DeviceAddress,
    stride: u32,
) {
    todo!()
}

pub(crate) type FUN_GetPipelineIndirectMemoryRequirementsNV =
    unsafe extern "C" fn(DeviceHandle, *const ComputePipelineCreateInfo, *mut MemoryRequirements2);
/// [`vkGetPipelineIndirectMemoryRequirementsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineIndirectMemoryRequirementsNV.html)
///
#[doc(alias = "vkGetPipelineIndirectMemoryRequirementsNV")]
#[inline(always)]
pub unsafe fn get_pipeline_indirect_memory_requirements_nv(
    device: DeviceHandle,
    create_info: *const ComputePipelineCreateInfo,
    memory_requirements: *mut MemoryRequirements2,
) {
    todo!()
}

pub(crate) type FUN_CmdUpdatePipelineIndirectBufferNV =
    unsafe extern "C" fn(CommandBufferHandle, PipelineBindPoint, Pipeline);
/// [`vkCmdUpdatePipelineIndirectBufferNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdatePipelineIndirectBufferNV.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdUpdatePipelineIndirectBufferNV")]
#[inline(always)]
pub unsafe fn cmd_update_pipeline_indirect_buffer_nv(
    command_buffer: CommandBufferHandle,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
) {
    todo!()
}

pub(crate) type FUN_GetPipelineIndirectDeviceAddressNV =
    unsafe extern "C" fn(DeviceHandle, *const PipelineIndirectDeviceAddressInfoNV) -> DeviceAddress;
/// [`vkGetPipelineIndirectDeviceAddressNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineIndirectDeviceAddressNV.html)
///
#[doc(alias = "vkGetPipelineIndirectDeviceAddressNV")]
#[inline(always)]
pub unsafe fn get_pipeline_indirect_device_address_nv(
    device: DeviceHandle,
    info: *const PipelineIndirectDeviceAddressInfoNV,
) -> DeviceAddress {
    todo!()
}

pub(crate) type FUN_GetNativeBufferPropertiesOHOS = unsafe extern "C" fn(
    DeviceHandle,
    *const OH_NativeBuffer,
    *mut NativeBufferPropertiesOHOS,
) -> ResultCode;
/// [`vkGetNativeBufferPropertiesOHOS`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetNativeBufferPropertiesOHOS.html)
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
#[inline(always)]
pub unsafe fn get_native_buffer_properties_ohos(
    device: DeviceHandle,
    buffer: *const OH_NativeBuffer,
    properties: *mut NativeBufferPropertiesOHOS,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetMemoryNativeBufferOHOS = unsafe extern "C" fn(
    DeviceHandle,
    *const MemoryGetNativeBufferInfoOHOS,
    *mut *mut OH_NativeBuffer,
) -> ResultCode;
/// [`vkGetMemoryNativeBufferOHOS`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryNativeBufferOHOS.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`OUT_OF_HOST_MEMORY`](ResultCode::ERROR_OUT_OF_HOST_MEMORY)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkGetMemoryNativeBufferOHOS")]
#[inline(always)]
pub unsafe fn get_memory_native_buffer_ohos(
    device: DeviceHandle,
    info: *const MemoryGetNativeBufferInfoOHOS,
    buffer: *mut *mut OH_NativeBuffer,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdSetDepthClampEnableEXT = unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetDepthClampEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClampEnableEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetDepthClampEnableEXT")]
#[inline(always)]
pub unsafe fn cmd_set_depth_clamp_enable_ext(
    command_buffer: CommandBufferHandle,
    depth_clamp_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetPolygonModeEXT = unsafe extern "C" fn(CommandBufferHandle, PolygonMode);
/// [`vkCmdSetPolygonModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPolygonModeEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetPolygonModeEXT")]
#[inline(always)]
pub unsafe fn cmd_set_polygon_mode_ext(
    command_buffer: CommandBufferHandle,
    polygon_mode: PolygonMode,
) {
    todo!()
}

pub(crate) type FUN_CmdSetRasterizationSamplesEXT =
    unsafe extern "C" fn(CommandBufferHandle, SampleCountFlags);
/// [`vkCmdSetRasterizationSamplesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizationSamplesEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetRasterizationSamplesEXT")]
#[inline(always)]
pub unsafe fn cmd_set_rasterization_samples_ext(
    command_buffer: CommandBufferHandle,
    rasterization_samples: SampleCountFlags,
) {
    todo!()
}

pub(crate) type FUN_CmdSetSampleMaskEXT =
    unsafe extern "C" fn(CommandBufferHandle, SampleCountFlags, *const SampleMask);
/// [`vkCmdSetSampleMaskEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleMaskEXT.html)
///
/// # Optional parameters
/// - sample_mask
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetSampleMaskEXT")]
#[inline(always)]
pub unsafe fn cmd_set_sample_mask_ext(
    command_buffer: CommandBufferHandle,
    samples: SampleCountFlags,
    sample_mask: *const SampleMask,
) {
    todo!()
}

pub(crate) type FUN_CmdSetAlphaToCoverageEnableEXT =
    unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetAlphaToCoverageEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAlphaToCoverageEnableEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetAlphaToCoverageEnableEXT")]
#[inline(always)]
pub unsafe fn cmd_set_alpha_to_coverage_enable_ext(
    command_buffer: CommandBufferHandle,
    alpha_to_coverage_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetAlphaToOneEnableEXT = unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetAlphaToOneEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAlphaToOneEnableEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetAlphaToOneEnableEXT")]
#[inline(always)]
pub unsafe fn cmd_set_alpha_to_one_enable_ext(
    command_buffer: CommandBufferHandle,
    alpha_to_one_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetLogicOpEnableEXT = unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetLogicOpEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLogicOpEnableEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetLogicOpEnableEXT")]
#[inline(always)]
pub unsafe fn cmd_set_logic_op_enable_ext(
    command_buffer: CommandBufferHandle,
    logic_op_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetColorBlendEnableEXT =
    unsafe extern "C" fn(CommandBufferHandle, u32, u32, *const Bool32);
/// [`vkCmdSetColorBlendEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendEnableEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetColorBlendEnableEXT")]
#[inline(always)]
pub unsafe fn cmd_set_color_blend_enable_ext(
    command_buffer: CommandBufferHandle,
    first_attachment: u32,
    attachment_count: u32,
    color_blend_enables: *const Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetColorBlendEquationEXT =
    unsafe extern "C" fn(CommandBufferHandle, u32, u32, *const ColorBlendEquationEXT);
/// [`vkCmdSetColorBlendEquationEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendEquationEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetColorBlendEquationEXT")]
#[inline(always)]
pub unsafe fn cmd_set_color_blend_equation_ext(
    command_buffer: CommandBufferHandle,
    first_attachment: u32,
    attachment_count: u32,
    color_blend_equations: *const ColorBlendEquationEXT,
) {
    todo!()
}

pub(crate) type FUN_CmdSetColorWriteMaskEXT =
    unsafe extern "C" fn(CommandBufferHandle, u32, u32, *const ColorComponentFlags);
/// [`vkCmdSetColorWriteMaskEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorWriteMaskEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetColorWriteMaskEXT")]
#[inline(always)]
pub unsafe fn cmd_set_color_write_mask_ext(
    command_buffer: CommandBufferHandle,
    first_attachment: u32,
    attachment_count: u32,
    color_write_masks: *const ColorComponentFlags,
) {
    todo!()
}

pub(crate) type FUN_CmdSetTessellationDomainOriginEXT =
    unsafe extern "C" fn(CommandBufferHandle, TessellationDomainOrigin);
/// [`vkCmdSetTessellationDomainOriginEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetTessellationDomainOriginEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetTessellationDomainOriginEXT")]
#[inline(always)]
pub unsafe fn cmd_set_tessellation_domain_origin_ext(
    command_buffer: CommandBufferHandle,
    domain_origin: TessellationDomainOrigin,
) {
    todo!()
}

pub(crate) type FUN_CmdSetRasterizationStreamEXT = unsafe extern "C" fn(CommandBufferHandle, u32);
/// [`vkCmdSetRasterizationStreamEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizationStreamEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetRasterizationStreamEXT")]
#[inline(always)]
pub unsafe fn cmd_set_rasterization_stream_ext(
    command_buffer: CommandBufferHandle,
    rasterization_stream: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetConservativeRasterizationModeEXT =
    unsafe extern "C" fn(CommandBufferHandle, ConservativeRasterizationModeEXT);
/// [`vkCmdSetConservativeRasterizationModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetConservativeRasterizationModeEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetConservativeRasterizationModeEXT")]
#[inline(always)]
pub unsafe fn cmd_set_conservative_rasterization_mode_ext(
    command_buffer: CommandBufferHandle,
    conservative_rasterization_mode: ConservativeRasterizationModeEXT,
) {
    todo!()
}

pub(crate) type FUN_CmdSetExtraPrimitiveOverestimationSizeEXT =
    unsafe extern "C" fn(CommandBufferHandle, f32);
/// [`vkCmdSetExtraPrimitiveOverestimationSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExtraPrimitiveOverestimationSizeEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetExtraPrimitiveOverestimationSizeEXT")]
#[inline(always)]
pub unsafe fn cmd_set_extra_primitive_overestimation_size_ext(
    command_buffer: CommandBufferHandle,
    extra_primitive_overestimation_size: f32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetDepthClipEnableEXT = unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetDepthClipEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClipEnableEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetDepthClipEnableEXT")]
#[inline(always)]
pub unsafe fn cmd_set_depth_clip_enable_ext(
    command_buffer: CommandBufferHandle,
    depth_clip_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetSampleLocationsEnableEXT =
    unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetSampleLocationsEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleLocationsEnableEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetSampleLocationsEnableEXT")]
#[inline(always)]
pub unsafe fn cmd_set_sample_locations_enable_ext(
    command_buffer: CommandBufferHandle,
    sample_locations_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetColorBlendAdvancedEXT =
    unsafe extern "C" fn(CommandBufferHandle, u32, u32, *const ColorBlendAdvancedEXT);
/// [`vkCmdSetColorBlendAdvancedEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendAdvancedEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetColorBlendAdvancedEXT")]
#[inline(always)]
pub unsafe fn cmd_set_color_blend_advanced_ext(
    command_buffer: CommandBufferHandle,
    first_attachment: u32,
    attachment_count: u32,
    color_blend_advanced: *const ColorBlendAdvancedEXT,
) {
    todo!()
}

pub(crate) type FUN_CmdSetProvokingVertexModeEXT =
    unsafe extern "C" fn(CommandBufferHandle, ProvokingVertexModeEXT);
/// [`vkCmdSetProvokingVertexModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetProvokingVertexModeEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetProvokingVertexModeEXT")]
#[inline(always)]
pub unsafe fn cmd_set_provoking_vertex_mode_ext(
    command_buffer: CommandBufferHandle,
    provoking_vertex_mode: ProvokingVertexModeEXT,
) {
    todo!()
}

pub(crate) type FUN_CmdSetLineRasterizationModeEXT =
    unsafe extern "C" fn(CommandBufferHandle, LineRasterizationModeEXT);
/// [`vkCmdSetLineRasterizationModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineRasterizationModeEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetLineRasterizationModeEXT")]
#[inline(always)]
pub unsafe fn cmd_set_line_rasterization_mode_ext(
    command_buffer: CommandBufferHandle,
    line_rasterization_mode: LineRasterizationModeEXT,
) {
    todo!()
}

pub(crate) type FUN_CmdSetLineStippleEnableEXT = unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetLineStippleEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleEnableEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetLineStippleEnableEXT")]
#[inline(always)]
pub unsafe fn cmd_set_line_stipple_enable_ext(
    command_buffer: CommandBufferHandle,
    stippled_line_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetDepthClipNegativeOneToOneEXT =
    unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetDepthClipNegativeOneToOneEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClipNegativeOneToOneEXT.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetDepthClipNegativeOneToOneEXT")]
#[inline(always)]
pub unsafe fn cmd_set_depth_clip_negative_one_to_one_ext(
    command_buffer: CommandBufferHandle,
    negative_one_to_one: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetViewportWScalingEnableNV =
    unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetViewportWScalingEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWScalingEnableNV.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetViewportWScalingEnableNV")]
#[inline(always)]
pub unsafe fn cmd_set_viewport_w_scaling_enable_nv(
    command_buffer: CommandBufferHandle,
    viewport_w_scaling_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetViewportSwizzleNV =
    unsafe extern "C" fn(CommandBufferHandle, u32, u32, *const ViewportSwizzleNV);
/// [`vkCmdSetViewportSwizzleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportSwizzleNV.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetViewportSwizzleNV")]
#[inline(always)]
pub unsafe fn cmd_set_viewport_swizzle_nv(
    command_buffer: CommandBufferHandle,
    first_viewport: u32,
    viewport_count: u32,
    viewport_swizzles: *const ViewportSwizzleNV,
) {
    todo!()
}

pub(crate) type FUN_CmdSetCoverageToColorEnableNV =
    unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetCoverageToColorEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageToColorEnableNV.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetCoverageToColorEnableNV")]
#[inline(always)]
pub unsafe fn cmd_set_coverage_to_color_enable_nv(
    command_buffer: CommandBufferHandle,
    coverage_to_color_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetCoverageToColorLocationNV =
    unsafe extern "C" fn(CommandBufferHandle, u32);
/// [`vkCmdSetCoverageToColorLocationNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageToColorLocationNV.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetCoverageToColorLocationNV")]
#[inline(always)]
pub unsafe fn cmd_set_coverage_to_color_location_nv(
    command_buffer: CommandBufferHandle,
    coverage_to_color_location: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetCoverageModulationModeNV =
    unsafe extern "C" fn(CommandBufferHandle, CoverageModulationModeNV);
/// [`vkCmdSetCoverageModulationModeNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationModeNV.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetCoverageModulationModeNV")]
#[inline(always)]
pub unsafe fn cmd_set_coverage_modulation_mode_nv(
    command_buffer: CommandBufferHandle,
    coverage_modulation_mode: CoverageModulationModeNV,
) {
    todo!()
}

pub(crate) type FUN_CmdSetCoverageModulationTableEnableNV =
    unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetCoverageModulationTableEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationTableEnableNV.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetCoverageModulationTableEnableNV")]
#[inline(always)]
pub unsafe fn cmd_set_coverage_modulation_table_enable_nv(
    command_buffer: CommandBufferHandle,
    coverage_modulation_table_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetCoverageModulationTableNV =
    unsafe extern "C" fn(CommandBufferHandle, u32, *const f32);
/// [`vkCmdSetCoverageModulationTableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationTableNV.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetCoverageModulationTableNV")]
#[inline(always)]
pub unsafe fn cmd_set_coverage_modulation_table_nv(
    command_buffer: CommandBufferHandle,
    coverage_modulation_table_count: u32,
    coverage_modulation_table: *const f32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetShadingRateImageEnableNV =
    unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetShadingRateImageEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetShadingRateImageEnableNV.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetShadingRateImageEnableNV")]
#[inline(always)]
pub unsafe fn cmd_set_shading_rate_image_enable_nv(
    command_buffer: CommandBufferHandle,
    shading_rate_image_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetRepresentativeFragmentTestEnableNV =
    unsafe extern "C" fn(CommandBufferHandle, Bool32);
/// [`vkCmdSetRepresentativeFragmentTestEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRepresentativeFragmentTestEnableNV.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetRepresentativeFragmentTestEnableNV")]
#[inline(always)]
pub unsafe fn cmd_set_representative_fragment_test_enable_nv(
    command_buffer: CommandBufferHandle,
    representative_fragment_test_enable: Bool32,
) {
    todo!()
}

pub(crate) type FUN_CmdSetCoverageReductionModeNV =
    unsafe extern "C" fn(CommandBufferHandle, CoverageReductionModeNV);
/// [`vkCmdSetCoverageReductionModeNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageReductionModeNV.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetCoverageReductionModeNV")]
#[inline(always)]
pub unsafe fn cmd_set_coverage_reduction_mode_nv(
    command_buffer: CommandBufferHandle,
    coverage_reduction_mode: CoverageReductionModeNV,
) {
    todo!()
}

pub(crate) type FUN_CreateTensorARM = unsafe extern "C" fn(
    DeviceHandle,
    *const TensorCreateInfoARM,
    *const AllocationCallbacks,
    *mut TensorARM,
) -> ResultCode;
/// [`vkCreateTensorARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateTensorARM.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_tensor_arm(
    device: DeviceHandle,
    create_info: *const TensorCreateInfoARM,
    allocator: *const AllocationCallbacks,
    tensor: *mut TensorARM,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyTensorARM =
    unsafe extern "C" fn(DeviceHandle, TensorARM, *const AllocationCallbacks);
/// [`vkDestroyTensorARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyTensorARM.html)
///
/// # Optional parameters
/// - tensor
/// - allocator
///
#[doc(alias = "vkDestroyTensorARM")]
#[inline(always)]
pub unsafe fn destroy_tensor_arm(
    device: DeviceHandle,
    tensor: TensorARM,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_CreateTensorViewARM = unsafe extern "C" fn(
    DeviceHandle,
    *const TensorViewCreateInfoARM,
    *const AllocationCallbacks,
    *mut TensorViewARM,
) -> ResultCode;
/// [`vkCreateTensorViewARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateTensorViewARM.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_tensor_view_arm(
    device: DeviceHandle,
    create_info: *const TensorViewCreateInfoARM,
    allocator: *const AllocationCallbacks,
    view: *mut TensorViewARM,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyTensorViewARM =
    unsafe extern "C" fn(DeviceHandle, TensorViewARM, *const AllocationCallbacks);
/// [`vkDestroyTensorViewARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyTensorViewARM.html)
///
/// # Optional parameters
/// - tensor_view
/// - allocator
///
#[doc(alias = "vkDestroyTensorViewARM")]
#[inline(always)]
pub unsafe fn destroy_tensor_view_arm(
    device: DeviceHandle,
    tensor_view: TensorViewARM,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_GetTensorMemoryRequirementsARM = unsafe extern "C" fn(
    DeviceHandle,
    *const TensorMemoryRequirementsInfoARM,
    *mut MemoryRequirements2,
);
/// [`vkGetTensorMemoryRequirementsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorMemoryRequirementsARM.html)
///
#[doc(alias = "vkGetTensorMemoryRequirementsARM")]
#[inline(always)]
pub unsafe fn get_tensor_memory_requirements_arm(
    device: DeviceHandle,
    info: *const TensorMemoryRequirementsInfoARM,
    memory_requirements: *mut MemoryRequirements2,
) {
    todo!()
}

pub(crate) type FUN_BindTensorMemoryARM =
    unsafe extern "C" fn(DeviceHandle, u32, *const BindTensorMemoryInfoARM) -> ResultCode;
/// [`vkBindTensorMemoryARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindTensorMemoryARM.html)
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
#[inline(always)]
pub unsafe fn bind_tensor_memory_arm(
    device: DeviceHandle,
    bind_info_count: u32,
    bind_infos: *const BindTensorMemoryInfoARM,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetDeviceTensorMemoryRequirementsARM = unsafe extern "C" fn(
    DeviceHandle,
    *const DeviceTensorMemoryRequirementsARM,
    *mut MemoryRequirements2,
);
/// [`vkGetDeviceTensorMemoryRequirementsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceTensorMemoryRequirementsARM.html)
///
#[doc(alias = "vkGetDeviceTensorMemoryRequirementsARM")]
#[inline(always)]
pub unsafe fn get_device_tensor_memory_requirements_arm(
    device: DeviceHandle,
    info: *const DeviceTensorMemoryRequirementsARM,
    memory_requirements: *mut MemoryRequirements2,
) {
    todo!()
}

pub(crate) type FUN_CmdCopyTensorARM =
    unsafe extern "C" fn(CommandBufferHandle, *const CopyTensorInfoARM);
/// [`vkCmdCopyTensorARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyTensorARM.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`TRANSFER`](QueueFlags::TRANSFER)
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyTensorARM")]
#[inline(always)]
pub unsafe fn cmd_copy_tensor_arm(
    command_buffer: CommandBufferHandle,
    copy_tensor_info: *const CopyTensorInfoARM,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceExternalTensorPropertiesARM = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *const PhysicalDeviceExternalTensorInfoARM,
    *mut ExternalTensorPropertiesARM,
);
/// [`vkGetPhysicalDeviceExternalTensorPropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalTensorPropertiesARM.html)
///
#[doc(alias = "vkGetPhysicalDeviceExternalTensorPropertiesARM")]
#[inline(always)]
pub unsafe fn get_physical_device_external_tensor_properties_arm(
    physical_device: PhysicalDeviceHandle,
    external_tensor_info: *const PhysicalDeviceExternalTensorInfoARM,
    external_tensor_properties: *mut ExternalTensorPropertiesARM,
) {
    todo!()
}

pub(crate) type FUN_GetTensorOpaqueCaptureDescriptorDataARM = unsafe extern "C" fn(
    DeviceHandle,
    *const TensorCaptureDescriptorDataInfoARM,
    *mut c_void,
) -> ResultCode;
/// [`vkGetTensorOpaqueCaptureDescriptorDataARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDescriptorDataARM.html)
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
#[inline(always)]
pub unsafe fn get_tensor_opaque_capture_descriptor_data_arm(
    device: DeviceHandle,
    info: *const TensorCaptureDescriptorDataInfoARM,
    data: *mut c_void,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetTensorViewOpaqueCaptureDescriptorDataARM =
    unsafe extern "C" fn(
        DeviceHandle,
        *const TensorViewCaptureDescriptorDataInfoARM,
        *mut c_void,
    ) -> ResultCode;
/// [`vkGetTensorViewOpaqueCaptureDescriptorDataARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorViewOpaqueCaptureDescriptorDataARM.html)
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
#[inline(always)]
pub unsafe fn get_tensor_view_opaque_capture_descriptor_data_arm(
    device: DeviceHandle,
    info: *const TensorViewCaptureDescriptorDataInfoARM,
    data: *mut c_void,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetShaderModuleIdentifierEXT =
    unsafe extern "C" fn(DeviceHandle, ShaderModule, *mut ShaderModuleIdentifierEXT);
/// [`vkGetShaderModuleIdentifierEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleIdentifierEXT.html)
///
#[doc(alias = "vkGetShaderModuleIdentifierEXT")]
#[inline(always)]
pub unsafe fn get_shader_module_identifier_ext(
    device: DeviceHandle,
    shader_module: ShaderModule,
    identifier: *mut ShaderModuleIdentifierEXT,
) {
    todo!()
}

pub(crate) type FUN_GetShaderModuleCreateInfoIdentifierEXT = unsafe extern "C" fn(
    DeviceHandle,
    *const ShaderModuleCreateInfo,
    *mut ShaderModuleIdentifierEXT,
);
/// [`vkGetShaderModuleCreateInfoIdentifierEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleCreateInfoIdentifierEXT.html)
///
#[doc(alias = "vkGetShaderModuleCreateInfoIdentifierEXT")]
#[inline(always)]
pub unsafe fn get_shader_module_create_info_identifier_ext(
    device: DeviceHandle,
    create_info: *const ShaderModuleCreateInfo,
    identifier: *mut ShaderModuleIdentifierEXT,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceOpticalFlowImageFormatsNV = unsafe extern "C" fn(
    PhysicalDeviceHandle,
    *const OpticalFlowImageFormatInfoNV,
    *mut u32,
    *mut OpticalFlowImageFormatPropertiesNV,
)
    -> ResultCode;
/// [`vkGetPhysicalDeviceOpticalFlowImageFormatsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceOpticalFlowImageFormatsNV.html)
///
/// # Optional parameters
/// - image_format_properties
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
#[inline(always)]
pub unsafe fn get_physical_device_optical_flow_image_formats_nv(
    physical_device: PhysicalDeviceHandle,
    optical_flow_image_format_info: *const OpticalFlowImageFormatInfoNV,
    format_count: *mut u32,
    image_format_properties: *mut OpticalFlowImageFormatPropertiesNV,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateOpticalFlowSessionNV = unsafe extern "C" fn(
    DeviceHandle,
    *const OpticalFlowSessionCreateInfoNV,
    *const AllocationCallbacks,
    *mut OpticalFlowSessionNV,
) -> ResultCode;
/// [`vkCreateOpticalFlowSessionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateOpticalFlowSessionNV.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_optical_flow_session_nv(
    device: DeviceHandle,
    create_info: *const OpticalFlowSessionCreateInfoNV,
    allocator: *const AllocationCallbacks,
    session: *mut OpticalFlowSessionNV,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyOpticalFlowSessionNV =
    unsafe extern "C" fn(DeviceHandle, OpticalFlowSessionNV, *const AllocationCallbacks);
/// [`vkDestroyOpticalFlowSessionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyOpticalFlowSessionNV.html)
///
/// # Optional parameters
/// - allocator
///
#[doc(alias = "vkDestroyOpticalFlowSessionNV")]
#[inline(always)]
pub unsafe fn destroy_optical_flow_session_nv(
    device: DeviceHandle,
    session: OpticalFlowSessionNV,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_BindOpticalFlowSessionImageNV = unsafe extern "C" fn(
    DeviceHandle,
    OpticalFlowSessionNV,
    OpticalFlowSessionBindingPointNV,
    ImageView,
    ImageLayout,
) -> ResultCode;
/// [`vkBindOpticalFlowSessionImageNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindOpticalFlowSessionImageNV.html)
///
/// # Optional parameters
/// - view
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
#[inline(always)]
pub unsafe fn bind_optical_flow_session_image_nv(
    device: DeviceHandle,
    session: OpticalFlowSessionNV,
    binding_point: OpticalFlowSessionBindingPointNV,
    view: ImageView,
    layout: ImageLayout,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdOpticalFlowExecuteNV = unsafe extern "C" fn(
    CommandBufferHandle,
    OpticalFlowSessionNV,
    *const OpticalFlowExecuteInfoNV,
);
/// [`vkCmdOpticalFlowExecuteNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdOpticalFlowExecuteNV.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`OPTICAL_FLOWNV`](QueueFlags::OPTICAL_FLOWNV)
///
#[doc(alias = "vkCmdOpticalFlowExecuteNV")]
#[inline(always)]
pub unsafe fn cmd_optical_flow_execute_nv(
    command_buffer: CommandBufferHandle,
    session: OpticalFlowSessionNV,
    execute_info: *const OpticalFlowExecuteInfoNV,
) {
    todo!()
}

pub(crate) type FUN_AntiLagUpdateAMD = unsafe extern "C" fn(DeviceHandle, *const AntiLagDataAMD);
/// [`vkAntiLagUpdateAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAntiLagUpdateAMD.html)
///
#[doc(alias = "vkAntiLagUpdateAMD")]
#[inline(always)]
pub unsafe fn anti_lag_update_amd(device: DeviceHandle, data: *const AntiLagDataAMD) {
    todo!()
}

pub(crate) type FUN_CreateShadersEXT = unsafe extern "C" fn(
    DeviceHandle,
    u32,
    *const ShaderCreateInfoEXT,
    *const AllocationCallbacks,
    *mut ShaderEXT,
) -> ResultCode;
/// [`vkCreateShadersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShadersEXT.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_shaders_ext(
    device: DeviceHandle,
    create_info_count: u32,
    create_infos: *const ShaderCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    shaders: *mut ShaderEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyShaderEXT =
    unsafe extern "C" fn(DeviceHandle, ShaderEXT, *const AllocationCallbacks);
/// [`vkDestroyShaderEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderEXT.html)
///
/// # Optional parameters
/// - shader
/// - allocator
///
#[doc(alias = "vkDestroyShaderEXT")]
#[inline(always)]
pub unsafe fn destroy_shader_ext(
    device: DeviceHandle,
    shader: ShaderEXT,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_GetShaderBinaryDataEXT =
    unsafe extern "C" fn(DeviceHandle, ShaderEXT, *mut usize, *mut c_void) -> ResultCode;
/// [`vkGetShaderBinaryDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderBinaryDataEXT.html)
///
/// # Optional parameters
/// - data
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
#[inline(always)]
pub unsafe fn get_shader_binary_data_ext(
    device: DeviceHandle,
    shader: ShaderEXT,
    data_size: *mut usize,
    data: *mut c_void,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdBindShadersEXT =
    unsafe extern "C" fn(CommandBufferHandle, u32, *const ShaderStageFlags, *const ShaderEXT);
/// [`vkCmdBindShadersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindShadersEXT.html)
///
/// # Optional parameters
/// - shaders
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdBindShadersEXT")]
#[inline(always)]
pub unsafe fn cmd_bind_shaders_ext(
    command_buffer: CommandBufferHandle,
    stage_count: u32,
    stages: *const ShaderStageFlags,
    shaders: *const ShaderEXT,
) {
    todo!()
}

pub(crate) type FUN_CmdSetDepthClampRangeEXT =
    unsafe extern "C" fn(CommandBufferHandle, DepthClampModeEXT, *const DepthClampRangeEXT);
/// [`vkCmdSetDepthClampRangeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClampRangeEXT.html)
///
/// # Optional parameters
/// - depth_clamp_range
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetDepthClampRangeEXT")]
#[inline(always)]
pub unsafe fn cmd_set_depth_clamp_range_ext(
    command_buffer: CommandBufferHandle,
    depth_clamp_mode: DepthClampModeEXT,
    depth_clamp_range: *const DepthClampRangeEXT,
) {
    todo!()
}

pub(crate) type FUN_GetFramebufferTilePropertiesQCOM = unsafe extern "C" fn(
    DeviceHandle,
    Framebuffer,
    *mut u32,
    *mut TilePropertiesQCOM,
) -> ResultCode;
/// [`vkGetFramebufferTilePropertiesQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFramebufferTilePropertiesQCOM.html)
///
/// # Optional parameters
/// - properties
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// - [`INCOMPLETE`](ResultCode::INCOMPLETE)
/// ## Error
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkGetFramebufferTilePropertiesQCOM")]
#[inline(always)]
pub unsafe fn get_framebuffer_tile_properties_qcom(
    device: DeviceHandle,
    framebuffer: Framebuffer,
    properties_count: *mut u32,
    properties: *mut TilePropertiesQCOM,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetDynamicRenderingTilePropertiesQCOM =
    unsafe extern "C" fn(DeviceHandle, *const RenderingInfo, *mut TilePropertiesQCOM) -> ResultCode;
/// [`vkGetDynamicRenderingTilePropertiesQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDynamicRenderingTilePropertiesQCOM.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkGetDynamicRenderingTilePropertiesQCOM")]
#[inline(always)]
pub unsafe fn get_dynamic_rendering_tile_properties_qcom(
    device: DeviceHandle,
    rendering_info: *const RenderingInfo,
    properties: *mut TilePropertiesQCOM,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceCooperativeVectorPropertiesNV =
    unsafe extern "C" fn(
        PhysicalDeviceHandle,
        *mut u32,
        *mut CooperativeVectorPropertiesNV,
    ) -> ResultCode;
/// [`vkGetPhysicalDeviceCooperativeVectorPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeVectorPropertiesNV.html)
///
/// # Optional parameters
/// - properties
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
#[inline(always)]
pub unsafe fn get_physical_device_cooperative_vector_properties_nv(
    physical_device: PhysicalDeviceHandle,
    property_count: *mut u32,
    properties: *mut CooperativeVectorPropertiesNV,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_ConvertCooperativeVectorMatrixNV =
    unsafe extern "C" fn(DeviceHandle, *const ConvertCooperativeVectorMatrixInfoNV) -> ResultCode;
/// [`vkConvertCooperativeVectorMatrixNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkConvertCooperativeVectorMatrixNV.html)
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
#[inline(always)]
pub unsafe fn convert_cooperative_vector_matrix_nv(
    device: DeviceHandle,
    info: *const ConvertCooperativeVectorMatrixInfoNV,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdConvertCooperativeVectorMatrixNV =
    unsafe extern "C" fn(CommandBufferHandle, u32, *const ConvertCooperativeVectorMatrixInfoNV);
/// [`vkCmdConvertCooperativeVectorMatrixNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdConvertCooperativeVectorMatrixNV.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdConvertCooperativeVectorMatrixNV")]
#[inline(always)]
pub unsafe fn cmd_convert_cooperative_vector_matrix_nv(
    command_buffer: CommandBufferHandle,
    info_count: u32,
    infos: *const ConvertCooperativeVectorMatrixInfoNV,
) {
    todo!()
}

pub(crate) type FUN_SetLatencySleepModeNV =
    unsafe extern "C" fn(DeviceHandle, SwapchainKHR, *const LatencySleepModeInfoNV) -> ResultCode;
/// [`vkSetLatencySleepModeNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencySleepModeNV.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`INITIALIZATION_FAILED`](ResultCode::ERROR_INITIALIZATION_FAILED)
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkSetLatencySleepModeNV")]
#[inline(always)]
pub unsafe fn set_latency_sleep_mode_nv(
    device: DeviceHandle,
    swapchain: SwapchainKHR,
    sleep_mode_info: *const LatencySleepModeInfoNV,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_LatencySleepNV =
    unsafe extern "C" fn(DeviceHandle, SwapchainKHR, *const LatencySleepInfoNV) -> ResultCode;
/// [`vkLatencySleepNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkLatencySleepNV.html)
///
/// # Result codes
/// ## Success
/// - [`SUCCESS`](ResultCode::SUCCESS)
/// ## Error
/// - [`UNKNOWN`](ResultCode::ERROR_UNKNOWN)
/// - [`VALIDATION_FAILED`](ResultCode::ERROR_VALIDATION_FAILED)
#[doc(alias = "vkLatencySleepNV")]
#[inline(always)]
pub unsafe fn latency_sleep_nv(
    device: DeviceHandle,
    swapchain: SwapchainKHR,
    sleep_info: *const LatencySleepInfoNV,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_SetLatencyMarkerNV =
    unsafe extern "C" fn(DeviceHandle, SwapchainKHR, *const SetLatencyMarkerInfoNV);
/// [`vkSetLatencyMarkerNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencyMarkerNV.html)
///
#[doc(alias = "vkSetLatencyMarkerNV")]
#[inline(always)]
pub unsafe fn set_latency_marker_nv(
    device: DeviceHandle,
    swapchain: SwapchainKHR,
    latency_marker_info: *const SetLatencyMarkerInfoNV,
) {
    todo!()
}

pub(crate) type FUN_GetLatencyTimingsNV =
    unsafe extern "C" fn(DeviceHandle, SwapchainKHR, *mut GetLatencyMarkerInfoNV);
/// [`vkGetLatencyTimingsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetLatencyTimingsNV.html)
///
#[doc(alias = "vkGetLatencyTimingsNV")]
#[inline(always)]
pub unsafe fn get_latency_timings_nv(
    device: DeviceHandle,
    swapchain: SwapchainKHR,
    latency_marker_info: *mut GetLatencyMarkerInfoNV,
) {
    todo!()
}

pub(crate) type FUN_QueueNotifyOutOfBandNV =
    unsafe extern "C" fn(QueueHandle, *const OutOfBandQueueTypeInfoNV);
/// [`vkQueueNotifyOutOfBandNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueNotifyOutOfBandNV.html)
///
#[doc(alias = "vkQueueNotifyOutOfBandNV")]
#[inline(always)]
pub unsafe fn queue_notify_out_of_band_nv(
    queue: QueueHandle,
    queue_type_info: *const OutOfBandQueueTypeInfoNV,
) {
    todo!()
}

pub(crate) type FUN_CreateDataGraphPipelinesARM = unsafe extern "C" fn(
    DeviceHandle,
    DeferredOperationKHR,
    PipelineCache,
    u32,
    *const DataGraphPipelineCreateInfoARM,
    *const AllocationCallbacks,
    *mut Pipeline,
) -> ResultCode;
/// [`vkCreateDataGraphPipelinesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDataGraphPipelinesARM.html)
///
/// # Optional parameters
/// - deferred_operation
/// - pipeline_cache
/// - allocator
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
#[inline(always)]
pub unsafe fn create_data_graph_pipelines_arm(
    device: DeviceHandle,
    deferred_operation: DeferredOperationKHR,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    create_infos: *const DataGraphPipelineCreateInfoARM,
    allocator: *const AllocationCallbacks,
    pipelines: *mut Pipeline,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateDataGraphPipelineSessionARM = unsafe extern "C" fn(
    DeviceHandle,
    *const DataGraphPipelineSessionCreateInfoARM,
    *const AllocationCallbacks,
    *mut DataGraphPipelineSessionARM,
) -> ResultCode;
/// [`vkCreateDataGraphPipelineSessionARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDataGraphPipelineSessionARM.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_data_graph_pipeline_session_arm(
    device: DeviceHandle,
    create_info: *const DataGraphPipelineSessionCreateInfoARM,
    allocator: *const AllocationCallbacks,
    session: *mut DataGraphPipelineSessionARM,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetDataGraphPipelineSessionBindPointRequirementsARM =
    unsafe extern "C" fn(
        DeviceHandle,
        *const DataGraphPipelineSessionBindPointRequirementsInfoARM,
        *mut u32,
        *mut DataGraphPipelineSessionBindPointRequirementARM,
    ) -> ResultCode;
/// [`vkGetDataGraphPipelineSessionBindPointRequirementsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineSessionBindPointRequirementsARM.html)
///
/// # Optional parameters
/// - bind_point_requirements
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
#[inline(always)]
pub unsafe fn get_data_graph_pipeline_session_bind_point_requirements_arm(
    device: DeviceHandle,
    info: *const DataGraphPipelineSessionBindPointRequirementsInfoARM,
    bind_point_requirement_count: *mut u32,
    bind_point_requirements: *mut DataGraphPipelineSessionBindPointRequirementARM,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetDataGraphPipelineSessionMemoryRequirementsARM = unsafe extern "C" fn(
    DeviceHandle,
    *const DataGraphPipelineSessionMemoryRequirementsInfoARM,
    *mut MemoryRequirements2,
);
/// [`vkGetDataGraphPipelineSessionMemoryRequirementsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineSessionMemoryRequirementsARM.html)
///
#[doc(alias = "vkGetDataGraphPipelineSessionMemoryRequirementsARM")]
#[inline(always)]
pub unsafe fn get_data_graph_pipeline_session_memory_requirements_arm(
    device: DeviceHandle,
    info: *const DataGraphPipelineSessionMemoryRequirementsInfoARM,
    memory_requirements: *mut MemoryRequirements2,
) {
    todo!()
}

pub(crate) type FUN_BindDataGraphPipelineSessionMemoryARM = unsafe extern "C" fn(
    DeviceHandle,
    u32,
    *const BindDataGraphPipelineSessionMemoryInfoARM,
) -> ResultCode;
/// [`vkBindDataGraphPipelineSessionMemoryARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindDataGraphPipelineSessionMemoryARM.html)
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
#[inline(always)]
pub unsafe fn bind_data_graph_pipeline_session_memory_arm(
    device: DeviceHandle,
    bind_info_count: u32,
    bind_infos: *const BindDataGraphPipelineSessionMemoryInfoARM,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyDataGraphPipelineSessionARM =
    unsafe extern "C" fn(DeviceHandle, DataGraphPipelineSessionARM, *const AllocationCallbacks);
/// [`vkDestroyDataGraphPipelineSessionARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDataGraphPipelineSessionARM.html)
///
/// # Optional parameters
/// - allocator
///
#[doc(alias = "vkDestroyDataGraphPipelineSessionARM")]
#[inline(always)]
pub unsafe fn destroy_data_graph_pipeline_session_arm(
    device: DeviceHandle,
    session: DataGraphPipelineSessionARM,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_CmdDispatchDataGraphARM = unsafe extern "C" fn(
    CommandBufferHandle,
    DataGraphPipelineSessionARM,
    *const DataGraphPipelineDispatchInfoARM,
);
/// [`vkCmdDispatchDataGraphARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchDataGraphARM.html)
///
/// # Optional parameters
/// - info
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`DATA_GRAPHARM`](QueueFlags::DATA_GRAPHARM)
///
#[doc(alias = "vkCmdDispatchDataGraphARM")]
#[inline(always)]
pub unsafe fn cmd_dispatch_data_graph_arm(
    command_buffer: CommandBufferHandle,
    session: DataGraphPipelineSessionARM,
    info: *const DataGraphPipelineDispatchInfoARM,
) {
    todo!()
}

pub(crate) type FUN_GetDataGraphPipelineAvailablePropertiesARM = unsafe extern "C" fn(
    DeviceHandle,
    *const DataGraphPipelineInfoARM,
    *mut u32,
    *mut DataGraphPipelinePropertyARM,
)
    -> ResultCode;
/// [`vkGetDataGraphPipelineAvailablePropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineAvailablePropertiesARM.html)
///
/// # Optional parameters
/// - properties
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
#[inline(always)]
pub unsafe fn get_data_graph_pipeline_available_properties_arm(
    device: DeviceHandle,
    pipeline_info: *const DataGraphPipelineInfoARM,
    properties_count: *mut u32,
    properties: *mut DataGraphPipelinePropertyARM,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetDataGraphPipelinePropertiesARM = unsafe extern "C" fn(
    DeviceHandle,
    *const DataGraphPipelineInfoARM,
    u32,
    *mut DataGraphPipelinePropertyQueryResultARM,
) -> ResultCode;
/// [`vkGetDataGraphPipelinePropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelinePropertiesARM.html)
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
#[inline(always)]
pub unsafe fn get_data_graph_pipeline_properties_arm(
    device: DeviceHandle,
    pipeline_info: *const DataGraphPipelineInfoARM,
    properties_count: u32,
    properties: *mut DataGraphPipelinePropertyQueryResultARM,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceQueueFamilyDataGraphPropertiesARM =
    unsafe extern "C" fn(
        PhysicalDeviceHandle,
        u32,
        *mut u32,
        *mut QueueFamilyDataGraphPropertiesARM,
    ) -> ResultCode;
/// [`vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM.html)
///
/// # Optional parameters
/// - queue_family_data_graph_properties
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
#[inline(always)]
pub unsafe fn get_physical_device_queue_family_data_graph_properties_arm(
    physical_device: PhysicalDeviceHandle,
    queue_family_index: u32,
    queue_family_data_graph_property_count: *mut u32,
    queue_family_data_graph_properties: *mut QueueFamilyDataGraphPropertiesARM,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM =
    unsafe extern "C" fn(
        PhysicalDeviceHandle,
        *const PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM,
        *mut QueueFamilyDataGraphProcessingEnginePropertiesARM,
    );
/// [`vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM.html)
///
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM")]
#[inline(always)]
pub unsafe fn get_physical_device_queue_family_data_graph_processing_engine_properties_arm(
    physical_device: PhysicalDeviceHandle,
    queue_family_data_graph_processing_engine_info: *const PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM,
    queue_family_data_graph_processing_engine_properties: *mut QueueFamilyDataGraphProcessingEnginePropertiesARM,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM =
    unsafe extern "C" fn(
        PhysicalDeviceHandle,
        u32,
        *const QueueFamilyDataGraphPropertiesARM,
        *mut BaseOutStructure,
    ) -> ResultCode;
/// [`vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM.html)
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
#[inline(always)]
pub unsafe fn get_physical_device_queue_family_data_graph_engine_operation_properties_arm(
    physical_device: PhysicalDeviceHandle,
    queue_family_index: u32,
    queue_family_data_graph_properties: *const QueueFamilyDataGraphPropertiesARM,
    properties: *mut BaseOutStructure,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdSetAttachmentFeedbackLoopEnableEXT =
    unsafe extern "C" fn(CommandBufferHandle, ImageAspectFlags);
/// [`vkCmdSetAttachmentFeedbackLoopEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAttachmentFeedbackLoopEnableEXT.html)
///
/// # Optional parameters
/// - aspect_mask
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetAttachmentFeedbackLoopEnableEXT")]
#[inline(always)]
pub unsafe fn cmd_set_attachment_feedback_loop_enable_ext(
    command_buffer: CommandBufferHandle,
    aspect_mask: ImageAspectFlags,
) {
    todo!()
}

pub(crate) type FUN_GetScreenBufferPropertiesQNX = unsafe extern "C" fn(
    DeviceHandle,
    *const _screen_buffer,
    *mut ScreenBufferPropertiesQNX,
) -> ResultCode;
/// [`vkGetScreenBufferPropertiesQNX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetScreenBufferPropertiesQNX.html)
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
#[inline(always)]
pub unsafe fn get_screen_buffer_properties_qnx(
    device: DeviceHandle,
    buffer: *const _screen_buffer,
    properties: *mut ScreenBufferPropertiesQNX,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdBindTileMemoryQCOM =
    unsafe extern "C" fn(CommandBufferHandle, *const TileMemoryBindInfoQCOM);
/// [`vkCmdBindTileMemoryQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTileMemoryQCOM.html)
///
/// # Optional parameters
/// - tile_memory_bind_info
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdBindTileMemoryQCOM")]
#[inline(always)]
pub unsafe fn cmd_bind_tile_memory_qcom(
    command_buffer: CommandBufferHandle,
    tile_memory_bind_info: *const TileMemoryBindInfoQCOM,
) {
    todo!()
}

pub(crate) type FUN_CmdDecompressMemoryEXT =
    unsafe extern "C" fn(CommandBufferHandle, *const DecompressMemoryInfoEXT);
/// [`vkCmdDecompressMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryEXT.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdDecompressMemoryEXT")]
#[inline(always)]
pub unsafe fn cmd_decompress_memory_ext(
    command_buffer: CommandBufferHandle,
    decompress_memory_info_ext: *const DecompressMemoryInfoEXT,
) {
    todo!()
}

pub(crate) type FUN_CmdDecompressMemoryIndirectCountEXT = unsafe extern "C" fn(
    CommandBufferHandle,
    MemoryDecompressionMethodFlagsEXT,
    DeviceAddress,
    DeviceAddress,
    u32,
    u32,
);
/// [`vkCmdDecompressMemoryIndirectCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryIndirectCountEXT.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdDecompressMemoryIndirectCountEXT")]
#[inline(always)]
pub unsafe fn cmd_decompress_memory_indirect_count_ext(
    command_buffer: CommandBufferHandle,
    decompression_method: MemoryDecompressionMethodFlagsEXT,
    indirect_commands_address: DeviceAddress,
    indirect_commands_count_address: DeviceAddress,
    max_decompression_count: u32,
    stride: u32,
) {
    todo!()
}

pub(crate) type FUN_CreateExternalComputeQueueNV = unsafe extern "C" fn(
    DeviceHandle,
    *const ExternalComputeQueueCreateInfoNV,
    *const AllocationCallbacks,
    *mut ExternalComputeQueueNVHandle,
) -> ResultCode;
/// [`vkCreateExternalComputeQueueNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateExternalComputeQueueNV.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_external_compute_queue_nv(
    device: DeviceHandle,
    create_info: *const ExternalComputeQueueCreateInfoNV,
    allocator: *const AllocationCallbacks,
    external_queue: *mut ExternalComputeQueueNVHandle,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyExternalComputeQueueNV =
    unsafe extern "C" fn(DeviceHandle, ExternalComputeQueueNVHandle, *const AllocationCallbacks);
/// [`vkDestroyExternalComputeQueueNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyExternalComputeQueueNV.html)
///
/// # Optional parameters
/// - allocator
///
#[doc(alias = "vkDestroyExternalComputeQueueNV")]
#[inline(always)]
pub unsafe fn destroy_external_compute_queue_nv(
    device: DeviceHandle,
    external_queue: ExternalComputeQueueNVHandle,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_GetExternalComputeQueueDataNV = unsafe extern "C" fn(
    ExternalComputeQueueNVHandle,
    *mut ExternalComputeQueueDataParamsNV,
    *mut c_void,
);
/// [`vkGetExternalComputeQueueDataNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExternalComputeQueueDataNV.html)
///
#[doc(alias = "vkGetExternalComputeQueueDataNV")]
#[inline(always)]
pub unsafe fn get_external_compute_queue_data_nv(
    external_queue: ExternalComputeQueueNVHandle,
    params: *mut ExternalComputeQueueDataParamsNV,
    data: *mut c_void,
) {
    todo!()
}

pub(crate) type FUN_GetClusterAccelerationStructureBuildSizesNV = unsafe extern "C" fn(
    DeviceHandle,
    *const ClusterAccelerationStructureInputInfoNV,
    *mut AccelerationStructureBuildSizesInfoKHR,
);
/// [`vkGetClusterAccelerationStructureBuildSizesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetClusterAccelerationStructureBuildSizesNV.html)
///
#[doc(alias = "vkGetClusterAccelerationStructureBuildSizesNV")]
#[inline(always)]
pub unsafe fn get_cluster_acceleration_structure_build_sizes_nv(
    device: DeviceHandle,
    info: *const ClusterAccelerationStructureInputInfoNV,
    size_info: *mut AccelerationStructureBuildSizesInfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdBuildClusterAccelerationStructureIndirectNV =
    unsafe extern "C" fn(CommandBufferHandle, *const ClusterAccelerationStructureCommandsInfoNV);
/// [`vkCmdBuildClusterAccelerationStructureIndirectNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildClusterAccelerationStructureIndirectNV.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdBuildClusterAccelerationStructureIndirectNV")]
#[inline(always)]
pub unsafe fn cmd_build_cluster_acceleration_structure_indirect_nv(
    command_buffer: CommandBufferHandle,
    command_infos: *const ClusterAccelerationStructureCommandsInfoNV,
) {
    todo!()
}

pub(crate) type FUN_GetPartitionedAccelerationStructuresBuildSizesNV = unsafe extern "C" fn(
    DeviceHandle,
    *const PartitionedAccelerationStructureInstancesInputNV,
    *mut AccelerationStructureBuildSizesInfoKHR,
);
/// [`vkGetPartitionedAccelerationStructuresBuildSizesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPartitionedAccelerationStructuresBuildSizesNV.html)
///
#[doc(alias = "vkGetPartitionedAccelerationStructuresBuildSizesNV")]
#[inline(always)]
pub unsafe fn get_partitioned_acceleration_structures_build_sizes_nv(
    device: DeviceHandle,
    info: *const PartitionedAccelerationStructureInstancesInputNV,
    size_info: *mut AccelerationStructureBuildSizesInfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdBuildPartitionedAccelerationStructuresNV =
    unsafe extern "C" fn(CommandBufferHandle, *const BuildPartitionedAccelerationStructureInfoNV);
/// [`vkCmdBuildPartitionedAccelerationStructuresNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildPartitionedAccelerationStructuresNV.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdBuildPartitionedAccelerationStructuresNV")]
#[inline(always)]
pub unsafe fn cmd_build_partitioned_acceleration_structures_nv(
    command_buffer: CommandBufferHandle,
    build_info: *const BuildPartitionedAccelerationStructureInfoNV,
) {
    todo!()
}

pub(crate) type FUN_GetGeneratedCommandsMemoryRequirementsEXT = unsafe extern "C" fn(
    DeviceHandle,
    *const GeneratedCommandsMemoryRequirementsInfoEXT,
    *mut MemoryRequirements2,
);
/// [`vkGetGeneratedCommandsMemoryRequirementsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGeneratedCommandsMemoryRequirementsEXT.html)
///
#[doc(alias = "vkGetGeneratedCommandsMemoryRequirementsEXT")]
#[inline(always)]
pub unsafe fn get_generated_commands_memory_requirements_ext(
    device: DeviceHandle,
    info: *const GeneratedCommandsMemoryRequirementsInfoEXT,
    memory_requirements: *mut MemoryRequirements2,
) {
    todo!()
}

pub(crate) type FUN_CmdPreprocessGeneratedCommandsEXT =
    unsafe extern "C" fn(CommandBufferHandle, *const GeneratedCommandsInfoEXT, CommandBufferHandle);
/// [`vkCmdPreprocessGeneratedCommandsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPreprocessGeneratedCommandsEXT.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdPreprocessGeneratedCommandsEXT")]
#[inline(always)]
pub unsafe fn cmd_preprocess_generated_commands_ext(
    command_buffer: CommandBufferHandle,
    generated_commands_info: *const GeneratedCommandsInfoEXT,
    state_command_buffer: CommandBufferHandle,
) {
    todo!()
}

pub(crate) type FUN_CmdExecuteGeneratedCommandsEXT =
    unsafe extern "C" fn(CommandBufferHandle, Bool32, *const GeneratedCommandsInfoEXT);
/// [`vkCmdExecuteGeneratedCommandsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteGeneratedCommandsEXT.html)
///
/// # Performed tasks
/// - `action`
/// - `indirection`
///
/// # Allowed command buffers
/// - Primary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdExecuteGeneratedCommandsEXT")]
#[inline(always)]
pub unsafe fn cmd_execute_generated_commands_ext(
    command_buffer: CommandBufferHandle,
    is_preprocessed: Bool32,
    generated_commands_info: *const GeneratedCommandsInfoEXT,
) {
    todo!()
}

pub(crate) type FUN_CreateIndirectCommandsLayoutEXT = unsafe extern "C" fn(
    DeviceHandle,
    *const IndirectCommandsLayoutCreateInfoEXT,
    *const AllocationCallbacks,
    *mut IndirectCommandsLayoutEXT,
) -> ResultCode;
/// [`vkCreateIndirectCommandsLayoutEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectCommandsLayoutEXT.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_indirect_commands_layout_ext(
    device: DeviceHandle,
    create_info: *const IndirectCommandsLayoutCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    indirect_commands_layout: *mut IndirectCommandsLayoutEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyIndirectCommandsLayoutEXT =
    unsafe extern "C" fn(DeviceHandle, IndirectCommandsLayoutEXT, *const AllocationCallbacks);
/// [`vkDestroyIndirectCommandsLayoutEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectCommandsLayoutEXT.html)
///
/// # Optional parameters
/// - indirect_commands_layout
/// - allocator
///
#[doc(alias = "vkDestroyIndirectCommandsLayoutEXT")]
#[inline(always)]
pub unsafe fn destroy_indirect_commands_layout_ext(
    device: DeviceHandle,
    indirect_commands_layout: IndirectCommandsLayoutEXT,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_CreateIndirectExecutionSetEXT = unsafe extern "C" fn(
    DeviceHandle,
    *const IndirectExecutionSetCreateInfoEXT,
    *const AllocationCallbacks,
    *mut IndirectExecutionSetEXT,
) -> ResultCode;
/// [`vkCreateIndirectExecutionSetEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectExecutionSetEXT.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_indirect_execution_set_ext(
    device: DeviceHandle,
    create_info: *const IndirectExecutionSetCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    indirect_execution_set: *mut IndirectExecutionSetEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyIndirectExecutionSetEXT =
    unsafe extern "C" fn(DeviceHandle, IndirectExecutionSetEXT, *const AllocationCallbacks);
/// [`vkDestroyIndirectExecutionSetEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectExecutionSetEXT.html)
///
/// # Optional parameters
/// - indirect_execution_set
/// - allocator
///
#[doc(alias = "vkDestroyIndirectExecutionSetEXT")]
#[inline(always)]
pub unsafe fn destroy_indirect_execution_set_ext(
    device: DeviceHandle,
    indirect_execution_set: IndirectExecutionSetEXT,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_UpdateIndirectExecutionSetPipelineEXT = unsafe extern "C" fn(
    DeviceHandle,
    IndirectExecutionSetEXT,
    u32,
    *const WriteIndirectExecutionSetPipelineEXT,
);
/// [`vkUpdateIndirectExecutionSetPipelineEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateIndirectExecutionSetPipelineEXT.html)
///
#[doc(alias = "vkUpdateIndirectExecutionSetPipelineEXT")]
#[inline(always)]
pub unsafe fn update_indirect_execution_set_pipeline_ext(
    device: DeviceHandle,
    indirect_execution_set: IndirectExecutionSetEXT,
    execution_set_write_count: u32,
    execution_set_writes: *const WriteIndirectExecutionSetPipelineEXT,
) {
    todo!()
}

pub(crate) type FUN_UpdateIndirectExecutionSetShaderEXT = unsafe extern "C" fn(
    DeviceHandle,
    IndirectExecutionSetEXT,
    u32,
    *const WriteIndirectExecutionSetShaderEXT,
);
/// [`vkUpdateIndirectExecutionSetShaderEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateIndirectExecutionSetShaderEXT.html)
///
#[doc(alias = "vkUpdateIndirectExecutionSetShaderEXT")]
#[inline(always)]
pub unsafe fn update_indirect_execution_set_shader_ext(
    device: DeviceHandle,
    indirect_execution_set: IndirectExecutionSetEXT,
    execution_set_write_count: u32,
    execution_set_writes: *const WriteIndirectExecutionSetShaderEXT,
) {
    todo!()
}

pub(crate) type FUN_CreateSurfaceOHOS = unsafe extern "C" fn(
    InstanceHandle,
    *const SurfaceCreateInfoOHOS,
    *const AllocationCallbacks,
    *mut SurfaceKHR,
) -> ResultCode;
/// [`vkCreateSurfaceOHOS`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSurfaceOHOS.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_surface_ohos(
    instance: InstanceHandle,
    create_info: *const SurfaceCreateInfoOHOS,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV =
    unsafe extern "C" fn(
        PhysicalDeviceHandle,
        *mut u32,
        *mut CooperativeMatrixFlexibleDimensionsPropertiesNV,
    ) -> ResultCode;
/// [`vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV.html)
///
/// # Optional parameters
/// - properties
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
#[inline(always)]
pub unsafe fn get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv(
    physical_device: PhysicalDeviceHandle,
    property_count: *mut u32,
    properties: *mut CooperativeMatrixFlexibleDimensionsPropertiesNV,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetMemoryMetalHandleEXT = unsafe extern "C" fn(
    DeviceHandle,
    *const MemoryGetMetalHandleInfoEXT,
    *mut *mut c_void,
) -> ResultCode;
/// [`vkGetMemoryMetalHandleEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryMetalHandleEXT.html)
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
#[inline(always)]
pub unsafe fn get_memory_metal_handle_ext(
    device: DeviceHandle,
    get_metal_handle_info: *const MemoryGetMetalHandleInfoEXT,
    handle: *mut *mut c_void,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetMemoryMetalHandlePropertiesEXT = unsafe extern "C" fn(
    DeviceHandle,
    ExternalMemoryHandleTypeFlags,
    *const c_void,
    *mut MemoryMetalHandlePropertiesEXT,
) -> ResultCode;
/// [`vkGetMemoryMetalHandlePropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryMetalHandlePropertiesEXT.html)
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
#[inline(always)]
pub unsafe fn get_memory_metal_handle_properties_ext(
    device: DeviceHandle,
    handle_type: ExternalMemoryHandleTypeFlags,
    handle: *const c_void,
    memory_metal_handle_properties: *mut MemoryMetalHandlePropertiesEXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_EnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM =
    unsafe extern "C" fn(
        PhysicalDeviceHandle,
        u32,
        *mut u32,
        *mut PerformanceCounterARM,
        *mut PerformanceCounterDescriptionARM,
    ) -> ResultCode;
/// [`vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM.html)
///
/// # Optional parameters
/// - counters
/// - counter_descriptions
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
#[inline(always)]
pub unsafe fn enumerate_physical_device_queue_family_performance_counters_by_region_arm(
    physical_device: PhysicalDeviceHandle,
    queue_family_index: u32,
    counter_count: *mut u32,
    counters: *mut PerformanceCounterARM,
    counter_descriptions: *mut PerformanceCounterDescriptionARM,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_EnumeratePhysicalDeviceShaderInstrumentationMetricsARM =
    unsafe extern "C" fn(
        PhysicalDeviceHandle,
        *mut u32,
        *mut ShaderInstrumentationMetricDescriptionARM,
    ) -> ResultCode;
/// [`vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM.html)
///
/// # Optional parameters
/// - descriptions
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
#[inline(always)]
pub unsafe fn enumerate_physical_device_shader_instrumentation_metrics_arm(
    physical_device: PhysicalDeviceHandle,
    description_count: *mut u32,
    descriptions: *mut ShaderInstrumentationMetricDescriptionARM,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateShaderInstrumentationARM = unsafe extern "C" fn(
    DeviceHandle,
    *const ShaderInstrumentationCreateInfoARM,
    *const AllocationCallbacks,
    *mut ShaderInstrumentationARM,
) -> ResultCode;
/// [`vkCreateShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShaderInstrumentationARM.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_shader_instrumentation_arm(
    device: DeviceHandle,
    create_info: *const ShaderInstrumentationCreateInfoARM,
    allocator: *const AllocationCallbacks,
    instrumentation: *mut ShaderInstrumentationARM,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyShaderInstrumentationARM =
    unsafe extern "C" fn(DeviceHandle, ShaderInstrumentationARM, *const AllocationCallbacks);
/// [`vkDestroyShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderInstrumentationARM.html)
///
/// # Optional parameters
/// - instrumentation
/// - allocator
///
#[doc(alias = "vkDestroyShaderInstrumentationARM")]
#[inline(always)]
pub unsafe fn destroy_shader_instrumentation_arm(
    device: DeviceHandle,
    instrumentation: ShaderInstrumentationARM,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_CmdBeginShaderInstrumentationARM =
    unsafe extern "C" fn(CommandBufferHandle, ShaderInstrumentationARM);
/// [`vkCmdBeginShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginShaderInstrumentationARM.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`DATA_GRAPHARM`](QueueFlags::DATA_GRAPHARM)
///
#[doc(alias = "vkCmdBeginShaderInstrumentationARM")]
#[inline(always)]
pub unsafe fn cmd_begin_shader_instrumentation_arm(
    command_buffer: CommandBufferHandle,
    instrumentation: ShaderInstrumentationARM,
) {
    todo!()
}

pub(crate) type FUN_CmdEndShaderInstrumentationARM = unsafe extern "C" fn(CommandBufferHandle);
/// [`vkCmdEndShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndShaderInstrumentationARM.html)
///
/// # Performed tasks
/// - `action`
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
/// - [`COMPUTE`](QueueFlags::COMPUTE)
/// - [`DATA_GRAPHARM`](QueueFlags::DATA_GRAPHARM)
///
#[doc(alias = "vkCmdEndShaderInstrumentationARM")]
#[inline(always)]
pub unsafe fn cmd_end_shader_instrumentation_arm(command_buffer: CommandBufferHandle) {
    todo!()
}

pub(crate) type FUN_GetShaderInstrumentationValuesARM = unsafe extern "C" fn(
    DeviceHandle,
    ShaderInstrumentationARM,
    *mut u32,
    *mut c_void,
    ShaderInstrumentationValuesFlagsARM,
) -> ResultCode;
/// [`vkGetShaderInstrumentationValuesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderInstrumentationValuesARM.html)
///
/// # Optional parameters
/// - metric_values
/// - flags
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
#[inline(always)]
pub unsafe fn get_shader_instrumentation_values_arm(
    device: DeviceHandle,
    instrumentation: ShaderInstrumentationARM,
    metric_block_count: *mut u32,
    metric_values: *mut c_void,
    flags: ShaderInstrumentationValuesFlagsARM,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_ClearShaderInstrumentationMetricsARM =
    unsafe extern "C" fn(DeviceHandle, ShaderInstrumentationARM);
/// [`vkClearShaderInstrumentationMetricsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkClearShaderInstrumentationMetricsARM.html)
///
#[doc(alias = "vkClearShaderInstrumentationMetricsARM")]
#[inline(always)]
pub unsafe fn clear_shader_instrumentation_metrics_arm(
    device: DeviceHandle,
    instrumentation: ShaderInstrumentationARM,
) {
    todo!()
}

pub(crate) type FUN_CmdEndRendering2EXT =
    unsafe extern "C" fn(CommandBufferHandle, *const RenderingEndInfoKHR);
/// [`vkCmdEndRendering2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering2EXT.html)
///
/// # Optional parameters
/// - rendering_end_info
///
/// # Performed tasks
/// - `action`
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdEndRendering2EXT")]
#[inline(always)]
pub unsafe fn cmd_end_rendering_2_ext(
    command_buffer: CommandBufferHandle,
    rendering_end_info: *const RenderingEndInfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdBeginCustomResolveEXT =
    unsafe extern "C" fn(CommandBufferHandle, *const BeginCustomResolveInfoEXT);
/// [`vkCmdBeginCustomResolveEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginCustomResolveEXT.html)
///
/// # Optional parameters
/// - begin_custom_resolve_info
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdBeginCustomResolveEXT")]
#[inline(always)]
pub unsafe fn cmd_begin_custom_resolve_ext(
    command_buffer: CommandBufferHandle,
    begin_custom_resolve_info: *const BeginCustomResolveInfoEXT,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM =
    unsafe extern "C" fn(
        PhysicalDeviceHandle,
        u32,
        *const QueueFamilyDataGraphPropertiesARM,
        *const DataGraphOpticalFlowImageFormatInfoARM,
        *mut u32,
        *mut DataGraphOpticalFlowImageFormatPropertiesARM,
    ) -> ResultCode;
/// [`vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM.html)
///
/// # Optional parameters
/// - image_format_properties
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
#[inline(always)]
pub unsafe fn get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm(
    physical_device: PhysicalDeviceHandle,
    queue_family_index: u32,
    queue_family_data_graph_properties: *const QueueFamilyDataGraphPropertiesARM,
    optical_flow_image_format_info: *const DataGraphOpticalFlowImageFormatInfoARM,
    format_count: *mut u32,
    image_format_properties: *mut DataGraphOpticalFlowImageFormatPropertiesARM,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdSetComputeOccupancyPriorityNV =
    unsafe extern "C" fn(CommandBufferHandle, *const ComputeOccupancyPriorityParametersNV);
/// [`vkCmdSetComputeOccupancyPriorityNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetComputeOccupancyPriorityNV.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdSetComputeOccupancyPriorityNV")]
#[inline(always)]
pub unsafe fn cmd_set_compute_occupancy_priority_nv(
    command_buffer: CommandBufferHandle,
    parameters: *const ComputeOccupancyPriorityParametersNV,
) {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceCooperativeMatrixProperties2EXT =
    unsafe extern "C" fn(
        PhysicalDeviceHandle,
        *const PhysicalDeviceCooperativeMatrixInfo2EXT,
        *mut u32,
        *mut CooperativeMatrixProperties2EXT,
    ) -> ResultCode;
/// [`vkGetPhysicalDeviceCooperativeMatrixProperties2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixProperties2EXT.html)
///
/// # Optional parameters
/// - properties
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
#[inline(always)]
pub unsafe fn get_physical_device_cooperative_matrix_properties_2_ext(
    physical_device: PhysicalDeviceHandle,
    cooperative_matrix_info: *const PhysicalDeviceCooperativeMatrixInfo2EXT,
    property_count: *mut u32,
    properties: *mut CooperativeMatrixProperties2EXT,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CreateUbmSurfaceSEC = unsafe extern "C" fn(
    InstanceHandle,
    *const UbmSurfaceCreateInfoSEC,
    *const AllocationCallbacks,
    *mut SurfaceKHR,
) -> ResultCode;
/// [`vkCreateUbmSurfaceSEC`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateUbmSurfaceSEC.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_ubm_surface_sec(
    instance: InstanceHandle,
    create_info: *const UbmSurfaceCreateInfoSEC,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetPhysicalDeviceUbmPresentationSupportSEC =
    unsafe extern "C" fn(PhysicalDeviceHandle, u32, *mut ubm_device) -> Bool32;
/// [`vkGetPhysicalDeviceUbmPresentationSupportSEC`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceUbmPresentationSupportSEC.html)
///
#[doc(alias = "vkGetPhysicalDeviceUbmPresentationSupportSEC")]
#[inline(always)]
pub unsafe fn get_physical_device_ubm_presentation_support_sec(
    physical_device: PhysicalDeviceHandle,
    queue_family_index: u32,
    device: *mut ubm_device,
) -> Bool32 {
    todo!()
}

pub(crate) type FUN_CmdSetPrimitiveRestartIndexEXT = unsafe extern "C" fn(CommandBufferHandle, u32);
/// [`vkCmdSetPrimitiveRestartIndexEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartIndexEXT.html)
///
/// # Optional parameters
/// - primitive_restart_index
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdSetPrimitiveRestartIndexEXT")]
#[inline(always)]
pub unsafe fn cmd_set_primitive_restart_index_ext(
    command_buffer: CommandBufferHandle,
    primitive_restart_index: u32,
) {
    todo!()
}

pub(crate) type FUN_CreateAccelerationStructureKHR = unsafe extern "C" fn(
    DeviceHandle,
    *const AccelerationStructureCreateInfoKHR,
    *const AllocationCallbacks,
    *mut AccelerationStructureKHR,
) -> ResultCode;
/// [`vkCreateAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructureKHR.html)
///
/// # Optional parameters
/// - allocator
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
#[inline(always)]
pub unsafe fn create_acceleration_structure_khr(
    device: DeviceHandle,
    create_info: *const AccelerationStructureCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    acceleration_structure: *mut AccelerationStructureKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_DestroyAccelerationStructureKHR =
    unsafe extern "C" fn(DeviceHandle, AccelerationStructureKHR, *const AllocationCallbacks);
/// [`vkDestroyAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureKHR.html)
///
/// # Optional parameters
/// - acceleration_structure
/// - allocator
///
#[doc(alias = "vkDestroyAccelerationStructureKHR")]
#[inline(always)]
pub unsafe fn destroy_acceleration_structure_khr(
    device: DeviceHandle,
    acceleration_structure: AccelerationStructureKHR,
    allocator: *const AllocationCallbacks,
) {
    todo!()
}

pub(crate) type FUN_CmdBuildAccelerationStructuresKHR = unsafe extern "C" fn(
    CommandBufferHandle,
    u32,
    *const AccelerationStructureBuildGeometryInfoKHR,
    *const *const AccelerationStructureBuildRangeInfoKHR,
);
/// [`vkCmdBuildAccelerationStructuresKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructuresKHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdBuildAccelerationStructuresKHR")]
#[inline(always)]
pub unsafe fn cmd_build_acceleration_structures_khr(
    command_buffer: CommandBufferHandle,
    info_count: u32,
    infos: *const AccelerationStructureBuildGeometryInfoKHR,
    build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdBuildAccelerationStructuresIndirectKHR = unsafe extern "C" fn(
    CommandBufferHandle,
    u32,
    *const AccelerationStructureBuildGeometryInfoKHR,
    *const DeviceAddress,
    *const u32,
    *const *const u32,
);
/// [`vkCmdBuildAccelerationStructuresIndirectKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructuresIndirectKHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdBuildAccelerationStructuresIndirectKHR")]
#[inline(always)]
pub unsafe fn cmd_build_acceleration_structures_indirect_khr(
    command_buffer: CommandBufferHandle,
    info_count: u32,
    infos: *const AccelerationStructureBuildGeometryInfoKHR,
    indirect_device_addresses: *const DeviceAddress,
    indirect_strides: *const u32,
    max_primitive_counts: *const *const u32,
) {
    todo!()
}

pub(crate) type FUN_BuildAccelerationStructuresKHR = unsafe extern "C" fn(
    DeviceHandle,
    DeferredOperationKHR,
    u32,
    *const AccelerationStructureBuildGeometryInfoKHR,
    *const *const AccelerationStructureBuildRangeInfoKHR,
) -> ResultCode;
/// [`vkBuildAccelerationStructuresKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBuildAccelerationStructuresKHR.html)
///
/// # Optional parameters
/// - deferred_operation
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
#[inline(always)]
pub unsafe fn build_acceleration_structures_khr(
    device: DeviceHandle,
    deferred_operation: DeferredOperationKHR,
    info_count: u32,
    infos: *const AccelerationStructureBuildGeometryInfoKHR,
    build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CopyAccelerationStructureKHR = unsafe extern "C" fn(
    DeviceHandle,
    DeferredOperationKHR,
    *const CopyAccelerationStructureInfoKHR,
) -> ResultCode;
/// [`vkCopyAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyAccelerationStructureKHR.html)
///
/// # Optional parameters
/// - deferred_operation
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
#[inline(always)]
pub unsafe fn copy_acceleration_structure_khr(
    device: DeviceHandle,
    deferred_operation: DeferredOperationKHR,
    info: *const CopyAccelerationStructureInfoKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CopyAccelerationStructureToMemoryKHR = unsafe extern "C" fn(
    DeviceHandle,
    DeferredOperationKHR,
    *const CopyAccelerationStructureToMemoryInfoKHR,
) -> ResultCode;
/// [`vkCopyAccelerationStructureToMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyAccelerationStructureToMemoryKHR.html)
///
/// # Optional parameters
/// - deferred_operation
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
#[inline(always)]
pub unsafe fn copy_acceleration_structure_to_memory_khr(
    device: DeviceHandle,
    deferred_operation: DeferredOperationKHR,
    info: *const CopyAccelerationStructureToMemoryInfoKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CopyMemoryToAccelerationStructureKHR = unsafe extern "C" fn(
    DeviceHandle,
    DeferredOperationKHR,
    *const CopyMemoryToAccelerationStructureInfoKHR,
) -> ResultCode;
/// [`vkCopyMemoryToAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToAccelerationStructureKHR.html)
///
/// # Optional parameters
/// - deferred_operation
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
#[inline(always)]
pub unsafe fn copy_memory_to_acceleration_structure_khr(
    device: DeviceHandle,
    deferred_operation: DeferredOperationKHR,
    info: *const CopyMemoryToAccelerationStructureInfoKHR,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_WriteAccelerationStructuresPropertiesKHR = unsafe extern "C" fn(
    DeviceHandle,
    u32,
    *const AccelerationStructureKHR,
    QueryType,
    usize,
    *mut c_void,
    usize,
) -> ResultCode;
/// [`vkWriteAccelerationStructuresPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteAccelerationStructuresPropertiesKHR.html)
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
#[inline(always)]
pub unsafe fn write_acceleration_structures_properties_khr(
    device: DeviceHandle,
    acceleration_structure_count: u32,
    acceleration_structures: *const AccelerationStructureKHR,
    query_type: QueryType,
    data_size: usize,
    data: *mut c_void,
    stride: usize,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdCopyAccelerationStructureKHR =
    unsafe extern "C" fn(CommandBufferHandle, *const CopyAccelerationStructureInfoKHR);
/// [`vkCmdCopyAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureKHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyAccelerationStructureKHR")]
#[inline(always)]
pub unsafe fn cmd_copy_acceleration_structure_khr(
    command_buffer: CommandBufferHandle,
    info: *const CopyAccelerationStructureInfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdCopyAccelerationStructureToMemoryKHR =
    unsafe extern "C" fn(CommandBufferHandle, *const CopyAccelerationStructureToMemoryInfoKHR);
/// [`vkCmdCopyAccelerationStructureToMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureToMemoryKHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyAccelerationStructureToMemoryKHR")]
#[inline(always)]
pub unsafe fn cmd_copy_acceleration_structure_to_memory_khr(
    command_buffer: CommandBufferHandle,
    info: *const CopyAccelerationStructureToMemoryInfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdCopyMemoryToAccelerationStructureKHR =
    unsafe extern "C" fn(CommandBufferHandle, *const CopyMemoryToAccelerationStructureInfoKHR);
/// [`vkCmdCopyMemoryToAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToAccelerationStructureKHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdCopyMemoryToAccelerationStructureKHR")]
#[inline(always)]
pub unsafe fn cmd_copy_memory_to_acceleration_structure_khr(
    command_buffer: CommandBufferHandle,
    info: *const CopyMemoryToAccelerationStructureInfoKHR,
) {
    todo!()
}

pub(crate) type FUN_GetAccelerationStructureDeviceAddressKHR =
    unsafe extern "C" fn(
        DeviceHandle,
        *const AccelerationStructureDeviceAddressInfoKHR,
    ) -> DeviceAddress;
/// [`vkGetAccelerationStructureDeviceAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureDeviceAddressKHR.html)
///
#[doc(alias = "vkGetAccelerationStructureDeviceAddressKHR")]
#[inline(always)]
pub unsafe fn get_acceleration_structure_device_address_khr(
    device: DeviceHandle,
    info: *const AccelerationStructureDeviceAddressInfoKHR,
) -> DeviceAddress {
    todo!()
}

pub(crate) type FUN_CmdWriteAccelerationStructuresPropertiesKHR = unsafe extern "C" fn(
    CommandBufferHandle,
    u32,
    *const AccelerationStructureKHR,
    QueryType,
    QueryPool,
    u32,
);
/// [`vkCmdWriteAccelerationStructuresPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteAccelerationStructuresPropertiesKHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesKHR")]
#[inline(always)]
pub unsafe fn cmd_write_acceleration_structures_properties_khr(
    command_buffer: CommandBufferHandle,
    acceleration_structure_count: u32,
    acceleration_structures: *const AccelerationStructureKHR,
    query_type: QueryType,
    query_pool: QueryPool,
    first_query: u32,
) {
    todo!()
}

pub(crate) type FUN_GetDeviceAccelerationStructureCompatibilityKHR = unsafe extern "C" fn(
    DeviceHandle,
    *const AccelerationStructureVersionInfoKHR,
    *mut AccelerationStructureCompatibilityKHR,
);
/// [`vkGetDeviceAccelerationStructureCompatibilityKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceAccelerationStructureCompatibilityKHR.html)
///
#[doc(alias = "vkGetDeviceAccelerationStructureCompatibilityKHR")]
#[inline(always)]
pub unsafe fn get_device_acceleration_structure_compatibility_khr(
    device: DeviceHandle,
    version_info: *const AccelerationStructureVersionInfoKHR,
    compatibility: *mut AccelerationStructureCompatibilityKHR,
) {
    todo!()
}

pub(crate) type FUN_GetAccelerationStructureBuildSizesKHR = unsafe extern "C" fn(
    DeviceHandle,
    AccelerationStructureBuildTypeKHR,
    *const AccelerationStructureBuildGeometryInfoKHR,
    *const u32,
    *mut AccelerationStructureBuildSizesInfoKHR,
);
/// [`vkGetAccelerationStructureBuildSizesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureBuildSizesKHR.html)
///
/// # Optional parameters
/// - max_primitive_counts
///
#[doc(alias = "vkGetAccelerationStructureBuildSizesKHR")]
#[inline(always)]
pub unsafe fn get_acceleration_structure_build_sizes_khr(
    device: DeviceHandle,
    build_type: AccelerationStructureBuildTypeKHR,
    build_info: *const AccelerationStructureBuildGeometryInfoKHR,
    max_primitive_counts: *const u32,
    size_info: *mut AccelerationStructureBuildSizesInfoKHR,
) {
    todo!()
}

pub(crate) type FUN_CmdTraceRaysKHR = unsafe extern "C" fn(
    CommandBufferHandle,
    *const StridedDeviceAddressRegionKHR,
    *const StridedDeviceAddressRegionKHR,
    *const StridedDeviceAddressRegionKHR,
    *const StridedDeviceAddressRegionKHR,
    u32,
    u32,
    u32,
);
/// [`vkCmdTraceRaysKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysKHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdTraceRaysKHR")]
#[inline(always)]
pub unsafe fn cmd_trace_rays_khr(
    command_buffer: CommandBufferHandle,
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

pub(crate) type FUN_CreateRayTracingPipelinesKHR = unsafe extern "C" fn(
    DeviceHandle,
    DeferredOperationKHR,
    PipelineCache,
    u32,
    *const RayTracingPipelineCreateInfoKHR,
    *const AllocationCallbacks,
    *mut Pipeline,
) -> ResultCode;
/// [`vkCreateRayTracingPipelinesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRayTracingPipelinesKHR.html)
///
/// # Optional parameters
/// - deferred_operation
/// - pipeline_cache
/// - allocator
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
#[inline(always)]
pub unsafe fn create_ray_tracing_pipelines_khr(
    device: DeviceHandle,
    deferred_operation: DeferredOperationKHR,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    create_infos: *const RayTracingPipelineCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    pipelines: *mut Pipeline,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_GetRayTracingCaptureReplayShaderGroupHandlesKHR =
    unsafe extern "C" fn(DeviceHandle, Pipeline, u32, u32, usize, *mut c_void) -> ResultCode;
/// [`vkGetRayTracingCaptureReplayShaderGroupHandlesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html)
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
#[inline(always)]
pub unsafe fn get_ray_tracing_capture_replay_shader_group_handles_khr(
    device: DeviceHandle,
    pipeline: Pipeline,
    first_group: u32,
    group_count: u32,
    data_size: usize,
    data: *mut c_void,
) -> ResultCode {
    todo!()
}

pub(crate) type FUN_CmdTraceRaysIndirectKHR = unsafe extern "C" fn(
    CommandBufferHandle,
    *const StridedDeviceAddressRegionKHR,
    *const StridedDeviceAddressRegionKHR,
    *const StridedDeviceAddressRegionKHR,
    *const StridedDeviceAddressRegionKHR,
    DeviceAddress,
);
/// [`vkCmdTraceRaysIndirectKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysIndirectKHR.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdTraceRaysIndirectKHR")]
#[inline(always)]
pub unsafe fn cmd_trace_rays_indirect_khr(
    command_buffer: CommandBufferHandle,
    raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    indirect_device_address: DeviceAddress,
) {
    todo!()
}

pub(crate) type FUN_GetRayTracingShaderGroupStackSizeKHR =
    unsafe extern "C" fn(DeviceHandle, Pipeline, u32, ShaderGroupShaderKHR) -> DeviceSize;
/// [`vkGetRayTracingShaderGroupStackSizeKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupStackSizeKHR.html)
///
#[doc(alias = "vkGetRayTracingShaderGroupStackSizeKHR")]
#[inline(always)]
pub unsafe fn get_ray_tracing_shader_group_stack_size_khr(
    device: DeviceHandle,
    pipeline: Pipeline,
    group: u32,
    group_shader: ShaderGroupShaderKHR,
) -> DeviceSize {
    todo!()
}

pub(crate) type FUN_CmdSetRayTracingPipelineStackSizeKHR =
    unsafe extern "C" fn(CommandBufferHandle, u32);
/// [`vkCmdSetRayTracingPipelineStackSizeKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRayTracingPipelineStackSizeKHR.html)
///
/// # Performed tasks
/// - `state`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`COMPUTE`](QueueFlags::COMPUTE)
///
#[doc(alias = "vkCmdSetRayTracingPipelineStackSizeKHR")]
#[inline(always)]
pub unsafe fn cmd_set_ray_tracing_pipeline_stack_size_khr(
    command_buffer: CommandBufferHandle,
    pipeline_stack_size: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdDrawMeshTasksEXT = unsafe extern "C" fn(CommandBufferHandle, u32, u32, u32);
/// [`vkCmdDrawMeshTasksEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksEXT.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawMeshTasksEXT")]
#[inline(always)]
pub unsafe fn cmd_draw_mesh_tasks_ext(
    command_buffer: CommandBufferHandle,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdDrawMeshTasksIndirectEXT =
    unsafe extern "C" fn(CommandBufferHandle, Buffer, DeviceSize, u32, u32);
/// [`vkCmdDrawMeshTasksIndirectEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectEXT.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawMeshTasksIndirectEXT")]
#[inline(always)]
pub unsafe fn cmd_draw_mesh_tasks_indirect_ext(
    command_buffer: CommandBufferHandle,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
) {
    todo!()
}

pub(crate) type FUN_CmdDrawMeshTasksIndirectCountEXT =
    unsafe extern "C" fn(CommandBufferHandle, Buffer, DeviceSize, Buffer, DeviceSize, u32, u32);
/// [`vkCmdDrawMeshTasksIndirectCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCountEXT.html)
///
/// # Performed tasks
/// - `action`
///
/// # Allowed command buffers
/// - Primary
/// - Secondary
///
/// # Allowed queues
/// - [`GRAPHICS`](QueueFlags::GRAPHICS)
///
#[doc(alias = "vkCmdDrawMeshTasksIndirectCountEXT")]
#[inline(always)]
pub unsafe fn cmd_draw_mesh_tasks_indirect_count_ext(
    command_buffer: CommandBufferHandle,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
) {
    todo!()
}
