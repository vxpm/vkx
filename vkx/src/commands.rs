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
/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateInstance.html>
#[doc(alias = "vkCreateInstance")]
pub unsafe extern "C" fn create_instance(
    p_create_info: *const InstanceCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_instance: *mut Instance,
) -> ResultCode {
    todo!()
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyInstance.html>
    #[doc(alias = "vkDestroyInstance")]
    pub unsafe extern "C" fn destroy_instance(self, p_allocator: *const AllocationCallbacks) {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDevices.html>
    #[doc(alias = "vkEnumeratePhysicalDevices")]
    pub unsafe extern "C" fn enumerate_physical_devices(
        self,
        p_physical_device_count: *mut u32,
        p_physical_devices: *mut PhysicalDevice,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures.html>
    #[doc(alias = "vkGetPhysicalDeviceFeatures")]
    pub unsafe extern "C" fn get_physical_device_features(
        self,
        p_features: *mut PhysicalDeviceFeatures,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties.html>
    #[doc(alias = "vkGetPhysicalDeviceFormatProperties")]
    pub unsafe extern "C" fn get_physical_device_format_properties(
        self,
        format: Format,
        p_format_properties: *mut FormatProperties,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties.html>
    #[doc(alias = "vkGetPhysicalDeviceImageFormatProperties")]
    pub unsafe extern "C" fn get_physical_device_image_format_properties(
        self,
        format: Format,
        type_: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        p_image_format_properties: *mut ImageFormatProperties,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties.html>
    #[doc(alias = "vkGetPhysicalDeviceProperties")]
    pub unsafe extern "C" fn get_physical_device_properties(
        self,
        p_properties: *mut PhysicalDeviceProperties,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties.html>
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties")]
    pub unsafe extern "C" fn get_physical_device_queue_family_properties(
        self,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut QueueFamilyProperties,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties.html>
    #[doc(alias = "vkGetPhysicalDeviceMemoryProperties")]
    pub unsafe extern "C" fn get_physical_device_memory_properties(
        self,
        p_memory_properties: *mut PhysicalDeviceMemoryProperties,
    ) {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetInstanceProcAddr.html>
    #[doc(alias = "vkGetInstanceProcAddr")]
    pub unsafe extern "C" fn get_instance_proc_addr(self, p_name: *const c_char) -> vkVoidFunction {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceProcAddr.html>
    #[doc(alias = "vkGetDeviceProcAddr")]
    pub unsafe extern "C" fn get_device_proc_addr(self, p_name: *const c_char) -> vkVoidFunction {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDevice.html>
    #[doc(alias = "vkCreateDevice")]
    pub unsafe extern "C" fn create_device(
        self,
        p_create_info: *const DeviceCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_device: *mut Device,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDevice.html>
    #[doc(alias = "vkDestroyDevice")]
    pub unsafe extern "C" fn destroy_device(self, p_allocator: *const AllocationCallbacks) {
        todo!()
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceExtensionProperties.html>
#[doc(alias = "vkEnumerateInstanceExtensionProperties")]
pub unsafe extern "C" fn enumerate_instance_extension_properties(
    p_layer_name: *const c_char,
    p_property_count: *mut u32,
    p_properties: *mut ExtensionProperties,
) -> ResultCode {
    todo!()
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceExtensionProperties.html>
    #[doc(alias = "vkEnumerateDeviceExtensionProperties")]
    pub unsafe extern "C" fn enumerate_device_extension_properties(
        self,
        p_layer_name: *const c_char,
        p_property_count: *mut u32,
        p_properties: *mut ExtensionProperties,
    ) -> ResultCode {
        todo!()
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceLayerProperties.html>
#[doc(alias = "vkEnumerateInstanceLayerProperties")]
pub unsafe extern "C" fn enumerate_instance_layer_properties(
    p_property_count: *mut u32,
    p_properties: *mut LayerProperties,
) -> ResultCode {
    todo!()
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceLayerProperties.html>
    #[doc(alias = "vkEnumerateDeviceLayerProperties")]
    pub unsafe extern "C" fn enumerate_device_layer_properties(
        self,
        p_property_count: *mut u32,
        p_properties: *mut LayerProperties,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue.html>
    #[doc(alias = "vkGetDeviceQueue")]
    pub unsafe extern "C" fn get_device_queue(
        self,
        queue_family_index: u32,
        queue_index: u32,
        p_queue: *mut Queue,
    ) {
        todo!()
    }
}

impl Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit.html>
    #[doc(alias = "vkQueueSubmit")]
    pub unsafe extern "C" fn queue_submit(
        self,
        submit_count: u32,
        p_submits: *const SubmitInfo,
        fence: Fence,
    ) -> ResultCode {
        todo!()
    }
}

impl Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueWaitIdle.html>
    #[doc(alias = "vkQueueWaitIdle")]
    pub unsafe extern "C" fn queue_wait_idle(self) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDeviceWaitIdle.html>
    #[doc(alias = "vkDeviceWaitIdle")]
    pub unsafe extern "C" fn device_wait_idle(self) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateMemory.html>
    #[doc(alias = "vkAllocateMemory")]
    pub unsafe extern "C" fn allocate_memory(
        self,
        p_allocate_info: *const MemoryAllocateInfo,
        p_allocator: *const AllocationCallbacks,
        p_memory: *mut DeviceMemory,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeMemory.html>
    #[doc(alias = "vkFreeMemory")]
    pub unsafe extern "C" fn free_memory(
        self,
        memory: DeviceMemory,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory.html>
    #[doc(alias = "vkMapMemory")]
    pub unsafe extern "C" fn map_memory(
        self,
        memory: DeviceMemory,
        offset: DeviceSize,
        size: DeviceSize,
        flags: MemoryMapFlags,
        pp_data: *mut *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory.html>
    #[doc(alias = "vkUnmapMemory")]
    pub unsafe extern "C" fn unmap_memory(self, memory: DeviceMemory) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkFlushMappedMemoryRanges.html>
    #[doc(alias = "vkFlushMappedMemoryRanges")]
    pub unsafe extern "C" fn flush_mapped_memory_ranges(
        self,
        memory_range_count: u32,
        p_memory_ranges: *const MappedMemoryRange,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkInvalidateMappedMemoryRanges.html>
    #[doc(alias = "vkInvalidateMappedMemoryRanges")]
    pub unsafe extern "C" fn invalidate_mapped_memory_ranges(
        self,
        memory_range_count: u32,
        p_memory_ranges: *const MappedMemoryRange,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryCommitment.html>
    #[doc(alias = "vkGetDeviceMemoryCommitment")]
    pub unsafe extern "C" fn get_device_memory_commitment(
        self,
        memory: DeviceMemory,
        p_committed_memory_in_bytes: *mut DeviceSize,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory.html>
    #[doc(alias = "vkBindBufferMemory")]
    pub unsafe extern "C" fn bind_buffer_memory(
        self,
        buffer: Buffer,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory.html>
    #[doc(alias = "vkBindImageMemory")]
    pub unsafe extern "C" fn bind_image_memory(
        self,
        image: Image,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements.html>
    #[doc(alias = "vkGetBufferMemoryRequirements")]
    pub unsafe extern "C" fn get_buffer_memory_requirements(
        self,
        buffer: Buffer,
        p_memory_requirements: *mut MemoryRequirements,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements.html>
    #[doc(alias = "vkGetImageMemoryRequirements")]
    pub unsafe extern "C" fn get_image_memory_requirements(
        self,
        image: Image,
        p_memory_requirements: *mut MemoryRequirements,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements.html>
    #[doc(alias = "vkGetImageSparseMemoryRequirements")]
    pub unsafe extern "C" fn get_image_sparse_memory_requirements(
        self,
        image: Image,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties.html>
    #[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties")]
    pub unsafe extern "C" fn get_physical_device_sparse_image_format_properties(
        self,
        format: Format,
        type_: ImageType,
        samples: SampleCountFlags,
        usage: ImageUsageFlags,
        tiling: ImageTiling,
        p_property_count: *mut u32,
        p_properties: *mut SparseImageFormatProperties,
    ) {
        todo!()
    }
}

impl Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBindSparse.html>
    #[doc(alias = "vkQueueBindSparse")]
    pub unsafe extern "C" fn queue_bind_sparse(
        self,
        bind_info_count: u32,
        p_bind_info: *const BindSparseInfo,
        fence: Fence,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateFence.html>
    #[doc(alias = "vkCreateFence")]
    pub unsafe extern "C" fn create_fence(
        self,
        p_create_info: *const FenceCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_fence: *mut Fence,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFence.html>
    #[doc(alias = "vkDestroyFence")]
    pub unsafe extern "C" fn destroy_fence(
        self,
        fence: Fence,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkResetFences.html>
    #[doc(alias = "vkResetFences")]
    pub unsafe extern "C" fn reset_fences(
        self,
        fence_count: u32,
        p_fences: *const Fence,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceStatus.html>
    #[doc(alias = "vkGetFenceStatus")]
    pub unsafe extern "C" fn get_fence_status(self, fence: Fence) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForFences.html>
    #[doc(alias = "vkWaitForFences")]
    pub unsafe extern "C" fn wait_for_fences(
        self,
        fence_count: u32,
        p_fences: *const Fence,
        wait_all: Bool32,
        timeout: u64,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSemaphore.html>
    #[doc(alias = "vkCreateSemaphore")]
    pub unsafe extern "C" fn create_semaphore(
        self,
        p_create_info: *const SemaphoreCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_semaphore: *mut Semaphore,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySemaphore.html>
    #[doc(alias = "vkDestroySemaphore")]
    pub unsafe extern "C" fn destroy_semaphore(
        self,
        semaphore: Semaphore,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateQueryPool.html>
    #[doc(alias = "vkCreateQueryPool")]
    pub unsafe extern "C" fn create_query_pool(
        self,
        p_create_info: *const QueryPoolCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_query_pool: *mut QueryPool,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyQueryPool.html>
    #[doc(alias = "vkDestroyQueryPool")]
    pub unsafe extern "C" fn destroy_query_pool(
        self,
        query_pool: QueryPool,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueryPoolResults.html>
    #[doc(alias = "vkGetQueryPoolResults")]
    pub unsafe extern "C" fn get_query_pool_results(
        self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        data_size: usize,
        p_data: *mut c_void,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBuffer.html>
    #[doc(alias = "vkCreateBuffer")]
    pub unsafe extern "C" fn create_buffer(
        self,
        p_create_info: *const BufferCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_buffer: *mut Buffer,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBuffer.html>
    #[doc(alias = "vkDestroyBuffer")]
    pub unsafe extern "C" fn destroy_buffer(
        self,
        buffer: Buffer,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImage.html>
    #[doc(alias = "vkCreateImage")]
    pub unsafe extern "C" fn create_image(
        self,
        p_create_info: *const ImageCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_image: *mut Image,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImage.html>
    #[doc(alias = "vkDestroyImage")]
    pub unsafe extern "C" fn destroy_image(
        self,
        image: Image,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout.html>
    #[doc(alias = "vkGetImageSubresourceLayout")]
    pub unsafe extern "C" fn get_image_subresource_layout(
        self,
        image: Image,
        p_subresource: *const ImageSubresource,
        p_layout: *mut SubresourceLayout,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImageView.html>
    #[doc(alias = "vkCreateImageView")]
    pub unsafe extern "C" fn create_image_view(
        self,
        p_create_info: *const ImageViewCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_view: *mut ImageView,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImageView.html>
    #[doc(alias = "vkDestroyImageView")]
    pub unsafe extern "C" fn destroy_image_view(
        self,
        image_view: ImageView,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCommandPool.html>
    #[doc(alias = "vkCreateCommandPool")]
    pub unsafe extern "C" fn create_command_pool(
        self,
        p_create_info: *const CommandPoolCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_command_pool: *mut CommandPool,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCommandPool.html>
    #[doc(alias = "vkDestroyCommandPool")]
    pub unsafe extern "C" fn destroy_command_pool(
        self,
        command_pool: CommandPool,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandPool.html>
    #[doc(alias = "vkResetCommandPool")]
    pub unsafe extern "C" fn reset_command_pool(
        self,
        command_pool: CommandPool,
        flags: CommandPoolResetFlags,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateCommandBuffers.html>
    #[doc(alias = "vkAllocateCommandBuffers")]
    pub unsafe extern "C" fn allocate_command_buffers(
        self,
        p_allocate_info: *const CommandBufferAllocateInfo,
        p_command_buffers: *mut CommandBuffer,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeCommandBuffers.html>
    #[doc(alias = "vkFreeCommandBuffers")]
    pub unsafe extern "C" fn free_command_buffers(
        self,
        command_pool: CommandPool,
        command_buffer_count: u32,
        p_command_buffers: *const CommandBuffer,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBeginCommandBuffer.html>
    #[doc(alias = "vkBeginCommandBuffer")]
    pub unsafe extern "C" fn begin_command_buffer(
        self,
        p_begin_info: *const CommandBufferBeginInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEndCommandBuffer.html>
    #[doc(alias = "vkEndCommandBuffer")]
    pub unsafe extern "C" fn end_command_buffer(self) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandBuffer.html>
    #[doc(alias = "vkResetCommandBuffer")]
    pub unsafe extern "C" fn reset_command_buffer(
        self,
        flags: CommandBufferResetFlags,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer.html>
    #[doc(alias = "vkCmdCopyBuffer")]
    pub unsafe extern "C" fn cmd_copy_buffer(
        self,
        src_buffer: Buffer,
        dst_buffer: Buffer,
        region_count: u32,
        p_regions: *const BufferCopy,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage.html>
    #[doc(alias = "vkCmdCopyImage")]
    pub unsafe extern "C" fn cmd_copy_image(
        self,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const ImageCopy,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage.html>
    #[doc(alias = "vkCmdCopyBufferToImage")]
    pub unsafe extern "C" fn cmd_copy_buffer_to_image(
        self,
        src_buffer: Buffer,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const BufferImageCopy,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer.html>
    #[doc(alias = "vkCmdCopyImageToBuffer")]
    pub unsafe extern "C" fn cmd_copy_image_to_buffer(
        self,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_buffer: Buffer,
        region_count: u32,
        p_regions: *const BufferImageCopy,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdateBuffer.html>
    #[doc(alias = "vkCmdUpdateBuffer")]
    pub unsafe extern "C" fn cmd_update_buffer(
        self,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        data_size: DeviceSize,
        p_data: *const c_void,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdFillBuffer.html>
    #[doc(alias = "vkCmdFillBuffer")]
    pub unsafe extern "C" fn cmd_fill_buffer(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier.html>
    #[doc(alias = "vkCmdPipelineBarrier")]
    pub unsafe extern "C" fn cmd_pipeline_barrier(
        self,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        dependency_flags: DependencyFlags,
        memory_barrier_count: u32,
        p_memory_barriers: *const MemoryBarrier,
        buffer_memory_barrier_count: u32,
        p_buffer_memory_barriers: *const BufferMemoryBarrier,
        image_memory_barrier_count: u32,
        p_image_memory_barriers: *const ImageMemoryBarrier,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginQuery.html>
    #[doc(alias = "vkCmdBeginQuery")]
    pub unsafe extern "C" fn cmd_begin_query(
        self,
        query_pool: QueryPool,
        query: u32,
        flags: QueryControlFlags,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndQuery.html>
    #[doc(alias = "vkCmdEndQuery")]
    pub unsafe extern "C" fn cmd_end_query(self, query_pool: QueryPool, query: u32) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetQueryPool.html>
    #[doc(alias = "vkCmdResetQueryPool")]
    pub unsafe extern "C" fn cmd_reset_query_pool(
        self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp.html>
    #[doc(alias = "vkCmdWriteTimestamp")]
    pub unsafe extern "C" fn cmd_write_timestamp(
        self,
        pipeline_stage: PipelineStageFlags,
        query_pool: QueryPool,
        query: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyQueryPoolResults.html>
    #[doc(alias = "vkCmdCopyQueryPoolResults")]
    pub unsafe extern "C" fn cmd_copy_query_pool_results(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteCommands.html>
    #[doc(alias = "vkCmdExecuteCommands")]
    pub unsafe extern "C" fn cmd_execute_commands(
        self,
        command_buffer_count: u32,
        p_command_buffers: *const CommandBuffer,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateEvent.html>
    #[doc(alias = "vkCreateEvent")]
    pub unsafe extern "C" fn create_event(
        self,
        p_create_info: *const EventCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_event: *mut Event,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyEvent.html>
    #[doc(alias = "vkDestroyEvent")]
    pub unsafe extern "C" fn destroy_event(
        self,
        event: Event,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetEventStatus.html>
    #[doc(alias = "vkGetEventStatus")]
    pub unsafe extern "C" fn get_event_status(self, event: Event) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetEvent.html>
    #[doc(alias = "vkSetEvent")]
    pub unsafe extern "C" fn set_event(self, event: Event) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkResetEvent.html>
    #[doc(alias = "vkResetEvent")]
    pub unsafe extern "C" fn reset_event(self, event: Event) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBufferView.html>
    #[doc(alias = "vkCreateBufferView")]
    pub unsafe extern "C" fn create_buffer_view(
        self,
        p_create_info: *const BufferViewCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_view: *mut BufferView,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferView.html>
    #[doc(alias = "vkDestroyBufferView")]
    pub unsafe extern "C" fn destroy_buffer_view(
        self,
        buffer_view: BufferView,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShaderModule.html>
    #[doc(alias = "vkCreateShaderModule")]
    pub unsafe extern "C" fn create_shader_module(
        self,
        p_create_info: *const ShaderModuleCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_shader_module: *mut ShaderModule,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderModule.html>
    #[doc(alias = "vkDestroyShaderModule")]
    pub unsafe extern "C" fn destroy_shader_module(
        self,
        shader_module: ShaderModule,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineCache.html>
    #[doc(alias = "vkCreatePipelineCache")]
    pub unsafe extern "C" fn create_pipeline_cache(
        self,
        p_create_info: *const PipelineCacheCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipeline_cache: *mut PipelineCache,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineCache.html>
    #[doc(alias = "vkDestroyPipelineCache")]
    pub unsafe extern "C" fn destroy_pipeline_cache(
        self,
        pipeline_cache: PipelineCache,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineCacheData.html>
    #[doc(alias = "vkGetPipelineCacheData")]
    pub unsafe extern "C" fn get_pipeline_cache_data(
        self,
        pipeline_cache: PipelineCache,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkMergePipelineCaches.html>
    #[doc(alias = "vkMergePipelineCaches")]
    pub unsafe extern "C" fn merge_pipeline_caches(
        self,
        dst_cache: PipelineCache,
        src_cache_count: u32,
        p_src_caches: *const PipelineCache,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateComputePipelines.html>
    #[doc(alias = "vkCreateComputePipelines")]
    pub unsafe extern "C" fn create_compute_pipelines(
        self,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const ComputePipelineCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipelines: *mut Pipeline,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipeline.html>
    #[doc(alias = "vkDestroyPipeline")]
    pub unsafe extern "C" fn destroy_pipeline(
        self,
        pipeline: Pipeline,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineLayout.html>
    #[doc(alias = "vkCreatePipelineLayout")]
    pub unsafe extern "C" fn create_pipeline_layout(
        self,
        p_create_info: *const PipelineLayoutCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipeline_layout: *mut PipelineLayout,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineLayout.html>
    #[doc(alias = "vkDestroyPipelineLayout")]
    pub unsafe extern "C" fn destroy_pipeline_layout(
        self,
        pipeline_layout: PipelineLayout,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSampler.html>
    #[doc(alias = "vkCreateSampler")]
    pub unsafe extern "C" fn create_sampler(
        self,
        p_create_info: *const SamplerCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_sampler: *mut Sampler,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySampler.html>
    #[doc(alias = "vkDestroySampler")]
    pub unsafe extern "C" fn destroy_sampler(
        self,
        sampler: Sampler,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorSetLayout.html>
    #[doc(alias = "vkCreateDescriptorSetLayout")]
    pub unsafe extern "C" fn create_descriptor_set_layout(
        self,
        p_create_info: *const DescriptorSetLayoutCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_set_layout: *mut DescriptorSetLayout,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorSetLayout.html>
    #[doc(alias = "vkDestroyDescriptorSetLayout")]
    pub unsafe extern "C" fn destroy_descriptor_set_layout(
        self,
        descriptor_set_layout: DescriptorSetLayout,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorPool.html>
    #[doc(alias = "vkCreateDescriptorPool")]
    pub unsafe extern "C" fn create_descriptor_pool(
        self,
        p_create_info: *const DescriptorPoolCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_descriptor_pool: *mut DescriptorPool,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorPool.html>
    #[doc(alias = "vkDestroyDescriptorPool")]
    pub unsafe extern "C" fn destroy_descriptor_pool(
        self,
        descriptor_pool: DescriptorPool,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkResetDescriptorPool.html>
    #[doc(alias = "vkResetDescriptorPool")]
    pub unsafe extern "C" fn reset_descriptor_pool(
        self,
        descriptor_pool: DescriptorPool,
        flags: DescriptorPoolResetFlags,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateDescriptorSets.html>
    #[doc(alias = "vkAllocateDescriptorSets")]
    pub unsafe extern "C" fn allocate_descriptor_sets(
        self,
        p_allocate_info: *const DescriptorSetAllocateInfo,
        p_descriptor_sets: *mut DescriptorSet,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeDescriptorSets.html>
    #[doc(alias = "vkFreeDescriptorSets")]
    pub unsafe extern "C" fn free_descriptor_sets(
        self,
        descriptor_pool: DescriptorPool,
        descriptor_set_count: u32,
        p_descriptor_sets: *const DescriptorSet,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSets.html>
    #[doc(alias = "vkUpdateDescriptorSets")]
    pub unsafe extern "C" fn update_descriptor_sets(
        self,
        descriptor_write_count: u32,
        p_descriptor_writes: *const WriteDescriptorSet,
        descriptor_copy_count: u32,
        p_descriptor_copies: *const CopyDescriptorSet,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindPipeline.html>
    #[doc(alias = "vkCmdBindPipeline")]
    pub unsafe extern "C" fn cmd_bind_pipeline(
        self,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets.html>
    #[doc(alias = "vkCmdBindDescriptorSets")]
    pub unsafe extern "C" fn cmd_bind_descriptor_sets(
        self,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        descriptor_set_count: u32,
        p_descriptor_sets: *const DescriptorSet,
        dynamic_offset_count: u32,
        p_dynamic_offsets: *const u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearColorImage.html>
    #[doc(alias = "vkCmdClearColorImage")]
    pub unsafe extern "C" fn cmd_clear_color_image(
        self,
        image: Image,
        image_layout: ImageLayout,
        p_color: *const ClearColorValue,
        range_count: u32,
        p_ranges: *const ImageSubresourceRange,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatch.html>
    #[doc(alias = "vkCmdDispatch")]
    pub unsafe extern "C" fn cmd_dispatch(
        self,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchIndirect.html>
    #[doc(alias = "vkCmdDispatchIndirect")]
    pub unsafe extern "C" fn cmd_dispatch_indirect(self, buffer: Buffer, offset: DeviceSize) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent.html>
    #[doc(alias = "vkCmdSetEvent")]
    pub unsafe extern "C" fn cmd_set_event(self, event: Event, stage_mask: PipelineStageFlags) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent.html>
    #[doc(alias = "vkCmdResetEvent")]
    pub unsafe extern "C" fn cmd_reset_event(self, event: Event, stage_mask: PipelineStageFlags) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents.html>
    #[doc(alias = "vkCmdWaitEvents")]
    pub unsafe extern "C" fn cmd_wait_events(
        self,
        event_count: u32,
        p_events: *const Event,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        memory_barrier_count: u32,
        p_memory_barriers: *const MemoryBarrier,
        buffer_memory_barrier_count: u32,
        p_buffer_memory_barriers: *const BufferMemoryBarrier,
        image_memory_barrier_count: u32,
        p_image_memory_barriers: *const ImageMemoryBarrier,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants.html>
    #[doc(alias = "vkCmdPushConstants")]
    pub unsafe extern "C" fn cmd_push_constants(
        self,
        layout: PipelineLayout,
        stage_flags: ShaderStageFlags,
        offset: u32,
        size: u32,
        p_values: *const c_void,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateGraphicsPipelines.html>
    #[doc(alias = "vkCreateGraphicsPipelines")]
    pub unsafe extern "C" fn create_graphics_pipelines(
        self,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const GraphicsPipelineCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipelines: *mut Pipeline,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateFramebuffer.html>
    #[doc(alias = "vkCreateFramebuffer")]
    pub unsafe extern "C" fn create_framebuffer(
        self,
        p_create_info: *const FramebufferCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_framebuffer: *mut Framebuffer,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFramebuffer.html>
    #[doc(alias = "vkDestroyFramebuffer")]
    pub unsafe extern "C" fn destroy_framebuffer(
        self,
        framebuffer: Framebuffer,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass.html>
    #[doc(alias = "vkCreateRenderPass")]
    pub unsafe extern "C" fn create_render_pass(
        self,
        p_create_info: *const RenderPassCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_render_pass: *mut RenderPass,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyRenderPass.html>
    #[doc(alias = "vkDestroyRenderPass")]
    pub unsafe extern "C" fn destroy_render_pass(
        self,
        render_pass: RenderPass,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderAreaGranularity.html>
    #[doc(alias = "vkGetRenderAreaGranularity")]
    pub unsafe extern "C" fn get_render_area_granularity(
        self,
        render_pass: RenderPass,
        p_granularity: *mut Extent2D,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewport.html>
    #[doc(alias = "vkCmdSetViewport")]
    pub unsafe extern "C" fn cmd_set_viewport(
        self,
        first_viewport: u32,
        viewport_count: u32,
        p_viewports: *const Viewport,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissor.html>
    #[doc(alias = "vkCmdSetScissor")]
    pub unsafe extern "C" fn cmd_set_scissor(
        self,
        first_scissor: u32,
        scissor_count: u32,
        p_scissors: *const Rect2D,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineWidth.html>
    #[doc(alias = "vkCmdSetLineWidth")]
    pub unsafe extern "C" fn cmd_set_line_width(self, line_width: f32) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias.html>
    #[doc(alias = "vkCmdSetDepthBias")]
    pub unsafe extern "C" fn cmd_set_depth_bias(
        self,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetBlendConstants.html>
    #[doc(alias = "vkCmdSetBlendConstants")]
    pub unsafe extern "C" fn cmd_set_blend_constants(
        self,
        blend_constants: *const [f32; 4 as usize],
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBounds.html>
    #[doc(alias = "vkCmdSetDepthBounds")]
    pub unsafe extern "C" fn cmd_set_depth_bounds(
        self,
        min_depth_bounds: f32,
        max_depth_bounds: f32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilCompareMask.html>
    #[doc(alias = "vkCmdSetStencilCompareMask")]
    pub unsafe extern "C" fn cmd_set_stencil_compare_mask(
        self,
        face_mask: StencilFaceFlags,
        compare_mask: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilWriteMask.html>
    #[doc(alias = "vkCmdSetStencilWriteMask")]
    pub unsafe extern "C" fn cmd_set_stencil_write_mask(
        self,
        face_mask: StencilFaceFlags,
        write_mask: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilReference.html>
    #[doc(alias = "vkCmdSetStencilReference")]
    pub unsafe extern "C" fn cmd_set_stencil_reference(
        self,
        face_mask: StencilFaceFlags,
        reference: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer.html>
    #[doc(alias = "vkCmdBindIndexBuffer")]
    pub unsafe extern "C" fn cmd_bind_index_buffer(
        self,
        buffer: Buffer,
        offset: DeviceSize,
        index_type: IndexType,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers.html>
    #[doc(alias = "vkCmdBindVertexBuffers")]
    pub unsafe extern "C" fn cmd_bind_vertex_buffers(
        self,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const Buffer,
        p_offsets: *const DeviceSize,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDraw.html>
    #[doc(alias = "vkCmdDraw")]
    pub unsafe extern "C" fn cmd_draw(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexed.html>
    #[doc(alias = "vkCmdDrawIndexed")]
    pub unsafe extern "C" fn cmd_draw_indexed(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirect.html>
    #[doc(alias = "vkCmdDrawIndirect")]
    pub unsafe extern "C" fn cmd_draw_indirect(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirect.html>
    #[doc(alias = "vkCmdDrawIndexedIndirect")]
    pub unsafe extern "C" fn cmd_draw_indexed_indirect(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage.html>
    #[doc(alias = "vkCmdBlitImage")]
    pub unsafe extern "C" fn cmd_blit_image(
        self,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const ImageBlit,
        filter: Filter,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearDepthStencilImage.html>
    #[doc(alias = "vkCmdClearDepthStencilImage")]
    pub unsafe extern "C" fn cmd_clear_depth_stencil_image(
        self,
        image: Image,
        image_layout: ImageLayout,
        p_depth_stencil: *const ClearDepthStencilValue,
        range_count: u32,
        p_ranges: *const ImageSubresourceRange,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearAttachments.html>
    #[doc(alias = "vkCmdClearAttachments")]
    pub unsafe extern "C" fn cmd_clear_attachments(
        self,
        attachment_count: u32,
        p_attachments: *const ClearAttachment,
        rect_count: u32,
        p_rects: *const ClearRect,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage.html>
    #[doc(alias = "vkCmdResolveImage")]
    pub unsafe extern "C" fn cmd_resolve_image(
        self,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const ImageResolve,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass.html>
    #[doc(alias = "vkCmdBeginRenderPass")]
    pub unsafe extern "C" fn cmd_begin_render_pass(
        self,
        p_render_pass_begin: *const RenderPassBeginInfo,
        contents: SubpassContents,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass.html>
    #[doc(alias = "vkCmdNextSubpass")]
    pub unsafe extern "C" fn cmd_next_subpass(self, contents: SubpassContents) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass.html>
    #[doc(alias = "vkCmdEndRenderPass")]
    pub unsafe extern "C" fn cmd_end_render_pass(self) {
        todo!()
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceVersion.html>
#[doc(alias = "vkEnumerateInstanceVersion")]
pub unsafe extern "C" fn enumerate_instance_version(p_api_version: *mut u32) -> ResultCode {
    todo!()
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory2.html>
    #[doc(alias = "vkBindBufferMemory2")]
    pub unsafe extern "C" fn bind_buffer_memory_2(
        self,
        bind_info_count: u32,
        p_bind_infos: *const BindBufferMemoryInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory2.html>
    #[doc(alias = "vkBindImageMemory2")]
    pub unsafe extern "C" fn bind_image_memory_2(
        self,
        bind_info_count: u32,
        p_bind_infos: *const BindImageMemoryInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPeerMemoryFeatures.html>
    #[doc(alias = "vkGetDeviceGroupPeerMemoryFeatures")]
    pub unsafe extern "C" fn get_device_group_peer_memory_features(
        self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
        p_peer_memory_features: *mut PeerMemoryFeatureFlags,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDeviceMask.html>
    #[doc(alias = "vkCmdSetDeviceMask")]
    pub unsafe extern "C" fn cmd_set_device_mask(self, device_mask: u32) {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroups.html>
    #[doc(alias = "vkEnumeratePhysicalDeviceGroups")]
    pub unsafe extern "C" fn enumerate_physical_device_groups(
        self,
        p_physical_device_group_count: *mut u32,
        p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements2.html>
    #[doc(alias = "vkGetImageMemoryRequirements2")]
    pub unsafe extern "C" fn get_image_memory_requirements_2(
        self,
        p_info: *const ImageMemoryRequirementsInfo2,
        p_memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements2.html>
    #[doc(alias = "vkGetBufferMemoryRequirements2")]
    pub unsafe extern "C" fn get_buffer_memory_requirements_2(
        self,
        p_info: *const BufferMemoryRequirementsInfo2,
        p_memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2.html>
    #[doc(alias = "vkGetImageSparseMemoryRequirements2")]
    pub unsafe extern "C" fn get_image_sparse_memory_requirements_2(
        self,
        p_info: *const ImageSparseMemoryRequirementsInfo2,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures2.html>
    #[doc(alias = "vkGetPhysicalDeviceFeatures2")]
    pub unsafe extern "C" fn get_physical_device_features_2(
        self,
        p_features: *mut PhysicalDeviceFeatures2,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties2.html>
    #[doc(alias = "vkGetPhysicalDeviceProperties2")]
    pub unsafe extern "C" fn get_physical_device_properties_2(
        self,
        p_properties: *mut PhysicalDeviceProperties2,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties2.html>
    #[doc(alias = "vkGetPhysicalDeviceFormatProperties2")]
    pub unsafe extern "C" fn get_physical_device_format_properties_2(
        self,
        format: Format,
        p_format_properties: *mut FormatProperties2,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties2.html>
    #[doc(alias = "vkGetPhysicalDeviceImageFormatProperties2")]
    pub unsafe extern "C" fn get_physical_device_image_format_properties_2(
        self,
        p_image_format_info: *const PhysicalDeviceImageFormatInfo2,
        p_image_format_properties: *mut ImageFormatProperties2,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2.html>
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2")]
    pub unsafe extern "C" fn get_physical_device_queue_family_properties_2(
        self,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut QueueFamilyProperties2,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties2.html>
    #[doc(alias = "vkGetPhysicalDeviceMemoryProperties2")]
    pub unsafe extern "C" fn get_physical_device_memory_properties_2(
        self,
        p_memory_properties: *mut PhysicalDeviceMemoryProperties2,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2.html>
    #[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2")]
    pub unsafe extern "C" fn get_physical_device_sparse_image_format_properties_2(
        self,
        p_format_info: *const PhysicalDeviceSparseImageFormatInfo2,
        p_property_count: *mut u32,
        p_properties: *mut SparseImageFormatProperties2,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkTrimCommandPool.html>
    #[doc(alias = "vkTrimCommandPool")]
    pub unsafe extern "C" fn trim_command_pool(
        self,
        command_pool: CommandPool,
        flags: CommandPoolTrimFlags,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue2.html>
    #[doc(alias = "vkGetDeviceQueue2")]
    pub unsafe extern "C" fn get_device_queue_2(
        self,
        p_queue_info: *const DeviceQueueInfo2,
        p_queue: *mut Queue,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalBufferProperties.html>
    #[doc(alias = "vkGetPhysicalDeviceExternalBufferProperties")]
    pub unsafe extern "C" fn get_physical_device_external_buffer_properties(
        self,
        p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo,
        p_external_buffer_properties: *mut ExternalBufferProperties,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalFenceProperties.html>
    #[doc(alias = "vkGetPhysicalDeviceExternalFenceProperties")]
    pub unsafe extern "C" fn get_physical_device_external_fence_properties(
        self,
        p_external_fence_info: *const PhysicalDeviceExternalFenceInfo,
        p_external_fence_properties: *mut ExternalFenceProperties,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalSemaphoreProperties.html>
    #[doc(alias = "vkGetPhysicalDeviceExternalSemaphoreProperties")]
    pub unsafe extern "C" fn get_physical_device_external_semaphore_properties(
        self,
        p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo,
        p_external_semaphore_properties: *mut ExternalSemaphoreProperties,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchBase.html>
    #[doc(alias = "vkCmdDispatchBase")]
    pub unsafe extern "C" fn cmd_dispatch_base(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorUpdateTemplate.html>
    #[doc(alias = "vkCreateDescriptorUpdateTemplate")]
    pub unsafe extern "C" fn create_descriptor_update_template(
        self,
        p_create_info: *const DescriptorUpdateTemplateCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_descriptor_update_template: *mut DescriptorUpdateTemplate,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplate.html>
    #[doc(alias = "vkDestroyDescriptorUpdateTemplate")]
    pub unsafe extern "C" fn destroy_descriptor_update_template(
        self,
        descriptor_update_template: DescriptorUpdateTemplate,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplate.html>
    #[doc(alias = "vkUpdateDescriptorSetWithTemplate")]
    pub unsafe extern "C" fn update_descriptor_set_with_template(
        self,
        descriptor_set: DescriptorSet,
        descriptor_update_template: DescriptorUpdateTemplate,
        p_data: *const c_void,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSupport.html>
    #[doc(alias = "vkGetDescriptorSetLayoutSupport")]
    pub unsafe extern "C" fn get_descriptor_set_layout_support(
        self,
        p_create_info: *const DescriptorSetLayoutCreateInfo,
        p_support: *mut DescriptorSetLayoutSupport,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSamplerYcbcrConversion.html>
    #[doc(alias = "vkCreateSamplerYcbcrConversion")]
    pub unsafe extern "C" fn create_sampler_ycbcr_conversion(
        self,
        p_create_info: *const SamplerYcbcrConversionCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_ycbcr_conversion: *mut SamplerYcbcrConversion,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversion.html>
    #[doc(alias = "vkDestroySamplerYcbcrConversion")]
    pub unsafe extern "C" fn destroy_sampler_ycbcr_conversion(
        self,
        ycbcr_conversion: SamplerYcbcrConversion,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkResetQueryPool.html>
    #[doc(alias = "vkResetQueryPool")]
    pub unsafe extern "C" fn reset_query_pool(
        self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValue.html>
    #[doc(alias = "vkGetSemaphoreCounterValue")]
    pub unsafe extern "C" fn get_semaphore_counter_value(
        self,
        semaphore: Semaphore,
        p_value: *mut u64,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitSemaphores.html>
    #[doc(alias = "vkWaitSemaphores")]
    pub unsafe extern "C" fn wait_semaphores(
        self,
        p_wait_info: *const SemaphoreWaitInfo,
        timeout: u64,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSignalSemaphore.html>
    #[doc(alias = "vkSignalSemaphore")]
    pub unsafe extern "C" fn signal_semaphore(
        self,
        p_signal_info: *const SemaphoreSignalInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddress.html>
    #[doc(alias = "vkGetBufferDeviceAddress")]
    pub unsafe extern "C" fn get_buffer_device_address(
        self,
        p_info: *const BufferDeviceAddressInfo,
    ) -> DeviceAddress {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureAddress.html>
    #[doc(alias = "vkGetBufferOpaqueCaptureAddress")]
    pub unsafe extern "C" fn get_buffer_opaque_capture_address(
        self,
        p_info: *const BufferDeviceAddressInfo,
    ) -> u64 {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryOpaqueCaptureAddress.html>
    #[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddress")]
    pub unsafe extern "C" fn get_device_memory_opaque_capture_address(
        self,
        p_info: *const DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> u64 {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount.html>
    #[doc(alias = "vkCmdDrawIndirectCount")]
    pub unsafe extern "C" fn cmd_draw_indirect_count(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount.html>
    #[doc(alias = "vkCmdDrawIndexedIndirectCount")]
    pub unsafe extern "C" fn cmd_draw_indexed_indirect_count(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass2.html>
    #[doc(alias = "vkCreateRenderPass2")]
    pub unsafe extern "C" fn create_render_pass_2(
        self,
        p_create_info: *const RenderPassCreateInfo2,
        p_allocator: *const AllocationCallbacks,
        p_render_pass: *mut RenderPass,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass2.html>
    #[doc(alias = "vkCmdBeginRenderPass2")]
    pub unsafe extern "C" fn cmd_begin_render_pass_2(
        self,
        p_render_pass_begin: *const RenderPassBeginInfo,
        p_subpass_begin_info: *const SubpassBeginInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass2.html>
    #[doc(alias = "vkCmdNextSubpass2")]
    pub unsafe extern "C" fn cmd_next_subpass_2(
        self,
        p_subpass_begin_info: *const SubpassBeginInfo,
        p_subpass_end_info: *const SubpassEndInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass2.html>
    #[doc(alias = "vkCmdEndRenderPass2")]
    pub unsafe extern "C" fn cmd_end_render_pass_2(
        self,
        p_subpass_end_info: *const SubpassEndInfo,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolProperties.html>
    #[doc(alias = "vkGetPhysicalDeviceToolProperties")]
    pub unsafe extern "C" fn get_physical_device_tool_properties(
        self,
        p_tool_count: *mut u32,
        p_tool_properties: *mut PhysicalDeviceToolProperties,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePrivateDataSlot.html>
    #[doc(alias = "vkCreatePrivateDataSlot")]
    pub unsafe extern "C" fn create_private_data_slot(
        self,
        p_create_info: *const PrivateDataSlotCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_private_data_slot: *mut PrivateDataSlot,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlot.html>
    #[doc(alias = "vkDestroyPrivateDataSlot")]
    pub unsafe extern "C" fn destroy_private_data_slot(
        self,
        private_data_slot: PrivateDataSlot,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetPrivateData.html>
    #[doc(alias = "vkSetPrivateData")]
    pub unsafe extern "C" fn set_private_data(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPrivateData.html>
    #[doc(alias = "vkGetPrivateData")]
    pub unsafe extern "C" fn get_private_data(
        self,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        p_data: *mut u64,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier2.html>
    #[doc(alias = "vkCmdPipelineBarrier2")]
    pub unsafe extern "C" fn cmd_pipeline_barrier_2(
        self,
        p_dependency_info: *const DependencyInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp2.html>
    #[doc(alias = "vkCmdWriteTimestamp2")]
    pub unsafe extern "C" fn cmd_write_timestamp_2(
        self,
        stage: PipelineStageFlags2,
        query_pool: QueryPool,
        query: u32,
    ) {
        todo!()
    }
}

impl Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2.html>
    #[doc(alias = "vkQueueSubmit2")]
    pub unsafe extern "C" fn queue_submit_2(
        self,
        submit_count: u32,
        p_submits: *const SubmitInfo2,
        fence: Fence,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2.html>
    #[doc(alias = "vkCmdCopyBuffer2")]
    pub unsafe extern "C" fn cmd_copy_buffer_2(self, p_copy_buffer_info: *const CopyBufferInfo2) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2.html>
    #[doc(alias = "vkCmdCopyImage2")]
    pub unsafe extern "C" fn cmd_copy_image_2(self, p_copy_image_info: *const CopyImageInfo2) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2.html>
    #[doc(alias = "vkCmdCopyBufferToImage2")]
    pub unsafe extern "C" fn cmd_copy_buffer_to_image_2(
        self,
        p_copy_buffer_to_image_info: *const CopyBufferToImageInfo2,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2.html>
    #[doc(alias = "vkCmdCopyImageToBuffer2")]
    pub unsafe extern "C" fn cmd_copy_image_to_buffer_2(
        self,
        p_copy_image_to_buffer_info: *const CopyImageToBufferInfo2,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceBufferMemoryRequirements.html>
    #[doc(alias = "vkGetDeviceBufferMemoryRequirements")]
    pub unsafe extern "C" fn get_device_buffer_memory_requirements(
        self,
        p_info: *const DeviceBufferMemoryRequirements,
        p_memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageMemoryRequirements.html>
    #[doc(alias = "vkGetDeviceImageMemoryRequirements")]
    pub unsafe extern "C" fn get_device_image_memory_requirements(
        self,
        p_info: *const DeviceImageMemoryRequirements,
        p_memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirements.html>
    #[doc(alias = "vkGetDeviceImageSparseMemoryRequirements")]
    pub unsafe extern "C" fn get_device_image_sparse_memory_requirements(
        self,
        p_info: *const DeviceImageMemoryRequirements,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent2.html>
    #[doc(alias = "vkCmdSetEvent2")]
    pub unsafe extern "C" fn cmd_set_event_2(
        self,
        event: Event,
        p_dependency_info: *const DependencyInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent2.html>
    #[doc(alias = "vkCmdResetEvent2")]
    pub unsafe extern "C" fn cmd_reset_event_2(
        self,
        event: Event,
        stage_mask: PipelineStageFlags2,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents2.html>
    #[doc(alias = "vkCmdWaitEvents2")]
    pub unsafe extern "C" fn cmd_wait_events_2(
        self,
        event_count: u32,
        p_events: *const Event,
        p_dependency_infos: *const DependencyInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2.html>
    #[doc(alias = "vkCmdBlitImage2")]
    pub unsafe extern "C" fn cmd_blit_image_2(self, p_blit_image_info: *const BlitImageInfo2) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage2.html>
    #[doc(alias = "vkCmdResolveImage2")]
    pub unsafe extern "C" fn cmd_resolve_image_2(
        self,
        p_resolve_image_info: *const ResolveImageInfo2,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRendering.html>
    #[doc(alias = "vkCmdBeginRendering")]
    pub unsafe extern "C" fn cmd_begin_rendering(self, p_rendering_info: *const RenderingInfo) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering.html>
    #[doc(alias = "vkCmdEndRendering")]
    pub unsafe extern "C" fn cmd_end_rendering(self) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCullMode.html>
    #[doc(alias = "vkCmdSetCullMode")]
    pub unsafe extern "C" fn cmd_set_cull_mode(self, cull_mode: CullModeFlags) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFrontFace.html>
    #[doc(alias = "vkCmdSetFrontFace")]
    pub unsafe extern "C" fn cmd_set_front_face(self, front_face: FrontFace) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveTopology.html>
    #[doc(alias = "vkCmdSetPrimitiveTopology")]
    pub unsafe extern "C" fn cmd_set_primitive_topology(
        self,
        primitive_topology: PrimitiveTopology,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCount.html>
    #[doc(alias = "vkCmdSetViewportWithCount")]
    pub unsafe extern "C" fn cmd_set_viewport_with_count(
        self,
        viewport_count: u32,
        p_viewports: *const Viewport,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCount.html>
    #[doc(alias = "vkCmdSetScissorWithCount")]
    pub unsafe extern "C" fn cmd_set_scissor_with_count(
        self,
        scissor_count: u32,
        p_scissors: *const Rect2D,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2.html>
    #[doc(alias = "vkCmdBindVertexBuffers2")]
    pub unsafe extern "C" fn cmd_bind_vertex_buffers_2(
        self,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const Buffer,
        p_offsets: *const DeviceSize,
        p_sizes: *const DeviceSize,
        p_strides: *const DeviceSize,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthTestEnable.html>
    #[doc(alias = "vkCmdSetDepthTestEnable")]
    pub unsafe extern "C" fn cmd_set_depth_test_enable(self, depth_test_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthWriteEnable.html>
    #[doc(alias = "vkCmdSetDepthWriteEnable")]
    pub unsafe extern "C" fn cmd_set_depth_write_enable(self, depth_write_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthCompareOp.html>
    #[doc(alias = "vkCmdSetDepthCompareOp")]
    pub unsafe extern "C" fn cmd_set_depth_compare_op(self, depth_compare_op: CompareOp) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBoundsTestEnable.html>
    #[doc(alias = "vkCmdSetDepthBoundsTestEnable")]
    pub unsafe extern "C" fn cmd_set_depth_bounds_test_enable(
        self,
        depth_bounds_test_enable: Bool32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilTestEnable.html>
    #[doc(alias = "vkCmdSetStencilTestEnable")]
    pub unsafe extern "C" fn cmd_set_stencil_test_enable(self, stencil_test_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilOp.html>
    #[doc(alias = "vkCmdSetStencilOp")]
    pub unsafe extern "C" fn cmd_set_stencil_op(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizerDiscardEnable.html>
    #[doc(alias = "vkCmdSetRasterizerDiscardEnable")]
    pub unsafe extern "C" fn cmd_set_rasterizer_discard_enable(
        self,
        rasterizer_discard_enable: Bool32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBiasEnable.html>
    #[doc(alias = "vkCmdSetDepthBiasEnable")]
    pub unsafe extern "C" fn cmd_set_depth_bias_enable(self, depth_bias_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartEnable.html>
    #[doc(alias = "vkCmdSetPrimitiveRestartEnable")]
    pub unsafe extern "C" fn cmd_set_primitive_restart_enable(
        self,
        primitive_restart_enable: Bool32,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2.html>
    #[doc(alias = "vkMapMemory2")]
    pub unsafe extern "C" fn map_memory_2(
        self,
        p_memory_map_info: *const MemoryMapInfo,
        pp_data: *mut *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2.html>
    #[doc(alias = "vkUnmapMemory2")]
    pub unsafe extern "C" fn unmap_memory_2(
        self,
        p_memory_unmap_info: *const MemoryUnmapInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSubresourceLayout.html>
    #[doc(alias = "vkGetDeviceImageSubresourceLayout")]
    pub unsafe extern "C" fn get_device_image_subresource_layout(
        self,
        p_info: *const DeviceImageSubresourceInfo,
        p_layout: *mut SubresourceLayout2,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2.html>
    #[doc(alias = "vkGetImageSubresourceLayout2")]
    pub unsafe extern "C" fn get_image_subresource_layout_2(
        self,
        image: Image,
        p_subresource: *const ImageSubresource2,
        p_layout: *mut SubresourceLayout2,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToImage.html>
    #[doc(alias = "vkCopyMemoryToImage")]
    pub unsafe extern "C" fn copy_memory_to_image(
        self,
        p_copy_memory_to_image_info: *const CopyMemoryToImageInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToMemory.html>
    #[doc(alias = "vkCopyImageToMemory")]
    pub unsafe extern "C" fn copy_image_to_memory(
        self,
        p_copy_image_to_memory_info: *const CopyImageToMemoryInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToImage.html>
    #[doc(alias = "vkCopyImageToImage")]
    pub unsafe extern "C" fn copy_image_to_image(
        self,
        p_copy_image_to_image_info: *const CopyImageToImageInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkTransitionImageLayout.html>
    #[doc(alias = "vkTransitionImageLayout")]
    pub unsafe extern "C" fn transition_image_layout(
        self,
        transition_count: u32,
        p_transitions: *const HostImageLayoutTransitionInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet.html>
    #[doc(alias = "vkCmdPushDescriptorSet")]
    pub unsafe extern "C" fn cmd_push_descriptor_set(
        self,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
        descriptor_write_count: u32,
        p_descriptor_writes: *const WriteDescriptorSet,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate.html>
    #[doc(alias = "vkCmdPushDescriptorSetWithTemplate")]
    pub unsafe extern "C" fn cmd_push_descriptor_set_with_template(
        self,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: u32,
        p_data: *const c_void,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2.html>
    #[doc(alias = "vkCmdBindDescriptorSets2")]
    pub unsafe extern "C" fn cmd_bind_descriptor_sets_2(
        self,
        p_bind_descriptor_sets_info: *const BindDescriptorSetsInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2.html>
    #[doc(alias = "vkCmdPushConstants2")]
    pub unsafe extern "C" fn cmd_push_constants_2(
        self,
        p_push_constants_info: *const PushConstantsInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2.html>
    #[doc(alias = "vkCmdPushDescriptorSet2")]
    pub unsafe extern "C" fn cmd_push_descriptor_set_2(
        self,
        p_push_descriptor_set_info: *const PushDescriptorSetInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate2.html>
    #[doc(alias = "vkCmdPushDescriptorSetWithTemplate2")]
    pub unsafe extern "C" fn cmd_push_descriptor_set_with_template_2(
        self,
        p_push_descriptor_set_with_template_info: *const PushDescriptorSetWithTemplateInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStipple.html>
    #[doc(alias = "vkCmdSetLineStipple")]
    pub unsafe extern "C" fn cmd_set_line_stipple(
        self,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer2.html>
    #[doc(alias = "vkCmdBindIndexBuffer2")]
    pub unsafe extern "C" fn cmd_bind_index_buffer_2(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderingAreaGranularity.html>
    #[doc(alias = "vkGetRenderingAreaGranularity")]
    pub unsafe extern "C" fn get_rendering_area_granularity(
        self,
        p_rendering_area_info: *const RenderingAreaInfo,
        p_granularity: *mut Extent2D,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocations.html>
    #[doc(alias = "vkCmdSetRenderingAttachmentLocations")]
    pub unsafe extern "C" fn cmd_set_rendering_attachment_locations(
        self,
        p_location_info: *const RenderingAttachmentLocationInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndices.html>
    #[doc(alias = "vkCmdSetRenderingInputAttachmentIndices")]
    pub unsafe extern "C" fn cmd_set_rendering_input_attachment_indices(
        self,
        p_input_attachment_index_info: *const RenderingInputAttachmentIndexInfo,
    ) {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySurfaceKHR.html>
    #[doc(alias = "vkDestroySurfaceKHR")]
    pub unsafe extern "C" fn destroy_surface_khr(
        self,
        surface: SurfaceKHR,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceSupportKHR.html>
    #[doc(alias = "vkGetPhysicalDeviceSurfaceSupportKHR")]
    pub unsafe extern "C" fn get_physical_device_surface_support_khr(
        self,
        queue_family_index: u32,
        surface: SurfaceKHR,
        p_supported: *mut Bool32,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html>
    #[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilitiesKHR")]
    pub unsafe extern "C" fn get_physical_device_surface_capabilities_khr(
        self,
        surface: SurfaceKHR,
        p_surface_capabilities: *mut SurfaceCapabilitiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceFormatsKHR.html>
    #[doc(alias = "vkGetPhysicalDeviceSurfaceFormatsKHR")]
    pub unsafe extern "C" fn get_physical_device_surface_formats_khr(
        self,
        surface: SurfaceKHR,
        p_surface_format_count: *mut u32,
        p_surface_formats: *mut SurfaceFormatKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfacePresentModesKHR.html>
    #[doc(alias = "vkGetPhysicalDeviceSurfacePresentModesKHR")]
    pub unsafe extern "C" fn get_physical_device_surface_present_modes_khr(
        self,
        surface: SurfaceKHR,
        p_present_mode_count: *mut u32,
        p_present_modes: *mut PresentModeKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSwapchainKHR.html>
    #[doc(alias = "vkCreateSwapchainKHR")]
    pub unsafe extern "C" fn create_swapchain_khr(
        self,
        p_create_info: *const SwapchainCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_swapchain: *mut SwapchainKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySwapchainKHR.html>
    #[doc(alias = "vkDestroySwapchainKHR")]
    pub unsafe extern "C" fn destroy_swapchain_khr(
        self,
        swapchain: SwapchainKHR,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainImagesKHR.html>
    #[doc(alias = "vkGetSwapchainImagesKHR")]
    pub unsafe extern "C" fn get_swapchain_images_khr(
        self,
        swapchain: SwapchainKHR,
        p_swapchain_image_count: *mut u32,
        p_swapchain_images: *mut Image,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImageKHR.html>
    #[doc(alias = "vkAcquireNextImageKHR")]
    pub unsafe extern "C" fn acquire_next_image_khr(
        self,
        swapchain: SwapchainKHR,
        timeout: u64,
        semaphore: Semaphore,
        fence: Fence,
        p_image_index: *mut u32,
    ) -> ResultCode {
        todo!()
    }
}

impl Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueuePresentKHR.html>
    #[doc(alias = "vkQueuePresentKHR")]
    pub unsafe extern "C" fn queue_present_khr(
        self,
        p_present_info: *const PresentInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPresentCapabilitiesKHR.html>
    #[doc(alias = "vkGetDeviceGroupPresentCapabilitiesKHR")]
    pub unsafe extern "C" fn get_device_group_present_capabilities_khr(
        self,
        p_device_group_present_capabilities: *mut DeviceGroupPresentCapabilitiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModesKHR.html>
    #[doc(alias = "vkGetDeviceGroupSurfacePresentModesKHR")]
    pub unsafe extern "C" fn get_device_group_surface_present_modes_khr(
        self,
        surface: SurfaceKHR,
        p_modes: *mut DeviceGroupPresentModeFlagsKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDevicePresentRectanglesKHR.html>
    #[doc(alias = "vkGetPhysicalDevicePresentRectanglesKHR")]
    pub unsafe extern "C" fn get_physical_device_present_rectangles_khr(
        self,
        surface: SurfaceKHR,
        p_rect_count: *mut u32,
        p_rects: *mut Rect2D,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImage2KHR.html>
    #[doc(alias = "vkAcquireNextImage2KHR")]
    pub unsafe extern "C" fn acquire_next_image_2_khr(
        self,
        p_acquire_info: *const AcquireNextImageInfoKHR,
        p_image_index: *mut u32,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPropertiesKHR.html>
    #[doc(alias = "vkGetPhysicalDeviceDisplayPropertiesKHR")]
    pub unsafe extern "C" fn get_physical_device_display_properties_khr(
        self,
        p_property_count: *mut u32,
        p_properties: *mut DisplayPropertiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html>
    #[doc(alias = "vkGetPhysicalDeviceDisplayPlanePropertiesKHR")]
    pub unsafe extern "C" fn get_physical_device_display_plane_properties_khr(
        self,
        p_property_count: *mut u32,
        p_properties: *mut DisplayPlanePropertiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneSupportedDisplaysKHR.html>
    #[doc(alias = "vkGetDisplayPlaneSupportedDisplaysKHR")]
    pub unsafe extern "C" fn get_display_plane_supported_displays_khr(
        self,
        plane_index: u32,
        p_display_count: *mut u32,
        p_displays: *mut DisplayKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayModePropertiesKHR.html>
    #[doc(alias = "vkGetDisplayModePropertiesKHR")]
    pub unsafe extern "C" fn get_display_mode_properties_khr(
        self,
        display: DisplayKHR,
        p_property_count: *mut u32,
        p_properties: *mut DisplayModePropertiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayModeKHR.html>
    #[doc(alias = "vkCreateDisplayModeKHR")]
    pub unsafe extern "C" fn create_display_mode_khr(
        self,
        display: DisplayKHR,
        p_create_info: *const DisplayModeCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_mode: *mut DisplayModeKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneCapabilitiesKHR.html>
    #[doc(alias = "vkGetDisplayPlaneCapabilitiesKHR")]
    pub unsafe extern "C" fn get_display_plane_capabilities_khr(
        self,
        mode: DisplayModeKHR,
        plane_index: u32,
        p_capabilities: *mut DisplayPlaneCapabilitiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayPlaneSurfaceKHR.html>
    #[doc(alias = "vkCreateDisplayPlaneSurfaceKHR")]
    pub unsafe extern "C" fn create_display_plane_surface_khr(
        self,
        p_create_info: *const DisplaySurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSharedSwapchainsKHR.html>
    #[doc(alias = "vkCreateSharedSwapchainsKHR")]
    pub unsafe extern "C" fn create_shared_swapchains_khr(
        self,
        swapchain_count: u32,
        p_create_infos: *const SwapchainCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_swapchains: *mut SwapchainKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXlibSurfaceKHR.html>
    #[doc(alias = "vkCreateXlibSurfaceKHR")]
    pub unsafe extern "C" fn create_xlib_surface_khr(
        self,
        p_create_info: *const XlibSurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceXlibPresentationSupportKHR.html>
    #[doc(alias = "vkGetPhysicalDeviceXlibPresentationSupportKHR")]
    pub unsafe extern "C" fn get_physical_device_xlib_presentation_support_khr(
        self,
        queue_family_index: u32,
        dpy: *mut Display,
        visual_id: VisualID,
    ) -> Bool32 {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXcbSurfaceKHR.html>
    #[doc(alias = "vkCreateXcbSurfaceKHR")]
    pub unsafe extern "C" fn create_xcb_surface_khr(
        self,
        p_create_info: *const XcbSurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceXcbPresentationSupportKHR.html>
    #[doc(alias = "vkGetPhysicalDeviceXcbPresentationSupportKHR")]
    pub unsafe extern "C" fn get_physical_device_xcb_presentation_support_khr(
        self,
        queue_family_index: u32,
        connection: *mut xcb_connection_t,
        visual_id: xcb_visualid_t,
    ) -> Bool32 {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWaylandSurfaceKHR.html>
    #[doc(alias = "vkCreateWaylandSurfaceKHR")]
    pub unsafe extern "C" fn create_wayland_surface_khr(
        self,
        p_create_info: *const WaylandSurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html>
    #[doc(alias = "vkGetPhysicalDeviceWaylandPresentationSupportKHR")]
    pub unsafe extern "C" fn get_physical_device_wayland_presentation_support_khr(
        self,
        queue_family_index: u32,
        display: *mut wl_display,
    ) -> Bool32 {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAndroidSurfaceKHR.html>
    #[doc(alias = "vkCreateAndroidSurfaceKHR")]
    pub unsafe extern "C" fn create_android_surface_khr(
        self,
        p_create_info: *const AndroidSurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWin32SurfaceKHR.html>
    #[doc(alias = "vkCreateWin32SurfaceKHR")]
    pub unsafe extern "C" fn create_win_32_surface_khr(
        self,
        p_create_info: *const Win32SurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceWin32PresentationSupportKHR.html>
    #[doc(alias = "vkGetPhysicalDeviceWin32PresentationSupportKHR")]
    pub unsafe extern "C" fn get_physical_device_win_32_presentation_support_khr(
        self,
        queue_family_index: u32,
    ) -> Bool32 {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoCapabilitiesKHR.html>
    #[doc(alias = "vkGetPhysicalDeviceVideoCapabilitiesKHR")]
    pub unsafe extern "C" fn get_physical_device_video_capabilities_khr(
        self,
        p_video_profile: *const VideoProfileInfoKHR,
        p_capabilities: *mut VideoCapabilitiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html>
    #[doc(alias = "vkGetPhysicalDeviceVideoFormatPropertiesKHR")]
    pub unsafe extern "C" fn get_physical_device_video_format_properties_khr(
        self,
        p_video_format_info: *const PhysicalDeviceVideoFormatInfoKHR,
        p_video_format_property_count: *mut u32,
        p_video_format_properties: *mut VideoFormatPropertiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateVideoSessionKHR.html>
    #[doc(alias = "vkCreateVideoSessionKHR")]
    pub unsafe extern "C" fn create_video_session_khr(
        self,
        p_create_info: *const VideoSessionCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_video_session: *mut VideoSessionKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyVideoSessionKHR.html>
    #[doc(alias = "vkDestroyVideoSessionKHR")]
    pub unsafe extern "C" fn destroy_video_session_khr(
        self,
        video_session: VideoSessionKHR,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetVideoSessionMemoryRequirementsKHR.html>
    #[doc(alias = "vkGetVideoSessionMemoryRequirementsKHR")]
    pub unsafe extern "C" fn get_video_session_memory_requirements_khr(
        self,
        video_session: VideoSessionKHR,
        p_memory_requirements_count: *mut u32,
        p_memory_requirements: *mut VideoSessionMemoryRequirementsKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindVideoSessionMemoryKHR.html>
    #[doc(alias = "vkBindVideoSessionMemoryKHR")]
    pub unsafe extern "C" fn bind_video_session_memory_khr(
        self,
        video_session: VideoSessionKHR,
        bind_session_memory_info_count: u32,
        p_bind_session_memory_infos: *const BindVideoSessionMemoryInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateVideoSessionParametersKHR.html>
    #[doc(alias = "vkCreateVideoSessionParametersKHR")]
    pub unsafe extern "C" fn create_video_session_parameters_khr(
        self,
        p_create_info: *const VideoSessionParametersCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_video_session_parameters: *mut VideoSessionParametersKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateVideoSessionParametersKHR.html>
    #[doc(alias = "vkUpdateVideoSessionParametersKHR")]
    pub unsafe extern "C" fn update_video_session_parameters_khr(
        self,
        video_session_parameters: VideoSessionParametersKHR,
        p_update_info: *const VideoSessionParametersUpdateInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyVideoSessionParametersKHR.html>
    #[doc(alias = "vkDestroyVideoSessionParametersKHR")]
    pub unsafe extern "C" fn destroy_video_session_parameters_khr(
        self,
        video_session_parameters: VideoSessionParametersKHR,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginVideoCodingKHR.html>
    #[doc(alias = "vkCmdBeginVideoCodingKHR")]
    pub unsafe extern "C" fn cmd_begin_video_coding_khr(
        self,
        p_begin_info: *const VideoBeginCodingInfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndVideoCodingKHR.html>
    #[doc(alias = "vkCmdEndVideoCodingKHR")]
    pub unsafe extern "C" fn cmd_end_video_coding_khr(
        self,
        p_end_coding_info: *const VideoEndCodingInfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdControlVideoCodingKHR.html>
    #[doc(alias = "vkCmdControlVideoCodingKHR")]
    pub unsafe extern "C" fn cmd_control_video_coding_khr(
        self,
        p_coding_control_info: *const VideoCodingControlInfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecodeVideoKHR.html>
    #[doc(alias = "vkCmdDecodeVideoKHR")]
    pub unsafe extern "C" fn cmd_decode_video_khr(self, p_decode_info: *const VideoDecodeInfoKHR) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderingKHR.html>
    #[doc(alias = "vkCmdBeginRenderingKHR")]
    pub unsafe extern "C" fn cmd_begin_rendering_khr(self, p_rendering_info: *const RenderingInfo) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderingKHR.html>
    #[doc(alias = "vkCmdEndRenderingKHR")]
    pub unsafe extern "C" fn cmd_end_rendering_khr(self) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures2KHR.html>
    #[doc(alias = "vkGetPhysicalDeviceFeatures2KHR")]
    pub unsafe extern "C" fn get_physical_device_features_2_khr(
        self,
        p_features: *mut PhysicalDeviceFeatures2,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties2KHR.html>
    #[doc(alias = "vkGetPhysicalDeviceProperties2KHR")]
    pub unsafe extern "C" fn get_physical_device_properties_2_khr(
        self,
        p_properties: *mut PhysicalDeviceProperties2,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties2KHR.html>
    #[doc(alias = "vkGetPhysicalDeviceFormatProperties2KHR")]
    pub unsafe extern "C" fn get_physical_device_format_properties_2_khr(
        self,
        format: Format,
        p_format_properties: *mut FormatProperties2,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties2KHR.html>
    #[doc(alias = "vkGetPhysicalDeviceImageFormatProperties2KHR")]
    pub unsafe extern "C" fn get_physical_device_image_format_properties_2_khr(
        self,
        p_image_format_info: *const PhysicalDeviceImageFormatInfo2,
        p_image_format_properties: *mut ImageFormatProperties2,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2KHR.html>
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2KHR")]
    pub unsafe extern "C" fn get_physical_device_queue_family_properties_2_khr(
        self,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut QueueFamilyProperties2,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties2KHR.html>
    #[doc(alias = "vkGetPhysicalDeviceMemoryProperties2KHR")]
    pub unsafe extern "C" fn get_physical_device_memory_properties_2_khr(
        self,
        p_memory_properties: *mut PhysicalDeviceMemoryProperties2,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2KHR.html>
    #[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2KHR")]
    pub unsafe extern "C" fn get_physical_device_sparse_image_format_properties_2_khr(
        self,
        p_format_info: *const PhysicalDeviceSparseImageFormatInfo2,
        p_property_count: *mut u32,
        p_properties: *mut SparseImageFormatProperties2,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPeerMemoryFeaturesKHR.html>
    #[doc(alias = "vkGetDeviceGroupPeerMemoryFeaturesKHR")]
    pub unsafe extern "C" fn get_device_group_peer_memory_features_khr(
        self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
        p_peer_memory_features: *mut PeerMemoryFeatureFlags,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDeviceMaskKHR.html>
    #[doc(alias = "vkCmdSetDeviceMaskKHR")]
    pub unsafe extern "C" fn cmd_set_device_mask_khr(self, device_mask: u32) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchBaseKHR.html>
    #[doc(alias = "vkCmdDispatchBaseKHR")]
    pub unsafe extern "C" fn cmd_dispatch_base_khr(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkTrimCommandPoolKHR.html>
    #[doc(alias = "vkTrimCommandPoolKHR")]
    pub unsafe extern "C" fn trim_command_pool_khr(
        self,
        command_pool: CommandPool,
        flags: CommandPoolTrimFlags,
    ) {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroupsKHR.html>
    #[doc(alias = "vkEnumeratePhysicalDeviceGroupsKHR")]
    pub unsafe extern "C" fn enumerate_physical_device_groups_khr(
        self,
        p_physical_device_group_count: *mut u32,
        p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalBufferPropertiesKHR.html>
    #[doc(alias = "vkGetPhysicalDeviceExternalBufferPropertiesKHR")]
    pub unsafe extern "C" fn get_physical_device_external_buffer_properties_khr(
        self,
        p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo,
        p_external_buffer_properties: *mut ExternalBufferProperties,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleKHR.html>
    #[doc(alias = "vkGetMemoryWin32HandleKHR")]
    pub unsafe extern "C" fn get_memory_win_32_handle_khr(
        self,
        p_get_win_32_handle_info: *const MemoryGetWin32HandleInfoKHR,
        p_handle: *mut HANDLE,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandlePropertiesKHR.html>
    #[doc(alias = "vkGetMemoryWin32HandlePropertiesKHR")]
    pub unsafe extern "C" fn get_memory_win_32_handle_properties_khr(
        self,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: HANDLE,
        p_memory_win_32_handle_properties: *mut MemoryWin32HandlePropertiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryFdKHR.html>
    #[doc(alias = "vkGetMemoryFdKHR")]
    pub unsafe extern "C" fn get_memory_fd_khr(
        self,
        p_get_fd_info: *const MemoryGetFdInfoKHR,
        p_fd: *mut c_int,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryFdPropertiesKHR.html>
    #[doc(alias = "vkGetMemoryFdPropertiesKHR")]
    pub unsafe extern "C" fn get_memory_fd_properties_khr(
        self,
        handle_type: ExternalMemoryHandleTypeFlags,
        fd: c_int,
        p_memory_fd_properties: *mut MemoryFdPropertiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalSemaphorePropertiesKHR.html>
    #[doc(alias = "vkGetPhysicalDeviceExternalSemaphorePropertiesKHR")]
    pub unsafe extern "C" fn get_physical_device_external_semaphore_properties_khr(
        self,
        p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo,
        p_external_semaphore_properties: *mut ExternalSemaphoreProperties,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreWin32HandleKHR.html>
    #[doc(alias = "vkImportSemaphoreWin32HandleKHR")]
    pub unsafe extern "C" fn import_semaphore_win_32_handle_khr(
        self,
        p_import_semaphore_win_32_handle_info: *const ImportSemaphoreWin32HandleInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreWin32HandleKHR.html>
    #[doc(alias = "vkGetSemaphoreWin32HandleKHR")]
    pub unsafe extern "C" fn get_semaphore_win_32_handle_khr(
        self,
        p_get_win_32_handle_info: *const SemaphoreGetWin32HandleInfoKHR,
        p_handle: *mut HANDLE,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreFdKHR.html>
    #[doc(alias = "vkImportSemaphoreFdKHR")]
    pub unsafe extern "C" fn import_semaphore_fd_khr(
        self,
        p_import_semaphore_fd_info: *const ImportSemaphoreFdInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreFdKHR.html>
    #[doc(alias = "vkGetSemaphoreFdKHR")]
    pub unsafe extern "C" fn get_semaphore_fd_khr(
        self,
        p_get_fd_info: *const SemaphoreGetFdInfoKHR,
        p_fd: *mut c_int,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetKHR.html>
    #[doc(alias = "vkCmdPushDescriptorSetKHR")]
    pub unsafe extern "C" fn cmd_push_descriptor_set_khr(
        self,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
        descriptor_write_count: u32,
        p_descriptor_writes: *const WriteDescriptorSet,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplateKHR.html>
    #[doc(alias = "vkCmdPushDescriptorSetWithTemplateKHR")]
    pub unsafe extern "C" fn cmd_push_descriptor_set_with_template_khr(
        self,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: u32,
        p_data: *const c_void,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorUpdateTemplateKHR.html>
    #[doc(alias = "vkCreateDescriptorUpdateTemplateKHR")]
    pub unsafe extern "C" fn create_descriptor_update_template_khr(
        self,
        p_create_info: *const DescriptorUpdateTemplateCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_descriptor_update_template: *mut DescriptorUpdateTemplate,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplateKHR.html>
    #[doc(alias = "vkDestroyDescriptorUpdateTemplateKHR")]
    pub unsafe extern "C" fn destroy_descriptor_update_template_khr(
        self,
        descriptor_update_template: DescriptorUpdateTemplate,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplateKHR.html>
    #[doc(alias = "vkUpdateDescriptorSetWithTemplateKHR")]
    pub unsafe extern "C" fn update_descriptor_set_with_template_khr(
        self,
        descriptor_set: DescriptorSet,
        descriptor_update_template: DescriptorUpdateTemplate,
        p_data: *const c_void,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass2KHR.html>
    #[doc(alias = "vkCreateRenderPass2KHR")]
    pub unsafe extern "C" fn create_render_pass_2_khr(
        self,
        p_create_info: *const RenderPassCreateInfo2,
        p_allocator: *const AllocationCallbacks,
        p_render_pass: *mut RenderPass,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass2KHR.html>
    #[doc(alias = "vkCmdBeginRenderPass2KHR")]
    pub unsafe extern "C" fn cmd_begin_render_pass_2_khr(
        self,
        p_render_pass_begin: *const RenderPassBeginInfo,
        p_subpass_begin_info: *const SubpassBeginInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass2KHR.html>
    #[doc(alias = "vkCmdNextSubpass2KHR")]
    pub unsafe extern "C" fn cmd_next_subpass_2_khr(
        self,
        p_subpass_begin_info: *const SubpassBeginInfo,
        p_subpass_end_info: *const SubpassEndInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass2KHR.html>
    #[doc(alias = "vkCmdEndRenderPass2KHR")]
    pub unsafe extern "C" fn cmd_end_render_pass_2_khr(
        self,
        p_subpass_end_info: *const SubpassEndInfo,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainStatusKHR.html>
    #[doc(alias = "vkGetSwapchainStatusKHR")]
    pub unsafe extern "C" fn get_swapchain_status_khr(self, swapchain: SwapchainKHR) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalFencePropertiesKHR.html>
    #[doc(alias = "vkGetPhysicalDeviceExternalFencePropertiesKHR")]
    pub unsafe extern "C" fn get_physical_device_external_fence_properties_khr(
        self,
        p_external_fence_info: *const PhysicalDeviceExternalFenceInfo,
        p_external_fence_properties: *mut ExternalFenceProperties,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceWin32HandleKHR.html>
    #[doc(alias = "vkImportFenceWin32HandleKHR")]
    pub unsafe extern "C" fn import_fence_win_32_handle_khr(
        self,
        p_import_fence_win_32_handle_info: *const ImportFenceWin32HandleInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceWin32HandleKHR.html>
    #[doc(alias = "vkGetFenceWin32HandleKHR")]
    pub unsafe extern "C" fn get_fence_win_32_handle_khr(
        self,
        p_get_win_32_handle_info: *const FenceGetWin32HandleInfoKHR,
        p_handle: *mut HANDLE,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceFdKHR.html>
    #[doc(alias = "vkImportFenceFdKHR")]
    pub unsafe extern "C" fn import_fence_fd_khr(
        self,
        p_import_fence_fd_info: *const ImportFenceFdInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceFdKHR.html>
    #[doc(alias = "vkGetFenceFdKHR")]
    pub unsafe extern "C" fn get_fence_fd_khr(
        self,
        p_get_fd_info: *const FenceGetFdInfoKHR,
        p_fd: *mut c_int,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html>
    #[doc(alias = "vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR")]
    pub unsafe extern "C" fn enumerate_physical_device_queue_family_performance_query_counters_khr(
        self,
        queue_family_index: u32,
        p_counter_count: *mut u32,
        p_counters: *mut PerformanceCounterKHR,
        p_counter_descriptions: *mut PerformanceCounterDescriptionKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html>
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR")]
    pub unsafe extern "C" fn get_physical_device_queue_family_performance_query_passes_khr(
        self,
        p_performance_query_create_info: *const QueryPoolPerformanceCreateInfoKHR,
        p_num_passes: *mut u32,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireProfilingLockKHR.html>
    #[doc(alias = "vkAcquireProfilingLockKHR")]
    pub unsafe extern "C" fn acquire_profiling_lock_khr(
        self,
        p_info: *const AcquireProfilingLockInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseProfilingLockKHR.html>
    #[doc(alias = "vkReleaseProfilingLockKHR")]
    pub unsafe extern "C" fn release_profiling_lock_khr(self) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html>
    #[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2KHR")]
    pub unsafe extern "C" fn get_physical_device_surface_capabilities_2_khr(
        self,
        p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
        p_surface_capabilities: *mut SurfaceCapabilities2KHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceFormats2KHR.html>
    #[doc(alias = "vkGetPhysicalDeviceSurfaceFormats2KHR")]
    pub unsafe extern "C" fn get_physical_device_surface_formats_2_khr(
        self,
        p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
        p_surface_format_count: *mut u32,
        p_surface_formats: *mut SurfaceFormat2KHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayProperties2KHR.html>
    #[doc(alias = "vkGetPhysicalDeviceDisplayProperties2KHR")]
    pub unsafe extern "C" fn get_physical_device_display_properties_2_khr(
        self,
        p_property_count: *mut u32,
        p_properties: *mut DisplayProperties2KHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html>
    #[doc(alias = "vkGetPhysicalDeviceDisplayPlaneProperties2KHR")]
    pub unsafe extern "C" fn get_physical_device_display_plane_properties_2_khr(
        self,
        p_property_count: *mut u32,
        p_properties: *mut DisplayPlaneProperties2KHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayModeProperties2KHR.html>
    #[doc(alias = "vkGetDisplayModeProperties2KHR")]
    pub unsafe extern "C" fn get_display_mode_properties_2_khr(
        self,
        display: DisplayKHR,
        p_property_count: *mut u32,
        p_properties: *mut DisplayModeProperties2KHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneCapabilities2KHR.html>
    #[doc(alias = "vkGetDisplayPlaneCapabilities2KHR")]
    pub unsafe extern "C" fn get_display_plane_capabilities_2_khr(
        self,
        p_display_plane_info: *const DisplayPlaneInfo2KHR,
        p_capabilities: *mut DisplayPlaneCapabilities2KHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements2KHR.html>
    #[doc(alias = "vkGetImageMemoryRequirements2KHR")]
    pub unsafe extern "C" fn get_image_memory_requirements_2_khr(
        self,
        p_info: *const ImageMemoryRequirementsInfo2,
        p_memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements2KHR.html>
    #[doc(alias = "vkGetBufferMemoryRequirements2KHR")]
    pub unsafe extern "C" fn get_buffer_memory_requirements_2_khr(
        self,
        p_info: *const BufferMemoryRequirementsInfo2,
        p_memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2KHR.html>
    #[doc(alias = "vkGetImageSparseMemoryRequirements2KHR")]
    pub unsafe extern "C" fn get_image_sparse_memory_requirements_2_khr(
        self,
        p_info: *const ImageSparseMemoryRequirementsInfo2,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSamplerYcbcrConversionKHR.html>
    #[doc(alias = "vkCreateSamplerYcbcrConversionKHR")]
    pub unsafe extern "C" fn create_sampler_ycbcr_conversion_khr(
        self,
        p_create_info: *const SamplerYcbcrConversionCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_ycbcr_conversion: *mut SamplerYcbcrConversion,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversionKHR.html>
    #[doc(alias = "vkDestroySamplerYcbcrConversionKHR")]
    pub unsafe extern "C" fn destroy_sampler_ycbcr_conversion_khr(
        self,
        ycbcr_conversion: SamplerYcbcrConversion,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory2KHR.html>
    #[doc(alias = "vkBindBufferMemory2KHR")]
    pub unsafe extern "C" fn bind_buffer_memory_2_khr(
        self,
        bind_info_count: u32,
        p_bind_infos: *const BindBufferMemoryInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory2KHR.html>
    #[doc(alias = "vkBindImageMemory2KHR")]
    pub unsafe extern "C" fn bind_image_memory_2_khr(
        self,
        bind_info_count: u32,
        p_bind_infos: *const BindImageMemoryInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSupportKHR.html>
    #[doc(alias = "vkGetDescriptorSetLayoutSupportKHR")]
    pub unsafe extern "C" fn get_descriptor_set_layout_support_khr(
        self,
        p_create_info: *const DescriptorSetLayoutCreateInfo,
        p_support: *mut DescriptorSetLayoutSupport,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCountKHR.html>
    #[doc(alias = "vkCmdDrawIndirectCountKHR")]
    pub unsafe extern "C" fn cmd_draw_indirect_count_khr(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCountKHR.html>
    #[doc(alias = "vkCmdDrawIndexedIndirectCountKHR")]
    pub unsafe extern "C" fn cmd_draw_indexed_indirect_count_khr(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValueKHR.html>
    #[doc(alias = "vkGetSemaphoreCounterValueKHR")]
    pub unsafe extern "C" fn get_semaphore_counter_value_khr(
        self,
        semaphore: Semaphore,
        p_value: *mut u64,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitSemaphoresKHR.html>
    #[doc(alias = "vkWaitSemaphoresKHR")]
    pub unsafe extern "C" fn wait_semaphores_khr(
        self,
        p_wait_info: *const SemaphoreWaitInfo,
        timeout: u64,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSignalSemaphoreKHR.html>
    #[doc(alias = "vkSignalSemaphoreKHR")]
    pub unsafe extern "C" fn signal_semaphore_khr(
        self,
        p_signal_info: *const SemaphoreSignalInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFragmentShadingRatesKHR.html>
    #[doc(alias = "vkGetPhysicalDeviceFragmentShadingRatesKHR")]
    pub unsafe extern "C" fn get_physical_device_fragment_shading_rates_khr(
        self,
        p_fragment_shading_rate_count: *mut u32,
        p_fragment_shading_rates: *mut PhysicalDeviceFragmentShadingRateKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFragmentShadingRateKHR.html>
    #[doc(alias = "vkCmdSetFragmentShadingRateKHR")]
    pub unsafe extern "C" fn cmd_set_fragment_shading_rate_khr(
        self,
        p_fragment_size: *const Extent2D,
        combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2 as usize],
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocationsKHR.html>
    #[doc(alias = "vkCmdSetRenderingAttachmentLocationsKHR")]
    pub unsafe extern "C" fn cmd_set_rendering_attachment_locations_khr(
        self,
        p_location_info: *const RenderingAttachmentLocationInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndicesKHR.html>
    #[doc(alias = "vkCmdSetRenderingInputAttachmentIndicesKHR")]
    pub unsafe extern "C" fn cmd_set_rendering_input_attachment_indices_khr(
        self,
        p_input_attachment_index_info: *const RenderingInputAttachmentIndexInfo,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresentKHR.html>
    #[doc(alias = "vkWaitForPresentKHR")]
    pub unsafe extern "C" fn wait_for_present_khr(
        self,
        swapchain: SwapchainKHR,
        present_id: u64,
        timeout: u64,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddressKHR.html>
    #[doc(alias = "vkGetBufferDeviceAddressKHR")]
    pub unsafe extern "C" fn get_buffer_device_address_khr(
        self,
        p_info: *const BufferDeviceAddressInfo,
    ) -> DeviceAddress {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureAddressKHR.html>
    #[doc(alias = "vkGetBufferOpaqueCaptureAddressKHR")]
    pub unsafe extern "C" fn get_buffer_opaque_capture_address_khr(
        self,
        p_info: *const BufferDeviceAddressInfo,
    ) -> u64 {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryOpaqueCaptureAddressKHR.html>
    #[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddressKHR")]
    pub unsafe extern "C" fn get_device_memory_opaque_capture_address_khr(
        self,
        p_info: *const DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> u64 {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDeferredOperationKHR.html>
    #[doc(alias = "vkCreateDeferredOperationKHR")]
    pub unsafe extern "C" fn create_deferred_operation_khr(
        self,
        p_allocator: *const AllocationCallbacks,
        p_deferred_operation: *mut DeferredOperationKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDeferredOperationKHR.html>
    #[doc(alias = "vkDestroyDeferredOperationKHR")]
    pub unsafe extern "C" fn destroy_deferred_operation_khr(
        self,
        operation: DeferredOperationKHR,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeferredOperationMaxConcurrencyKHR.html>
    #[doc(alias = "vkGetDeferredOperationMaxConcurrencyKHR")]
    pub unsafe extern "C" fn get_deferred_operation_max_concurrency_khr(
        self,
        operation: DeferredOperationKHR,
    ) -> u32 {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeferredOperationResultKHR.html>
    #[doc(alias = "vkGetDeferredOperationResultKHR")]
    pub unsafe extern "C" fn get_deferred_operation_result_khr(
        self,
        operation: DeferredOperationKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDeferredOperationJoinKHR.html>
    #[doc(alias = "vkDeferredOperationJoinKHR")]
    pub unsafe extern "C" fn deferred_operation_join_khr(
        self,
        operation: DeferredOperationKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutablePropertiesKHR.html>
    #[doc(alias = "vkGetPipelineExecutablePropertiesKHR")]
    pub unsafe extern "C" fn get_pipeline_executable_properties_khr(
        self,
        p_pipeline_info: *const PipelineInfoKHR,
        p_executable_count: *mut u32,
        p_properties: *mut PipelineExecutablePropertiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutableStatisticsKHR.html>
    #[doc(alias = "vkGetPipelineExecutableStatisticsKHR")]
    pub unsafe extern "C" fn get_pipeline_executable_statistics_khr(
        self,
        p_executable_info: *const PipelineExecutableInfoKHR,
        p_statistic_count: *mut u32,
        p_statistics: *mut PipelineExecutableStatisticKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutableInternalRepresentationsKHR.html>
    #[doc(alias = "vkGetPipelineExecutableInternalRepresentationsKHR")]
    pub unsafe extern "C" fn get_pipeline_executable_internal_representations_khr(
        self,
        p_executable_info: *const PipelineExecutableInfoKHR,
        p_internal_representation_count: *mut u32,
        p_internal_representations: *mut PipelineExecutableInternalRepresentationKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2KHR.html>
    #[doc(alias = "vkMapMemory2KHR")]
    pub unsafe extern "C" fn map_memory_2_khr(
        self,
        p_memory_map_info: *const MemoryMapInfo,
        pp_data: *mut *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2KHR.html>
    #[doc(alias = "vkUnmapMemory2KHR")]
    pub unsafe extern "C" fn unmap_memory_2_khr(
        self,
        p_memory_unmap_info: *const MemoryUnmapInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR.html>
    #[doc(alias = "vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR")]
    pub unsafe extern "C" fn get_physical_device_video_encode_quality_level_properties_khr(
        self,
        p_quality_level_info: *const PhysicalDeviceVideoEncodeQualityLevelInfoKHR,
        p_quality_level_properties: *mut VideoEncodeQualityLevelPropertiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetEncodedVideoSessionParametersKHR.html>
    #[doc(alias = "vkGetEncodedVideoSessionParametersKHR")]
    pub unsafe extern "C" fn get_encoded_video_session_parameters_khr(
        self,
        p_video_session_parameters_info: *const VideoEncodeSessionParametersGetInfoKHR,
        p_feedback_info: *mut VideoEncodeSessionParametersFeedbackInfoKHR,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEncodeVideoKHR.html>
    #[doc(alias = "vkCmdEncodeVideoKHR")]
    pub unsafe extern "C" fn cmd_encode_video_khr(self, p_encode_info: *const VideoEncodeInfoKHR) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent2KHR.html>
    #[doc(alias = "vkCmdSetEvent2KHR")]
    pub unsafe extern "C" fn cmd_set_event_2_khr(
        self,
        event: Event,
        p_dependency_info: *const DependencyInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent2KHR.html>
    #[doc(alias = "vkCmdResetEvent2KHR")]
    pub unsafe extern "C" fn cmd_reset_event_2_khr(
        self,
        event: Event,
        stage_mask: PipelineStageFlags2,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents2KHR.html>
    #[doc(alias = "vkCmdWaitEvents2KHR")]
    pub unsafe extern "C" fn cmd_wait_events_2_khr(
        self,
        event_count: u32,
        p_events: *const Event,
        p_dependency_infos: *const DependencyInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier2KHR.html>
    #[doc(alias = "vkCmdPipelineBarrier2KHR")]
    pub unsafe extern "C" fn cmd_pipeline_barrier_2_khr(
        self,
        p_dependency_info: *const DependencyInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp2KHR.html>
    #[doc(alias = "vkCmdWriteTimestamp2KHR")]
    pub unsafe extern "C" fn cmd_write_timestamp_2_khr(
        self,
        stage: PipelineStageFlags2,
        query_pool: QueryPool,
        query: u32,
    ) {
        todo!()
    }
}

impl Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2KHR.html>
    #[doc(alias = "vkQueueSubmit2KHR")]
    pub unsafe extern "C" fn queue_submit_2_khr(
        self,
        submit_count: u32,
        p_submits: *const SubmitInfo2,
        fence: Fence,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer3KHR.html>
    #[doc(alias = "vkCmdBindIndexBuffer3KHR")]
    pub unsafe extern "C" fn cmd_bind_index_buffer_3_khr(
        self,
        p_info: *const BindIndexBuffer3InfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers3KHR.html>
    #[doc(alias = "vkCmdBindVertexBuffers3KHR")]
    pub unsafe extern "C" fn cmd_bind_vertex_buffers_3_khr(
        self,
        first_binding: u32,
        binding_count: u32,
        p_binding_infos: *const BindVertexBuffer3InfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirect2KHR.html>
    #[doc(alias = "vkCmdDrawIndirect2KHR")]
    pub unsafe extern "C" fn cmd_draw_indirect_2_khr(self, p_info: *const DrawIndirect2InfoKHR) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirect2KHR.html>
    #[doc(alias = "vkCmdDrawIndexedIndirect2KHR")]
    pub unsafe extern "C" fn cmd_draw_indexed_indirect_2_khr(
        self,
        p_info: *const DrawIndirect2InfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchIndirect2KHR.html>
    #[doc(alias = "vkCmdDispatchIndirect2KHR")]
    pub unsafe extern "C" fn cmd_dispatch_indirect_2_khr(
        self,
        p_info: *const DispatchIndirect2InfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryKHR.html>
    #[doc(alias = "vkCmdCopyMemoryKHR")]
    pub unsafe extern "C" fn cmd_copy_memory_khr(
        self,
        p_copy_memory_info: *const CopyDeviceMemoryInfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageKHR.html>
    #[doc(alias = "vkCmdCopyMemoryToImageKHR")]
    pub unsafe extern "C" fn cmd_copy_memory_to_image_khr(
        self,
        p_copy_memory_info: *const CopyDeviceMemoryImageInfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToMemoryKHR.html>
    #[doc(alias = "vkCmdCopyImageToMemoryKHR")]
    pub unsafe extern "C" fn cmd_copy_image_to_memory_khr(
        self,
        p_copy_memory_info: *const CopyDeviceMemoryImageInfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdateMemoryKHR.html>
    #[doc(alias = "vkCmdUpdateMemoryKHR")]
    pub unsafe extern "C" fn cmd_update_memory_khr(
        self,
        p_dst_range: *const DeviceAddressRangeKHR,
        dst_flags: AddressCommandFlagsKHR,
        data_size: DeviceSize,
        p_data: *const c_void,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdFillMemoryKHR.html>
    #[doc(alias = "vkCmdFillMemoryKHR")]
    pub unsafe extern "C" fn cmd_fill_memory_khr(
        self,
        p_dst_range: *const DeviceAddressRangeKHR,
        dst_flags: AddressCommandFlagsKHR,
        data: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyQueryPoolResultsToMemoryKHR.html>
    #[doc(alias = "vkCmdCopyQueryPoolResultsToMemoryKHR")]
    pub unsafe extern "C" fn cmd_copy_query_pool_results_to_memory_khr(
        self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        p_dst_range: *const StridedDeviceAddressRangeKHR,
        dst_flags: AddressCommandFlagsKHR,
        query_result_flags: QueryResultFlags,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount2KHR.html>
    #[doc(alias = "vkCmdDrawIndirectCount2KHR")]
    pub unsafe extern "C" fn cmd_draw_indirect_count_2_khr(
        self,
        p_info: *const DrawIndirectCount2InfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount2KHR.html>
    #[doc(alias = "vkCmdDrawIndexedIndirectCount2KHR")]
    pub unsafe extern "C" fn cmd_draw_indexed_indirect_count_2_khr(
        self,
        p_info: *const DrawIndirectCount2InfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginConditionalRendering2EXT.html>
    #[doc(alias = "vkCmdBeginConditionalRendering2EXT")]
    pub unsafe extern "C" fn cmd_begin_conditional_rendering_2_ext(
        self,
        p_conditional_rendering_begin: *const ConditionalRenderingBeginInfo2EXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTransformFeedbackBuffers2EXT.html>
    #[doc(alias = "vkCmdBindTransformFeedbackBuffers2EXT")]
    pub unsafe extern "C" fn cmd_bind_transform_feedback_buffers_2_ext(
        self,
        first_binding: u32,
        binding_count: u32,
        p_binding_infos: *const BindTransformFeedbackBuffer2InfoEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginTransformFeedback2EXT.html>
    #[doc(alias = "vkCmdBeginTransformFeedback2EXT")]
    pub unsafe extern "C" fn cmd_begin_transform_feedback_2_ext(
        self,
        first_counter_range: u32,
        counter_range_count: u32,
        p_counter_infos: *const BindTransformFeedbackBuffer2InfoEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndTransformFeedback2EXT.html>
    #[doc(alias = "vkCmdEndTransformFeedback2EXT")]
    pub unsafe extern "C" fn cmd_end_transform_feedback_2_ext(
        self,
        first_counter_range: u32,
        counter_range_count: u32,
        p_counter_infos: *const BindTransformFeedbackBuffer2InfoEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectByteCount2EXT.html>
    #[doc(alias = "vkCmdDrawIndirectByteCount2EXT")]
    pub unsafe extern "C" fn cmd_draw_indirect_byte_count_2_ext(
        self,
        instance_count: u32,
        first_instance: u32,
        p_counter_info: *const BindTransformFeedbackBuffer2InfoEXT,
        counter_offset: u32,
        vertex_stride: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirect2EXT.html>
    #[doc(alias = "vkCmdDrawMeshTasksIndirect2EXT")]
    pub unsafe extern "C" fn cmd_draw_mesh_tasks_indirect_2_ext(
        self,
        p_info: *const DrawIndirect2InfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCount2EXT.html>
    #[doc(alias = "vkCmdDrawMeshTasksIndirectCount2EXT")]
    pub unsafe extern "C" fn cmd_draw_mesh_tasks_indirect_count_2_ext(
        self,
        p_info: *const DrawIndirectCount2InfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteMarkerToMemoryAMD.html>
    #[doc(alias = "vkCmdWriteMarkerToMemoryAMD")]
    pub unsafe extern "C" fn cmd_write_marker_to_memory_amd(
        self,
        p_info: *const MemoryMarkerInfoAMD,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructure2KHR.html>
    #[doc(alias = "vkCreateAccelerationStructure2KHR")]
    pub unsafe extern "C" fn create_acceleration_structure_2_khr(
        self,
        p_create_info: *const AccelerationStructureCreateInfo2KHR,
        p_allocator: *const AllocationCallbacks,
        p_acceleration_structure: *mut AccelerationStructureKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2KHR.html>
    #[doc(alias = "vkCmdCopyBuffer2KHR")]
    pub unsafe extern "C" fn cmd_copy_buffer_2_khr(
        self,
        p_copy_buffer_info: *const CopyBufferInfo2,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2KHR.html>
    #[doc(alias = "vkCmdCopyImage2KHR")]
    pub unsafe extern "C" fn cmd_copy_image_2_khr(self, p_copy_image_info: *const CopyImageInfo2) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2KHR.html>
    #[doc(alias = "vkCmdCopyBufferToImage2KHR")]
    pub unsafe extern "C" fn cmd_copy_buffer_to_image_2_khr(
        self,
        p_copy_buffer_to_image_info: *const CopyBufferToImageInfo2,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2KHR.html>
    #[doc(alias = "vkCmdCopyImageToBuffer2KHR")]
    pub unsafe extern "C" fn cmd_copy_image_to_buffer_2_khr(
        self,
        p_copy_image_to_buffer_info: *const CopyImageToBufferInfo2,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2KHR.html>
    #[doc(alias = "vkCmdBlitImage2KHR")]
    pub unsafe extern "C" fn cmd_blit_image_2_khr(self, p_blit_image_info: *const BlitImageInfo2) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage2KHR.html>
    #[doc(alias = "vkCmdResolveImage2KHR")]
    pub unsafe extern "C" fn cmd_resolve_image_2_khr(
        self,
        p_resolve_image_info: *const ResolveImageInfo2,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysIndirect2KHR.html>
    #[doc(alias = "vkCmdTraceRaysIndirect2KHR")]
    pub unsafe extern "C" fn cmd_trace_rays_indirect_2_khr(
        self,
        indirect_device_address: DeviceAddress,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceBufferMemoryRequirementsKHR.html>
    #[doc(alias = "vkGetDeviceBufferMemoryRequirementsKHR")]
    pub unsafe extern "C" fn get_device_buffer_memory_requirements_khr(
        self,
        p_info: *const DeviceBufferMemoryRequirements,
        p_memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageMemoryRequirementsKHR.html>
    #[doc(alias = "vkGetDeviceImageMemoryRequirementsKHR")]
    pub unsafe extern "C" fn get_device_image_memory_requirements_khr(
        self,
        p_info: *const DeviceImageMemoryRequirements,
        p_memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirementsKHR.html>
    #[doc(alias = "vkGetDeviceImageSparseMemoryRequirementsKHR")]
    pub unsafe extern "C" fn get_device_image_sparse_memory_requirements_khr(
        self,
        p_info: *const DeviceImageMemoryRequirements,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer2KHR.html>
    #[doc(alias = "vkCmdBindIndexBuffer2KHR")]
    pub unsafe extern "C" fn cmd_bind_index_buffer_2_khr(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderingAreaGranularityKHR.html>
    #[doc(alias = "vkGetRenderingAreaGranularityKHR")]
    pub unsafe extern "C" fn get_rendering_area_granularity_khr(
        self,
        p_rendering_area_info: *const RenderingAreaInfo,
        p_granularity: *mut Extent2D,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSubresourceLayoutKHR.html>
    #[doc(alias = "vkGetDeviceImageSubresourceLayoutKHR")]
    pub unsafe extern "C" fn get_device_image_subresource_layout_khr(
        self,
        p_info: *const DeviceImageSubresourceInfo,
        p_layout: *mut SubresourceLayout2,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2KHR.html>
    #[doc(alias = "vkGetImageSubresourceLayout2KHR")]
    pub unsafe extern "C" fn get_image_subresource_layout_2_khr(
        self,
        image: Image,
        p_subresource: *const ImageSubresource2,
        p_layout: *mut SubresourceLayout2,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresent2KHR.html>
    #[doc(alias = "vkWaitForPresent2KHR")]
    pub unsafe extern "C" fn wait_for_present_2_khr(
        self,
        swapchain: SwapchainKHR,
        p_present_wait_2_info: *const PresentWait2InfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineBinariesKHR.html>
    #[doc(alias = "vkCreatePipelineBinariesKHR")]
    pub unsafe extern "C" fn create_pipeline_binaries_khr(
        self,
        p_create_info: *const PipelineBinaryCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_binaries: *mut PipelineBinaryHandlesInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineBinaryKHR.html>
    #[doc(alias = "vkDestroyPipelineBinaryKHR")]
    pub unsafe extern "C" fn destroy_pipeline_binary_khr(
        self,
        pipeline_binary: PipelineBinaryKHR,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineKeyKHR.html>
    #[doc(alias = "vkGetPipelineKeyKHR")]
    pub unsafe extern "C" fn get_pipeline_key_khr(
        self,
        p_pipeline_create_info: *const PipelineCreateInfoKHR,
        p_pipeline_key: *mut PipelineBinaryKeyKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineBinaryDataKHR.html>
    #[doc(alias = "vkGetPipelineBinaryDataKHR")]
    pub unsafe extern "C" fn get_pipeline_binary_data_khr(
        self,
        p_info: *const PipelineBinaryDataInfoKHR,
        p_pipeline_binary_key: *mut PipelineBinaryKeyKHR,
        p_pipeline_binary_data_size: *mut usize,
        p_pipeline_binary_data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseCapturedPipelineDataKHR.html>
    #[doc(alias = "vkReleaseCapturedPipelineDataKHR")]
    pub unsafe extern "C" fn release_captured_pipeline_data_khr(
        self,
        p_info: *const ReleaseCapturedPipelineDataInfoKHR,
        p_allocator: *const AllocationCallbacks,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseSwapchainImagesKHR.html>
    #[doc(alias = "vkReleaseSwapchainImagesKHR")]
    pub unsafe extern "C" fn release_swapchain_images_khr(
        self,
        p_release_info: *const ReleaseSwapchainImagesInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR.html>
    #[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR")]
    pub unsafe extern "C" fn get_physical_device_cooperative_matrix_properties_khr(
        self,
        p_property_count: *mut u32,
        p_properties: *mut CooperativeMatrixPropertiesKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleKHR.html>
    #[doc(alias = "vkCmdSetLineStippleKHR")]
    pub unsafe extern "C" fn cmd_set_line_stipple_khr(
        self,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCalibrateableTimeDomainsKHR.html>
    #[doc(alias = "vkGetPhysicalDeviceCalibrateableTimeDomainsKHR")]
    pub unsafe extern "C" fn get_physical_device_calibrateable_time_domains_khr(
        self,
        p_time_domain_count: *mut u32,
        p_time_domains: *mut TimeDomainKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCalibratedTimestampsKHR.html>
    #[doc(alias = "vkGetCalibratedTimestampsKHR")]
    pub unsafe extern "C" fn get_calibrated_timestamps_khr(
        self,
        timestamp_count: u32,
        p_timestamp_infos: *const CalibratedTimestampInfoKHR,
        p_timestamps: *mut u64,
        p_max_deviation: *mut u64,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2KHR.html>
    #[doc(alias = "vkCmdBindDescriptorSets2KHR")]
    pub unsafe extern "C" fn cmd_bind_descriptor_sets_2_khr(
        self,
        p_bind_descriptor_sets_info: *const BindDescriptorSetsInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2KHR.html>
    #[doc(alias = "vkCmdPushConstants2KHR")]
    pub unsafe extern "C" fn cmd_push_constants_2_khr(
        self,
        p_push_constants_info: *const PushConstantsInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2KHR.html>
    #[doc(alias = "vkCmdPushDescriptorSet2KHR")]
    pub unsafe extern "C" fn cmd_push_descriptor_set_2_khr(
        self,
        p_push_descriptor_set_info: *const PushDescriptorSetInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate2KHR.html>
    #[doc(alias = "vkCmdPushDescriptorSetWithTemplate2KHR")]
    pub unsafe extern "C" fn cmd_push_descriptor_set_with_template_2_khr(
        self,
        p_push_descriptor_set_with_template_info: *const PushDescriptorSetWithTemplateInfo,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDescriptorBufferOffsets2EXT.html>
    #[doc(alias = "vkCmdSetDescriptorBufferOffsets2EXT")]
    pub unsafe extern "C" fn cmd_set_descriptor_buffer_offsets_2_ext(
        self,
        p_set_descriptor_buffer_offsets_info: *const SetDescriptorBufferOffsetsInfoEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBufferEmbeddedSamplers2EXT.html>
    #[doc(alias = "vkCmdBindDescriptorBufferEmbeddedSamplers2EXT")]
    pub unsafe extern "C" fn cmd_bind_descriptor_buffer_embedded_samplers_2_ext(
        self,
        p_bind_descriptor_buffer_embedded_samplers_info: *const BindDescriptorBufferEmbeddedSamplersInfoEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryIndirectKHR.html>
    #[doc(alias = "vkCmdCopyMemoryIndirectKHR")]
    pub unsafe extern "C" fn cmd_copy_memory_indirect_khr(
        self,
        p_copy_memory_indirect_info: *const CopyMemoryIndirectInfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageIndirectKHR.html>
    #[doc(alias = "vkCmdCopyMemoryToImageIndirectKHR")]
    pub unsafe extern "C" fn cmd_copy_memory_to_image_indirect_khr(
        self,
        p_copy_memory_to_image_indirect_info: *const CopyMemoryToImageIndirectInfoKHR,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultReportsKHR.html>
    #[doc(alias = "vkGetDeviceFaultReportsKHR")]
    pub unsafe extern "C" fn get_device_fault_reports_khr(
        self,
        timeout: u64,
        p_fault_counts: *mut u32,
        p_fault_info: *mut DeviceFaultInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultDebugInfoKHR.html>
    #[doc(alias = "vkGetDeviceFaultDebugInfoKHR")]
    pub unsafe extern "C" fn get_device_fault_debug_info_khr(
        self,
        p_debug_info: *mut DeviceFaultDebugInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering2KHR.html>
    #[doc(alias = "vkCmdEndRendering2KHR")]
    pub unsafe extern "C" fn cmd_end_rendering_2_khr(
        self,
        p_rendering_end_info: *const RenderingEndInfoKHR,
    ) {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDebugReportCallbackEXT.html>
    #[doc(alias = "vkCreateDebugReportCallbackEXT")]
    pub unsafe extern "C" fn create_debug_report_callback_ext(
        self,
        p_create_info: *const DebugReportCallbackCreateInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_callback: *mut DebugReportCallbackEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugReportCallbackEXT.html>
    #[doc(alias = "vkDestroyDebugReportCallbackEXT")]
    pub unsafe extern "C" fn destroy_debug_report_callback_ext(
        self,
        callback: DebugReportCallbackEXT,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugReportMessageEXT.html>
    #[doc(alias = "vkDebugReportMessageEXT")]
    pub unsafe extern "C" fn debug_report_message_ext(
        self,
        flags: DebugReportFlagsEXT,
        object_type: DebugReportObjectTypeEXT,
        object: u64,
        location: usize,
        message_code: i32,
        p_layer_prefix: *const c_char,
        p_message: *const c_char,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugMarkerSetObjectTagEXT.html>
    #[doc(alias = "vkDebugMarkerSetObjectTagEXT")]
    pub unsafe extern "C" fn debug_marker_set_object_tag_ext(
        self,
        p_tag_info: *const DebugMarkerObjectTagInfoEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugMarkerSetObjectNameEXT.html>
    #[doc(alias = "vkDebugMarkerSetObjectNameEXT")]
    pub unsafe extern "C" fn debug_marker_set_object_name_ext(
        self,
        p_name_info: *const DebugMarkerObjectNameInfoEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerBeginEXT.html>
    #[doc(alias = "vkCmdDebugMarkerBeginEXT")]
    pub unsafe extern "C" fn cmd_debug_marker_begin_ext(
        self,
        p_marker_info: *const DebugMarkerMarkerInfoEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerEndEXT.html>
    #[doc(alias = "vkCmdDebugMarkerEndEXT")]
    pub unsafe extern "C" fn cmd_debug_marker_end_ext(self) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerInsertEXT.html>
    #[doc(alias = "vkCmdDebugMarkerInsertEXT")]
    pub unsafe extern "C" fn cmd_debug_marker_insert_ext(
        self,
        p_marker_info: *const DebugMarkerMarkerInfoEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTransformFeedbackBuffersEXT.html>
    #[doc(alias = "vkCmdBindTransformFeedbackBuffersEXT")]
    pub unsafe extern "C" fn cmd_bind_transform_feedback_buffers_ext(
        self,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const Buffer,
        p_offsets: *const DeviceSize,
        p_sizes: *const DeviceSize,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginTransformFeedbackEXT.html>
    #[doc(alias = "vkCmdBeginTransformFeedbackEXT")]
    pub unsafe extern "C" fn cmd_begin_transform_feedback_ext(
        self,
        first_counter_buffer: u32,
        counter_buffer_count: u32,
        p_counter_buffers: *const Buffer,
        p_counter_buffer_offsets: *const DeviceSize,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndTransformFeedbackEXT.html>
    #[doc(alias = "vkCmdEndTransformFeedbackEXT")]
    pub unsafe extern "C" fn cmd_end_transform_feedback_ext(
        self,
        first_counter_buffer: u32,
        counter_buffer_count: u32,
        p_counter_buffers: *const Buffer,
        p_counter_buffer_offsets: *const DeviceSize,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginQueryIndexedEXT.html>
    #[doc(alias = "vkCmdBeginQueryIndexedEXT")]
    pub unsafe extern "C" fn cmd_begin_query_indexed_ext(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndQueryIndexedEXT.html>
    #[doc(alias = "vkCmdEndQueryIndexedEXT")]
    pub unsafe extern "C" fn cmd_end_query_indexed_ext(
        self,
        query_pool: QueryPool,
        query: u32,
        index: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectByteCountEXT.html>
    #[doc(alias = "vkCmdDrawIndirectByteCountEXT")]
    pub unsafe extern "C" fn cmd_draw_indirect_byte_count_ext(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCuModuleNVX.html>
    #[doc(alias = "vkCreateCuModuleNVX")]
    pub unsafe extern "C" fn create_cu_module_nvx(
        self,
        p_create_info: *const CuModuleCreateInfoNVX,
        p_allocator: *const AllocationCallbacks,
        p_module: *mut CuModuleNVX,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCuFunctionNVX.html>
    #[doc(alias = "vkCreateCuFunctionNVX")]
    pub unsafe extern "C" fn create_cu_function_nvx(
        self,
        p_create_info: *const CuFunctionCreateInfoNVX,
        p_allocator: *const AllocationCallbacks,
        p_function: *mut CuFunctionNVX,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuModuleNVX.html>
    #[doc(alias = "vkDestroyCuModuleNVX")]
    pub unsafe extern "C" fn destroy_cu_module_nvx(
        self,
        module: CuModuleNVX,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuFunctionNVX.html>
    #[doc(alias = "vkDestroyCuFunctionNVX")]
    pub unsafe extern "C" fn destroy_cu_function_nvx(
        self,
        function: CuFunctionNVX,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCuLaunchKernelNVX.html>
    #[doc(alias = "vkCmdCuLaunchKernelNVX")]
    pub unsafe extern "C" fn cmd_cu_launch_kernel_nvx(self, p_launch_info: *const CuLaunchInfoNVX) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewHandleNVX.html>
    #[doc(alias = "vkGetImageViewHandleNVX")]
    pub unsafe extern "C" fn get_image_view_handle_nvx(
        self,
        p_info: *const ImageViewHandleInfoNVX,
    ) -> u32 {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewHandle64NVX.html>
    #[doc(alias = "vkGetImageViewHandle64NVX")]
    pub unsafe extern "C" fn get_image_view_handle_64_nvx(
        self,
        p_info: *const ImageViewHandleInfoNVX,
    ) -> u64 {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewAddressNVX.html>
    #[doc(alias = "vkGetImageViewAddressNVX")]
    pub unsafe extern "C" fn get_image_view_address_nvx(
        self,
        image_view: ImageView,
        p_properties: *mut ImageViewAddressPropertiesNVX,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceCombinedImageSamplerIndexNVX.html>
    #[doc(alias = "vkGetDeviceCombinedImageSamplerIndexNVX")]
    pub unsafe extern "C" fn get_device_combined_image_sampler_index_nvx(
        self,
        image_view_index: u64,
        sampler_index: u64,
    ) -> u64 {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCountAMD.html>
    #[doc(alias = "vkCmdDrawIndirectCountAMD")]
    pub unsafe extern "C" fn cmd_draw_indirect_count_amd(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCountAMD.html>
    #[doc(alias = "vkCmdDrawIndexedIndirectCountAMD")]
    pub unsafe extern "C" fn cmd_draw_indexed_indirect_count_amd(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderInfoAMD.html>
    #[doc(alias = "vkGetShaderInfoAMD")]
    pub unsafe extern "C" fn get_shader_info_amd(
        self,
        pipeline: Pipeline,
        shader_stage: ShaderStageFlags,
        info_type: ShaderInfoTypeAMD,
        p_info_size: *mut usize,
        p_info: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateStreamDescriptorSurfaceGGP.html>
    #[doc(alias = "vkCreateStreamDescriptorSurfaceGGP")]
    pub unsafe extern "C" fn create_stream_descriptor_surface_ggp(
        self,
        p_create_info: *const StreamDescriptorSurfaceCreateInfoGGP,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html>
    #[doc(alias = "vkGetPhysicalDeviceExternalImageFormatPropertiesNV")]
    pub unsafe extern "C" fn get_physical_device_external_image_format_properties_nv(
        self,
        format: Format,
        type_: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        external_handle_type: ExternalMemoryHandleTypeFlagsNV,
        p_external_image_format_properties: *mut ExternalImageFormatPropertiesNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleNV.html>
    #[doc(alias = "vkGetMemoryWin32HandleNV")]
    pub unsafe extern "C" fn get_memory_win_32_handle_nv(
        self,
        memory: DeviceMemory,
        handle_type: ExternalMemoryHandleTypeFlagsNV,
        p_handle: *mut HANDLE,
    ) -> ResultCode {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateViSurfaceNN.html>
    #[doc(alias = "vkCreateViSurfaceNN")]
    pub unsafe extern "C" fn create_vi_surface_nn(
        self,
        p_create_info: *const ViSurfaceCreateInfoNN,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginConditionalRenderingEXT.html>
    #[doc(alias = "vkCmdBeginConditionalRenderingEXT")]
    pub unsafe extern "C" fn cmd_begin_conditional_rendering_ext(
        self,
        p_conditional_rendering_begin: *const ConditionalRenderingBeginInfoEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndConditionalRenderingEXT.html>
    #[doc(alias = "vkCmdEndConditionalRenderingEXT")]
    pub unsafe extern "C" fn cmd_end_conditional_rendering_ext(self) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWScalingNV.html>
    #[doc(alias = "vkCmdSetViewportWScalingNV")]
    pub unsafe extern "C" fn cmd_set_viewport_w_scaling_nv(
        self,
        first_viewport: u32,
        viewport_count: u32,
        p_viewport_w_scalings: *const ViewportWScalingNV,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseDisplayEXT.html>
    #[doc(alias = "vkReleaseDisplayEXT")]
    pub unsafe extern "C" fn release_display_ext(self, display: DisplayKHR) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireXlibDisplayEXT.html>
    #[doc(alias = "vkAcquireXlibDisplayEXT")]
    pub unsafe extern "C" fn acquire_xlib_display_ext(
        self,
        dpy: *mut Display,
        display: DisplayKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRandROutputDisplayEXT.html>
    #[doc(alias = "vkGetRandROutputDisplayEXT")]
    pub unsafe extern "C" fn get_rand_r_output_display_ext(
        self,
        dpy: *mut Display,
        rr_output: RROutput,
        p_display: *mut DisplayKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html>
    #[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2EXT")]
    pub unsafe extern "C" fn get_physical_device_surface_capabilities_2_ext(
        self,
        surface: SurfaceKHR,
        p_surface_capabilities: *mut SurfaceCapabilities2EXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDisplayPowerControlEXT.html>
    #[doc(alias = "vkDisplayPowerControlEXT")]
    pub unsafe extern "C" fn display_power_control_ext(
        self,
        display: DisplayKHR,
        p_display_power_info: *const DisplayPowerInfoEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterDeviceEventEXT.html>
    #[doc(alias = "vkRegisterDeviceEventEXT")]
    pub unsafe extern "C" fn register_device_event_ext(
        self,
        p_device_event_info: *const DeviceEventInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_fence: *mut Fence,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterDisplayEventEXT.html>
    #[doc(alias = "vkRegisterDisplayEventEXT")]
    pub unsafe extern "C" fn register_display_event_ext(
        self,
        display: DisplayKHR,
        p_display_event_info: *const DisplayEventInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_fence: *mut Fence,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainCounterEXT.html>
    #[doc(alias = "vkGetSwapchainCounterEXT")]
    pub unsafe extern "C" fn get_swapchain_counter_ext(
        self,
        swapchain: SwapchainKHR,
        counter: SurfaceCounterFlagsEXT,
        p_counter_value: *mut u64,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRefreshCycleDurationGOOGLE.html>
    #[doc(alias = "vkGetRefreshCycleDurationGOOGLE")]
    pub unsafe extern "C" fn get_refresh_cycle_duration_google(
        self,
        swapchain: SwapchainKHR,
        p_display_timing_properties: *mut RefreshCycleDurationGOOGLE,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingGOOGLE.html>
    #[doc(alias = "vkGetPastPresentationTimingGOOGLE")]
    pub unsafe extern "C" fn get_past_presentation_timing_google(
        self,
        swapchain: SwapchainKHR,
        p_presentation_timing_count: *mut u32,
        p_presentation_timings: *mut PastPresentationTimingGOOGLE,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleEXT.html>
    #[doc(alias = "vkCmdSetDiscardRectangleEXT")]
    pub unsafe extern "C" fn cmd_set_discard_rectangle_ext(
        self,
        first_discard_rectangle: u32,
        discard_rectangle_count: u32,
        p_discard_rectangles: *const Rect2D,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleEnableEXT.html>
    #[doc(alias = "vkCmdSetDiscardRectangleEnableEXT")]
    pub unsafe extern "C" fn cmd_set_discard_rectangle_enable_ext(
        self,
        discard_rectangle_enable: Bool32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleModeEXT.html>
    #[doc(alias = "vkCmdSetDiscardRectangleModeEXT")]
    pub unsafe extern "C" fn cmd_set_discard_rectangle_mode_ext(
        self,
        discard_rectangle_mode: DiscardRectangleModeEXT,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetHdrMetadataEXT.html>
    #[doc(alias = "vkSetHdrMetadataEXT")]
    pub unsafe extern "C" fn set_hdr_metadata_ext(
        self,
        swapchain_count: u32,
        p_swapchains: *const SwapchainKHR,
        p_metadata: *const HdrMetadataEXT,
    ) {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIOSSurfaceMVK.html>
    #[doc(alias = "vkCreateIOSSurfaceMVK")]
    pub unsafe extern "C" fn create_ios_surface_mvk(
        self,
        p_create_info: *const IOSSurfaceCreateInfoMVK,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMacOSSurfaceMVK.html>
    #[doc(alias = "vkCreateMacOSSurfaceMVK")]
    pub unsafe extern "C" fn create_mac_os_surface_mvk(
        self,
        p_create_info: *const MacOSSurfaceCreateInfoMVK,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDebugUtilsObjectNameEXT.html>
    #[doc(alias = "vkSetDebugUtilsObjectNameEXT")]
    pub unsafe extern "C" fn set_debug_utils_object_name_ext(
        self,
        p_name_info: *const DebugUtilsObjectNameInfoEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDebugUtilsObjectTagEXT.html>
    #[doc(alias = "vkSetDebugUtilsObjectTagEXT")]
    pub unsafe extern "C" fn set_debug_utils_object_tag_ext(
        self,
        p_tag_info: *const DebugUtilsObjectTagInfoEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBeginDebugUtilsLabelEXT.html>
    #[doc(alias = "vkQueueBeginDebugUtilsLabelEXT")]
    pub unsafe extern "C" fn queue_begin_debug_utils_label_ext(
        self,
        p_label_info: *const DebugUtilsLabelEXT,
    ) {
        todo!()
    }
}

impl Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueEndDebugUtilsLabelEXT.html>
    #[doc(alias = "vkQueueEndDebugUtilsLabelEXT")]
    pub unsafe extern "C" fn queue_end_debug_utils_label_ext(self) {
        todo!()
    }
}

impl Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueInsertDebugUtilsLabelEXT.html>
    #[doc(alias = "vkQueueInsertDebugUtilsLabelEXT")]
    pub unsafe extern "C" fn queue_insert_debug_utils_label_ext(
        self,
        p_label_info: *const DebugUtilsLabelEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginDebugUtilsLabelEXT.html>
    #[doc(alias = "vkCmdBeginDebugUtilsLabelEXT")]
    pub unsafe extern "C" fn cmd_begin_debug_utils_label_ext(
        self,
        p_label_info: *const DebugUtilsLabelEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndDebugUtilsLabelEXT.html>
    #[doc(alias = "vkCmdEndDebugUtilsLabelEXT")]
    pub unsafe extern "C" fn cmd_end_debug_utils_label_ext(self) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdInsertDebugUtilsLabelEXT.html>
    #[doc(alias = "vkCmdInsertDebugUtilsLabelEXT")]
    pub unsafe extern "C" fn cmd_insert_debug_utils_label_ext(
        self,
        p_label_info: *const DebugUtilsLabelEXT,
    ) {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDebugUtilsMessengerEXT.html>
    #[doc(alias = "vkCreateDebugUtilsMessengerEXT")]
    pub unsafe extern "C" fn create_debug_utils_messenger_ext(
        self,
        p_create_info: *const DebugUtilsMessengerCreateInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_messenger: *mut DebugUtilsMessengerEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugUtilsMessengerEXT.html>
    #[doc(alias = "vkDestroyDebugUtilsMessengerEXT")]
    pub unsafe extern "C" fn destroy_debug_utils_messenger_ext(
        self,
        messenger: DebugUtilsMessengerEXT,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSubmitDebugUtilsMessageEXT.html>
    #[doc(alias = "vkSubmitDebugUtilsMessageEXT")]
    pub unsafe extern "C" fn submit_debug_utils_message_ext(
        self,
        message_severity: DebugUtilsMessageSeverityFlagsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAndroidHardwareBufferPropertiesANDROID.html>
    #[doc(alias = "vkGetAndroidHardwareBufferPropertiesANDROID")]
    pub unsafe extern "C" fn get_android_hardware_buffer_properties_android(
        self,
        buffer: *const AHardwareBuffer,
        p_properties: *mut AndroidHardwareBufferPropertiesANDROID,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryAndroidHardwareBufferANDROID.html>
    #[doc(alias = "vkGetMemoryAndroidHardwareBufferANDROID")]
    pub unsafe extern "C" fn get_memory_android_hardware_buffer_android(
        self,
        p_info: *const MemoryGetAndroidHardwareBufferInfoANDROID,
        p_buffer: *mut *mut AHardwareBuffer,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateGpaSessionAMD.html>
    #[doc(alias = "vkCreateGpaSessionAMD")]
    pub unsafe extern "C" fn create_gpa_session_amd(
        self,
        p_create_info: *const GpaSessionCreateInfoAMD,
        p_allocator: *const AllocationCallbacks,
        p_gpa_session: *mut GpaSessionAMD,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyGpaSessionAMD.html>
    #[doc(alias = "vkDestroyGpaSessionAMD")]
    pub unsafe extern "C" fn destroy_gpa_session_amd(
        self,
        gpa_session: GpaSessionAMD,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetGpaDeviceClockModeAMD.html>
    #[doc(alias = "vkSetGpaDeviceClockModeAMD")]
    pub unsafe extern "C" fn set_gpa_device_clock_mode_amd(
        self,
        p_info: *mut GpaDeviceClockModeInfoAMD,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaDeviceClockInfoAMD.html>
    #[doc(alias = "vkGetGpaDeviceClockInfoAMD")]
    pub unsafe extern "C" fn get_gpa_device_clock_info_amd(
        self,
        p_info: *mut GpaDeviceGetClockInfoAMD,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginGpaSessionAMD.html>
    #[doc(alias = "vkCmdBeginGpaSessionAMD")]
    pub unsafe extern "C" fn cmd_begin_gpa_session_amd(
        self,
        gpa_session: GpaSessionAMD,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndGpaSessionAMD.html>
    #[doc(alias = "vkCmdEndGpaSessionAMD")]
    pub unsafe extern "C" fn cmd_end_gpa_session_amd(
        self,
        gpa_session: GpaSessionAMD,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginGpaSampleAMD.html>
    #[doc(alias = "vkCmdBeginGpaSampleAMD")]
    pub unsafe extern "C" fn cmd_begin_gpa_sample_amd(
        self,
        gpa_session: GpaSessionAMD,
        p_gpa_sample_begin_info: *const GpaSampleBeginInfoAMD,
        p_sample_id: *mut u32,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndGpaSampleAMD.html>
    #[doc(alias = "vkCmdEndGpaSampleAMD")]
    pub unsafe extern "C" fn cmd_end_gpa_sample_amd(
        self,
        gpa_session: GpaSessionAMD,
        sample_id: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaSessionStatusAMD.html>
    #[doc(alias = "vkGetGpaSessionStatusAMD")]
    pub unsafe extern "C" fn get_gpa_session_status_amd(
        self,
        gpa_session: GpaSessionAMD,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaSessionResultsAMD.html>
    #[doc(alias = "vkGetGpaSessionResultsAMD")]
    pub unsafe extern "C" fn get_gpa_session_results_amd(
        self,
        gpa_session: GpaSessionAMD,
        sample_id: u32,
        p_size_in_bytes: *mut usize,
        p_data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkResetGpaSessionAMD.html>
    #[doc(alias = "vkResetGpaSessionAMD")]
    pub unsafe extern "C" fn reset_gpa_session_amd(self, gpa_session: GpaSessionAMD) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyGpaSessionResultsAMD.html>
    #[doc(alias = "vkCmdCopyGpaSessionResultsAMD")]
    pub unsafe extern "C" fn cmd_copy_gpa_session_results_amd(self, gpa_session: GpaSessionAMD) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateExecutionGraphPipelinesAMDX.html>
    #[doc(alias = "vkCreateExecutionGraphPipelinesAMDX")]
    pub unsafe extern "C" fn create_execution_graph_pipelines_amdx(
        self,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const ExecutionGraphPipelineCreateInfoAMDX,
        p_allocator: *const AllocationCallbacks,
        p_pipelines: *mut Pipeline,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExecutionGraphPipelineScratchSizeAMDX.html>
    #[doc(alias = "vkGetExecutionGraphPipelineScratchSizeAMDX")]
    pub unsafe extern "C" fn get_execution_graph_pipeline_scratch_size_amdx(
        self,
        execution_graph: Pipeline,
        p_size_info: *mut ExecutionGraphPipelineScratchSizeAMDX,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExecutionGraphPipelineNodeIndexAMDX.html>
    #[doc(alias = "vkGetExecutionGraphPipelineNodeIndexAMDX")]
    pub unsafe extern "C" fn get_execution_graph_pipeline_node_index_amdx(
        self,
        execution_graph: Pipeline,
        p_node_info: *const PipelineShaderStageNodeCreateInfoAMDX,
        p_node_index: *mut u32,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdInitializeGraphScratchMemoryAMDX.html>
    #[doc(alias = "vkCmdInitializeGraphScratchMemoryAMDX")]
    pub unsafe extern "C" fn cmd_initialize_graph_scratch_memory_amdx(
        self,
        execution_graph: Pipeline,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphAMDX.html>
    #[doc(alias = "vkCmdDispatchGraphAMDX")]
    pub unsafe extern "C" fn cmd_dispatch_graph_amdx(
        self,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        p_count_info: *const DispatchGraphCountInfoAMDX,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphIndirectAMDX.html>
    #[doc(alias = "vkCmdDispatchGraphIndirectAMDX")]
    pub unsafe extern "C" fn cmd_dispatch_graph_indirect_amdx(
        self,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        p_count_info: *const DispatchGraphCountInfoAMDX,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphIndirectCountAMDX.html>
    #[doc(alias = "vkCmdDispatchGraphIndirectCountAMDX")]
    pub unsafe extern "C" fn cmd_dispatch_graph_indirect_count_amdx(
        self,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: DeviceAddress,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteSamplerDescriptorsEXT.html>
    #[doc(alias = "vkWriteSamplerDescriptorsEXT")]
    pub unsafe extern "C" fn write_sampler_descriptors_ext(
        self,
        sampler_count: u32,
        p_samplers: *const SamplerCreateInfo,
        p_descriptors: *const HostAddressRangeEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteResourceDescriptorsEXT.html>
    #[doc(alias = "vkWriteResourceDescriptorsEXT")]
    pub unsafe extern "C" fn write_resource_descriptors_ext(
        self,
        resource_count: u32,
        p_resources: *const ResourceDescriptorInfoEXT,
        p_descriptors: *const HostAddressRangeEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindSamplerHeapEXT.html>
    #[doc(alias = "vkCmdBindSamplerHeapEXT")]
    pub unsafe extern "C" fn cmd_bind_sampler_heap_ext(self, p_bind_info: *const BindHeapInfoEXT) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindResourceHeapEXT.html>
    #[doc(alias = "vkCmdBindResourceHeapEXT")]
    pub unsafe extern "C" fn cmd_bind_resource_heap_ext(self, p_bind_info: *const BindHeapInfoEXT) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDataEXT.html>
    #[doc(alias = "vkCmdPushDataEXT")]
    pub unsafe extern "C" fn cmd_push_data_ext(self, p_push_data_info: *const PushDataInfoEXT) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageOpaqueCaptureDataEXT.html>
    #[doc(alias = "vkGetImageOpaqueCaptureDataEXT")]
    pub unsafe extern "C" fn get_image_opaque_capture_data_ext(
        self,
        image_count: u32,
        p_images: *const Image,
        p_datas: *mut HostAddressRangeEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDescriptorSizeEXT.html>
    #[doc(alias = "vkGetPhysicalDeviceDescriptorSizeEXT")]
    pub unsafe extern "C" fn get_physical_device_descriptor_size_ext(
        self,
        descriptor_type: DescriptorType,
    ) -> DeviceSize {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterCustomBorderColorEXT.html>
    #[doc(alias = "vkRegisterCustomBorderColorEXT")]
    pub unsafe extern "C" fn register_custom_border_color_ext(
        self,
        p_border_color: *const SamplerCustomBorderColorCreateInfoEXT,
        request_index: Bool32,
        p_index: *mut u32,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUnregisterCustomBorderColorEXT.html>
    #[doc(alias = "vkUnregisterCustomBorderColorEXT")]
    pub unsafe extern "C" fn unregister_custom_border_color_ext(self, index: u32) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDataARM.html>
    #[doc(alias = "vkGetTensorOpaqueCaptureDataARM")]
    pub unsafe extern "C" fn get_tensor_opaque_capture_data_arm(
        self,
        tensor_count: u32,
        p_tensors: *const TensorARM,
        p_datas: *mut HostAddressRangeEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleLocationsEXT.html>
    #[doc(alias = "vkCmdSetSampleLocationsEXT")]
    pub unsafe extern "C" fn cmd_set_sample_locations_ext(
        self,
        p_sample_locations_info: *const SampleLocationsInfoEXT,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMultisamplePropertiesEXT.html>
    #[doc(alias = "vkGetPhysicalDeviceMultisamplePropertiesEXT")]
    pub unsafe extern "C" fn get_physical_device_multisample_properties_ext(
        self,
        samples: SampleCountFlags,
        p_multisample_properties: *mut MultisamplePropertiesEXT,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageDrmFormatModifierPropertiesEXT.html>
    #[doc(alias = "vkGetImageDrmFormatModifierPropertiesEXT")]
    pub unsafe extern "C" fn get_image_drm_format_modifier_properties_ext(
        self,
        image: Image,
        p_properties: *mut ImageDrmFormatModifierPropertiesEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateValidationCacheEXT.html>
    #[doc(alias = "vkCreateValidationCacheEXT")]
    pub unsafe extern "C" fn create_validation_cache_ext(
        self,
        p_create_info: *const ValidationCacheCreateInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_validation_cache: *mut ValidationCacheEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyValidationCacheEXT.html>
    #[doc(alias = "vkDestroyValidationCacheEXT")]
    pub unsafe extern "C" fn destroy_validation_cache_ext(
        self,
        validation_cache: ValidationCacheEXT,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkMergeValidationCachesEXT.html>
    #[doc(alias = "vkMergeValidationCachesEXT")]
    pub unsafe extern "C" fn merge_validation_caches_ext(
        self,
        dst_cache: ValidationCacheEXT,
        src_cache_count: u32,
        p_src_caches: *const ValidationCacheEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetValidationCacheDataEXT.html>
    #[doc(alias = "vkGetValidationCacheDataEXT")]
    pub unsafe extern "C" fn get_validation_cache_data_ext(
        self,
        validation_cache: ValidationCacheEXT,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindShadingRateImageNV.html>
    #[doc(alias = "vkCmdBindShadingRateImageNV")]
    pub unsafe extern "C" fn cmd_bind_shading_rate_image_nv(
        self,
        image_view: ImageView,
        image_layout: ImageLayout,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportShadingRatePaletteNV.html>
    #[doc(alias = "vkCmdSetViewportShadingRatePaletteNV")]
    pub unsafe extern "C" fn cmd_set_viewport_shading_rate_palette_nv(
        self,
        first_viewport: u32,
        viewport_count: u32,
        p_shading_rate_palettes: *const ShadingRatePaletteNV,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoarseSampleOrderNV.html>
    #[doc(alias = "vkCmdSetCoarseSampleOrderNV")]
    pub unsafe extern "C" fn cmd_set_coarse_sample_order_nv(
        self,
        sample_order_type: CoarseSampleOrderTypeNV,
        custom_sample_order_count: u32,
        p_custom_sample_orders: *const CoarseSampleOrderCustomNV,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructureNV.html>
    #[doc(alias = "vkCreateAccelerationStructureNV")]
    pub unsafe extern "C" fn create_acceleration_structure_nv(
        self,
        p_create_info: *const AccelerationStructureCreateInfoNV,
        p_allocator: *const AllocationCallbacks,
        p_acceleration_structure: *mut AccelerationStructureNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureNV.html>
    #[doc(alias = "vkDestroyAccelerationStructureNV")]
    pub unsafe extern "C" fn destroy_acceleration_structure_nv(
        self,
        acceleration_structure: AccelerationStructureNV,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureMemoryRequirementsNV.html>
    #[doc(alias = "vkGetAccelerationStructureMemoryRequirementsNV")]
    pub unsafe extern "C" fn get_acceleration_structure_memory_requirements_nv(
        self,
        p_info: *const AccelerationStructureMemoryRequirementsInfoNV,
        p_memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindAccelerationStructureMemoryNV.html>
    #[doc(alias = "vkBindAccelerationStructureMemoryNV")]
    pub unsafe extern "C" fn bind_acceleration_structure_memory_nv(
        self,
        bind_info_count: u32,
        p_bind_infos: *const BindAccelerationStructureMemoryInfoNV,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructureNV.html>
    #[doc(alias = "vkCmdBuildAccelerationStructureNV")]
    pub unsafe extern "C" fn cmd_build_acceleration_structure_nv(
        self,
        p_info: *const AccelerationStructureInfoNV,
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureNV.html>
    #[doc(alias = "vkCmdCopyAccelerationStructureNV")]
    pub unsafe extern "C" fn cmd_copy_acceleration_structure_nv(
        self,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        mode: CopyAccelerationStructureModeKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysNV.html>
    #[doc(alias = "vkCmdTraceRaysNV")]
    pub unsafe extern "C" fn cmd_trace_rays_nv(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRayTracingPipelinesNV.html>
    #[doc(alias = "vkCreateRayTracingPipelinesNV")]
    pub unsafe extern "C" fn create_ray_tracing_pipelines_nv(
        self,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const RayTracingPipelineCreateInfoNV,
        p_allocator: *const AllocationCallbacks,
        p_pipelines: *mut Pipeline,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupHandlesKHR.html>
    #[doc(alias = "vkGetRayTracingShaderGroupHandlesKHR")]
    pub unsafe extern "C" fn get_ray_tracing_shader_group_handles_khr(
        self,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupHandlesNV.html>
    #[doc(alias = "vkGetRayTracingShaderGroupHandlesNV")]
    pub unsafe extern "C" fn get_ray_tracing_shader_group_handles_nv(
        self,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureHandleNV.html>
    #[doc(alias = "vkGetAccelerationStructureHandleNV")]
    pub unsafe extern "C" fn get_acceleration_structure_handle_nv(
        self,
        acceleration_structure: AccelerationStructureNV,
        data_size: usize,
        p_data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteAccelerationStructuresPropertiesNV.html>
    #[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesNV")]
    pub unsafe extern "C" fn cmd_write_acceleration_structures_properties_nv(
        self,
        acceleration_structure_count: u32,
        p_acceleration_structures: *const AccelerationStructureNV,
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCompileDeferredNV.html>
    #[doc(alias = "vkCompileDeferredNV")]
    pub unsafe extern "C" fn compile_deferred_nv(
        self,
        pipeline: Pipeline,
        shader: u32,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryHostPointerPropertiesEXT.html>
    #[doc(alias = "vkGetMemoryHostPointerPropertiesEXT")]
    pub unsafe extern "C" fn get_memory_host_pointer_properties_ext(
        self,
        handle_type: ExternalMemoryHandleTypeFlags,
        p_host_pointer: *const c_void,
        p_memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteBufferMarkerAMD.html>
    #[doc(alias = "vkCmdWriteBufferMarkerAMD")]
    pub unsafe extern "C" fn cmd_write_buffer_marker_amd(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteBufferMarker2AMD.html>
    #[doc(alias = "vkCmdWriteBufferMarker2AMD")]
    pub unsafe extern "C" fn cmd_write_buffer_marker_2_amd(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html>
    #[doc(alias = "vkGetPhysicalDeviceCalibrateableTimeDomainsEXT")]
    pub unsafe extern "C" fn get_physical_device_calibrateable_time_domains_ext(
        self,
        p_time_domain_count: *mut u32,
        p_time_domains: *mut TimeDomainKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCalibratedTimestampsEXT.html>
    #[doc(alias = "vkGetCalibratedTimestampsEXT")]
    pub unsafe extern "C" fn get_calibrated_timestamps_ext(
        self,
        timestamp_count: u32,
        p_timestamp_infos: *const CalibratedTimestampInfoKHR,
        p_timestamps: *mut u64,
        p_max_deviation: *mut u64,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksNV.html>
    #[doc(alias = "vkCmdDrawMeshTasksNV")]
    pub unsafe extern "C" fn cmd_draw_mesh_tasks_nv(self, task_count: u32, first_task: u32) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectNV.html>
    #[doc(alias = "vkCmdDrawMeshTasksIndirectNV")]
    pub unsafe extern "C" fn cmd_draw_mesh_tasks_indirect_nv(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCountNV.html>
    #[doc(alias = "vkCmdDrawMeshTasksIndirectCountNV")]
    pub unsafe extern "C" fn cmd_draw_mesh_tasks_indirect_count_nv(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExclusiveScissorEnableNV.html>
    #[doc(alias = "vkCmdSetExclusiveScissorEnableNV")]
    pub unsafe extern "C" fn cmd_set_exclusive_scissor_enable_nv(
        self,
        first_exclusive_scissor: u32,
        exclusive_scissor_count: u32,
        p_exclusive_scissor_enables: *const Bool32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExclusiveScissorNV.html>
    #[doc(alias = "vkCmdSetExclusiveScissorNV")]
    pub unsafe extern "C" fn cmd_set_exclusive_scissor_nv(
        self,
        first_exclusive_scissor: u32,
        exclusive_scissor_count: u32,
        p_exclusive_scissors: *const Rect2D,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCheckpointNV.html>
    #[doc(alias = "vkCmdSetCheckpointNV")]
    pub unsafe extern "C" fn cmd_set_checkpoint_nv(self, p_checkpoint_marker: *const c_void) {
        todo!()
    }
}

impl Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointDataNV.html>
    #[doc(alias = "vkGetQueueCheckpointDataNV")]
    pub unsafe extern "C" fn get_queue_checkpoint_data_nv(
        self,
        p_checkpoint_data_count: *mut u32,
        p_checkpoint_data: *mut CheckpointDataNV,
    ) {
        todo!()
    }
}

impl Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointData2NV.html>
    #[doc(alias = "vkGetQueueCheckpointData2NV")]
    pub unsafe extern "C" fn get_queue_checkpoint_data_2_nv(
        self,
        p_checkpoint_data_count: *mut u32,
        p_checkpoint_data: *mut CheckpointData2NV,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetSwapchainPresentTimingQueueSizeEXT.html>
    #[doc(alias = "vkSetSwapchainPresentTimingQueueSizeEXT")]
    pub unsafe extern "C" fn set_swapchain_present_timing_queue_size_ext(
        self,
        swapchain: SwapchainKHR,
        size: u32,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainTimingPropertiesEXT.html>
    #[doc(alias = "vkGetSwapchainTimingPropertiesEXT")]
    pub unsafe extern "C" fn get_swapchain_timing_properties_ext(
        self,
        swapchain: SwapchainKHR,
        p_swapchain_timing_properties: *mut SwapchainTimingPropertiesEXT,
        p_swapchain_timing_properties_counter: *mut u64,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainTimeDomainPropertiesEXT.html>
    #[doc(alias = "vkGetSwapchainTimeDomainPropertiesEXT")]
    pub unsafe extern "C" fn get_swapchain_time_domain_properties_ext(
        self,
        swapchain: SwapchainKHR,
        p_swapchain_time_domain_properties: *mut SwapchainTimeDomainPropertiesEXT,
        p_time_domains_counter: *mut u64,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingEXT.html>
    #[doc(alias = "vkGetPastPresentationTimingEXT")]
    pub unsafe extern "C" fn get_past_presentation_timing_ext(
        self,
        p_past_presentation_timing_info: *const PastPresentationTimingInfoEXT,
        p_past_presentation_timing_properties: *mut PastPresentationTimingPropertiesEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkInitializePerformanceApiINTEL.html>
    #[doc(alias = "vkInitializePerformanceApiINTEL")]
    pub unsafe extern "C" fn initialize_performance_api_intel(
        self,
        p_initialize_info: *const InitializePerformanceApiInfoINTEL,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUninitializePerformanceApiINTEL.html>
    #[doc(alias = "vkUninitializePerformanceApiINTEL")]
    pub unsafe extern "C" fn uninitialize_performance_api_intel(self) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceMarkerINTEL.html>
    #[doc(alias = "vkCmdSetPerformanceMarkerINTEL")]
    pub unsafe extern "C" fn cmd_set_performance_marker_intel(
        self,
        p_marker_info: *const PerformanceMarkerInfoINTEL,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceStreamMarkerINTEL.html>
    #[doc(alias = "vkCmdSetPerformanceStreamMarkerINTEL")]
    pub unsafe extern "C" fn cmd_set_performance_stream_marker_intel(
        self,
        p_marker_info: *const PerformanceStreamMarkerInfoINTEL,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceOverrideINTEL.html>
    #[doc(alias = "vkCmdSetPerformanceOverrideINTEL")]
    pub unsafe extern "C" fn cmd_set_performance_override_intel(
        self,
        p_override_info: *const PerformanceOverrideInfoINTEL,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquirePerformanceConfigurationINTEL.html>
    #[doc(alias = "vkAcquirePerformanceConfigurationINTEL")]
    pub unsafe extern "C" fn acquire_performance_configuration_intel(
        self,
        p_acquire_info: *const PerformanceConfigurationAcquireInfoINTEL,
        p_configuration: *mut PerformanceConfigurationINTEL,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleasePerformanceConfigurationINTEL.html>
    #[doc(alias = "vkReleasePerformanceConfigurationINTEL")]
    pub unsafe extern "C" fn release_performance_configuration_intel(
        self,
        configuration: PerformanceConfigurationINTEL,
    ) -> ResultCode {
        todo!()
    }
}

impl Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSetPerformanceConfigurationINTEL.html>
    #[doc(alias = "vkQueueSetPerformanceConfigurationINTEL")]
    pub unsafe extern "C" fn queue_set_performance_configuration_intel(
        self,
        configuration: PerformanceConfigurationINTEL,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPerformanceParameterINTEL.html>
    #[doc(alias = "vkGetPerformanceParameterINTEL")]
    pub unsafe extern "C" fn get_performance_parameter_intel(
        self,
        parameter: PerformanceParameterTypeINTEL,
        p_value: *mut PerformanceValueINTEL,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLocalDimmingAMD.html>
    #[doc(alias = "vkSetLocalDimmingAMD")]
    pub unsafe extern "C" fn set_local_dimming_amd(
        self,
        swap_chain: SwapchainKHR,
        local_dimming_enable: Bool32,
    ) {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImagePipeSurfaceFUCHSIA.html>
    #[doc(alias = "vkCreateImagePipeSurfaceFUCHSIA")]
    pub unsafe extern "C" fn create_image_pipe_surface_fuchsia(
        self,
        p_create_info: *const ImagePipeSurfaceCreateInfoFUCHSIA,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMetalSurfaceEXT.html>
    #[doc(alias = "vkCreateMetalSurfaceEXT")]
    pub unsafe extern "C" fn create_metal_surface_ext(
        self,
        p_create_info: *const MetalSurfaceCreateInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddressEXT.html>
    #[doc(alias = "vkGetBufferDeviceAddressEXT")]
    pub unsafe extern "C" fn get_buffer_device_address_ext(
        self,
        p_info: *const BufferDeviceAddressInfo,
    ) -> DeviceAddress {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolPropertiesEXT.html>
    #[doc(alias = "vkGetPhysicalDeviceToolPropertiesEXT")]
    pub unsafe extern "C" fn get_physical_device_tool_properties_ext(
        self,
        p_tool_count: *mut u32,
        p_tool_properties: *mut PhysicalDeviceToolProperties,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html>
    #[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixPropertiesNV")]
    pub unsafe extern "C" fn get_physical_device_cooperative_matrix_properties_nv(
        self,
        p_property_count: *mut u32,
        p_properties: *mut CooperativeMatrixPropertiesNV,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html>
    #[doc(alias = "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV")]
    pub unsafe extern "C" fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv(
        self,
        p_combination_count: *mut u32,
        p_combinations: *mut FramebufferMixedSamplesCombinationNV,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfacePresentModes2EXT.html>
    #[doc(alias = "vkGetPhysicalDeviceSurfacePresentModes2EXT")]
    pub unsafe extern "C" fn get_physical_device_surface_present_modes_2_ext(
        self,
        p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
        p_present_mode_count: *mut u32,
        p_present_modes: *mut PresentModeKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireFullScreenExclusiveModeEXT.html>
    #[doc(alias = "vkAcquireFullScreenExclusiveModeEXT")]
    pub unsafe extern "C" fn acquire_full_screen_exclusive_mode_ext(
        self,
        swapchain: SwapchainKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseFullScreenExclusiveModeEXT.html>
    #[doc(alias = "vkReleaseFullScreenExclusiveModeEXT")]
    pub unsafe extern "C" fn release_full_screen_exclusive_mode_ext(
        self,
        swapchain: SwapchainKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModes2EXT.html>
    #[doc(alias = "vkGetDeviceGroupSurfacePresentModes2EXT")]
    pub unsafe extern "C" fn get_device_group_surface_present_modes_2_ext(
        self,
        p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
        p_modes: *mut DeviceGroupPresentModeFlagsKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateHeadlessSurfaceEXT.html>
    #[doc(alias = "vkCreateHeadlessSurfaceEXT")]
    pub unsafe extern "C" fn create_headless_surface_ext(
        self,
        p_create_info: *const HeadlessSurfaceCreateInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleEXT.html>
    #[doc(alias = "vkCmdSetLineStippleEXT")]
    pub unsafe extern "C" fn cmd_set_line_stipple_ext(
        self,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkResetQueryPoolEXT.html>
    #[doc(alias = "vkResetQueryPoolEXT")]
    pub unsafe extern "C" fn reset_query_pool_ext(
        self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCullModeEXT.html>
    #[doc(alias = "vkCmdSetCullModeEXT")]
    pub unsafe extern "C" fn cmd_set_cull_mode_ext(self, cull_mode: CullModeFlags) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFrontFaceEXT.html>
    #[doc(alias = "vkCmdSetFrontFaceEXT")]
    pub unsafe extern "C" fn cmd_set_front_face_ext(self, front_face: FrontFace) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveTopologyEXT.html>
    #[doc(alias = "vkCmdSetPrimitiveTopologyEXT")]
    pub unsafe extern "C" fn cmd_set_primitive_topology_ext(
        self,
        primitive_topology: PrimitiveTopology,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCountEXT.html>
    #[doc(alias = "vkCmdSetViewportWithCountEXT")]
    pub unsafe extern "C" fn cmd_set_viewport_with_count_ext(
        self,
        viewport_count: u32,
        p_viewports: *const Viewport,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCountEXT.html>
    #[doc(alias = "vkCmdSetScissorWithCountEXT")]
    pub unsafe extern "C" fn cmd_set_scissor_with_count_ext(
        self,
        scissor_count: u32,
        p_scissors: *const Rect2D,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2EXT.html>
    #[doc(alias = "vkCmdBindVertexBuffers2EXT")]
    pub unsafe extern "C" fn cmd_bind_vertex_buffers_2_ext(
        self,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const Buffer,
        p_offsets: *const DeviceSize,
        p_sizes: *const DeviceSize,
        p_strides: *const DeviceSize,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthTestEnableEXT.html>
    #[doc(alias = "vkCmdSetDepthTestEnableEXT")]
    pub unsafe extern "C" fn cmd_set_depth_test_enable_ext(self, depth_test_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthWriteEnableEXT.html>
    #[doc(alias = "vkCmdSetDepthWriteEnableEXT")]
    pub unsafe extern "C" fn cmd_set_depth_write_enable_ext(self, depth_write_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthCompareOpEXT.html>
    #[doc(alias = "vkCmdSetDepthCompareOpEXT")]
    pub unsafe extern "C" fn cmd_set_depth_compare_op_ext(self, depth_compare_op: CompareOp) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBoundsTestEnableEXT.html>
    #[doc(alias = "vkCmdSetDepthBoundsTestEnableEXT")]
    pub unsafe extern "C" fn cmd_set_depth_bounds_test_enable_ext(
        self,
        depth_bounds_test_enable: Bool32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilTestEnableEXT.html>
    #[doc(alias = "vkCmdSetStencilTestEnableEXT")]
    pub unsafe extern "C" fn cmd_set_stencil_test_enable_ext(self, stencil_test_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilOpEXT.html>
    #[doc(alias = "vkCmdSetStencilOpEXT")]
    pub unsafe extern "C" fn cmd_set_stencil_op_ext(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToImageEXT.html>
    #[doc(alias = "vkCopyMemoryToImageEXT")]
    pub unsafe extern "C" fn copy_memory_to_image_ext(
        self,
        p_copy_memory_to_image_info: *const CopyMemoryToImageInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToMemoryEXT.html>
    #[doc(alias = "vkCopyImageToMemoryEXT")]
    pub unsafe extern "C" fn copy_image_to_memory_ext(
        self,
        p_copy_image_to_memory_info: *const CopyImageToMemoryInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToImageEXT.html>
    #[doc(alias = "vkCopyImageToImageEXT")]
    pub unsafe extern "C" fn copy_image_to_image_ext(
        self,
        p_copy_image_to_image_info: *const CopyImageToImageInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkTransitionImageLayoutEXT.html>
    #[doc(alias = "vkTransitionImageLayoutEXT")]
    pub unsafe extern "C" fn transition_image_layout_ext(
        self,
        transition_count: u32,
        p_transitions: *const HostImageLayoutTransitionInfo,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2EXT.html>
    #[doc(alias = "vkGetImageSubresourceLayout2EXT")]
    pub unsafe extern "C" fn get_image_subresource_layout_2_ext(
        self,
        image: Image,
        p_subresource: *const ImageSubresource2,
        p_layout: *mut SubresourceLayout2,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseSwapchainImagesEXT.html>
    #[doc(alias = "vkReleaseSwapchainImagesEXT")]
    pub unsafe extern "C" fn release_swapchain_images_ext(
        self,
        p_release_info: *const ReleaseSwapchainImagesInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGeneratedCommandsMemoryRequirementsNV.html>
    #[doc(alias = "vkGetGeneratedCommandsMemoryRequirementsNV")]
    pub unsafe extern "C" fn get_generated_commands_memory_requirements_nv(
        self,
        p_info: *const GeneratedCommandsMemoryRequirementsInfoNV,
        p_memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPreprocessGeneratedCommandsNV.html>
    #[doc(alias = "vkCmdPreprocessGeneratedCommandsNV")]
    pub unsafe extern "C" fn cmd_preprocess_generated_commands_nv(
        self,
        p_generated_commands_info: *const GeneratedCommandsInfoNV,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteGeneratedCommandsNV.html>
    #[doc(alias = "vkCmdExecuteGeneratedCommandsNV")]
    pub unsafe extern "C" fn cmd_execute_generated_commands_nv(
        self,
        is_preprocessed: Bool32,
        p_generated_commands_info: *const GeneratedCommandsInfoNV,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindPipelineShaderGroupNV.html>
    #[doc(alias = "vkCmdBindPipelineShaderGroupNV")]
    pub unsafe extern "C" fn cmd_bind_pipeline_shader_group_nv(
        self,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
        group_index: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectCommandsLayoutNV.html>
    #[doc(alias = "vkCreateIndirectCommandsLayoutNV")]
    pub unsafe extern "C" fn create_indirect_commands_layout_nv(
        self,
        p_create_info: *const IndirectCommandsLayoutCreateInfoNV,
        p_allocator: *const AllocationCallbacks,
        p_indirect_commands_layout: *mut IndirectCommandsLayoutNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectCommandsLayoutNV.html>
    #[doc(alias = "vkDestroyIndirectCommandsLayoutNV")]
    pub unsafe extern "C" fn destroy_indirect_commands_layout_nv(
        self,
        indirect_commands_layout: IndirectCommandsLayoutNV,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias2EXT.html>
    #[doc(alias = "vkCmdSetDepthBias2EXT")]
    pub unsafe extern "C" fn cmd_set_depth_bias_2_ext(
        self,
        p_depth_bias_info: *const DepthBiasInfoEXT,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireDrmDisplayEXT.html>
    #[doc(alias = "vkAcquireDrmDisplayEXT")]
    pub unsafe extern "C" fn acquire_drm_display_ext(
        self,
        drm_fd: i32,
        display: DisplayKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDrmDisplayEXT.html>
    #[doc(alias = "vkGetDrmDisplayEXT")]
    pub unsafe extern "C" fn get_drm_display_ext(
        self,
        drm_fd: i32,
        connector_id: u32,
        display: *mut DisplayKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePrivateDataSlotEXT.html>
    #[doc(alias = "vkCreatePrivateDataSlotEXT")]
    pub unsafe extern "C" fn create_private_data_slot_ext(
        self,
        p_create_info: *const PrivateDataSlotCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_private_data_slot: *mut PrivateDataSlot,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlotEXT.html>
    #[doc(alias = "vkDestroyPrivateDataSlotEXT")]
    pub unsafe extern "C" fn destroy_private_data_slot_ext(
        self,
        private_data_slot: PrivateDataSlot,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetPrivateDataEXT.html>
    #[doc(alias = "vkSetPrivateDataEXT")]
    pub unsafe extern "C" fn set_private_data_ext(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPrivateDataEXT.html>
    #[doc(alias = "vkGetPrivateDataEXT")]
    pub unsafe extern "C" fn get_private_data_ext(
        self,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        p_data: *mut u64,
    ) {
        todo!()
    }
}

impl Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSetPerfHintQCOM.html>
    #[doc(alias = "vkQueueSetPerfHintQCOM")]
    pub unsafe extern "C" fn queue_set_perf_hint_qcom(
        self,
        p_perf_hint_info: *const PerfHintInfoQCOM,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCudaModuleNV.html>
    #[doc(alias = "vkCreateCudaModuleNV")]
    pub unsafe extern "C" fn create_cuda_module_nv(
        self,
        p_create_info: *const CudaModuleCreateInfoNV,
        p_allocator: *const AllocationCallbacks,
        p_module: *mut CudaModuleNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCudaModuleCacheNV.html>
    #[doc(alias = "vkGetCudaModuleCacheNV")]
    pub unsafe extern "C" fn get_cuda_module_cache_nv(
        self,
        module: CudaModuleNV,
        p_cache_size: *mut usize,
        p_cache_data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCudaFunctionNV.html>
    #[doc(alias = "vkCreateCudaFunctionNV")]
    pub unsafe extern "C" fn create_cuda_function_nv(
        self,
        p_create_info: *const CudaFunctionCreateInfoNV,
        p_allocator: *const AllocationCallbacks,
        p_function: *mut CudaFunctionNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCudaModuleNV.html>
    #[doc(alias = "vkDestroyCudaModuleNV")]
    pub unsafe extern "C" fn destroy_cuda_module_nv(
        self,
        module: CudaModuleNV,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCudaFunctionNV.html>
    #[doc(alias = "vkDestroyCudaFunctionNV")]
    pub unsafe extern "C" fn destroy_cuda_function_nv(
        self,
        function: CudaFunctionNV,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCudaLaunchKernelNV.html>
    #[doc(alias = "vkCmdCudaLaunchKernelNV")]
    pub unsafe extern "C" fn cmd_cuda_launch_kernel_nv(
        self,
        p_launch_info: *const CudaLaunchInfoNV,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchTileQCOM.html>
    #[doc(alias = "vkCmdDispatchTileQCOM")]
    pub unsafe extern "C" fn cmd_dispatch_tile_qcom(
        self,
        p_dispatch_tile_info: *const DispatchTileInfoQCOM,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginPerTileExecutionQCOM.html>
    #[doc(alias = "vkCmdBeginPerTileExecutionQCOM")]
    pub unsafe extern "C" fn cmd_begin_per_tile_execution_qcom(
        self,
        p_per_tile_begin_info: *const PerTileBeginInfoQCOM,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndPerTileExecutionQCOM.html>
    #[doc(alias = "vkCmdEndPerTileExecutionQCOM")]
    pub unsafe extern "C" fn cmd_end_per_tile_execution_qcom(
        self,
        p_per_tile_end_info: *const PerTileEndInfoQCOM,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencySleepModeLegacyNV.html>
    #[doc(alias = "vkSetLatencySleepModeLegacyNV")]
    pub unsafe extern "C" fn set_latency_sleep_mode_legacy_nv(
        self,
        low_latency_mode: Bool32,
        low_latency_boost: Bool32,
        minimum_interval_us: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkLatencySleepLegacyNV.html>
    #[doc(alias = "vkLatencySleepLegacyNV")]
    pub unsafe extern "C" fn latency_sleep_legacy_nv(
        self,
        signal_semaphore: Semaphore,
        value: u64,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencyMarkerLegacyNV.html>
    #[doc(alias = "vkSetLatencyMarkerLegacyNV")]
    pub unsafe extern "C" fn set_latency_marker_legacy_nv(self, frame_id: u64, marker: u32) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetLatencyTimingsLegacyNV.html>
    #[doc(alias = "vkGetLatencyTimingsLegacyNV")]
    pub unsafe extern "C" fn get_latency_timings_legacy_nv(self, p_timings: *mut c_void) {
        todo!()
    }
}

impl Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueNotifyOutOfBandLegacyNV.html>
    #[doc(alias = "vkQueueNotifyOutOfBandLegacyNV")]
    pub unsafe extern "C" fn queue_notify_out_of_band_legacy_nv(self, queue_type: u32) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSleepStatusLegacyNV.html>
    #[doc(alias = "vkGetSleepStatusLegacyNV")]
    pub unsafe extern "C" fn get_sleep_status_legacy_nv(self, p_low_latency_mode: *mut Bool32) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkShutdownLatencyDeviceLegacyNV.html>
    #[doc(alias = "vkShutdownLatencyDeviceLegacyNV")]
    pub unsafe extern "C" fn shutdown_latency_device_legacy_nv(self) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkExportMetalObjectsEXT.html>
    #[doc(alias = "vkExportMetalObjectsEXT")]
    pub unsafe extern "C" fn export_metal_objects_ext(
        self,
        p_metal_objects_info: *mut ExportMetalObjectsInfoEXT,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSizeEXT.html>
    #[doc(alias = "vkGetDescriptorSetLayoutSizeEXT")]
    pub unsafe extern "C" fn get_descriptor_set_layout_size_ext(
        self,
        layout: DescriptorSetLayout,
        p_layout_size_in_bytes: *mut DeviceSize,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutBindingOffsetEXT.html>
    #[doc(alias = "vkGetDescriptorSetLayoutBindingOffsetEXT")]
    pub unsafe extern "C" fn get_descriptor_set_layout_binding_offset_ext(
        self,
        layout: DescriptorSetLayout,
        binding: u32,
        p_offset: *mut DeviceSize,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorEXT.html>
    #[doc(alias = "vkGetDescriptorEXT")]
    pub unsafe extern "C" fn get_descriptor_ext(
        self,
        p_descriptor_info: *const DescriptorGetInfoEXT,
        data_size: usize,
        p_descriptor: *mut c_void,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBuffersEXT.html>
    #[doc(alias = "vkCmdBindDescriptorBuffersEXT")]
    pub unsafe extern "C" fn cmd_bind_descriptor_buffers_ext(
        self,
        buffer_count: u32,
        p_binding_infos: *const DescriptorBufferBindingInfoEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDescriptorBufferOffsetsEXT.html>
    #[doc(alias = "vkCmdSetDescriptorBufferOffsetsEXT")]
    pub unsafe extern "C" fn cmd_set_descriptor_buffer_offsets_ext(
        self,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        set_count: u32,
        p_buffer_indices: *const u32,
        p_offsets: *const DeviceSize,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBufferEmbeddedSamplersEXT.html>
    #[doc(alias = "vkCmdBindDescriptorBufferEmbeddedSamplersEXT")]
    pub unsafe extern "C" fn cmd_bind_descriptor_buffer_embedded_samplers_ext(
        self,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureDescriptorDataEXT.html>
    #[doc(alias = "vkGetBufferOpaqueCaptureDescriptorDataEXT")]
    pub unsafe extern "C" fn get_buffer_opaque_capture_descriptor_data_ext(
        self,
        p_info: *const BufferCaptureDescriptorDataInfoEXT,
        p_data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageOpaqueCaptureDescriptorDataEXT.html>
    #[doc(alias = "vkGetImageOpaqueCaptureDescriptorDataEXT")]
    pub unsafe extern "C" fn get_image_opaque_capture_descriptor_data_ext(
        self,
        p_info: *const ImageCaptureDescriptorDataInfoEXT,
        p_data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewOpaqueCaptureDescriptorDataEXT.html>
    #[doc(alias = "vkGetImageViewOpaqueCaptureDescriptorDataEXT")]
    pub unsafe extern "C" fn get_image_view_opaque_capture_descriptor_data_ext(
        self,
        p_info: *const ImageViewCaptureDescriptorDataInfoEXT,
        p_data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSamplerOpaqueCaptureDescriptorDataEXT.html>
    #[doc(alias = "vkGetSamplerOpaqueCaptureDescriptorDataEXT")]
    pub unsafe extern "C" fn get_sampler_opaque_capture_descriptor_data_ext(
        self,
        p_info: *const SamplerCaptureDescriptorDataInfoEXT,
        p_data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT.html>
    #[doc(alias = "vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT")]
    pub unsafe extern "C" fn get_acceleration_structure_opaque_capture_descriptor_data_ext(
        self,
        p_info: *const AccelerationStructureCaptureDescriptorDataInfoEXT,
        p_data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFragmentShadingRateEnumNV.html>
    #[doc(alias = "vkCmdSetFragmentShadingRateEnumNV")]
    pub unsafe extern "C" fn cmd_set_fragment_shading_rate_enum_nv(
        self,
        shading_rate: FragmentShadingRateNV,
        combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2 as usize],
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultInfoEXT.html>
    #[doc(alias = "vkGetDeviceFaultInfoEXT")]
    pub unsafe extern "C" fn get_device_fault_info_ext(
        self,
        p_fault_counts: *mut DeviceFaultCountsEXT,
        p_fault_info: *mut DeviceFaultInfoEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireWinrtDisplayNV.html>
    #[doc(alias = "vkAcquireWinrtDisplayNV")]
    pub unsafe extern "C" fn acquire_winrt_display_nv(self, display: DisplayKHR) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetWinrtDisplayNV.html>
    #[doc(alias = "vkGetWinrtDisplayNV")]
    pub unsafe extern "C" fn get_winrt_display_nv(
        self,
        device_relative_id: u32,
        p_display: *mut DisplayKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDirectFBSurfaceEXT.html>
    #[doc(alias = "vkCreateDirectFBSurfaceEXT")]
    pub unsafe extern "C" fn create_direct_fb_surface_ext(
        self,
        p_create_info: *const DirectFBSurfaceCreateInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html>
    #[doc(alias = "vkGetPhysicalDeviceDirectFBPresentationSupportEXT")]
    pub unsafe extern "C" fn get_physical_device_direct_fb_presentation_support_ext(
        self,
        queue_family_index: u32,
        dfb: *mut IDirectFB,
    ) -> Bool32 {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetVertexInputEXT.html>
    #[doc(alias = "vkCmdSetVertexInputEXT")]
    pub unsafe extern "C" fn cmd_set_vertex_input_ext(
        self,
        vertex_binding_description_count: u32,
        p_vertex_binding_descriptions: *const VertexInputBindingDescription2EXT,
        vertex_attribute_description_count: u32,
        p_vertex_attribute_descriptions: *const VertexInputAttributeDescription2EXT,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryZirconHandleFUCHSIA.html>
    #[doc(alias = "vkGetMemoryZirconHandleFUCHSIA")]
    pub unsafe extern "C" fn get_memory_zircon_handle_fuchsia(
        self,
        p_get_zircon_handle_info: *const MemoryGetZirconHandleInfoFUCHSIA,
        p_zircon_handle: *mut zx_handle_t,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryZirconHandlePropertiesFUCHSIA.html>
    #[doc(alias = "vkGetMemoryZirconHandlePropertiesFUCHSIA")]
    pub unsafe extern "C" fn get_memory_zircon_handle_properties_fuchsia(
        self,
        handle_type: ExternalMemoryHandleTypeFlags,
        zircon_handle: zx_handle_t,
        p_memory_zircon_handle_properties: *mut MemoryZirconHandlePropertiesFUCHSIA,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreZirconHandleFUCHSIA.html>
    #[doc(alias = "vkImportSemaphoreZirconHandleFUCHSIA")]
    pub unsafe extern "C" fn import_semaphore_zircon_handle_fuchsia(
        self,
        p_import_semaphore_zircon_handle_info: *const ImportSemaphoreZirconHandleInfoFUCHSIA,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreZirconHandleFUCHSIA.html>
    #[doc(alias = "vkGetSemaphoreZirconHandleFUCHSIA")]
    pub unsafe extern "C" fn get_semaphore_zircon_handle_fuchsia(
        self,
        p_get_zircon_handle_info: *const SemaphoreGetZirconHandleInfoFUCHSIA,
        p_zircon_handle: *mut zx_handle_t,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBufferCollectionFUCHSIA.html>
    #[doc(alias = "vkCreateBufferCollectionFUCHSIA")]
    pub unsafe extern "C" fn create_buffer_collection_fuchsia(
        self,
        p_create_info: *const BufferCollectionCreateInfoFUCHSIA,
        p_allocator: *const AllocationCallbacks,
        p_collection: *mut BufferCollectionFUCHSIA,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionImageConstraintsFUCHSIA.html>
    #[doc(alias = "vkSetBufferCollectionImageConstraintsFUCHSIA")]
    pub unsafe extern "C" fn set_buffer_collection_image_constraints_fuchsia(
        self,
        collection: BufferCollectionFUCHSIA,
        p_image_constraints_info: *const ImageConstraintsInfoFUCHSIA,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionBufferConstraintsFUCHSIA.html>
    #[doc(alias = "vkSetBufferCollectionBufferConstraintsFUCHSIA")]
    pub unsafe extern "C" fn set_buffer_collection_buffer_constraints_fuchsia(
        self,
        collection: BufferCollectionFUCHSIA,
        p_buffer_constraints_info: *const BufferConstraintsInfoFUCHSIA,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferCollectionFUCHSIA.html>
    #[doc(alias = "vkDestroyBufferCollectionFUCHSIA")]
    pub unsafe extern "C" fn destroy_buffer_collection_fuchsia(
        self,
        collection: BufferCollectionFUCHSIA,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferCollectionPropertiesFUCHSIA.html>
    #[doc(alias = "vkGetBufferCollectionPropertiesFUCHSIA")]
    pub unsafe extern "C" fn get_buffer_collection_properties_fuchsia(
        self,
        collection: BufferCollectionFUCHSIA,
        p_properties: *mut BufferCollectionPropertiesFUCHSIA,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html>
    #[doc(alias = "vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI")]
    pub unsafe extern "C" fn get_device_subpass_shading_max_workgroup_size_huawei(
        self,
        renderpass: RenderPass,
        p_max_workgroup_size: *mut Extent2D,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSubpassShadingHUAWEI.html>
    #[doc(alias = "vkCmdSubpassShadingHUAWEI")]
    pub unsafe extern "C" fn cmd_subpass_shading_huawei(self) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindInvocationMaskHUAWEI.html>
    #[doc(alias = "vkCmdBindInvocationMaskHUAWEI")]
    pub unsafe extern "C" fn cmd_bind_invocation_mask_huawei(
        self,
        image_view: ImageView,
        image_layout: ImageLayout,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryRemoteAddressNV.html>
    #[doc(alias = "vkGetMemoryRemoteAddressNV")]
    pub unsafe extern "C" fn get_memory_remote_address_nv(
        self,
        p_memory_get_remote_address_info: *const MemoryGetRemoteAddressInfoNV,
        p_address: *mut RemoteAddressNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelinePropertiesEXT.html>
    #[doc(alias = "vkGetPipelinePropertiesEXT")]
    pub unsafe extern "C" fn get_pipeline_properties_ext(
        self,
        p_pipeline_info: *const PipelineInfoKHR,
        p_pipeline_properties: *mut BaseOutStructure,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPatchControlPointsEXT.html>
    #[doc(alias = "vkCmdSetPatchControlPointsEXT")]
    pub unsafe extern "C" fn cmd_set_patch_control_points_ext(self, patch_control_points: u32) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizerDiscardEnableEXT.html>
    #[doc(alias = "vkCmdSetRasterizerDiscardEnableEXT")]
    pub unsafe extern "C" fn cmd_set_rasterizer_discard_enable_ext(
        self,
        rasterizer_discard_enable: Bool32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBiasEnableEXT.html>
    #[doc(alias = "vkCmdSetDepthBiasEnableEXT")]
    pub unsafe extern "C" fn cmd_set_depth_bias_enable_ext(self, depth_bias_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLogicOpEXT.html>
    #[doc(alias = "vkCmdSetLogicOpEXT")]
    pub unsafe extern "C" fn cmd_set_logic_op_ext(self, logic_op: LogicOp) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartEnableEXT.html>
    #[doc(alias = "vkCmdSetPrimitiveRestartEnableEXT")]
    pub unsafe extern "C" fn cmd_set_primitive_restart_enable_ext(
        self,
        primitive_restart_enable: Bool32,
    ) {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateScreenSurfaceQNX.html>
    #[doc(alias = "vkCreateScreenSurfaceQNX")]
    pub unsafe extern "C" fn create_screen_surface_qnx(
        self,
        p_create_info: *const ScreenSurfaceCreateInfoQNX,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceScreenPresentationSupportQNX.html>
    #[doc(alias = "vkGetPhysicalDeviceScreenPresentationSupportQNX")]
    pub unsafe extern "C" fn get_physical_device_screen_presentation_support_qnx(
        self,
        queue_family_index: u32,
        window: *mut _screen_window,
    ) -> Bool32 {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorWriteEnableEXT.html>
    #[doc(alias = "vkCmdSetColorWriteEnableEXT")]
    pub unsafe extern "C" fn cmd_set_color_write_enable_ext(
        self,
        attachment_count: u32,
        p_color_write_enables: *const Bool32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMultiEXT.html>
    #[doc(alias = "vkCmdDrawMultiEXT")]
    pub unsafe extern "C" fn cmd_draw_multi_ext(
        self,
        draw_count: u32,
        p_vertex_info: *const MultiDrawInfoEXT,
        instance_count: u32,
        first_instance: u32,
        stride: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMultiIndexedEXT.html>
    #[doc(alias = "vkCmdDrawMultiIndexedEXT")]
    pub unsafe extern "C" fn cmd_draw_multi_indexed_ext(
        self,
        draw_count: u32,
        p_index_info: *const MultiDrawIndexedInfoEXT,
        instance_count: u32,
        first_instance: u32,
        stride: u32,
        p_vertex_offset: *const i32,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMicromapEXT.html>
    #[doc(alias = "vkCreateMicromapEXT")]
    pub unsafe extern "C" fn create_micromap_ext(
        self,
        p_create_info: *const MicromapCreateInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_micromap: *mut MicromapEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyMicromapEXT.html>
    #[doc(alias = "vkDestroyMicromapEXT")]
    pub unsafe extern "C" fn destroy_micromap_ext(
        self,
        micromap: MicromapEXT,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildMicromapsEXT.html>
    #[doc(alias = "vkCmdBuildMicromapsEXT")]
    pub unsafe extern "C" fn cmd_build_micromaps_ext(
        self,
        info_count: u32,
        p_infos: *const MicromapBuildInfoEXT,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBuildMicromapsEXT.html>
    #[doc(alias = "vkBuildMicromapsEXT")]
    pub unsafe extern "C" fn build_micromaps_ext(
        self,
        deferred_operation: DeferredOperationKHR,
        info_count: u32,
        p_infos: *const MicromapBuildInfoEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMicromapEXT.html>
    #[doc(alias = "vkCopyMicromapEXT")]
    pub unsafe extern "C" fn copy_micromap_ext(
        self,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyMicromapInfoEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMicromapToMemoryEXT.html>
    #[doc(alias = "vkCopyMicromapToMemoryEXT")]
    pub unsafe extern "C" fn copy_micromap_to_memory_ext(
        self,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyMicromapToMemoryInfoEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToMicromapEXT.html>
    #[doc(alias = "vkCopyMemoryToMicromapEXT")]
    pub unsafe extern "C" fn copy_memory_to_micromap_ext(
        self,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyMemoryToMicromapInfoEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteMicromapsPropertiesEXT.html>
    #[doc(alias = "vkWriteMicromapsPropertiesEXT")]
    pub unsafe extern "C" fn write_micromaps_properties_ext(
        self,
        micromap_count: u32,
        p_micromaps: *const MicromapEXT,
        query_type: QueryType,
        data_size: usize,
        p_data: *mut c_void,
        stride: usize,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMicromapEXT.html>
    #[doc(alias = "vkCmdCopyMicromapEXT")]
    pub unsafe extern "C" fn cmd_copy_micromap_ext(self, p_info: *const CopyMicromapInfoEXT) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMicromapToMemoryEXT.html>
    #[doc(alias = "vkCmdCopyMicromapToMemoryEXT")]
    pub unsafe extern "C" fn cmd_copy_micromap_to_memory_ext(
        self,
        p_info: *const CopyMicromapToMemoryInfoEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToMicromapEXT.html>
    #[doc(alias = "vkCmdCopyMemoryToMicromapEXT")]
    pub unsafe extern "C" fn cmd_copy_memory_to_micromap_ext(
        self,
        p_info: *const CopyMemoryToMicromapInfoEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteMicromapsPropertiesEXT.html>
    #[doc(alias = "vkCmdWriteMicromapsPropertiesEXT")]
    pub unsafe extern "C" fn cmd_write_micromaps_properties_ext(
        self,
        micromap_count: u32,
        p_micromaps: *const MicromapEXT,
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMicromapCompatibilityEXT.html>
    #[doc(alias = "vkGetDeviceMicromapCompatibilityEXT")]
    pub unsafe extern "C" fn get_device_micromap_compatibility_ext(
        self,
        p_version_info: *const MicromapVersionInfoEXT,
        p_compatibility: *mut AccelerationStructureCompatibilityKHR,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMicromapBuildSizesEXT.html>
    #[doc(alias = "vkGetMicromapBuildSizesEXT")]
    pub unsafe extern "C" fn get_micromap_build_sizes_ext(
        self,
        build_type: AccelerationStructureBuildTypeKHR,
        p_build_info: *const MicromapBuildInfoEXT,
        p_size_info: *mut MicromapBuildSizesInfoEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawClusterHUAWEI.html>
    #[doc(alias = "vkCmdDrawClusterHUAWEI")]
    pub unsafe extern "C" fn cmd_draw_cluster_huawei(
        self,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawClusterIndirectHUAWEI.html>
    #[doc(alias = "vkCmdDrawClusterIndirectHUAWEI")]
    pub unsafe extern "C" fn cmd_draw_cluster_indirect_huawei(
        self,
        buffer: Buffer,
        offset: DeviceSize,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDeviceMemoryPriorityEXT.html>
    #[doc(alias = "vkSetDeviceMemoryPriorityEXT")]
    pub unsafe extern "C" fn set_device_memory_priority_ext(
        self,
        memory: DeviceMemory,
        priority: f32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDispatchParametersARM.html>
    #[doc(alias = "vkCmdSetDispatchParametersARM")]
    pub unsafe extern "C" fn cmd_set_dispatch_parameters_arm(
        self,
        p_dispatch_parameters: *const DispatchParametersARM,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutHostMappingInfoVALVE.html>
    #[doc(alias = "vkGetDescriptorSetLayoutHostMappingInfoVALVE")]
    pub unsafe extern "C" fn get_descriptor_set_layout_host_mapping_info_valve(
        self,
        p_binding_reference: *const DescriptorSetBindingReferenceVALVE,
        p_host_mapping: *mut DescriptorSetLayoutHostMappingInfoVALVE,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetHostMappingVALVE.html>
    #[doc(alias = "vkGetDescriptorSetHostMappingVALVE")]
    pub unsafe extern "C" fn get_descriptor_set_host_mapping_valve(
        self,
        descriptor_set: DescriptorSet,
        pp_data: *mut *mut c_void,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryIndirectNV.html>
    #[doc(alias = "vkCmdCopyMemoryIndirectNV")]
    pub unsafe extern "C" fn cmd_copy_memory_indirect_nv(
        self,
        copy_buffer_address: DeviceAddress,
        copy_count: u32,
        stride: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageIndirectNV.html>
    #[doc(alias = "vkCmdCopyMemoryToImageIndirectNV")]
    pub unsafe extern "C" fn cmd_copy_memory_to_image_indirect_nv(
        self,
        copy_buffer_address: DeviceAddress,
        copy_count: u32,
        stride: u32,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        p_image_subresources: *const ImageSubresourceLayers,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryNV.html>
    #[doc(alias = "vkCmdDecompressMemoryNV")]
    pub unsafe extern "C" fn cmd_decompress_memory_nv(
        self,
        decompress_region_count: u32,
        p_decompress_memory_regions: *const DecompressMemoryRegionNV,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryIndirectCountNV.html>
    #[doc(alias = "vkCmdDecompressMemoryIndirectCountNV")]
    pub unsafe extern "C" fn cmd_decompress_memory_indirect_count_nv(
        self,
        indirect_commands_address: DeviceAddress,
        indirect_commands_count_address: DeviceAddress,
        stride: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineIndirectMemoryRequirementsNV.html>
    #[doc(alias = "vkGetPipelineIndirectMemoryRequirementsNV")]
    pub unsafe extern "C" fn get_pipeline_indirect_memory_requirements_nv(
        self,
        p_create_info: *const ComputePipelineCreateInfo,
        p_memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdatePipelineIndirectBufferNV.html>
    #[doc(alias = "vkCmdUpdatePipelineIndirectBufferNV")]
    pub unsafe extern "C" fn cmd_update_pipeline_indirect_buffer_nv(
        self,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineIndirectDeviceAddressNV.html>
    #[doc(alias = "vkGetPipelineIndirectDeviceAddressNV")]
    pub unsafe extern "C" fn get_pipeline_indirect_device_address_nv(
        self,
        p_info: *const PipelineIndirectDeviceAddressInfoNV,
    ) -> DeviceAddress {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetNativeBufferPropertiesOHOS.html>
    #[doc(alias = "vkGetNativeBufferPropertiesOHOS")]
    pub unsafe extern "C" fn get_native_buffer_properties_ohos(
        self,
        buffer: *const OH_NativeBuffer,
        p_properties: *mut NativeBufferPropertiesOHOS,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryNativeBufferOHOS.html>
    #[doc(alias = "vkGetMemoryNativeBufferOHOS")]
    pub unsafe extern "C" fn get_memory_native_buffer_ohos(
        self,
        p_info: *const MemoryGetNativeBufferInfoOHOS,
        p_buffer: *mut *mut OH_NativeBuffer,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClampEnableEXT.html>
    #[doc(alias = "vkCmdSetDepthClampEnableEXT")]
    pub unsafe extern "C" fn cmd_set_depth_clamp_enable_ext(self, depth_clamp_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPolygonModeEXT.html>
    #[doc(alias = "vkCmdSetPolygonModeEXT")]
    pub unsafe extern "C" fn cmd_set_polygon_mode_ext(self, polygon_mode: PolygonMode) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizationSamplesEXT.html>
    #[doc(alias = "vkCmdSetRasterizationSamplesEXT")]
    pub unsafe extern "C" fn cmd_set_rasterization_samples_ext(
        self,
        rasterization_samples: SampleCountFlags,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleMaskEXT.html>
    #[doc(alias = "vkCmdSetSampleMaskEXT")]
    pub unsafe extern "C" fn cmd_set_sample_mask_ext(
        self,
        samples: SampleCountFlags,
        p_sample_mask: *const SampleMask,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAlphaToCoverageEnableEXT.html>
    #[doc(alias = "vkCmdSetAlphaToCoverageEnableEXT")]
    pub unsafe extern "C" fn cmd_set_alpha_to_coverage_enable_ext(
        self,
        alpha_to_coverage_enable: Bool32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAlphaToOneEnableEXT.html>
    #[doc(alias = "vkCmdSetAlphaToOneEnableEXT")]
    pub unsafe extern "C" fn cmd_set_alpha_to_one_enable_ext(self, alpha_to_one_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLogicOpEnableEXT.html>
    #[doc(alias = "vkCmdSetLogicOpEnableEXT")]
    pub unsafe extern "C" fn cmd_set_logic_op_enable_ext(self, logic_op_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendEnableEXT.html>
    #[doc(alias = "vkCmdSetColorBlendEnableEXT")]
    pub unsafe extern "C" fn cmd_set_color_blend_enable_ext(
        self,
        first_attachment: u32,
        attachment_count: u32,
        p_color_blend_enables: *const Bool32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendEquationEXT.html>
    #[doc(alias = "vkCmdSetColorBlendEquationEXT")]
    pub unsafe extern "C" fn cmd_set_color_blend_equation_ext(
        self,
        first_attachment: u32,
        attachment_count: u32,
        p_color_blend_equations: *const ColorBlendEquationEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorWriteMaskEXT.html>
    #[doc(alias = "vkCmdSetColorWriteMaskEXT")]
    pub unsafe extern "C" fn cmd_set_color_write_mask_ext(
        self,
        first_attachment: u32,
        attachment_count: u32,
        p_color_write_masks: *const ColorComponentFlags,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetTessellationDomainOriginEXT.html>
    #[doc(alias = "vkCmdSetTessellationDomainOriginEXT")]
    pub unsafe extern "C" fn cmd_set_tessellation_domain_origin_ext(
        self,
        domain_origin: TessellationDomainOrigin,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizationStreamEXT.html>
    #[doc(alias = "vkCmdSetRasterizationStreamEXT")]
    pub unsafe extern "C" fn cmd_set_rasterization_stream_ext(self, rasterization_stream: u32) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetConservativeRasterizationModeEXT.html>
    #[doc(alias = "vkCmdSetConservativeRasterizationModeEXT")]
    pub unsafe extern "C" fn cmd_set_conservative_rasterization_mode_ext(
        self,
        conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExtraPrimitiveOverestimationSizeEXT.html>
    #[doc(alias = "vkCmdSetExtraPrimitiveOverestimationSizeEXT")]
    pub unsafe extern "C" fn cmd_set_extra_primitive_overestimation_size_ext(
        self,
        extra_primitive_overestimation_size: f32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClipEnableEXT.html>
    #[doc(alias = "vkCmdSetDepthClipEnableEXT")]
    pub unsafe extern "C" fn cmd_set_depth_clip_enable_ext(self, depth_clip_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleLocationsEnableEXT.html>
    #[doc(alias = "vkCmdSetSampleLocationsEnableEXT")]
    pub unsafe extern "C" fn cmd_set_sample_locations_enable_ext(
        self,
        sample_locations_enable: Bool32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendAdvancedEXT.html>
    #[doc(alias = "vkCmdSetColorBlendAdvancedEXT")]
    pub unsafe extern "C" fn cmd_set_color_blend_advanced_ext(
        self,
        first_attachment: u32,
        attachment_count: u32,
        p_color_blend_advanced: *const ColorBlendAdvancedEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetProvokingVertexModeEXT.html>
    #[doc(alias = "vkCmdSetProvokingVertexModeEXT")]
    pub unsafe extern "C" fn cmd_set_provoking_vertex_mode_ext(
        self,
        provoking_vertex_mode: ProvokingVertexModeEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineRasterizationModeEXT.html>
    #[doc(alias = "vkCmdSetLineRasterizationModeEXT")]
    pub unsafe extern "C" fn cmd_set_line_rasterization_mode_ext(
        self,
        line_rasterization_mode: LineRasterizationModeEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleEnableEXT.html>
    #[doc(alias = "vkCmdSetLineStippleEnableEXT")]
    pub unsafe extern "C" fn cmd_set_line_stipple_enable_ext(self, stippled_line_enable: Bool32) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClipNegativeOneToOneEXT.html>
    #[doc(alias = "vkCmdSetDepthClipNegativeOneToOneEXT")]
    pub unsafe extern "C" fn cmd_set_depth_clip_negative_one_to_one_ext(
        self,
        negative_one_to_one: Bool32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWScalingEnableNV.html>
    #[doc(alias = "vkCmdSetViewportWScalingEnableNV")]
    pub unsafe extern "C" fn cmd_set_viewport_w_scaling_enable_nv(
        self,
        viewport_w_scaling_enable: Bool32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportSwizzleNV.html>
    #[doc(alias = "vkCmdSetViewportSwizzleNV")]
    pub unsafe extern "C" fn cmd_set_viewport_swizzle_nv(
        self,
        first_viewport: u32,
        viewport_count: u32,
        p_viewport_swizzles: *const ViewportSwizzleNV,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageToColorEnableNV.html>
    #[doc(alias = "vkCmdSetCoverageToColorEnableNV")]
    pub unsafe extern "C" fn cmd_set_coverage_to_color_enable_nv(
        self,
        coverage_to_color_enable: Bool32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageToColorLocationNV.html>
    #[doc(alias = "vkCmdSetCoverageToColorLocationNV")]
    pub unsafe extern "C" fn cmd_set_coverage_to_color_location_nv(
        self,
        coverage_to_color_location: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationModeNV.html>
    #[doc(alias = "vkCmdSetCoverageModulationModeNV")]
    pub unsafe extern "C" fn cmd_set_coverage_modulation_mode_nv(
        self,
        coverage_modulation_mode: CoverageModulationModeNV,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationTableEnableNV.html>
    #[doc(alias = "vkCmdSetCoverageModulationTableEnableNV")]
    pub unsafe extern "C" fn cmd_set_coverage_modulation_table_enable_nv(
        self,
        coverage_modulation_table_enable: Bool32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationTableNV.html>
    #[doc(alias = "vkCmdSetCoverageModulationTableNV")]
    pub unsafe extern "C" fn cmd_set_coverage_modulation_table_nv(
        self,
        coverage_modulation_table_count: u32,
        p_coverage_modulation_table: *const f32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetShadingRateImageEnableNV.html>
    #[doc(alias = "vkCmdSetShadingRateImageEnableNV")]
    pub unsafe extern "C" fn cmd_set_shading_rate_image_enable_nv(
        self,
        shading_rate_image_enable: Bool32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRepresentativeFragmentTestEnableNV.html>
    #[doc(alias = "vkCmdSetRepresentativeFragmentTestEnableNV")]
    pub unsafe extern "C" fn cmd_set_representative_fragment_test_enable_nv(
        self,
        representative_fragment_test_enable: Bool32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageReductionModeNV.html>
    #[doc(alias = "vkCmdSetCoverageReductionModeNV")]
    pub unsafe extern "C" fn cmd_set_coverage_reduction_mode_nv(
        self,
        coverage_reduction_mode: CoverageReductionModeNV,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateTensorARM.html>
    #[doc(alias = "vkCreateTensorARM")]
    pub unsafe extern "C" fn create_tensor_arm(
        self,
        p_create_info: *const TensorCreateInfoARM,
        p_allocator: *const AllocationCallbacks,
        p_tensor: *mut TensorARM,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyTensorARM.html>
    #[doc(alias = "vkDestroyTensorARM")]
    pub unsafe extern "C" fn destroy_tensor_arm(
        self,
        tensor: TensorARM,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateTensorViewARM.html>
    #[doc(alias = "vkCreateTensorViewARM")]
    pub unsafe extern "C" fn create_tensor_view_arm(
        self,
        p_create_info: *const TensorViewCreateInfoARM,
        p_allocator: *const AllocationCallbacks,
        p_view: *mut TensorViewARM,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyTensorViewARM.html>
    #[doc(alias = "vkDestroyTensorViewARM")]
    pub unsafe extern "C" fn destroy_tensor_view_arm(
        self,
        tensor_view: TensorViewARM,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorMemoryRequirementsARM.html>
    #[doc(alias = "vkGetTensorMemoryRequirementsARM")]
    pub unsafe extern "C" fn get_tensor_memory_requirements_arm(
        self,
        p_info: *const TensorMemoryRequirementsInfoARM,
        p_memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindTensorMemoryARM.html>
    #[doc(alias = "vkBindTensorMemoryARM")]
    pub unsafe extern "C" fn bind_tensor_memory_arm(
        self,
        bind_info_count: u32,
        p_bind_infos: *const BindTensorMemoryInfoARM,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceTensorMemoryRequirementsARM.html>
    #[doc(alias = "vkGetDeviceTensorMemoryRequirementsARM")]
    pub unsafe extern "C" fn get_device_tensor_memory_requirements_arm(
        self,
        p_info: *const DeviceTensorMemoryRequirementsARM,
        p_memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyTensorARM.html>
    #[doc(alias = "vkCmdCopyTensorARM")]
    pub unsafe extern "C" fn cmd_copy_tensor_arm(
        self,
        p_copy_tensor_info: *const CopyTensorInfoARM,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalTensorPropertiesARM.html>
    #[doc(alias = "vkGetPhysicalDeviceExternalTensorPropertiesARM")]
    pub unsafe extern "C" fn get_physical_device_external_tensor_properties_arm(
        self,
        p_external_tensor_info: *const PhysicalDeviceExternalTensorInfoARM,
        p_external_tensor_properties: *mut ExternalTensorPropertiesARM,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDescriptorDataARM.html>
    #[doc(alias = "vkGetTensorOpaqueCaptureDescriptorDataARM")]
    pub unsafe extern "C" fn get_tensor_opaque_capture_descriptor_data_arm(
        self,
        p_info: *const TensorCaptureDescriptorDataInfoARM,
        p_data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorViewOpaqueCaptureDescriptorDataARM.html>
    #[doc(alias = "vkGetTensorViewOpaqueCaptureDescriptorDataARM")]
    pub unsafe extern "C" fn get_tensor_view_opaque_capture_descriptor_data_arm(
        self,
        p_info: *const TensorViewCaptureDescriptorDataInfoARM,
        p_data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleIdentifierEXT.html>
    #[doc(alias = "vkGetShaderModuleIdentifierEXT")]
    pub unsafe extern "C" fn get_shader_module_identifier_ext(
        self,
        shader_module: ShaderModule,
        p_identifier: *mut ShaderModuleIdentifierEXT,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleCreateInfoIdentifierEXT.html>
    #[doc(alias = "vkGetShaderModuleCreateInfoIdentifierEXT")]
    pub unsafe extern "C" fn get_shader_module_create_info_identifier_ext(
        self,
        p_create_info: *const ShaderModuleCreateInfo,
        p_identifier: *mut ShaderModuleIdentifierEXT,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceOpticalFlowImageFormatsNV.html>
    #[doc(alias = "vkGetPhysicalDeviceOpticalFlowImageFormatsNV")]
    pub unsafe extern "C" fn get_physical_device_optical_flow_image_formats_nv(
        self,
        p_optical_flow_image_format_info: *const OpticalFlowImageFormatInfoNV,
        p_format_count: *mut u32,
        p_image_format_properties: *mut OpticalFlowImageFormatPropertiesNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateOpticalFlowSessionNV.html>
    #[doc(alias = "vkCreateOpticalFlowSessionNV")]
    pub unsafe extern "C" fn create_optical_flow_session_nv(
        self,
        p_create_info: *const OpticalFlowSessionCreateInfoNV,
        p_allocator: *const AllocationCallbacks,
        p_session: *mut OpticalFlowSessionNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyOpticalFlowSessionNV.html>
    #[doc(alias = "vkDestroyOpticalFlowSessionNV")]
    pub unsafe extern "C" fn destroy_optical_flow_session_nv(
        self,
        session: OpticalFlowSessionNV,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindOpticalFlowSessionImageNV.html>
    #[doc(alias = "vkBindOpticalFlowSessionImageNV")]
    pub unsafe extern "C" fn bind_optical_flow_session_image_nv(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdOpticalFlowExecuteNV.html>
    #[doc(alias = "vkCmdOpticalFlowExecuteNV")]
    pub unsafe extern "C" fn cmd_optical_flow_execute_nv(
        self,
        session: OpticalFlowSessionNV,
        p_execute_info: *const OpticalFlowExecuteInfoNV,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAntiLagUpdateAMD.html>
    #[doc(alias = "vkAntiLagUpdateAMD")]
    pub unsafe extern "C" fn anti_lag_update_amd(self, p_data: *const AntiLagDataAMD) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShadersEXT.html>
    #[doc(alias = "vkCreateShadersEXT")]
    pub unsafe extern "C" fn create_shaders_ext(
        self,
        create_info_count: u32,
        p_create_infos: *const ShaderCreateInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_shaders: *mut ShaderEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderEXT.html>
    #[doc(alias = "vkDestroyShaderEXT")]
    pub unsafe extern "C" fn destroy_shader_ext(
        self,
        shader: ShaderEXT,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderBinaryDataEXT.html>
    #[doc(alias = "vkGetShaderBinaryDataEXT")]
    pub unsafe extern "C" fn get_shader_binary_data_ext(
        self,
        shader: ShaderEXT,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindShadersEXT.html>
    #[doc(alias = "vkCmdBindShadersEXT")]
    pub unsafe extern "C" fn cmd_bind_shaders_ext(
        self,
        stage_count: u32,
        p_stages: *const ShaderStageFlags,
        p_shaders: *const ShaderEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClampRangeEXT.html>
    #[doc(alias = "vkCmdSetDepthClampRangeEXT")]
    pub unsafe extern "C" fn cmd_set_depth_clamp_range_ext(
        self,
        depth_clamp_mode: DepthClampModeEXT,
        p_depth_clamp_range: *const DepthClampRangeEXT,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFramebufferTilePropertiesQCOM.html>
    #[doc(alias = "vkGetFramebufferTilePropertiesQCOM")]
    pub unsafe extern "C" fn get_framebuffer_tile_properties_qcom(
        self,
        framebuffer: Framebuffer,
        p_properties_count: *mut u32,
        p_properties: *mut TilePropertiesQCOM,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDynamicRenderingTilePropertiesQCOM.html>
    #[doc(alias = "vkGetDynamicRenderingTilePropertiesQCOM")]
    pub unsafe extern "C" fn get_dynamic_rendering_tile_properties_qcom(
        self,
        p_rendering_info: *const RenderingInfo,
        p_properties: *mut TilePropertiesQCOM,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeVectorPropertiesNV.html>
    #[doc(alias = "vkGetPhysicalDeviceCooperativeVectorPropertiesNV")]
    pub unsafe extern "C" fn get_physical_device_cooperative_vector_properties_nv(
        self,
        p_property_count: *mut u32,
        p_properties: *mut CooperativeVectorPropertiesNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkConvertCooperativeVectorMatrixNV.html>
    #[doc(alias = "vkConvertCooperativeVectorMatrixNV")]
    pub unsafe extern "C" fn convert_cooperative_vector_matrix_nv(
        self,
        p_info: *const ConvertCooperativeVectorMatrixInfoNV,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdConvertCooperativeVectorMatrixNV.html>
    #[doc(alias = "vkCmdConvertCooperativeVectorMatrixNV")]
    pub unsafe extern "C" fn cmd_convert_cooperative_vector_matrix_nv(
        self,
        info_count: u32,
        p_infos: *const ConvertCooperativeVectorMatrixInfoNV,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencySleepModeNV.html>
    #[doc(alias = "vkSetLatencySleepModeNV")]
    pub unsafe extern "C" fn set_latency_sleep_mode_nv(
        self,
        swapchain: SwapchainKHR,
        p_sleep_mode_info: *const LatencySleepModeInfoNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkLatencySleepNV.html>
    #[doc(alias = "vkLatencySleepNV")]
    pub unsafe extern "C" fn latency_sleep_nv(
        self,
        swapchain: SwapchainKHR,
        p_sleep_info: *const LatencySleepInfoNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencyMarkerNV.html>
    #[doc(alias = "vkSetLatencyMarkerNV")]
    pub unsafe extern "C" fn set_latency_marker_nv(
        self,
        swapchain: SwapchainKHR,
        p_latency_marker_info: *const SetLatencyMarkerInfoNV,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetLatencyTimingsNV.html>
    #[doc(alias = "vkGetLatencyTimingsNV")]
    pub unsafe extern "C" fn get_latency_timings_nv(
        self,
        swapchain: SwapchainKHR,
        p_latency_marker_info: *mut GetLatencyMarkerInfoNV,
    ) {
        todo!()
    }
}

impl Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueNotifyOutOfBandNV.html>
    #[doc(alias = "vkQueueNotifyOutOfBandNV")]
    pub unsafe extern "C" fn queue_notify_out_of_band_nv(
        self,
        p_queue_type_info: *const OutOfBandQueueTypeInfoNV,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDataGraphPipelinesARM.html>
    #[doc(alias = "vkCreateDataGraphPipelinesARM")]
    pub unsafe extern "C" fn create_data_graph_pipelines_arm(
        self,
        deferred_operation: DeferredOperationKHR,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const DataGraphPipelineCreateInfoARM,
        p_allocator: *const AllocationCallbacks,
        p_pipelines: *mut Pipeline,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDataGraphPipelineSessionARM.html>
    #[doc(alias = "vkCreateDataGraphPipelineSessionARM")]
    pub unsafe extern "C" fn create_data_graph_pipeline_session_arm(
        self,
        p_create_info: *const DataGraphPipelineSessionCreateInfoARM,
        p_allocator: *const AllocationCallbacks,
        p_session: *mut DataGraphPipelineSessionARM,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineSessionBindPointRequirementsARM.html>
    #[doc(alias = "vkGetDataGraphPipelineSessionBindPointRequirementsARM")]
    pub unsafe extern "C" fn get_data_graph_pipeline_session_bind_point_requirements_arm(
        self,
        p_info: *const DataGraphPipelineSessionBindPointRequirementsInfoARM,
        p_bind_point_requirement_count: *mut u32,
        p_bind_point_requirements: *mut DataGraphPipelineSessionBindPointRequirementARM,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineSessionMemoryRequirementsARM.html>
    #[doc(alias = "vkGetDataGraphPipelineSessionMemoryRequirementsARM")]
    pub unsafe extern "C" fn get_data_graph_pipeline_session_memory_requirements_arm(
        self,
        p_info: *const DataGraphPipelineSessionMemoryRequirementsInfoARM,
        p_memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindDataGraphPipelineSessionMemoryARM.html>
    #[doc(alias = "vkBindDataGraphPipelineSessionMemoryARM")]
    pub unsafe extern "C" fn bind_data_graph_pipeline_session_memory_arm(
        self,
        bind_info_count: u32,
        p_bind_infos: *const BindDataGraphPipelineSessionMemoryInfoARM,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDataGraphPipelineSessionARM.html>
    #[doc(alias = "vkDestroyDataGraphPipelineSessionARM")]
    pub unsafe extern "C" fn destroy_data_graph_pipeline_session_arm(
        self,
        session: DataGraphPipelineSessionARM,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchDataGraphARM.html>
    #[doc(alias = "vkCmdDispatchDataGraphARM")]
    pub unsafe extern "C" fn cmd_dispatch_data_graph_arm(
        self,
        session: DataGraphPipelineSessionARM,
        p_info: *const DataGraphPipelineDispatchInfoARM,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineAvailablePropertiesARM.html>
    #[doc(alias = "vkGetDataGraphPipelineAvailablePropertiesARM")]
    pub unsafe extern "C" fn get_data_graph_pipeline_available_properties_arm(
        self,
        p_pipeline_info: *const DataGraphPipelineInfoARM,
        p_properties_count: *mut u32,
        p_properties: *mut DataGraphPipelinePropertyARM,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelinePropertiesARM.html>
    #[doc(alias = "vkGetDataGraphPipelinePropertiesARM")]
    pub unsafe extern "C" fn get_data_graph_pipeline_properties_arm(
        self,
        p_pipeline_info: *const DataGraphPipelineInfoARM,
        properties_count: u32,
        p_properties: *mut DataGraphPipelinePropertyQueryResultARM,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM.html>
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM")]
    pub unsafe extern "C" fn get_physical_device_queue_family_data_graph_properties_arm(
        self,
        queue_family_index: u32,
        p_queue_family_data_graph_property_count: *mut u32,
        p_queue_family_data_graph_properties: *mut QueueFamilyDataGraphPropertiesARM,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM.html>
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM")]
    pub unsafe extern "C" fn get_physical_device_queue_family_data_graph_processing_engine_properties_arm(
        self,
        p_queue_family_data_graph_processing_engine_info: *const PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM,
        p_queue_family_data_graph_processing_engine_properties: *mut QueueFamilyDataGraphProcessingEnginePropertiesARM,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM.html>
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM")]
    pub unsafe extern "C" fn get_physical_device_queue_family_data_graph_engine_operation_properties_arm(
        self,
        queue_family_index: u32,
        p_queue_family_data_graph_properties: *const QueueFamilyDataGraphPropertiesARM,
        p_properties: *mut BaseOutStructure,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAttachmentFeedbackLoopEnableEXT.html>
    #[doc(alias = "vkCmdSetAttachmentFeedbackLoopEnableEXT")]
    pub unsafe extern "C" fn cmd_set_attachment_feedback_loop_enable_ext(
        self,
        aspect_mask: ImageAspectFlags,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetScreenBufferPropertiesQNX.html>
    #[doc(alias = "vkGetScreenBufferPropertiesQNX")]
    pub unsafe extern "C" fn get_screen_buffer_properties_qnx(
        self,
        buffer: *const _screen_buffer,
        p_properties: *mut ScreenBufferPropertiesQNX,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTileMemoryQCOM.html>
    #[doc(alias = "vkCmdBindTileMemoryQCOM")]
    pub unsafe extern "C" fn cmd_bind_tile_memory_qcom(
        self,
        p_tile_memory_bind_info: *const TileMemoryBindInfoQCOM,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryEXT.html>
    #[doc(alias = "vkCmdDecompressMemoryEXT")]
    pub unsafe extern "C" fn cmd_decompress_memory_ext(
        self,
        p_decompress_memory_info_ext: *const DecompressMemoryInfoEXT,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryIndirectCountEXT.html>
    #[doc(alias = "vkCmdDecompressMemoryIndirectCountEXT")]
    pub unsafe extern "C" fn cmd_decompress_memory_indirect_count_ext(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateExternalComputeQueueNV.html>
    #[doc(alias = "vkCreateExternalComputeQueueNV")]
    pub unsafe extern "C" fn create_external_compute_queue_nv(
        self,
        p_create_info: *const ExternalComputeQueueCreateInfoNV,
        p_allocator: *const AllocationCallbacks,
        p_external_queue: *mut ExternalComputeQueueNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyExternalComputeQueueNV.html>
    #[doc(alias = "vkDestroyExternalComputeQueueNV")]
    pub unsafe extern "C" fn destroy_external_compute_queue_nv(
        self,
        external_queue: ExternalComputeQueueNV,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExternalComputeQueueDataNV.html>
#[doc(alias = "vkGetExternalComputeQueueDataNV")]
pub unsafe extern "C" fn get_external_compute_queue_data_nv(
    external_queue: ExternalComputeQueueNV,
    params: *mut ExternalComputeQueueDataParamsNV,
    p_data: *mut c_void,
) {
    todo!()
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetClusterAccelerationStructureBuildSizesNV.html>
    #[doc(alias = "vkGetClusterAccelerationStructureBuildSizesNV")]
    pub unsafe extern "C" fn get_cluster_acceleration_structure_build_sizes_nv(
        self,
        p_info: *const ClusterAccelerationStructureInputInfoNV,
        p_size_info: *mut AccelerationStructureBuildSizesInfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildClusterAccelerationStructureIndirectNV.html>
    #[doc(alias = "vkCmdBuildClusterAccelerationStructureIndirectNV")]
    pub unsafe extern "C" fn cmd_build_cluster_acceleration_structure_indirect_nv(
        self,
        p_command_infos: *const ClusterAccelerationStructureCommandsInfoNV,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPartitionedAccelerationStructuresBuildSizesNV.html>
    #[doc(alias = "vkGetPartitionedAccelerationStructuresBuildSizesNV")]
    pub unsafe extern "C" fn get_partitioned_acceleration_structures_build_sizes_nv(
        self,
        p_info: *const PartitionedAccelerationStructureInstancesInputNV,
        p_size_info: *mut AccelerationStructureBuildSizesInfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildPartitionedAccelerationStructuresNV.html>
    #[doc(alias = "vkCmdBuildPartitionedAccelerationStructuresNV")]
    pub unsafe extern "C" fn cmd_build_partitioned_acceleration_structures_nv(
        self,
        p_build_info: *const BuildPartitionedAccelerationStructureInfoNV,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGeneratedCommandsMemoryRequirementsEXT.html>
    #[doc(alias = "vkGetGeneratedCommandsMemoryRequirementsEXT")]
    pub unsafe extern "C" fn get_generated_commands_memory_requirements_ext(
        self,
        p_info: *const GeneratedCommandsMemoryRequirementsInfoEXT,
        p_memory_requirements: *mut MemoryRequirements2,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPreprocessGeneratedCommandsEXT.html>
    #[doc(alias = "vkCmdPreprocessGeneratedCommandsEXT")]
    pub unsafe extern "C" fn cmd_preprocess_generated_commands_ext(
        self,
        p_generated_commands_info: *const GeneratedCommandsInfoEXT,
        state_command_buffer: CommandBuffer,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteGeneratedCommandsEXT.html>
    #[doc(alias = "vkCmdExecuteGeneratedCommandsEXT")]
    pub unsafe extern "C" fn cmd_execute_generated_commands_ext(
        self,
        is_preprocessed: Bool32,
        p_generated_commands_info: *const GeneratedCommandsInfoEXT,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectCommandsLayoutEXT.html>
    #[doc(alias = "vkCreateIndirectCommandsLayoutEXT")]
    pub unsafe extern "C" fn create_indirect_commands_layout_ext(
        self,
        p_create_info: *const IndirectCommandsLayoutCreateInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_indirect_commands_layout: *mut IndirectCommandsLayoutEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectCommandsLayoutEXT.html>
    #[doc(alias = "vkDestroyIndirectCommandsLayoutEXT")]
    pub unsafe extern "C" fn destroy_indirect_commands_layout_ext(
        self,
        indirect_commands_layout: IndirectCommandsLayoutEXT,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectExecutionSetEXT.html>
    #[doc(alias = "vkCreateIndirectExecutionSetEXT")]
    pub unsafe extern "C" fn create_indirect_execution_set_ext(
        self,
        p_create_info: *const IndirectExecutionSetCreateInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_indirect_execution_set: *mut IndirectExecutionSetEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectExecutionSetEXT.html>
    #[doc(alias = "vkDestroyIndirectExecutionSetEXT")]
    pub unsafe extern "C" fn destroy_indirect_execution_set_ext(
        self,
        indirect_execution_set: IndirectExecutionSetEXT,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateIndirectExecutionSetPipelineEXT.html>
    #[doc(alias = "vkUpdateIndirectExecutionSetPipelineEXT")]
    pub unsafe extern "C" fn update_indirect_execution_set_pipeline_ext(
        self,
        indirect_execution_set: IndirectExecutionSetEXT,
        execution_set_write_count: u32,
        p_execution_set_writes: *const WriteIndirectExecutionSetPipelineEXT,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateIndirectExecutionSetShaderEXT.html>
    #[doc(alias = "vkUpdateIndirectExecutionSetShaderEXT")]
    pub unsafe extern "C" fn update_indirect_execution_set_shader_ext(
        self,
        indirect_execution_set: IndirectExecutionSetEXT,
        execution_set_write_count: u32,
        p_execution_set_writes: *const WriteIndirectExecutionSetShaderEXT,
    ) {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSurfaceOHOS.html>
    #[doc(alias = "vkCreateSurfaceOHOS")]
    pub unsafe extern "C" fn create_surface_ohos(
        self,
        p_create_info: *const SurfaceCreateInfoOHOS,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV.html>
    #[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV")]
    pub unsafe extern "C" fn get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv(
        self,
        p_property_count: *mut u32,
        p_properties: *mut CooperativeMatrixFlexibleDimensionsPropertiesNV,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryMetalHandleEXT.html>
    #[doc(alias = "vkGetMemoryMetalHandleEXT")]
    pub unsafe extern "C" fn get_memory_metal_handle_ext(
        self,
        p_get_metal_handle_info: *const MemoryGetMetalHandleInfoEXT,
        p_handle: *mut *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryMetalHandlePropertiesEXT.html>
    #[doc(alias = "vkGetMemoryMetalHandlePropertiesEXT")]
    pub unsafe extern "C" fn get_memory_metal_handle_properties_ext(
        self,
        handle_type: ExternalMemoryHandleTypeFlags,
        p_handle: *const c_void,
        p_memory_metal_handle_properties: *mut MemoryMetalHandlePropertiesEXT,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM.html>
    #[doc(alias = "vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM")]
    pub unsafe extern "C" fn enumerate_physical_device_queue_family_performance_counters_by_region_arm(
        self,
        queue_family_index: u32,
        p_counter_count: *mut u32,
        p_counters: *mut PerformanceCounterARM,
        p_counter_descriptions: *mut PerformanceCounterDescriptionARM,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM.html>
    #[doc(alias = "vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM")]
    pub unsafe extern "C" fn enumerate_physical_device_shader_instrumentation_metrics_arm(
        self,
        p_description_count: *mut u32,
        p_descriptions: *mut ShaderInstrumentationMetricDescriptionARM,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShaderInstrumentationARM.html>
    #[doc(alias = "vkCreateShaderInstrumentationARM")]
    pub unsafe extern "C" fn create_shader_instrumentation_arm(
        self,
        p_create_info: *const ShaderInstrumentationCreateInfoARM,
        p_allocator: *const AllocationCallbacks,
        p_instrumentation: *mut ShaderInstrumentationARM,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderInstrumentationARM.html>
    #[doc(alias = "vkDestroyShaderInstrumentationARM")]
    pub unsafe extern "C" fn destroy_shader_instrumentation_arm(
        self,
        instrumentation: ShaderInstrumentationARM,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginShaderInstrumentationARM.html>
    #[doc(alias = "vkCmdBeginShaderInstrumentationARM")]
    pub unsafe extern "C" fn cmd_begin_shader_instrumentation_arm(
        self,
        instrumentation: ShaderInstrumentationARM,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndShaderInstrumentationARM.html>
    #[doc(alias = "vkCmdEndShaderInstrumentationARM")]
    pub unsafe extern "C" fn cmd_end_shader_instrumentation_arm(self) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderInstrumentationValuesARM.html>
    #[doc(alias = "vkGetShaderInstrumentationValuesARM")]
    pub unsafe extern "C" fn get_shader_instrumentation_values_arm(
        self,
        instrumentation: ShaderInstrumentationARM,
        p_metric_block_count: *mut u32,
        p_metric_values: *mut c_void,
        flags: ShaderInstrumentationValuesFlagsARM,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkClearShaderInstrumentationMetricsARM.html>
    #[doc(alias = "vkClearShaderInstrumentationMetricsARM")]
    pub unsafe extern "C" fn clear_shader_instrumentation_metrics_arm(
        self,
        instrumentation: ShaderInstrumentationARM,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering2EXT.html>
    #[doc(alias = "vkCmdEndRendering2EXT")]
    pub unsafe extern "C" fn cmd_end_rendering_2_ext(
        self,
        p_rendering_end_info: *const RenderingEndInfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginCustomResolveEXT.html>
    #[doc(alias = "vkCmdBeginCustomResolveEXT")]
    pub unsafe extern "C" fn cmd_begin_custom_resolve_ext(
        self,
        p_begin_custom_resolve_info: *const BeginCustomResolveInfoEXT,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM.html>
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM")]
    pub unsafe extern "C" fn get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm(
        self,
        queue_family_index: u32,
        p_queue_family_data_graph_properties: *const QueueFamilyDataGraphPropertiesARM,
        p_optical_flow_image_format_info: *const DataGraphOpticalFlowImageFormatInfoARM,
        p_format_count: *mut u32,
        p_image_format_properties: *mut DataGraphOpticalFlowImageFormatPropertiesARM,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetComputeOccupancyPriorityNV.html>
    #[doc(alias = "vkCmdSetComputeOccupancyPriorityNV")]
    pub unsafe extern "C" fn cmd_set_compute_occupancy_priority_nv(
        self,
        p_parameters: *const ComputeOccupancyPriorityParametersNV,
    ) {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixProperties2EXT.html>
    #[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixProperties2EXT")]
    pub unsafe extern "C" fn get_physical_device_cooperative_matrix_properties_2_ext(
        self,
        p_cooperative_matrix_info: *const PhysicalDeviceCooperativeMatrixInfo2EXT,
        p_property_count: *mut u32,
        p_properties: *mut CooperativeMatrixProperties2EXT,
    ) -> ResultCode {
        todo!()
    }
}

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateUbmSurfaceSEC.html>
    #[doc(alias = "vkCreateUbmSurfaceSEC")]
    pub unsafe extern "C" fn create_ubm_surface_sec(
        self,
        p_create_info: *const UbmSurfaceCreateInfoSEC,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceUbmPresentationSupportSEC.html>
    #[doc(alias = "vkGetPhysicalDeviceUbmPresentationSupportSEC")]
    pub unsafe extern "C" fn get_physical_device_ubm_presentation_support_sec(
        self,
        queue_family_index: u32,
        device: *mut ubm_device,
    ) -> Bool32 {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartIndexEXT.html>
    #[doc(alias = "vkCmdSetPrimitiveRestartIndexEXT")]
    pub unsafe extern "C" fn cmd_set_primitive_restart_index_ext(
        self,
        primitive_restart_index: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructureKHR.html>
    #[doc(alias = "vkCreateAccelerationStructureKHR")]
    pub unsafe extern "C" fn create_acceleration_structure_khr(
        self,
        p_create_info: *const AccelerationStructureCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_acceleration_structure: *mut AccelerationStructureKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureKHR.html>
    #[doc(alias = "vkDestroyAccelerationStructureKHR")]
    pub unsafe extern "C" fn destroy_acceleration_structure_khr(
        self,
        acceleration_structure: AccelerationStructureKHR,
        p_allocator: *const AllocationCallbacks,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructuresKHR.html>
    #[doc(alias = "vkCmdBuildAccelerationStructuresKHR")]
    pub unsafe extern "C" fn cmd_build_acceleration_structures_khr(
        self,
        info_count: u32,
        p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
        pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructuresIndirectKHR.html>
    #[doc(alias = "vkCmdBuildAccelerationStructuresIndirectKHR")]
    pub unsafe extern "C" fn cmd_build_acceleration_structures_indirect_khr(
        self,
        info_count: u32,
        p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
        p_indirect_device_addresses: *const DeviceAddress,
        p_indirect_strides: *const u32,
        pp_max_primitive_counts: *const *const u32,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBuildAccelerationStructuresKHR.html>
    #[doc(alias = "vkBuildAccelerationStructuresKHR")]
    pub unsafe extern "C" fn build_acceleration_structures_khr(
        self,
        deferred_operation: DeferredOperationKHR,
        info_count: u32,
        p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
        pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyAccelerationStructureKHR.html>
    #[doc(alias = "vkCopyAccelerationStructureKHR")]
    pub unsafe extern "C" fn copy_acceleration_structure_khr(
        self,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyAccelerationStructureInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyAccelerationStructureToMemoryKHR.html>
    #[doc(alias = "vkCopyAccelerationStructureToMemoryKHR")]
    pub unsafe extern "C" fn copy_acceleration_structure_to_memory_khr(
        self,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyAccelerationStructureToMemoryInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToAccelerationStructureKHR.html>
    #[doc(alias = "vkCopyMemoryToAccelerationStructureKHR")]
    pub unsafe extern "C" fn copy_memory_to_acceleration_structure_khr(
        self,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyMemoryToAccelerationStructureInfoKHR,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteAccelerationStructuresPropertiesKHR.html>
    #[doc(alias = "vkWriteAccelerationStructuresPropertiesKHR")]
    pub unsafe extern "C" fn write_acceleration_structures_properties_khr(
        self,
        acceleration_structure_count: u32,
        p_acceleration_structures: *const AccelerationStructureKHR,
        query_type: QueryType,
        data_size: usize,
        p_data: *mut c_void,
        stride: usize,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureKHR.html>
    #[doc(alias = "vkCmdCopyAccelerationStructureKHR")]
    pub unsafe extern "C" fn cmd_copy_acceleration_structure_khr(
        self,
        p_info: *const CopyAccelerationStructureInfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureToMemoryKHR.html>
    #[doc(alias = "vkCmdCopyAccelerationStructureToMemoryKHR")]
    pub unsafe extern "C" fn cmd_copy_acceleration_structure_to_memory_khr(
        self,
        p_info: *const CopyAccelerationStructureToMemoryInfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToAccelerationStructureKHR.html>
    #[doc(alias = "vkCmdCopyMemoryToAccelerationStructureKHR")]
    pub unsafe extern "C" fn cmd_copy_memory_to_acceleration_structure_khr(
        self,
        p_info: *const CopyMemoryToAccelerationStructureInfoKHR,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureDeviceAddressKHR.html>
    #[doc(alias = "vkGetAccelerationStructureDeviceAddressKHR")]
    pub unsafe extern "C" fn get_acceleration_structure_device_address_khr(
        self,
        p_info: *const AccelerationStructureDeviceAddressInfoKHR,
    ) -> DeviceAddress {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteAccelerationStructuresPropertiesKHR.html>
    #[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesKHR")]
    pub unsafe extern "C" fn cmd_write_acceleration_structures_properties_khr(
        self,
        acceleration_structure_count: u32,
        p_acceleration_structures: *const AccelerationStructureKHR,
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceAccelerationStructureCompatibilityKHR.html>
    #[doc(alias = "vkGetDeviceAccelerationStructureCompatibilityKHR")]
    pub unsafe extern "C" fn get_device_acceleration_structure_compatibility_khr(
        self,
        p_version_info: *const AccelerationStructureVersionInfoKHR,
        p_compatibility: *mut AccelerationStructureCompatibilityKHR,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureBuildSizesKHR.html>
    #[doc(alias = "vkGetAccelerationStructureBuildSizesKHR")]
    pub unsafe extern "C" fn get_acceleration_structure_build_sizes_khr(
        self,
        build_type: AccelerationStructureBuildTypeKHR,
        p_build_info: *const AccelerationStructureBuildGeometryInfoKHR,
        p_max_primitive_counts: *const u32,
        p_size_info: *mut AccelerationStructureBuildSizesInfoKHR,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysKHR.html>
    #[doc(alias = "vkCmdTraceRaysKHR")]
    pub unsafe extern "C" fn cmd_trace_rays_khr(
        self,
        p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRayTracingPipelinesKHR.html>
    #[doc(alias = "vkCreateRayTracingPipelinesKHR")]
    pub unsafe extern "C" fn create_ray_tracing_pipelines_khr(
        self,
        deferred_operation: DeferredOperationKHR,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const RayTracingPipelineCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_pipelines: *mut Pipeline,
    ) -> ResultCode {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html>
    #[doc(alias = "vkGetRayTracingCaptureReplayShaderGroupHandlesKHR")]
    pub unsafe extern "C" fn get_ray_tracing_capture_replay_shader_group_handles_khr(
        self,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut c_void,
    ) -> ResultCode {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysIndirectKHR.html>
    #[doc(alias = "vkCmdTraceRaysIndirectKHR")]
    pub unsafe extern "C" fn cmd_trace_rays_indirect_khr(
        self,
        p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        indirect_device_address: DeviceAddress,
    ) {
        todo!()
    }
}

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupStackSizeKHR.html>
    #[doc(alias = "vkGetRayTracingShaderGroupStackSizeKHR")]
    pub unsafe extern "C" fn get_ray_tracing_shader_group_stack_size_khr(
        self,
        pipeline: Pipeline,
        group: u32,
        group_shader: ShaderGroupShaderKHR,
    ) -> DeviceSize {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRayTracingPipelineStackSizeKHR.html>
    #[doc(alias = "vkCmdSetRayTracingPipelineStackSizeKHR")]
    pub unsafe extern "C" fn cmd_set_ray_tracing_pipeline_stack_size_khr(
        self,
        pipeline_stack_size: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksEXT.html>
    #[doc(alias = "vkCmdDrawMeshTasksEXT")]
    pub unsafe extern "C" fn cmd_draw_mesh_tasks_ext(
        self,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectEXT.html>
    #[doc(alias = "vkCmdDrawMeshTasksIndirectEXT")]
    pub unsafe extern "C" fn cmd_draw_mesh_tasks_indirect_ext(
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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCountEXT.html>
    #[doc(alias = "vkCmdDrawMeshTasksIndirectCountEXT")]
    pub unsafe extern "C" fn cmd_draw_mesh_tasks_indirect_count_ext(
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
