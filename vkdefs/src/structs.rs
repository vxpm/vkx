// WARNING: AUTO GENERATED MODULE
#![allow(nonstandard_style)]
#![allow(unused_imports)]

use crate::manual::*;
use crate::platform::*;
use std::ffi::{c_char, c_int, c_uint, c_void};

use crate::bitmasks::*;
use crate::enums::*;
use crate::flags::*;
use crate::fn_ptrs::*;
use crate::handles::*;
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExtent2D.html>
#[doc(alias = "VkExtent2D")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExtent3D.html>
#[doc(alias = "VkExtent3D")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Extent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOffset2D.html>
#[doc(alias = "VkOffset2D")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Offset2D {
    pub x: i32,
    pub y: i32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOffset3D.html>
#[doc(alias = "VkOffset3D")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Offset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRect2D.html>
#[doc(alias = "VkRect2D")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Rect2D {
    pub offset: Offset2D,
    pub extent: Extent2D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBaseInStructure.html>
#[doc(alias = "VkBaseInStructure")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BaseInStructure {
    pub s_type: StructureType,
    pub p_next: *const BaseInStructure,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBaseOutStructure.html>
#[doc(alias = "VkBaseOutStructure")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BaseOutStructure {
    pub s_type: StructureType,
    pub p_next: *mut BaseOutStructure,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAllocationCallbacks.html>
#[doc(alias = "VkAllocationCallbacks")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AllocationCallbacks {
    pub p_user_data: *mut c_void,
    pub pfn_allocation: vkAllocationFunction,
    pub pfn_reallocation: vkReallocationFunction,
    pub pfn_free: vkFreeFunction,
    pub pfn_internal_allocation: vkInternalAllocationNotification,
    pub pfn_internal_free: vkInternalFreeNotification,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkApplicationInfo.html>
#[doc(alias = "VkApplicationInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ApplicationInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_application_name: *const c_char,
    pub application_version: u32,
    pub p_engine_name: *const c_char,
    pub engine_version: u32,
    pub api_version: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatProperties.html>
#[doc(alias = "VkFormatProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FormatProperties {
    pub linear_tiling_features: FormatFeatureFlags,
    pub optimal_tiling_features: FormatFeatureFlags,
    pub buffer_features: FormatFeatureFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageFormatProperties.html>
#[doc(alias = "VkImageFormatProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageFormatProperties {
    pub max_extent: Extent3D,
    pub max_mip_levels: u32,
    pub max_array_layers: u32,
    pub sample_counts: SampleCountFlags,
    pub max_resource_size: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkInstanceCreateInfo.html>
#[doc(alias = "VkInstanceCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct InstanceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: InstanceCreateFlags,
    pub p_application_info: *const ApplicationInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const c_char,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryHeap.html>
#[doc(alias = "VkMemoryHeap")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryHeap {
    pub size: DeviceSize,
    pub flags: MemoryHeapFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryType.html>
#[doc(alias = "VkMemoryType")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryType {
    pub property_flags: MemoryPropertyFlags,
    pub heap_index: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFeatures.html>
#[doc(alias = "VkPhysicalDeviceFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFeatures {
    pub robust_buffer_access: Bool32,
    pub full_draw_index_uint_32: Bool32,
    pub image_cube_array: Bool32,
    pub independent_blend: Bool32,
    pub geometry_shader: Bool32,
    pub tessellation_shader: Bool32,
    pub sample_rate_shading: Bool32,
    pub dual_src_blend: Bool32,
    pub logic_op: Bool32,
    pub multi_draw_indirect: Bool32,
    pub draw_indirect_first_instance: Bool32,
    pub depth_clamp: Bool32,
    pub depth_bias_clamp: Bool32,
    pub fill_mode_non_solid: Bool32,
    pub depth_bounds: Bool32,
    pub wide_lines: Bool32,
    pub large_points: Bool32,
    pub alpha_to_one: Bool32,
    pub multi_viewport: Bool32,
    pub sampler_anisotropy: Bool32,
    pub texture_compression_etc_2: Bool32,
    pub texture_compression_astc_ldr: Bool32,
    pub texture_compression_bc: Bool32,
    pub occlusion_query_precise: Bool32,
    pub pipeline_statistics_query: Bool32,
    pub vertex_pipeline_stores_and_atomics: Bool32,
    pub fragment_stores_and_atomics: Bool32,
    pub shader_tessellation_and_geometry_point_size: Bool32,
    pub shader_image_gather_extended: Bool32,
    pub shader_storage_image_extended_formats: Bool32,
    pub shader_storage_image_multisample: Bool32,
    pub shader_storage_image_read_without_format: Bool32,
    pub shader_storage_image_write_without_format: Bool32,
    pub shader_uniform_buffer_array_dynamic_indexing: Bool32,
    pub shader_sampled_image_array_dynamic_indexing: Bool32,
    pub shader_storage_buffer_array_dynamic_indexing: Bool32,
    pub shader_storage_image_array_dynamic_indexing: Bool32,
    pub shader_clip_distance: Bool32,
    pub shader_cull_distance: Bool32,
    pub shader_float_64: Bool32,
    pub shader_int_64: Bool32,
    pub shader_int_16: Bool32,
    pub shader_resource_residency: Bool32,
    pub shader_resource_min_lod: Bool32,
    pub sparse_binding: Bool32,
    pub sparse_residency_buffer: Bool32,
    pub sparse_residency_image_2_d: Bool32,
    pub sparse_residency_image_3_d: Bool32,
    pub sparse_residency_2_samples: Bool32,
    pub sparse_residency_4_samples: Bool32,
    pub sparse_residency_8_samples: Bool32,
    pub sparse_residency_16_samples: Bool32,
    pub sparse_residency_aliased: Bool32,
    pub variable_multisample_rate: Bool32,
    pub inherited_queries: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceLimits.html>
#[doc(alias = "VkPhysicalDeviceLimits")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceLimits {
    pub max_image_dimension_1_d: u32,
    pub max_image_dimension_2_d: u32,
    pub max_image_dimension_3_d: u32,
    pub max_image_dimension_cube: u32,
    pub max_image_array_layers: u32,
    pub max_texel_buffer_elements: u32,
    pub max_uniform_buffer_range: u32,
    pub max_storage_buffer_range: u32,
    pub max_push_constants_size: u32,
    pub max_memory_allocation_count: u32,
    pub max_sampler_allocation_count: u32,
    pub buffer_image_granularity: DeviceSize,
    pub sparse_address_space_size: DeviceSize,
    pub max_bound_descriptor_sets: u32,
    pub max_per_stage_descriptor_samplers: u32,
    pub max_per_stage_descriptor_uniform_buffers: u32,
    pub max_per_stage_descriptor_storage_buffers: u32,
    pub max_per_stage_descriptor_sampled_images: u32,
    pub max_per_stage_descriptor_storage_images: u32,
    pub max_per_stage_descriptor_input_attachments: u32,
    pub max_per_stage_resources: u32,
    pub max_descriptor_set_samplers: u32,
    pub max_descriptor_set_uniform_buffers: u32,
    pub max_descriptor_set_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_storage_buffers: u32,
    pub max_descriptor_set_storage_buffers_dynamic: u32,
    pub max_descriptor_set_sampled_images: u32,
    pub max_descriptor_set_storage_images: u32,
    pub max_descriptor_set_input_attachments: u32,
    pub max_vertex_input_attributes: u32,
    pub max_vertex_input_bindings: u32,
    pub max_vertex_input_attribute_offset: u32,
    pub max_vertex_input_binding_stride: u32,
    pub max_vertex_output_components: u32,
    pub max_tessellation_generation_level: u32,
    pub max_tessellation_patch_size: u32,
    pub max_tessellation_control_per_vertex_input_components: u32,
    pub max_tessellation_control_per_vertex_output_components: u32,
    pub max_tessellation_control_per_patch_output_components: u32,
    pub max_tessellation_control_total_output_components: u32,
    pub max_tessellation_evaluation_input_components: u32,
    pub max_tessellation_evaluation_output_components: u32,
    pub max_geometry_shader_invocations: u32,
    pub max_geometry_input_components: u32,
    pub max_geometry_output_components: u32,
    pub max_geometry_output_vertices: u32,
    pub max_geometry_total_output_components: u32,
    pub max_fragment_input_components: u32,
    pub max_fragment_output_attachments: u32,
    pub max_fragment_dual_src_attachments: u32,
    pub max_fragment_combined_output_resources: u32,
    pub max_compute_shared_memory_size: u32,
    pub max_compute_work_group_count: [u32; 3],
    pub max_compute_work_group_invocations: u32,
    pub max_compute_work_group_size: [u32; 3],
    pub sub_pixel_precision_bits: u32,
    pub sub_texel_precision_bits: u32,
    pub mipmap_precision_bits: u32,
    pub max_draw_indexed_index_value: u32,
    pub max_draw_indirect_count: u32,
    pub max_sampler_lod_bias: f32,
    pub max_sampler_anisotropy: f32,
    pub max_viewports: u32,
    pub max_viewport_dimensions: [u32; 2],
    pub viewport_bounds_range: [f32; 2],
    pub viewport_sub_pixel_bits: u32,
    pub min_memory_map_alignment: usize,
    pub min_texel_buffer_offset_alignment: DeviceSize,
    pub min_uniform_buffer_offset_alignment: DeviceSize,
    pub min_storage_buffer_offset_alignment: DeviceSize,
    pub min_texel_offset: i32,
    pub max_texel_offset: u32,
    pub min_texel_gather_offset: i32,
    pub max_texel_gather_offset: u32,
    pub min_interpolation_offset: f32,
    pub max_interpolation_offset: f32,
    pub sub_pixel_interpolation_offset_bits: u32,
    pub max_framebuffer_width: u32,
    pub max_framebuffer_height: u32,
    pub max_framebuffer_layers: u32,
    pub framebuffer_color_sample_counts: SampleCountFlags,
    pub framebuffer_depth_sample_counts: SampleCountFlags,
    pub framebuffer_stencil_sample_counts: SampleCountFlags,
    pub framebuffer_no_attachments_sample_counts: SampleCountFlags,
    pub max_color_attachments: u32,
    pub sampled_image_color_sample_counts: SampleCountFlags,
    pub sampled_image_integer_sample_counts: SampleCountFlags,
    pub sampled_image_depth_sample_counts: SampleCountFlags,
    pub sampled_image_stencil_sample_counts: SampleCountFlags,
    pub storage_image_sample_counts: SampleCountFlags,
    pub max_sample_mask_words: u32,
    pub timestamp_compute_and_graphics: Bool32,
    pub timestamp_period: f32,
    pub max_clip_distances: u32,
    pub max_cull_distances: u32,
    pub max_combined_clip_and_cull_distances: u32,
    pub discrete_queue_priorities: u32,
    pub point_size_range: [f32; 2],
    pub line_width_range: [f32; 2],
    pub point_size_granularity: f32,
    pub line_width_granularity: f32,
    pub strict_lines: Bool32,
    pub standard_sample_locations: Bool32,
    pub optimal_buffer_copy_offset_alignment: DeviceSize,
    pub optimal_buffer_copy_row_pitch_alignment: DeviceSize,
    pub non_coherent_atom_size: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMemoryProperties.html>
#[doc(alias = "VkPhysicalDeviceMemoryProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMemoryProperties {
    pub memory_type_count: u32,
    pub memory_types: [MemoryType; 32],
    pub memory_heap_count: u32,
    pub memory_heaps: [MemoryHeap; 16],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSparseProperties.html>
#[doc(alias = "VkPhysicalDeviceSparseProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSparseProperties {
    pub residency_standard_2_d_block_shape: Bool32,
    pub residency_standard_2_d_multisample_block_shape: Bool32,
    pub residency_standard_3_d_block_shape: Bool32,
    pub residency_aligned_mip_size: Bool32,
    pub residency_non_resident_strict: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceProperties.html>
#[doc(alias = "VkPhysicalDeviceProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceProperties {
    pub api_version: u32,
    pub driver_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub device_type: PhysicalDeviceType,
    pub device_name: [c_char; 256],
    pub pipeline_cache_uuid: [u8; 16],
    pub limits: PhysicalDeviceLimits,
    pub sparse_properties: PhysicalDeviceSparseProperties,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyProperties.html>
#[doc(alias = "VkQueueFamilyProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QueueFamilyProperties {
    pub queue_flags: QueueFlags,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: Extent3D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceQueueCreateInfo.html>
#[doc(alias = "VkDeviceQueueCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceQueueCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_count: u32,
    pub p_queue_priorities: *const f32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceCreateInfo.html>
#[doc(alias = "VkDeviceCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceCreateFlags,
    pub queue_create_info_count: u32,
    pub p_queue_create_infos: *const DeviceQueueCreateInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const c_char,
    pub p_enabled_features: *const PhysicalDeviceFeatures,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExtensionProperties.html>
#[doc(alias = "VkExtensionProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExtensionProperties {
    pub extension_name: [c_char; 256],
    pub spec_version: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkLayerProperties.html>
#[doc(alias = "VkLayerProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct LayerProperties {
    pub layer_name: [c_char; 256],
    pub spec_version: u32,
    pub implementation_version: u32,
    pub description: [c_char; 256],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubmitInfo.html>
#[doc(alias = "VkSubmitInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const Semaphore,
    pub p_wait_dst_stage_mask: *const PipelineStageFlags,
    pub command_buffer_count: u32,
    pub p_command_buffers: *const CommandBuffer,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphores: *const Semaphore,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMappedMemoryRange.html>
#[doc(alias = "VkMappedMemoryRange")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MappedMemoryRange {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryAllocateInfo.html>
#[doc(alias = "VkMemoryAllocateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub allocation_size: DeviceSize,
    pub memory_type_index: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryRequirements.html>
#[doc(alias = "VkMemoryRequirements")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryRequirements {
    pub size: DeviceSize,
    pub alignment: DeviceSize,
    pub memory_type_bits: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageSubresource.html>
#[doc(alias = "VkImageSubresource")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageSubresource {
    pub aspect_mask: ImageAspectFlags,
    pub mip_level: u32,
    pub array_layer: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageFormatProperties.html>
#[doc(alias = "VkSparseImageFormatProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SparseImageFormatProperties {
    pub aspect_mask: ImageAspectFlags,
    pub image_granularity: Extent3D,
    pub flags: SparseImageFormatFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageMemoryBind.html>
#[doc(alias = "VkSparseImageMemoryBind")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SparseImageMemoryBind {
    pub subresource: ImageSubresource,
    pub offset: Offset3D,
    pub extent: Extent3D,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub flags: SparseMemoryBindFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageMemoryBindInfo.html>
#[doc(alias = "VkSparseImageMemoryBindInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SparseImageMemoryBindInfo {
    pub image: Image,
    pub bind_count: u32,
    pub p_binds: *const SparseImageMemoryBind,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageMemoryRequirements.html>
#[doc(alias = "VkSparseImageMemoryRequirements")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SparseImageMemoryRequirements {
    pub format_properties: SparseImageFormatProperties,
    pub image_mip_tail_first_lod: u32,
    pub image_mip_tail_size: DeviceSize,
    pub image_mip_tail_offset: DeviceSize,
    pub image_mip_tail_stride: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseMemoryBind.html>
#[doc(alias = "VkSparseMemoryBind")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SparseMemoryBind {
    pub resource_offset: DeviceSize,
    pub size: DeviceSize,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub flags: SparseMemoryBindFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseBufferMemoryBindInfo.html>
#[doc(alias = "VkSparseBufferMemoryBindInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SparseBufferMemoryBindInfo {
    pub buffer: Buffer,
    pub bind_count: u32,
    pub p_binds: *const SparseMemoryBind,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageOpaqueMemoryBindInfo.html>
#[doc(alias = "VkSparseImageOpaqueMemoryBindInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SparseImageOpaqueMemoryBindInfo {
    pub image: Image,
    pub bind_count: u32,
    pub p_binds: *const SparseMemoryBind,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindSparseInfo.html>
#[doc(alias = "VkBindSparseInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BindSparseInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const Semaphore,
    pub buffer_bind_count: u32,
    pub p_buffer_binds: *const SparseBufferMemoryBindInfo,
    pub image_opaque_bind_count: u32,
    pub p_image_opaque_binds: *const SparseImageOpaqueMemoryBindInfo,
    pub image_bind_count: u32,
    pub p_image_binds: *const SparseImageMemoryBindInfo,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphores: *const Semaphore,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceCreateInfo.html>
#[doc(alias = "VkFenceCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FenceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: FenceCreateFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreCreateInfo.html>
#[doc(alias = "VkSemaphoreCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SemaphoreCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SemaphoreCreateFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPoolCreateInfo.html>
#[doc(alias = "VkQueryPoolCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QueryPoolCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: QueryPoolCreateFlags,
    pub query_type: QueryType,
    pub query_count: u32,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCreateInfo.html>
#[doc(alias = "VkBufferCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BufferCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: BufferCreateFlags,
    pub size: DeviceSize,
    pub usage: BufferUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCreateInfo.html>
#[doc(alias = "VkImageCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ImageCreateFlags,
    pub image_type: ImageType,
    pub format: Format,
    pub extent: Extent3D,
    pub mip_levels: u32,
    pub array_layers: u32,
    pub samples: SampleCountFlags,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub initial_layout: ImageLayout,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubresourceLayout.html>
#[doc(alias = "VkSubresourceLayout")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SubresourceLayout {
    pub offset: DeviceSize,
    pub size: DeviceSize,
    pub row_pitch: DeviceSize,
    pub array_pitch: DeviceSize,
    pub depth_pitch: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkComponentMapping.html>
#[doc(alias = "VkComponentMapping")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ComponentMapping {
    pub r: ComponentSwizzle,
    pub g: ComponentSwizzle,
    pub b: ComponentSwizzle,
    pub a: ComponentSwizzle,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageSubresourceRange.html>
#[doc(alias = "VkImageSubresourceRange")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageSubresourceRange {
    pub aspect_mask: ImageAspectFlags,
    pub base_mip_level: u32,
    pub level_count: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewCreateInfo.html>
#[doc(alias = "VkImageViewCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageViewCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ImageViewCreateFlags,
    pub image: Image,
    pub view_type: ImageViewType,
    pub format: Format,
    pub components: ComponentMapping,
    pub subresource_range: ImageSubresourceRange,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolCreateInfo.html>
#[doc(alias = "VkCommandPoolCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CommandPoolCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: CommandPoolCreateFlags,
    pub queue_family_index: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferAllocateInfo.html>
#[doc(alias = "VkCommandBufferAllocateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CommandBufferAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub command_pool: CommandPool,
    pub level: CommandBufferLevel,
    pub command_buffer_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferInheritanceInfo.html>
#[doc(alias = "VkCommandBufferInheritanceInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CommandBufferInheritanceInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub render_pass: RenderPass,
    pub subpass: u32,
    pub framebuffer: Framebuffer,
    pub occlusion_query_enable: Bool32,
    pub query_flags: QueryControlFlags,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferBeginInfo.html>
#[doc(alias = "VkCommandBufferBeginInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CommandBufferBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: CommandBufferUsageFlags,
    pub p_inheritance_info: *const CommandBufferInheritanceInfo,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCopy.html>
#[doc(alias = "VkBufferCopy")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BufferCopy {
    pub src_offset: DeviceSize,
    pub dst_offset: DeviceSize,
    pub size: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageSubresourceLayers.html>
#[doc(alias = "VkImageSubresourceLayers")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageSubresourceLayers {
    pub aspect_mask: ImageAspectFlags,
    pub mip_level: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferImageCopy.html>
#[doc(alias = "VkBufferImageCopy")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BufferImageCopy {
    pub buffer_offset: DeviceSize,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: Offset3D,
    pub image_extent: Extent3D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCopy.html>
#[doc(alias = "VkImageCopy")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageCopy {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3D,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferMemoryBarrier.html>
#[doc(alias = "VkBufferMemoryBarrier")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BufferMemoryBarrier {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageMemoryBarrier.html>
#[doc(alias = "VkImageMemoryBarrier")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageMemoryBarrier {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub old_layout: ImageLayout,
    pub new_layout: ImageLayout,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub image: Image,
    pub subresource_range: ImageSubresourceRange,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryBarrier.html>
#[doc(alias = "VkMemoryBarrier")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryBarrier {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDispatchIndirectCommand.html>
#[doc(alias = "VkDispatchIndirectCommand")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCacheHeaderVersionOne.html>
#[doc(alias = "VkPipelineCacheHeaderVersionOne")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineCacheHeaderVersionOne {
    pub header_size: u32,
    pub header_version: PipelineCacheHeaderVersion,
    pub vendor_id: u32,
    pub device_id: u32,
    pub pipeline_cache_uuid: [u8; 16],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkEventCreateInfo.html>
#[doc(alias = "VkEventCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EventCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: EventCreateFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferViewCreateInfo.html>
#[doc(alias = "VkBufferViewCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BufferViewCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: BufferViewCreateFlags,
    pub buffer: Buffer,
    pub format: Format,
    pub offset: DeviceSize,
    pub range: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderModuleCreateInfo.html>
#[doc(alias = "VkShaderModuleCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ShaderModuleCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ShaderModuleCreateFlags,
    pub code_size: usize,
    pub p_code: *const u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCacheCreateInfo.html>
#[doc(alias = "VkPipelineCacheCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineCacheCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCacheCreateFlags,
    pub initial_data_size: usize,
    pub p_initial_data: *const c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSpecializationMapEntry.html>
#[doc(alias = "VkSpecializationMapEntry")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SpecializationMapEntry {
    pub constant_id: u32,
    pub offset: u32,
    pub size: usize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSpecializationInfo.html>
#[doc(alias = "VkSpecializationInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SpecializationInfo {
    pub map_entry_count: u32,
    pub p_map_entries: *const SpecializationMapEntry,
    pub data_size: usize,
    pub p_data: *const c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineShaderStageCreateInfo.html>
#[doc(alias = "VkPipelineShaderStageCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineShaderStageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineShaderStageCreateFlags,
    pub stage: ShaderStageFlags,
    pub module: ShaderModule,
    pub p_name: *const c_char,
    pub p_specialization_info: *const SpecializationInfo,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkComputePipelineCreateInfo.html>
#[doc(alias = "VkComputePipelineCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ComputePipelineCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage: PipelineShaderStageCreateInfo,
    pub layout: PipelineLayout,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPushConstantRange.html>
#[doc(alias = "VkPushConstantRange")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PushConstantRange {
    pub stage_flags: ShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineLayoutCreateInfo.html>
#[doc(alias = "VkPipelineLayoutCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineLayoutCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineLayoutCreateFlags,
    pub set_layout_count: u32,
    pub p_set_layouts: *const DescriptorSetLayout,
    pub push_constant_range_count: u32,
    pub p_push_constant_ranges: *const PushConstantRange,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerCreateInfo.html>
#[doc(alias = "VkSamplerCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SamplerCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SamplerCreateFlags,
    pub mag_filter: Filter,
    pub min_filter: Filter,
    pub mipmap_mode: SamplerMipmapMode,
    pub address_mode_u: SamplerAddressMode,
    pub address_mode_v: SamplerAddressMode,
    pub address_mode_w: SamplerAddressMode,
    pub mip_lod_bias: f32,
    pub anisotropy_enable: Bool32,
    pub max_anisotropy: f32,
    pub compare_enable: Bool32,
    pub compare_op: CompareOp,
    pub min_lod: f32,
    pub max_lod: f32,
    pub border_color: BorderColor,
    pub unnormalized_coordinates: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyDescriptorSet.html>
#[doc(alias = "VkCopyDescriptorSet")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CopyDescriptorSet {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_set: DescriptorSet,
    pub src_binding: u32,
    pub src_array_element: u32,
    pub dst_set: DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorBufferInfo.html>
#[doc(alias = "VkDescriptorBufferInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorBufferInfo {
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub range: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorImageInfo.html>
#[doc(alias = "VkDescriptorImageInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorImageInfo {
    pub sampler: Sampler,
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPoolSize.html>
#[doc(alias = "VkDescriptorPoolSize")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorPoolSize {
    pub type_: DescriptorType,
    pub descriptor_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPoolCreateInfo.html>
#[doc(alias = "VkDescriptorPoolCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorPoolCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DescriptorPoolCreateFlags,
    pub max_sets: u32,
    pub pool_size_count: u32,
    pub p_pool_sizes: *const DescriptorPoolSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetAllocateInfo.html>
#[doc(alias = "VkDescriptorSetAllocateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorSetAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub descriptor_pool: DescriptorPool,
    pub descriptor_set_count: u32,
    pub p_set_layouts: *const DescriptorSetLayout,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayoutBinding.html>
#[doc(alias = "VkDescriptorSetLayoutBinding")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptor_type: DescriptorType,
    pub descriptor_count: u32,
    pub stage_flags: ShaderStageFlags,
    pub p_immutable_samplers: *const Sampler,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayoutCreateInfo.html>
#[doc(alias = "VkDescriptorSetLayoutCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorSetLayoutCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DescriptorSetLayoutCreateFlags,
    pub binding_count: u32,
    pub p_bindings: *const DescriptorSetLayoutBinding,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkWriteDescriptorSet.html>
#[doc(alias = "VkWriteDescriptorSet")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct WriteDescriptorSet {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dst_set: DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub descriptor_type: DescriptorType,
    pub p_image_info: *const DescriptorImageInfo,
    pub p_buffer_info: *const DescriptorBufferInfo,
    pub p_texel_buffer_view: *const BufferView,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClearColorValue.html>
#[doc(alias = "VkClearColorValue")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ClearColorValue {
    pub float_32: [f32; 4],
    pub int_32: [i32; 4],
    pub uint_32: [u32; 4],
}
impl std::fmt::Debug for ClearColorValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClearColorValue {{ .. }}")
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDrawIndexedIndirectCommand.html>
#[doc(alias = "VkDrawIndexedIndirectCommand")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DrawIndexedIndirectCommand {
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub vertex_offset: i32,
    pub first_instance: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDrawIndirectCommand.html>
#[doc(alias = "VkDrawIndirectCommand")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DrawIndirectCommand {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub first_vertex: u32,
    pub first_instance: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkStencilOpState.html>
#[doc(alias = "VkStencilOpState")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct StencilOpState {
    pub fail_op: StencilOp,
    pub pass_op: StencilOp,
    pub depth_fail_op: StencilOp,
    pub compare_op: CompareOp,
    pub compare_mask: u32,
    pub write_mask: u32,
    pub reference: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVertexInputAttributeDescription.html>
#[doc(alias = "VkVertexInputAttributeDescription")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: Format,
    pub offset: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVertexInputBindingDescription.html>
#[doc(alias = "VkVertexInputBindingDescription")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub input_rate: VertexInputRate,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkViewport.html>
#[doc(alias = "VkViewport")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineColorBlendAttachmentState.html>
#[doc(alias = "VkPipelineColorBlendAttachmentState")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineColorBlendAttachmentState {
    pub blend_enable: Bool32,
    pub src_color_blend_factor: BlendFactor,
    pub dst_color_blend_factor: BlendFactor,
    pub color_blend_op: BlendOp,
    pub src_alpha_blend_factor: BlendFactor,
    pub dst_alpha_blend_factor: BlendFactor,
    pub alpha_blend_op: BlendOp,
    pub color_write_mask: ColorComponentFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineColorBlendStateCreateInfo.html>
#[doc(alias = "VkPipelineColorBlendStateCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineColorBlendStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineColorBlendStateCreateFlags,
    pub logic_op_enable: Bool32,
    pub logic_op: LogicOp,
    pub attachment_count: u32,
    pub p_attachments: *const PipelineColorBlendAttachmentState,
    pub blend_constants: [f32; 4],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineDepthStencilStateCreateInfo.html>
#[doc(alias = "VkPipelineDepthStencilStateCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineDepthStencilStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineDepthStencilStateCreateFlags,
    pub depth_test_enable: Bool32,
    pub depth_write_enable: Bool32,
    pub depth_compare_op: CompareOp,
    pub depth_bounds_test_enable: Bool32,
    pub stencil_test_enable: Bool32,
    pub front: StencilOpState,
    pub back: StencilOpState,
    pub min_depth_bounds: f32,
    pub max_depth_bounds: f32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineDynamicStateCreateInfo.html>
#[doc(alias = "VkPipelineDynamicStateCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineDynamicStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineDynamicStateCreateFlags,
    pub dynamic_state_count: u32,
    pub p_dynamic_states: *const DynamicState,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineInputAssemblyStateCreateInfo.html>
#[doc(alias = "VkPipelineInputAssemblyStateCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineInputAssemblyStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineInputAssemblyStateCreateFlags,
    pub topology: PrimitiveTopology,
    pub primitive_restart_enable: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineMultisampleStateCreateInfo.html>
#[doc(alias = "VkPipelineMultisampleStateCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineMultisampleStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineMultisampleStateCreateFlags,
    pub rasterization_samples: SampleCountFlags,
    pub sample_shading_enable: Bool32,
    pub min_sample_shading: f32,
    pub p_sample_mask: *const SampleMask,
    pub alpha_to_coverage_enable: Bool32,
    pub alpha_to_one_enable: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationStateCreateInfo.html>
#[doc(alias = "VkPipelineRasterizationStateCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineRasterizationStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineRasterizationStateCreateFlags,
    pub depth_clamp_enable: Bool32,
    pub rasterizer_discard_enable: Bool32,
    pub polygon_mode: PolygonMode,
    pub cull_mode: CullModeFlags,
    pub front_face: FrontFace,
    pub depth_bias_enable: Bool32,
    pub depth_bias_constant_factor: f32,
    pub depth_bias_clamp: f32,
    pub depth_bias_slope_factor: f32,
    pub line_width: f32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineTessellationStateCreateInfo.html>
#[doc(alias = "VkPipelineTessellationStateCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineTessellationStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineTessellationStateCreateFlags,
    pub patch_control_points: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineVertexInputStateCreateInfo.html>
#[doc(alias = "VkPipelineVertexInputStateCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineVertexInputStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineVertexInputStateCreateFlags,
    pub vertex_binding_description_count: u32,
    pub p_vertex_binding_descriptions: *const VertexInputBindingDescription,
    pub vertex_attribute_description_count: u32,
    pub p_vertex_attribute_descriptions: *const VertexInputAttributeDescription,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportStateCreateInfo.html>
#[doc(alias = "VkPipelineViewportStateCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineViewportStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineViewportStateCreateFlags,
    pub viewport_count: u32,
    pub p_viewports: *const Viewport,
    pub scissor_count: u32,
    pub p_scissors: *const Rect2D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGraphicsPipelineCreateInfo.html>
#[doc(alias = "VkGraphicsPipelineCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GraphicsPipelineCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage_count: u32,
    pub p_stages: *const PipelineShaderStageCreateInfo,
    pub p_vertex_input_state: *const PipelineVertexInputStateCreateInfo,
    pub p_input_assembly_state: *const PipelineInputAssemblyStateCreateInfo,
    pub p_tessellation_state: *const PipelineTessellationStateCreateInfo,
    pub p_viewport_state: *const PipelineViewportStateCreateInfo,
    pub p_rasterization_state: *const PipelineRasterizationStateCreateInfo,
    pub p_multisample_state: *const PipelineMultisampleStateCreateInfo,
    pub p_depth_stencil_state: *const PipelineDepthStencilStateCreateInfo,
    pub p_color_blend_state: *const PipelineColorBlendStateCreateInfo,
    pub p_dynamic_state: *const PipelineDynamicStateCreateInfo,
    pub layout: PipelineLayout,
    pub render_pass: RenderPass,
    pub subpass: u32,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentDescription.html>
#[doc(alias = "VkAttachmentDescription")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AttachmentDescription {
    pub flags: AttachmentDescriptionFlags,
    pub format: Format,
    pub samples: SampleCountFlags,
    pub load_op: AttachmentLoadOp,
    pub store_op: AttachmentStoreOp,
    pub stencil_load_op: AttachmentLoadOp,
    pub stencil_store_op: AttachmentStoreOp,
    pub initial_layout: ImageLayout,
    pub final_layout: ImageLayout,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentReference.html>
#[doc(alias = "VkAttachmentReference")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AttachmentReference {
    pub attachment: u32,
    pub layout: ImageLayout,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebufferCreateInfo.html>
#[doc(alias = "VkFramebufferCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FramebufferCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: FramebufferCreateFlags,
    pub render_pass: RenderPass,
    pub attachment_count: u32,
    pub p_attachments: *const ImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassDependency.html>
#[doc(alias = "VkSubpassDependency")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SubpassDependency {
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: PipelineStageFlags,
    pub dst_stage_mask: PipelineStageFlags,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub dependency_flags: DependencyFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassDescription.html>
#[doc(alias = "VkSubpassDescription")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SubpassDescription {
    pub flags: SubpassDescriptionFlags,
    pub pipeline_bind_point: PipelineBindPoint,
    pub input_attachment_count: u32,
    pub p_input_attachments: *const AttachmentReference,
    pub color_attachment_count: u32,
    pub p_color_attachments: *const AttachmentReference,
    pub p_resolve_attachments: *const AttachmentReference,
    pub p_depth_stencil_attachment: *const AttachmentReference,
    pub preserve_attachment_count: u32,
    pub p_preserve_attachments: *const u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassCreateInfo.html>
#[doc(alias = "VkRenderPassCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderPassCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: RenderPassCreateFlags,
    pub attachment_count: u32,
    pub p_attachments: *const AttachmentDescription,
    pub subpass_count: u32,
    pub p_subpasses: *const SubpassDescription,
    pub dependency_count: u32,
    pub p_dependencies: *const SubpassDependency,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClearDepthStencilValue.html>
#[doc(alias = "VkClearDepthStencilValue")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ClearDepthStencilValue {
    pub depth: f32,
    pub stencil: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClearRect.html>
#[doc(alias = "VkClearRect")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ClearRect {
    pub rect: Rect2D,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClearValue.html>
#[doc(alias = "VkClearValue")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ClearValue {
    pub color: ClearColorValue,
    pub depth_stencil: ClearDepthStencilValue,
}
impl std::fmt::Debug for ClearValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClearValue {{ .. }}")
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClearAttachment.html>
#[doc(alias = "VkClearAttachment")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ClearAttachment {
    pub aspect_mask: ImageAspectFlags,
    pub color_attachment: u32,
    pub clear_value: ClearValue,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageBlit.html>
#[doc(alias = "VkImageBlit")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageBlit {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offsets: [Offset3D; 2],
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offsets: [Offset3D; 2],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageResolve.html>
#[doc(alias = "VkImageResolve")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageResolve {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3D,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassBeginInfo.html>
#[doc(alias = "VkRenderPassBeginInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderPassBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub render_pass: RenderPass,
    pub framebuffer: Framebuffer,
    pub render_area: Rect2D,
    pub clear_value_count: u32,
    pub p_clear_values: *const ClearValue,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindBufferMemoryInfo.html>
#[doc(alias = "VkBindBufferMemoryInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BindBufferMemoryInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
}
pub type BindBufferMemoryInfoKHR = BindBufferMemoryInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindImageMemoryInfo.html>
#[doc(alias = "VkBindImageMemoryInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BindImageMemoryInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
}
pub type BindImageMemoryInfoKHR = BindImageMemoryInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryDedicatedRequirements.html>
#[doc(alias = "VkMemoryDedicatedRequirements")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryDedicatedRequirements {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub prefers_dedicated_allocation: Bool32,
    pub requires_dedicated_allocation: Bool32,
}
pub type MemoryDedicatedRequirementsKHR = MemoryDedicatedRequirements;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryDedicatedAllocateInfo.html>
#[doc(alias = "VkMemoryDedicatedAllocateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryDedicatedAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub buffer: Buffer,
}
pub type MemoryDedicatedAllocateInfoKHR = MemoryDedicatedAllocateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryAllocateFlagsInfo.html>
#[doc(alias = "VkMemoryAllocateFlagsInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryAllocateFlagsInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: MemoryAllocateFlags,
    pub device_mask: u32,
}
pub type MemoryAllocateFlagsInfoKHR = MemoryAllocateFlagsInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupCommandBufferBeginInfo.html>
#[doc(alias = "VkDeviceGroupCommandBufferBeginInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceGroupCommandBufferBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_mask: u32,
}
pub type DeviceGroupCommandBufferBeginInfoKHR = DeviceGroupCommandBufferBeginInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupSubmitInfo.html>
#[doc(alias = "VkDeviceGroupSubmitInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceGroupSubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphore_device_indices: *const u32,
    pub command_buffer_count: u32,
    pub p_command_buffer_device_masks: *const u32,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphore_device_indices: *const u32,
}
pub type DeviceGroupSubmitInfoKHR = DeviceGroupSubmitInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupBindSparseInfo.html>
#[doc(alias = "VkDeviceGroupBindSparseInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceGroupBindSparseInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub resource_device_index: u32,
    pub memory_device_index: u32,
}
pub type DeviceGroupBindSparseInfoKHR = DeviceGroupBindSparseInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindBufferMemoryDeviceGroupInfo.html>
#[doc(alias = "VkBindBufferMemoryDeviceGroupInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BindBufferMemoryDeviceGroupInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_index_count: u32,
    pub p_device_indices: *const u32,
}
pub type BindBufferMemoryDeviceGroupInfoKHR = BindBufferMemoryDeviceGroupInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindImageMemoryDeviceGroupInfo.html>
#[doc(alias = "VkBindImageMemoryDeviceGroupInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BindImageMemoryDeviceGroupInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_index_count: u32,
    pub p_device_indices: *const u32,
    pub split_instance_bind_region_count: u32,
    pub p_split_instance_bind_regions: *const Rect2D,
}
pub type BindImageMemoryDeviceGroupInfoKHR = BindImageMemoryDeviceGroupInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceGroupProperties.html>
#[doc(alias = "VkPhysicalDeviceGroupProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceGroupProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub physical_device_count: u32,
    pub physical_devices: [PhysicalDevice; 32],
    pub subset_allocation: Bool32,
}
pub type PhysicalDeviceGroupPropertiesKHR = PhysicalDeviceGroupProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupDeviceCreateInfo.html>
#[doc(alias = "VkDeviceGroupDeviceCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceGroupDeviceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub physical_device_count: u32,
    pub p_physical_devices: *const PhysicalDevice,
}
pub type DeviceGroupDeviceCreateInfoKHR = DeviceGroupDeviceCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferMemoryRequirementsInfo2.html>
#[doc(alias = "VkBufferMemoryRequirementsInfo2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BufferMemoryRequirementsInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
}
pub type BufferMemoryRequirementsInfo2KHR = BufferMemoryRequirementsInfo2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageMemoryRequirementsInfo2.html>
#[doc(alias = "VkImageMemoryRequirementsInfo2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageMemoryRequirementsInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
}
pub type ImageMemoryRequirementsInfo2KHR = ImageMemoryRequirementsInfo2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageSparseMemoryRequirementsInfo2.html>
#[doc(alias = "VkImageSparseMemoryRequirementsInfo2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageSparseMemoryRequirementsInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
}
pub type ImageSparseMemoryRequirementsInfo2KHR = ImageSparseMemoryRequirementsInfo2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryRequirements2.html>
#[doc(alias = "VkMemoryRequirements2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryRequirements2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_requirements: MemoryRequirements,
}
pub type MemoryRequirements2KHR = MemoryRequirements2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageMemoryRequirements2.html>
#[doc(alias = "VkSparseImageMemoryRequirements2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SparseImageMemoryRequirements2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_requirements: SparseImageMemoryRequirements,
}
pub type SparseImageMemoryRequirements2KHR = SparseImageMemoryRequirements2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFeatures2.html>
#[doc(alias = "VkPhysicalDeviceFeatures2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFeatures2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub features: PhysicalDeviceFeatures,
}
pub type PhysicalDeviceFeatures2KHR = PhysicalDeviceFeatures2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceProperties2.html>
#[doc(alias = "VkPhysicalDeviceProperties2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub properties: PhysicalDeviceProperties,
}
pub type PhysicalDeviceProperties2KHR = PhysicalDeviceProperties2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatProperties2.html>
#[doc(alias = "VkFormatProperties2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FormatProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format_properties: FormatProperties,
}
pub type FormatProperties2KHR = FormatProperties2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageFormatProperties2.html>
#[doc(alias = "VkImageFormatProperties2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageFormatProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_format_properties: ImageFormatProperties,
}
pub type ImageFormatProperties2KHR = ImageFormatProperties2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageFormatInfo2.html>
#[doc(alias = "VkPhysicalDeviceImageFormatInfo2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageFormatInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format: Format,
    pub type_: ImageType,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub flags: ImageCreateFlags,
}
pub type PhysicalDeviceImageFormatInfo2KHR = PhysicalDeviceImageFormatInfo2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyProperties2.html>
#[doc(alias = "VkQueueFamilyProperties2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QueueFamilyProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub queue_family_properties: QueueFamilyProperties,
}
pub type QueueFamilyProperties2KHR = QueueFamilyProperties2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMemoryProperties2.html>
#[doc(alias = "VkPhysicalDeviceMemoryProperties2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMemoryProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_properties: PhysicalDeviceMemoryProperties,
}
pub type PhysicalDeviceMemoryProperties2KHR = PhysicalDeviceMemoryProperties2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageFormatProperties2.html>
#[doc(alias = "VkSparseImageFormatProperties2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SparseImageFormatProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub properties: SparseImageFormatProperties,
}
pub type SparseImageFormatProperties2KHR = SparseImageFormatProperties2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSparseImageFormatInfo2.html>
#[doc(alias = "VkPhysicalDeviceSparseImageFormatInfo2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSparseImageFormatInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format: Format,
    pub type_: ImageType,
    pub samples: SampleCountFlags,
    pub usage: ImageUsageFlags,
    pub tiling: ImageTiling,
}
pub type PhysicalDeviceSparseImageFormatInfo2KHR = PhysicalDeviceSparseImageFormatInfo2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewUsageCreateInfo.html>
#[doc(alias = "VkImageViewUsageCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageViewUsageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub usage: ImageUsageFlags,
}
pub type ImageViewUsageCreateInfoKHR = ImageViewUsageCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceProtectedMemoryFeatures.html>
#[doc(alias = "VkPhysicalDeviceProtectedMemoryFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceProtectedMemoryFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub protected_memory: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceProtectedMemoryProperties.html>
#[doc(alias = "VkPhysicalDeviceProtectedMemoryProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceProtectedMemoryProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub protected_no_fault: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceQueueInfo2.html>
#[doc(alias = "VkDeviceQueueInfo2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceQueueInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_index: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkProtectedSubmitInfo.html>
#[doc(alias = "VkProtectedSubmitInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ProtectedSubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub protected_submit: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindImagePlaneMemoryInfo.html>
#[doc(alias = "VkBindImagePlaneMemoryInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BindImagePlaneMemoryInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub plane_aspect: ImageAspectFlags,
}
pub type BindImagePlaneMemoryInfoKHR = BindImagePlaneMemoryInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImagePlaneMemoryRequirementsInfo.html>
#[doc(alias = "VkImagePlaneMemoryRequirementsInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImagePlaneMemoryRequirementsInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub plane_aspect: ImageAspectFlags,
}
pub type ImagePlaneMemoryRequirementsInfoKHR = ImagePlaneMemoryRequirementsInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryProperties.html>
#[doc(alias = "VkExternalMemoryProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExternalMemoryProperties {
    pub external_memory_features: ExternalMemoryFeatureFlags,
    pub export_from_imported_handle_types: ExternalMemoryHandleTypeFlags,
    pub compatible_handle_types: ExternalMemoryHandleTypeFlags,
}
pub type ExternalMemoryPropertiesKHR = ExternalMemoryProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalImageFormatInfo.html>
#[doc(alias = "VkPhysicalDeviceExternalImageFormatInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExternalImageFormatInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlags,
}
pub type PhysicalDeviceExternalImageFormatInfoKHR = PhysicalDeviceExternalImageFormatInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalImageFormatProperties.html>
#[doc(alias = "VkExternalImageFormatProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExternalImageFormatProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_memory_properties: ExternalMemoryProperties,
}
pub type ExternalImageFormatPropertiesKHR = ExternalImageFormatProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalBufferInfo.html>
#[doc(alias = "VkPhysicalDeviceExternalBufferInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExternalBufferInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: BufferCreateFlags,
    pub usage: BufferUsageFlags,
    pub handle_type: ExternalMemoryHandleTypeFlags,
}
pub type PhysicalDeviceExternalBufferInfoKHR = PhysicalDeviceExternalBufferInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalBufferProperties.html>
#[doc(alias = "VkExternalBufferProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExternalBufferProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_memory_properties: ExternalMemoryProperties,
}
pub type ExternalBufferPropertiesKHR = ExternalBufferProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceIDProperties.html>
#[doc(alias = "VkPhysicalDeviceIDProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceIDProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_uuid: [u8; 16],
    pub driver_uuid: [u8; 16],
    pub device_luid: [u8; 8],
    pub device_node_mask: u32,
    pub device_luid_valid: Bool32,
}
pub type PhysicalDeviceIDPropertiesKHR = PhysicalDeviceIDProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryImageCreateInfo.html>
#[doc(alias = "VkExternalMemoryImageCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExternalMemoryImageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
pub type ExternalMemoryImageCreateInfoKHR = ExternalMemoryImageCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryBufferCreateInfo.html>
#[doc(alias = "VkExternalMemoryBufferCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExternalMemoryBufferCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
pub type ExternalMemoryBufferCreateInfoKHR = ExternalMemoryBufferCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExportMemoryAllocateInfo.html>
#[doc(alias = "VkExportMemoryAllocateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExportMemoryAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
pub type ExportMemoryAllocateInfoKHR = ExportMemoryAllocateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalFenceInfo.html>
#[doc(alias = "VkPhysicalDeviceExternalFenceInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExternalFenceInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalFenceHandleTypeFlags,
}
pub type PhysicalDeviceExternalFenceInfoKHR = PhysicalDeviceExternalFenceInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceProperties.html>
#[doc(alias = "VkExternalFenceProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExternalFenceProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub export_from_imported_handle_types: ExternalFenceHandleTypeFlags,
    pub compatible_handle_types: ExternalFenceHandleTypeFlags,
    pub external_fence_features: ExternalFenceFeatureFlags,
}
pub type ExternalFencePropertiesKHR = ExternalFenceProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExportFenceCreateInfo.html>
#[doc(alias = "VkExportFenceCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExportFenceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalFenceHandleTypeFlags,
}
pub type ExportFenceCreateInfoKHR = ExportFenceCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExportSemaphoreCreateInfo.html>
#[doc(alias = "VkExportSemaphoreCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExportSemaphoreCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalSemaphoreHandleTypeFlags,
}
pub type ExportSemaphoreCreateInfoKHR = ExportSemaphoreCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalSemaphoreInfo.html>
#[doc(alias = "VkPhysicalDeviceExternalSemaphoreInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExternalSemaphoreInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalSemaphoreHandleTypeFlags,
}
pub type PhysicalDeviceExternalSemaphoreInfoKHR = PhysicalDeviceExternalSemaphoreInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreProperties.html>
#[doc(alias = "VkExternalSemaphoreProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExternalSemaphoreProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub export_from_imported_handle_types: ExternalSemaphoreHandleTypeFlags,
    pub compatible_handle_types: ExternalSemaphoreHandleTypeFlags,
    pub external_semaphore_features: ExternalSemaphoreFeatureFlags,
}
pub type ExternalSemaphorePropertiesKHR = ExternalSemaphoreProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSubgroupProperties.html>
#[doc(alias = "VkPhysicalDeviceSubgroupProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSubgroupProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub subgroup_size: u32,
    pub supported_stages: ShaderStageFlags,
    pub supported_operations: SubgroupFeatureFlags,
    pub quad_operations_in_all_stages: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevice16BitStorageFeatures.html>
#[doc(alias = "VkPhysicalDevice16BitStorageFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevice16BitStorageFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub storage_buffer_16_bit_access: Bool32,
    pub uniform_and_storage_buffer_16_bit_access: Bool32,
    pub storage_push_constant_16: Bool32,
    pub storage_input_output_16: Bool32,
}
pub type PhysicalDevice16BitStorageFeaturesKHR = PhysicalDevice16BitStorageFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVariablePointersFeatures.html>
#[doc(alias = "VkPhysicalDeviceVariablePointersFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVariablePointersFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub variable_pointers_storage_buffer: Bool32,
    pub variable_pointers: Bool32,
}
pub type PhysicalDeviceVariablePointerFeatures = PhysicalDeviceVariablePointersFeatures;
pub type PhysicalDeviceVariablePointerFeaturesKHR = PhysicalDeviceVariablePointersFeatures;
pub type PhysicalDeviceVariablePointersFeaturesKHR = PhysicalDeviceVariablePointersFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplateEntry.html>
#[doc(alias = "VkDescriptorUpdateTemplateEntry")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorUpdateTemplateEntry {
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub descriptor_type: DescriptorType,
    pub offset: usize,
    pub stride: usize,
}
pub type DescriptorUpdateTemplateEntryKHR = DescriptorUpdateTemplateEntry;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplateCreateInfo.html>
#[doc(alias = "VkDescriptorUpdateTemplateCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorUpdateTemplateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DescriptorUpdateTemplateCreateFlags,
    pub descriptor_update_entry_count: u32,
    pub p_descriptor_update_entries: *const DescriptorUpdateTemplateEntry,
    pub template_type: DescriptorUpdateTemplateType,
    pub descriptor_set_layout: DescriptorSetLayout,
    pub pipeline_bind_point: PipelineBindPoint,
    pub pipeline_layout: PipelineLayout,
    pub set: u32,
}
pub type DescriptorUpdateTemplateCreateInfoKHR = DescriptorUpdateTemplateCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance3Properties.html>
#[doc(alias = "VkPhysicalDeviceMaintenance3Properties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance3Properties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_per_set_descriptors: u32,
    pub max_memory_allocation_size: DeviceSize,
}
pub type PhysicalDeviceMaintenance3PropertiesKHR = PhysicalDeviceMaintenance3Properties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayoutSupport.html>
#[doc(alias = "VkDescriptorSetLayoutSupport")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorSetLayoutSupport {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supported: Bool32,
}
pub type DescriptorSetLayoutSupportKHR = DescriptorSetLayoutSupport;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversionCreateInfo.html>
#[doc(alias = "VkSamplerYcbcrConversionCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SamplerYcbcrConversionCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format: Format,
    pub ycbcr_model: SamplerYcbcrModelConversion,
    pub ycbcr_range: SamplerYcbcrRange,
    pub components: ComponentMapping,
    pub x_chroma_offset: ChromaLocation,
    pub y_chroma_offset: ChromaLocation,
    pub chroma_filter: Filter,
    pub force_explicit_reconstruction: Bool32,
}
pub type SamplerYcbcrConversionCreateInfoKHR = SamplerYcbcrConversionCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversionInfo.html>
#[doc(alias = "VkSamplerYcbcrConversionInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SamplerYcbcrConversionInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub conversion: SamplerYcbcrConversion,
}
pub type SamplerYcbcrConversionInfoKHR = SamplerYcbcrConversionInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSamplerYcbcrConversionFeatures.html>
#[doc(alias = "VkPhysicalDeviceSamplerYcbcrConversionFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSamplerYcbcrConversionFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub sampler_ycbcr_conversion: Bool32,
}
pub type PhysicalDeviceSamplerYcbcrConversionFeaturesKHR =
    PhysicalDeviceSamplerYcbcrConversionFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversionImageFormatProperties.html>
#[doc(alias = "VkSamplerYcbcrConversionImageFormatProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SamplerYcbcrConversionImageFormatProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub combined_image_sampler_descriptor_count: u32,
}
pub type SamplerYcbcrConversionImageFormatPropertiesKHR =
    SamplerYcbcrConversionImageFormatProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupRenderPassBeginInfo.html>
#[doc(alias = "VkDeviceGroupRenderPassBeginInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceGroupRenderPassBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_mask: u32,
    pub device_render_area_count: u32,
    pub p_device_render_areas: *const Rect2D,
}
pub type DeviceGroupRenderPassBeginInfoKHR = DeviceGroupRenderPassBeginInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePointClippingProperties.html>
#[doc(alias = "VkPhysicalDevicePointClippingProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePointClippingProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub point_clipping_behavior: PointClippingBehavior,
}
pub type PhysicalDevicePointClippingPropertiesKHR = PhysicalDevicePointClippingProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkInputAttachmentAspectReference.html>
#[doc(alias = "VkInputAttachmentAspectReference")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct InputAttachmentAspectReference {
    pub subpass: u32,
    pub input_attachment_index: u32,
    pub aspect_mask: ImageAspectFlags,
}
pub type InputAttachmentAspectReferenceKHR = InputAttachmentAspectReference;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassInputAttachmentAspectCreateInfo.html>
#[doc(alias = "VkRenderPassInputAttachmentAspectCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderPassInputAttachmentAspectCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub aspect_reference_count: u32,
    pub p_aspect_references: *const InputAttachmentAspectReference,
}
pub type RenderPassInputAttachmentAspectCreateInfoKHR = RenderPassInputAttachmentAspectCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineTessellationDomainOriginStateCreateInfo.html>
#[doc(alias = "VkPipelineTessellationDomainOriginStateCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineTessellationDomainOriginStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub domain_origin: TessellationDomainOrigin,
}
pub type PipelineTessellationDomainOriginStateCreateInfoKHR =
    PipelineTessellationDomainOriginStateCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassMultiviewCreateInfo.html>
#[doc(alias = "VkRenderPassMultiviewCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderPassMultiviewCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub subpass_count: u32,
    pub p_view_masks: *const u32,
    pub dependency_count: u32,
    pub p_view_offsets: *const i32,
    pub correlation_mask_count: u32,
    pub p_correlation_masks: *const u32,
}
pub type RenderPassMultiviewCreateInfoKHR = RenderPassMultiviewCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMultiviewFeatures.html>
#[doc(alias = "VkPhysicalDeviceMultiviewFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMultiviewFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub multiview: Bool32,
    pub multiview_geometry_shader: Bool32,
    pub multiview_tessellation_shader: Bool32,
}
pub type PhysicalDeviceMultiviewFeaturesKHR = PhysicalDeviceMultiviewFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMultiviewProperties.html>
#[doc(alias = "VkPhysicalDeviceMultiviewProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMultiviewProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_multiview_view_count: u32,
    pub max_multiview_instance_index: u32,
}
pub type PhysicalDeviceMultiviewPropertiesKHR = PhysicalDeviceMultiviewProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderDrawParametersFeatures.html>
#[doc(alias = "VkPhysicalDeviceShaderDrawParametersFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderDrawParametersFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_draw_parameters: Bool32,
}
pub type PhysicalDeviceShaderDrawParameterFeatures = PhysicalDeviceShaderDrawParametersFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkConformanceVersion.html>
#[doc(alias = "VkConformanceVersion")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ConformanceVersion {
    pub major: u8,
    pub minor: u8,
    pub subminor: u8,
    pub patch: u8,
}
pub type ConformanceVersionKHR = ConformanceVersion;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDriverProperties.html>
#[doc(alias = "VkPhysicalDeviceDriverProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDriverProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub driver_id: DriverId,
    pub driver_name: [c_char; 256],
    pub driver_info: [c_char; 256],
    pub conformance_version: ConformanceVersion,
}
pub type PhysicalDeviceDriverPropertiesKHR = PhysicalDeviceDriverProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkan11Features.html>
#[doc(alias = "VkPhysicalDeviceVulkan11Features")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVulkan11Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub storage_buffer_16_bit_access: Bool32,
    pub uniform_and_storage_buffer_16_bit_access: Bool32,
    pub storage_push_constant_16: Bool32,
    pub storage_input_output_16: Bool32,
    pub multiview: Bool32,
    pub multiview_geometry_shader: Bool32,
    pub multiview_tessellation_shader: Bool32,
    pub variable_pointers_storage_buffer: Bool32,
    pub variable_pointers: Bool32,
    pub protected_memory: Bool32,
    pub sampler_ycbcr_conversion: Bool32,
    pub shader_draw_parameters: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkan11Properties.html>
#[doc(alias = "VkPhysicalDeviceVulkan11Properties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVulkan11Properties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_uuid: [u8; 16],
    pub driver_uuid: [u8; 16],
    pub device_luid: [u8; 8],
    pub device_node_mask: u32,
    pub device_luid_valid: Bool32,
    pub subgroup_size: u32,
    pub subgroup_supported_stages: ShaderStageFlags,
    pub subgroup_supported_operations: SubgroupFeatureFlags,
    pub subgroup_quad_operations_in_all_stages: Bool32,
    pub point_clipping_behavior: PointClippingBehavior,
    pub max_multiview_view_count: u32,
    pub max_multiview_instance_index: u32,
    pub protected_no_fault: Bool32,
    pub max_per_set_descriptors: u32,
    pub max_memory_allocation_size: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkan12Features.html>
#[doc(alias = "VkPhysicalDeviceVulkan12Features")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVulkan12Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub sampler_mirror_clamp_to_edge: Bool32,
    pub draw_indirect_count: Bool32,
    pub storage_buffer_8_bit_access: Bool32,
    pub uniform_and_storage_buffer_8_bit_access: Bool32,
    pub storage_push_constant_8: Bool32,
    pub shader_buffer_int_64_atomics: Bool32,
    pub shader_shared_int_64_atomics: Bool32,
    pub shader_float_16: Bool32,
    pub shader_int_8: Bool32,
    pub descriptor_indexing: Bool32,
    pub shader_input_attachment_array_dynamic_indexing: Bool32,
    pub shader_uniform_texel_buffer_array_dynamic_indexing: Bool32,
    pub shader_storage_texel_buffer_array_dynamic_indexing: Bool32,
    pub shader_uniform_buffer_array_non_uniform_indexing: Bool32,
    pub shader_sampled_image_array_non_uniform_indexing: Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing: Bool32,
    pub shader_storage_image_array_non_uniform_indexing: Bool32,
    pub shader_input_attachment_array_non_uniform_indexing: Bool32,
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32,
    pub shader_storage_texel_buffer_array_non_uniform_indexing: Bool32,
    pub descriptor_binding_uniform_buffer_update_after_bind: Bool32,
    pub descriptor_binding_sampled_image_update_after_bind: Bool32,
    pub descriptor_binding_storage_image_update_after_bind: Bool32,
    pub descriptor_binding_storage_buffer_update_after_bind: Bool32,
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32,
    pub descriptor_binding_storage_texel_buffer_update_after_bind: Bool32,
    pub descriptor_binding_update_unused_while_pending: Bool32,
    pub descriptor_binding_partially_bound: Bool32,
    pub descriptor_binding_variable_descriptor_count: Bool32,
    pub runtime_descriptor_array: Bool32,
    pub sampler_filter_minmax: Bool32,
    pub scalar_block_layout: Bool32,
    pub imageless_framebuffer: Bool32,
    pub uniform_buffer_standard_layout: Bool32,
    pub shader_subgroup_extended_types: Bool32,
    pub separate_depth_stencil_layouts: Bool32,
    pub host_query_reset: Bool32,
    pub timeline_semaphore: Bool32,
    pub buffer_device_address: Bool32,
    pub buffer_device_address_capture_replay: Bool32,
    pub buffer_device_address_multi_device: Bool32,
    pub vulkan_memory_model: Bool32,
    pub vulkan_memory_model_device_scope: Bool32,
    pub vulkan_memory_model_availability_visibility_chains: Bool32,
    pub shader_output_viewport_index: Bool32,
    pub shader_output_layer: Bool32,
    pub subgroup_broadcast_dynamic_id: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkan12Properties.html>
#[doc(alias = "VkPhysicalDeviceVulkan12Properties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVulkan12Properties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub driver_id: DriverId,
    pub driver_name: [c_char; 256],
    pub driver_info: [c_char; 256],
    pub conformance_version: ConformanceVersion,
    pub denorm_behavior_independence: ShaderFloatControlsIndependence,
    pub rounding_mode_independence: ShaderFloatControlsIndependence,
    pub shader_signed_zero_inf_nan_preserve_float_16: Bool32,
    pub shader_signed_zero_inf_nan_preserve_float_32: Bool32,
    pub shader_signed_zero_inf_nan_preserve_float_64: Bool32,
    pub shader_denorm_preserve_float_16: Bool32,
    pub shader_denorm_preserve_float_32: Bool32,
    pub shader_denorm_preserve_float_64: Bool32,
    pub shader_denorm_flush_to_zero_float_16: Bool32,
    pub shader_denorm_flush_to_zero_float_32: Bool32,
    pub shader_denorm_flush_to_zero_float_64: Bool32,
    pub shader_rounding_mode_rte_float_16: Bool32,
    pub shader_rounding_mode_rte_float_32: Bool32,
    pub shader_rounding_mode_rte_float_64: Bool32,
    pub shader_rounding_mode_rtz_float_16: Bool32,
    pub shader_rounding_mode_rtz_float_32: Bool32,
    pub shader_rounding_mode_rtz_float_64: Bool32,
    pub max_update_after_bind_descriptors_in_all_pools: u32,
    pub shader_uniform_buffer_array_non_uniform_indexing_native: Bool32,
    pub shader_sampled_image_array_non_uniform_indexing_native: Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing_native: Bool32,
    pub shader_storage_image_array_non_uniform_indexing_native: Bool32,
    pub shader_input_attachment_array_non_uniform_indexing_native: Bool32,
    pub robust_buffer_access_update_after_bind: Bool32,
    pub quad_divergent_implicit_lod: Bool32,
    pub max_per_stage_descriptor_update_after_bind_samplers: u32,
    pub max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
    pub max_per_stage_descriptor_update_after_bind_sampled_images: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_images: u32,
    pub max_per_stage_descriptor_update_after_bind_input_attachments: u32,
    pub max_per_stage_update_after_bind_resources: u32,
    pub max_descriptor_set_update_after_bind_samplers: u32,
    pub max_descriptor_set_update_after_bind_uniform_buffers: u32,
    pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_storage_buffers: u32,
    pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_sampled_images: u32,
    pub max_descriptor_set_update_after_bind_storage_images: u32,
    pub max_descriptor_set_update_after_bind_input_attachments: u32,
    pub supported_depth_resolve_modes: ResolveModeFlags,
    pub supported_stencil_resolve_modes: ResolveModeFlags,
    pub independent_resolve_none: Bool32,
    pub independent_resolve: Bool32,
    pub filter_minmax_single_component_formats: Bool32,
    pub filter_minmax_image_component_mapping: Bool32,
    pub max_timeline_semaphore_value_difference: u64,
    pub framebuffer_integer_color_sample_counts: SampleCountFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageFormatListCreateInfo.html>
#[doc(alias = "VkImageFormatListCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageFormatListCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub view_format_count: u32,
    pub p_view_formats: *const Format,
}
pub type ImageFormatListCreateInfoKHR = ImageFormatListCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkanMemoryModelFeatures.html>
#[doc(alias = "VkPhysicalDeviceVulkanMemoryModelFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVulkanMemoryModelFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub vulkan_memory_model: Bool32,
    pub vulkan_memory_model_device_scope: Bool32,
    pub vulkan_memory_model_availability_visibility_chains: Bool32,
}
pub type PhysicalDeviceVulkanMemoryModelFeaturesKHR = PhysicalDeviceVulkanMemoryModelFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceHostQueryResetFeatures.html>
#[doc(alias = "VkPhysicalDeviceHostQueryResetFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceHostQueryResetFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub host_query_reset: Bool32,
}
pub type PhysicalDeviceHostQueryResetFeaturesEXT = PhysicalDeviceHostQueryResetFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTimelineSemaphoreFeatures.html>
#[doc(alias = "VkPhysicalDeviceTimelineSemaphoreFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTimelineSemaphoreFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub timeline_semaphore: Bool32,
}
pub type PhysicalDeviceTimelineSemaphoreFeaturesKHR = PhysicalDeviceTimelineSemaphoreFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTimelineSemaphoreProperties.html>
#[doc(alias = "VkPhysicalDeviceTimelineSemaphoreProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTimelineSemaphoreProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_timeline_semaphore_value_difference: u64,
}
pub type PhysicalDeviceTimelineSemaphorePropertiesKHR = PhysicalDeviceTimelineSemaphoreProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreTypeCreateInfo.html>
#[doc(alias = "VkSemaphoreTypeCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SemaphoreTypeCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore_type: SemaphoreType,
    pub initial_value: u64,
}
pub type SemaphoreTypeCreateInfoKHR = SemaphoreTypeCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTimelineSemaphoreSubmitInfo.html>
#[doc(alias = "VkTimelineSemaphoreSubmitInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TimelineSemaphoreSubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_value_count: u32,
    pub p_wait_semaphore_values: *const u64,
    pub signal_semaphore_value_count: u32,
    pub p_signal_semaphore_values: *const u64,
}
pub type TimelineSemaphoreSubmitInfoKHR = TimelineSemaphoreSubmitInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreWaitInfo.html>
#[doc(alias = "VkSemaphoreWaitInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SemaphoreWaitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SemaphoreWaitFlags,
    pub semaphore_count: u32,
    pub p_semaphores: *const Semaphore,
    pub p_values: *const u64,
}
pub type SemaphoreWaitInfoKHR = SemaphoreWaitInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreSignalInfo.html>
#[doc(alias = "VkSemaphoreSignalInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SemaphoreSignalInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub value: u64,
}
pub type SemaphoreSignalInfoKHR = SemaphoreSignalInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceBufferDeviceAddressFeatures.html>
#[doc(alias = "VkPhysicalDeviceBufferDeviceAddressFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceBufferDeviceAddressFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub buffer_device_address: Bool32,
    pub buffer_device_address_capture_replay: Bool32,
    pub buffer_device_address_multi_device: Bool32,
}
pub type PhysicalDeviceBufferDeviceAddressFeaturesKHR = PhysicalDeviceBufferDeviceAddressFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferDeviceAddressInfo.html>
#[doc(alias = "VkBufferDeviceAddressInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BufferDeviceAddressInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
}
pub type BufferDeviceAddressInfoKHR = BufferDeviceAddressInfo;
pub type BufferDeviceAddressInfoEXT = BufferDeviceAddressInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferOpaqueCaptureAddressCreateInfo.html>
#[doc(alias = "VkBufferOpaqueCaptureAddressCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BufferOpaqueCaptureAddressCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub opaque_capture_address: u64,
}
pub type BufferOpaqueCaptureAddressCreateInfoKHR = BufferOpaqueCaptureAddressCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryOpaqueCaptureAddressAllocateInfo.html>
#[doc(alias = "VkMemoryOpaqueCaptureAddressAllocateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryOpaqueCaptureAddressAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub opaque_capture_address: u64,
}
pub type MemoryOpaqueCaptureAddressAllocateInfoKHR = MemoryOpaqueCaptureAddressAllocateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemoryOpaqueCaptureAddressInfo.html>
#[doc(alias = "VkDeviceMemoryOpaqueCaptureAddressInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceMemoryOpaqueCaptureAddressInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
}
pub type DeviceMemoryOpaqueCaptureAddressInfoKHR = DeviceMemoryOpaqueCaptureAddressInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevice8BitStorageFeatures.html>
#[doc(alias = "VkPhysicalDevice8BitStorageFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevice8BitStorageFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub storage_buffer_8_bit_access: Bool32,
    pub uniform_and_storage_buffer_8_bit_access: Bool32,
    pub storage_push_constant_8: Bool32,
}
pub type PhysicalDevice8BitStorageFeaturesKHR = PhysicalDevice8BitStorageFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderAtomicInt64Features.html>
#[doc(alias = "VkPhysicalDeviceShaderAtomicInt64Features")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderAtomicInt64Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_buffer_int_64_atomics: Bool32,
    pub shader_shared_int_64_atomics: Bool32,
}
pub type PhysicalDeviceShaderAtomicInt64FeaturesKHR = PhysicalDeviceShaderAtomicInt64Features;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderFloat16Int8Features.html>
#[doc(alias = "VkPhysicalDeviceShaderFloat16Int8Features")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderFloat16Int8Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_float_16: Bool32,
    pub shader_int_8: Bool32,
}
pub type PhysicalDeviceShaderFloat16Int8FeaturesKHR = PhysicalDeviceShaderFloat16Int8Features;
pub type PhysicalDeviceFloat16Int8FeaturesKHR = PhysicalDeviceShaderFloat16Int8Features;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFloatControlsProperties.html>
#[doc(alias = "VkPhysicalDeviceFloatControlsProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFloatControlsProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub denorm_behavior_independence: ShaderFloatControlsIndependence,
    pub rounding_mode_independence: ShaderFloatControlsIndependence,
    pub shader_signed_zero_inf_nan_preserve_float_16: Bool32,
    pub shader_signed_zero_inf_nan_preserve_float_32: Bool32,
    pub shader_signed_zero_inf_nan_preserve_float_64: Bool32,
    pub shader_denorm_preserve_float_16: Bool32,
    pub shader_denorm_preserve_float_32: Bool32,
    pub shader_denorm_preserve_float_64: Bool32,
    pub shader_denorm_flush_to_zero_float_16: Bool32,
    pub shader_denorm_flush_to_zero_float_32: Bool32,
    pub shader_denorm_flush_to_zero_float_64: Bool32,
    pub shader_rounding_mode_rte_float_16: Bool32,
    pub shader_rounding_mode_rte_float_32: Bool32,
    pub shader_rounding_mode_rte_float_64: Bool32,
    pub shader_rounding_mode_rtz_float_16: Bool32,
    pub shader_rounding_mode_rtz_float_32: Bool32,
    pub shader_rounding_mode_rtz_float_64: Bool32,
}
pub type PhysicalDeviceFloatControlsPropertiesKHR = PhysicalDeviceFloatControlsProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayoutBindingFlagsCreateInfo.html>
#[doc(alias = "VkDescriptorSetLayoutBindingFlagsCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorSetLayoutBindingFlagsCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub binding_count: u32,
    pub p_binding_flags: *const DescriptorBindingFlags,
}
pub type DescriptorSetLayoutBindingFlagsCreateInfoEXT = DescriptorSetLayoutBindingFlagsCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDescriptorIndexingFeatures.html>
#[doc(alias = "VkPhysicalDeviceDescriptorIndexingFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDescriptorIndexingFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_input_attachment_array_dynamic_indexing: Bool32,
    pub shader_uniform_texel_buffer_array_dynamic_indexing: Bool32,
    pub shader_storage_texel_buffer_array_dynamic_indexing: Bool32,
    pub shader_uniform_buffer_array_non_uniform_indexing: Bool32,
    pub shader_sampled_image_array_non_uniform_indexing: Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing: Bool32,
    pub shader_storage_image_array_non_uniform_indexing: Bool32,
    pub shader_input_attachment_array_non_uniform_indexing: Bool32,
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32,
    pub shader_storage_texel_buffer_array_non_uniform_indexing: Bool32,
    pub descriptor_binding_uniform_buffer_update_after_bind: Bool32,
    pub descriptor_binding_sampled_image_update_after_bind: Bool32,
    pub descriptor_binding_storage_image_update_after_bind: Bool32,
    pub descriptor_binding_storage_buffer_update_after_bind: Bool32,
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32,
    pub descriptor_binding_storage_texel_buffer_update_after_bind: Bool32,
    pub descriptor_binding_update_unused_while_pending: Bool32,
    pub descriptor_binding_partially_bound: Bool32,
    pub descriptor_binding_variable_descriptor_count: Bool32,
    pub runtime_descriptor_array: Bool32,
}
pub type PhysicalDeviceDescriptorIndexingFeaturesEXT = PhysicalDeviceDescriptorIndexingFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDescriptorIndexingProperties.html>
#[doc(alias = "VkPhysicalDeviceDescriptorIndexingProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDescriptorIndexingProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_update_after_bind_descriptors_in_all_pools: u32,
    pub shader_uniform_buffer_array_non_uniform_indexing_native: Bool32,
    pub shader_sampled_image_array_non_uniform_indexing_native: Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing_native: Bool32,
    pub shader_storage_image_array_non_uniform_indexing_native: Bool32,
    pub shader_input_attachment_array_non_uniform_indexing_native: Bool32,
    pub robust_buffer_access_update_after_bind: Bool32,
    pub quad_divergent_implicit_lod: Bool32,
    pub max_per_stage_descriptor_update_after_bind_samplers: u32,
    pub max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
    pub max_per_stage_descriptor_update_after_bind_sampled_images: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_images: u32,
    pub max_per_stage_descriptor_update_after_bind_input_attachments: u32,
    pub max_per_stage_update_after_bind_resources: u32,
    pub max_descriptor_set_update_after_bind_samplers: u32,
    pub max_descriptor_set_update_after_bind_uniform_buffers: u32,
    pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_storage_buffers: u32,
    pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_sampled_images: u32,
    pub max_descriptor_set_update_after_bind_storage_images: u32,
    pub max_descriptor_set_update_after_bind_input_attachments: u32,
}
pub type PhysicalDeviceDescriptorIndexingPropertiesEXT = PhysicalDeviceDescriptorIndexingProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetVariableDescriptorCountAllocateInfo.html>
#[doc(alias = "VkDescriptorSetVariableDescriptorCountAllocateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorSetVariableDescriptorCountAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub descriptor_set_count: u32,
    pub p_descriptor_counts: *const u32,
}
pub type DescriptorSetVariableDescriptorCountAllocateInfoEXT =
    DescriptorSetVariableDescriptorCountAllocateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetVariableDescriptorCountLayoutSupport.html>
#[doc(alias = "VkDescriptorSetVariableDescriptorCountLayoutSupport")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorSetVariableDescriptorCountLayoutSupport {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_variable_descriptor_count: u32,
}
pub type DescriptorSetVariableDescriptorCountLayoutSupportEXT =
    DescriptorSetVariableDescriptorCountLayoutSupport;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceScalarBlockLayoutFeatures.html>
#[doc(alias = "VkPhysicalDeviceScalarBlockLayoutFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceScalarBlockLayoutFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub scalar_block_layout: Bool32,
}
pub type PhysicalDeviceScalarBlockLayoutFeaturesEXT = PhysicalDeviceScalarBlockLayoutFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerReductionModeCreateInfo.html>
#[doc(alias = "VkSamplerReductionModeCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SamplerReductionModeCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub reduction_mode: SamplerReductionMode,
}
pub type SamplerReductionModeCreateInfoEXT = SamplerReductionModeCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSamplerFilterMinmaxProperties.html>
#[doc(alias = "VkPhysicalDeviceSamplerFilterMinmaxProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSamplerFilterMinmaxProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub filter_minmax_single_component_formats: Bool32,
    pub filter_minmax_image_component_mapping: Bool32,
}
pub type PhysicalDeviceSamplerFilterMinmaxPropertiesEXT =
    PhysicalDeviceSamplerFilterMinmaxProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceUniformBufferStandardLayoutFeatures.html>
#[doc(alias = "VkPhysicalDeviceUniformBufferStandardLayoutFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceUniformBufferStandardLayoutFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub uniform_buffer_standard_layout: Bool32,
}
pub type PhysicalDeviceUniformBufferStandardLayoutFeaturesKHR =
    PhysicalDeviceUniformBufferStandardLayoutFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures.html>
#[doc(alias = "VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_subgroup_extended_types: Bool32,
}
pub type PhysicalDeviceShaderSubgroupExtendedTypesFeaturesKHR =
    PhysicalDeviceShaderSubgroupExtendedTypesFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentDescription2.html>
#[doc(alias = "VkAttachmentDescription2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AttachmentDescription2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: AttachmentDescriptionFlags,
    pub format: Format,
    pub samples: SampleCountFlags,
    pub load_op: AttachmentLoadOp,
    pub store_op: AttachmentStoreOp,
    pub stencil_load_op: AttachmentLoadOp,
    pub stencil_store_op: AttachmentStoreOp,
    pub initial_layout: ImageLayout,
    pub final_layout: ImageLayout,
}
pub type AttachmentDescription2KHR = AttachmentDescription2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentReference2.html>
#[doc(alias = "VkAttachmentReference2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AttachmentReference2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub attachment: u32,
    pub layout: ImageLayout,
    pub aspect_mask: ImageAspectFlags,
}
pub type AttachmentReference2KHR = AttachmentReference2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassDescription2.html>
#[doc(alias = "VkSubpassDescription2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SubpassDescription2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SubpassDescriptionFlags,
    pub pipeline_bind_point: PipelineBindPoint,
    pub view_mask: u32,
    pub input_attachment_count: u32,
    pub p_input_attachments: *const AttachmentReference2,
    pub color_attachment_count: u32,
    pub p_color_attachments: *const AttachmentReference2,
    pub p_resolve_attachments: *const AttachmentReference2,
    pub p_depth_stencil_attachment: *const AttachmentReference2,
    pub preserve_attachment_count: u32,
    pub p_preserve_attachments: *const u32,
}
pub type SubpassDescription2KHR = SubpassDescription2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassDependency2.html>
#[doc(alias = "VkSubpassDependency2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SubpassDependency2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: PipelineStageFlags,
    pub dst_stage_mask: PipelineStageFlags,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub dependency_flags: DependencyFlags,
    pub view_offset: i32,
}
pub type SubpassDependency2KHR = SubpassDependency2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassBeginInfo.html>
#[doc(alias = "VkSubpassBeginInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SubpassBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub contents: SubpassContents,
}
pub type SubpassBeginInfoKHR = SubpassBeginInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassEndInfo.html>
#[doc(alias = "VkSubpassEndInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SubpassEndInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
}
pub type SubpassEndInfoKHR = SubpassEndInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassCreateInfo2.html>
#[doc(alias = "VkRenderPassCreateInfo2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderPassCreateInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: RenderPassCreateFlags,
    pub attachment_count: u32,
    pub p_attachments: *const AttachmentDescription2,
    pub subpass_count: u32,
    pub p_subpasses: *const SubpassDescription2,
    pub dependency_count: u32,
    pub p_dependencies: *const SubpassDependency2,
    pub correlated_view_mask_count: u32,
    pub p_correlated_view_masks: *const u32,
}
pub type RenderPassCreateInfo2KHR = RenderPassCreateInfo2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassDescriptionDepthStencilResolve.html>
#[doc(alias = "VkSubpassDescriptionDepthStencilResolve")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SubpassDescriptionDepthStencilResolve {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub depth_resolve_mode: ResolveModeFlags,
    pub stencil_resolve_mode: ResolveModeFlags,
    pub p_depth_stencil_resolve_attachment: *const AttachmentReference2,
}
pub type SubpassDescriptionDepthStencilResolveKHR = SubpassDescriptionDepthStencilResolve;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDepthStencilResolveProperties.html>
#[doc(alias = "VkPhysicalDeviceDepthStencilResolveProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDepthStencilResolveProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supported_depth_resolve_modes: ResolveModeFlags,
    pub supported_stencil_resolve_modes: ResolveModeFlags,
    pub independent_resolve_none: Bool32,
    pub independent_resolve: Bool32,
}
pub type PhysicalDeviceDepthStencilResolvePropertiesKHR =
    PhysicalDeviceDepthStencilResolveProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageStencilUsageCreateInfo.html>
#[doc(alias = "VkImageStencilUsageCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageStencilUsageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stencil_usage: ImageUsageFlags,
}
pub type ImageStencilUsageCreateInfoEXT = ImageStencilUsageCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImagelessFramebufferFeatures.html>
#[doc(alias = "VkPhysicalDeviceImagelessFramebufferFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImagelessFramebufferFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub imageless_framebuffer: Bool32,
}
pub type PhysicalDeviceImagelessFramebufferFeaturesKHR = PhysicalDeviceImagelessFramebufferFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebufferAttachmentImageInfo.html>
#[doc(alias = "VkFramebufferAttachmentImageInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FramebufferAttachmentImageInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ImageCreateFlags,
    pub usage: ImageUsageFlags,
    pub width: u32,
    pub height: u32,
    pub layer_count: u32,
    pub view_format_count: u32,
    pub p_view_formats: *const Format,
}
pub type FramebufferAttachmentImageInfoKHR = FramebufferAttachmentImageInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassAttachmentBeginInfo.html>
#[doc(alias = "VkRenderPassAttachmentBeginInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderPassAttachmentBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub attachment_count: u32,
    pub p_attachments: *const ImageView,
}
pub type RenderPassAttachmentBeginInfoKHR = RenderPassAttachmentBeginInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebufferAttachmentsCreateInfo.html>
#[doc(alias = "VkFramebufferAttachmentsCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FramebufferAttachmentsCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub attachment_image_info_count: u32,
    pub p_attachment_image_infos: *const FramebufferAttachmentImageInfo,
}
pub type FramebufferAttachmentsCreateInfoKHR = FramebufferAttachmentsCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures.html>
#[doc(alias = "VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub separate_depth_stencil_layouts: Bool32,
}
pub type PhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR =
    PhysicalDeviceSeparateDepthStencilLayoutsFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentReferenceStencilLayout.html>
#[doc(alias = "VkAttachmentReferenceStencilLayout")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AttachmentReferenceStencilLayout {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub stencil_layout: ImageLayout,
}
pub type AttachmentReferenceStencilLayoutKHR = AttachmentReferenceStencilLayout;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentDescriptionStencilLayout.html>
#[doc(alias = "VkAttachmentDescriptionStencilLayout")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AttachmentDescriptionStencilLayout {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub stencil_initial_layout: ImageLayout,
    pub stencil_final_layout: ImageLayout,
}
pub type AttachmentDescriptionStencilLayoutKHR = AttachmentDescriptionStencilLayout;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkan13Features.html>
#[doc(alias = "VkPhysicalDeviceVulkan13Features")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVulkan13Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub robust_image_access: Bool32,
    pub inline_uniform_block: Bool32,
    pub descriptor_binding_inline_uniform_block_update_after_bind: Bool32,
    pub pipeline_creation_cache_control: Bool32,
    pub private_data: Bool32,
    pub shader_demote_to_helper_invocation: Bool32,
    pub shader_terminate_invocation: Bool32,
    pub subgroup_size_control: Bool32,
    pub compute_full_subgroups: Bool32,
    pub synchronization_2: Bool32,
    pub texture_compression_astc_hdr: Bool32,
    pub shader_zero_initialize_workgroup_memory: Bool32,
    pub dynamic_rendering: Bool32,
    pub shader_integer_dot_product: Bool32,
    pub maintenance_4: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkan13Properties.html>
#[doc(alias = "VkPhysicalDeviceVulkan13Properties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVulkan13Properties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_subgroup_size: u32,
    pub max_subgroup_size: u32,
    pub max_compute_workgroup_subgroups: u32,
    pub required_subgroup_size_stages: ShaderStageFlags,
    pub max_inline_uniform_block_size: u32,
    pub max_per_stage_descriptor_inline_uniform_blocks: u32,
    pub max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
    pub max_descriptor_set_inline_uniform_blocks: u32,
    pub max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
    pub max_inline_uniform_total_size: u32,
    pub integer_dot_product_8_bit_unsigned_accelerated: Bool32,
    pub integer_dot_product_8_bit_signed_accelerated: Bool32,
    pub integer_dot_product_8_bit_mixed_signedness_accelerated: Bool32,
    pub integer_dot_product_4_x_8_bit_packed_unsigned_accelerated: Bool32,
    pub integer_dot_product_4_x_8_bit_packed_signed_accelerated: Bool32,
    pub integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated: Bool32,
    pub integer_dot_product_16_bit_unsigned_accelerated: Bool32,
    pub integer_dot_product_16_bit_signed_accelerated: Bool32,
    pub integer_dot_product_16_bit_mixed_signedness_accelerated: Bool32,
    pub integer_dot_product_32_bit_unsigned_accelerated: Bool32,
    pub integer_dot_product_32_bit_signed_accelerated: Bool32,
    pub integer_dot_product_32_bit_mixed_signedness_accelerated: Bool32,
    pub integer_dot_product_64_bit_unsigned_accelerated: Bool32,
    pub integer_dot_product_64_bit_signed_accelerated: Bool32,
    pub integer_dot_product_64_bit_mixed_signedness_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_8_bit_signed_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated:
        Bool32,
    pub integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_16_bit_signed_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_32_bit_signed_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_64_bit_signed_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated: Bool32,
    pub storage_texel_buffer_offset_alignment_bytes: DeviceSize,
    pub storage_texel_buffer_offset_single_texel_alignment: Bool32,
    pub uniform_texel_buffer_offset_alignment_bytes: DeviceSize,
    pub uniform_texel_buffer_offset_single_texel_alignment: Bool32,
    pub max_buffer_size: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceToolProperties.html>
#[doc(alias = "VkPhysicalDeviceToolProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceToolProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub name: [c_char; 256],
    pub version: [c_char; 256],
    pub purposes: ToolPurposeFlags,
    pub description: [c_char; 256],
    pub layer: [c_char; 256],
}
pub type PhysicalDeviceToolPropertiesEXT = PhysicalDeviceToolProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePrivateDataFeatures.html>
#[doc(alias = "VkPhysicalDevicePrivateDataFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePrivateDataFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub private_data: Bool32,
}
pub type PhysicalDevicePrivateDataFeaturesEXT = PhysicalDevicePrivateDataFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDevicePrivateDataCreateInfo.html>
#[doc(alias = "VkDevicePrivateDataCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DevicePrivateDataCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub private_data_slot_request_count: u32,
}
pub type DevicePrivateDataCreateInfoEXT = DevicePrivateDataCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlotCreateInfo.html>
#[doc(alias = "VkPrivateDataSlotCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PrivateDataSlotCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PrivateDataSlotCreateFlags,
}
pub type PrivateDataSlotCreateInfoEXT = PrivateDataSlotCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryBarrier2.html>
#[doc(alias = "VkMemoryBarrier2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryBarrier2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_stage_mask: PipelineStageFlags2,
    pub src_access_mask: AccessFlags2,
    pub dst_stage_mask: PipelineStageFlags2,
    pub dst_access_mask: AccessFlags2,
}
pub type MemoryBarrier2KHR = MemoryBarrier2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferMemoryBarrier2.html>
#[doc(alias = "VkBufferMemoryBarrier2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BufferMemoryBarrier2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_stage_mask: PipelineStageFlags2,
    pub src_access_mask: AccessFlags2,
    pub dst_stage_mask: PipelineStageFlags2,
    pub dst_access_mask: AccessFlags2,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}
pub type BufferMemoryBarrier2KHR = BufferMemoryBarrier2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageMemoryBarrier2.html>
#[doc(alias = "VkImageMemoryBarrier2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageMemoryBarrier2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_stage_mask: PipelineStageFlags2,
    pub src_access_mask: AccessFlags2,
    pub dst_stage_mask: PipelineStageFlags2,
    pub dst_access_mask: AccessFlags2,
    pub old_layout: ImageLayout,
    pub new_layout: ImageLayout,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub image: Image,
    pub subresource_range: ImageSubresourceRange,
}
pub type ImageMemoryBarrier2KHR = ImageMemoryBarrier2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDependencyInfo.html>
#[doc(alias = "VkDependencyInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DependencyInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dependency_flags: DependencyFlags,
    pub memory_barrier_count: u32,
    pub p_memory_barriers: *const MemoryBarrier2,
    pub buffer_memory_barrier_count: u32,
    pub p_buffer_memory_barriers: *const BufferMemoryBarrier2,
    pub image_memory_barrier_count: u32,
    pub p_image_memory_barriers: *const ImageMemoryBarrier2,
}
pub type DependencyInfoKHR = DependencyInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreSubmitInfo.html>
#[doc(alias = "VkSemaphoreSubmitInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SemaphoreSubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub value: u64,
    pub stage_mask: PipelineStageFlags2,
    pub device_index: u32,
}
pub type SemaphoreSubmitInfoKHR = SemaphoreSubmitInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferSubmitInfo.html>
#[doc(alias = "VkCommandBufferSubmitInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CommandBufferSubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub command_buffer: CommandBuffer,
    pub device_mask: u32,
}
pub type CommandBufferSubmitInfoKHR = CommandBufferSubmitInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubmitInfo2.html>
#[doc(alias = "VkSubmitInfo2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SubmitInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SubmitFlags,
    pub wait_semaphore_info_count: u32,
    pub p_wait_semaphore_infos: *const SemaphoreSubmitInfo,
    pub command_buffer_info_count: u32,
    pub p_command_buffer_infos: *const CommandBufferSubmitInfo,
    pub signal_semaphore_info_count: u32,
    pub p_signal_semaphore_infos: *const SemaphoreSubmitInfo,
}
pub type SubmitInfo2KHR = SubmitInfo2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSynchronization2Features.html>
#[doc(alias = "VkPhysicalDeviceSynchronization2Features")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSynchronization2Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub synchronization_2: Bool32,
}
pub type PhysicalDeviceSynchronization2FeaturesKHR = PhysicalDeviceSynchronization2Features;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCopy2.html>
#[doc(alias = "VkBufferCopy2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BufferCopy2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_offset: DeviceSize,
    pub dst_offset: DeviceSize,
    pub size: DeviceSize,
}
pub type BufferCopy2KHR = BufferCopy2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyBufferInfo2.html>
#[doc(alias = "VkCopyBufferInfo2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CopyBufferInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_buffer: Buffer,
    pub dst_buffer: Buffer,
    pub region_count: u32,
    pub p_regions: *const BufferCopy2,
}
pub type CopyBufferInfo2KHR = CopyBufferInfo2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCopy2.html>
#[doc(alias = "VkImageCopy2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageCopy2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3D,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}
pub type ImageCopy2KHR = ImageCopy2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyImageInfo2.html>
#[doc(alias = "VkCopyImageInfo2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CopyImageInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_image: Image,
    pub src_image_layout: ImageLayout,
    pub dst_image: Image,
    pub dst_image_layout: ImageLayout,
    pub region_count: u32,
    pub p_regions: *const ImageCopy2,
}
pub type CopyImageInfo2KHR = CopyImageInfo2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferImageCopy2.html>
#[doc(alias = "VkBufferImageCopy2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BufferImageCopy2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer_offset: DeviceSize,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: Offset3D,
    pub image_extent: Extent3D,
}
pub type BufferImageCopy2KHR = BufferImageCopy2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyBufferToImageInfo2.html>
#[doc(alias = "VkCopyBufferToImageInfo2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CopyBufferToImageInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_buffer: Buffer,
    pub dst_image: Image,
    pub dst_image_layout: ImageLayout,
    pub region_count: u32,
    pub p_regions: *const BufferImageCopy2,
}
pub type CopyBufferToImageInfo2KHR = CopyBufferToImageInfo2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyImageToBufferInfo2.html>
#[doc(alias = "VkCopyImageToBufferInfo2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CopyImageToBufferInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_image: Image,
    pub src_image_layout: ImageLayout,
    pub dst_buffer: Buffer,
    pub region_count: u32,
    pub p_regions: *const BufferImageCopy2,
}
pub type CopyImageToBufferInfo2KHR = CopyImageToBufferInfo2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTextureCompressionASTCHDRFeatures.html>
#[doc(alias = "VkPhysicalDeviceTextureCompressionASTCHDRFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTextureCompressionASTCHDRFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub texture_compression_astc_hdr: Bool32,
}
pub type PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT =
    PhysicalDeviceTextureCompressionASTCHDRFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatProperties3.html>
#[doc(alias = "VkFormatProperties3")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FormatProperties3 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub linear_tiling_features: FormatFeatureFlags2,
    pub optimal_tiling_features: FormatFeatureFlags2,
    pub buffer_features: FormatFeatureFlags2,
}
pub type FormatProperties3KHR = FormatProperties3;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance4Features.html>
#[doc(alias = "VkPhysicalDeviceMaintenance4Features")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance4Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub maintenance_4: Bool32,
}
pub type PhysicalDeviceMaintenance4FeaturesKHR = PhysicalDeviceMaintenance4Features;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance4Properties.html>
#[doc(alias = "VkPhysicalDeviceMaintenance4Properties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance4Properties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_buffer_size: DeviceSize,
}
pub type PhysicalDeviceMaintenance4PropertiesKHR = PhysicalDeviceMaintenance4Properties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceBufferMemoryRequirements.html>
#[doc(alias = "VkDeviceBufferMemoryRequirements")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceBufferMemoryRequirements {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_create_info: *const BufferCreateInfo,
}
pub type DeviceBufferMemoryRequirementsKHR = DeviceBufferMemoryRequirements;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceImageMemoryRequirements.html>
#[doc(alias = "VkDeviceImageMemoryRequirements")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceImageMemoryRequirements {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_create_info: *const ImageCreateInfo,
    pub plane_aspect: ImageAspectFlags,
}
pub type DeviceImageMemoryRequirementsKHR = DeviceImageMemoryRequirements;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreationFeedback.html>
#[doc(alias = "VkPipelineCreationFeedback")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineCreationFeedback {
    pub flags: PipelineCreationFeedbackFlags,
    pub duration: u64,
}
pub type PipelineCreationFeedbackEXT = PipelineCreationFeedback;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreationFeedbackCreateInfo.html>
#[doc(alias = "VkPipelineCreationFeedbackCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineCreationFeedbackCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_pipeline_creation_feedback: *mut PipelineCreationFeedback,
    pub pipeline_stage_creation_feedback_count: u32,
    pub p_pipeline_stage_creation_feedbacks: *mut PipelineCreationFeedback,
}
pub type PipelineCreationFeedbackCreateInfoEXT = PipelineCreationFeedbackCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderTerminateInvocationFeatures.html>
#[doc(alias = "VkPhysicalDeviceShaderTerminateInvocationFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderTerminateInvocationFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_terminate_invocation: Bool32,
}
pub type PhysicalDeviceShaderTerminateInvocationFeaturesKHR =
    PhysicalDeviceShaderTerminateInvocationFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures.html>
#[doc(alias = "VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderDemoteToHelperInvocationFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_demote_to_helper_invocation: Bool32,
}
pub type PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT =
    PhysicalDeviceShaderDemoteToHelperInvocationFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineCreationCacheControlFeatures.html>
#[doc(alias = "VkPhysicalDevicePipelineCreationCacheControlFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePipelineCreationCacheControlFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_creation_cache_control: Bool32,
}
pub type PhysicalDevicePipelineCreationCacheControlFeaturesEXT =
    PhysicalDevicePipelineCreationCacheControlFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures.html>
#[doc(alias = "VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_zero_initialize_workgroup_memory: Bool32,
}
pub type PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR =
    PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageRobustnessFeatures.html>
#[doc(alias = "VkPhysicalDeviceImageRobustnessFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageRobustnessFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub robust_image_access: Bool32,
}
pub type PhysicalDeviceImageRobustnessFeaturesEXT = PhysicalDeviceImageRobustnessFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSubgroupSizeControlFeatures.html>
#[doc(alias = "VkPhysicalDeviceSubgroupSizeControlFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSubgroupSizeControlFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub subgroup_size_control: Bool32,
    pub compute_full_subgroups: Bool32,
}
pub type PhysicalDeviceSubgroupSizeControlFeaturesEXT = PhysicalDeviceSubgroupSizeControlFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSubgroupSizeControlProperties.html>
#[doc(alias = "VkPhysicalDeviceSubgroupSizeControlProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSubgroupSizeControlProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_subgroup_size: u32,
    pub max_subgroup_size: u32,
    pub max_compute_workgroup_subgroups: u32,
    pub required_subgroup_size_stages: ShaderStageFlags,
}
pub type PhysicalDeviceSubgroupSizeControlPropertiesEXT =
    PhysicalDeviceSubgroupSizeControlProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineShaderStageRequiredSubgroupSizeCreateInfo.html>
#[doc(alias = "VkPipelineShaderStageRequiredSubgroupSizeCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineShaderStageRequiredSubgroupSizeCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub required_subgroup_size: u32,
}
pub type PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT =
    PipelineShaderStageRequiredSubgroupSizeCreateInfo;
pub type ShaderRequiredSubgroupSizeCreateInfoEXT =
    PipelineShaderStageRequiredSubgroupSizeCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceInlineUniformBlockFeatures.html>
#[doc(alias = "VkPhysicalDeviceInlineUniformBlockFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceInlineUniformBlockFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub inline_uniform_block: Bool32,
    pub descriptor_binding_inline_uniform_block_update_after_bind: Bool32,
}
pub type PhysicalDeviceInlineUniformBlockFeaturesEXT = PhysicalDeviceInlineUniformBlockFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceInlineUniformBlockProperties.html>
#[doc(alias = "VkPhysicalDeviceInlineUniformBlockProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceInlineUniformBlockProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_inline_uniform_block_size: u32,
    pub max_per_stage_descriptor_inline_uniform_blocks: u32,
    pub max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
    pub max_descriptor_set_inline_uniform_blocks: u32,
    pub max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
}
pub type PhysicalDeviceInlineUniformBlockPropertiesEXT = PhysicalDeviceInlineUniformBlockProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkWriteDescriptorSetInlineUniformBlock.html>
#[doc(alias = "VkWriteDescriptorSetInlineUniformBlock")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct WriteDescriptorSetInlineUniformBlock {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub data_size: u32,
    pub p_data: *const c_void,
}
pub type WriteDescriptorSetInlineUniformBlockEXT = WriteDescriptorSetInlineUniformBlock;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPoolInlineUniformBlockCreateInfo.html>
#[doc(alias = "VkDescriptorPoolInlineUniformBlockCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorPoolInlineUniformBlockCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_inline_uniform_block_bindings: u32,
}
pub type DescriptorPoolInlineUniformBlockCreateInfoEXT = DescriptorPoolInlineUniformBlockCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderIntegerDotProductFeatures.html>
#[doc(alias = "VkPhysicalDeviceShaderIntegerDotProductFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderIntegerDotProductFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_integer_dot_product: Bool32,
}
pub type PhysicalDeviceShaderIntegerDotProductFeaturesKHR =
    PhysicalDeviceShaderIntegerDotProductFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderIntegerDotProductProperties.html>
#[doc(alias = "VkPhysicalDeviceShaderIntegerDotProductProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderIntegerDotProductProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub integer_dot_product_8_bit_unsigned_accelerated: Bool32,
    pub integer_dot_product_8_bit_signed_accelerated: Bool32,
    pub integer_dot_product_8_bit_mixed_signedness_accelerated: Bool32,
    pub integer_dot_product_4_x_8_bit_packed_unsigned_accelerated: Bool32,
    pub integer_dot_product_4_x_8_bit_packed_signed_accelerated: Bool32,
    pub integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated: Bool32,
    pub integer_dot_product_16_bit_unsigned_accelerated: Bool32,
    pub integer_dot_product_16_bit_signed_accelerated: Bool32,
    pub integer_dot_product_16_bit_mixed_signedness_accelerated: Bool32,
    pub integer_dot_product_32_bit_unsigned_accelerated: Bool32,
    pub integer_dot_product_32_bit_signed_accelerated: Bool32,
    pub integer_dot_product_32_bit_mixed_signedness_accelerated: Bool32,
    pub integer_dot_product_64_bit_unsigned_accelerated: Bool32,
    pub integer_dot_product_64_bit_signed_accelerated: Bool32,
    pub integer_dot_product_64_bit_mixed_signedness_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_8_bit_signed_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated:
        Bool32,
    pub integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_16_bit_signed_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_32_bit_signed_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_64_bit_signed_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated: Bool32,
}
pub type PhysicalDeviceShaderIntegerDotProductPropertiesKHR =
    PhysicalDeviceShaderIntegerDotProductProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTexelBufferAlignmentProperties.html>
#[doc(alias = "VkPhysicalDeviceTexelBufferAlignmentProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTexelBufferAlignmentProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub storage_texel_buffer_offset_alignment_bytes: DeviceSize,
    pub storage_texel_buffer_offset_single_texel_alignment: Bool32,
    pub uniform_texel_buffer_offset_alignment_bytes: DeviceSize,
    pub uniform_texel_buffer_offset_single_texel_alignment: Bool32,
}
pub type PhysicalDeviceTexelBufferAlignmentPropertiesEXT =
    PhysicalDeviceTexelBufferAlignmentProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageBlit2.html>
#[doc(alias = "VkImageBlit2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageBlit2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_subresource: ImageSubresourceLayers,
    pub src_offsets: [Offset3D; 2],
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offsets: [Offset3D; 2],
}
pub type ImageBlit2KHR = ImageBlit2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBlitImageInfo2.html>
#[doc(alias = "VkBlitImageInfo2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BlitImageInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_image: Image,
    pub src_image_layout: ImageLayout,
    pub dst_image: Image,
    pub dst_image_layout: ImageLayout,
    pub region_count: u32,
    pub p_regions: *const ImageBlit2,
    pub filter: Filter,
}
pub type BlitImageInfo2KHR = BlitImageInfo2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageResolve2.html>
#[doc(alias = "VkImageResolve2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageResolve2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3D,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}
pub type ImageResolve2KHR = ImageResolve2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkResolveImageInfo2.html>
#[doc(alias = "VkResolveImageInfo2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ResolveImageInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_image: Image,
    pub src_image_layout: ImageLayout,
    pub dst_image: Image,
    pub dst_image_layout: ImageLayout,
    pub region_count: u32,
    pub p_regions: *const ImageResolve2,
}
pub type ResolveImageInfo2KHR = ResolveImageInfo2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingAttachmentInfo.html>
#[doc(alias = "VkRenderingAttachmentInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderingAttachmentInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
    pub resolve_mode: ResolveModeFlags,
    pub resolve_image_view: ImageView,
    pub resolve_image_layout: ImageLayout,
    pub load_op: AttachmentLoadOp,
    pub store_op: AttachmentStoreOp,
    pub clear_value: ClearValue,
}
pub type RenderingAttachmentInfoKHR = RenderingAttachmentInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingInfo.html>
#[doc(alias = "VkRenderingInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderingInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: RenderingFlags,
    pub render_area: Rect2D,
    pub layer_count: u32,
    pub view_mask: u32,
    pub color_attachment_count: u32,
    pub p_color_attachments: *const RenderingAttachmentInfo,
    pub p_depth_attachment: *const RenderingAttachmentInfo,
    pub p_stencil_attachment: *const RenderingAttachmentInfo,
}
pub type RenderingInfoKHR = RenderingInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRenderingCreateInfo.html>
#[doc(alias = "VkPipelineRenderingCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineRenderingCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub view_mask: u32,
    pub color_attachment_count: u32,
    pub p_color_attachment_formats: *const Format,
    pub depth_attachment_format: Format,
    pub stencil_attachment_format: Format,
}
pub type PipelineRenderingCreateInfoKHR = PipelineRenderingCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDynamicRenderingFeatures.html>
#[doc(alias = "VkPhysicalDeviceDynamicRenderingFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDynamicRenderingFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub dynamic_rendering: Bool32,
}
pub type PhysicalDeviceDynamicRenderingFeaturesKHR = PhysicalDeviceDynamicRenderingFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferInheritanceRenderingInfo.html>
#[doc(alias = "VkCommandBufferInheritanceRenderingInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CommandBufferInheritanceRenderingInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: RenderingFlags,
    pub view_mask: u32,
    pub color_attachment_count: u32,
    pub p_color_attachment_formats: *const Format,
    pub depth_attachment_format: Format,
    pub stencil_attachment_format: Format,
    pub rasterization_samples: SampleCountFlags,
}
pub type CommandBufferInheritanceRenderingInfoKHR = CommandBufferInheritanceRenderingInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkan14Features.html>
#[doc(alias = "VkPhysicalDeviceVulkan14Features")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVulkan14Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub global_priority_query: Bool32,
    pub shader_subgroup_rotate: Bool32,
    pub shader_subgroup_rotate_clustered: Bool32,
    pub shader_float_controls_2: Bool32,
    pub shader_expect_assume: Bool32,
    pub rectangular_lines: Bool32,
    pub bresenham_lines: Bool32,
    pub smooth_lines: Bool32,
    pub stippled_rectangular_lines: Bool32,
    pub stippled_bresenham_lines: Bool32,
    pub stippled_smooth_lines: Bool32,
    pub vertex_attribute_instance_rate_divisor: Bool32,
    pub vertex_attribute_instance_rate_zero_divisor: Bool32,
    pub index_type_uint_8: Bool32,
    pub dynamic_rendering_local_read: Bool32,
    pub maintenance_5: Bool32,
    pub maintenance_6: Bool32,
    pub pipeline_protected_access: Bool32,
    pub pipeline_robustness: Bool32,
    pub host_image_copy: Bool32,
    pub push_descriptor: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkan14Properties.html>
#[doc(alias = "VkPhysicalDeviceVulkan14Properties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVulkan14Properties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub line_sub_pixel_precision_bits: u32,
    pub max_vertex_attrib_divisor: u32,
    pub supports_non_zero_first_instance: Bool32,
    pub max_push_descriptors: u32,
    pub dynamic_rendering_local_read_depth_stencil_attachments: Bool32,
    pub dynamic_rendering_local_read_multisampled_attachments: Bool32,
    pub early_fragment_multisample_coverage_after_sample_counting: Bool32,
    pub early_fragment_sample_mask_test_before_sample_counting: Bool32,
    pub depth_stencil_swizzle_one_support: Bool32,
    pub polygon_mode_point_size: Bool32,
    pub non_strict_single_pixel_wide_lines_use_parallelogram: Bool32,
    pub non_strict_wide_lines_use_parallelogram: Bool32,
    pub block_texel_view_compatible_multiple_layers: Bool32,
    pub max_combined_image_sampler_descriptor_count: u32,
    pub fragment_shading_rate_clamp_combiner_inputs: Bool32,
    pub default_robustness_storage_buffers: PipelineRobustnessBufferBehavior,
    pub default_robustness_uniform_buffers: PipelineRobustnessBufferBehavior,
    pub default_robustness_vertex_inputs: PipelineRobustnessBufferBehavior,
    pub default_robustness_images: PipelineRobustnessImageBehavior,
    pub copy_src_layout_count: u32,
    pub p_copy_src_layouts: *mut ImageLayout,
    pub copy_dst_layout_count: u32,
    pub p_copy_dst_layouts: *mut ImageLayout,
    pub optimal_tiling_layout_uuid: [u8; 16],
    pub identical_memory_type_requirements: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceQueueGlobalPriorityCreateInfo.html>
#[doc(alias = "VkDeviceQueueGlobalPriorityCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceQueueGlobalPriorityCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub global_priority: QueueGlobalPriority,
}
pub type DeviceQueueGlobalPriorityCreateInfoKHR = DeviceQueueGlobalPriorityCreateInfo;
pub type DeviceQueueGlobalPriorityCreateInfoEXT = DeviceQueueGlobalPriorityCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceGlobalPriorityQueryFeatures.html>
#[doc(alias = "VkPhysicalDeviceGlobalPriorityQueryFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceGlobalPriorityQueryFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub global_priority_query: Bool32,
}
pub type PhysicalDeviceGlobalPriorityQueryFeaturesKHR = PhysicalDeviceGlobalPriorityQueryFeatures;
pub type PhysicalDeviceGlobalPriorityQueryFeaturesEXT = PhysicalDeviceGlobalPriorityQueryFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyGlobalPriorityProperties.html>
#[doc(alias = "VkQueueFamilyGlobalPriorityProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QueueFamilyGlobalPriorityProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub priority_count: u32,
    pub priorities: [QueueGlobalPriority; 16],
}
pub type QueueFamilyGlobalPriorityPropertiesKHR = QueueFamilyGlobalPriorityProperties;
pub type QueueFamilyGlobalPriorityPropertiesEXT = QueueFamilyGlobalPriorityProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceIndexTypeUint8Features.html>
#[doc(alias = "VkPhysicalDeviceIndexTypeUint8Features")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceIndexTypeUint8Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub index_type_uint_8: Bool32,
}
pub type PhysicalDeviceIndexTypeUint8FeaturesKHR = PhysicalDeviceIndexTypeUint8Features;
pub type PhysicalDeviceIndexTypeUint8FeaturesEXT = PhysicalDeviceIndexTypeUint8Features;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryMapInfo.html>
#[doc(alias = "VkMemoryMapInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryMapInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: MemoryMapFlags,
    pub memory: DeviceMemory,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}
pub type MemoryMapInfoKHR = MemoryMapInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryUnmapInfo.html>
#[doc(alias = "VkMemoryUnmapInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryUnmapInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: MemoryUnmapFlags,
    pub memory: DeviceMemory,
}
pub type MemoryUnmapInfoKHR = MemoryUnmapInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance5Features.html>
#[doc(alias = "VkPhysicalDeviceMaintenance5Features")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance5Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub maintenance_5: Bool32,
}
pub type PhysicalDeviceMaintenance5FeaturesKHR = PhysicalDeviceMaintenance5Features;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance5Properties.html>
#[doc(alias = "VkPhysicalDeviceMaintenance5Properties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance5Properties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub early_fragment_multisample_coverage_after_sample_counting: Bool32,
    pub early_fragment_sample_mask_test_before_sample_counting: Bool32,
    pub depth_stencil_swizzle_one_support: Bool32,
    pub polygon_mode_point_size: Bool32,
    pub non_strict_single_pixel_wide_lines_use_parallelogram: Bool32,
    pub non_strict_wide_lines_use_parallelogram: Bool32,
}
pub type PhysicalDeviceMaintenance5PropertiesKHR = PhysicalDeviceMaintenance5Properties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubresourceLayout2.html>
#[doc(alias = "VkSubresourceLayout2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SubresourceLayout2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub subresource_layout: SubresourceLayout,
}
pub type SubresourceLayout2KHR = SubresourceLayout2;
pub type SubresourceLayout2EXT = SubresourceLayout2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageSubresource2.html>
#[doc(alias = "VkImageSubresource2")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageSubresource2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_subresource: ImageSubresource,
}
pub type ImageSubresource2KHR = ImageSubresource2;
pub type ImageSubresource2EXT = ImageSubresource2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceImageSubresourceInfo.html>
#[doc(alias = "VkDeviceImageSubresourceInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceImageSubresourceInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_create_info: *const ImageCreateInfo,
    pub p_subresource: *const ImageSubresource2,
}
pub type DeviceImageSubresourceInfoKHR = DeviceImageSubresourceInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlags2CreateInfo.html>
#[doc(alias = "VkBufferUsageFlags2CreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BufferUsageFlags2CreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub usage: BufferUsageFlags2,
}
pub type BufferUsageFlags2CreateInfoKHR = BufferUsageFlags2CreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance6Features.html>
#[doc(alias = "VkPhysicalDeviceMaintenance6Features")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance6Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub maintenance_6: Bool32,
}
pub type PhysicalDeviceMaintenance6FeaturesKHR = PhysicalDeviceMaintenance6Features;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance6Properties.html>
#[doc(alias = "VkPhysicalDeviceMaintenance6Properties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance6Properties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub block_texel_view_compatible_multiple_layers: Bool32,
    pub max_combined_image_sampler_descriptor_count: u32,
    pub fragment_shading_rate_clamp_combiner_inputs: Bool32,
}
pub type PhysicalDeviceMaintenance6PropertiesKHR = PhysicalDeviceMaintenance6Properties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindMemoryStatus.html>
#[doc(alias = "VkBindMemoryStatus")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BindMemoryStatus {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_result: *mut ResultCode,
}
pub type BindMemoryStatusKHR = BindMemoryStatus;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceHostImageCopyFeatures.html>
#[doc(alias = "VkPhysicalDeviceHostImageCopyFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceHostImageCopyFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub host_image_copy: Bool32,
}
pub type PhysicalDeviceHostImageCopyFeaturesEXT = PhysicalDeviceHostImageCopyFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceHostImageCopyProperties.html>
#[doc(alias = "VkPhysicalDeviceHostImageCopyProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceHostImageCopyProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub copy_src_layout_count: u32,
    pub p_copy_src_layouts: *mut ImageLayout,
    pub copy_dst_layout_count: u32,
    pub p_copy_dst_layouts: *mut ImageLayout,
    pub optimal_tiling_layout_uuid: [u8; 16],
    pub identical_memory_type_requirements: Bool32,
}
pub type PhysicalDeviceHostImageCopyPropertiesEXT = PhysicalDeviceHostImageCopyProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryToImageCopy.html>
#[doc(alias = "VkMemoryToImageCopy")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryToImageCopy {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_host_pointer: *const c_void,
    pub memory_row_length: u32,
    pub memory_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: Offset3D,
    pub image_extent: Extent3D,
}
pub type MemoryToImageCopyEXT = MemoryToImageCopy;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageToMemoryCopy.html>
#[doc(alias = "VkImageToMemoryCopy")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageToMemoryCopy {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_host_pointer: *mut c_void,
    pub memory_row_length: u32,
    pub memory_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: Offset3D,
    pub image_extent: Extent3D,
}
pub type ImageToMemoryCopyEXT = ImageToMemoryCopy;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyMemoryToImageInfo.html>
#[doc(alias = "VkCopyMemoryToImageInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CopyMemoryToImageInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: HostImageCopyFlags,
    pub dst_image: Image,
    pub dst_image_layout: ImageLayout,
    pub region_count: u32,
    pub p_regions: *const MemoryToImageCopy,
}
pub type CopyMemoryToImageInfoEXT = CopyMemoryToImageInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyImageToMemoryInfo.html>
#[doc(alias = "VkCopyImageToMemoryInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CopyImageToMemoryInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: HostImageCopyFlags,
    pub src_image: Image,
    pub src_image_layout: ImageLayout,
    pub region_count: u32,
    pub p_regions: *const ImageToMemoryCopy,
}
pub type CopyImageToMemoryInfoEXT = CopyImageToMemoryInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyImageToImageInfo.html>
#[doc(alias = "VkCopyImageToImageInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CopyImageToImageInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: HostImageCopyFlags,
    pub src_image: Image,
    pub src_image_layout: ImageLayout,
    pub dst_image: Image,
    pub dst_image_layout: ImageLayout,
    pub region_count: u32,
    pub p_regions: *const ImageCopy2,
}
pub type CopyImageToImageInfoEXT = CopyImageToImageInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkHostImageLayoutTransitionInfo.html>
#[doc(alias = "VkHostImageLayoutTransitionInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct HostImageLayoutTransitionInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub old_layout: ImageLayout,
    pub new_layout: ImageLayout,
    pub subresource_range: ImageSubresourceRange,
}
pub type HostImageLayoutTransitionInfoEXT = HostImageLayoutTransitionInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubresourceHostMemcpySize.html>
#[doc(alias = "VkSubresourceHostMemcpySize")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SubresourceHostMemcpySize {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub size: DeviceSize,
}
pub type SubresourceHostMemcpySizeEXT = SubresourceHostMemcpySize;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkHostImageCopyDevicePerformanceQuery.html>
#[doc(alias = "VkHostImageCopyDevicePerformanceQuery")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct HostImageCopyDevicePerformanceQuery {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub optimal_device_access: Bool32,
    pub identical_memory_layout: Bool32,
}
pub type HostImageCopyDevicePerformanceQueryEXT = HostImageCopyDevicePerformanceQuery;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderSubgroupRotateFeatures.html>
#[doc(alias = "VkPhysicalDeviceShaderSubgroupRotateFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderSubgroupRotateFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_subgroup_rotate: Bool32,
    pub shader_subgroup_rotate_clustered: Bool32,
}
pub type PhysicalDeviceShaderSubgroupRotateFeaturesKHR = PhysicalDeviceShaderSubgroupRotateFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderFloatControls2Features.html>
#[doc(alias = "VkPhysicalDeviceShaderFloatControls2Features")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderFloatControls2Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_float_controls_2: Bool32,
}
pub type PhysicalDeviceShaderFloatControls2FeaturesKHR = PhysicalDeviceShaderFloatControls2Features;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderExpectAssumeFeatures.html>
#[doc(alias = "VkPhysicalDeviceShaderExpectAssumeFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderExpectAssumeFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_expect_assume: Bool32,
}
pub type PhysicalDeviceShaderExpectAssumeFeaturesKHR = PhysicalDeviceShaderExpectAssumeFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlags2CreateInfo.html>
#[doc(alias = "VkPipelineCreateFlags2CreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineCreateFlags2CreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags2,
}
pub type PipelineCreateFlags2CreateInfoKHR = PipelineCreateFlags2CreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePushDescriptorProperties.html>
#[doc(alias = "VkPhysicalDevicePushDescriptorProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePushDescriptorProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_push_descriptors: u32,
}
pub type PhysicalDevicePushDescriptorPropertiesKHR = PhysicalDevicePushDescriptorProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindDescriptorSetsInfo.html>
#[doc(alias = "VkBindDescriptorSetsInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BindDescriptorSetsInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stage_flags: ShaderStageFlags,
    pub layout: PipelineLayout,
    pub first_set: u32,
    pub descriptor_set_count: u32,
    pub p_descriptor_sets: *const DescriptorSet,
    pub dynamic_offset_count: u32,
    pub p_dynamic_offsets: *const u32,
}
pub type BindDescriptorSetsInfoKHR = BindDescriptorSetsInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPushConstantsInfo.html>
#[doc(alias = "VkPushConstantsInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PushConstantsInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub layout: PipelineLayout,
    pub stage_flags: ShaderStageFlags,
    pub offset: u32,
    pub size: u32,
    pub p_values: *const c_void,
}
pub type PushConstantsInfoKHR = PushConstantsInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPushDescriptorSetInfo.html>
#[doc(alias = "VkPushDescriptorSetInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PushDescriptorSetInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stage_flags: ShaderStageFlags,
    pub layout: PipelineLayout,
    pub set: u32,
    pub descriptor_write_count: u32,
    pub p_descriptor_writes: *const WriteDescriptorSet,
}
pub type PushDescriptorSetInfoKHR = PushDescriptorSetInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPushDescriptorSetWithTemplateInfo.html>
#[doc(alias = "VkPushDescriptorSetWithTemplateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PushDescriptorSetWithTemplateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub descriptor_update_template: DescriptorUpdateTemplate,
    pub layout: PipelineLayout,
    pub set: u32,
    pub p_data: *const c_void,
}
pub type PushDescriptorSetWithTemplateInfoKHR = PushDescriptorSetWithTemplateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineProtectedAccessFeatures.html>
#[doc(alias = "VkPhysicalDevicePipelineProtectedAccessFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePipelineProtectedAccessFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_protected_access: Bool32,
}
pub type PhysicalDevicePipelineProtectedAccessFeaturesEXT =
    PhysicalDevicePipelineProtectedAccessFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineRobustnessFeatures.html>
#[doc(alias = "VkPhysicalDevicePipelineRobustnessFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePipelineRobustnessFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_robustness: Bool32,
}
pub type PhysicalDevicePipelineRobustnessFeaturesEXT = PhysicalDevicePipelineRobustnessFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineRobustnessProperties.html>
#[doc(alias = "VkPhysicalDevicePipelineRobustnessProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePipelineRobustnessProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub default_robustness_storage_buffers: PipelineRobustnessBufferBehavior,
    pub default_robustness_uniform_buffers: PipelineRobustnessBufferBehavior,
    pub default_robustness_vertex_inputs: PipelineRobustnessBufferBehavior,
    pub default_robustness_images: PipelineRobustnessImageBehavior,
}
pub type PhysicalDevicePipelineRobustnessPropertiesEXT = PhysicalDevicePipelineRobustnessProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRobustnessCreateInfo.html>
#[doc(alias = "VkPipelineRobustnessCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineRobustnessCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub storage_buffers: PipelineRobustnessBufferBehavior,
    pub uniform_buffers: PipelineRobustnessBufferBehavior,
    pub vertex_inputs: PipelineRobustnessBufferBehavior,
    pub images: PipelineRobustnessImageBehavior,
}
pub type PipelineRobustnessCreateInfoEXT = PipelineRobustnessCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceLineRasterizationFeatures.html>
#[doc(alias = "VkPhysicalDeviceLineRasterizationFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceLineRasterizationFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub rectangular_lines: Bool32,
    pub bresenham_lines: Bool32,
    pub smooth_lines: Bool32,
    pub stippled_rectangular_lines: Bool32,
    pub stippled_bresenham_lines: Bool32,
    pub stippled_smooth_lines: Bool32,
}
pub type PhysicalDeviceLineRasterizationFeaturesKHR = PhysicalDeviceLineRasterizationFeatures;
pub type PhysicalDeviceLineRasterizationFeaturesEXT = PhysicalDeviceLineRasterizationFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceLineRasterizationProperties.html>
#[doc(alias = "VkPhysicalDeviceLineRasterizationProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceLineRasterizationProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub line_sub_pixel_precision_bits: u32,
}
pub type PhysicalDeviceLineRasterizationPropertiesKHR = PhysicalDeviceLineRasterizationProperties;
pub type PhysicalDeviceLineRasterizationPropertiesEXT = PhysicalDeviceLineRasterizationProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationLineStateCreateInfo.html>
#[doc(alias = "VkPipelineRasterizationLineStateCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineRasterizationLineStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub line_rasterization_mode: LineRasterizationMode,
    pub stippled_line_enable: Bool32,
    pub line_stipple_factor: u32,
    pub line_stipple_pattern: u16,
}
pub type PipelineRasterizationLineStateCreateInfoKHR = PipelineRasterizationLineStateCreateInfo;
pub type PipelineRasterizationLineStateCreateInfoEXT = PipelineRasterizationLineStateCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVertexAttributeDivisorProperties.html>
#[doc(alias = "VkPhysicalDeviceVertexAttributeDivisorProperties")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVertexAttributeDivisorProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_vertex_attrib_divisor: u32,
    pub supports_non_zero_first_instance: Bool32,
}
pub type PhysicalDeviceVertexAttributeDivisorPropertiesKHR =
    PhysicalDeviceVertexAttributeDivisorProperties;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVertexInputBindingDivisorDescription.html>
#[doc(alias = "VkVertexInputBindingDivisorDescription")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VertexInputBindingDivisorDescription {
    pub binding: u32,
    pub divisor: u32,
}
pub type VertexInputBindingDivisorDescriptionKHR = VertexInputBindingDivisorDescription;
pub type VertexInputBindingDivisorDescriptionEXT = VertexInputBindingDivisorDescription;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineVertexInputDivisorStateCreateInfo.html>
#[doc(alias = "VkPipelineVertexInputDivisorStateCreateInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineVertexInputDivisorStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vertex_binding_divisor_count: u32,
    pub p_vertex_binding_divisors: *const VertexInputBindingDivisorDescription,
}
pub type PipelineVertexInputDivisorStateCreateInfoKHR = PipelineVertexInputDivisorStateCreateInfo;
pub type PipelineVertexInputDivisorStateCreateInfoEXT = PipelineVertexInputDivisorStateCreateInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVertexAttributeDivisorFeatures.html>
#[doc(alias = "VkPhysicalDeviceVertexAttributeDivisorFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVertexAttributeDivisorFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub vertex_attribute_instance_rate_divisor: Bool32,
    pub vertex_attribute_instance_rate_zero_divisor: Bool32,
}
pub type PhysicalDeviceVertexAttributeDivisorFeaturesKHR =
    PhysicalDeviceVertexAttributeDivisorFeatures;
pub type PhysicalDeviceVertexAttributeDivisorFeaturesEXT =
    PhysicalDeviceVertexAttributeDivisorFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingAreaInfo.html>
#[doc(alias = "VkRenderingAreaInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderingAreaInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub view_mask: u32,
    pub color_attachment_count: u32,
    pub p_color_attachment_formats: *const Format,
    pub depth_attachment_format: Format,
    pub stencil_attachment_format: Format,
}
pub type RenderingAreaInfoKHR = RenderingAreaInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDynamicRenderingLocalReadFeatures.html>
#[doc(alias = "VkPhysicalDeviceDynamicRenderingLocalReadFeatures")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDynamicRenderingLocalReadFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub dynamic_rendering_local_read: Bool32,
}
pub type PhysicalDeviceDynamicRenderingLocalReadFeaturesKHR =
    PhysicalDeviceDynamicRenderingLocalReadFeatures;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingAttachmentLocationInfo.html>
#[doc(alias = "VkRenderingAttachmentLocationInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderingAttachmentLocationInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub color_attachment_count: u32,
    pub p_color_attachment_locations: *const u32,
}
pub type RenderingAttachmentLocationInfoKHR = RenderingAttachmentLocationInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingInputAttachmentIndexInfo.html>
#[doc(alias = "VkRenderingInputAttachmentIndexInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderingInputAttachmentIndexInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub color_attachment_count: u32,
    pub p_color_attachment_input_indices: *const u32,
    pub p_depth_input_attachment_index: *const u32,
    pub p_stencil_input_attachment_index: *const u32,
}
pub type RenderingInputAttachmentIndexInfoKHR = RenderingInputAttachmentIndexInfo;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCapabilitiesKHR.html>
#[doc(alias = "VkSurfaceCapabilitiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SurfaceCapabilitiesKHR {
    pub min_image_count: u32,
    pub max_image_count: u32,
    pub current_extent: Extent2D,
    pub min_image_extent: Extent2D,
    pub max_image_extent: Extent2D,
    pub max_image_array_layers: u32,
    pub supported_transforms: SurfaceTransformFlagsKHR,
    pub current_transform: SurfaceTransformFlagsKHR,
    pub supported_composite_alpha: CompositeAlphaFlagsKHR,
    pub supported_usage_flags: ImageUsageFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceFormatKHR.html>
#[doc(alias = "VkSurfaceFormatKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SurfaceFormatKHR {
    pub format: Format,
    pub color_space: ColorSpaceKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainCreateInfoKHR.html>
#[doc(alias = "VkSwapchainCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SwapchainCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SwapchainCreateFlagsKHR,
    pub surface: SurfaceKHR,
    pub min_image_count: u32,
    pub image_format: Format,
    pub image_color_space: ColorSpaceKHR,
    pub image_extent: Extent2D,
    pub image_array_layers: u32,
    pub image_usage: ImageUsageFlags,
    pub image_sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub pre_transform: SurfaceTransformFlagsKHR,
    pub composite_alpha: CompositeAlphaFlagsKHR,
    pub present_mode: PresentModeKHR,
    pub clipped: Bool32,
    pub old_swapchain: SwapchainKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentInfoKHR.html>
#[doc(alias = "VkPresentInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PresentInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const Semaphore,
    pub swapchain_count: u32,
    pub p_swapchains: *const SwapchainKHR,
    pub p_image_indices: *const u32,
    pub p_results: *mut ResultCode,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageSwapchainCreateInfoKHR.html>
#[doc(alias = "VkImageSwapchainCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageSwapchainCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain: SwapchainKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindImageMemorySwapchainInfoKHR.html>
#[doc(alias = "VkBindImageMemorySwapchainInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BindImageMemorySwapchainInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain: SwapchainKHR,
    pub image_index: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAcquireNextImageInfoKHR.html>
#[doc(alias = "VkAcquireNextImageInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AcquireNextImageInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain: SwapchainKHR,
    pub timeout: u64,
    pub semaphore: Semaphore,
    pub fence: Fence,
    pub device_mask: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupPresentCapabilitiesKHR.html>
#[doc(alias = "VkDeviceGroupPresentCapabilitiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceGroupPresentCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_mask: [u32; 32],
    pub modes: DeviceGroupPresentModeFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupPresentInfoKHR.html>
#[doc(alias = "VkDeviceGroupPresentInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceGroupPresentInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_device_masks: *const u32,
    pub mode: DeviceGroupPresentModeFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupSwapchainCreateInfoKHR.html>
#[doc(alias = "VkDeviceGroupSwapchainCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceGroupSwapchainCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub modes: DeviceGroupPresentModeFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayModeParametersKHR.html>
#[doc(alias = "VkDisplayModeParametersKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DisplayModeParametersKHR {
    pub visible_region: Extent2D,
    pub refresh_rate: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayModeCreateInfoKHR.html>
#[doc(alias = "VkDisplayModeCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DisplayModeCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DisplayModeCreateFlagsKHR,
    pub parameters: DisplayModeParametersKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayModePropertiesKHR.html>
#[doc(alias = "VkDisplayModePropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DisplayModePropertiesKHR {
    pub display_mode: DisplayModeKHR,
    pub parameters: DisplayModeParametersKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayPlaneCapabilitiesKHR.html>
#[doc(alias = "VkDisplayPlaneCapabilitiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DisplayPlaneCapabilitiesKHR {
    pub supported_alpha: DisplayPlaneAlphaFlagsKHR,
    pub min_src_position: Offset2D,
    pub max_src_position: Offset2D,
    pub min_src_extent: Extent2D,
    pub max_src_extent: Extent2D,
    pub min_dst_position: Offset2D,
    pub max_dst_position: Offset2D,
    pub min_dst_extent: Extent2D,
    pub max_dst_extent: Extent2D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayPlanePropertiesKHR.html>
#[doc(alias = "VkDisplayPlanePropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DisplayPlanePropertiesKHR {
    pub current_display: DisplayKHR,
    pub current_stack_index: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayPropertiesKHR.html>
#[doc(alias = "VkDisplayPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DisplayPropertiesKHR {
    pub display: DisplayKHR,
    pub display_name: *const c_char,
    pub physical_dimensions: Extent2D,
    pub physical_resolution: Extent2D,
    pub supported_transforms: SurfaceTransformFlagsKHR,
    pub plane_reorder_possible: Bool32,
    pub persistent_content: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplaySurfaceCreateInfoKHR.html>
#[doc(alias = "VkDisplaySurfaceCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DisplaySurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DisplaySurfaceCreateFlagsKHR,
    pub display_mode: DisplayModeKHR,
    pub plane_index: u32,
    pub plane_stack_index: u32,
    pub transform: SurfaceTransformFlagsKHR,
    pub global_alpha: f32,
    pub alpha_mode: DisplayPlaneAlphaFlagsKHR,
    pub image_extent: Extent2D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayPresentInfoKHR.html>
#[doc(alias = "VkDisplayPresentInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DisplayPresentInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_rect: Rect2D,
    pub dst_rect: Rect2D,
    pub persistent: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkXlibSurfaceCreateInfoKHR.html>
#[doc(alias = "VkXlibSurfaceCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XlibSurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: XlibSurfaceCreateFlagsKHR,
    pub dpy: *mut Display,
    pub window: Window,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkXcbSurfaceCreateInfoKHR.html>
#[doc(alias = "VkXcbSurfaceCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XcbSurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: XcbSurfaceCreateFlagsKHR,
    pub connection: *mut xcb_connection_t,
    pub window: xcb_window_t,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkWaylandSurfaceCreateInfoKHR.html>
#[doc(alias = "VkWaylandSurfaceCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct WaylandSurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: WaylandSurfaceCreateFlagsKHR,
    pub display: *mut wl_display,
    pub surface: *mut wl_surface,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAndroidSurfaceCreateInfoKHR.html>
#[doc(alias = "VkAndroidSurfaceCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AndroidSurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: AndroidSurfaceCreateFlagsKHR,
    pub window: *mut ANativeWindow,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkWin32SurfaceCreateInfoKHR.html>
#[doc(alias = "VkWin32SurfaceCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Win32SurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: Win32SurfaceCreateFlagsKHR,
    pub hinstance: HINSTANCE,
    pub hwnd: HWND,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyQueryResultStatusPropertiesKHR.html>
#[doc(alias = "VkQueueFamilyQueryResultStatusPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QueueFamilyQueryResultStatusPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub query_result_status_support: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyVideoPropertiesKHR.html>
#[doc(alias = "VkQueueFamilyVideoPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QueueFamilyVideoPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_codec_operations: VideoCodecOperationFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoProfileInfoKHR.html>
#[doc(alias = "VkVideoProfileInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoProfileInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub video_codec_operation: VideoCodecOperationFlagsKHR,
    pub chroma_subsampling: VideoChromaSubsamplingFlagsKHR,
    pub luma_bit_depth: VideoComponentBitDepthFlagsKHR,
    pub chroma_bit_depth: VideoComponentBitDepthFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoProfileListInfoKHR.html>
#[doc(alias = "VkVideoProfileListInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoProfileListInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub profile_count: u32,
    pub p_profiles: *const VideoProfileInfoKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoCapabilitiesKHR.html>
#[doc(alias = "VkVideoCapabilitiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: VideoCapabilityFlagsKHR,
    pub min_bitstream_buffer_offset_alignment: DeviceSize,
    pub min_bitstream_buffer_size_alignment: DeviceSize,
    pub picture_access_granularity: Extent2D,
    pub min_coded_extent: Extent2D,
    pub max_coded_extent: Extent2D,
    pub max_dpb_slots: u32,
    pub max_active_reference_pictures: u32,
    pub std_header_version: ExtensionProperties,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVideoFormatInfoKHR.html>
#[doc(alias = "VkPhysicalDeviceVideoFormatInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVideoFormatInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_usage: ImageUsageFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoFormatPropertiesKHR.html>
#[doc(alias = "VkVideoFormatPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoFormatPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format: Format,
    pub component_mapping: ComponentMapping,
    pub image_create_flags: ImageCreateFlags,
    pub image_type: ImageType,
    pub image_tiling: ImageTiling,
    pub image_usage_flags: ImageUsageFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoPictureResourceInfoKHR.html>
#[doc(alias = "VkVideoPictureResourceInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoPictureResourceInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub coded_offset: Offset2D,
    pub coded_extent: Extent2D,
    pub base_array_layer: u32,
    pub image_view_binding: ImageView,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoReferenceSlotInfoKHR.html>
#[doc(alias = "VkVideoReferenceSlotInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoReferenceSlotInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub slot_index: i32,
    pub p_picture_resource: *const VideoPictureResourceInfoKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionMemoryRequirementsKHR.html>
#[doc(alias = "VkVideoSessionMemoryRequirementsKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoSessionMemoryRequirementsKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_bind_index: u32,
    pub memory_requirements: MemoryRequirements,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindVideoSessionMemoryInfoKHR.html>
#[doc(alias = "VkBindVideoSessionMemoryInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BindVideoSessionMemoryInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory_bind_index: u32,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub memory_size: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionCreateInfoKHR.html>
#[doc(alias = "VkVideoSessionCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoSessionCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub queue_family_index: u32,
    pub flags: VideoSessionCreateFlagsKHR,
    pub p_video_profile: *const VideoProfileInfoKHR,
    pub picture_format: Format,
    pub max_coded_extent: Extent2D,
    pub reference_picture_format: Format,
    pub max_dpb_slots: u32,
    pub max_active_reference_pictures: u32,
    pub p_std_header_version: *const ExtensionProperties,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionParametersCreateInfoKHR.html>
#[doc(alias = "VkVideoSessionParametersCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoSessionParametersCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoSessionParametersCreateFlagsKHR,
    pub video_session_parameters_template: VideoSessionParametersKHR,
    pub video_session: VideoSessionKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionParametersUpdateInfoKHR.html>
#[doc(alias = "VkVideoSessionParametersUpdateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoSessionParametersUpdateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub update_sequence_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoBeginCodingInfoKHR.html>
#[doc(alias = "VkVideoBeginCodingInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoBeginCodingInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoBeginCodingFlagsKHR,
    pub video_session: VideoSessionKHR,
    pub video_session_parameters: VideoSessionParametersKHR,
    pub reference_slot_count: u32,
    pub p_reference_slots: *const VideoReferenceSlotInfoKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEndCodingInfoKHR.html>
#[doc(alias = "VkVideoEndCodingInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEndCodingInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoEndCodingFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoCodingControlInfoKHR.html>
#[doc(alias = "VkVideoCodingControlInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoCodingControlInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoCodingControlFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeCapabilitiesKHR.html>
#[doc(alias = "VkVideoDecodeCapabilitiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: VideoDecodeCapabilityFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeUsageInfoKHR.html>
#[doc(alias = "VkVideoDecodeUsageInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeUsageInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub video_usage_hints: VideoDecodeUsageFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeInfoKHR.html>
#[doc(alias = "VkVideoDecodeInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoDecodeFlagsKHR,
    pub src_buffer: Buffer,
    pub src_buffer_offset: DeviceSize,
    pub src_buffer_range: DeviceSize,
    pub dst_picture_resource: VideoPictureResourceInfoKHR,
    pub p_setup_reference_slot: *const VideoReferenceSlotInfoKHR,
    pub reference_slot_count: u32,
    pub p_reference_slots: *const VideoReferenceSlotInfoKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264CapabilitiesKHR.html>
#[doc(alias = "VkVideoEncodeH264CapabilitiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH264CapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: VideoEncodeH264CapabilityFlagsKHR,
    pub max_level_idc: H264LevelIdc,
    pub max_slice_count: u32,
    pub max_p_picture_l_0_reference_count: u32,
    pub max_b_picture_l_0_reference_count: u32,
    pub max_l_1_reference_count: u32,
    pub max_temporal_layer_count: u32,
    pub expect_dyadic_temporal_layer_pattern: Bool32,
    pub min_qp: i32,
    pub max_qp: i32,
    pub prefers_gop_remaining_frames: Bool32,
    pub requires_gop_remaining_frames: Bool32,
    pub std_syntax_flags: VideoEncodeH264StdFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264QpKHR.html>
#[doc(alias = "VkVideoEncodeH264QpKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH264QpKHR {
    pub qp_i: i32,
    pub qp_p: i32,
    pub qp_b: i32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264QualityLevelPropertiesKHR.html>
#[doc(alias = "VkVideoEncodeH264QualityLevelPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH264QualityLevelPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub preferred_rate_control_flags: VideoEncodeH264RateControlFlagsKHR,
    pub preferred_gop_frame_count: u32,
    pub preferred_idr_period: u32,
    pub preferred_consecutive_b_frame_count: u32,
    pub preferred_temporal_layer_count: u32,
    pub preferred_constant_qp: VideoEncodeH264QpKHR,
    pub preferred_max_l_0_reference_count: u32,
    pub preferred_max_l_1_reference_count: u32,
    pub preferred_std_entropy_coding_mode_flag: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264SessionCreateInfoKHR.html>
#[doc(alias = "VkVideoEncodeH264SessionCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH264SessionCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_max_level_idc: Bool32,
    pub max_level_idc: H264LevelIdc,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264SessionParametersAddInfoKHR.html>
#[doc(alias = "VkVideoEncodeH264SessionParametersAddInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH264SessionParametersAddInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_sps_count: u32,
    pub p_std_sp_ss: *const H264SequenceParameterSet,
    pub std_pps_count: u32,
    pub p_std_pp_ss: *const H264PictureParameterSet,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264SessionParametersCreateInfoKHR.html>
#[doc(alias = "VkVideoEncodeH264SessionParametersCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH264SessionParametersCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_std_sps_count: u32,
    pub max_std_pps_count: u32,
    pub p_parameters_add_info: *const VideoEncodeH264SessionParametersAddInfoKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264SessionParametersGetInfoKHR.html>
#[doc(alias = "VkVideoEncodeH264SessionParametersGetInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH264SessionParametersGetInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub write_std_sps: Bool32,
    pub write_std_pps: Bool32,
    pub std_sps_id: u32,
    pub std_pps_id: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264SessionParametersFeedbackInfoKHR.html>
#[doc(alias = "VkVideoEncodeH264SessionParametersFeedbackInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH264SessionParametersFeedbackInfoKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub has_std_sps_overrides: Bool32,
    pub has_std_pps_overrides: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264NaluSliceInfoKHR.html>
#[doc(alias = "VkVideoEncodeH264NaluSliceInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH264NaluSliceInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub constant_qp: i32,
    pub p_std_slice_header: *const EncodeH264SliceHeader,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264PictureInfoKHR.html>
#[doc(alias = "VkVideoEncodeH264PictureInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH264PictureInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub nalu_slice_entry_count: u32,
    pub p_nalu_slice_entries: *const VideoEncodeH264NaluSliceInfoKHR,
    pub p_std_picture_info: *const EncodeH264PictureInfo,
    pub generate_prefix_nalu: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264DpbSlotInfoKHR.html>
#[doc(alias = "VkVideoEncodeH264DpbSlotInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH264DpbSlotInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_reference_info: *const EncodeH264ReferenceInfo,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264ProfileInfoKHR.html>
#[doc(alias = "VkVideoEncodeH264ProfileInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH264ProfileInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_profile_idc: H264ProfileIdc,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264RateControlInfoKHR.html>
#[doc(alias = "VkVideoEncodeH264RateControlInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH264RateControlInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoEncodeH264RateControlFlagsKHR,
    pub gop_frame_count: u32,
    pub idr_period: u32,
    pub consecutive_b_frame_count: u32,
    pub temporal_layer_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264FrameSizeKHR.html>
#[doc(alias = "VkVideoEncodeH264FrameSizeKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH264FrameSizeKHR {
    pub frame_i_size: u32,
    pub frame_p_size: u32,
    pub frame_b_size: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264RateControlLayerInfoKHR.html>
#[doc(alias = "VkVideoEncodeH264RateControlLayerInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH264RateControlLayerInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_min_qp: Bool32,
    pub min_qp: VideoEncodeH264QpKHR,
    pub use_max_qp: Bool32,
    pub max_qp: VideoEncodeH264QpKHR,
    pub use_max_frame_size: Bool32,
    pub max_frame_size: VideoEncodeH264FrameSizeKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264GopRemainingFrameInfoKHR.html>
#[doc(alias = "VkVideoEncodeH264GopRemainingFrameInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH264GopRemainingFrameInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_gop_remaining_frames: Bool32,
    pub gop_remaining_i: u32,
    pub gop_remaining_p: u32,
    pub gop_remaining_b: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265CapabilitiesKHR.html>
#[doc(alias = "VkVideoEncodeH265CapabilitiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH265CapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: VideoEncodeH265CapabilityFlagsKHR,
    pub max_level_idc: H265LevelIdc,
    pub max_slice_segment_count: u32,
    pub max_tiles: Extent2D,
    pub ctb_sizes: VideoEncodeH265CtbSizeFlagsKHR,
    pub transform_block_sizes: VideoEncodeH265TransformBlockSizeFlagsKHR,
    pub max_p_picture_l_0_reference_count: u32,
    pub max_b_picture_l_0_reference_count: u32,
    pub max_l_1_reference_count: u32,
    pub max_sub_layer_count: u32,
    pub expect_dyadic_temporal_sub_layer_pattern: Bool32,
    pub min_qp: i32,
    pub max_qp: i32,
    pub prefers_gop_remaining_frames: Bool32,
    pub requires_gop_remaining_frames: Bool32,
    pub std_syntax_flags: VideoEncodeH265StdFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265SessionCreateInfoKHR.html>
#[doc(alias = "VkVideoEncodeH265SessionCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH265SessionCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_max_level_idc: Bool32,
    pub max_level_idc: H265LevelIdc,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265QpKHR.html>
#[doc(alias = "VkVideoEncodeH265QpKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH265QpKHR {
    pub qp_i: i32,
    pub qp_p: i32,
    pub qp_b: i32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265QualityLevelPropertiesKHR.html>
#[doc(alias = "VkVideoEncodeH265QualityLevelPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH265QualityLevelPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub preferred_rate_control_flags: VideoEncodeH265RateControlFlagsKHR,
    pub preferred_gop_frame_count: u32,
    pub preferred_idr_period: u32,
    pub preferred_consecutive_b_frame_count: u32,
    pub preferred_sub_layer_count: u32,
    pub preferred_constant_qp: VideoEncodeH265QpKHR,
    pub preferred_max_l_0_reference_count: u32,
    pub preferred_max_l_1_reference_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265SessionParametersAddInfoKHR.html>
#[doc(alias = "VkVideoEncodeH265SessionParametersAddInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH265SessionParametersAddInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_vps_count: u32,
    pub p_std_vp_ss: *const H265VideoParameterSet,
    pub std_sps_count: u32,
    pub p_std_sp_ss: *const H265SequenceParameterSet,
    pub std_pps_count: u32,
    pub p_std_pp_ss: *const H265PictureParameterSet,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265SessionParametersCreateInfoKHR.html>
#[doc(alias = "VkVideoEncodeH265SessionParametersCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH265SessionParametersCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_std_vps_count: u32,
    pub max_std_sps_count: u32,
    pub max_std_pps_count: u32,
    pub p_parameters_add_info: *const VideoEncodeH265SessionParametersAddInfoKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265SessionParametersGetInfoKHR.html>
#[doc(alias = "VkVideoEncodeH265SessionParametersGetInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH265SessionParametersGetInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub write_std_vps: Bool32,
    pub write_std_sps: Bool32,
    pub write_std_pps: Bool32,
    pub std_vps_id: u32,
    pub std_sps_id: u32,
    pub std_pps_id: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265SessionParametersFeedbackInfoKHR.html>
#[doc(alias = "VkVideoEncodeH265SessionParametersFeedbackInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH265SessionParametersFeedbackInfoKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub has_std_vps_overrides: Bool32,
    pub has_std_sps_overrides: Bool32,
    pub has_std_pps_overrides: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265NaluSliceSegmentInfoKHR.html>
#[doc(alias = "VkVideoEncodeH265NaluSliceSegmentInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH265NaluSliceSegmentInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub constant_qp: i32,
    pub p_std_slice_segment_header: *const EncodeH265SliceSegmentHeader,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265PictureInfoKHR.html>
#[doc(alias = "VkVideoEncodeH265PictureInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH265PictureInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub nalu_slice_segment_entry_count: u32,
    pub p_nalu_slice_segment_entries: *const VideoEncodeH265NaluSliceSegmentInfoKHR,
    pub p_std_picture_info: *const EncodeH265PictureInfo,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265DpbSlotInfoKHR.html>
#[doc(alias = "VkVideoEncodeH265DpbSlotInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH265DpbSlotInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_reference_info: *const EncodeH265ReferenceInfo,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265ProfileInfoKHR.html>
#[doc(alias = "VkVideoEncodeH265ProfileInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH265ProfileInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_profile_idc: H265ProfileIdc,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265RateControlInfoKHR.html>
#[doc(alias = "VkVideoEncodeH265RateControlInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH265RateControlInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoEncodeH265RateControlFlagsKHR,
    pub gop_frame_count: u32,
    pub idr_period: u32,
    pub consecutive_b_frame_count: u32,
    pub sub_layer_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265FrameSizeKHR.html>
#[doc(alias = "VkVideoEncodeH265FrameSizeKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH265FrameSizeKHR {
    pub frame_i_size: u32,
    pub frame_p_size: u32,
    pub frame_b_size: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265RateControlLayerInfoKHR.html>
#[doc(alias = "VkVideoEncodeH265RateControlLayerInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH265RateControlLayerInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_min_qp: Bool32,
    pub min_qp: VideoEncodeH265QpKHR,
    pub use_max_qp: Bool32,
    pub max_qp: VideoEncodeH265QpKHR,
    pub use_max_frame_size: Bool32,
    pub max_frame_size: VideoEncodeH265FrameSizeKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265GopRemainingFrameInfoKHR.html>
#[doc(alias = "VkVideoEncodeH265GopRemainingFrameInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH265GopRemainingFrameInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_gop_remaining_frames: Bool32,
    pub gop_remaining_i: u32,
    pub gop_remaining_p: u32,
    pub gop_remaining_b: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeH264ProfileInfoKHR.html>
#[doc(alias = "VkVideoDecodeH264ProfileInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeH264ProfileInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_profile_idc: H264ProfileIdc,
    pub picture_layout: VideoDecodeH264PictureLayoutFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeH264CapabilitiesKHR.html>
#[doc(alias = "VkVideoDecodeH264CapabilitiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeH264CapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_level_idc: H264LevelIdc,
    pub field_offset_granularity: Offset2D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeH264SessionParametersAddInfoKHR.html>
#[doc(alias = "VkVideoDecodeH264SessionParametersAddInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeH264SessionParametersAddInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_sps_count: u32,
    pub p_std_sp_ss: *const H264SequenceParameterSet,
    pub std_pps_count: u32,
    pub p_std_pp_ss: *const H264PictureParameterSet,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeH264SessionParametersCreateInfoKHR.html>
#[doc(alias = "VkVideoDecodeH264SessionParametersCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeH264SessionParametersCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_std_sps_count: u32,
    pub max_std_pps_count: u32,
    pub p_parameters_add_info: *const VideoDecodeH264SessionParametersAddInfoKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeH264PictureInfoKHR.html>
#[doc(alias = "VkVideoDecodeH264PictureInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeH264PictureInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_picture_info: *const DecodeH264PictureInfo,
    pub slice_count: u32,
    pub p_slice_offsets: *const u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeH264DpbSlotInfoKHR.html>
#[doc(alias = "VkVideoDecodeH264DpbSlotInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeH264DpbSlotInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_reference_info: *const DecodeH264ReferenceInfo,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImportMemoryWin32HandleInfoKHR.html>
#[doc(alias = "VkImportMemoryWin32HandleInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImportMemoryWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlags,
    pub handle: HANDLE,
    pub name: LPCWSTR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExportMemoryWin32HandleInfoKHR.html>
#[doc(alias = "VkExportMemoryWin32HandleInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExportMemoryWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_attributes: *const SECURITY_ATTRIBUTES,
    pub dw_access: DWORD,
    pub name: LPCWSTR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryWin32HandlePropertiesKHR.html>
#[doc(alias = "VkMemoryWin32HandlePropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryWin32HandlePropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_type_bits: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryGetWin32HandleInfoKHR.html>
#[doc(alias = "VkMemoryGetWin32HandleInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryGetWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub handle_type: ExternalMemoryHandleTypeFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImportMemoryFdInfoKHR.html>
#[doc(alias = "VkImportMemoryFdInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImportMemoryFdInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlags,
    pub fd: c_int,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryFdPropertiesKHR.html>
#[doc(alias = "VkMemoryFdPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryFdPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_type_bits: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryGetFdInfoKHR.html>
#[doc(alias = "VkMemoryGetFdInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryGetFdInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub handle_type: ExternalMemoryHandleTypeFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkWin32KeyedMutexAcquireReleaseInfoKHR.html>
#[doc(alias = "VkWin32KeyedMutexAcquireReleaseInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Win32KeyedMutexAcquireReleaseInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acquire_count: u32,
    pub p_acquire_syncs: *const DeviceMemory,
    pub p_acquire_keys: *const u64,
    pub p_acquire_timeouts: *const u32,
    pub release_count: u32,
    pub p_release_syncs: *const DeviceMemory,
    pub p_release_keys: *const u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImportSemaphoreWin32HandleInfoKHR.html>
#[doc(alias = "VkImportSemaphoreWin32HandleInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImportSemaphoreWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub flags: SemaphoreImportFlags,
    pub handle_type: ExternalSemaphoreHandleTypeFlags,
    pub handle: HANDLE,
    pub name: LPCWSTR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExportSemaphoreWin32HandleInfoKHR.html>
#[doc(alias = "VkExportSemaphoreWin32HandleInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExportSemaphoreWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_attributes: *const SECURITY_ATTRIBUTES,
    pub dw_access: DWORD,
    pub name: LPCWSTR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkD3D12FenceSubmitInfoKHR.html>
#[doc(alias = "VkD3D12FenceSubmitInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct D3D12FenceSubmitInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_values_count: u32,
    pub p_wait_semaphore_values: *const u64,
    pub signal_semaphore_values_count: u32,
    pub p_signal_semaphore_values: *const u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreGetWin32HandleInfoKHR.html>
#[doc(alias = "VkSemaphoreGetWin32HandleInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SemaphoreGetWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub handle_type: ExternalSemaphoreHandleTypeFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImportSemaphoreFdInfoKHR.html>
#[doc(alias = "VkImportSemaphoreFdInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImportSemaphoreFdInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub flags: SemaphoreImportFlags,
    pub handle_type: ExternalSemaphoreHandleTypeFlags,
    pub fd: c_int,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreGetFdInfoKHR.html>
#[doc(alias = "VkSemaphoreGetFdInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SemaphoreGetFdInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub handle_type: ExternalSemaphoreHandleTypeFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRectLayerKHR.html>
#[doc(alias = "VkRectLayerKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RectLayerKHR {
    pub offset: Offset2D,
    pub extent: Extent2D,
    pub layer: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentRegionKHR.html>
#[doc(alias = "VkPresentRegionKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PresentRegionKHR {
    pub rectangle_count: u32,
    pub p_rectangles: *const RectLayerKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentRegionsKHR.html>
#[doc(alias = "VkPresentRegionsKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PresentRegionsKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_regions: *const PresentRegionKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSharedPresentSurfaceCapabilitiesKHR.html>
#[doc(alias = "VkSharedPresentSurfaceCapabilitiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SharedPresentSurfaceCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shared_present_supported_usage_flags: ImageUsageFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImportFenceWin32HandleInfoKHR.html>
#[doc(alias = "VkImportFenceWin32HandleInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImportFenceWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fence: Fence,
    pub flags: FenceImportFlags,
    pub handle_type: ExternalFenceHandleTypeFlags,
    pub handle: HANDLE,
    pub name: LPCWSTR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExportFenceWin32HandleInfoKHR.html>
#[doc(alias = "VkExportFenceWin32HandleInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExportFenceWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_attributes: *const SECURITY_ATTRIBUTES,
    pub dw_access: DWORD,
    pub name: LPCWSTR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceGetWin32HandleInfoKHR.html>
#[doc(alias = "VkFenceGetWin32HandleInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FenceGetWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fence: Fence,
    pub handle_type: ExternalFenceHandleTypeFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImportFenceFdInfoKHR.html>
#[doc(alias = "VkImportFenceFdInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImportFenceFdInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fence: Fence,
    pub flags: FenceImportFlags,
    pub handle_type: ExternalFenceHandleTypeFlags,
    pub fd: c_int,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceGetFdInfoKHR.html>
#[doc(alias = "VkFenceGetFdInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FenceGetFdInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fence: Fence,
    pub handle_type: ExternalFenceHandleTypeFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePerformanceQueryFeaturesKHR.html>
#[doc(alias = "VkPhysicalDevicePerformanceQueryFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePerformanceQueryFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub performance_counter_query_pools: Bool32,
    pub performance_counter_multiple_query_pools: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePerformanceQueryPropertiesKHR.html>
#[doc(alias = "VkPhysicalDevicePerformanceQueryPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePerformanceQueryPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub allow_command_buffer_query_copies: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterKHR.html>
#[doc(alias = "VkPerformanceCounterKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PerformanceCounterKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub unit: PerformanceCounterUnitKHR,
    pub scope: PerformanceCounterScopeKHR,
    pub storage: PerformanceCounterStorageKHR,
    pub uuid: [u8; 16],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterDescriptionKHR.html>
#[doc(alias = "VkPerformanceCounterDescriptionKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PerformanceCounterDescriptionKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: PerformanceCounterDescriptionFlagsKHR,
    pub name: [c_char; 256],
    pub category: [c_char; 256],
    pub description: [c_char; 256],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPoolPerformanceCreateInfoKHR.html>
#[doc(alias = "VkQueryPoolPerformanceCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QueryPoolPerformanceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub queue_family_index: u32,
    pub counter_index_count: u32,
    pub p_counter_indices: *const u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterResultKHR.html>
#[doc(alias = "VkPerformanceCounterResultKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PerformanceCounterResultKHR {
    pub int_32: i32,
    pub int_64: i64,
    pub uint_32: u32,
    pub uint_64: u64,
    pub float_32: f32,
    pub float_64: f64,
}
impl std::fmt::Debug for PerformanceCounterResultKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PerformanceCounterResultKHR {{ .. }}")
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAcquireProfilingLockInfoKHR.html>
#[doc(alias = "VkAcquireProfilingLockInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AcquireProfilingLockInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: AcquireProfilingLockFlagsKHR,
    pub timeout: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceQuerySubmitInfoKHR.html>
#[doc(alias = "VkPerformanceQuerySubmitInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PerformanceQuerySubmitInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub counter_pass_index: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSurfaceInfo2KHR.html>
#[doc(alias = "VkPhysicalDeviceSurfaceInfo2KHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSurfaceInfo2KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub surface: SurfaceKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCapabilities2KHR.html>
#[doc(alias = "VkSurfaceCapabilities2KHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SurfaceCapabilities2KHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub surface_capabilities: SurfaceCapabilitiesKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceFormat2KHR.html>
#[doc(alias = "VkSurfaceFormat2KHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SurfaceFormat2KHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub surface_format: SurfaceFormatKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayProperties2KHR.html>
#[doc(alias = "VkDisplayProperties2KHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DisplayProperties2KHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub display_properties: DisplayPropertiesKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayPlaneProperties2KHR.html>
#[doc(alias = "VkDisplayPlaneProperties2KHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DisplayPlaneProperties2KHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub display_plane_properties: DisplayPlanePropertiesKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayModeProperties2KHR.html>
#[doc(alias = "VkDisplayModeProperties2KHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DisplayModeProperties2KHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub display_mode_properties: DisplayModePropertiesKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayPlaneInfo2KHR.html>
#[doc(alias = "VkDisplayPlaneInfo2KHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DisplayPlaneInfo2KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mode: DisplayModeKHR,
    pub plane_index: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayPlaneCapabilities2KHR.html>
#[doc(alias = "VkDisplayPlaneCapabilities2KHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DisplayPlaneCapabilities2KHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub capabilities: DisplayPlaneCapabilitiesKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderBfloat16FeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceShaderBfloat16FeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderBfloat16FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_b_float_16_type: Bool32,
    pub shader_b_float_16_dot_product: Bool32,
    pub shader_b_float_16_cooperative_matrix: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePortabilitySubsetFeaturesKHR.html>
#[doc(alias = "VkPhysicalDevicePortabilitySubsetFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePortabilitySubsetFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub constant_alpha_color_blend_factors: Bool32,
    pub events: Bool32,
    pub image_view_format_reinterpretation: Bool32,
    pub image_view_format_swizzle: Bool32,
    pub image_view_2_d_on_3_d_image: Bool32,
    pub multisample_array_image: Bool32,
    pub mutable_comparison_samplers: Bool32,
    pub point_polygons: Bool32,
    pub sampler_mip_lod_bias: Bool32,
    pub separate_stencil_mask_ref: Bool32,
    pub shader_sample_rate_interpolation_functions: Bool32,
    pub tessellation_isolines: Bool32,
    pub tessellation_point_mode: Bool32,
    pub triangle_fans: Bool32,
    pub vertex_attribute_access_beyond_stride: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePortabilitySubsetPropertiesKHR.html>
#[doc(alias = "VkPhysicalDevicePortabilitySubsetPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePortabilitySubsetPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_vertex_input_binding_stride_alignment: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderClockFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceShaderClockFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderClockFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_subgroup_clock: Bool32,
    pub shader_device_clock: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeH265ProfileInfoKHR.html>
#[doc(alias = "VkVideoDecodeH265ProfileInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeH265ProfileInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_profile_idc: H265ProfileIdc,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeH265CapabilitiesKHR.html>
#[doc(alias = "VkVideoDecodeH265CapabilitiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeH265CapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_level_idc: H265LevelIdc,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeH265SessionParametersAddInfoKHR.html>
#[doc(alias = "VkVideoDecodeH265SessionParametersAddInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeH265SessionParametersAddInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_vps_count: u32,
    pub p_std_vp_ss: *const H265VideoParameterSet,
    pub std_sps_count: u32,
    pub p_std_sp_ss: *const H265SequenceParameterSet,
    pub std_pps_count: u32,
    pub p_std_pp_ss: *const H265PictureParameterSet,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeH265SessionParametersCreateInfoKHR.html>
#[doc(alias = "VkVideoDecodeH265SessionParametersCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeH265SessionParametersCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_std_vps_count: u32,
    pub max_std_sps_count: u32,
    pub max_std_pps_count: u32,
    pub p_parameters_add_info: *const VideoDecodeH265SessionParametersAddInfoKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeH265PictureInfoKHR.html>
#[doc(alias = "VkVideoDecodeH265PictureInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeH265PictureInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_picture_info: *const DecodeH265PictureInfo,
    pub slice_segment_count: u32,
    pub p_slice_segment_offsets: *const u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeH265DpbSlotInfoKHR.html>
#[doc(alias = "VkVideoDecodeH265DpbSlotInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeH265DpbSlotInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_reference_info: *const DecodeH265ReferenceInfo,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFragmentShadingRateAttachmentInfoKHR.html>
#[doc(alias = "VkFragmentShadingRateAttachmentInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FragmentShadingRateAttachmentInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_fragment_shading_rate_attachment: *const AttachmentReference2,
    pub shading_rate_attachment_texel_size: Extent2D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineFragmentShadingRateStateCreateInfoKHR.html>
#[doc(alias = "VkPipelineFragmentShadingRateStateCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineFragmentShadingRateStateCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fragment_size: Extent2D,
    pub combiner_ops: [FragmentShadingRateCombinerOpKHR; 2],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentShadingRateFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRateFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_fragment_shading_rate: Bool32,
    pub primitive_fragment_shading_rate: Bool32,
    pub attachment_fragment_shading_rate: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentShadingRatePropertiesKHR.html>
#[doc(alias = "VkPhysicalDeviceFragmentShadingRatePropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRatePropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_fragment_shading_rate_attachment_texel_size: Extent2D,
    pub max_fragment_shading_rate_attachment_texel_size: Extent2D,
    pub max_fragment_shading_rate_attachment_texel_size_aspect_ratio: u32,
    pub primitive_fragment_shading_rate_with_multiple_viewports: Bool32,
    pub layered_shading_rate_attachments: Bool32,
    pub fragment_shading_rate_non_trivial_combiner_ops: Bool32,
    pub max_fragment_size: Extent2D,
    pub max_fragment_size_aspect_ratio: u32,
    pub max_fragment_shading_rate_coverage_samples: u32,
    pub max_fragment_shading_rate_rasterization_samples: SampleCountFlags,
    pub fragment_shading_rate_with_shader_depth_stencil_writes: Bool32,
    pub fragment_shading_rate_with_sample_mask: Bool32,
    pub fragment_shading_rate_with_shader_sample_mask: Bool32,
    pub fragment_shading_rate_with_conservative_rasterization: Bool32,
    pub fragment_shading_rate_with_fragment_shader_interlock: Bool32,
    pub fragment_shading_rate_with_custom_sample_locations: Bool32,
    pub fragment_shading_rate_strict_multiply_combiner: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentShadingRateKHR.html>
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRateKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub sample_counts: SampleCountFlags,
    pub fragment_size: Extent2D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingFragmentShadingRateAttachmentInfoKHR.html>
#[doc(alias = "VkRenderingFragmentShadingRateAttachmentInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderingFragmentShadingRateAttachmentInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
    pub shading_rate_attachment_texel_size: Extent2D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderConstantDataFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceShaderConstantDataFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderConstantDataFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_constant_data: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderAbortFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceShaderAbortFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderAbortFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_abort: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceFaultShaderAbortMessageInfoKHR.html>
#[doc(alias = "VkDeviceFaultShaderAbortMessageInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceFaultShaderAbortMessageInfoKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub message_data_size: u64,
    pub p_message_data: *mut c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderAbortPropertiesKHR.html>
#[doc(alias = "VkPhysicalDeviceShaderAbortPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderAbortPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_shader_abort_message_size: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderQuadControlFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceShaderQuadControlFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderQuadControlFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_quad_control: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceProtectedCapabilitiesKHR.html>
#[doc(alias = "VkSurfaceProtectedCapabilitiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SurfaceProtectedCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supports_protected: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePresentWaitFeaturesKHR.html>
#[doc(alias = "VkPhysicalDevicePresentWaitFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePresentWaitFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_wait: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR.html>
#[doc(alias = "VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_executable_info: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineInfoKHR.html>
#[doc(alias = "VkPipelineInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline: Pipeline,
}
pub type PipelineInfoEXT = PipelineInfoKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineExecutablePropertiesKHR.html>
#[doc(alias = "VkPipelineExecutablePropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineExecutablePropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub stages: ShaderStageFlags,
    pub name: [c_char; 256],
    pub description: [c_char; 256],
    pub subgroup_size: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineExecutableInfoKHR.html>
#[doc(alias = "VkPipelineExecutableInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineExecutableInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline: Pipeline,
    pub executable_index: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineExecutableStatisticValueKHR.html>
#[doc(alias = "VkPipelineExecutableStatisticValueKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineExecutableStatisticValueKHR {
    pub b_32: Bool32,
    pub i_64: i64,
    pub u_64: u64,
    pub f_64: f64,
}
impl std::fmt::Debug for PipelineExecutableStatisticValueKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PipelineExecutableStatisticValueKHR {{ .. }}")
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineExecutableStatisticKHR.html>
#[doc(alias = "VkPipelineExecutableStatisticKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineExecutableStatisticKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub name: [c_char; 256],
    pub description: [c_char; 256],
    pub format: PipelineExecutableStatisticFormatKHR,
    pub value: PipelineExecutableStatisticValueKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineExecutableInternalRepresentationKHR.html>
#[doc(alias = "VkPipelineExecutableInternalRepresentationKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineExecutableInternalRepresentationKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub name: [c_char; 256],
    pub description: [c_char; 256],
    pub is_text: Bool32,
    pub data_size: usize,
    pub p_data: *mut c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineLibraryCreateInfoKHR.html>
#[doc(alias = "VkPipelineLibraryCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineLibraryCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub library_count: u32,
    pub p_libraries: *const Pipeline,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentIdKHR.html>
#[doc(alias = "VkPresentIdKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PresentIdKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_present_ids: *const u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePresentIdFeaturesKHR.html>
#[doc(alias = "VkPhysicalDevicePresentIdFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePresentIdFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_id: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeInfoKHR.html>
#[doc(alias = "VkVideoEncodeInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoEncodeFlagsKHR,
    pub dst_buffer: Buffer,
    pub dst_buffer_offset: DeviceSize,
    pub dst_buffer_range: DeviceSize,
    pub src_picture_resource: VideoPictureResourceInfoKHR,
    pub p_setup_reference_slot: *const VideoReferenceSlotInfoKHR,
    pub reference_slot_count: u32,
    pub p_reference_slots: *const VideoReferenceSlotInfoKHR,
    pub preceding_externally_encoded_bytes: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeCapabilitiesKHR.html>
#[doc(alias = "VkVideoEncodeCapabilitiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: VideoEncodeCapabilityFlagsKHR,
    pub rate_control_modes: VideoEncodeRateControlModeFlagsKHR,
    pub max_rate_control_layers: u32,
    pub max_bitrate: u64,
    pub max_quality_levels: u32,
    pub encode_input_picture_granularity: Extent2D,
    pub supported_encode_feedback_flags: VideoEncodeFeedbackFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPoolVideoEncodeFeedbackCreateInfoKHR.html>
#[doc(alias = "VkQueryPoolVideoEncodeFeedbackCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QueryPoolVideoEncodeFeedbackCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub encode_feedback_flags: VideoEncodeFeedbackFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeUsageInfoKHR.html>
#[doc(alias = "VkVideoEncodeUsageInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeUsageInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub video_usage_hints: VideoEncodeUsageFlagsKHR,
    pub video_content_hints: VideoEncodeContentFlagsKHR,
    pub tuning_mode: VideoEncodeTuningModeKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRateControlLayerInfoKHR.html>
#[doc(alias = "VkVideoEncodeRateControlLayerInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeRateControlLayerInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub average_bitrate: u64,
    pub max_bitrate: u64,
    pub frame_rate_numerator: u32,
    pub frame_rate_denominator: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRateControlInfoKHR.html>
#[doc(alias = "VkVideoEncodeRateControlInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeRateControlInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoEncodeRateControlFlagsKHR,
    pub rate_control_mode: VideoEncodeRateControlModeFlagsKHR,
    pub layer_count: u32,
    pub p_layers: *const VideoEncodeRateControlLayerInfoKHR,
    pub virtual_buffer_size_in_ms: u32,
    pub initial_virtual_buffer_size_in_ms: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVideoEncodeQualityLevelInfoKHR.html>
#[doc(alias = "VkPhysicalDeviceVideoEncodeQualityLevelInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVideoEncodeQualityLevelInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_video_profile: *const VideoProfileInfoKHR,
    pub quality_level: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeQualityLevelPropertiesKHR.html>
#[doc(alias = "VkVideoEncodeQualityLevelPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeQualityLevelPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub preferred_rate_control_mode: VideoEncodeRateControlModeFlagsKHR,
    pub preferred_rate_control_layer_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeQualityLevelInfoKHR.html>
#[doc(alias = "VkVideoEncodeQualityLevelInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeQualityLevelInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub quality_level: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeSessionParametersGetInfoKHR.html>
#[doc(alias = "VkVideoEncodeSessionParametersGetInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeSessionParametersGetInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub video_session_parameters: VideoSessionParametersKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeSessionParametersFeedbackInfoKHR.html>
#[doc(alias = "VkVideoEncodeSessionParametersFeedbackInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeSessionParametersFeedbackInfoKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub has_overrides: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceAddressRangeKHR.html>
#[doc(alias = "VkDeviceAddressRangeKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceAddressRangeKHR {
    pub address: DeviceAddress,
    pub size: DeviceSize,
}
pub type DeviceAddressRangeEXT = DeviceAddressRangeKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkStridedDeviceAddressRangeKHR.html>
#[doc(alias = "VkStridedDeviceAddressRangeKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct StridedDeviceAddressRangeKHR {
    pub address: DeviceAddress,
    pub size: DeviceSize,
    pub stride: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemoryCopyKHR.html>
#[doc(alias = "VkDeviceMemoryCopyKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceMemoryCopyKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_range: DeviceAddressRangeKHR,
    pub src_flags: AddressCommandFlagsKHR,
    pub dst_range: DeviceAddressRangeKHR,
    pub dst_flags: AddressCommandFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyDeviceMemoryInfoKHR.html>
#[doc(alias = "VkCopyDeviceMemoryInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CopyDeviceMemoryInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub region_count: u32,
    pub p_regions: *const DeviceMemoryCopyKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemoryImageCopyKHR.html>
#[doc(alias = "VkDeviceMemoryImageCopyKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceMemoryImageCopyKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub address_range: DeviceAddressRangeKHR,
    pub address_flags: AddressCommandFlagsKHR,
    pub address_row_length: u32,
    pub address_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_layout: ImageLayout,
    pub image_offset: Offset3D,
    pub image_extent: Extent3D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyDeviceMemoryImageInfoKHR.html>
#[doc(alias = "VkCopyDeviceMemoryImageInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CopyDeviceMemoryImageInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub region_count: u32,
    pub p_regions: *const DeviceMemoryImageCopyKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryRangeBarrierKHR.html>
#[doc(alias = "VkMemoryRangeBarrierKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryRangeBarrierKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_stage_mask: PipelineStageFlags2,
    pub src_access_mask: AccessFlags2,
    pub dst_stage_mask: PipelineStageFlags2,
    pub dst_access_mask: AccessFlags2,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub address_range: DeviceAddressRangeKHR,
    pub address_flags: AddressCommandFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryRangeBarriersInfoKHR.html>
#[doc(alias = "VkMemoryRangeBarriersInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryRangeBarriersInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory_range_barrier_count: u32,
    pub p_memory_range_barriers: *const MemoryRangeBarrierKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDeviceAddressCommandsFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceDeviceAddressCommandsFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDeviceAddressCommandsFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_address_commands: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindIndexBuffer3InfoKHR.html>
#[doc(alias = "VkBindIndexBuffer3InfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BindIndexBuffer3InfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub address_range: DeviceAddressRangeKHR,
    pub address_flags: AddressCommandFlagsKHR,
    pub index_type: IndexType,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindVertexBuffer3InfoKHR.html>
#[doc(alias = "VkBindVertexBuffer3InfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BindVertexBuffer3InfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub set_stride: Bool32,
    pub address_range: StridedDeviceAddressRangeKHR,
    pub address_flags: AddressCommandFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDrawIndirect2InfoKHR.html>
#[doc(alias = "VkDrawIndirect2InfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DrawIndirect2InfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub address_range: StridedDeviceAddressRangeKHR,
    pub address_flags: AddressCommandFlagsKHR,
    pub draw_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDrawIndirectCount2InfoKHR.html>
#[doc(alias = "VkDrawIndirectCount2InfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DrawIndirectCount2InfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub address_range: StridedDeviceAddressRangeKHR,
    pub address_flags: AddressCommandFlagsKHR,
    pub count_address_range: DeviceAddressRangeKHR,
    pub count_address_flags: AddressCommandFlagsKHR,
    pub max_draw_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDispatchIndirect2InfoKHR.html>
#[doc(alias = "VkDispatchIndirect2InfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DispatchIndirect2InfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub address_range: DeviceAddressRangeKHR,
    pub address_flags: AddressCommandFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkConditionalRenderingBeginInfo2EXT.html>
#[doc(alias = "VkConditionalRenderingBeginInfo2EXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ConditionalRenderingBeginInfo2EXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub address_range: DeviceAddressRangeKHR,
    pub address_flags: AddressCommandFlagsKHR,
    pub flags: ConditionalRenderingFlagsEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindTransformFeedbackBuffer2InfoEXT.html>
#[doc(alias = "VkBindTransformFeedbackBuffer2InfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BindTransformFeedbackBuffer2InfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub address_range: DeviceAddressRangeKHR,
    pub address_flags: AddressCommandFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryMarkerInfoAMD.html>
#[doc(alias = "VkMemoryMarkerInfoAMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryMarkerInfoAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stage: PipelineStageFlags2KHR,
    pub dst_range: DeviceAddressRangeKHR,
    pub dst_flags: AddressCommandFlagsKHR,
    pub marker: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureCreateInfo2KHR.html>
#[doc(alias = "VkAccelerationStructureCreateInfo2KHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureCreateInfo2KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub create_flags: AccelerationStructureCreateFlagsKHR,
    pub address_range: DeviceAddressRangeKHR,
    pub address_flags: AddressCommandFlagsKHR,
    pub type_: AccelerationStructureTypeKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShaderBarycentricFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub fragment_shader_barycentric: Bool32,
}
pub type PhysicalDeviceFragmentShaderBarycentricFeaturesNV =
    PhysicalDeviceFragmentShaderBarycentricFeaturesKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR.html>
#[doc(alias = "VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShaderBarycentricPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tri_strip_vertex_order_independent_of_provoking_vertex: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_subgroup_uniform_control_flow: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub workgroup_memory_explicit_layout: Bool32,
    pub workgroup_memory_explicit_layout_scalar_block_layout: Bool32,
    pub workgroup_memory_explicit_layout_8_bit_access: Bool32,
    pub workgroup_memory_explicit_layout_16_bit_access: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingMaintenance1FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_tracing_maintenance_1: Bool32,
    pub ray_tracing_pipeline_trace_rays_indirect_2: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTraceRaysIndirectCommand2KHR.html>
#[doc(alias = "VkTraceRaysIndirectCommand2KHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TraceRaysIndirectCommand2KHR {
    pub raygen_shader_record_address: DeviceAddress,
    pub raygen_shader_record_size: DeviceSize,
    pub miss_shader_binding_table_address: DeviceAddress,
    pub miss_shader_binding_table_size: DeviceSize,
    pub miss_shader_binding_table_stride: DeviceSize,
    pub hit_shader_binding_table_address: DeviceAddress,
    pub hit_shader_binding_table_size: DeviceSize,
    pub hit_shader_binding_table_stride: DeviceSize,
    pub callable_shader_binding_table_address: DeviceAddress,
    pub callable_shader_binding_table_size: DeviceSize,
    pub callable_shader_binding_table_stride: DeviceSize,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderUntypedPointersFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceShaderUntypedPointersFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderUntypedPointersFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_untyped_pointers: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderMaximalReconvergenceFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceShaderMaximalReconvergenceFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderMaximalReconvergenceFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_maximal_reconvergence: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCapabilitiesPresentId2KHR.html>
#[doc(alias = "VkSurfaceCapabilitiesPresentId2KHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SurfaceCapabilitiesPresentId2KHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_id_2_supported: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentId2KHR.html>
#[doc(alias = "VkPresentId2KHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PresentId2KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_present_ids: *const u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePresentId2FeaturesKHR.html>
#[doc(alias = "VkPhysicalDevicePresentId2FeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePresentId2FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_id_2: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCapabilitiesPresentWait2KHR.html>
#[doc(alias = "VkSurfaceCapabilitiesPresentWait2KHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SurfaceCapabilitiesPresentWait2KHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_wait_2_supported: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePresentWait2FeaturesKHR.html>
#[doc(alias = "VkPhysicalDevicePresentWait2FeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePresentWait2FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_wait_2: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentWait2InfoKHR.html>
#[doc(alias = "VkPresentWait2InfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PresentWait2InfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_id: u64,
    pub timeout: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRayTracingPositionFetchFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceRayTracingPositionFetchFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingPositionFetchFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_tracing_position_fetch: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineBinaryFeaturesKHR.html>
#[doc(alias = "VkPhysicalDevicePipelineBinaryFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePipelineBinaryFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_binaries: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineBinaryPropertiesKHR.html>
#[doc(alias = "VkPhysicalDevicePipelineBinaryPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePipelineBinaryPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_binary_internal_cache: Bool32,
    pub pipeline_binary_internal_cache_control: Bool32,
    pub pipeline_binary_prefers_internal_cache: Bool32,
    pub pipeline_binary_precompiled_internal_cache: Bool32,
    pub pipeline_binary_compressed_data: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDevicePipelineBinaryInternalCacheControlKHR.html>
#[doc(alias = "VkDevicePipelineBinaryInternalCacheControlKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DevicePipelineBinaryInternalCacheControlKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub disable_internal_cache: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBinaryKeyKHR.html>
#[doc(alias = "VkPipelineBinaryKeyKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineBinaryKeyKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub key_size: u32,
    pub key: [u8; 32],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBinaryDataKHR.html>
#[doc(alias = "VkPipelineBinaryDataKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineBinaryDataKHR {
    pub data_size: usize,
    pub p_data: *mut c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBinaryKeysAndDataKHR.html>
#[doc(alias = "VkPipelineBinaryKeysAndDataKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineBinaryKeysAndDataKHR {
    pub binary_count: u32,
    pub p_pipeline_binary_keys: *const PipelineBinaryKeyKHR,
    pub p_pipeline_binary_data: *const PipelineBinaryDataKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateInfoKHR.html>
#[doc(alias = "VkPipelineCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBinaryCreateInfoKHR.html>
#[doc(alias = "VkPipelineBinaryCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineBinaryCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_keys_and_data_info: *const PipelineBinaryKeysAndDataKHR,
    pub pipeline: Pipeline,
    pub p_pipeline_create_info: *const PipelineCreateInfoKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBinaryInfoKHR.html>
#[doc(alias = "VkPipelineBinaryInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineBinaryInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub binary_count: u32,
    pub p_pipeline_binaries: *const PipelineBinaryKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkReleaseCapturedPipelineDataInfoKHR.html>
#[doc(alias = "VkReleaseCapturedPipelineDataInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ReleaseCapturedPipelineDataInfoKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline: Pipeline,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBinaryDataInfoKHR.html>
#[doc(alias = "VkPipelineBinaryDataInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineBinaryDataInfoKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_binary: PipelineBinaryKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBinaryHandlesInfoKHR.html>
#[doc(alias = "VkPipelineBinaryHandlesInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineBinaryHandlesInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline_binary_count: u32,
    pub p_pipeline_binaries: *mut PipelineBinaryKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfacePresentModeKHR.html>
#[doc(alias = "VkSurfacePresentModeKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SurfacePresentModeKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_mode: PresentModeKHR,
}
pub type SurfacePresentModeEXT = SurfacePresentModeKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfacePresentScalingCapabilitiesKHR.html>
#[doc(alias = "VkSurfacePresentScalingCapabilitiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SurfacePresentScalingCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supported_present_scaling: PresentScalingFlagsKHR,
    pub supported_present_gravity_x: PresentGravityFlagsKHR,
    pub supported_present_gravity_y: PresentGravityFlagsKHR,
    pub min_scaled_image_extent: Extent2D,
    pub max_scaled_image_extent: Extent2D,
}
pub type SurfacePresentScalingCapabilitiesEXT = SurfacePresentScalingCapabilitiesKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfacePresentModeCompatibilityKHR.html>
#[doc(alias = "VkSurfacePresentModeCompatibilityKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SurfacePresentModeCompatibilityKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_mode_count: u32,
    pub p_present_modes: *mut PresentModeKHR,
}
pub type SurfacePresentModeCompatibilityEXT = SurfacePresentModeCompatibilityKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSwapchainMaintenance1FeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceSwapchainMaintenance1FeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSwapchainMaintenance1FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub swapchain_maintenance_1: Bool32,
}
pub type PhysicalDeviceSwapchainMaintenance1FeaturesEXT =
    PhysicalDeviceSwapchainMaintenance1FeaturesKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainPresentFenceInfoKHR.html>
#[doc(alias = "VkSwapchainPresentFenceInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SwapchainPresentFenceInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_fences: *const Fence,
}
pub type SwapchainPresentFenceInfoEXT = SwapchainPresentFenceInfoKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainPresentModesCreateInfoKHR.html>
#[doc(alias = "VkSwapchainPresentModesCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SwapchainPresentModesCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_mode_count: u32,
    pub p_present_modes: *const PresentModeKHR,
}
pub type SwapchainPresentModesCreateInfoEXT = SwapchainPresentModesCreateInfoKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainPresentModeInfoKHR.html>
#[doc(alias = "VkSwapchainPresentModeInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SwapchainPresentModeInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_present_modes: *const PresentModeKHR,
}
pub type SwapchainPresentModeInfoEXT = SwapchainPresentModeInfoKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainPresentScalingCreateInfoKHR.html>
#[doc(alias = "VkSwapchainPresentScalingCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SwapchainPresentScalingCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub scaling_behavior: PresentScalingFlagsKHR,
    pub present_gravity_x: PresentGravityFlagsKHR,
    pub present_gravity_y: PresentGravityFlagsKHR,
}
pub type SwapchainPresentScalingCreateInfoEXT = SwapchainPresentScalingCreateInfoKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkReleaseSwapchainImagesInfoKHR.html>
#[doc(alias = "VkReleaseSwapchainImagesInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ReleaseSwapchainImagesInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain: SwapchainKHR,
    pub image_index_count: u32,
    pub p_image_indices: *const u32,
}
pub type ReleaseSwapchainImagesInfoEXT = ReleaseSwapchainImagesInfoKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceInternallySynchronizedQueuesFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceInternallySynchronizedQueuesFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceInternallySynchronizedQueuesFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub internally_synchronized_queues: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCooperativeMatrixPropertiesKHR.html>
#[doc(alias = "VkCooperativeMatrixPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CooperativeMatrixPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub m_size: u32,
    pub n_size: u32,
    pub k_size: u32,
    pub a_type: ComponentTypeKHR,
    pub b_type: ComponentTypeKHR,
    pub c_type: ComponentTypeKHR,
    pub result_type: ComponentTypeKHR,
    pub saturating_accumulation: Bool32,
    pub scope: ScopeKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCooperativeMatrixFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceCooperativeMatrixFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_matrix: Bool32,
    pub cooperative_matrix_robust_buffer_access: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCooperativeMatrixPropertiesKHR.html>
#[doc(alias = "VkPhysicalDeviceCooperativeMatrixPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_matrix_supported_stages: ShaderStageFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceComputeShaderDerivativesFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceComputeShaderDerivativesFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceComputeShaderDerivativesFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub compute_derivative_group_quads: Bool32,
    pub compute_derivative_group_linear: Bool32,
}
pub type PhysicalDeviceComputeShaderDerivativesFeaturesNV =
    PhysicalDeviceComputeShaderDerivativesFeaturesKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceComputeShaderDerivativesPropertiesKHR.html>
#[doc(alias = "VkPhysicalDeviceComputeShaderDerivativesPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceComputeShaderDerivativesPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub mesh_and_task_shader_derivatives: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeAV1ProfileInfoKHR.html>
#[doc(alias = "VkVideoDecodeAV1ProfileInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeAV1ProfileInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_profile: AV1Profile,
    pub film_grain_support: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeAV1CapabilitiesKHR.html>
#[doc(alias = "VkVideoDecodeAV1CapabilitiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeAV1CapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_level: AV1Level,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeAV1SessionParametersCreateInfoKHR.html>
#[doc(alias = "VkVideoDecodeAV1SessionParametersCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeAV1SessionParametersCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_sequence_header: *const AV1SequenceHeader,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeAV1PictureInfoKHR.html>
#[doc(alias = "VkVideoDecodeAV1PictureInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeAV1PictureInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_picture_info: *const DecodeAV1PictureInfo,
    pub reference_name_slot_indices: [i32; 7],
    pub frame_header_offset: u32,
    pub tile_count: u32,
    pub p_tile_offsets: *const u32,
    pub p_tile_sizes: *const u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeAV1DpbSlotInfoKHR.html>
#[doc(alias = "VkVideoDecodeAV1DpbSlotInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeAV1DpbSlotInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_reference_info: *const DecodeAV1ReferenceInfo,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVideoEncodeAV1FeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceVideoEncodeAV1FeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVideoEncodeAV1FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_encode_av_1: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1CapabilitiesKHR.html>
#[doc(alias = "VkVideoEncodeAV1CapabilitiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeAV1CapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: VideoEncodeAV1CapabilityFlagsKHR,
    pub max_level: AV1Level,
    pub coded_picture_alignment: Extent2D,
    pub max_tiles: Extent2D,
    pub min_tile_size: Extent2D,
    pub max_tile_size: Extent2D,
    pub superblock_sizes: VideoEncodeAV1SuperblockSizeFlagsKHR,
    pub max_single_reference_count: u32,
    pub single_reference_name_mask: u32,
    pub max_unidirectional_compound_reference_count: u32,
    pub max_unidirectional_compound_group_1_reference_count: u32,
    pub unidirectional_compound_reference_name_mask: u32,
    pub max_bidirectional_compound_reference_count: u32,
    pub max_bidirectional_compound_group_1_reference_count: u32,
    pub max_bidirectional_compound_group_2_reference_count: u32,
    pub bidirectional_compound_reference_name_mask: u32,
    pub max_temporal_layer_count: u32,
    pub max_spatial_layer_count: u32,
    pub max_operating_points: u32,
    pub min_q_index: u32,
    pub max_q_index: u32,
    pub prefers_gop_remaining_frames: Bool32,
    pub requires_gop_remaining_frames: Bool32,
    pub std_syntax_flags: VideoEncodeAV1StdFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1QIndexKHR.html>
#[doc(alias = "VkVideoEncodeAV1QIndexKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeAV1QIndexKHR {
    pub intra_q_index: u32,
    pub predictive_q_index: u32,
    pub bipredictive_q_index: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1QualityLevelPropertiesKHR.html>
#[doc(alias = "VkVideoEncodeAV1QualityLevelPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeAV1QualityLevelPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub preferred_rate_control_flags: VideoEncodeAV1RateControlFlagsKHR,
    pub preferred_gop_frame_count: u32,
    pub preferred_key_frame_period: u32,
    pub preferred_consecutive_bipredictive_frame_count: u32,
    pub preferred_temporal_layer_count: u32,
    pub preferred_constant_q_index: VideoEncodeAV1QIndexKHR,
    pub preferred_max_single_reference_count: u32,
    pub preferred_single_reference_name_mask: u32,
    pub preferred_max_unidirectional_compound_reference_count: u32,
    pub preferred_max_unidirectional_compound_group_1_reference_count: u32,
    pub preferred_unidirectional_compound_reference_name_mask: u32,
    pub preferred_max_bidirectional_compound_reference_count: u32,
    pub preferred_max_bidirectional_compound_group_1_reference_count: u32,
    pub preferred_max_bidirectional_compound_group_2_reference_count: u32,
    pub preferred_bidirectional_compound_reference_name_mask: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1SessionCreateInfoKHR.html>
#[doc(alias = "VkVideoEncodeAV1SessionCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeAV1SessionCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_max_level: Bool32,
    pub max_level: AV1Level,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1SessionParametersCreateInfoKHR.html>
#[doc(alias = "VkVideoEncodeAV1SessionParametersCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeAV1SessionParametersCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_sequence_header: *const AV1SequenceHeader,
    pub p_std_decoder_model_info: *const EncodeAV1DecoderModelInfo,
    pub std_operating_point_count: u32,
    pub p_std_operating_points: *const EncodeAV1OperatingPointInfo,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1PictureInfoKHR.html>
#[doc(alias = "VkVideoEncodeAV1PictureInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeAV1PictureInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub prediction_mode: VideoEncodeAV1PredictionModeKHR,
    pub rate_control_group: VideoEncodeAV1RateControlGroupKHR,
    pub constant_q_index: u32,
    pub p_std_picture_info: *const EncodeAV1PictureInfo,
    pub reference_name_slot_indices: [i32; 7],
    pub primary_reference_cdf_only: Bool32,
    pub generate_obu_extension_header: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1DpbSlotInfoKHR.html>
#[doc(alias = "VkVideoEncodeAV1DpbSlotInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeAV1DpbSlotInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_reference_info: *const EncodeAV1ReferenceInfo,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1ProfileInfoKHR.html>
#[doc(alias = "VkVideoEncodeAV1ProfileInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeAV1ProfileInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_profile: AV1Profile,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1FrameSizeKHR.html>
#[doc(alias = "VkVideoEncodeAV1FrameSizeKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeAV1FrameSizeKHR {
    pub intra_frame_size: u32,
    pub predictive_frame_size: u32,
    pub bipredictive_frame_size: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1GopRemainingFrameInfoKHR.html>
#[doc(alias = "VkVideoEncodeAV1GopRemainingFrameInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeAV1GopRemainingFrameInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_gop_remaining_frames: Bool32,
    pub gop_remaining_intra: u32,
    pub gop_remaining_predictive: u32,
    pub gop_remaining_bipredictive: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1RateControlInfoKHR.html>
#[doc(alias = "VkVideoEncodeAV1RateControlInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeAV1RateControlInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoEncodeAV1RateControlFlagsKHR,
    pub gop_frame_count: u32,
    pub key_frame_period: u32,
    pub consecutive_bipredictive_frame_count: u32,
    pub temporal_layer_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1RateControlLayerInfoKHR.html>
#[doc(alias = "VkVideoEncodeAV1RateControlLayerInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeAV1RateControlLayerInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_min_q_index: Bool32,
    pub min_q_index: VideoEncodeAV1QIndexKHR,
    pub use_max_q_index: Bool32,
    pub max_q_index: VideoEncodeAV1QIndexKHR,
    pub use_max_frame_size: Bool32,
    pub max_frame_size: VideoEncodeAV1FrameSizeKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVideoDecodeVP9FeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceVideoDecodeVP9FeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVideoDecodeVP9FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_decode_vp_9: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeVP9ProfileInfoKHR.html>
#[doc(alias = "VkVideoDecodeVP9ProfileInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeVP9ProfileInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_profile: VP9Profile,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeVP9CapabilitiesKHR.html>
#[doc(alias = "VkVideoDecodeVP9CapabilitiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeVP9CapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_level: VP9Level,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeVP9PictureInfoKHR.html>
#[doc(alias = "VkVideoDecodeVP9PictureInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeVP9PictureInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_picture_info: *const DecodeVP9PictureInfo,
    pub reference_name_slot_indices: [i32; 3],
    pub uncompressed_header_offset: u32,
    pub compressed_header_offset: u32,
    pub tiles_offset: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVideoMaintenance1FeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceVideoMaintenance1FeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVideoMaintenance1FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_maintenance_1: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoInlineQueryInfoKHR.html>
#[doc(alias = "VkVideoInlineQueryInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoInlineQueryInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub query_pool: QueryPool,
    pub first_query: u32,
    pub query_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceUnifiedImageLayoutsFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceUnifiedImageLayoutsFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceUnifiedImageLayoutsFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub unified_image_layouts: Bool32,
    pub unified_image_layouts_video: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentFeedbackLoopInfoEXT.html>
#[doc(alias = "VkAttachmentFeedbackLoopInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AttachmentFeedbackLoopInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub feedback_loop_enable: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCalibratedTimestampInfoKHR.html>
#[doc(alias = "VkCalibratedTimestampInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CalibratedTimestampInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub time_domain: TimeDomainKHR,
}
pub type CalibratedTimestampInfoEXT = CalibratedTimestampInfoKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSetDescriptorBufferOffsetsInfoEXT.html>
#[doc(alias = "VkSetDescriptorBufferOffsetsInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SetDescriptorBufferOffsetsInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stage_flags: ShaderStageFlags,
    pub layout: PipelineLayout,
    pub first_set: u32,
    pub set_count: u32,
    pub p_buffer_indices: *const u32,
    pub p_offsets: *const DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindDescriptorBufferEmbeddedSamplersInfoEXT.html>
#[doc(alias = "VkBindDescriptorBufferEmbeddedSamplersInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BindDescriptorBufferEmbeddedSamplersInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stage_flags: ShaderStageFlags,
    pub layout: PipelineLayout,
    pub set: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyMemoryIndirectCommandKHR.html>
#[doc(alias = "VkCopyMemoryIndirectCommandKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CopyMemoryIndirectCommandKHR {
    pub src_address: DeviceAddress,
    pub dst_address: DeviceAddress,
    pub size: DeviceSize,
}
pub type CopyMemoryIndirectCommandNV = CopyMemoryIndirectCommandKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyMemoryIndirectInfoKHR.html>
#[doc(alias = "VkCopyMemoryIndirectInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CopyMemoryIndirectInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_copy_flags: AddressCopyFlagsKHR,
    pub dst_copy_flags: AddressCopyFlagsKHR,
    pub copy_count: u32,
    pub copy_address_range: StridedDeviceAddressRangeKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyMemoryToImageIndirectCommandKHR.html>
#[doc(alias = "VkCopyMemoryToImageIndirectCommandKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CopyMemoryToImageIndirectCommandKHR {
    pub src_address: DeviceAddress,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: Offset3D,
    pub image_extent: Extent3D,
}
pub type CopyMemoryToImageIndirectCommandNV = CopyMemoryToImageIndirectCommandKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyMemoryToImageIndirectInfoKHR.html>
#[doc(alias = "VkCopyMemoryToImageIndirectInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CopyMemoryToImageIndirectInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_copy_flags: AddressCopyFlagsKHR,
    pub copy_count: u32,
    pub copy_address_range: StridedDeviceAddressRangeKHR,
    pub dst_image: Image,
    pub dst_image_layout: ImageLayout,
    pub p_image_subresources: *const ImageSubresourceLayers,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCopyMemoryIndirectFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceCopyMemoryIndirectFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCopyMemoryIndirectFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub indirect_memory_copy: Bool32,
    pub indirect_memory_to_image_copy: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCopyMemoryIndirectPropertiesKHR.html>
#[doc(alias = "VkPhysicalDeviceCopyMemoryIndirectPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCopyMemoryIndirectPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supported_queues: QueueFlags,
}
pub type PhysicalDeviceCopyMemoryIndirectPropertiesNV =
    PhysicalDeviceCopyMemoryIndirectPropertiesKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeIntraRefreshCapabilitiesKHR.html>
#[doc(alias = "VkVideoEncodeIntraRefreshCapabilitiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeIntraRefreshCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub intra_refresh_modes: VideoEncodeIntraRefreshModeFlagsKHR,
    pub max_intra_refresh_cycle_duration: u32,
    pub max_intra_refresh_active_reference_pictures: u32,
    pub partition_independent_intra_refresh_regions: Bool32,
    pub non_rectangular_intra_refresh_regions: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeSessionIntraRefreshCreateInfoKHR.html>
#[doc(alias = "VkVideoEncodeSessionIntraRefreshCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeSessionIntraRefreshCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub intra_refresh_mode: VideoEncodeIntraRefreshModeFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeIntraRefreshInfoKHR.html>
#[doc(alias = "VkVideoEncodeIntraRefreshInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeIntraRefreshInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub intra_refresh_cycle_duration: u32,
    pub intra_refresh_index: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoReferenceIntraRefreshInfoKHR.html>
#[doc(alias = "VkVideoReferenceIntraRefreshInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoReferenceIntraRefreshInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dirty_intra_refresh_regions: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_encode_intra_refresh: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeQuantizationMapCapabilitiesKHR.html>
#[doc(alias = "VkVideoEncodeQuantizationMapCapabilitiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeQuantizationMapCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_quantization_map_extent: Extent2D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoFormatQuantizationMapPropertiesKHR.html>
#[doc(alias = "VkVideoFormatQuantizationMapPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoFormatQuantizationMapPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub quantization_map_texel_size: Extent2D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeQuantizationMapInfoKHR.html>
#[doc(alias = "VkVideoEncodeQuantizationMapInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeQuantizationMapInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub quantization_map: ImageView,
    pub quantization_map_extent: Extent2D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeQuantizationMapSessionParametersCreateInfoKHR.html>
#[doc(alias = "VkVideoEncodeQuantizationMapSessionParametersCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeQuantizationMapSessionParametersCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub quantization_map_texel_size: Extent2D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_encode_quantization_map: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264QuantizationMapCapabilitiesKHR.html>
#[doc(alias = "VkVideoEncodeH264QuantizationMapCapabilitiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH264QuantizationMapCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_qp_delta: i32,
    pub max_qp_delta: i32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265QuantizationMapCapabilitiesKHR.html>
#[doc(alias = "VkVideoEncodeH265QuantizationMapCapabilitiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeH265QuantizationMapCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_qp_delta: i32,
    pub max_qp_delta: i32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoFormatH265QuantizationMapPropertiesKHR.html>
#[doc(alias = "VkVideoFormatH265QuantizationMapPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoFormatH265QuantizationMapPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub compatible_ctb_sizes: VideoEncodeH265CtbSizeFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1QuantizationMapCapabilitiesKHR.html>
#[doc(alias = "VkVideoEncodeAV1QuantizationMapCapabilitiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeAV1QuantizationMapCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_q_index_delta: i32,
    pub max_q_index_delta: i32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoFormatAV1QuantizationMapPropertiesKHR.html>
#[doc(alias = "VkVideoFormatAV1QuantizationMapPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoFormatAV1QuantizationMapPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub compatible_superblock_sizes: VideoEncodeAV1SuperblockSizeFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderRelaxedExtendedInstructionFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceShaderRelaxedExtendedInstructionFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderRelaxedExtendedInstructionFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_relaxed_extended_instruction: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance7FeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceMaintenance7FeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance7FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub maintenance_7: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance7PropertiesKHR.html>
#[doc(alias = "VkPhysicalDeviceMaintenance7PropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance7PropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub robust_fragment_shading_rate_attachment_access: Bool32,
    pub separate_depth_stencil_attachment_access: Bool32,
    pub max_descriptor_set_total_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_total_storage_buffers_dynamic: u32,
    pub max_descriptor_set_total_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_total_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_total_storage_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_total_buffers_dynamic: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceLayeredApiPropertiesKHR.html>
#[doc(alias = "VkPhysicalDeviceLayeredApiPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceLayeredApiPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub vendor_id: u32,
    pub device_id: u32,
    pub layered_api: PhysicalDeviceLayeredApiKHR,
    pub device_name: [c_char; 256],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceLayeredApiPropertiesListKHR.html>
#[doc(alias = "VkPhysicalDeviceLayeredApiPropertiesListKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceLayeredApiPropertiesListKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub layered_api_count: u32,
    pub p_layered_apis: *mut PhysicalDeviceLayeredApiPropertiesKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceLayeredApiVulkanPropertiesKHR.html>
#[doc(alias = "VkPhysicalDeviceLayeredApiVulkanPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceLayeredApiVulkanPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub properties: PhysicalDeviceProperties2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFaultFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceFaultFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFaultFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_fault: Bool32,
    pub device_fault_vendor_binary: Bool32,
    pub device_fault_report_masked: Bool32,
    pub device_fault_device_lost_on_masked: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFaultPropertiesKHR.html>
#[doc(alias = "VkPhysicalDeviceFaultPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFaultPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_device_fault_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceFaultAddressInfoKHR.html>
#[doc(alias = "VkDeviceFaultAddressInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceFaultAddressInfoKHR {
    pub address_type: DeviceFaultAddressTypeKHR,
    pub reported_address: DeviceAddress,
    pub address_precision: DeviceSize,
}
pub type DeviceFaultAddressInfoEXT = DeviceFaultAddressInfoKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceFaultVendorInfoKHR.html>
#[doc(alias = "VkDeviceFaultVendorInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceFaultVendorInfoKHR {
    pub description: [c_char; 256],
    pub vendor_fault_code: u64,
    pub vendor_fault_data: u64,
}
pub type DeviceFaultVendorInfoEXT = DeviceFaultVendorInfoKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceFaultInfoKHR.html>
#[doc(alias = "VkDeviceFaultInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceFaultInfoKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: DeviceFaultFlagsKHR,
    pub group_id: u64,
    pub description: [c_char; 256],
    pub fault_address_info: DeviceFaultAddressInfoKHR,
    pub instruction_address_info: DeviceFaultAddressInfoKHR,
    pub vendor_info: DeviceFaultVendorInfoKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceFaultDebugInfoKHR.html>
#[doc(alias = "VkDeviceFaultDebugInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceFaultDebugInfoKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub vendor_binary_size: u32,
    pub p_vendor_binary_data: *mut c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceFaultVendorBinaryHeaderVersionOneKHR.html>
#[doc(alias = "VkDeviceFaultVendorBinaryHeaderVersionOneKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceFaultVendorBinaryHeaderVersionOneKHR {
    pub header_size: u32,
    pub header_version: DeviceFaultVendorBinaryHeaderVersionKHR,
    pub vendor_id: u32,
    pub device_id: u32,
    pub driver_version: u32,
    pub pipeline_cache_uuid: [u8; 16],
    pub application_name_offset: u32,
    pub application_version: u32,
    pub engine_name_offset: u32,
    pub engine_version: u32,
    pub api_version: u32,
}
pub type DeviceFaultVendorBinaryHeaderVersionOneEXT = DeviceFaultVendorBinaryHeaderVersionOneKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryBarrierAccessFlags3KHR.html>
#[doc(alias = "VkMemoryBarrierAccessFlags3KHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryBarrierAccessFlags3KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_access_mask_3: AccessFlags3KHR,
    pub dst_access_mask_3: AccessFlags3KHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance8FeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceMaintenance8FeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance8FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub maintenance_8: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderFmaFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceShaderFmaFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderFmaFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_fma_float_16: Bool32,
    pub shader_fma_float_32: Bool32,
    pub shader_fma_float_64: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance9FeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceMaintenance9FeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance9FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub maintenance_9: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance9PropertiesKHR.html>
#[doc(alias = "VkPhysicalDeviceMaintenance9PropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance9PropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_2_d_view_of_3_d_sparse: Bool32,
    pub default_vertex_attribute_value: DefaultVertexAttributeValueKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyOwnershipTransferPropertiesKHR.html>
#[doc(alias = "VkQueueFamilyOwnershipTransferPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QueueFamilyOwnershipTransferPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub optimal_image_transfer_to_queue_families: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVideoMaintenance2FeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceVideoMaintenance2FeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVideoMaintenance2FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_maintenance_2: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeH264InlineSessionParametersInfoKHR.html>
#[doc(alias = "VkVideoDecodeH264InlineSessionParametersInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeH264InlineSessionParametersInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_sps: *const H264SequenceParameterSet,
    pub p_std_pps: *const H264PictureParameterSet,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeH265InlineSessionParametersInfoKHR.html>
#[doc(alias = "VkVideoDecodeH265InlineSessionParametersInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeH265InlineSessionParametersInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_vps: *const H265VideoParameterSet,
    pub p_std_sps: *const H265SequenceParameterSet,
    pub p_std_pps: *const H265PictureParameterSet,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeAV1InlineSessionParametersInfoKHR.html>
#[doc(alias = "VkVideoDecodeAV1InlineSessionParametersInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeAV1InlineSessionParametersInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_sequence_header: *const AV1SequenceHeader,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVideoEncodeFeedback2FeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceVideoEncodeFeedback2FeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVideoEncodeFeedback2FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_encode_feedback_2: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeFeedback2CapabilitiesKHR.html>
#[doc(alias = "VkVideoEncodeFeedback2CapabilitiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeFeedback2CapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_per_partition_feedback_entries: u32,
    pub supported_per_partition_encode_feedback_flags: VideoEncodePerPartitionFeedbackFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPoolVideoEncodePerPartitionFeedbackCreateInfoKHR.html>
#[doc(alias = "VkQueryPoolVideoEncodePerPartitionFeedbackCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QueryPoolVideoEncodePerPartitionFeedbackCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_per_partition_feedback_entries: u32,
    pub per_partition_encode_feedback_flags: VideoEncodePerPartitionFeedbackFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDepthClampZeroOneFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceDepthClampZeroOneFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDepthClampZeroOneFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub depth_clamp_zero_one: Bool32,
}
pub type PhysicalDeviceDepthClampZeroOneFeaturesEXT = PhysicalDeviceDepthClampZeroOneFeaturesKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRobustness2FeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceRobustness2FeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRobustness2FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub robust_buffer_access_2: Bool32,
    pub robust_image_access_2: Bool32,
    pub null_descriptor: Bool32,
}
pub type PhysicalDeviceRobustness2FeaturesEXT = PhysicalDeviceRobustness2FeaturesKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRobustness2PropertiesKHR.html>
#[doc(alias = "VkPhysicalDeviceRobustness2PropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRobustness2PropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub robust_storage_buffer_access_size_alignment: DeviceSize,
    pub robust_uniform_buffer_access_size_alignment: DeviceSize,
}
pub type PhysicalDeviceRobustness2PropertiesEXT = PhysicalDeviceRobustness2PropertiesKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePresentModeFifoLatestReadyFeaturesKHR.html>
#[doc(alias = "VkPhysicalDevicePresentModeFifoLatestReadyFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePresentModeFifoLatestReadyFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_mode_fifo_latest_ready: Bool32,
}
pub type PhysicalDevicePresentModeFifoLatestReadyFeaturesEXT =
    PhysicalDevicePresentModeFifoLatestReadyFeaturesKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapUsageKHR.html>
#[doc(alias = "VkMicromapUsageKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MicromapUsageKHR {
    pub count: u32,
    pub subdivision_level: u32,
    pub format: OpacityMicromapFormatKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureGeometryMicromapDataKHR.html>
#[doc(alias = "VkAccelerationStructureGeometryMicromapDataKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureGeometryMicromapDataKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub usage_counts_count: u32,
    pub p_usage_counts: *const MicromapUsageKHR,
    pub pp_usage_counts: *const *const MicromapUsageKHR,
    pub data: DeviceAddress,
    pub triangle_array: DeviceAddress,
    pub triangle_array_stride: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceOpacityMicromapFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceOpacityMicromapFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceOpacityMicromapFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub micromap: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceOpacityMicromapPropertiesKHR.html>
#[doc(alias = "VkPhysicalDeviceOpacityMicromapPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceOpacityMicromapPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_opacity_2_state_subdivision_level: u32,
    pub max_opacity_4_state_subdivision_level: u32,
    pub max_opacity_lossy_4_state_subdivision_level: u32,
    pub max_micromap_triangles: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapTriangleKHR.html>
#[doc(alias = "VkMicromapTriangleKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MicromapTriangleKHR {
    pub data_offset: u32,
    pub subdivision_level: u16,
    pub format: u16,
}
pub type MicromapTriangleEXT = MicromapTriangleKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureTrianglesOpacityMicromapKHR.html>
#[doc(alias = "VkAccelerationStructureTrianglesOpacityMicromapKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureTrianglesOpacityMicromapKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub index_type: IndexType,
    pub index_buffer: DeviceAddress,
    pub index_stride: DeviceSize,
    pub base_triangle: u32,
    pub micromap: AccelerationStructureKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance10FeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceMaintenance10FeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance10FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub maintenance_10: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance10PropertiesKHR.html>
#[doc(alias = "VkPhysicalDeviceMaintenance10PropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance10PropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub rgba_4_opaque_black_swizzled: Bool32,
    pub resolve_srgb_format_applies_transfer_function: Bool32,
    pub resolve_srgb_format_supports_transfer_function_control: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingEndInfoKHR.html>
#[doc(alias = "VkRenderingEndInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderingEndInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
}
pub type RenderingEndInfoEXT = RenderingEndInfoKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingAttachmentFlagsInfoKHR.html>
#[doc(alias = "VkRenderingAttachmentFlagsInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderingAttachmentFlagsInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: RenderingAttachmentFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkResolveImageModeInfoKHR.html>
#[doc(alias = "VkResolveImageModeInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ResolveImageModeInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ResolveImageFlagsKHR,
    pub resolve_mode: ResolveModeFlags,
    pub stencil_resolve_mode: ResolveModeFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesKHR.html>
#[doc(alias = "VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePipelineLibraryGroupHandlesFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_library_group_handles: Bool32,
}
pub type PhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT =
    PhysicalDevicePipelineLibraryGroupHandlesFeaturesKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance11FeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceMaintenance11FeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance11FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub maintenance_11: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyOptimalImageTransferGranularityPropertiesKHR.html>
#[doc(alias = "VkQueueFamilyOptimalImageTransferGranularityPropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QueueFamilyOptimalImageTransferGranularityPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub optimal_image_transfer_granularity: Extent3D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatProperties4KHR.html>
#[doc(alias = "VkFormatProperties4KHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FormatProperties4KHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub linear_tiling_features: FormatFeatureFlags4KHR,
    pub optimal_tiling_features: FormatFeatureFlags4KHR,
    pub buffer_features: FormatFeatureFlags4KHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageUsageFlags2CreateInfoKHR.html>
#[doc(alias = "VkImageUsageFlags2CreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageUsageFlags2CreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub usage: ImageUsageFlags2KHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCreateFlags2CreateInfoKHR.html>
#[doc(alias = "VkImageCreateFlags2CreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageCreateFlags2CreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: ImageCreateFlags2KHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewUsage2CreateInfoKHR.html>
#[doc(alias = "VkImageViewUsage2CreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageViewUsage2CreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub usage: ImageUsageFlags2KHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedFlagsFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceExtendedFlagsFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExtendedFlagsFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub extended_flags: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageStencilUsage2CreateInfoKHR.html>
#[doc(alias = "VkImageStencilUsage2CreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageStencilUsage2CreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub stencil_usage: ImageUsageFlags2KHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSharedPresentSurfaceCapabilities2KHR.html>
#[doc(alias = "VkSharedPresentSurfaceCapabilities2KHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SharedPresentSurfaceCapabilities2KHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shared_present_supported_usage_flags: ImageUsageFlags2KHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugReportCallbackCreateInfoEXT.html>
#[doc(alias = "VkDebugReportCallbackCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DebugReportCallbackCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DebugReportFlagsEXT,
    pub pfn_callback: vkDebugReportCallbackEXT,
    pub p_user_data: *mut c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationStateRasterizationOrderAMD.html>
#[doc(alias = "VkPipelineRasterizationStateRasterizationOrderAMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineRasterizationStateRasterizationOrderAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub rasterization_order: RasterizationOrderAMD,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugMarkerObjectNameInfoEXT.html>
#[doc(alias = "VkDebugMarkerObjectNameInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DebugMarkerObjectNameInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_type: DebugReportObjectTypeEXT,
    pub object: u64,
    pub p_object_name: *const c_char,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugMarkerObjectTagInfoEXT.html>
#[doc(alias = "VkDebugMarkerObjectTagInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DebugMarkerObjectTagInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_type: DebugReportObjectTypeEXT,
    pub object: u64,
    pub tag_name: u64,
    pub tag_size: usize,
    pub p_tag: *const c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugMarkerMarkerInfoEXT.html>
#[doc(alias = "VkDebugMarkerMarkerInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DebugMarkerMarkerInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_marker_name: *const c_char,
    pub color: [f32; 4],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDedicatedAllocationImageCreateInfoNV.html>
#[doc(alias = "VkDedicatedAllocationImageCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DedicatedAllocationImageCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dedicated_allocation: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDedicatedAllocationBufferCreateInfoNV.html>
#[doc(alias = "VkDedicatedAllocationBufferCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DedicatedAllocationBufferCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dedicated_allocation: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDedicatedAllocationMemoryAllocateInfoNV.html>
#[doc(alias = "VkDedicatedAllocationMemoryAllocateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DedicatedAllocationMemoryAllocateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub buffer: Buffer,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTransformFeedbackFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceTransformFeedbackFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTransformFeedbackFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub transform_feedback: Bool32,
    pub geometry_streams: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTransformFeedbackPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceTransformFeedbackPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTransformFeedbackPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_transform_feedback_streams: u32,
    pub max_transform_feedback_buffers: u32,
    pub max_transform_feedback_buffer_size: DeviceSize,
    pub max_transform_feedback_stream_data_size: u32,
    pub max_transform_feedback_buffer_data_size: u32,
    pub max_transform_feedback_buffer_data_stride: u32,
    pub transform_feedback_queries: Bool32,
    pub transform_feedback_streams_lines_triangles: Bool32,
    pub transform_feedback_rasterization_stream_select: Bool32,
    pub transform_feedback_draw: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationStateStreamCreateInfoEXT.html>
#[doc(alias = "VkPipelineRasterizationStateStreamCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineRasterizationStateStreamCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineRasterizationStateStreamCreateFlagsEXT,
    pub rasterization_stream: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCuModuleCreateInfoNVX.html>
#[doc(alias = "VkCuModuleCreateInfoNVX")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CuModuleCreateInfoNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub data_size: usize,
    pub p_data: *const c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCuModuleTexturingModeCreateInfoNVX.html>
#[doc(alias = "VkCuModuleTexturingModeCreateInfoNVX")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CuModuleTexturingModeCreateInfoNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_64_bit_texturing: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCuFunctionCreateInfoNVX.html>
#[doc(alias = "VkCuFunctionCreateInfoNVX")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CuFunctionCreateInfoNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub module: CuModuleNVX,
    pub p_name: *const c_char,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCuLaunchInfoNVX.html>
#[doc(alias = "VkCuLaunchInfoNVX")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CuLaunchInfoNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub function: CuFunctionNVX,
    pub grid_dim_x: u32,
    pub grid_dim_y: u32,
    pub grid_dim_z: u32,
    pub block_dim_x: u32,
    pub block_dim_y: u32,
    pub block_dim_z: u32,
    pub shared_mem_bytes: u32,
    pub param_count: usize,
    pub p_params: *const *const c_void,
    pub extra_count: usize,
    pub p_extras: *const *const c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewHandleInfoNVX.html>
#[doc(alias = "VkImageViewHandleInfoNVX")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageViewHandleInfoNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_view: ImageView,
    pub descriptor_type: DescriptorType,
    pub sampler: Sampler,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewAddressPropertiesNVX.html>
#[doc(alias = "VkImageViewAddressPropertiesNVX")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageViewAddressPropertiesNVX {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_address: DeviceAddress,
    pub size: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTextureLODGatherFormatPropertiesAMD.html>
#[doc(alias = "VkTextureLODGatherFormatPropertiesAMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TextureLODGatherFormatPropertiesAMD {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supports_texture_gather_lod_bias_amd: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderResourceUsageAMD.html>
#[doc(alias = "VkShaderResourceUsageAMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ShaderResourceUsageAMD {
    pub num_used_vgprs: u32,
    pub num_used_sgprs: u32,
    pub lds_size_per_local_work_group: u32,
    pub lds_usage_size_in_bytes: usize,
    pub scratch_mem_usage_in_bytes: usize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderStatisticsInfoAMD.html>
#[doc(alias = "VkShaderStatisticsInfoAMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ShaderStatisticsInfoAMD {
    pub shader_stage_mask: ShaderStageFlags,
    pub resource_usage: ShaderResourceUsageAMD,
    pub num_physical_vgprs: u32,
    pub num_physical_sgprs: u32,
    pub num_available_vgprs: u32,
    pub num_available_sgprs: u32,
    pub compute_work_group_size: [u32; 3],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkStreamDescriptorSurfaceCreateInfoGGP.html>
#[doc(alias = "VkStreamDescriptorSurfaceCreateInfoGGP")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct StreamDescriptorSurfaceCreateInfoGGP {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: StreamDescriptorSurfaceCreateFlagsGGP,
    pub stream_descriptor: GgpStreamDescriptor,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCornerSampledImageFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceCornerSampledImageFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCornerSampledImageFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub corner_sampled_image: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalImageFormatPropertiesNV.html>
#[doc(alias = "VkExternalImageFormatPropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExternalImageFormatPropertiesNV {
    pub image_format_properties: ImageFormatProperties,
    pub external_memory_features: ExternalMemoryFeatureFlagsNV,
    pub export_from_imported_handle_types: ExternalMemoryHandleTypeFlagsNV,
    pub compatible_handle_types: ExternalMemoryHandleTypeFlagsNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryImageCreateInfoNV.html>
#[doc(alias = "VkExternalMemoryImageCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExternalMemoryImageCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlagsNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExportMemoryAllocateInfoNV.html>
#[doc(alias = "VkExportMemoryAllocateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExportMemoryAllocateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlagsNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImportMemoryWin32HandleInfoNV.html>
#[doc(alias = "VkImportMemoryWin32HandleInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImportMemoryWin32HandleInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlagsNV,
    pub handle: HANDLE,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExportMemoryWin32HandleInfoNV.html>
#[doc(alias = "VkExportMemoryWin32HandleInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExportMemoryWin32HandleInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_attributes: *const SECURITY_ATTRIBUTES,
    pub dw_access: DWORD,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkWin32KeyedMutexAcquireReleaseInfoNV.html>
#[doc(alias = "VkWin32KeyedMutexAcquireReleaseInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Win32KeyedMutexAcquireReleaseInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acquire_count: u32,
    pub p_acquire_syncs: *const DeviceMemory,
    pub p_acquire_keys: *const u64,
    pub p_acquire_timeout_milliseconds: *const u32,
    pub release_count: u32,
    pub p_release_syncs: *const DeviceMemory,
    pub p_release_keys: *const u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkValidationFlagsEXT.html>
#[doc(alias = "VkValidationFlagsEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ValidationFlagsEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub disabled_validation_check_count: u32,
    pub p_disabled_validation_checks: *const ValidationCheckEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkViSurfaceCreateInfoNN.html>
#[doc(alias = "VkViSurfaceCreateInfoNN")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ViSurfaceCreateInfoNN {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ViSurfaceCreateFlagsNN,
    pub window: *mut c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewASTCDecodeModeEXT.html>
#[doc(alias = "VkImageViewASTCDecodeModeEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageViewASTCDecodeModeEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub decode_mode: Format,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceASTCDecodeFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceASTCDecodeFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceASTCDecodeFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub decode_mode_shared_exponent: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkConditionalRenderingBeginInfoEXT.html>
#[doc(alias = "VkConditionalRenderingBeginInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ConditionalRenderingBeginInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub flags: ConditionalRenderingFlagsEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceConditionalRenderingFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceConditionalRenderingFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceConditionalRenderingFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub conditional_rendering: Bool32,
    pub inherited_conditional_rendering: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferInheritanceConditionalRenderingInfoEXT.html>
#[doc(alias = "VkCommandBufferInheritanceConditionalRenderingInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CommandBufferInheritanceConditionalRenderingInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub conditional_rendering_enable: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkViewportWScalingNV.html>
#[doc(alias = "VkViewportWScalingNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ViewportWScalingNV {
    pub xcoeff: f32,
    pub ycoeff: f32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportWScalingStateCreateInfoNV.html>
#[doc(alias = "VkPipelineViewportWScalingStateCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineViewportWScalingStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub viewport_w_scaling_enable: Bool32,
    pub viewport_count: u32,
    pub p_viewport_w_scalings: *const ViewportWScalingNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCapabilities2EXT.html>
#[doc(alias = "VkSurfaceCapabilities2EXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SurfaceCapabilities2EXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_image_count: u32,
    pub max_image_count: u32,
    pub current_extent: Extent2D,
    pub min_image_extent: Extent2D,
    pub max_image_extent: Extent2D,
    pub max_image_array_layers: u32,
    pub supported_transforms: SurfaceTransformFlagsKHR,
    pub current_transform: SurfaceTransformFlagsKHR,
    pub supported_composite_alpha: CompositeAlphaFlagsKHR,
    pub supported_usage_flags: ImageUsageFlags,
    pub supported_surface_counters: SurfaceCounterFlagsEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayPowerInfoEXT.html>
#[doc(alias = "VkDisplayPowerInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DisplayPowerInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub power_state: DisplayPowerStateEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceEventInfoEXT.html>
#[doc(alias = "VkDeviceEventInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceEventInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_event: DeviceEventTypeEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayEventInfoEXT.html>
#[doc(alias = "VkDisplayEventInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DisplayEventInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub display_event: DisplayEventTypeEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainCounterCreateInfoEXT.html>
#[doc(alias = "VkSwapchainCounterCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SwapchainCounterCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub surface_counters: SurfaceCounterFlagsEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRefreshCycleDurationGOOGLE.html>
#[doc(alias = "VkRefreshCycleDurationGOOGLE")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RefreshCycleDurationGOOGLE {
    pub refresh_duration: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPastPresentationTimingGOOGLE.html>
#[doc(alias = "VkPastPresentationTimingGOOGLE")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PastPresentationTimingGOOGLE {
    pub present_id: u32,
    pub desired_present_time: u64,
    pub actual_present_time: u64,
    pub earliest_present_time: u64,
    pub present_margin: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentTimeGOOGLE.html>
#[doc(alias = "VkPresentTimeGOOGLE")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PresentTimeGOOGLE {
    pub present_id: u32,
    pub desired_present_time: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentTimesInfoGOOGLE.html>
#[doc(alias = "VkPresentTimesInfoGOOGLE")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PresentTimesInfoGOOGLE {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_times: *const PresentTimeGOOGLE,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX.html>
#[doc(alias = "VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub per_view_position_all_components: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMultiviewPerViewAttributesInfoNVX.html>
#[doc(alias = "VkMultiviewPerViewAttributesInfoNVX")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MultiviewPerViewAttributesInfoNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub per_view_attributes: Bool32,
    pub per_view_attributes_position_x_only: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkViewportSwizzleNV.html>
#[doc(alias = "VkViewportSwizzleNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ViewportSwizzleNV {
    pub x: ViewportCoordinateSwizzleNV,
    pub y: ViewportCoordinateSwizzleNV,
    pub z: ViewportCoordinateSwizzleNV,
    pub w: ViewportCoordinateSwizzleNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportSwizzleStateCreateInfoNV.html>
#[doc(alias = "VkPipelineViewportSwizzleStateCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineViewportSwizzleStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineViewportSwizzleStateCreateFlagsNV,
    pub viewport_count: u32,
    pub p_viewport_swizzles: *const ViewportSwizzleNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDiscardRectanglePropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceDiscardRectanglePropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDiscardRectanglePropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_discard_rectangles: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineDiscardRectangleStateCreateInfoEXT.html>
#[doc(alias = "VkPipelineDiscardRectangleStateCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineDiscardRectangleStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineDiscardRectangleStateCreateFlagsEXT,
    pub discard_rectangle_mode: DiscardRectangleModeEXT,
    pub discard_rectangle_count: u32,
    pub p_discard_rectangles: *const Rect2D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceConservativeRasterizationPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceConservativeRasterizationPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceConservativeRasterizationPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub primitive_overestimation_size: f32,
    pub max_extra_primitive_overestimation_size: f32,
    pub extra_primitive_overestimation_size_granularity: f32,
    pub primitive_underestimation: Bool32,
    pub conservative_point_and_line_rasterization: Bool32,
    pub degenerate_triangles_rasterized: Bool32,
    pub degenerate_lines_rasterized: Bool32,
    pub fully_covered_fragment_shader_input_variable: Bool32,
    pub conservative_rasterization_post_depth_coverage: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationConservativeStateCreateInfoEXT.html>
#[doc(alias = "VkPipelineRasterizationConservativeStateCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineRasterizationConservativeStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineRasterizationConservativeStateCreateFlagsEXT,
    pub conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    pub extra_primitive_overestimation_size: f32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDepthClipEnableFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceDepthClipEnableFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDepthClipEnableFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub depth_clip_enable: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationDepthClipStateCreateInfoEXT.html>
#[doc(alias = "VkPipelineRasterizationDepthClipStateCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineRasterizationDepthClipStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineRasterizationDepthClipStateCreateFlagsEXT,
    pub depth_clip_enable: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkXYColorEXT.html>
#[doc(alias = "VkXYColorEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XYColorEXT {
    pub x: f32,
    pub y: f32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkHdrMetadataEXT.html>
#[doc(alias = "VkHdrMetadataEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct HdrMetadataEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub display_primary_red: XYColorEXT,
    pub display_primary_green: XYColorEXT,
    pub display_primary_blue: XYColorEXT,
    pub white_point: XYColorEXT,
    pub max_luminance: f32,
    pub min_luminance: f32,
    pub max_content_light_level: f32,
    pub max_frame_average_light_level: f32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG.html>
#[doc(alias = "VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRelaxedLineRasterizationFeaturesIMG {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub relaxed_line_rasterization: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIOSSurfaceCreateInfoMVK.html>
#[doc(alias = "VkIOSSurfaceCreateInfoMVK")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct IOSSurfaceCreateInfoMVK {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: IOSSurfaceCreateFlagsMVK,
    pub p_view: *const c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMacOSSurfaceCreateInfoMVK.html>
#[doc(alias = "VkMacOSSurfaceCreateInfoMVK")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MacOSSurfaceCreateInfoMVK {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: MacOSSurfaceCreateFlagsMVK,
    pub p_view: *const c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsLabelEXT.html>
#[doc(alias = "VkDebugUtilsLabelEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DebugUtilsLabelEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_label_name: *const c_char,
    pub color: [f32; 4],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsObjectNameInfoEXT.html>
#[doc(alias = "VkDebugUtilsObjectNameInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DebugUtilsObjectNameInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_type: ObjectType,
    pub object_handle: u64,
    pub p_object_name: *const c_char,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessengerCallbackDataEXT.html>
#[doc(alias = "VkDebugUtilsMessengerCallbackDataEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DebugUtilsMessengerCallbackDataEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DebugUtilsMessengerCallbackDataFlagsEXT,
    pub p_message_id_name: *const c_char,
    pub message_id_number: i32,
    pub p_message: *const c_char,
    pub queue_label_count: u32,
    pub p_queue_labels: *const DebugUtilsLabelEXT,
    pub cmd_buf_label_count: u32,
    pub p_cmd_buf_labels: *const DebugUtilsLabelEXT,
    pub object_count: u32,
    pub p_objects: *const DebugUtilsObjectNameInfoEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessengerCreateInfoEXT.html>
#[doc(alias = "VkDebugUtilsMessengerCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DebugUtilsMessengerCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DebugUtilsMessengerCreateFlagsEXT,
    pub message_severity: DebugUtilsMessageSeverityFlagsEXT,
    pub message_type: DebugUtilsMessageTypeFlagsEXT,
    pub pfn_user_callback: vkDebugUtilsMessengerCallbackEXT,
    pub p_user_data: *mut c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsObjectTagInfoEXT.html>
#[doc(alias = "VkDebugUtilsObjectTagInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DebugUtilsObjectTagInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_type: ObjectType,
    pub object_handle: u64,
    pub tag_name: u64,
    pub tag_size: usize,
    pub p_tag: *const c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAndroidHardwareBufferUsageANDROID.html>
#[doc(alias = "VkAndroidHardwareBufferUsageANDROID")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AndroidHardwareBufferUsageANDROID {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub android_hardware_buffer_usage: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAndroidHardwareBufferPropertiesANDROID.html>
#[doc(alias = "VkAndroidHardwareBufferPropertiesANDROID")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AndroidHardwareBufferPropertiesANDROID {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub allocation_size: DeviceSize,
    pub memory_type_bits: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAndroidHardwareBufferFormatPropertiesANDROID.html>
#[doc(alias = "VkAndroidHardwareBufferFormatPropertiesANDROID")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AndroidHardwareBufferFormatPropertiesANDROID {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format: Format,
    pub external_format: u64,
    pub format_features: FormatFeatureFlags,
    pub sampler_ycbcr_conversion_components: ComponentMapping,
    pub suggested_ycbcr_model: SamplerYcbcrModelConversion,
    pub suggested_ycbcr_range: SamplerYcbcrRange,
    pub suggested_x_chroma_offset: ChromaLocation,
    pub suggested_y_chroma_offset: ChromaLocation,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImportAndroidHardwareBufferInfoANDROID.html>
#[doc(alias = "VkImportAndroidHardwareBufferInfoANDROID")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImportAndroidHardwareBufferInfoANDROID {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: *mut AHardwareBuffer,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryGetAndroidHardwareBufferInfoANDROID.html>
#[doc(alias = "VkMemoryGetAndroidHardwareBufferInfoANDROID")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryGetAndroidHardwareBufferInfoANDROID {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFormatANDROID.html>
#[doc(alias = "VkExternalFormatANDROID")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExternalFormatANDROID {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_format: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAndroidHardwareBufferFormatProperties2ANDROID.html>
#[doc(alias = "VkAndroidHardwareBufferFormatProperties2ANDROID")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AndroidHardwareBufferFormatProperties2ANDROID {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format: Format,
    pub external_format: u64,
    pub format_features: FormatFeatureFlags2,
    pub sampler_ycbcr_conversion_components: ComponentMapping,
    pub suggested_ycbcr_model: SamplerYcbcrModelConversion,
    pub suggested_ycbcr_range: SamplerYcbcrRange,
    pub suggested_x_chroma_offset: ChromaLocation,
    pub suggested_y_chroma_offset: ChromaLocation,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaPerfBlockPropertiesAMD.html>
#[doc(alias = "VkGpaPerfBlockPropertiesAMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GpaPerfBlockPropertiesAMD {
    pub block_type: GpaPerfBlockAMD,
    pub flags: GpaPerfBlockPropertiesFlagsAMD,
    pub instance_count: u32,
    pub max_event_id: u32,
    pub max_global_only_counters: u32,
    pub max_global_shared_counters: u32,
    pub max_streaming_counters: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceGpaFeaturesAMD.html>
#[doc(alias = "VkPhysicalDeviceGpaFeaturesAMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceGpaFeaturesAMD {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub perf_counters: Bool32,
    pub streaming_perf_counters: Bool32,
    pub sq_thread_tracing: Bool32,
    pub clock_modes: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceGpaPropertiesAMD.html>
#[doc(alias = "VkPhysicalDeviceGpaPropertiesAMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceGpaPropertiesAMD {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: PhysicalDeviceGpaPropertiesFlagsAMD,
    pub max_sqtt_se_buffer_size: DeviceSize,
    pub shader_engine_count: u32,
    pub perf_block_count: u32,
    pub p_perf_blocks: *mut GpaPerfBlockPropertiesAMD,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceGpaProperties2AMD.html>
#[doc(alias = "VkPhysicalDeviceGpaProperties2AMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceGpaProperties2AMD {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub revision_id: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaPerfCounterAMD.html>
#[doc(alias = "VkGpaPerfCounterAMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GpaPerfCounterAMD {
    pub block_type: GpaPerfBlockAMD,
    pub block_instance: u32,
    pub event_id: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaSampleBeginInfoAMD.html>
#[doc(alias = "VkGpaSampleBeginInfoAMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GpaSampleBeginInfoAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sample_type: GpaSampleTypeAMD,
    pub sample_internal_operations: Bool32,
    pub cache_flush_on_counter_collection: Bool32,
    pub sq_shader_mask_enable: Bool32,
    pub sq_shader_mask: GpaSqShaderStageFlagsAMD,
    pub perf_counter_count: u32,
    pub p_perf_counters: *const GpaPerfCounterAMD,
    pub streaming_perf_trace_sample_interval: u32,
    pub perf_counter_device_memory_limit: DeviceSize,
    pub sq_thread_trace_enable: Bool32,
    pub sq_thread_trace_suppress_instruction_tokens: Bool32,
    pub sq_thread_trace_device_memory_limit: DeviceSize,
    pub timing_pre_sample: PipelineStageFlags,
    pub timing_post_sample: PipelineStageFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaDeviceClockModeInfoAMD.html>
#[doc(alias = "VkGpaDeviceClockModeInfoAMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GpaDeviceClockModeInfoAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub clock_mode: GpaDeviceClockModeAMD,
    pub memory_clock_ratio_to_peak: f32,
    pub engine_clock_ratio_to_peak: f32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaDeviceGetClockInfoAMD.html>
#[doc(alias = "VkGpaDeviceGetClockInfoAMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GpaDeviceGetClockInfoAMD {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_clock_ratio_to_peak: f32,
    pub engine_clock_ratio_to_peak: f32,
    pub memory_clock_frequency: u32,
    pub engine_clock_frequency: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaSessionCreateInfoAMD.html>
#[doc(alias = "VkGpaSessionCreateInfoAMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GpaSessionCreateInfoAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub secondary_copy_source: GpaSessionAMD,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderEnqueueFeaturesAMDX.html>
#[doc(alias = "VkPhysicalDeviceShaderEnqueueFeaturesAMDX")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderEnqueueFeaturesAMDX {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_enqueue: Bool32,
    pub shader_mesh_enqueue: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderEnqueuePropertiesAMDX.html>
#[doc(alias = "VkPhysicalDeviceShaderEnqueuePropertiesAMDX")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderEnqueuePropertiesAMDX {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_execution_graph_depth: u32,
    pub max_execution_graph_shader_output_nodes: u32,
    pub max_execution_graph_shader_payload_size: u32,
    pub max_execution_graph_shader_payload_count: u32,
    pub execution_graph_dispatch_address_alignment: u32,
    pub max_execution_graph_workgroup_count: [u32; 3],
    pub max_execution_graph_workgroups: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExecutionGraphPipelineScratchSizeAMDX.html>
#[doc(alias = "VkExecutionGraphPipelineScratchSizeAMDX")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExecutionGraphPipelineScratchSizeAMDX {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_size: DeviceSize,
    pub max_size: DeviceSize,
    pub size_granularity: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExecutionGraphPipelineCreateInfoAMDX.html>
#[doc(alias = "VkExecutionGraphPipelineCreateInfoAMDX")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExecutionGraphPipelineCreateInfoAMDX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage_count: u32,
    pub p_stages: *const PipelineShaderStageCreateInfo,
    pub p_library_info: *const PipelineLibraryCreateInfoKHR,
    pub layout: PipelineLayout,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceOrHostAddressConstAMDX.html>
#[doc(alias = "VkDeviceOrHostAddressConstAMDX")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceOrHostAddressConstAMDX {
    pub device_address: DeviceAddress,
    pub host_address: *const c_void,
}
impl std::fmt::Debug for DeviceOrHostAddressConstAMDX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeviceOrHostAddressConstAMDX {{ .. }}")
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDispatchGraphInfoAMDX.html>
#[doc(alias = "VkDispatchGraphInfoAMDX")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DispatchGraphInfoAMDX {
    pub node_index: u32,
    pub payload_count: u32,
    pub payloads: DeviceOrHostAddressConstAMDX,
    pub payload_stride: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDispatchGraphCountInfoAMDX.html>
#[doc(alias = "VkDispatchGraphCountInfoAMDX")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DispatchGraphCountInfoAMDX {
    pub count: u32,
    pub infos: DeviceOrHostAddressConstAMDX,
    pub stride: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineShaderStageNodeCreateInfoAMDX.html>
#[doc(alias = "VkPipelineShaderStageNodeCreateInfoAMDX")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineShaderStageNodeCreateInfoAMDX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_name: *const c_char,
    pub index: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkHostAddressRangeEXT.html>
#[doc(alias = "VkHostAddressRangeEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct HostAddressRangeEXT {
    pub address: *mut c_void,
    pub size: usize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkHostAddressRangeConstEXT.html>
#[doc(alias = "VkHostAddressRangeConstEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct HostAddressRangeConstEXT {
    pub address: *const c_void,
    pub size: usize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTexelBufferDescriptorInfoEXT.html>
#[doc(alias = "VkTexelBufferDescriptorInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TexelBufferDescriptorInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format: Format,
    pub address_range: DeviceAddressRangeEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageDescriptorInfoEXT.html>
#[doc(alias = "VkImageDescriptorInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageDescriptorInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_view: *const ImageViewCreateInfo,
    pub layout: ImageLayout,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorViewCreateInfoARM.html>
#[doc(alias = "VkTensorViewCreateInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TensorViewCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: TensorViewCreateFlagsARM,
    pub tensor: TensorARM,
    pub format: Format,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkResourceDescriptorDataEXT.html>
#[doc(alias = "VkResourceDescriptorDataEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ResourceDescriptorDataEXT {
    pub p_image: *const ImageDescriptorInfoEXT,
    pub p_texel_buffer: *const TexelBufferDescriptorInfoEXT,
    pub p_address_range: *const DeviceAddressRangeEXT,
    pub p_tensor_arm: *const TensorViewCreateInfoARM,
}
impl std::fmt::Debug for ResourceDescriptorDataEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceDescriptorDataEXT {{ .. }}")
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkResourceDescriptorInfoEXT.html>
#[doc(alias = "VkResourceDescriptorInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ResourceDescriptorInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub type_: DescriptorType,
    pub data: ResourceDescriptorDataEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindHeapInfoEXT.html>
#[doc(alias = "VkBindHeapInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BindHeapInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub heap_range: DeviceAddressRangeEXT,
    pub reserved_range_offset: DeviceSize,
    pub reserved_range_size: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPushDataInfoEXT.html>
#[doc(alias = "VkPushDataInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PushDataInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub offset: u32,
    pub data: HostAddressRangeConstEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorMappingSourceConstantOffsetEXT.html>
#[doc(alias = "VkDescriptorMappingSourceConstantOffsetEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorMappingSourceConstantOffsetEXT {
    pub heap_offset: u32,
    pub heap_array_stride: u32,
    pub p_embedded_sampler: *const SamplerCreateInfo,
    pub sampler_heap_offset: u32,
    pub sampler_heap_array_stride: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorMappingSourcePushIndexEXT.html>
#[doc(alias = "VkDescriptorMappingSourcePushIndexEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorMappingSourcePushIndexEXT {
    pub heap_offset: u32,
    pub push_offset: u32,
    pub heap_index_stride: u32,
    pub heap_array_stride: u32,
    pub p_embedded_sampler: *const SamplerCreateInfo,
    pub use_combined_image_sampler_index: Bool32,
    pub sampler_heap_offset: u32,
    pub sampler_push_offset: u32,
    pub sampler_heap_index_stride: u32,
    pub sampler_heap_array_stride: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorMappingSourceIndirectIndexEXT.html>
#[doc(alias = "VkDescriptorMappingSourceIndirectIndexEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorMappingSourceIndirectIndexEXT {
    pub heap_offset: u32,
    pub push_offset: u32,
    pub address_offset: u32,
    pub heap_index_stride: u32,
    pub heap_array_stride: u32,
    pub p_embedded_sampler: *const SamplerCreateInfo,
    pub use_combined_image_sampler_index: Bool32,
    pub sampler_heap_offset: u32,
    pub sampler_push_offset: u32,
    pub sampler_address_offset: u32,
    pub sampler_heap_index_stride: u32,
    pub sampler_heap_array_stride: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorMappingSourceHeapDataEXT.html>
#[doc(alias = "VkDescriptorMappingSourceHeapDataEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorMappingSourceHeapDataEXT {
    pub heap_offset: u32,
    pub push_offset: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorMappingSourceIndirectAddressEXT.html>
#[doc(alias = "VkDescriptorMappingSourceIndirectAddressEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorMappingSourceIndirectAddressEXT {
    pub push_offset: u32,
    pub address_offset: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorMappingSourceShaderRecordIndexEXT.html>
#[doc(alias = "VkDescriptorMappingSourceShaderRecordIndexEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorMappingSourceShaderRecordIndexEXT {
    pub heap_offset: u32,
    pub shader_record_offset: u32,
    pub heap_index_stride: u32,
    pub heap_array_stride: u32,
    pub p_embedded_sampler: *const SamplerCreateInfo,
    pub use_combined_image_sampler_index: Bool32,
    pub sampler_heap_offset: u32,
    pub sampler_shader_record_offset: u32,
    pub sampler_heap_index_stride: u32,
    pub sampler_heap_array_stride: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorMappingSourceIndirectIndexArrayEXT.html>
#[doc(alias = "VkDescriptorMappingSourceIndirectIndexArrayEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorMappingSourceIndirectIndexArrayEXT {
    pub heap_offset: u32,
    pub push_offset: u32,
    pub address_offset: u32,
    pub heap_index_stride: u32,
    pub p_embedded_sampler: *const SamplerCreateInfo,
    pub use_combined_image_sampler_index: Bool32,
    pub sampler_heap_offset: u32,
    pub sampler_push_offset: u32,
    pub sampler_address_offset: u32,
    pub sampler_heap_index_stride: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorMappingSourceDataEXT.html>
#[doc(alias = "VkDescriptorMappingSourceDataEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DescriptorMappingSourceDataEXT {
    pub constant_offset: DescriptorMappingSourceConstantOffsetEXT,
    pub push_index: DescriptorMappingSourcePushIndexEXT,
    pub indirect_index: DescriptorMappingSourceIndirectIndexEXT,
    pub indirect_index_array: DescriptorMappingSourceIndirectIndexArrayEXT,
    pub heap_data: DescriptorMappingSourceHeapDataEXT,
    pub push_data_offset: u32,
    pub push_address_offset: u32,
    pub indirect_address: DescriptorMappingSourceIndirectAddressEXT,
    pub shader_record_index: DescriptorMappingSourceShaderRecordIndexEXT,
    pub shader_record_data_offset: u32,
    pub shader_record_address_offset: u32,
}
impl std::fmt::Debug for DescriptorMappingSourceDataEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DescriptorMappingSourceDataEXT {{ .. }}")
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetAndBindingMappingEXT.html>
#[doc(alias = "VkDescriptorSetAndBindingMappingEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorSetAndBindingMappingEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub descriptor_set: u32,
    pub first_binding: u32,
    pub binding_count: u32,
    pub resource_mask: SpirvResourceTypeFlagsEXT,
    pub source: DescriptorMappingSourceEXT,
    pub source_data: DescriptorMappingSourceDataEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderDescriptorSetAndBindingMappingInfoEXT.html>
#[doc(alias = "VkShaderDescriptorSetAndBindingMappingInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ShaderDescriptorSetAndBindingMappingInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mapping_count: u32,
    pub p_mappings: *const DescriptorSetAndBindingMappingEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpaqueCaptureDataCreateInfoEXT.html>
#[doc(alias = "VkOpaqueCaptureDataCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct OpaqueCaptureDataCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_data: *const HostAddressRangeConstEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDescriptorHeapFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceDescriptorHeapFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDescriptorHeapFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub descriptor_heap: Bool32,
    pub descriptor_heap_capture_replay: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDescriptorHeapPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceDescriptorHeapPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDescriptorHeapPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub sampler_heap_alignment: DeviceSize,
    pub resource_heap_alignment: DeviceSize,
    pub max_sampler_heap_size: DeviceSize,
    pub max_resource_heap_size: DeviceSize,
    pub min_sampler_heap_reserved_range: DeviceSize,
    pub min_sampler_heap_reserved_range_with_embedded: DeviceSize,
    pub min_resource_heap_reserved_range: DeviceSize,
    pub sampler_descriptor_size: DeviceSize,
    pub image_descriptor_size: DeviceSize,
    pub buffer_descriptor_size: DeviceSize,
    pub sampler_descriptor_alignment: DeviceSize,
    pub image_descriptor_alignment: DeviceSize,
    pub buffer_descriptor_alignment: DeviceSize,
    pub max_push_data_size: DeviceSize,
    pub image_capture_replay_opaque_data_size: usize,
    pub max_descriptor_heap_embedded_samplers: u32,
    pub sampler_ycbcr_conversion_count: u32,
    pub sparse_descriptor_heaps: Bool32,
    pub protected_descriptor_heaps: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferInheritanceDescriptorHeapInfoEXT.html>
#[doc(alias = "VkCommandBufferInheritanceDescriptorHeapInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CommandBufferInheritanceDescriptorHeapInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_sampler_heap_bind_info: *const BindHeapInfoEXT,
    pub p_resource_heap_bind_info: *const BindHeapInfoEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerCustomBorderColorIndexCreateInfoEXT.html>
#[doc(alias = "VkSamplerCustomBorderColorIndexCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SamplerCustomBorderColorIndexCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub index: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerCustomBorderColorCreateInfoEXT.html>
#[doc(alias = "VkSamplerCustomBorderColorCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SamplerCustomBorderColorCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub custom_border_color: ClearColorValue,
    pub format: Format,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutPushDataTokenNV.html>
#[doc(alias = "VkIndirectCommandsLayoutPushDataTokenNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct IndirectCommandsLayoutPushDataTokenNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub push_data_offset: u32,
    pub push_data_size: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubsampledImageFormatPropertiesEXT.html>
#[doc(alias = "VkSubsampledImageFormatPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SubsampledImageFormatPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub subsampled_image_descriptor_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDescriptorHeapTensorPropertiesARM.html>
#[doc(alias = "VkPhysicalDeviceDescriptorHeapTensorPropertiesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDescriptorHeapTensorPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tensor_descriptor_size: DeviceSize,
    pub tensor_descriptor_alignment: DeviceSize,
    pub tensor_capture_replay_opaque_data_size: usize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentSampleCountInfoAMD.html>
#[doc(alias = "VkAttachmentSampleCountInfoAMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AttachmentSampleCountInfoAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub color_attachment_count: u32,
    pub p_color_attachment_samples: *const SampleCountFlags,
    pub depth_stencil_attachment_samples: SampleCountFlags,
}
pub type AttachmentSampleCountInfoNV = AttachmentSampleCountInfoAMD;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSampleLocationEXT.html>
#[doc(alias = "VkSampleLocationEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SampleLocationEXT {
    pub x: f32,
    pub y: f32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSampleLocationsInfoEXT.html>
#[doc(alias = "VkSampleLocationsInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SampleLocationsInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sample_locations_per_pixel: SampleCountFlags,
    pub sample_location_grid_size: Extent2D,
    pub sample_locations_count: u32,
    pub p_sample_locations: *const SampleLocationEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentSampleLocationsEXT.html>
#[doc(alias = "VkAttachmentSampleLocationsEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AttachmentSampleLocationsEXT {
    pub attachment_index: u32,
    pub sample_locations_info: SampleLocationsInfoEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassSampleLocationsEXT.html>
#[doc(alias = "VkSubpassSampleLocationsEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SubpassSampleLocationsEXT {
    pub subpass_index: u32,
    pub sample_locations_info: SampleLocationsInfoEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassSampleLocationsBeginInfoEXT.html>
#[doc(alias = "VkRenderPassSampleLocationsBeginInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderPassSampleLocationsBeginInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub attachment_initial_sample_locations_count: u32,
    pub p_attachment_initial_sample_locations: *const AttachmentSampleLocationsEXT,
    pub post_subpass_sample_locations_count: u32,
    pub p_post_subpass_sample_locations: *const SubpassSampleLocationsEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineSampleLocationsStateCreateInfoEXT.html>
#[doc(alias = "VkPipelineSampleLocationsStateCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineSampleLocationsStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sample_locations_enable: Bool32,
    pub sample_locations_info: SampleLocationsInfoEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSampleLocationsPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceSampleLocationsPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSampleLocationsPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub sample_location_sample_counts: SampleCountFlags,
    pub max_sample_location_grid_size: Extent2D,
    pub sample_location_coordinate_range: [f32; 2],
    pub sample_location_sub_pixel_bits: u32,
    pub variable_sample_locations: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMultisamplePropertiesEXT.html>
#[doc(alias = "VkMultisamplePropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MultisamplePropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_sample_location_grid_size: Extent2D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub advanced_blend_coherent_operations: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub advanced_blend_max_color_attachments: u32,
    pub advanced_blend_independent_blend: Bool32,
    pub advanced_blend_non_premultiplied_src_color: Bool32,
    pub advanced_blend_non_premultiplied_dst_color: Bool32,
    pub advanced_blend_correlated_overlap: Bool32,
    pub advanced_blend_all_operations: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineColorBlendAdvancedStateCreateInfoEXT.html>
#[doc(alias = "VkPipelineColorBlendAdvancedStateCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineColorBlendAdvancedStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_premultiplied: Bool32,
    pub dst_premultiplied: Bool32,
    pub blend_overlap: BlendOverlapEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCoverageToColorStateCreateInfoNV.html>
#[doc(alias = "VkPipelineCoverageToColorStateCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineCoverageToColorStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCoverageToColorStateCreateFlagsNV,
    pub coverage_to_color_enable: Bool32,
    pub coverage_to_color_location: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCoverageModulationStateCreateInfoNV.html>
#[doc(alias = "VkPipelineCoverageModulationStateCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineCoverageModulationStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCoverageModulationStateCreateFlagsNV,
    pub coverage_modulation_mode: CoverageModulationModeNV,
    pub coverage_modulation_table_enable: Bool32,
    pub coverage_modulation_table_count: u32,
    pub p_coverage_modulation_table: *const f32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderSMBuiltinsPropertiesNV.html>
#[doc(alias = "VkPhysicalDeviceShaderSMBuiltinsPropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderSMBuiltinsPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_sm_count: u32,
    pub shader_warps_per_sm: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderSMBuiltinsFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceShaderSMBuiltinsFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderSMBuiltinsFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_sm_builtins: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDrmFormatModifierPropertiesEXT.html>
#[doc(alias = "VkDrmFormatModifierPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DrmFormatModifierPropertiesEXT {
    pub drm_format_modifier: u64,
    pub drm_format_modifier_plane_count: u32,
    pub drm_format_modifier_tiling_features: FormatFeatureFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDrmFormatModifierPropertiesListEXT.html>
#[doc(alias = "VkDrmFormatModifierPropertiesListEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DrmFormatModifierPropertiesListEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub drm_format_modifier_count: u32,
    pub p_drm_format_modifier_properties: *mut DrmFormatModifierPropertiesEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageDrmFormatModifierInfoEXT.html>
#[doc(alias = "VkPhysicalDeviceImageDrmFormatModifierInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageDrmFormatModifierInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub drm_format_modifier: u64,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageDrmFormatModifierListCreateInfoEXT.html>
#[doc(alias = "VkImageDrmFormatModifierListCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageDrmFormatModifierListCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub drm_format_modifier_count: u32,
    pub p_drm_format_modifiers: *const u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageDrmFormatModifierExplicitCreateInfoEXT.html>
#[doc(alias = "VkImageDrmFormatModifierExplicitCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageDrmFormatModifierExplicitCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub drm_format_modifier: u64,
    pub drm_format_modifier_plane_count: u32,
    pub p_plane_layouts: *const SubresourceLayout,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageDrmFormatModifierPropertiesEXT.html>
#[doc(alias = "VkImageDrmFormatModifierPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageDrmFormatModifierPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub drm_format_modifier: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDrmFormatModifierProperties2EXT.html>
#[doc(alias = "VkDrmFormatModifierProperties2EXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DrmFormatModifierProperties2EXT {
    pub drm_format_modifier: u64,
    pub drm_format_modifier_plane_count: u32,
    pub drm_format_modifier_tiling_features: FormatFeatureFlags2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDrmFormatModifierPropertiesList2EXT.html>
#[doc(alias = "VkDrmFormatModifierPropertiesList2EXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DrmFormatModifierPropertiesList2EXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub drm_format_modifier_count: u32,
    pub p_drm_format_modifier_properties: *mut DrmFormatModifierProperties2EXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkValidationCacheCreateInfoEXT.html>
#[doc(alias = "VkValidationCacheCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ValidationCacheCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ValidationCacheCreateFlagsEXT,
    pub initial_data_size: usize,
    pub p_initial_data: *const c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderModuleValidationCacheCreateInfoEXT.html>
#[doc(alias = "VkShaderModuleValidationCacheCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ShaderModuleValidationCacheCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub validation_cache: ValidationCacheEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShadingRatePaletteNV.html>
#[doc(alias = "VkShadingRatePaletteNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ShadingRatePaletteNV {
    pub shading_rate_palette_entry_count: u32,
    pub p_shading_rate_palette_entries: *const ShadingRatePaletteEntryNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportShadingRateImageStateCreateInfoNV.html>
#[doc(alias = "VkPipelineViewportShadingRateImageStateCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineViewportShadingRateImageStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shading_rate_image_enable: Bool32,
    pub viewport_count: u32,
    pub p_shading_rate_palettes: *const ShadingRatePaletteNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShadingRateImageFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceShadingRateImageFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShadingRateImageFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shading_rate_image: Bool32,
    pub shading_rate_coarse_sample_order: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShadingRateImagePropertiesNV.html>
#[doc(alias = "VkPhysicalDeviceShadingRateImagePropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShadingRateImagePropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shading_rate_texel_size: Extent2D,
    pub shading_rate_palette_size: u32,
    pub shading_rate_max_coarse_samples: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCoarseSampleLocationNV.html>
#[doc(alias = "VkCoarseSampleLocationNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CoarseSampleLocationNV {
    pub pixel_x: u32,
    pub pixel_y: u32,
    pub sample: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCoarseSampleOrderCustomNV.html>
#[doc(alias = "VkCoarseSampleOrderCustomNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CoarseSampleOrderCustomNV {
    pub shading_rate: ShadingRatePaletteEntryNV,
    pub sample_count: u32,
    pub sample_location_count: u32,
    pub p_sample_locations: *const CoarseSampleLocationNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportCoarseSampleOrderStateCreateInfoNV.html>
#[doc(alias = "VkPipelineViewportCoarseSampleOrderStateCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sample_order_type: CoarseSampleOrderTypeNV,
    pub custom_sample_order_count: u32,
    pub p_custom_sample_orders: *const CoarseSampleOrderCustomNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRayTracingShaderGroupCreateInfoNV.html>
#[doc(alias = "VkRayTracingShaderGroupCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RayTracingShaderGroupCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub type_: RayTracingShaderGroupTypeKHR,
    pub general_shader: u32,
    pub closest_hit_shader: u32,
    pub any_hit_shader: u32,
    pub intersection_shader: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRayTracingPipelineCreateInfoNV.html>
#[doc(alias = "VkRayTracingPipelineCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RayTracingPipelineCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage_count: u32,
    pub p_stages: *const PipelineShaderStageCreateInfo,
    pub group_count: u32,
    pub p_groups: *const RayTracingShaderGroupCreateInfoNV,
    pub max_recursion_depth: u32,
    pub layout: PipelineLayout,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryTrianglesNV.html>
#[doc(alias = "VkGeometryTrianglesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GeometryTrianglesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vertex_data: Buffer,
    pub vertex_offset: DeviceSize,
    pub vertex_count: u32,
    pub vertex_stride: DeviceSize,
    pub vertex_format: Format,
    pub index_data: Buffer,
    pub index_offset: DeviceSize,
    pub index_count: u32,
    pub index_type: IndexType,
    pub transform_data: Buffer,
    pub transform_offset: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryAABBNV.html>
#[doc(alias = "VkGeometryAABBNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GeometryAABBNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub aabb_data: Buffer,
    pub num_aab_bs: u32,
    pub stride: u32,
    pub offset: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryDataNV.html>
#[doc(alias = "VkGeometryDataNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GeometryDataNV {
    pub triangles: GeometryTrianglesNV,
    pub aabbs: GeometryAABBNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryNV.html>
#[doc(alias = "VkGeometryNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GeometryNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub geometry_type: GeometryTypeKHR,
    pub geometry: GeometryDataNV,
    pub flags: GeometryFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureInfoNV.html>
#[doc(alias = "VkAccelerationStructureInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub type_: AccelerationStructureTypeNV,
    pub flags: BuildAccelerationStructureFlagsKHR,
    pub instance_count: u32,
    pub geometry_count: u32,
    pub p_geometries: *const GeometryNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureCreateInfoNV.html>
#[doc(alias = "VkAccelerationStructureCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub compacted_size: DeviceSize,
    pub info: AccelerationStructureInfoNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindAccelerationStructureMemoryInfoNV.html>
#[doc(alias = "VkBindAccelerationStructureMemoryInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BindAccelerationStructureMemoryInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acceleration_structure: AccelerationStructureNV,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub device_index_count: u32,
    pub p_device_indices: *const u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkWriteDescriptorSetAccelerationStructureNV.html>
#[doc(alias = "VkWriteDescriptorSetAccelerationStructureNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct WriteDescriptorSetAccelerationStructureNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acceleration_structure_count: u32,
    pub p_acceleration_structures: *const AccelerationStructureNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMemoryRequirementsInfoNV.html>
#[doc(alias = "VkAccelerationStructureMemoryRequirementsInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureMemoryRequirementsInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub type_: AccelerationStructureMemoryRequirementsTypeNV,
    pub acceleration_structure: AccelerationStructureNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRayTracingPropertiesNV.html>
#[doc(alias = "VkPhysicalDeviceRayTracingPropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_group_handle_size: u32,
    pub max_recursion_depth: u32,
    pub max_shader_group_stride: u32,
    pub shader_group_base_alignment: u32,
    pub max_geometry_count: u64,
    pub max_instance_count: u64,
    pub max_triangle_count: u64,
    pub max_descriptor_set_acceleration_structures: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTransformMatrixKHR.html>
#[doc(alias = "VkTransformMatrixKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TransformMatrixKHR {
    pub matrix: [[f32; 3]; 4],
}
pub type TransformMatrixNV = TransformMatrixKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAabbPositionsKHR.html>
#[doc(alias = "VkAabbPositionsKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AabbPositionsKHR {
    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,
}
pub type AabbPositionsNV = AabbPositionsKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureInstanceKHR.html>
#[doc(alias = "VkAccelerationStructureInstanceKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureInstanceKHR {
    pub transform: TransformMatrixKHR,
    pub instance_custom_index: u32,
    pub mask: u32,
    pub instance_shader_binding_table_record_offset: u32,
    pub flags: GeometryInstanceFlagsKHR,
    pub acceleration_structure_reference: u64,
}
pub type AccelerationStructureInstanceNV = AccelerationStructureInstanceKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub representative_fragment_test: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRepresentativeFragmentTestStateCreateInfoNV.html>
#[doc(alias = "VkPipelineRepresentativeFragmentTestStateCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineRepresentativeFragmentTestStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub representative_fragment_test_enable: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageViewImageFormatInfoEXT.html>
#[doc(alias = "VkPhysicalDeviceImageViewImageFormatInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageViewImageFormatInfoEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_view_type: ImageViewType,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFilterCubicImageViewImageFormatPropertiesEXT.html>
#[doc(alias = "VkFilterCubicImageViewImageFormatPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FilterCubicImageViewImageFormatPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub filter_cubic: Bool32,
    pub filter_cubic_minmax: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCooperativeMatrixConversionFeaturesQCOM.html>
#[doc(alias = "VkPhysicalDeviceCooperativeMatrixConversionFeaturesQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixConversionFeaturesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_matrix_conversion: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceElapsedTimerQueryFeaturesQCOM.html>
#[doc(alias = "VkPhysicalDeviceElapsedTimerQueryFeaturesQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceElapsedTimerQueryFeaturesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub elapsed_timer_query: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImportMemoryHostPointerInfoEXT.html>
#[doc(alias = "VkImportMemoryHostPointerInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImportMemoryHostPointerInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlags,
    pub p_host_pointer: *mut c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryHostPointerPropertiesEXT.html>
#[doc(alias = "VkMemoryHostPointerPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryHostPointerPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_type_bits: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalMemoryHostPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceExternalMemoryHostPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExternalMemoryHostPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_imported_host_pointer_alignment: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCompilerControlCreateInfoAMD.html>
#[doc(alias = "VkPipelineCompilerControlCreateInfoAMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineCompilerControlCreateInfoAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub compiler_control_flags: PipelineCompilerControlFlagsAMD,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderCorePropertiesAMD.html>
#[doc(alias = "VkPhysicalDeviceShaderCorePropertiesAMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderCorePropertiesAMD {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_engine_count: u32,
    pub shader_arrays_per_engine_count: u32,
    pub compute_units_per_shader_array: u32,
    pub simd_per_compute_unit: u32,
    pub wavefronts_per_simd: u32,
    pub wavefront_size: u32,
    pub sgprs_per_simd: u32,
    pub min_sgpr_allocation: u32,
    pub max_sgpr_allocation: u32,
    pub sgpr_allocation_granularity: u32,
    pub vgprs_per_simd: u32,
    pub min_vgpr_allocation: u32,
    pub max_vgpr_allocation: u32,
    pub vgpr_allocation_granularity: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemoryOverallocationCreateInfoAMD.html>
#[doc(alias = "VkDeviceMemoryOverallocationCreateInfoAMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceMemoryOverallocationCreateInfoAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub overallocation_behavior: MemoryOverallocationBehaviorAMD,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_vertex_attrib_divisor: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentFrameTokenGGP.html>
#[doc(alias = "VkPresentFrameTokenGGP")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PresentFrameTokenGGP {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub frame_token: GgpFrameToken,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMeshShaderFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceMeshShaderFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMeshShaderFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub task_shader: Bool32,
    pub mesh_shader: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMeshShaderPropertiesNV.html>
#[doc(alias = "VkPhysicalDeviceMeshShaderPropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMeshShaderPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_draw_mesh_tasks_count: u32,
    pub max_task_work_group_invocations: u32,
    pub max_task_work_group_size: [u32; 3],
    pub max_task_total_memory_size: u32,
    pub max_task_output_count: u32,
    pub max_mesh_work_group_invocations: u32,
    pub max_mesh_work_group_size: [u32; 3],
    pub max_mesh_total_memory_size: u32,
    pub max_mesh_output_vertices: u32,
    pub max_mesh_output_primitives: u32,
    pub max_mesh_multiview_view_count: u32,
    pub mesh_output_per_vertex_granularity: u32,
    pub mesh_output_per_primitive_granularity: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDrawMeshTasksIndirectCommandNV.html>
#[doc(alias = "VkDrawMeshTasksIndirectCommandNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DrawMeshTasksIndirectCommandNV {
    pub task_count: u32,
    pub first_task: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderImageFootprintFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceShaderImageFootprintFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderImageFootprintFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_footprint: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportExclusiveScissorStateCreateInfoNV.html>
#[doc(alias = "VkPipelineViewportExclusiveScissorStateCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineViewportExclusiveScissorStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub exclusive_scissor_count: u32,
    pub p_exclusive_scissors: *const Rect2D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExclusiveScissorFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceExclusiveScissorFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExclusiveScissorFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub exclusive_scissor: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyCheckpointPropertiesNV.html>
#[doc(alias = "VkQueueFamilyCheckpointPropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QueueFamilyCheckpointPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub checkpoint_execution_stage_mask: PipelineStageFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCheckpointDataNV.html>
#[doc(alias = "VkCheckpointDataNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CheckpointDataNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub stage: PipelineStageFlags,
    pub p_checkpoint_marker: *mut c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyCheckpointProperties2NV.html>
#[doc(alias = "VkQueueFamilyCheckpointProperties2NV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QueueFamilyCheckpointProperties2NV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub checkpoint_execution_stage_mask: PipelineStageFlags2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCheckpointData2NV.html>
#[doc(alias = "VkCheckpointData2NV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CheckpointData2NV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub stage: PipelineStageFlags2,
    pub p_checkpoint_marker: *mut c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePresentTimingFeaturesEXT.html>
#[doc(alias = "VkPhysicalDevicePresentTimingFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePresentTimingFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_timing: Bool32,
    pub present_at_absolute_time: Bool32,
    pub present_at_relative_time: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentTimingSurfaceCapabilitiesEXT.html>
#[doc(alias = "VkPresentTimingSurfaceCapabilitiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PresentTimingSurfaceCapabilitiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_timing_supported: Bool32,
    pub present_at_absolute_time_supported: Bool32,
    pub present_at_relative_time_supported: Bool32,
    pub present_stage_queries: PresentStageFlagsEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainCalibratedTimestampInfoEXT.html>
#[doc(alias = "VkSwapchainCalibratedTimestampInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SwapchainCalibratedTimestampInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain: SwapchainKHR,
    pub present_stage: PresentStageFlagsEXT,
    pub time_domain_id: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainTimingPropertiesEXT.html>
#[doc(alias = "VkSwapchainTimingPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SwapchainTimingPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub refresh_duration: u64,
    pub refresh_interval: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainTimeDomainPropertiesEXT.html>
#[doc(alias = "VkSwapchainTimeDomainPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SwapchainTimeDomainPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub time_domain_count: u32,
    pub p_time_domains: *mut TimeDomainKHR,
    pub p_time_domain_ids: *mut u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPastPresentationTimingInfoEXT.html>
#[doc(alias = "VkPastPresentationTimingInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PastPresentationTimingInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PastPresentationTimingFlagsEXT,
    pub swapchain: SwapchainKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentStageTimeEXT.html>
#[doc(alias = "VkPresentStageTimeEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PresentStageTimeEXT {
    pub stage: PresentStageFlagsEXT,
    pub time: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPastPresentationTimingEXT.html>
#[doc(alias = "VkPastPresentationTimingEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PastPresentationTimingEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_id: u64,
    pub target_time: u64,
    pub present_stage_count: u32,
    pub p_present_stages: *mut PresentStageTimeEXT,
    pub time_domain: TimeDomainKHR,
    pub time_domain_id: u64,
    pub report_complete: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPastPresentationTimingPropertiesEXT.html>
#[doc(alias = "VkPastPresentationTimingPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PastPresentationTimingPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub timing_properties_counter: u64,
    pub time_domains_counter: u64,
    pub presentation_timing_count: u32,
    pub p_presentation_timings: *mut PastPresentationTimingEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentTimingInfoEXT.html>
#[doc(alias = "VkPresentTimingInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PresentTimingInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PresentTimingInfoFlagsEXT,
    pub target_time: u64,
    pub time_domain_id: u64,
    pub present_stage_queries: PresentStageFlagsEXT,
    pub target_time_domain_present_stage: PresentStageFlagsEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentTimingsInfoEXT.html>
#[doc(alias = "VkPresentTimingsInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PresentTimingsInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_timing_infos: *const PresentTimingInfoEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL.html>
#[doc(alias = "VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_integer_functions_2: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceValueDataINTEL.html>
#[doc(alias = "VkPerformanceValueDataINTEL")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PerformanceValueDataINTEL {
    pub value_32: u32,
    pub value_64: u64,
    pub value_float: f32,
    pub value_bool: Bool32,
    pub value_string: *const c_char,
}
impl std::fmt::Debug for PerformanceValueDataINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PerformanceValueDataINTEL {{ .. }}")
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceValueINTEL.html>
#[doc(alias = "VkPerformanceValueINTEL")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PerformanceValueINTEL {
    pub type_: PerformanceValueTypeINTEL,
    pub data: PerformanceValueDataINTEL,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkInitializePerformanceApiInfoINTEL.html>
#[doc(alias = "VkInitializePerformanceApiInfoINTEL")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct InitializePerformanceApiInfoINTEL {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_user_data: *mut c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPoolPerformanceQueryCreateInfoINTEL.html>
#[doc(alias = "VkQueryPoolPerformanceQueryCreateInfoINTEL")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QueryPoolPerformanceQueryCreateInfoINTEL {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub performance_counters_sampling: QueryPoolSamplingModeINTEL,
}
pub type QueryPoolCreateInfoINTEL = QueryPoolPerformanceQueryCreateInfoINTEL;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceMarkerInfoINTEL.html>
#[doc(alias = "VkPerformanceMarkerInfoINTEL")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PerformanceMarkerInfoINTEL {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub marker: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceStreamMarkerInfoINTEL.html>
#[doc(alias = "VkPerformanceStreamMarkerInfoINTEL")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PerformanceStreamMarkerInfoINTEL {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub marker: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceOverrideInfoINTEL.html>
#[doc(alias = "VkPerformanceOverrideInfoINTEL")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PerformanceOverrideInfoINTEL {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub type_: PerformanceOverrideTypeINTEL,
    pub enable: Bool32,
    pub parameter: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceConfigurationAcquireInfoINTEL.html>
#[doc(alias = "VkPerformanceConfigurationAcquireInfoINTEL")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PerformanceConfigurationAcquireInfoINTEL {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub type_: PerformanceConfigurationTypeINTEL,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePCIBusInfoPropertiesEXT.html>
#[doc(alias = "VkPhysicalDevicePCIBusInfoPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePCIBusInfoPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pci_domain: u32,
    pub pci_bus: u32,
    pub pci_device: u32,
    pub pci_function: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayNativeHdrSurfaceCapabilitiesAMD.html>
#[doc(alias = "VkDisplayNativeHdrSurfaceCapabilitiesAMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DisplayNativeHdrSurfaceCapabilitiesAMD {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub local_dimming_support: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainDisplayNativeHdrCreateInfoAMD.html>
#[doc(alias = "VkSwapchainDisplayNativeHdrCreateInfoAMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SwapchainDisplayNativeHdrCreateInfoAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub local_dimming_enable: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImagePipeSurfaceCreateInfoFUCHSIA.html>
#[doc(alias = "VkImagePipeSurfaceCreateInfoFUCHSIA")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImagePipeSurfaceCreateInfoFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ImagePipeSurfaceCreateFlagsFUCHSIA,
    pub image_pipe_handle: zx_handle_t,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMetalSurfaceCreateInfoEXT.html>
#[doc(alias = "VkMetalSurfaceCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MetalSurfaceCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: MetalSurfaceCreateFlagsEXT,
    pub p_layer: *const CAMetalLayer,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentDensityMapFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceFragmentDensityMapFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub fragment_density_map: Bool32,
    pub fragment_density_map_dynamic: Bool32,
    pub fragment_density_map_non_subsampled_images: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentDensityMapPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceFragmentDensityMapPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_fragment_density_texel_size: Extent2D,
    pub max_fragment_density_texel_size: Extent2D,
    pub fragment_density_invocations: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassFragmentDensityMapCreateInfoEXT.html>
#[doc(alias = "VkRenderPassFragmentDensityMapCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderPassFragmentDensityMapCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fragment_density_map_attachment: AttachmentReference,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingFragmentDensityMapAttachmentInfoEXT.html>
#[doc(alias = "VkRenderingFragmentDensityMapAttachmentInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderingFragmentDensityMapAttachmentInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderCoreProperties2AMD.html>
#[doc(alias = "VkPhysicalDeviceShaderCoreProperties2AMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderCoreProperties2AMD {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_core_features: ShaderCorePropertiesFlagsAMD,
    pub active_compute_unit_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCoherentMemoryFeaturesAMD.html>
#[doc(alias = "VkPhysicalDeviceCoherentMemoryFeaturesAMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCoherentMemoryFeaturesAMD {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_coherent_memory: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_image_int_64_atomics: Bool32,
    pub sparse_image_int_64_atomics: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMemoryBudgetPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceMemoryBudgetPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMemoryBudgetPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub heap_budget: [DeviceSize; 16],
    pub heap_usage: [DeviceSize; 16],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMemoryPriorityFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceMemoryPriorityFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMemoryPriorityFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_priority: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryPriorityAllocateInfoEXT.html>
#[doc(alias = "VkMemoryPriorityAllocateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryPriorityAllocateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub priority: f32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub dedicated_allocation_image_aliasing: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceBufferDeviceAddressFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceBufferDeviceAddressFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub buffer_device_address: Bool32,
    pub buffer_device_address_capture_replay: Bool32,
    pub buffer_device_address_multi_device: Bool32,
}
pub type PhysicalDeviceBufferAddressFeaturesEXT = PhysicalDeviceBufferDeviceAddressFeaturesEXT;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferDeviceAddressCreateInfoEXT.html>
#[doc(alias = "VkBufferDeviceAddressCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BufferDeviceAddressCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_address: DeviceAddress,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkValidationFeaturesEXT.html>
#[doc(alias = "VkValidationFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ValidationFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub enabled_validation_feature_count: u32,
    pub p_enabled_validation_features: *const ValidationFeatureEnableEXT,
    pub disabled_validation_feature_count: u32,
    pub p_disabled_validation_features: *const ValidationFeatureDisableEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCooperativeMatrixPropertiesNV.html>
#[doc(alias = "VkCooperativeMatrixPropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CooperativeMatrixPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub m_size: u32,
    pub n_size: u32,
    pub k_size: u32,
    pub a_type: ComponentTypeNV,
    pub b_type: ComponentTypeNV,
    pub c_type: ComponentTypeNV,
    pub d_type: ComponentTypeNV,
    pub scope: ScopeNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCooperativeMatrixFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceCooperativeMatrixFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_matrix: Bool32,
    pub cooperative_matrix_robust_buffer_access: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCooperativeMatrixPropertiesNV.html>
#[doc(alias = "VkPhysicalDeviceCooperativeMatrixPropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_matrix_supported_stages: ShaderStageFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCoverageReductionModeFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceCoverageReductionModeFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCoverageReductionModeFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub coverage_reduction_mode: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCoverageReductionStateCreateInfoNV.html>
#[doc(alias = "VkPipelineCoverageReductionStateCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineCoverageReductionStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCoverageReductionStateCreateFlagsNV,
    pub coverage_reduction_mode: CoverageReductionModeNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebufferMixedSamplesCombinationNV.html>
#[doc(alias = "VkFramebufferMixedSamplesCombinationNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FramebufferMixedSamplesCombinationNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub coverage_reduction_mode: CoverageReductionModeNV,
    pub rasterization_samples: SampleCountFlags,
    pub depth_stencil_samples: SampleCountFlags,
    pub color_samples: SampleCountFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub fragment_shader_sample_interlock: Bool32,
    pub fragment_shader_pixel_interlock: Bool32,
    pub fragment_shader_shading_rate_interlock: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceYcbcrImageArraysFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceYcbcrImageArraysFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ycbcr_image_arrays: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceProvokingVertexFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceProvokingVertexFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceProvokingVertexFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub provoking_vertex_last: Bool32,
    pub transform_feedback_preserves_provoking_vertex: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceProvokingVertexPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceProvokingVertexPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceProvokingVertexPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub provoking_vertex_mode_per_pipeline: Bool32,
    pub transform_feedback_preserves_triangle_fan_provoking_vertex: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationProvokingVertexStateCreateInfoEXT.html>
#[doc(alias = "VkPipelineRasterizationProvokingVertexStateCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineRasterizationProvokingVertexStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub provoking_vertex_mode: ProvokingVertexModeEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceFullScreenExclusiveInfoEXT.html>
#[doc(alias = "VkSurfaceFullScreenExclusiveInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SurfaceFullScreenExclusiveInfoEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub full_screen_exclusive: FullScreenExclusiveEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCapabilitiesFullScreenExclusiveEXT.html>
#[doc(alias = "VkSurfaceCapabilitiesFullScreenExclusiveEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SurfaceCapabilitiesFullScreenExclusiveEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub full_screen_exclusive_supported: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceFullScreenExclusiveWin32InfoEXT.html>
#[doc(alias = "VkSurfaceFullScreenExclusiveWin32InfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SurfaceFullScreenExclusiveWin32InfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub hmonitor: HMONITOR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkHeadlessSurfaceCreateInfoEXT.html>
#[doc(alias = "VkHeadlessSurfaceCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct HeadlessSurfaceCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: HeadlessSurfaceCreateFlagsEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderAtomicFloatFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceShaderAtomicFloatFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderAtomicFloatFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_buffer_float_32_atomics: Bool32,
    pub shader_buffer_float_32_atomic_add: Bool32,
    pub shader_buffer_float_64_atomics: Bool32,
    pub shader_buffer_float_64_atomic_add: Bool32,
    pub shader_shared_float_32_atomics: Bool32,
    pub shader_shared_float_32_atomic_add: Bool32,
    pub shader_shared_float_64_atomics: Bool32,
    pub shader_shared_float_64_atomic_add: Bool32,
    pub shader_image_float_32_atomics: Bool32,
    pub shader_image_float_32_atomic_add: Bool32,
    pub sparse_image_float_32_atomics: Bool32,
    pub sparse_image_float_32_atomic_add: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicStateFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceExtendedDynamicStateFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub extended_dynamic_state: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMapMemoryPlacedFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceMapMemoryPlacedFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMapMemoryPlacedFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_map_placed: Bool32,
    pub memory_map_range_placed: Bool32,
    pub memory_unmap_reserve: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMapMemoryPlacedPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceMapMemoryPlacedPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMapMemoryPlacedPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_placed_memory_map_alignment: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryMapPlacedInfoEXT.html>
#[doc(alias = "VkMemoryMapPlacedInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryMapPlacedInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_placed_address: *mut c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderAtomicFloat2FeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_buffer_float_16_atomics: Bool32,
    pub shader_buffer_float_16_atomic_add: Bool32,
    pub shader_buffer_float_16_atomic_min_max: Bool32,
    pub shader_buffer_float_32_atomic_min_max: Bool32,
    pub shader_buffer_float_64_atomic_min_max: Bool32,
    pub shader_shared_float_16_atomics: Bool32,
    pub shader_shared_float_16_atomic_add: Bool32,
    pub shader_shared_float_16_atomic_min_max: Bool32,
    pub shader_shared_float_32_atomic_min_max: Bool32,
    pub shader_shared_float_64_atomic_min_max: Bool32,
    pub shader_image_float_32_atomic_min_max: Bool32,
    pub sparse_image_float_32_atomic_min_max: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV.html>
#[doc(alias = "VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_graphics_shader_group_count: u32,
    pub max_indirect_sequence_count: u32,
    pub max_indirect_commands_token_count: u32,
    pub max_indirect_commands_stream_count: u32,
    pub max_indirect_commands_token_offset: u32,
    pub max_indirect_commands_stream_stride: u32,
    pub min_sequences_count_buffer_offset_alignment: u32,
    pub min_sequences_index_buffer_offset_alignment: u32,
    pub min_indirect_commands_buffer_offset_alignment: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_generated_commands: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGraphicsShaderGroupCreateInfoNV.html>
#[doc(alias = "VkGraphicsShaderGroupCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GraphicsShaderGroupCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stage_count: u32,
    pub p_stages: *const PipelineShaderStageCreateInfo,
    pub p_vertex_input_state: *const PipelineVertexInputStateCreateInfo,
    pub p_tessellation_state: *const PipelineTessellationStateCreateInfo,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGraphicsPipelineShaderGroupsCreateInfoNV.html>
#[doc(alias = "VkGraphicsPipelineShaderGroupsCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GraphicsPipelineShaderGroupsCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub group_count: u32,
    pub p_groups: *const GraphicsShaderGroupCreateInfoNV,
    pub pipeline_count: u32,
    pub p_pipelines: *const Pipeline,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindShaderGroupIndirectCommandNV.html>
#[doc(alias = "VkBindShaderGroupIndirectCommandNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BindShaderGroupIndirectCommandNV {
    pub group_index: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindIndexBufferIndirectCommandNV.html>
#[doc(alias = "VkBindIndexBufferIndirectCommandNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BindIndexBufferIndirectCommandNV {
    pub buffer_address: DeviceAddress,
    pub size: u32,
    pub index_type: IndexType,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindVertexBufferIndirectCommandNV.html>
#[doc(alias = "VkBindVertexBufferIndirectCommandNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BindVertexBufferIndirectCommandNV {
    pub buffer_address: DeviceAddress,
    pub size: u32,
    pub stride: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSetStateFlagsIndirectCommandNV.html>
#[doc(alias = "VkSetStateFlagsIndirectCommandNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SetStateFlagsIndirectCommandNV {
    pub data: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsStreamNV.html>
#[doc(alias = "VkIndirectCommandsStreamNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct IndirectCommandsStreamNV {
    pub buffer: Buffer,
    pub offset: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutTokenNV.html>
#[doc(alias = "VkIndirectCommandsLayoutTokenNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct IndirectCommandsLayoutTokenNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub token_type: IndirectCommandsTokenTypeNV,
    pub stream: u32,
    pub offset: u32,
    pub vertex_binding_unit: u32,
    pub vertex_dynamic_stride: Bool32,
    pub pushconstant_pipeline_layout: PipelineLayout,
    pub pushconstant_shader_stage_flags: ShaderStageFlags,
    pub pushconstant_offset: u32,
    pub pushconstant_size: u32,
    pub indirect_state_flags: IndirectStateFlagsNV,
    pub index_type_count: u32,
    pub p_index_types: *const IndexType,
    pub p_index_type_values: *const u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutCreateInfoNV.html>
#[doc(alias = "VkIndirectCommandsLayoutCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct IndirectCommandsLayoutCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: IndirectCommandsLayoutUsageFlagsNV,
    pub pipeline_bind_point: PipelineBindPoint,
    pub token_count: u32,
    pub p_tokens: *const IndirectCommandsLayoutTokenNV,
    pub stream_count: u32,
    pub p_stream_strides: *const u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGeneratedCommandsInfoNV.html>
#[doc(alias = "VkGeneratedCommandsInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GeneratedCommandsInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline_bind_point: PipelineBindPoint,
    pub pipeline: Pipeline,
    pub indirect_commands_layout: IndirectCommandsLayoutNV,
    pub stream_count: u32,
    pub p_streams: *const IndirectCommandsStreamNV,
    pub sequences_count: u32,
    pub preprocess_buffer: Buffer,
    pub preprocess_offset: DeviceSize,
    pub preprocess_size: DeviceSize,
    pub sequences_count_buffer: Buffer,
    pub sequences_count_offset: DeviceSize,
    pub sequences_index_buffer: Buffer,
    pub sequences_index_offset: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGeneratedCommandsMemoryRequirementsInfoNV.html>
#[doc(alias = "VkGeneratedCommandsMemoryRequirementsInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GeneratedCommandsMemoryRequirementsInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline_bind_point: PipelineBindPoint,
    pub pipeline: Pipeline,
    pub indirect_commands_layout: IndirectCommandsLayoutNV,
    pub max_sequences_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceInheritedViewportScissorFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceInheritedViewportScissorFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceInheritedViewportScissorFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub inherited_viewport_scissor_2_d: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferInheritanceViewportScissorInfoNV.html>
#[doc(alias = "VkCommandBufferInheritanceViewportScissorInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CommandBufferInheritanceViewportScissorInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub viewport_scissor_2_d: Bool32,
    pub viewport_depth_count: u32,
    pub p_viewport_depths: *const Viewport,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub texel_buffer_alignment: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassTransformBeginInfoQCOM.html>
#[doc(alias = "VkRenderPassTransformBeginInfoQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderPassTransformBeginInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub transform: SurfaceTransformFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferInheritanceRenderPassTransformInfoQCOM.html>
#[doc(alias = "VkCommandBufferInheritanceRenderPassTransformInfoQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CommandBufferInheritanceRenderPassTransformInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub transform: SurfaceTransformFlagsKHR,
    pub render_area: Rect2D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDepthBiasControlFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceDepthBiasControlFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDepthBiasControlFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub depth_bias_control: Bool32,
    pub least_representable_value_force_unorm_representation: Bool32,
    pub float_representation: Bool32,
    pub depth_bias_exact: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDepthBiasInfoEXT.html>
#[doc(alias = "VkDepthBiasInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DepthBiasInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub depth_bias_constant_factor: f32,
    pub depth_bias_clamp: f32,
    pub depth_bias_slope_factor: f32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDepthBiasRepresentationInfoEXT.html>
#[doc(alias = "VkDepthBiasRepresentationInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DepthBiasRepresentationInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub depth_bias_representation: DepthBiasRepresentationEXT,
    pub depth_bias_exact: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDeviceMemoryReportFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceDeviceMemoryReportFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDeviceMemoryReportFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_memory_report: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemoryReportCallbackDataEXT.html>
#[doc(alias = "VkDeviceMemoryReportCallbackDataEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceMemoryReportCallbackDataEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: DeviceMemoryReportFlagsEXT,
    pub type_: DeviceMemoryReportEventTypeEXT,
    pub memory_object_id: u64,
    pub size: DeviceSize,
    pub object_type: ObjectType,
    pub object_handle: u64,
    pub heap_index: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceDeviceMemoryReportCreateInfoEXT.html>
#[doc(alias = "VkDeviceDeviceMemoryReportCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceDeviceMemoryReportCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceMemoryReportFlagsEXT,
    pub pfn_user_callback: vkDeviceMemoryReportCallbackEXT,
    pub p_user_data: *mut c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCustomBorderColorPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceCustomBorderColorPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCustomBorderColorPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_custom_border_color_samplers: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCustomBorderColorFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceCustomBorderColorFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCustomBorderColorFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub custom_border_colors: Bool32,
    pub custom_border_color_without_format: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTextureCompressionASTC3DFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceTextureCompressionASTC3DFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTextureCompressionASTC3DFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub texture_compression_astc_3_d: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePresentBarrierFeaturesNV.html>
#[doc(alias = "VkPhysicalDevicePresentBarrierFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePresentBarrierFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_barrier: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCapabilitiesPresentBarrierNV.html>
#[doc(alias = "VkSurfaceCapabilitiesPresentBarrierNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SurfaceCapabilitiesPresentBarrierNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_barrier_supported: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainPresentBarrierCreateInfoNV.html>
#[doc(alias = "VkSwapchainPresentBarrierCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SwapchainPresentBarrierCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_barrier_enable: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDiagnosticsConfigFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceDiagnosticsConfigFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDiagnosticsConfigFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub diagnostics_config: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceDiagnosticsConfigCreateInfoNV.html>
#[doc(alias = "VkDeviceDiagnosticsConfigCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceDiagnosticsConfigCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceDiagnosticsConfigFlagsNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerfHintInfoQCOM.html>
#[doc(alias = "VkPerfHintInfoQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PerfHintInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub type_: PerfHintTypeQCOM,
    pub scale: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceQueuePerfHintFeaturesQCOM.html>
#[doc(alias = "VkPhysicalDeviceQueuePerfHintFeaturesQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceQueuePerfHintFeaturesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub queue_perf_hint: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceQueuePerfHintPropertiesQCOM.html>
#[doc(alias = "VkPhysicalDeviceQueuePerfHintPropertiesQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceQueuePerfHintPropertiesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supported_queues: QueueFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageProcessing3FeaturesQCOM.html>
#[doc(alias = "VkPhysicalDeviceImageProcessing3FeaturesQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageProcessing3FeaturesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_gather_linear: Bool32,
    pub image_gather_extended_modes: Bool32,
    pub block_match_extended_clamp_to_edge: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM.html>
#[doc(alias = "VkPhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_multiple_wait_queues: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM.html>
#[doc(alias = "VkPhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_shader_wait_queues: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderSplitBarrierFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceShaderSplitBarrierFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderSplitBarrierFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_split_barrier: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderSplitBarrierPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceShaderSplitBarrierPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderSplitBarrierPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub split_barrier_reserved_shared_memory: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCudaModuleCreateInfoNV.html>
#[doc(alias = "VkCudaModuleCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CudaModuleCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub data_size: usize,
    pub p_data: *const c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCudaFunctionCreateInfoNV.html>
#[doc(alias = "VkCudaFunctionCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CudaFunctionCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub module: CudaModuleNV,
    pub p_name: *const c_char,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCudaLaunchInfoNV.html>
#[doc(alias = "VkCudaLaunchInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CudaLaunchInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub function: CudaFunctionNV,
    pub grid_dim_x: u32,
    pub grid_dim_y: u32,
    pub grid_dim_z: u32,
    pub block_dim_x: u32,
    pub block_dim_y: u32,
    pub block_dim_z: u32,
    pub shared_mem_bytes: u32,
    pub param_count: usize,
    pub p_params: *const *const c_void,
    pub extra_count: usize,
    pub p_extras: *const *const c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCudaKernelLaunchFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceCudaKernelLaunchFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCudaKernelLaunchFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cuda_kernel_launch_features: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCudaKernelLaunchPropertiesNV.html>
#[doc(alias = "VkPhysicalDeviceCudaKernelLaunchPropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCudaKernelLaunchPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub compute_capability_minor: u32,
    pub compute_capability_major: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTileShadingFeaturesQCOM.html>
#[doc(alias = "VkPhysicalDeviceTileShadingFeaturesQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTileShadingFeaturesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tile_shading: Bool32,
    pub tile_shading_fragment_stage: Bool32,
    pub tile_shading_color_attachments: Bool32,
    pub tile_shading_depth_attachments: Bool32,
    pub tile_shading_stencil_attachments: Bool32,
    pub tile_shading_input_attachments: Bool32,
    pub tile_shading_sampled_attachments: Bool32,
    pub tile_shading_per_tile_draw: Bool32,
    pub tile_shading_per_tile_dispatch: Bool32,
    pub tile_shading_dispatch_tile: Bool32,
    pub tile_shading_apron: Bool32,
    pub tile_shading_anisotropic_apron: Bool32,
    pub tile_shading_atomic_ops: Bool32,
    pub tile_shading_image_processing: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTileShadingPropertiesQCOM.html>
#[doc(alias = "VkPhysicalDeviceTileShadingPropertiesQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTileShadingPropertiesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_apron_size: u32,
    pub prefer_non_coherent: Bool32,
    pub tile_granularity: Extent2D,
    pub max_tile_shading_rate: Extent2D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassTileShadingCreateInfoQCOM.html>
#[doc(alias = "VkRenderPassTileShadingCreateInfoQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderPassTileShadingCreateInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: TileShadingRenderPassFlagsQCOM,
    pub tile_apron_size: Extent2D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerTileBeginInfoQCOM.html>
#[doc(alias = "VkPerTileBeginInfoQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PerTileBeginInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerTileEndInfoQCOM.html>
#[doc(alias = "VkPerTileEndInfoQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PerTileEndInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDispatchTileInfoQCOM.html>
#[doc(alias = "VkDispatchTileInfoQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DispatchTileInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryLowLatencySupportNV.html>
#[doc(alias = "VkQueryLowLatencySupportNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QueryLowLatencySupportNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_queried_low_latency_data: *mut c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExportMetalObjectCreateInfoEXT.html>
#[doc(alias = "VkExportMetalObjectCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExportMetalObjectCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub export_object_type: ExportMetalObjectTypeFlagsEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExportMetalObjectsInfoEXT.html>
#[doc(alias = "VkExportMetalObjectsInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExportMetalObjectsInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExportMetalDeviceInfoEXT.html>
#[doc(alias = "VkExportMetalDeviceInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExportMetalDeviceInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mtl_device: MTLDevice_id,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExportMetalCommandQueueInfoEXT.html>
#[doc(alias = "VkExportMetalCommandQueueInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExportMetalCommandQueueInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub queue: Queue,
    pub mtl_command_queue: MTLCommandQueue_id,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExportMetalBufferInfoEXT.html>
#[doc(alias = "VkExportMetalBufferInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExportMetalBufferInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub mtl_buffer: MTLBuffer_id,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImportMetalBufferInfoEXT.html>
#[doc(alias = "VkImportMetalBufferInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImportMetalBufferInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mtl_buffer: MTLBuffer_id,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExportMetalTextureInfoEXT.html>
#[doc(alias = "VkExportMetalTextureInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExportMetalTextureInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub image_view: ImageView,
    pub buffer_view: BufferView,
    pub plane: ImageAspectFlags,
    pub mtl_texture: MTLTexture_id,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImportMetalTextureInfoEXT.html>
#[doc(alias = "VkImportMetalTextureInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImportMetalTextureInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub plane: ImageAspectFlags,
    pub mtl_texture: MTLTexture_id,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExportMetalIOSurfaceInfoEXT.html>
#[doc(alias = "VkExportMetalIOSurfaceInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExportMetalIOSurfaceInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub io_surface: IOSurfaceRef,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImportMetalIOSurfaceInfoEXT.html>
#[doc(alias = "VkImportMetalIOSurfaceInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImportMetalIOSurfaceInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub io_surface: IOSurfaceRef,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExportMetalSharedEventInfoEXT.html>
#[doc(alias = "VkExportMetalSharedEventInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExportMetalSharedEventInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub event: Event,
    pub mtl_shared_event: MTLSharedEvent_id,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImportMetalSharedEventInfoEXT.html>
#[doc(alias = "VkImportMetalSharedEventInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImportMetalSharedEventInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mtl_shared_event: MTLSharedEvent_id,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDescriptorBufferPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceDescriptorBufferPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDescriptorBufferPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub combined_image_sampler_descriptor_single_array: Bool32,
    pub bufferless_push_descriptors: Bool32,
    pub allow_sampler_image_view_post_submit_creation: Bool32,
    pub descriptor_buffer_offset_alignment: DeviceSize,
    pub max_descriptor_buffer_bindings: u32,
    pub max_resource_descriptor_buffer_bindings: u32,
    pub max_sampler_descriptor_buffer_bindings: u32,
    pub max_embedded_immutable_sampler_bindings: u32,
    pub max_embedded_immutable_samplers: u32,
    pub buffer_capture_replay_descriptor_data_size: usize,
    pub image_capture_replay_descriptor_data_size: usize,
    pub image_view_capture_replay_descriptor_data_size: usize,
    pub sampler_capture_replay_descriptor_data_size: usize,
    pub acceleration_structure_capture_replay_descriptor_data_size: usize,
    pub sampler_descriptor_size: usize,
    pub combined_image_sampler_descriptor_size: usize,
    pub sampled_image_descriptor_size: usize,
    pub storage_image_descriptor_size: usize,
    pub uniform_texel_buffer_descriptor_size: usize,
    pub robust_uniform_texel_buffer_descriptor_size: usize,
    pub storage_texel_buffer_descriptor_size: usize,
    pub robust_storage_texel_buffer_descriptor_size: usize,
    pub uniform_buffer_descriptor_size: usize,
    pub robust_uniform_buffer_descriptor_size: usize,
    pub storage_buffer_descriptor_size: usize,
    pub robust_storage_buffer_descriptor_size: usize,
    pub input_attachment_descriptor_size: usize,
    pub acceleration_structure_descriptor_size: usize,
    pub max_sampler_descriptor_buffer_range: DeviceSize,
    pub max_resource_descriptor_buffer_range: DeviceSize,
    pub sampler_descriptor_buffer_address_space_size: DeviceSize,
    pub resource_descriptor_buffer_address_space_size: DeviceSize,
    pub descriptor_buffer_address_space_size: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDescriptorBufferFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceDescriptorBufferFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDescriptorBufferFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub descriptor_buffer: Bool32,
    pub descriptor_buffer_capture_replay: Bool32,
    pub descriptor_buffer_image_layout_ignored: Bool32,
    pub descriptor_buffer_push_descriptors: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorAddressInfoEXT.html>
#[doc(alias = "VkDescriptorAddressInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorAddressInfoEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub address: DeviceAddress,
    pub range: DeviceSize,
    pub format: Format,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorBufferBindingInfoEXT.html>
#[doc(alias = "VkDescriptorBufferBindingInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorBufferBindingInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub address: DeviceAddress,
    pub usage: BufferUsageFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorBufferBindingPushDescriptorBufferHandleEXT.html>
#[doc(alias = "VkDescriptorBufferBindingPushDescriptorBufferHandleEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorBufferBindingPushDescriptorBufferHandleEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorDataEXT.html>
#[doc(alias = "VkDescriptorDataEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DescriptorDataEXT {
    pub p_sampler: *const Sampler,
    pub p_combined_image_sampler: *const DescriptorImageInfo,
    pub p_input_attachment_image: *const DescriptorImageInfo,
    pub p_sampled_image: *const DescriptorImageInfo,
    pub p_storage_image: *const DescriptorImageInfo,
    pub p_uniform_texel_buffer: *const DescriptorAddressInfoEXT,
    pub p_storage_texel_buffer: *const DescriptorAddressInfoEXT,
    pub p_uniform_buffer: *const DescriptorAddressInfoEXT,
    pub p_storage_buffer: *const DescriptorAddressInfoEXT,
    pub acceleration_structure: DeviceAddress,
}
impl std::fmt::Debug for DescriptorDataEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DescriptorDataEXT {{ .. }}")
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorGetInfoEXT.html>
#[doc(alias = "VkDescriptorGetInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorGetInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub type_: DescriptorType,
    pub data: DescriptorDataEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCaptureDescriptorDataInfoEXT.html>
#[doc(alias = "VkBufferCaptureDescriptorDataInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BufferCaptureDescriptorDataInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCaptureDescriptorDataInfoEXT.html>
#[doc(alias = "VkImageCaptureDescriptorDataInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageCaptureDescriptorDataInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewCaptureDescriptorDataInfoEXT.html>
#[doc(alias = "VkImageViewCaptureDescriptorDataInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageViewCaptureDescriptorDataInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_view: ImageView,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerCaptureDescriptorDataInfoEXT.html>
#[doc(alias = "VkSamplerCaptureDescriptorDataInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SamplerCaptureDescriptorDataInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sampler: Sampler,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpaqueCaptureDescriptorDataCreateInfoEXT.html>
#[doc(alias = "VkOpaqueCaptureDescriptorDataCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct OpaqueCaptureDescriptorDataCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub opaque_capture_descriptor_data: *const c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureCaptureDescriptorDataInfoEXT.html>
#[doc(alias = "VkAccelerationStructureCaptureDescriptorDataInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureCaptureDescriptorDataInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acceleration_structure: AccelerationStructureKHR,
    pub acceleration_structure_nv: AccelerationStructureNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDescriptorBufferDensityMapPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceDescriptorBufferDensityMapPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDescriptorBufferDensityMapPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub combined_image_sampler_density_map_descriptor_size: usize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub graphics_pipeline_library: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub graphics_pipeline_library_fast_linking: Bool32,
    pub graphics_pipeline_library_independent_interpolation_decoration: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGraphicsPipelineLibraryCreateInfoEXT.html>
#[doc(alias = "VkGraphicsPipelineLibraryCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GraphicsPipelineLibraryCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: GraphicsPipelineLibraryFlagsEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD.html>
#[doc(alias = "VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_early_and_late_fragment_tests: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub fragment_shading_rate_enums: Bool32,
    pub supersample_fragment_shading_rates: Bool32,
    pub no_invocation_fragment_shading_rates: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV.html>
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_fragment_shading_rate_invocation_count: SampleCountFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineFragmentShadingRateEnumStateCreateInfoNV.html>
#[doc(alias = "VkPipelineFragmentShadingRateEnumStateCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineFragmentShadingRateEnumStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shading_rate_type: FragmentShadingRateTypeNV,
    pub shading_rate: FragmentShadingRateNV,
    pub combiner_ops: [FragmentShadingRateCombinerOpKHR; 2],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceOrHostAddressConstKHR.html>
#[doc(alias = "VkDeviceOrHostAddressConstKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceOrHostAddressConstKHR {
    pub device_address: DeviceAddress,
    pub host_address: *const c_void,
}
impl std::fmt::Debug for DeviceOrHostAddressConstKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeviceOrHostAddressConstKHR {{ .. }}")
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureGeometryMotionTrianglesDataNV.html>
#[doc(alias = "VkAccelerationStructureGeometryMotionTrianglesDataNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureGeometryMotionTrianglesDataNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vertex_data: DeviceOrHostAddressConstKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMotionInfoNV.html>
#[doc(alias = "VkAccelerationStructureMotionInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureMotionInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_instances: u32,
    pub flags: AccelerationStructureMotionInfoFlagsNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMatrixMotionInstanceNV.html>
#[doc(alias = "VkAccelerationStructureMatrixMotionInstanceNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureMatrixMotionInstanceNV {
    pub transform_t_0: TransformMatrixKHR,
    pub transform_t_1: TransformMatrixKHR,
    pub instance_custom_index: u32,
    pub mask: u32,
    pub instance_shader_binding_table_record_offset: u32,
    pub flags: GeometryInstanceFlagsKHR,
    pub acceleration_structure_reference: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSRTDataNV.html>
#[doc(alias = "VkSRTDataNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SRTDataNV {
    pub sx: f32,
    pub a: f32,
    pub b: f32,
    pub pvx: f32,
    pub sy: f32,
    pub c: f32,
    pub pvy: f32,
    pub sz: f32,
    pub pvz: f32,
    pub qx: f32,
    pub qy: f32,
    pub qz: f32,
    pub qw: f32,
    pub tx: f32,
    pub ty: f32,
    pub tz: f32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureSRTMotionInstanceNV.html>
#[doc(alias = "VkAccelerationStructureSRTMotionInstanceNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureSRTMotionInstanceNV {
    pub transform_t_0: SRTDataNV,
    pub transform_t_1: SRTDataNV,
    pub instance_custom_index: u32,
    pub mask: u32,
    pub instance_shader_binding_table_record_offset: u32,
    pub flags: GeometryInstanceFlagsKHR,
    pub acceleration_structure_reference: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMotionInstanceDataNV.html>
#[doc(alias = "VkAccelerationStructureMotionInstanceDataNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureMotionInstanceDataNV {
    pub static_instance: AccelerationStructureInstanceKHR,
    pub matrix_motion_instance: AccelerationStructureMatrixMotionInstanceNV,
    pub srt_motion_instance: AccelerationStructureSRTMotionInstanceNV,
}
impl std::fmt::Debug for AccelerationStructureMotionInstanceDataNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AccelerationStructureMotionInstanceDataNV {{ .. }}")
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMotionInstanceNV.html>
#[doc(alias = "VkAccelerationStructureMotionInstanceNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureMotionInstanceNV {
    pub type_: AccelerationStructureMotionInstanceTypeNV,
    pub flags: AccelerationStructureMotionInstanceFlagsNV,
    pub data: AccelerationStructureMotionInstanceDataNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRayTracingMotionBlurFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceRayTracingMotionBlurFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingMotionBlurFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_tracing_motion_blur: Bool32,
    pub ray_tracing_motion_blur_pipeline_trace_rays_indirect: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ycbcr_2_plane_444_formats: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentDensityMap2FeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceFragmentDensityMap2FeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMap2FeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub fragment_density_map_deferred: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentDensityMap2PropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceFragmentDensityMap2PropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMap2PropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub subsampled_loads: Bool32,
    pub subsampled_coarse_reconstruction_early_access: Bool32,
    pub max_subsampled_array_layers: u32,
    pub max_descriptor_set_subsampled_samplers: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyCommandTransformInfoQCOM.html>
#[doc(alias = "VkCopyCommandTransformInfoQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CopyCommandTransformInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub transform: SurfaceTransformFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageCompressionControlFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceImageCompressionControlFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageCompressionControlFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_compression_control: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCompressionControlEXT.html>
#[doc(alias = "VkImageCompressionControlEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageCompressionControlEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ImageCompressionFlagsEXT,
    pub compression_control_plane_count: u32,
    pub p_fixed_rate_flags: *mut ImageCompressionFixedRateFlagsEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCompressionPropertiesEXT.html>
#[doc(alias = "VkImageCompressionPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageCompressionPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_compression_flags: ImageCompressionFlagsEXT,
    pub image_compression_fixed_rate_flags: ImageCompressionFixedRateFlagsEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub attachment_feedback_loop_layout: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevice4444FormatsFeaturesEXT.html>
#[doc(alias = "VkPhysicalDevice4444FormatsFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevice4444FormatsFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format_a_4_r_4_g_4_b_4: Bool32,
    pub format_a_4_b_4_g_4_r_4: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFaultFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceFaultFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFaultFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_fault: Bool32,
    pub device_fault_vendor_binary: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceFaultCountsEXT.html>
#[doc(alias = "VkDeviceFaultCountsEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceFaultCountsEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub address_info_count: u32,
    pub vendor_info_count: u32,
    pub vendor_binary_size: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceFaultInfoEXT.html>
#[doc(alias = "VkDeviceFaultInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceFaultInfoEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub description: [c_char; 256],
    pub p_address_infos: *mut DeviceFaultAddressInfoKHR,
    pub p_vendor_infos: *mut DeviceFaultVendorInfoKHR,
    pub p_vendor_binary_data: *mut c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub rasterization_order_color_attachment_access: Bool32,
    pub rasterization_order_depth_attachment_access: Bool32,
    pub rasterization_order_stencil_attachment_access: Bool32,
}
pub type PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM =
    PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRGBA10X6FormatsFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format_rgba_10_x_6_without_y_cb_cr_sampler: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDirectFBSurfaceCreateInfoEXT.html>
#[doc(alias = "VkDirectFBSurfaceCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DirectFBSurfaceCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DirectFBSurfaceCreateFlagsEXT,
    pub dfb: *mut IDirectFB,
    pub surface: *mut IDirectFBSurface,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMutableDescriptorTypeFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub mutable_descriptor_type: Bool32,
}
pub type PhysicalDeviceMutableDescriptorTypeFeaturesVALVE =
    PhysicalDeviceMutableDescriptorTypeFeaturesEXT;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMutableDescriptorTypeListEXT.html>
#[doc(alias = "VkMutableDescriptorTypeListEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MutableDescriptorTypeListEXT {
    pub descriptor_type_count: u32,
    pub p_descriptor_types: *const DescriptorType,
}
pub type MutableDescriptorTypeListVALVE = MutableDescriptorTypeListEXT;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMutableDescriptorTypeCreateInfoEXT.html>
#[doc(alias = "VkMutableDescriptorTypeCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MutableDescriptorTypeCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mutable_descriptor_type_list_count: u32,
    pub p_mutable_descriptor_type_lists: *const MutableDescriptorTypeListEXT,
}
pub type MutableDescriptorTypeCreateInfoVALVE = MutableDescriptorTypeCreateInfoEXT;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVertexInputDynamicStateFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub vertex_input_dynamic_state: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVertexInputBindingDescription2EXT.html>
#[doc(alias = "VkVertexInputBindingDescription2EXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VertexInputBindingDescription2EXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub binding: u32,
    pub stride: u32,
    pub input_rate: VertexInputRate,
    pub divisor: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVertexInputAttributeDescription2EXT.html>
#[doc(alias = "VkVertexInputAttributeDescription2EXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VertexInputAttributeDescription2EXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub location: u32,
    pub binding: u32,
    pub format: Format,
    pub offset: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDrmPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceDrmPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDrmPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub has_primary: Bool32,
    pub has_render: Bool32,
    pub primary_major: i64,
    pub primary_minor: i64,
    pub render_major: i64,
    pub render_minor: i64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceAddressBindingReportFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceAddressBindingReportFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceAddressBindingReportFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub report_address_binding: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceAddressBindingCallbackDataEXT.html>
#[doc(alias = "VkDeviceAddressBindingCallbackDataEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceAddressBindingCallbackDataEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: DeviceAddressBindingFlagsEXT,
    pub base_address: DeviceAddress,
    pub size: DeviceSize,
    pub binding_type: DeviceAddressBindingTypeEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDepthClipControlFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceDepthClipControlFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDepthClipControlFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub depth_clip_control: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportDepthClipControlCreateInfoEXT.html>
#[doc(alias = "VkPipelineViewportDepthClipControlCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineViewportDepthClipControlCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub negative_one_to_one: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT.html>
#[doc(alias = "VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub primitive_topology_list_restart: Bool32,
    pub primitive_topology_patch_list_restart: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImportMemoryZirconHandleInfoFUCHSIA.html>
#[doc(alias = "VkImportMemoryZirconHandleInfoFUCHSIA")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImportMemoryZirconHandleInfoFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlags,
    pub handle: zx_handle_t,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryZirconHandlePropertiesFUCHSIA.html>
#[doc(alias = "VkMemoryZirconHandlePropertiesFUCHSIA")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryZirconHandlePropertiesFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_type_bits: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryGetZirconHandleInfoFUCHSIA.html>
#[doc(alias = "VkMemoryGetZirconHandleInfoFUCHSIA")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryGetZirconHandleInfoFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub handle_type: ExternalMemoryHandleTypeFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImportSemaphoreZirconHandleInfoFUCHSIA.html>
#[doc(alias = "VkImportSemaphoreZirconHandleInfoFUCHSIA")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImportSemaphoreZirconHandleInfoFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub flags: SemaphoreImportFlags,
    pub handle_type: ExternalSemaphoreHandleTypeFlags,
    pub zircon_handle: zx_handle_t,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreGetZirconHandleInfoFUCHSIA.html>
#[doc(alias = "VkSemaphoreGetZirconHandleInfoFUCHSIA")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SemaphoreGetZirconHandleInfoFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub handle_type: ExternalSemaphoreHandleTypeFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCollectionCreateInfoFUCHSIA.html>
#[doc(alias = "VkBufferCollectionCreateInfoFUCHSIA")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BufferCollectionCreateInfoFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub collection_token: zx_handle_t,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImportMemoryBufferCollectionFUCHSIA.html>
#[doc(alias = "VkImportMemoryBufferCollectionFUCHSIA")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImportMemoryBufferCollectionFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub collection: BufferCollectionFUCHSIA,
    pub index: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCollectionImageCreateInfoFUCHSIA.html>
#[doc(alias = "VkBufferCollectionImageCreateInfoFUCHSIA")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BufferCollectionImageCreateInfoFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub collection: BufferCollectionFUCHSIA,
    pub index: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCollectionConstraintsInfoFUCHSIA.html>
#[doc(alias = "VkBufferCollectionConstraintsInfoFUCHSIA")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BufferCollectionConstraintsInfoFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub min_buffer_count: u32,
    pub max_buffer_count: u32,
    pub min_buffer_count_for_camping: u32,
    pub min_buffer_count_for_dedicated_slack: u32,
    pub min_buffer_count_for_shared_slack: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferConstraintsInfoFUCHSIA.html>
#[doc(alias = "VkBufferConstraintsInfoFUCHSIA")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BufferConstraintsInfoFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub create_info: BufferCreateInfo,
    pub required_format_features: FormatFeatureFlags,
    pub buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCollectionBufferCreateInfoFUCHSIA.html>
#[doc(alias = "VkBufferCollectionBufferCreateInfoFUCHSIA")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BufferCollectionBufferCreateInfoFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub collection: BufferCollectionFUCHSIA,
    pub index: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSysmemColorSpaceFUCHSIA.html>
#[doc(alias = "VkSysmemColorSpaceFUCHSIA")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SysmemColorSpaceFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub color_space: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCollectionPropertiesFUCHSIA.html>
#[doc(alias = "VkBufferCollectionPropertiesFUCHSIA")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BufferCollectionPropertiesFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_type_bits: u32,
    pub buffer_count: u32,
    pub create_info_index: u32,
    pub sysmem_pixel_format: u64,
    pub format_features: FormatFeatureFlags,
    pub sysmem_color_space_index: SysmemColorSpaceFUCHSIA,
    pub sampler_ycbcr_conversion_components: ComponentMapping,
    pub suggested_ycbcr_model: SamplerYcbcrModelConversion,
    pub suggested_ycbcr_range: SamplerYcbcrRange,
    pub suggested_x_chroma_offset: ChromaLocation,
    pub suggested_y_chroma_offset: ChromaLocation,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageFormatConstraintsInfoFUCHSIA.html>
#[doc(alias = "VkImageFormatConstraintsInfoFUCHSIA")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageFormatConstraintsInfoFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_create_info: ImageCreateInfo,
    pub required_format_features: FormatFeatureFlags,
    pub flags: ImageFormatConstraintsFlagsFUCHSIA,
    pub sysmem_pixel_format: u64,
    pub color_space_count: u32,
    pub p_color_spaces: *const SysmemColorSpaceFUCHSIA,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageConstraintsInfoFUCHSIA.html>
#[doc(alias = "VkImageConstraintsInfoFUCHSIA")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageConstraintsInfoFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format_constraints_count: u32,
    pub p_format_constraints: *const ImageFormatConstraintsInfoFUCHSIA,
    pub buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA,
    pub flags: ImageConstraintsInfoFlagsFUCHSIA,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassShadingPipelineCreateInfoHUAWEI.html>
#[doc(alias = "VkSubpassShadingPipelineCreateInfoHUAWEI")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SubpassShadingPipelineCreateInfoHUAWEI {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub render_pass: RenderPass,
    pub subpass: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSubpassShadingFeaturesHUAWEI.html>
#[doc(alias = "VkPhysicalDeviceSubpassShadingFeaturesHUAWEI")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSubpassShadingFeaturesHUAWEI {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub subpass_shading: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSubpassShadingPropertiesHUAWEI.html>
#[doc(alias = "VkPhysicalDeviceSubpassShadingPropertiesHUAWEI")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSubpassShadingPropertiesHUAWEI {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_subpass_shading_workgroup_size_aspect_ratio: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceInvocationMaskFeaturesHUAWEI.html>
#[doc(alias = "VkPhysicalDeviceInvocationMaskFeaturesHUAWEI")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceInvocationMaskFeaturesHUAWEI {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub invocation_mask: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryGetRemoteAddressInfoNV.html>
#[doc(alias = "VkMemoryGetRemoteAddressInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryGetRemoteAddressInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub handle_type: ExternalMemoryHandleTypeFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalMemoryRDMAFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceExternalMemoryRDMAFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExternalMemoryRDMAFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_memory_rdma: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelinePropertiesIdentifierEXT.html>
#[doc(alias = "VkPipelinePropertiesIdentifierEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelinePropertiesIdentifierEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_identifier: [u8; 16],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelinePropertiesFeaturesEXT.html>
#[doc(alias = "VkPhysicalDevicePipelinePropertiesFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePipelinePropertiesFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_properties_identifier: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFrameBoundaryFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceFrameBoundaryFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFrameBoundaryFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub frame_boundary: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFrameBoundaryEXT.html>
#[doc(alias = "VkFrameBoundaryEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FrameBoundaryEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: FrameBoundaryFlagsEXT,
    pub frame_id: u64,
    pub image_count: u32,
    pub p_images: *const Image,
    pub buffer_count: u32,
    pub p_buffers: *const Buffer,
    pub tag_name: u64,
    pub tag_size: usize,
    pub p_tag: *const c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub multisampled_render_to_single_sampled: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassResolvePerformanceQueryEXT.html>
#[doc(alias = "VkSubpassResolvePerformanceQueryEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SubpassResolvePerformanceQueryEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub optimal: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMultisampledRenderToSingleSampledInfoEXT.html>
#[doc(alias = "VkMultisampledRenderToSingleSampledInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MultisampledRenderToSingleSampledInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub multisampled_render_to_single_sampled_enable: Bool32,
    pub rasterization_samples: SampleCountFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState2FeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceExtendedDynamicState2FeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub extended_dynamic_state_2: Bool32,
    pub extended_dynamic_state_2_logic_op: Bool32,
    pub extended_dynamic_state_2_patch_control_points: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkScreenSurfaceCreateInfoQNX.html>
#[doc(alias = "VkScreenSurfaceCreateInfoQNX")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ScreenSurfaceCreateInfoQNX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ScreenSurfaceCreateFlagsQNX,
    pub context: *mut _screen_context,
    pub window: *mut _screen_window,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceColorWriteEnableFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceColorWriteEnableFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceColorWriteEnableFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub color_write_enable: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineColorWriteCreateInfoEXT.html>
#[doc(alias = "VkPipelineColorWriteCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineColorWriteCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub attachment_count: u32,
    pub p_color_write_enables: *const Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT.html>
#[doc(alias = "VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub primitives_generated_query: Bool32,
    pub primitives_generated_query_with_rasterizer_discard: Bool32,
    pub primitives_generated_query_with_non_zero_streams: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE.html>
#[doc(alias = "VkPhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_encode_rgb_conversion: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbConversionCapabilitiesVALVE.html>
#[doc(alias = "VkVideoEncodeRgbConversionCapabilitiesVALVE")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeRgbConversionCapabilitiesVALVE {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub rgb_models: VideoEncodeRgbModelConversionFlagsVALVE,
    pub rgb_ranges: VideoEncodeRgbRangeCompressionFlagsVALVE,
    pub x_chroma_offsets: VideoEncodeRgbChromaOffsetFlagsVALVE,
    pub y_chroma_offsets: VideoEncodeRgbChromaOffsetFlagsVALVE,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeProfileRgbConversionInfoVALVE.html>
#[doc(alias = "VkVideoEncodeProfileRgbConversionInfoVALVE")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeProfileRgbConversionInfoVALVE {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub perform_encode_rgb_conversion: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeSessionRgbConversionCreateInfoVALVE.html>
#[doc(alias = "VkVideoEncodeSessionRgbConversionCreateInfoVALVE")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeSessionRgbConversionCreateInfoVALVE {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub rgb_model: VideoEncodeRgbModelConversionFlagsVALVE,
    pub rgb_range: VideoEncodeRgbRangeCompressionFlagsVALVE,
    pub x_chroma_offset: VideoEncodeRgbChromaOffsetFlagsVALVE,
    pub y_chroma_offset: VideoEncodeRgbChromaOffsetFlagsVALVE,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageViewMinLodFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceImageViewMinLodFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageViewMinLodFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_lod: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewMinLodCreateInfoEXT.html>
#[doc(alias = "VkImageViewMinLodCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageViewMinLodCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub min_lod: f32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMultiDrawFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceMultiDrawFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMultiDrawFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub multi_draw: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMultiDrawPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceMultiDrawPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMultiDrawPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_multi_draw_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMultiDrawInfoEXT.html>
#[doc(alias = "VkMultiDrawInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MultiDrawInfoEXT {
    pub first_vertex: u32,
    pub vertex_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMultiDrawIndexedInfoEXT.html>
#[doc(alias = "VkMultiDrawIndexedInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MultiDrawIndexedInfoEXT {
    pub first_index: u32,
    pub index_count: u32,
    pub vertex_offset: i32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImage2DViewOf3DFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceImage2DViewOf3DFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImage2DViewOf3DFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_2_d_view_of_3_d: Bool32,
    pub sampler_2_d_view_of_3_d: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderTileImageFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceShaderTileImageFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderTileImageFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_tile_image_color_read_access: Bool32,
    pub shader_tile_image_depth_read_access: Bool32,
    pub shader_tile_image_stencil_read_access: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderTileImagePropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceShaderTileImagePropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderTileImagePropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_tile_image_coherent_read_accelerated: Bool32,
    pub shader_tile_image_read_sample_from_pixel_rate_invocation: Bool32,
    pub shader_tile_image_read_from_helper_invocation: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapUsageEXT.html>
#[doc(alias = "VkMicromapUsageEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MicromapUsageEXT {
    pub count: u32,
    pub subdivision_level: u32,
    pub format: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceOrHostAddressKHR.html>
#[doc(alias = "VkDeviceOrHostAddressKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceOrHostAddressKHR {
    pub device_address: DeviceAddress,
    pub host_address: *mut c_void,
}
impl std::fmt::Debug for DeviceOrHostAddressKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeviceOrHostAddressKHR {{ .. }}")
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapBuildInfoEXT.html>
#[doc(alias = "VkMicromapBuildInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MicromapBuildInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub type_: MicromapTypeEXT,
    pub flags: BuildMicromapFlagsEXT,
    pub mode: BuildMicromapModeEXT,
    pub dst_micromap: MicromapEXT,
    pub usage_counts_count: u32,
    pub p_usage_counts: *const MicromapUsageEXT,
    pub pp_usage_counts: *const *const MicromapUsageEXT,
    pub data: DeviceOrHostAddressConstKHR,
    pub scratch_data: DeviceOrHostAddressKHR,
    pub triangle_array: DeviceOrHostAddressConstKHR,
    pub triangle_array_stride: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapCreateInfoEXT.html>
#[doc(alias = "VkMicromapCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MicromapCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub create_flags: MicromapCreateFlagsEXT,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub size: DeviceSize,
    pub type_: MicromapTypeEXT,
    pub device_address: DeviceAddress,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceOpacityMicromapFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceOpacityMicromapFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceOpacityMicromapFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub micromap: Bool32,
    pub micromap_capture_replay: Bool32,
    pub micromap_host_commands: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceOpacityMicromapPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceOpacityMicromapPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceOpacityMicromapPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_opacity_2_state_subdivision_level: u32,
    pub max_opacity_4_state_subdivision_level: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapVersionInfoEXT.html>
#[doc(alias = "VkMicromapVersionInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MicromapVersionInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_version_data: *const u8,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyMicromapToMemoryInfoEXT.html>
#[doc(alias = "VkCopyMicromapToMemoryInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CopyMicromapToMemoryInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src: MicromapEXT,
    pub dst: DeviceOrHostAddressKHR,
    pub mode: CopyMicromapModeEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyMemoryToMicromapInfoEXT.html>
#[doc(alias = "VkCopyMemoryToMicromapInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CopyMemoryToMicromapInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src: DeviceOrHostAddressConstKHR,
    pub dst: MicromapEXT,
    pub mode: CopyMicromapModeEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyMicromapInfoEXT.html>
#[doc(alias = "VkCopyMicromapInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CopyMicromapInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src: MicromapEXT,
    pub dst: MicromapEXT,
    pub mode: CopyMicromapModeEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapBuildSizesInfoEXT.html>
#[doc(alias = "VkMicromapBuildSizesInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MicromapBuildSizesInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub micromap_size: DeviceSize,
    pub build_scratch_size: DeviceSize,
    pub discardable: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureTrianglesOpacityMicromapEXT.html>
#[doc(alias = "VkAccelerationStructureTrianglesOpacityMicromapEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureTrianglesOpacityMicromapEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub index_type: IndexType,
    pub index_buffer: DeviceOrHostAddressConstKHR,
    pub index_stride: DeviceSize,
    pub base_triangle: u32,
    pub usage_counts_count: u32,
    pub p_usage_counts: *const MicromapUsageEXT,
    pub pp_usage_counts: *const *const MicromapUsageEXT,
    pub micromap: MicromapEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDisplacementMicromapFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceDisplacementMicromapFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDisplacementMicromapFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub displacement_micromap: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDisplacementMicromapPropertiesNV.html>
#[doc(alias = "VkPhysicalDeviceDisplacementMicromapPropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDisplacementMicromapPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_displacement_micromap_subdivision_level: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureTrianglesDisplacementMicromapNV.html>
#[doc(alias = "VkAccelerationStructureTrianglesDisplacementMicromapNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureTrianglesDisplacementMicromapNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub displacement_bias_and_scale_format: Format,
    pub displacement_vector_format: Format,
    pub displacement_bias_and_scale_buffer: DeviceOrHostAddressConstKHR,
    pub displacement_bias_and_scale_stride: DeviceSize,
    pub displacement_vector_buffer: DeviceOrHostAddressConstKHR,
    pub displacement_vector_stride: DeviceSize,
    pub displaced_micromap_primitive_flags: DeviceOrHostAddressConstKHR,
    pub displaced_micromap_primitive_flags_stride: DeviceSize,
    pub index_type: IndexType,
    pub index_buffer: DeviceOrHostAddressConstKHR,
    pub index_stride: DeviceSize,
    pub base_triangle: u32,
    pub usage_counts_count: u32,
    pub p_usage_counts: *const MicromapUsageEXT,
    pub pp_usage_counts: *const *const MicromapUsageEXT,
    pub micromap: MicromapEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI.html>
#[doc(alias = "VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceClusterCullingShaderFeaturesHUAWEI {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub clusterculling_shader: Bool32,
    pub multiview_cluster_culling_shader: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI.html>
#[doc(alias = "VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceClusterCullingShaderPropertiesHUAWEI {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_work_group_count: [u32; 3],
    pub max_work_group_size: [u32; 3],
    pub max_output_cluster_count: u32,
    pub indirect_buffer_offset_alignment: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI.html>
#[doc(alias = "VkPhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cluster_shading_rate: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceBorderColorSwizzleFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceBorderColorSwizzleFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceBorderColorSwizzleFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub border_color_swizzle: Bool32,
    pub border_color_swizzle_from_image: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerBorderColorComponentMappingCreateInfoEXT.html>
#[doc(alias = "VkSamplerBorderColorComponentMappingCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SamplerBorderColorComponentMappingCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub components: ComponentMapping,
    pub srgb: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT.html>
#[doc(alias = "VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pageable_device_local_memory: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderCorePropertiesARM.html>
#[doc(alias = "VkPhysicalDeviceShaderCorePropertiesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderCorePropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pixel_rate: u32,
    pub texel_rate: u32,
    pub fma_rate: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceQueueShaderCoreControlCreateInfoARM.html>
#[doc(alias = "VkDeviceQueueShaderCoreControlCreateInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceQueueShaderCoreControlCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_core_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSchedulingControlsFeaturesARM.html>
#[doc(alias = "VkPhysicalDeviceSchedulingControlsFeaturesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSchedulingControlsFeaturesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub scheduling_controls: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSchedulingControlsPropertiesARM.html>
#[doc(alias = "VkPhysicalDeviceSchedulingControlsPropertiesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSchedulingControlsPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub scheduling_controls_flags: PhysicalDeviceSchedulingControlsFlagsARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDispatchParametersARM.html>
#[doc(alias = "VkDispatchParametersARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DispatchParametersARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub work_group_batch_size: u32,
    pub max_queued_work_group_batches: u32,
    pub max_warps_per_shader_core: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM.html>
#[doc(alias = "VkPhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub scheduling_controls_max_warps_count: u32,
    pub scheduling_controls_max_queued_batches_count: u32,
    pub scheduling_controls_max_work_group_batch_size: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageSlicedViewOf3DFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceImageSlicedViewOf3DFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageSlicedViewOf3DFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_sliced_view_of_3_d: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewSlicedCreateInfoEXT.html>
#[doc(alias = "VkImageViewSlicedCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageViewSlicedCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub slice_offset: u32,
    pub slice_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE.html>
#[doc(alias = "VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub descriptor_set_host_mapping: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetBindingReferenceVALVE.html>
#[doc(alias = "VkDescriptorSetBindingReferenceVALVE")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorSetBindingReferenceVALVE {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub descriptor_set_layout: DescriptorSetLayout,
    pub binding: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayoutHostMappingInfoVALVE.html>
#[doc(alias = "VkDescriptorSetLayoutHostMappingInfoVALVE")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorSetLayoutHostMappingInfoVALVE {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub descriptor_offset: usize,
    pub descriptor_size: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceNonSeamlessCubeMapFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub non_seamless_cube_map: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRenderPassStripedFeaturesARM.html>
#[doc(alias = "VkPhysicalDeviceRenderPassStripedFeaturesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRenderPassStripedFeaturesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub render_pass_striped: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRenderPassStripedPropertiesARM.html>
#[doc(alias = "VkPhysicalDeviceRenderPassStripedPropertiesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRenderPassStripedPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub render_pass_stripe_granularity: Extent2D,
    pub max_render_pass_stripes: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassStripeInfoARM.html>
#[doc(alias = "VkRenderPassStripeInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderPassStripeInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stripe_area: Rect2D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassStripeBeginInfoARM.html>
#[doc(alias = "VkRenderPassStripeBeginInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderPassStripeBeginInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stripe_info_count: u32,
    pub p_stripe_infos: *const RenderPassStripeInfoARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassStripeSubmitInfoARM.html>
#[doc(alias = "VkRenderPassStripeSubmitInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderPassStripeSubmitInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stripe_semaphore_info_count: u32,
    pub p_stripe_semaphore_infos: *const SemaphoreSubmitInfo,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentDensityMapOffsetFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceFragmentDensityMapOffsetFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapOffsetFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub fragment_density_map_offset: Bool32,
}
pub type PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM =
    PhysicalDeviceFragmentDensityMapOffsetFeaturesEXT;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentDensityMapOffsetPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceFragmentDensityMapOffsetPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapOffsetPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub fragment_density_offset_granularity: Extent2D,
}
pub type PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM =
    PhysicalDeviceFragmentDensityMapOffsetPropertiesEXT;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassFragmentDensityMapOffsetEndInfoEXT.html>
#[doc(alias = "VkRenderPassFragmentDensityMapOffsetEndInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderPassFragmentDensityMapOffsetEndInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fragment_density_offset_count: u32,
    pub p_fragment_density_offsets: *const Offset2D,
}
pub type SubpassFragmentDensityMapOffsetEndInfoQCOM = RenderPassFragmentDensityMapOffsetEndInfoEXT;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCopyMemoryIndirectFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceCopyMemoryIndirectFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCopyMemoryIndirectFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub indirect_copy: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDecompressMemoryRegionNV.html>
#[doc(alias = "VkDecompressMemoryRegionNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DecompressMemoryRegionNV {
    pub src_address: DeviceAddress,
    pub dst_address: DeviceAddress,
    pub compressed_size: DeviceSize,
    pub decompressed_size: DeviceSize,
    pub decompression_method: MemoryDecompressionMethodFlagsEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMemoryDecompressionFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceMemoryDecompressionFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMemoryDecompressionFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_decompression: Bool32,
}
pub type PhysicalDeviceMemoryDecompressionFeaturesNV = PhysicalDeviceMemoryDecompressionFeaturesEXT;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMemoryDecompressionPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceMemoryDecompressionPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMemoryDecompressionPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub decompression_methods: MemoryDecompressionMethodFlagsEXT,
    pub max_decompression_indirect_count: u64,
}
pub type PhysicalDeviceMemoryDecompressionPropertiesNV =
    PhysicalDeviceMemoryDecompressionPropertiesEXT;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_generated_compute: Bool32,
    pub device_generated_compute_pipelines: Bool32,
    pub device_generated_compute_capture_replay: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkComputePipelineIndirectBufferInfoNV.html>
#[doc(alias = "VkComputePipelineIndirectBufferInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ComputePipelineIndirectBufferInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_address: DeviceAddress,
    pub size: DeviceSize,
    pub pipeline_device_address_capture_replay: DeviceAddress,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineIndirectDeviceAddressInfoNV.html>
#[doc(alias = "VkPipelineIndirectDeviceAddressInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineIndirectDeviceAddressInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline_bind_point: PipelineBindPoint,
    pub pipeline: Pipeline,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindPipelineIndirectCommandNV.html>
#[doc(alias = "VkBindPipelineIndirectCommandNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BindPipelineIndirectCommandNV {
    pub pipeline_address: DeviceAddress,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub spheres: Bool32,
    pub linear_swept_spheres: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureGeometryLinearSweptSpheresDataNV.html>
#[doc(alias = "VkAccelerationStructureGeometryLinearSweptSpheresDataNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureGeometryLinearSweptSpheresDataNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vertex_format: Format,
    pub vertex_data: DeviceOrHostAddressConstKHR,
    pub vertex_stride: DeviceSize,
    pub radius_format: Format,
    pub radius_data: DeviceOrHostAddressConstKHR,
    pub radius_stride: DeviceSize,
    pub index_type: IndexType,
    pub index_data: DeviceOrHostAddressConstKHR,
    pub index_stride: DeviceSize,
    pub indexing_mode: RayTracingLssIndexingModeNV,
    pub end_caps_mode: RayTracingLssPrimitiveEndCapsModeNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureGeometrySpheresDataNV.html>
#[doc(alias = "VkAccelerationStructureGeometrySpheresDataNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureGeometrySpheresDataNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vertex_format: Format,
    pub vertex_data: DeviceOrHostAddressConstKHR,
    pub vertex_stride: DeviceSize,
    pub radius_format: Format,
    pub radius_data: DeviceOrHostAddressConstKHR,
    pub radius_stride: DeviceSize,
    pub index_type: IndexType,
    pub index_data: DeviceOrHostAddressConstKHR,
    pub index_stride: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceLinearColorAttachmentFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceLinearColorAttachmentFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceLinearColorAttachmentFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub linear_color_attachment: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_compression_control_swapchain: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewSampleWeightCreateInfoQCOM.html>
#[doc(alias = "VkImageViewSampleWeightCreateInfoQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageViewSampleWeightCreateInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub filter_center: Offset2D,
    pub filter_size: Extent2D,
    pub num_phases: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageProcessingFeaturesQCOM.html>
#[doc(alias = "VkPhysicalDeviceImageProcessingFeaturesQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageProcessingFeaturesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub texture_sample_weighted: Bool32,
    pub texture_box_filter: Bool32,
    pub texture_block_match: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageProcessingPropertiesQCOM.html>
#[doc(alias = "VkPhysicalDeviceImageProcessingPropertiesQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageProcessingPropertiesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_weight_filter_phases: u32,
    pub max_weight_filter_dimension: Extent2D,
    pub max_block_match_region: Extent2D,
    pub max_box_filter_block_size: Extent2D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceNestedCommandBufferFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceNestedCommandBufferFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceNestedCommandBufferFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub nested_command_buffer: Bool32,
    pub nested_command_buffer_rendering: Bool32,
    pub nested_command_buffer_simultaneous_use: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceNestedCommandBufferPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceNestedCommandBufferPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceNestedCommandBufferPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_command_buffer_nesting_level: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkNativeBufferUsageOHOS.html>
#[doc(alias = "VkNativeBufferUsageOHOS")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct NativeBufferUsageOHOS {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ohos_native_buffer_usage: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkNativeBufferPropertiesOHOS.html>
#[doc(alias = "VkNativeBufferPropertiesOHOS")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct NativeBufferPropertiesOHOS {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub allocation_size: DeviceSize,
    pub memory_type_bits: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkNativeBufferFormatPropertiesOHOS.html>
#[doc(alias = "VkNativeBufferFormatPropertiesOHOS")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct NativeBufferFormatPropertiesOHOS {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format: Format,
    pub external_format: u64,
    pub format_features: FormatFeatureFlags,
    pub sampler_ycbcr_conversion_components: ComponentMapping,
    pub suggested_ycbcr_model: SamplerYcbcrModelConversion,
    pub suggested_ycbcr_range: SamplerYcbcrRange,
    pub suggested_x_chroma_offset: ChromaLocation,
    pub suggested_y_chroma_offset: ChromaLocation,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImportNativeBufferInfoOHOS.html>
#[doc(alias = "VkImportNativeBufferInfoOHOS")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImportNativeBufferInfoOHOS {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: *mut OH_NativeBuffer,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryGetNativeBufferInfoOHOS.html>
#[doc(alias = "VkMemoryGetNativeBufferInfoOHOS")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryGetNativeBufferInfoOHOS {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFormatOHOS.html>
#[doc(alias = "VkExternalFormatOHOS")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExternalFormatOHOS {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_format: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryAcquireUnmodifiedEXT.html>
#[doc(alias = "VkExternalMemoryAcquireUnmodifiedEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExternalMemoryAcquireUnmodifiedEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acquire_unmodified_memory: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceExtendedDynamicState3FeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExtendedDynamicState3FeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub extended_dynamic_state_3_tessellation_domain_origin: Bool32,
    pub extended_dynamic_state_3_depth_clamp_enable: Bool32,
    pub extended_dynamic_state_3_polygon_mode: Bool32,
    pub extended_dynamic_state_3_rasterization_samples: Bool32,
    pub extended_dynamic_state_3_sample_mask: Bool32,
    pub extended_dynamic_state_3_alpha_to_coverage_enable: Bool32,
    pub extended_dynamic_state_3_alpha_to_one_enable: Bool32,
    pub extended_dynamic_state_3_logic_op_enable: Bool32,
    pub extended_dynamic_state_3_color_blend_enable: Bool32,
    pub extended_dynamic_state_3_color_blend_equation: Bool32,
    pub extended_dynamic_state_3_color_write_mask: Bool32,
    pub extended_dynamic_state_3_rasterization_stream: Bool32,
    pub extended_dynamic_state_3_conservative_rasterization_mode: Bool32,
    pub extended_dynamic_state_3_extra_primitive_overestimation_size: Bool32,
    pub extended_dynamic_state_3_depth_clip_enable: Bool32,
    pub extended_dynamic_state_3_sample_locations_enable: Bool32,
    pub extended_dynamic_state_3_color_blend_advanced: Bool32,
    pub extended_dynamic_state_3_provoking_vertex_mode: Bool32,
    pub extended_dynamic_state_3_line_rasterization_mode: Bool32,
    pub extended_dynamic_state_3_line_stipple_enable: Bool32,
    pub extended_dynamic_state_3_depth_clip_negative_one_to_one: Bool32,
    pub extended_dynamic_state_3_viewport_w_scaling_enable: Bool32,
    pub extended_dynamic_state_3_viewport_swizzle: Bool32,
    pub extended_dynamic_state_3_coverage_to_color_enable: Bool32,
    pub extended_dynamic_state_3_coverage_to_color_location: Bool32,
    pub extended_dynamic_state_3_coverage_modulation_mode: Bool32,
    pub extended_dynamic_state_3_coverage_modulation_table_enable: Bool32,
    pub extended_dynamic_state_3_coverage_modulation_table: Bool32,
    pub extended_dynamic_state_3_coverage_reduction_mode: Bool32,
    pub extended_dynamic_state_3_representative_fragment_test_enable: Bool32,
    pub extended_dynamic_state_3_shading_rate_image_enable: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3PropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceExtendedDynamicState3PropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExtendedDynamicState3PropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub dynamic_primitive_topology_unrestricted: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkColorBlendEquationEXT.html>
#[doc(alias = "VkColorBlendEquationEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ColorBlendEquationEXT {
    pub src_color_blend_factor: BlendFactor,
    pub dst_color_blend_factor: BlendFactor,
    pub color_blend_op: BlendOp,
    pub src_alpha_blend_factor: BlendFactor,
    pub dst_alpha_blend_factor: BlendFactor,
    pub alpha_blend_op: BlendOp,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkColorBlendAdvancedEXT.html>
#[doc(alias = "VkColorBlendAdvancedEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ColorBlendAdvancedEXT {
    pub advanced_blend_op: BlendOp,
    pub src_premultiplied: Bool32,
    pub dst_premultiplied: Bool32,
    pub blend_overlap: BlendOverlapEXT,
    pub clamp_results: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSubpassMergeFeedbackFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub subpass_merge_feedback: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassCreationControlEXT.html>
#[doc(alias = "VkRenderPassCreationControlEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderPassCreationControlEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub disallow_merging: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassCreationFeedbackInfoEXT.html>
#[doc(alias = "VkRenderPassCreationFeedbackInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderPassCreationFeedbackInfoEXT {
    pub post_merge_subpass_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassCreationFeedbackCreateInfoEXT.html>
#[doc(alias = "VkRenderPassCreationFeedbackCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderPassCreationFeedbackCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_render_pass_feedback: *mut RenderPassCreationFeedbackInfoEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassSubpassFeedbackInfoEXT.html>
#[doc(alias = "VkRenderPassSubpassFeedbackInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderPassSubpassFeedbackInfoEXT {
    pub subpass_merge_status: SubpassMergeStatusEXT,
    pub description: [c_char; 256],
    pub post_merge_index: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassSubpassFeedbackCreateInfoEXT.html>
#[doc(alias = "VkRenderPassSubpassFeedbackCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderPassSubpassFeedbackCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_subpass_feedback: *mut RenderPassSubpassFeedbackInfoEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDirectDriverLoadingInfoLUNARG.html>
#[doc(alias = "VkDirectDriverLoadingInfoLUNARG")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DirectDriverLoadingInfoLUNARG {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: DirectDriverLoadingFlagsLUNARG,
    pub pfn_get_instance_proc_addr: vkGetInstanceProcAddrLUNARG,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDirectDriverLoadingListLUNARG.html>
#[doc(alias = "VkDirectDriverLoadingListLUNARG")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DirectDriverLoadingListLUNARG {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mode: DirectDriverLoadingModeLUNARG,
    pub driver_count: u32,
    pub p_drivers: *const DirectDriverLoadingInfoLUNARG,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorDescriptionARM.html>
#[doc(alias = "VkTensorDescriptionARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TensorDescriptionARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tiling: TensorTilingARM,
    pub format: Format,
    pub dimension_count: u32,
    pub p_dimensions: *const i64,
    pub p_strides: *const i64,
    pub usage: TensorUsageFlagsARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorCreateInfoARM.html>
#[doc(alias = "VkTensorCreateInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TensorCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: TensorCreateFlagsARM,
    pub p_description: *const TensorDescriptionARM,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorMemoryRequirementsInfoARM.html>
#[doc(alias = "VkTensorMemoryRequirementsInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TensorMemoryRequirementsInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor: TensorARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindTensorMemoryInfoARM.html>
#[doc(alias = "VkBindTensorMemoryInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BindTensorMemoryInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor: TensorARM,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkWriteDescriptorSetTensorARM.html>
#[doc(alias = "VkWriteDescriptorSetTensorARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct WriteDescriptorSetTensorARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor_view_count: u32,
    pub p_tensor_views: *const TensorViewARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorFormatPropertiesARM.html>
#[doc(alias = "VkTensorFormatPropertiesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TensorFormatPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub optimal_tiling_tensor_features: FormatFeatureFlags2,
    pub linear_tiling_tensor_features: FormatFeatureFlags2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTensorPropertiesARM.html>
#[doc(alias = "VkPhysicalDeviceTensorPropertiesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTensorPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_tensor_dimension_count: u32,
    pub max_tensor_elements: u64,
    pub max_per_dimension_tensor_elements: u64,
    pub max_tensor_stride: i64,
    pub max_tensor_size: u64,
    pub max_tensor_shader_access_array_length: u32,
    pub max_tensor_shader_access_size: u32,
    pub max_descriptor_set_storage_tensors: u32,
    pub max_per_stage_descriptor_set_storage_tensors: u32,
    pub max_descriptor_set_update_after_bind_storage_tensors: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_tensors: u32,
    pub shader_storage_tensor_array_non_uniform_indexing_native: Bool32,
    pub shader_tensor_supported_stages: ShaderStageFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorMemoryBarrierARM.html>
#[doc(alias = "VkTensorMemoryBarrierARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TensorMemoryBarrierARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_stage_mask: PipelineStageFlags2,
    pub src_access_mask: AccessFlags2,
    pub dst_stage_mask: PipelineStageFlags2,
    pub dst_access_mask: AccessFlags2,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub tensor: TensorARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorDependencyInfoARM.html>
#[doc(alias = "VkTensorDependencyInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TensorDependencyInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor_memory_barrier_count: u32,
    pub p_tensor_memory_barriers: *const TensorMemoryBarrierARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTensorFeaturesARM.html>
#[doc(alias = "VkPhysicalDeviceTensorFeaturesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTensorFeaturesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tensor_non_packed: Bool32,
    pub shader_tensor_access: Bool32,
    pub shader_storage_tensor_array_dynamic_indexing: Bool32,
    pub shader_storage_tensor_array_non_uniform_indexing: Bool32,
    pub descriptor_binding_storage_tensor_update_after_bind: Bool32,
    pub tensors: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceTensorMemoryRequirementsARM.html>
#[doc(alias = "VkDeviceTensorMemoryRequirementsARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DeviceTensorMemoryRequirementsARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_create_info: *const TensorCreateInfoARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorCopyARM.html>
#[doc(alias = "VkTensorCopyARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TensorCopyARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dimension_count: u32,
    pub p_src_offset: *const u64,
    pub p_dst_offset: *const u64,
    pub p_extent: *const u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyTensorInfoARM.html>
#[doc(alias = "VkCopyTensorInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CopyTensorInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_tensor: TensorARM,
    pub dst_tensor: TensorARM,
    pub region_count: u32,
    pub p_regions: *const TensorCopyARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryDedicatedAllocateInfoTensorARM.html>
#[doc(alias = "VkMemoryDedicatedAllocateInfoTensorARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryDedicatedAllocateInfoTensorARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor: TensorARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalTensorInfoARM.html>
#[doc(alias = "VkPhysicalDeviceExternalTensorInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExternalTensorInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: TensorCreateFlagsARM,
    pub p_description: *const TensorDescriptionARM,
    pub handle_type: ExternalMemoryHandleTypeFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalTensorPropertiesARM.html>
#[doc(alias = "VkExternalTensorPropertiesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExternalTensorPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub external_memory_properties: ExternalMemoryProperties,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryTensorCreateInfoARM.html>
#[doc(alias = "VkExternalMemoryTensorCreateInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExternalMemoryTensorCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDescriptorBufferTensorFeaturesARM.html>
#[doc(alias = "VkPhysicalDeviceDescriptorBufferTensorFeaturesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDescriptorBufferTensorFeaturesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub descriptor_buffer_tensor_descriptors: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDescriptorBufferTensorPropertiesARM.html>
#[doc(alias = "VkPhysicalDeviceDescriptorBufferTensorPropertiesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDescriptorBufferTensorPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tensor_capture_replay_descriptor_data_size: usize,
    pub tensor_view_capture_replay_descriptor_data_size: usize,
    pub tensor_descriptor_size: usize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorGetTensorInfoARM.html>
#[doc(alias = "VkDescriptorGetTensorInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorGetTensorInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor_view: TensorViewARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorCaptureDescriptorDataInfoARM.html>
#[doc(alias = "VkTensorCaptureDescriptorDataInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TensorCaptureDescriptorDataInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor: TensorARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorViewCaptureDescriptorDataInfoARM.html>
#[doc(alias = "VkTensorViewCaptureDescriptorDataInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TensorViewCaptureDescriptorDataInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor_view: TensorViewARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFrameBoundaryTensorsARM.html>
#[doc(alias = "VkFrameBoundaryTensorsARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FrameBoundaryTensorsARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor_count: u32,
    pub p_tensors: *const TensorARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderModuleIdentifierFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_module_identifier: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderModuleIdentifierPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceShaderModuleIdentifierPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderModuleIdentifierPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_module_identifier_algorithm_uuid: [u8; 16],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineShaderStageModuleIdentifierCreateInfoEXT.html>
#[doc(alias = "VkPipelineShaderStageModuleIdentifierCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineShaderStageModuleIdentifierCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub identifier_size: u32,
    pub p_identifier: *const u8,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderModuleIdentifierEXT.html>
#[doc(alias = "VkShaderModuleIdentifierEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ShaderModuleIdentifierEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub identifier_size: u32,
    pub identifier: [u8; 32],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceOpticalFlowFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceOpticalFlowFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceOpticalFlowFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub optical_flow: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceOpticalFlowPropertiesNV.html>
#[doc(alias = "VkPhysicalDeviceOpticalFlowPropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceOpticalFlowPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supported_output_grid_sizes: OpticalFlowGridSizeFlagsNV,
    pub supported_hint_grid_sizes: OpticalFlowGridSizeFlagsNV,
    pub hint_supported: Bool32,
    pub cost_supported: Bool32,
    pub bidirectional_flow_supported: Bool32,
    pub global_flow_supported: Bool32,
    pub min_width: u32,
    pub min_height: u32,
    pub max_width: u32,
    pub max_height: u32,
    pub max_num_regions_of_interest: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowImageFormatInfoNV.html>
#[doc(alias = "VkOpticalFlowImageFormatInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct OpticalFlowImageFormatInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub usage: OpticalFlowUsageFlagsNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowImageFormatPropertiesNV.html>
#[doc(alias = "VkOpticalFlowImageFormatPropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct OpticalFlowImageFormatPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format: Format,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowSessionCreateInfoNV.html>
#[doc(alias = "VkOpticalFlowSessionCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct OpticalFlowSessionCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub width: u32,
    pub height: u32,
    pub image_format: Format,
    pub flow_vector_format: Format,
    pub cost_format: Format,
    pub output_grid_size: OpticalFlowGridSizeFlagsNV,
    pub hint_grid_size: OpticalFlowGridSizeFlagsNV,
    pub performance_level: OpticalFlowPerformanceLevelNV,
    pub flags: OpticalFlowSessionCreateFlagsNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowSessionCreatePrivateDataInfoNV.html>
#[doc(alias = "VkOpticalFlowSessionCreatePrivateDataInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct OpticalFlowSessionCreatePrivateDataInfoNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub id: u32,
    pub size: u32,
    pub p_private_data: *const c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowExecuteInfoNV.html>
#[doc(alias = "VkOpticalFlowExecuteInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct OpticalFlowExecuteInfoNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: OpticalFlowExecuteFlagsNV,
    pub region_count: u32,
    pub p_regions: *const Rect2D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceLegacyDitheringFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceLegacyDitheringFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceLegacyDitheringFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub legacy_dithering: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalFormatResolveFeaturesANDROID.html>
#[doc(alias = "VkPhysicalDeviceExternalFormatResolveFeaturesANDROID")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExternalFormatResolveFeaturesANDROID {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_format_resolve: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalFormatResolvePropertiesANDROID.html>
#[doc(alias = "VkPhysicalDeviceExternalFormatResolvePropertiesANDROID")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExternalFormatResolvePropertiesANDROID {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub null_color_attachment_with_external_format_resolve: Bool32,
    pub external_format_resolve_chroma_offset_x: ChromaLocation,
    pub external_format_resolve_chroma_offset_y: ChromaLocation,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAndroidHardwareBufferFormatResolvePropertiesANDROID.html>
#[doc(alias = "VkAndroidHardwareBufferFormatResolvePropertiesANDROID")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AndroidHardwareBufferFormatResolvePropertiesANDROID {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub color_attachment_format: Format,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceAntiLagFeaturesAMD.html>
#[doc(alias = "VkPhysicalDeviceAntiLagFeaturesAMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceAntiLagFeaturesAMD {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub anti_lag: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAntiLagPresentationInfoAMD.html>
#[doc(alias = "VkAntiLagPresentationInfoAMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AntiLagPresentationInfoAMD {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub stage: AntiLagStageAMD,
    pub frame_index: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAntiLagDataAMD.html>
#[doc(alias = "VkAntiLagDataAMD")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AntiLagDataAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mode: AntiLagModeAMD,
    pub max_fps: u32,
    pub p_presentation_info: *const AntiLagPresentationInfoAMD,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDenseGeometryFormatFeaturesAMDX.html>
#[doc(alias = "VkPhysicalDeviceDenseGeometryFormatFeaturesAMDX")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDenseGeometryFormatFeaturesAMDX {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub dense_geometry_format: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureDenseGeometryFormatTrianglesDataAMDX.html>
#[doc(alias = "VkAccelerationStructureDenseGeometryFormatTrianglesDataAMDX")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureDenseGeometryFormatTrianglesDataAMDX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub compressed_data: DeviceOrHostAddressConstKHR,
    pub data_size: DeviceSize,
    pub num_triangles: u32,
    pub num_vertices: u32,
    pub max_primitive_index: u32,
    pub max_geometry_index: u32,
    pub format: CompressedTriangleFormatAMDX,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderObjectFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceShaderObjectFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderObjectFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_object: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderObjectPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceShaderObjectPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderObjectPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_binary_uuid: [u8; 16],
    pub shader_binary_version: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderCreateInfoEXT.html>
#[doc(alias = "VkShaderCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ShaderCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ShaderCreateFlagsEXT,
    pub stage: ShaderStageFlags,
    pub next_stage: ShaderStageFlags,
    pub code_type: ShaderCodeTypeEXT,
    pub code_size: usize,
    pub p_code: *const c_void,
    pub p_name: *const c_char,
    pub set_layout_count: u32,
    pub p_set_layouts: *const DescriptorSetLayout,
    pub push_constant_range_count: u32,
    pub p_push_constant_ranges: *const PushConstantRange,
    pub p_specialization_info: *const SpecializationInfo,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDepthClampRangeEXT.html>
#[doc(alias = "VkDepthClampRangeEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DepthClampRangeEXT {
    pub min_depth_clamp: f32,
    pub max_depth_clamp: f32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTilePropertiesFeaturesQCOM.html>
#[doc(alias = "VkPhysicalDeviceTilePropertiesFeaturesQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTilePropertiesFeaturesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tile_properties: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTilePropertiesQCOM.html>
#[doc(alias = "VkTilePropertiesQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TilePropertiesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tile_size: Extent3D,
    pub apron_size: Extent2D,
    pub origin: Offset2D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceAmigoProfilingFeaturesSEC.html>
#[doc(alias = "VkPhysicalDeviceAmigoProfilingFeaturesSEC")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceAmigoProfilingFeaturesSEC {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub amigo_profiling: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAmigoProfilingSubmitInfoSEC.html>
#[doc(alias = "VkAmigoProfilingSubmitInfoSEC")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AmigoProfilingSubmitInfoSEC {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub first_draw_timestamp: u64,
    pub swap_buffer_timestamp: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM.html>
#[doc(alias = "VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub multiview_per_view_viewports: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV.html>
#[doc(alias = "VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingInvocationReorderPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_tracing_invocation_reorder_reordering_hint: RayTracingInvocationReorderModeEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingInvocationReorderFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_tracing_invocation_reorder: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCooperativeVectorPropertiesNV.html>
#[doc(alias = "VkPhysicalDeviceCooperativeVectorPropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCooperativeVectorPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_vector_supported_stages: ShaderStageFlags,
    pub cooperative_vector_training_float_16_accumulation: Bool32,
    pub cooperative_vector_training_float_32_accumulation: Bool32,
    pub max_cooperative_vector_components: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCooperativeVectorFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceCooperativeVectorFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCooperativeVectorFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_vector: Bool32,
    pub cooperative_vector_training: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCooperativeVectorPropertiesNV.html>
#[doc(alias = "VkCooperativeVectorPropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CooperativeVectorPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub input_type: ComponentTypeKHR,
    pub input_interpretation: ComponentTypeKHR,
    pub matrix_interpretation: ComponentTypeKHR,
    pub bias_interpretation: ComponentTypeKHR,
    pub result_type: ComponentTypeKHR,
    pub transpose: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkConvertCooperativeVectorMatrixInfoNV.html>
#[doc(alias = "VkConvertCooperativeVectorMatrixInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ConvertCooperativeVectorMatrixInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_size: usize,
    pub src_data: DeviceOrHostAddressConstKHR,
    pub p_dst_size: *mut usize,
    pub dst_data: DeviceOrHostAddressKHR,
    pub src_component_type: ComponentTypeKHR,
    pub dst_component_type: ComponentTypeKHR,
    pub num_rows: u32,
    pub num_columns: u32,
    pub src_layout: CooperativeVectorMatrixLayoutNV,
    pub src_stride: usize,
    pub dst_layout: CooperativeVectorMatrixLayoutNV,
    pub dst_stride: usize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedSparseAddressSpaceFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceExtendedSparseAddressSpaceFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExtendedSparseAddressSpaceFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub extended_sparse_address_space: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNV.html>
#[doc(alias = "VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExtendedSparseAddressSpacePropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub extended_sparse_address_space_size: DeviceSize,
    pub extended_sparse_image_usage_flags: ImageUsageFlags,
    pub extended_sparse_buffer_usage_flags: BufferUsageFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceLegacyVertexAttributesFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceLegacyVertexAttributesFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceLegacyVertexAttributesFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub legacy_vertex_attributes: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceLegacyVertexAttributesPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceLegacyVertexAttributesPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceLegacyVertexAttributesPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub native_unaligned_performance: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkLayerSettingEXT.html>
#[doc(alias = "VkLayerSettingEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct LayerSettingEXT {
    pub p_layer_name: *const c_char,
    pub p_setting_name: *const c_char,
    pub type_: LayerSettingTypeEXT,
    pub value_count: u32,
    pub p_values: *const c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkLayerSettingsCreateInfoEXT.html>
#[doc(alias = "VkLayerSettingsCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct LayerSettingsCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub setting_count: u32,
    pub p_settings: *const LayerSettingEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM.html>
#[doc(alias = "VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderCoreBuiltinsFeaturesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_core_builtins: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM.html>
#[doc(alias = "VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderCoreBuiltinsPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_core_mask: u64,
    pub shader_core_count: u32,
    pub shader_warps_per_core: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub dynamic_rendering_unused_attachments: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkLatencySleepModeInfoNV.html>
#[doc(alias = "VkLatencySleepModeInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct LatencySleepModeInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub low_latency_mode: Bool32,
    pub low_latency_boost: Bool32,
    pub minimum_interval_us: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkLatencySleepInfoNV.html>
#[doc(alias = "VkLatencySleepInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct LatencySleepInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub signal_semaphore: Semaphore,
    pub value: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSetLatencyMarkerInfoNV.html>
#[doc(alias = "VkSetLatencyMarkerInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SetLatencyMarkerInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_id: u64,
    pub marker: LatencyMarkerNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkLatencyTimingsFrameReportNV.html>
#[doc(alias = "VkLatencyTimingsFrameReportNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct LatencyTimingsFrameReportNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_id: u64,
    pub input_sample_time_us: u64,
    pub sim_start_time_us: u64,
    pub sim_end_time_us: u64,
    pub render_submit_start_time_us: u64,
    pub render_submit_end_time_us: u64,
    pub present_start_time_us: u64,
    pub present_end_time_us: u64,
    pub driver_start_time_us: u64,
    pub driver_end_time_us: u64,
    pub os_render_queue_start_time_us: u64,
    pub os_render_queue_end_time_us: u64,
    pub gpu_render_start_time_us: u64,
    pub gpu_render_end_time_us: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGetLatencyMarkerInfoNV.html>
#[doc(alias = "VkGetLatencyMarkerInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GetLatencyMarkerInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub timing_count: u32,
    pub p_timings: *mut LatencyTimingsFrameReportNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkLatencySubmissionPresentIdNV.html>
#[doc(alias = "VkLatencySubmissionPresentIdNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct LatencySubmissionPresentIdNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_id: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainLatencyCreateInfoNV.html>
#[doc(alias = "VkSwapchainLatencyCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SwapchainLatencyCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub latency_mode_enable: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOutOfBandQueueTypeInfoNV.html>
#[doc(alias = "VkOutOfBandQueueTypeInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct OutOfBandQueueTypeInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub queue_type: OutOfBandQueueTypeNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkLatencySurfaceCapabilitiesNV.html>
#[doc(alias = "VkLatencySurfaceCapabilitiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct LatencySurfaceCapabilitiesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_mode_count: u32,
    pub p_present_modes: *mut PresentModeKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDataGraphFeaturesARM.html>
#[doc(alias = "VkPhysicalDeviceDataGraphFeaturesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDataGraphFeaturesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub data_graph: Bool32,
    pub data_graph_update_after_bind: Bool32,
    pub data_graph_specialization_constants: Bool32,
    pub data_graph_descriptor_buffer: Bool32,
    pub data_graph_shader_module: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineConstantARM.html>
#[doc(alias = "VkDataGraphPipelineConstantARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphPipelineConstantARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub id: u32,
    pub p_constant_data: *const c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineResourceInfoARM.html>
#[doc(alias = "VkDataGraphPipelineResourceInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphPipelineResourceInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub descriptor_set: u32,
    pub binding: u32,
    pub array_element: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineCompilerControlCreateInfoARM.html>
#[doc(alias = "VkDataGraphPipelineCompilerControlCreateInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphPipelineCompilerControlCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_vendor_options: *const c_char,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineCreateInfoARM.html>
#[doc(alias = "VkDataGraphPipelineCreateInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphPipelineCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags2,
    pub layout: PipelineLayout,
    pub resource_info_count: u32,
    pub p_resource_infos: *const DataGraphPipelineResourceInfoARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineShaderModuleCreateInfoARM.html>
#[doc(alias = "VkDataGraphPipelineShaderModuleCreateInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphPipelineShaderModuleCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub module: ShaderModule,
    pub p_name: *const c_char,
    pub p_specialization_info: *const SpecializationInfo,
    pub constant_count: u32,
    pub p_constants: *const DataGraphPipelineConstantARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionCreateInfoARM.html>
#[doc(alias = "VkDataGraphPipelineSessionCreateInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphPipelineSessionCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DataGraphPipelineSessionCreateFlagsARM,
    pub data_graph_pipeline: Pipeline,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionBindPointRequirementsInfoARM.html>
#[doc(alias = "VkDataGraphPipelineSessionBindPointRequirementsInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphPipelineSessionBindPointRequirementsInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub session: DataGraphPipelineSessionARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionBindPointRequirementARM.html>
#[doc(alias = "VkDataGraphPipelineSessionBindPointRequirementARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphPipelineSessionBindPointRequirementARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub bind_point: DataGraphPipelineSessionBindPointARM,
    pub bind_point_type: DataGraphPipelineSessionBindPointTypeARM,
    pub num_objects: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionMemoryRequirementsInfoARM.html>
#[doc(alias = "VkDataGraphPipelineSessionMemoryRequirementsInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphPipelineSessionMemoryRequirementsInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub session: DataGraphPipelineSessionARM,
    pub bind_point: DataGraphPipelineSessionBindPointARM,
    pub object_index: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindDataGraphPipelineSessionMemoryInfoARM.html>
#[doc(alias = "VkBindDataGraphPipelineSessionMemoryInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BindDataGraphPipelineSessionMemoryInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub session: DataGraphPipelineSessionARM,
    pub bind_point: DataGraphPipelineSessionBindPointARM,
    pub object_index: u32,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineInfoARM.html>
#[doc(alias = "VkDataGraphPipelineInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphPipelineInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub data_graph_pipeline: Pipeline,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelinePropertyQueryResultARM.html>
#[doc(alias = "VkDataGraphPipelinePropertyQueryResultARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphPipelinePropertyQueryResultARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub property: DataGraphPipelinePropertyARM,
    pub is_text: Bool32,
    pub data_size: usize,
    pub p_data: *mut c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineIdentifierCreateInfoARM.html>
#[doc(alias = "VkDataGraphPipelineIdentifierCreateInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphPipelineIdentifierCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub identifier_size: u32,
    pub p_identifier: *const u8,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineDispatchInfoARM.html>
#[doc(alias = "VkDataGraphPipelineDispatchInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphPipelineDispatchInfoARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: DataGraphPipelineDispatchFlagsARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDataGraphProcessingEngineARM.html>
#[doc(alias = "VkPhysicalDeviceDataGraphProcessingEngineARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDataGraphProcessingEngineARM {
    pub type_: PhysicalDeviceDataGraphProcessingEngineTypeARM,
    pub is_foreign: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDataGraphOperationSupportARM.html>
#[doc(alias = "VkPhysicalDeviceDataGraphOperationSupportARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDataGraphOperationSupportARM {
    pub operation_type: PhysicalDeviceDataGraphOperationTypeARM,
    pub name: [c_char; 128],
    pub version: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyDataGraphPropertiesARM.html>
#[doc(alias = "VkQueueFamilyDataGraphPropertiesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QueueFamilyDataGraphPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub engine: PhysicalDeviceDataGraphProcessingEngineARM,
    pub operation: PhysicalDeviceDataGraphOperationSupportARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphProcessingEngineCreateInfoARM.html>
#[doc(alias = "VkDataGraphProcessingEngineCreateInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphProcessingEngineCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub processing_engine_count: u32,
    pub p_processing_engines: *mut PhysicalDeviceDataGraphProcessingEngineARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM.html>
#[doc(alias = "VkPhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub queue_family_index: u32,
    pub engine_type: PhysicalDeviceDataGraphProcessingEngineTypeARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyDataGraphProcessingEnginePropertiesARM.html>
#[doc(alias = "VkQueueFamilyDataGraphProcessingEnginePropertiesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QueueFamilyDataGraphProcessingEnginePropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub foreign_semaphore_handle_types: ExternalSemaphoreHandleTypeFlags,
    pub foreign_memory_handle_types: ExternalMemoryHandleTypeFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineConstantTensorSemiStructuredSparsityInfoARM.html>
#[doc(alias = "VkDataGraphPipelineConstantTensorSemiStructuredSparsityInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphPipelineConstantTensorSemiStructuredSparsityInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dimension: u32,
    pub zero_count: u32,
    pub group_size: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphTOSANameQualityARM.html>
#[doc(alias = "VkDataGraphTOSANameQualityARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphTOSANameQualityARM {
    pub name: [c_char; 128],
    pub quality_flags: DataGraphTOSAQualityFlagsARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyDataGraphTOSAPropertiesARM.html>
#[doc(alias = "VkQueueFamilyDataGraphTOSAPropertiesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QueueFamilyDataGraphTOSAPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub profile_count: u32,
    pub p_profiles: *const DataGraphTOSANameQualityARM,
    pub extension_count: u32,
    pub p_extensions: *const DataGraphTOSANameQualityARM,
    pub level: DataGraphTOSALevelARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM.html>
#[doc(alias = "VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub multiview_per_view_render_areas: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM.html>
#[doc(alias = "VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MultiviewPerViewRenderAreasRenderPassBeginInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub per_view_render_area_count: u32,
    pub p_per_view_render_areas: *const Rect2D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePerStageDescriptorSetFeaturesNV.html>
#[doc(alias = "VkPhysicalDevicePerStageDescriptorSetFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePerStageDescriptorSetFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub per_stage_descriptor_set: Bool32,
    pub dynamic_pipeline_layout: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageProcessing2FeaturesQCOM.html>
#[doc(alias = "VkPhysicalDeviceImageProcessing2FeaturesQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageProcessing2FeaturesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub texture_block_match_2: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageProcessing2PropertiesQCOM.html>
#[doc(alias = "VkPhysicalDeviceImageProcessing2PropertiesQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageProcessing2PropertiesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_block_match_window: Extent2D,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerBlockMatchWindowCreateInfoQCOM.html>
#[doc(alias = "VkSamplerBlockMatchWindowCreateInfoQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SamplerBlockMatchWindowCreateInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub window_extent: Extent2D,
    pub window_compare_mode: BlockMatchWindowCompareModeQCOM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCubicWeightsFeaturesQCOM.html>
#[doc(alias = "VkPhysicalDeviceCubicWeightsFeaturesQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCubicWeightsFeaturesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub selectable_cubic_weights: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerCubicWeightsCreateInfoQCOM.html>
#[doc(alias = "VkSamplerCubicWeightsCreateInfoQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SamplerCubicWeightsCreateInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub cubic_weights: CubicFilterWeightsQCOM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBlitImageCubicWeightsInfoQCOM.html>
#[doc(alias = "VkBlitImageCubicWeightsInfoQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BlitImageCubicWeightsInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub cubic_weights: CubicFilterWeightsQCOM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceYcbcrDegammaFeaturesQCOM.html>
#[doc(alias = "VkPhysicalDeviceYcbcrDegammaFeaturesQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceYcbcrDegammaFeaturesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ycbcr_degamma: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM.html>
#[doc(alias = "VkSamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub enable_y_degamma: Bool32,
    pub enable_cb_cr_degamma: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCubicClampFeaturesQCOM.html>
#[doc(alias = "VkPhysicalDeviceCubicClampFeaturesQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCubicClampFeaturesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cubic_range_clamp: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub attachment_feedback_loop_dynamic_state: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkScreenBufferPropertiesQNX.html>
#[doc(alias = "VkScreenBufferPropertiesQNX")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ScreenBufferPropertiesQNX {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub allocation_size: DeviceSize,
    pub memory_type_bits: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkScreenBufferFormatPropertiesQNX.html>
#[doc(alias = "VkScreenBufferFormatPropertiesQNX")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ScreenBufferFormatPropertiesQNX {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format: Format,
    pub external_format: u64,
    pub screen_usage: u64,
    pub format_features: FormatFeatureFlags,
    pub sampler_ycbcr_conversion_components: ComponentMapping,
    pub suggested_ycbcr_model: SamplerYcbcrModelConversion,
    pub suggested_ycbcr_range: SamplerYcbcrRange,
    pub suggested_x_chroma_offset: ChromaLocation,
    pub suggested_y_chroma_offset: ChromaLocation,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImportScreenBufferInfoQNX.html>
#[doc(alias = "VkImportScreenBufferInfoQNX")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImportScreenBufferInfoQNX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: *mut _screen_buffer,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFormatQNX.html>
#[doc(alias = "VkExternalFormatQNX")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExternalFormatQNX {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_format: u64,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalMemoryScreenBufferFeaturesQNX.html>
#[doc(alias = "VkPhysicalDeviceExternalMemoryScreenBufferFeaturesQNX")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExternalMemoryScreenBufferFeaturesQNX {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub screen_buffer_import: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceLayeredDriverPropertiesMSFT.html>
#[doc(alias = "VkPhysicalDeviceLayeredDriverPropertiesMSFT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceLayeredDriverPropertiesMSFT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub underlying_api: LayeredDriverUnderlyingApiMSFT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDescriptorPoolOverallocationFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceDescriptorPoolOverallocationFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDescriptorPoolOverallocationFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub descriptor_pool_overallocation: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTileMemoryHeapFeaturesQCOM.html>
#[doc(alias = "VkPhysicalDeviceTileMemoryHeapFeaturesQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTileMemoryHeapFeaturesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tile_memory_heap: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTileMemoryHeapPropertiesQCOM.html>
#[doc(alias = "VkPhysicalDeviceTileMemoryHeapPropertiesQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTileMemoryHeapPropertiesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub queue_submit_boundary: Bool32,
    pub tile_buffer_transfers: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTileMemoryRequirementsQCOM.html>
#[doc(alias = "VkTileMemoryRequirementsQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TileMemoryRequirementsQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub size: DeviceSize,
    pub alignment: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTileMemoryBindInfoQCOM.html>
#[doc(alias = "VkTileMemoryBindInfoQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TileMemoryBindInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTileMemorySizeInfoQCOM.html>
#[doc(alias = "VkTileMemorySizeInfoQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TileMemorySizeInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub size: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDecompressMemoryRegionEXT.html>
#[doc(alias = "VkDecompressMemoryRegionEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DecompressMemoryRegionEXT {
    pub src_address: DeviceAddress,
    pub dst_address: DeviceAddress,
    pub compressed_size: DeviceSize,
    pub decompressed_size: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDecompressMemoryInfoEXT.html>
#[doc(alias = "VkDecompressMemoryInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DecompressMemoryInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub decompression_method: MemoryDecompressionMethodFlagsEXT,
    pub region_count: u32,
    pub p_regions: *const DecompressMemoryRegionEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplaySurfaceStereoCreateInfoNV.html>
#[doc(alias = "VkDisplaySurfaceStereoCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DisplaySurfaceStereoCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stereo_type: DisplaySurfaceStereoTypeNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayModeStereoPropertiesNV.html>
#[doc(alias = "VkDisplayModeStereoPropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DisplayModeStereoPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub hdmi_3_d_supported: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRawAccessChainsFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceRawAccessChainsFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRawAccessChainsFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_raw_access_chains: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalComputeQueueDeviceCreateInfoNV.html>
#[doc(alias = "VkExternalComputeQueueDeviceCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExternalComputeQueueDeviceCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub reserved_external_queues: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalComputeQueueCreateInfoNV.html>
#[doc(alias = "VkExternalComputeQueueCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExternalComputeQueueCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub preferred_queue: Queue,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalComputeQueueDataParamsNV.html>
#[doc(alias = "VkExternalComputeQueueDataParamsNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExternalComputeQueueDataParamsNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_index: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalComputeQueuePropertiesNV.html>
#[doc(alias = "VkPhysicalDeviceExternalComputeQueuePropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExternalComputeQueuePropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_data_size: u32,
    pub max_external_queues: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCommandBufferInheritanceFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceCommandBufferInheritanceFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCommandBufferInheritanceFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub command_buffer_inheritance: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderAtomicFloat16VectorFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceShaderAtomicFloat16VectorFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderAtomicFloat16VectorFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_float_16_vector_atomics: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderReplicatedCompositesFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceShaderReplicatedCompositesFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderReplicatedCompositesFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_replicated_composites: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorRollingBackingCreateInfoARM.html>
#[doc(alias = "VkTensorRollingBackingCreateInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TensorRollingBackingCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wraps: [u32; 4],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorExplicitTilingFormatPropertiesARM.html>
#[doc(alias = "VkTensorExplicitTilingFormatPropertiesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TensorExplicitTilingFormatPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub brick_16_tiling_tensor_features: FormatFeatureFlags2,
    pub brick_8_tiling_tensor_features: FormatFeatureFlags2,
    pub brick_4_tiling_tensor_features: FormatFeatureFlags2,
    pub block_u_tiling_tensor_features: FormatFeatureFlags2,
    pub block_u_64_k_tiling_tensor_features: FormatFeatureFlags2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderFloat8FeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceShaderFloat8FeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderFloat8FeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_float_8: Bool32,
    pub shader_float_8_cooperative_matrix: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRayTracingValidationFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceRayTracingValidationFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingValidationFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_tracing_validation: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceClusterAccelerationStructureFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceClusterAccelerationStructureFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceClusterAccelerationStructureFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cluster_acceleration_structure: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceClusterAccelerationStructurePropertiesNV.html>
#[doc(alias = "VkPhysicalDeviceClusterAccelerationStructurePropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceClusterAccelerationStructurePropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_vertices_per_cluster: u32,
    pub max_triangles_per_cluster: u32,
    pub cluster_scratch_byte_alignment: u32,
    pub cluster_byte_alignment: u32,
    pub cluster_template_byte_alignment: u32,
    pub cluster_bottom_level_byte_alignment: u32,
    pub cluster_template_bounds_byte_alignment: u32,
    pub max_cluster_geometry_index: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureClustersBottomLevelInputNV.html>
#[doc(alias = "VkClusterAccelerationStructureClustersBottomLevelInputNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ClusterAccelerationStructureClustersBottomLevelInputNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_total_cluster_count: u32,
    pub max_cluster_count_per_acceleration_structure: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureTriangleClusterInputNV.html>
#[doc(alias = "VkClusterAccelerationStructureTriangleClusterInputNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ClusterAccelerationStructureTriangleClusterInputNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub vertex_format: Format,
    pub max_geometry_index_value: u32,
    pub max_cluster_unique_geometry_count: u32,
    pub max_cluster_triangle_count: u32,
    pub max_cluster_vertex_count: u32,
    pub max_total_triangle_count: u32,
    pub max_total_vertex_count: u32,
    pub min_position_truncate_bit_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureMoveObjectsInputNV.html>
#[doc(alias = "VkClusterAccelerationStructureMoveObjectsInputNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ClusterAccelerationStructureMoveObjectsInputNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub type_: ClusterAccelerationStructureTypeNV,
    pub no_move_overlap: Bool32,
    pub max_moved_bytes: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureOpInputNV.html>
#[doc(alias = "VkClusterAccelerationStructureOpInputNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ClusterAccelerationStructureOpInputNV {
    pub p_clusters_bottom_level: *mut ClusterAccelerationStructureClustersBottomLevelInputNV,
    pub p_triangle_clusters: *mut ClusterAccelerationStructureTriangleClusterInputNV,
    pub p_move_objects: *mut ClusterAccelerationStructureMoveObjectsInputNV,
}
impl std::fmt::Debug for ClusterAccelerationStructureOpInputNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClusterAccelerationStructureOpInputNV {{ .. }}")
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureInputInfoNV.html>
#[doc(alias = "VkClusterAccelerationStructureInputInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ClusterAccelerationStructureInputInfoNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_acceleration_structure_count: u32,
    pub flags: BuildAccelerationStructureFlagsKHR,
    pub op_type: ClusterAccelerationStructureOpTypeNV,
    pub op_mode: ClusterAccelerationStructureOpModeNV,
    pub op_input: ClusterAccelerationStructureOpInputNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkStridedDeviceAddressRegionKHR.html>
#[doc(alias = "VkStridedDeviceAddressRegionKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct StridedDeviceAddressRegionKHR {
    pub device_address: DeviceAddress,
    pub stride: DeviceSize,
    pub size: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureCommandsInfoNV.html>
#[doc(alias = "VkClusterAccelerationStructureCommandsInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ClusterAccelerationStructureCommandsInfoNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub input: ClusterAccelerationStructureInputInfoNV,
    pub dst_implicit_data: DeviceAddress,
    pub scratch_data: DeviceAddress,
    pub dst_addresses_array: StridedDeviceAddressRegionKHR,
    pub dst_sizes_array: StridedDeviceAddressRegionKHR,
    pub src_infos_array: StridedDeviceAddressRegionKHR,
    pub src_infos_count: DeviceAddress,
    pub address_resolution_flags: ClusterAccelerationStructureAddressResolutionFlagsNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkStridedDeviceAddressNV.html>
#[doc(alias = "VkStridedDeviceAddressNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct StridedDeviceAddressNV {
    pub start_address: DeviceAddress,
    pub stride_in_bytes: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV.html>
#[doc(alias = "VkClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV {
    pub geometry_index: u32,
    pub reserved: u32,
    pub geometry_flags: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureMoveObjectsInfoNV.html>
#[doc(alias = "VkClusterAccelerationStructureMoveObjectsInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ClusterAccelerationStructureMoveObjectsInfoNV {
    pub src_acceleration_structure: DeviceAddress,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureBuildClustersBottomLevelInfoNV.html>
#[doc(alias = "VkClusterAccelerationStructureBuildClustersBottomLevelInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ClusterAccelerationStructureBuildClustersBottomLevelInfoNV {
    pub cluster_references_count: u32,
    pub cluster_references_stride: u32,
    pub cluster_references: DeviceAddress,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureBuildTriangleClusterInfoNV.html>
#[doc(alias = "VkClusterAccelerationStructureBuildTriangleClusterInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ClusterAccelerationStructureBuildTriangleClusterInfoNV {
    pub cluster_id: u32,
    pub cluster_flags: ClusterAccelerationStructureClusterFlagsNV,
    pub triangle_count: u32,
    pub vertex_count: u32,
    pub position_truncate_bit_count: u32,
    pub index_type: u32,
    pub opacity_micromap_index_type: u32,
    pub base_geometry_index_and_geometry_flags:
        ClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV,
    pub index_buffer_stride: u16,
    pub vertex_buffer_stride: u16,
    pub geometry_index_and_flags_buffer_stride: u16,
    pub opacity_micromap_index_buffer_stride: u16,
    pub index_buffer: DeviceAddress,
    pub vertex_buffer: DeviceAddress,
    pub geometry_index_and_flags_buffer: DeviceAddress,
    pub opacity_micromap_array: DeviceAddress,
    pub opacity_micromap_index_buffer: DeviceAddress,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureBuildTriangleClusterTemplateInfoNV.html>
#[doc(alias = "VkClusterAccelerationStructureBuildTriangleClusterTemplateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ClusterAccelerationStructureBuildTriangleClusterTemplateInfoNV {
    pub cluster_id: u32,
    pub cluster_flags: ClusterAccelerationStructureClusterFlagsNV,
    pub triangle_count: u32,
    pub vertex_count: u32,
    pub position_truncate_bit_count: u32,
    pub index_type: u32,
    pub opacity_micromap_index_type: u32,
    pub base_geometry_index_and_geometry_flags:
        ClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV,
    pub index_buffer_stride: u16,
    pub vertex_buffer_stride: u16,
    pub geometry_index_and_flags_buffer_stride: u16,
    pub opacity_micromap_index_buffer_stride: u16,
    pub index_buffer: DeviceAddress,
    pub vertex_buffer: DeviceAddress,
    pub geometry_index_and_flags_buffer: DeviceAddress,
    pub opacity_micromap_array: DeviceAddress,
    pub opacity_micromap_index_buffer: DeviceAddress,
    pub instantiation_bounding_box_limit: DeviceAddress,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureInstantiateClusterInfoNV.html>
#[doc(alias = "VkClusterAccelerationStructureInstantiateClusterInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ClusterAccelerationStructureInstantiateClusterInfoNV {
    pub cluster_id_offset: u32,
    pub geometry_index_offset: u32,
    pub reserved: u32,
    pub cluster_template_address: DeviceAddress,
    pub vertex_buffer: StridedDeviceAddressNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureGetTemplateIndicesInfoNV.html>
#[doc(alias = "VkClusterAccelerationStructureGetTemplateIndicesInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ClusterAccelerationStructureGetTemplateIndicesInfoNV {
    pub cluster_template_address: DeviceAddress,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureBuildSizesInfoKHR.html>
#[doc(alias = "VkAccelerationStructureBuildSizesInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureBuildSizesInfoKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub acceleration_structure_size: DeviceSize,
    pub update_scratch_size: DeviceSize,
    pub build_scratch_size: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRayTracingPipelineClusterAccelerationStructureCreateInfoNV.html>
#[doc(alias = "VkRayTracingPipelineClusterAccelerationStructureCreateInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RayTracingPipelineClusterAccelerationStructureCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub allow_cluster_acceleration_structure: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePartitionedAccelerationStructureFeaturesNV.html>
#[doc(alias = "VkPhysicalDevicePartitionedAccelerationStructureFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePartitionedAccelerationStructureFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub partitioned_acceleration_structure: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePartitionedAccelerationStructurePropertiesNV.html>
#[doc(alias = "VkPhysicalDevicePartitionedAccelerationStructurePropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePartitionedAccelerationStructurePropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_partition_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPartitionedAccelerationStructureFlagsNV.html>
#[doc(alias = "VkPartitionedAccelerationStructureFlagsNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PartitionedAccelerationStructureFlagsNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub enable_partition_translation: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildPartitionedAccelerationStructureIndirectCommandNV.html>
#[doc(alias = "VkBuildPartitionedAccelerationStructureIndirectCommandNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BuildPartitionedAccelerationStructureIndirectCommandNV {
    pub op_type: PartitionedAccelerationStructureOpTypeNV,
    pub arg_count: u32,
    pub arg_data: StridedDeviceAddressNV,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPartitionedAccelerationStructureWriteInstanceDataNV.html>
#[doc(alias = "VkPartitionedAccelerationStructureWriteInstanceDataNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PartitionedAccelerationStructureWriteInstanceDataNV {
    pub transform: TransformMatrixKHR,
    pub explicit_aabb: [f32; 6],
    pub instance_id: u32,
    pub instance_mask: u32,
    pub instance_contribution_to_hit_group_index: u32,
    pub instance_flags: PartitionedAccelerationStructureInstanceFlagsNV,
    pub instance_index: u32,
    pub partition_index: u32,
    pub acceleration_structure: DeviceAddress,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPartitionedAccelerationStructureUpdateInstanceDataNV.html>
#[doc(alias = "VkPartitionedAccelerationStructureUpdateInstanceDataNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PartitionedAccelerationStructureUpdateInstanceDataNV {
    pub instance_index: u32,
    pub instance_contribution_to_hit_group_index: u32,
    pub acceleration_structure: DeviceAddress,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPartitionedAccelerationStructureWritePartitionTranslationDataNV.html>
#[doc(alias = "VkPartitionedAccelerationStructureWritePartitionTranslationDataNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PartitionedAccelerationStructureWritePartitionTranslationDataNV {
    pub partition_index: u32,
    pub partition_translation: [f32; 3],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkWriteDescriptorSetPartitionedAccelerationStructureNV.html>
#[doc(alias = "VkWriteDescriptorSetPartitionedAccelerationStructureNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct WriteDescriptorSetPartitionedAccelerationStructureNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub acceleration_structure_count: u32,
    pub p_acceleration_structures: *const DeviceAddress,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPartitionedAccelerationStructureInstancesInputNV.html>
#[doc(alias = "VkPartitionedAccelerationStructureInstancesInputNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PartitionedAccelerationStructureInstancesInputNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: BuildAccelerationStructureFlagsKHR,
    pub instance_count: u32,
    pub max_instance_per_partition_count: u32,
    pub partition_count: u32,
    pub max_instance_in_global_partition_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildPartitionedAccelerationStructureInfoNV.html>
#[doc(alias = "VkBuildPartitionedAccelerationStructureInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BuildPartitionedAccelerationStructureInfoNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub input: PartitionedAccelerationStructureInstancesInputNV,
    pub src_acceleration_structure_data: DeviceAddress,
    pub dst_acceleration_structure_data: DeviceAddress,
    pub scratch_data: DeviceAddress,
    pub src_infos: DeviceAddress,
    pub src_infos_count: DeviceAddress,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDeviceGeneratedCommandsFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceDeviceGeneratedCommandsFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDeviceGeneratedCommandsFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_generated_commands: Bool32,
    pub dynamic_generated_pipeline_layout: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDeviceGeneratedCommandsPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceDeviceGeneratedCommandsPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDeviceGeneratedCommandsPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_indirect_pipeline_count: u32,
    pub max_indirect_shader_object_count: u32,
    pub max_indirect_sequence_count: u32,
    pub max_indirect_commands_token_count: u32,
    pub max_indirect_commands_token_offset: u32,
    pub max_indirect_commands_indirect_stride: u32,
    pub supported_indirect_commands_input_modes: IndirectCommandsInputModeFlagsEXT,
    pub supported_indirect_commands_shader_stages: ShaderStageFlags,
    pub supported_indirect_commands_shader_stages_pipeline_binding: ShaderStageFlags,
    pub supported_indirect_commands_shader_stages_shader_binding: ShaderStageFlags,
    pub device_generated_commands_transform_feedback: Bool32,
    pub device_generated_commands_multi_draw_indirect_count: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGeneratedCommandsMemoryRequirementsInfoEXT.html>
#[doc(alias = "VkGeneratedCommandsMemoryRequirementsInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GeneratedCommandsMemoryRequirementsInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub indirect_execution_set: IndirectExecutionSetEXT,
    pub indirect_commands_layout: IndirectCommandsLayoutEXT,
    pub max_sequence_count: u32,
    pub max_draw_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectExecutionSetPipelineInfoEXT.html>
#[doc(alias = "VkIndirectExecutionSetPipelineInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct IndirectExecutionSetPipelineInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub initial_pipeline: Pipeline,
    pub max_pipeline_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectExecutionSetShaderLayoutInfoEXT.html>
#[doc(alias = "VkIndirectExecutionSetShaderLayoutInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct IndirectExecutionSetShaderLayoutInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub set_layout_count: u32,
    pub p_set_layouts: *const DescriptorSetLayout,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectExecutionSetShaderInfoEXT.html>
#[doc(alias = "VkIndirectExecutionSetShaderInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct IndirectExecutionSetShaderInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shader_count: u32,
    pub p_initial_shaders: *const ShaderEXT,
    pub p_set_layout_infos: *const IndirectExecutionSetShaderLayoutInfoEXT,
    pub max_shader_count: u32,
    pub push_constant_range_count: u32,
    pub p_push_constant_ranges: *const PushConstantRange,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectExecutionSetInfoEXT.html>
#[doc(alias = "VkIndirectExecutionSetInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct IndirectExecutionSetInfoEXT {
    pub p_pipeline_info: *const IndirectExecutionSetPipelineInfoEXT,
    pub p_shader_info: *const IndirectExecutionSetShaderInfoEXT,
}
impl std::fmt::Debug for IndirectExecutionSetInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IndirectExecutionSetInfoEXT {{ .. }}")
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectExecutionSetCreateInfoEXT.html>
#[doc(alias = "VkIndirectExecutionSetCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct IndirectExecutionSetCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub type_: IndirectExecutionSetInfoTypeEXT,
    pub info: IndirectExecutionSetInfoEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGeneratedCommandsInfoEXT.html>
#[doc(alias = "VkGeneratedCommandsInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GeneratedCommandsInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shader_stages: ShaderStageFlags,
    pub indirect_execution_set: IndirectExecutionSetEXT,
    pub indirect_commands_layout: IndirectCommandsLayoutEXT,
    pub indirect_address: DeviceAddress,
    pub indirect_address_size: DeviceSize,
    pub preprocess_address: DeviceAddress,
    pub preprocess_size: DeviceSize,
    pub max_sequence_count: u32,
    pub sequence_count_address: DeviceAddress,
    pub max_draw_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkWriteIndirectExecutionSetPipelineEXT.html>
#[doc(alias = "VkWriteIndirectExecutionSetPipelineEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct WriteIndirectExecutionSetPipelineEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub index: u32,
    pub pipeline: Pipeline,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsPushConstantTokenEXT.html>
#[doc(alias = "VkIndirectCommandsPushConstantTokenEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct IndirectCommandsPushConstantTokenEXT {
    pub update_range: PushConstantRange,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsVertexBufferTokenEXT.html>
#[doc(alias = "VkIndirectCommandsVertexBufferTokenEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct IndirectCommandsVertexBufferTokenEXT {
    pub vertex_binding_unit: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsIndexBufferTokenEXT.html>
#[doc(alias = "VkIndirectCommandsIndexBufferTokenEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct IndirectCommandsIndexBufferTokenEXT {
    pub mode: IndirectCommandsInputModeFlagsEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsExecutionSetTokenEXT.html>
#[doc(alias = "VkIndirectCommandsExecutionSetTokenEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct IndirectCommandsExecutionSetTokenEXT {
    pub type_: IndirectExecutionSetInfoTypeEXT,
    pub shader_stages: ShaderStageFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsTokenDataEXT.html>
#[doc(alias = "VkIndirectCommandsTokenDataEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct IndirectCommandsTokenDataEXT {
    pub p_push_constant: *const IndirectCommandsPushConstantTokenEXT,
    pub p_vertex_buffer: *const IndirectCommandsVertexBufferTokenEXT,
    pub p_index_buffer: *const IndirectCommandsIndexBufferTokenEXT,
    pub p_execution_set: *const IndirectCommandsExecutionSetTokenEXT,
}
impl std::fmt::Debug for IndirectCommandsTokenDataEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IndirectCommandsTokenDataEXT {{ .. }}")
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutTokenEXT.html>
#[doc(alias = "VkIndirectCommandsLayoutTokenEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct IndirectCommandsLayoutTokenEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub type_: IndirectCommandsTokenTypeEXT,
    pub data: IndirectCommandsTokenDataEXT,
    pub offset: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutCreateInfoEXT.html>
#[doc(alias = "VkIndirectCommandsLayoutCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct IndirectCommandsLayoutCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: IndirectCommandsLayoutUsageFlagsEXT,
    pub shader_stages: ShaderStageFlags,
    pub indirect_stride: u32,
    pub pipeline_layout: PipelineLayout,
    pub token_count: u32,
    pub p_tokens: *const IndirectCommandsLayoutTokenEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDrawIndirectCountIndirectCommandEXT.html>
#[doc(alias = "VkDrawIndirectCountIndirectCommandEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DrawIndirectCountIndirectCommandEXT {
    pub buffer_address: DeviceAddress,
    pub stride: u32,
    pub command_count: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindVertexBufferIndirectCommandEXT.html>
#[doc(alias = "VkBindVertexBufferIndirectCommandEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BindVertexBufferIndirectCommandEXT {
    pub buffer_address: DeviceAddress,
    pub size: u32,
    pub stride: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindIndexBufferIndirectCommandEXT.html>
#[doc(alias = "VkBindIndexBufferIndirectCommandEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BindIndexBufferIndirectCommandEXT {
    pub buffer_address: DeviceAddress,
    pub size: u32,
    pub index_type: IndexType,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGeneratedCommandsPipelineInfoEXT.html>
#[doc(alias = "VkGeneratedCommandsPipelineInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GeneratedCommandsPipelineInfoEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline: Pipeline,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGeneratedCommandsShaderInfoEXT.html>
#[doc(alias = "VkGeneratedCommandsShaderInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GeneratedCommandsShaderInfoEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_count: u32,
    pub p_shaders: *const ShaderEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkWriteIndirectExecutionSetShaderEXT.html>
#[doc(alias = "VkWriteIndirectExecutionSetShaderEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct WriteIndirectExecutionSetShaderEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub index: u32,
    pub shader: ShaderEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageAlignmentControlFeaturesMESA.html>
#[doc(alias = "VkPhysicalDeviceImageAlignmentControlFeaturesMESA")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageAlignmentControlFeaturesMESA {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_alignment_control: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageAlignmentControlPropertiesMESA.html>
#[doc(alias = "VkPhysicalDeviceImageAlignmentControlPropertiesMESA")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageAlignmentControlPropertiesMESA {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supported_image_alignment_mask: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageAlignmentControlCreateInfoMESA.html>
#[doc(alias = "VkImageAlignmentControlCreateInfoMESA")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageAlignmentControlCreateInfoMESA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub maximum_requested_alignment: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPushConstantBankInfoNV.html>
#[doc(alias = "VkPushConstantBankInfoNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PushConstantBankInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub bank: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePushConstantBankFeaturesNV.html>
#[doc(alias = "VkPhysicalDevicePushConstantBankFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePushConstantBankFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub push_constant_bank: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePushConstantBankPropertiesNV.html>
#[doc(alias = "VkPhysicalDevicePushConstantBankPropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePushConstantBankPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_graphics_push_constant_banks: u32,
    pub max_compute_push_constant_banks: u32,
    pub max_graphics_push_data_banks: u32,
    pub max_compute_push_data_banks: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRayTracingInvocationReorderPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceRayTracingInvocationReorderPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingInvocationReorderPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_tracing_invocation_reorder_reordering_hint: RayTracingInvocationReorderModeEXT,
    pub max_shader_binding_table_record_index: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRayTracingInvocationReorderFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceRayTracingInvocationReorderFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingInvocationReorderFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_tracing_invocation_reorder: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDepthClampControlFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceDepthClampControlFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDepthClampControlFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub depth_clamp_control: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportDepthClampControlCreateInfoEXT.html>
#[doc(alias = "VkPipelineViewportDepthClampControlCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineViewportDepthClampControlCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub depth_clamp_mode: DepthClampModeEXT,
    pub p_depth_clamp_range: *const DepthClampRangeEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCreateInfoOHOS.html>
#[doc(alias = "VkSurfaceCreateInfoOHOS")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SurfaceCreateInfoOHOS {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SurfaceCreateFlagsOHOS,
    pub window: *mut OHNativeWindow,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceHdrVividFeaturesHUAWEI.html>
#[doc(alias = "VkPhysicalDeviceHdrVividFeaturesHUAWEI")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceHdrVividFeaturesHUAWEI {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub hdr_vivid: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkHdrVividDynamicMetadataHUAWEI.html>
#[doc(alias = "VkHdrVividDynamicMetadataHUAWEI")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct HdrVividDynamicMetadataHUAWEI {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dynamic_metadata_size: usize,
    pub p_dynamic_metadata: *const c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCooperativeMatrixFlexibleDimensionsPropertiesNV.html>
#[doc(alias = "VkCooperativeMatrixFlexibleDimensionsPropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CooperativeMatrixFlexibleDimensionsPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub m_granularity: u32,
    pub n_granularity: u32,
    pub k_granularity: u32,
    pub a_type: ComponentTypeKHR,
    pub b_type: ComponentTypeKHR,
    pub c_type: ComponentTypeKHR,
    pub result_type: ComponentTypeKHR,
    pub saturating_accumulation: Bool32,
    pub scope: ScopeKHR,
    pub workgroup_invocations: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCooperativeMatrix2FeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceCooperativeMatrix2FeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrix2FeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_matrix_workgroup_scope: Bool32,
    pub cooperative_matrix_flexible_dimensions: Bool32,
    pub cooperative_matrix_reductions: Bool32,
    pub cooperative_matrix_conversions: Bool32,
    pub cooperative_matrix_per_element_operations: Bool32,
    pub cooperative_matrix_tensor_addressing: Bool32,
    pub cooperative_matrix_block_loads: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCooperativeMatrix2PropertiesNV.html>
#[doc(alias = "VkPhysicalDeviceCooperativeMatrix2PropertiesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrix2PropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_matrix_workgroup_scope_max_workgroup_size: u32,
    pub cooperative_matrix_flexible_dimensions_max_dimension: u32,
    pub cooperative_matrix_workgroup_scope_reserved_shared_memory: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineOpacityMicromapFeaturesARM.html>
#[doc(alias = "VkPhysicalDevicePipelineOpacityMicromapFeaturesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePipelineOpacityMicromapFeaturesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_opacity_micromap: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImportMemoryMetalHandleInfoEXT.html>
#[doc(alias = "VkImportMemoryMetalHandleInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImportMemoryMetalHandleInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlags,
    pub handle: *mut c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryMetalHandlePropertiesEXT.html>
#[doc(alias = "VkMemoryMetalHandlePropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryMetalHandlePropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_type_bits: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryGetMetalHandleInfoEXT.html>
#[doc(alias = "VkMemoryGetMetalHandleInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryGetMetalHandleInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub handle_type: ExternalMemoryHandleTypeFlags,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePerformanceCountersByRegionFeaturesARM.html>
#[doc(alias = "VkPhysicalDevicePerformanceCountersByRegionFeaturesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePerformanceCountersByRegionFeaturesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub performance_counters_by_region: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePerformanceCountersByRegionPropertiesARM.html>
#[doc(alias = "VkPhysicalDevicePerformanceCountersByRegionPropertiesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePerformanceCountersByRegionPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_per_region_performance_counters: u32,
    pub performance_counter_region_size: Extent2D,
    pub row_stride_alignment: u32,
    pub region_alignment: u32,
    pub identity_transform_order: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterARM.html>
#[doc(alias = "VkPerformanceCounterARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PerformanceCounterARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub counter_id: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterDescriptionARM.html>
#[doc(alias = "VkPerformanceCounterDescriptionARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PerformanceCounterDescriptionARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: PerformanceCounterDescriptionFlagsARM,
    pub name: [c_char; 256],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassPerformanceCountersByRegionBeginInfoARM.html>
#[doc(alias = "VkRenderPassPerformanceCountersByRegionBeginInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderPassPerformanceCountersByRegionBeginInfoARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub counter_address_count: u32,
    pub p_counter_addresses: *const DeviceAddress,
    pub serialize_regions: Bool32,
    pub counter_index_count: u32,
    pub p_counter_indices: *mut u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderInstrumentationFeaturesARM.html>
#[doc(alias = "VkPhysicalDeviceShaderInstrumentationFeaturesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderInstrumentationFeaturesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_instrumentation: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderInstrumentationPropertiesARM.html>
#[doc(alias = "VkPhysicalDeviceShaderInstrumentationPropertiesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderInstrumentationPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub num_metrics: u32,
    pub per_basic_block_granularity: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderInstrumentationCreateInfoARM.html>
#[doc(alias = "VkShaderInstrumentationCreateInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ShaderInstrumentationCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderInstrumentationMetricDescriptionARM.html>
#[doc(alias = "VkShaderInstrumentationMetricDescriptionARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ShaderInstrumentationMetricDescriptionARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub name: [c_char; 256],
    pub description: [c_char; 256],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderInstrumentationMetricDataHeaderARM.html>
#[doc(alias = "VkShaderInstrumentationMetricDataHeaderARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ShaderInstrumentationMetricDataHeaderARM {
    pub result_index: u32,
    pub result_sub_index: u32,
    pub stages: ShaderStageFlags,
    pub basic_block_index: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVertexAttributeRobustnessFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceVertexAttributeRobustnessFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVertexAttributeRobustnessFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub vertex_attribute_robustness: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFormatPackFeaturesARM.html>
#[doc(alias = "VkPhysicalDeviceFormatPackFeaturesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFormatPackFeaturesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format_pack: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE.html>
#[doc(alias = "VkPhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub fragment_density_map_layered: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE.html>
#[doc(alias = "VkPhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_fragment_density_map_layers: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineFragmentDensityMapLayeredCreateInfoVALVE.html>
#[doc(alias = "VkPipelineFragmentDensityMapLayeredCreateInfoVALVE")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineFragmentDensityMapLayeredCreateInfoVALVE {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_fragment_density_map_layers: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSetPresentConfigNV.html>
#[doc(alias = "VkSetPresentConfigNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SetPresentConfigNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub num_frames_per_batch: u32,
    pub present_config_feedback: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePresentMeteringFeaturesNV.html>
#[doc(alias = "VkPhysicalDevicePresentMeteringFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePresentMeteringFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_metering: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMultisampledRenderToSwapchainFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceMultisampledRenderToSwapchainFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMultisampledRenderToSwapchainFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub multisampled_render_to_swapchain: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainFlagsSurfaceCapabilitiesEXT.html>
#[doc(alias = "VkSwapchainFlagsSurfaceCapabilitiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SwapchainFlagsSurfaceCapabilitiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub swapchain_supported_flags: SwapchainCreateFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceZeroInitializeDeviceMemoryFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceZeroInitializeDeviceMemoryFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceZeroInitializeDeviceMemoryFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub zero_initialize_device_memory: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShader64BitIndexingFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceShader64BitIndexingFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShader64BitIndexingFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_64_bit_indexing: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCustomResolveFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceCustomResolveFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCustomResolveFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub custom_resolve: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBeginCustomResolveInfoEXT.html>
#[doc(alias = "VkBeginCustomResolveInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BeginCustomResolveInfoEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCustomResolveCreateInfoEXT.html>
#[doc(alias = "VkCustomResolveCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CustomResolveCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub custom_resolve: Bool32,
    pub color_attachment_count: u32,
    pub p_color_attachment_formats: *const Format,
    pub depth_attachment_format: Format,
    pub stencil_attachment_format: Format,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCacheHeaderVersionDataGraphQCOM.html>
#[doc(alias = "VkPipelineCacheHeaderVersionDataGraphQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PipelineCacheHeaderVersionDataGraphQCOM {
    pub header_size: u32,
    pub header_version: PipelineCacheHeaderVersion,
    pub cache_type: DataGraphModelCacheTypeQCOM,
    pub cache_version: u32,
    pub toolchain_version: [u32; 3],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineBuiltinModelCreateInfoQCOM.html>
#[doc(alias = "VkDataGraphPipelineBuiltinModelCreateInfoQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphPipelineBuiltinModelCreateInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_operation: *const PhysicalDeviceDataGraphOperationSupportARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDataGraphModelFeaturesQCOM.html>
#[doc(alias = "VkPhysicalDeviceDataGraphModelFeaturesQCOM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDataGraphModelFeaturesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub data_graph_model: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDataGraphOpticalFlowFeaturesARM.html>
#[doc(alias = "VkPhysicalDeviceDataGraphOpticalFlowFeaturesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDataGraphOpticalFlowFeaturesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub data_graph_optical_flow: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyDataGraphOpticalFlowPropertiesARM.html>
#[doc(alias = "VkQueueFamilyDataGraphOpticalFlowPropertiesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QueueFamilyDataGraphOpticalFlowPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supported_output_grid_sizes: DataGraphOpticalFlowGridSizeFlagsARM,
    pub supported_hint_grid_sizes: DataGraphOpticalFlowGridSizeFlagsARM,
    pub hint_supported: Bool32,
    pub cost_supported: Bool32,
    pub min_width: u32,
    pub min_height: u32,
    pub max_width: u32,
    pub max_height: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineOpticalFlowCreateInfoARM.html>
#[doc(alias = "VkDataGraphPipelineOpticalFlowCreateInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphPipelineOpticalFlowCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub width: u32,
    pub height: u32,
    pub image_format: Format,
    pub flow_vector_format: Format,
    pub cost_format: Format,
    pub output_grid_size: DataGraphOpticalFlowGridSizeFlagsARM,
    pub hint_grid_size: DataGraphOpticalFlowGridSizeFlagsARM,
    pub performance_level: DataGraphOpticalFlowPerformanceLevelARM,
    pub flags: DataGraphOpticalFlowCreateFlagsARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowImageFormatPropertiesARM.html>
#[doc(alias = "VkDataGraphOpticalFlowImageFormatPropertiesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphOpticalFlowImageFormatPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format: Format,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowImageFormatInfoARM.html>
#[doc(alias = "VkDataGraphOpticalFlowImageFormatInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphOpticalFlowImageFormatInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub usage: DataGraphOpticalFlowImageUsageFlagsARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineOpticalFlowDispatchInfoARM.html>
#[doc(alias = "VkDataGraphPipelineOpticalFlowDispatchInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphPipelineOpticalFlowDispatchInfoARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: DataGraphOpticalFlowExecuteFlagsARM,
    pub mean_flow_l_1_norm_hint: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineResourceInfoImageLayoutARM.html>
#[doc(alias = "VkDataGraphPipelineResourceInfoImageLayoutARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphPipelineResourceInfoImageLayoutARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub layout: ImageLayout,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSingleNodeConnectionARM.html>
#[doc(alias = "VkDataGraphPipelineSingleNodeConnectionARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphPipelineSingleNodeConnectionARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub set: u32,
    pub binding: u32,
    pub connection: DataGraphPipelineNodeConnectionTypeARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSingleNodeCreateInfoARM.html>
#[doc(alias = "VkDataGraphPipelineSingleNodeCreateInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphPipelineSingleNodeCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub node_type: DataGraphPipelineNodeTypeARM,
    pub connection_count: u32,
    pub p_connections: *const DataGraphPipelineSingleNodeConnectionARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderLongVectorFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceShaderLongVectorFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderLongVectorFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub long_vector: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderLongVectorPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceShaderLongVectorPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderLongVectorPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_vector_components: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineCacheIncrementalModeFeaturesSEC.html>
#[doc(alias = "VkPhysicalDevicePipelineCacheIncrementalModeFeaturesSEC")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePipelineCacheIncrementalModeFeaturesSEC {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_cache_incremental_mode: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_uniform_buffer_unsized_array: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkComputeOccupancyPriorityParametersNV.html>
#[doc(alias = "VkComputeOccupancyPriorityParametersNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ComputeOccupancyPriorityParametersNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub occupancy_priority: f32,
    pub occupancy_throttling: f32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceComputeOccupancyPriorityFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceComputeOccupancyPriorityFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceComputeOccupancyPriorityFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub compute_occupancy_priority: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCooperativeMatrixProperties2EXT.html>
#[doc(alias = "VkCooperativeMatrixProperties2EXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CooperativeMatrixProperties2EXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub m_granularity: u32,
    pub n_granularity: u32,
    pub k_granularity: u32,
    pub a_type: ComponentTypeKHR,
    pub b_type: ComponentTypeKHR,
    pub c_type: ComponentTypeKHR,
    pub result_type: ComponentTypeKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCooperativeMatrixInfo2EXT.html>
#[doc(alias = "VkPhysicalDeviceCooperativeMatrixInfo2EXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixInfo2EXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub scope: ScopeKHR,
    pub invocations: u32,
    pub subgroup_size: u32,
    pub flags: CooperativeMatrixFlagsEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCooperativeMatrixMaintenance1FeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceCooperativeMatrixMaintenance1FeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixMaintenance1FeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_matrix_properties_2: Bool32,
    pub cooperative_matrix_reductions: Bool32,
    pub cooperative_matrix_conversions: Bool32,
    pub cooperative_matrix_per_element_operations: Bool32,
    pub cooperative_matrix_get_coordinate: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderSubgroupPartitionedFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceShaderSubgroupPartitionedFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderSubgroupPartitionedFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_subgroup_partitioned: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkUbmSurfaceCreateInfoSEC.html>
#[doc(alias = "VkUbmSurfaceCreateInfoSEC")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct UbmSurfaceCreateInfoSEC {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: UbmSurfaceCreateFlagsSEC,
    pub device: *mut ubm_device,
    pub surface: *mut ubm_surface,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_float_4: Bool32,
    pub shader_float_6: Bool32,
    pub shader_float_8_unsigned_e_8_m_0: Bool32,
    pub shader_mx_int_8: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE.html>
#[doc(alias = "VkPhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_mixed_float_dot_product_float_16_acc_float_32: Bool32,
    pub shader_mixed_float_dot_product_float_16_acc_float_16: Bool32,
    pub shader_mixed_float_dot_product_b_float_16_acc: Bool32,
    pub shader_mixed_float_dot_product_float_8_acc_float_32: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkThrottleHintSubmitInfoSEC.html>
#[doc(alias = "VkThrottleHintSubmitInfoSEC")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ThrottleHintSubmitInfoSEC {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub throttle_hint: ThrottleHintTypeSEC,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceThrottleHintFeaturesSEC.html>
#[doc(alias = "VkPhysicalDeviceThrottleHintFeaturesSEC")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceThrottleHintFeaturesSEC {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub throttle_hint: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM.html>
#[doc(alias = "VkPhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub data_graph_neural_accelerator_statistics: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineNeuralStatisticsCreateInfoARM.html>
#[doc(alias = "VkDataGraphPipelineNeuralStatisticsCreateInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphPipelineNeuralStatisticsCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub allow_neural_statistics: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionNeuralStatisticsCreateInfoARM.html>
#[doc(alias = "VkDataGraphPipelineSessionNeuralStatisticsCreateInfoARM")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataGraphPipelineSessionNeuralStatisticsCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mode: NeuralAcceleratorStatisticsModeARM,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePrimitiveRestartIndexFeaturesEXT.html>
#[doc(alias = "VkPhysicalDevicePrimitiveRestartIndexFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePrimitiveRestartIndexFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub primitive_restart_index: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageTilingControlFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceImageTilingControlFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageTilingControlFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_tiling_control: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageTilingControlCreateInfoEXT.html>
#[doc(alias = "VkImageTilingControlCreateInfoEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ImageTilingControlCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tiling_control: ImageTilingControlEXT,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV.html>
#[doc(alias = "VkPhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_matrix_decode_vector: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePrivateDataBaseHandleFeaturesNV.html>
#[doc(alias = "VkPhysicalDevicePrivateDataBaseHandleFeaturesNV")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePrivateDataBaseHandleFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub private_data_base_handle: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceBufferDeviceAddressAllocationAlignmentFeaturesVALVE.html>
#[doc(alias = "VkPhysicalDeviceBufferDeviceAddressAllocationAlignmentFeaturesVALVE")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceBufferDeviceAddressAllocationAlignmentFeaturesVALVE {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub buffer_device_address_allocation_alignment: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceBufferDeviceAddressAllocationAlignmentPropertiesVALVE.html>
#[doc(alias = "VkPhysicalDeviceBufferDeviceAddressAllocationAlignmentPropertiesVALVE")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceBufferDeviceAddressAllocationAlignmentPropertiesVALVE {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_buffer_device_address_allocation_alignment: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferDeviceAddressAlignmentAllocateInfoVALVE.html>
#[doc(alias = "VkBufferDeviceAddressAlignmentAllocateInfoVALVE")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BufferDeviceAddressAlignmentAllocateInfoVALVE {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub alignment: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureBuildRangeInfoKHR.html>
#[doc(alias = "VkAccelerationStructureBuildRangeInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureBuildRangeInfoKHR {
    pub primitive_count: u32,
    pub primitive_offset: u32,
    pub first_vertex: u32,
    pub transform_offset: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureGeometryTrianglesDataKHR.html>
#[doc(alias = "VkAccelerationStructureGeometryTrianglesDataKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureGeometryTrianglesDataKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vertex_format: Format,
    pub vertex_data: DeviceOrHostAddressConstKHR,
    pub vertex_stride: DeviceSize,
    pub max_vertex: u32,
    pub index_type: IndexType,
    pub index_data: DeviceOrHostAddressConstKHR,
    pub transform_data: DeviceOrHostAddressConstKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureGeometryAabbsDataKHR.html>
#[doc(alias = "VkAccelerationStructureGeometryAabbsDataKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureGeometryAabbsDataKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub data: DeviceOrHostAddressConstKHR,
    pub stride: DeviceSize,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureGeometryInstancesDataKHR.html>
#[doc(alias = "VkAccelerationStructureGeometryInstancesDataKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureGeometryInstancesDataKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub array_of_pointers: Bool32,
    pub data: DeviceOrHostAddressConstKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureGeometryDataKHR.html>
#[doc(alias = "VkAccelerationStructureGeometryDataKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureGeometryDataKHR {
    pub triangles: AccelerationStructureGeometryTrianglesDataKHR,
    pub aabbs: AccelerationStructureGeometryAabbsDataKHR,
    pub instances: AccelerationStructureGeometryInstancesDataKHR,
}
impl std::fmt::Debug for AccelerationStructureGeometryDataKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AccelerationStructureGeometryDataKHR {{ .. }}")
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureGeometryKHR.html>
#[doc(alias = "VkAccelerationStructureGeometryKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureGeometryKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub geometry_type: GeometryTypeKHR,
    pub geometry: AccelerationStructureGeometryDataKHR,
    pub flags: GeometryFlagsKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureBuildGeometryInfoKHR.html>
#[doc(alias = "VkAccelerationStructureBuildGeometryInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureBuildGeometryInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub type_: AccelerationStructureTypeKHR,
    pub flags: BuildAccelerationStructureFlagsKHR,
    pub mode: BuildAccelerationStructureModeKHR,
    pub src_acceleration_structure: AccelerationStructureKHR,
    pub dst_acceleration_structure: AccelerationStructureKHR,
    pub geometry_count: u32,
    pub p_geometries: *const AccelerationStructureGeometryKHR,
    pub pp_geometries: *const *const AccelerationStructureGeometryKHR,
    pub scratch_data: DeviceOrHostAddressKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureCreateInfoKHR.html>
#[doc(alias = "VkAccelerationStructureCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub create_flags: AccelerationStructureCreateFlagsKHR,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub size: DeviceSize,
    pub type_: AccelerationStructureTypeKHR,
    pub device_address: DeviceAddress,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkWriteDescriptorSetAccelerationStructureKHR.html>
#[doc(alias = "VkWriteDescriptorSetAccelerationStructureKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct WriteDescriptorSetAccelerationStructureKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acceleration_structure_count: u32,
    pub p_acceleration_structures: *const AccelerationStructureKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceAccelerationStructureFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceAccelerationStructureFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceAccelerationStructureFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub acceleration_structure: Bool32,
    pub acceleration_structure_capture_replay: Bool32,
    pub acceleration_structure_indirect_build: Bool32,
    pub acceleration_structure_host_commands: Bool32,
    pub descriptor_binding_acceleration_structure_update_after_bind: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceAccelerationStructurePropertiesKHR.html>
#[doc(alias = "VkPhysicalDeviceAccelerationStructurePropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceAccelerationStructurePropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_geometry_count: u64,
    pub max_instance_count: u64,
    pub max_primitive_count: u64,
    pub max_per_stage_descriptor_acceleration_structures: u32,
    pub max_per_stage_descriptor_update_after_bind_acceleration_structures: u32,
    pub max_descriptor_set_acceleration_structures: u32,
    pub max_descriptor_set_update_after_bind_acceleration_structures: u32,
    pub min_acceleration_structure_scratch_offset_alignment: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureDeviceAddressInfoKHR.html>
#[doc(alias = "VkAccelerationStructureDeviceAddressInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureDeviceAddressInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acceleration_structure: AccelerationStructureKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureVersionInfoKHR.html>
#[doc(alias = "VkAccelerationStructureVersionInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureVersionInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_version_data: *const u8,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyAccelerationStructureToMemoryInfoKHR.html>
#[doc(alias = "VkCopyAccelerationStructureToMemoryInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CopyAccelerationStructureToMemoryInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src: AccelerationStructureKHR,
    pub dst: DeviceOrHostAddressKHR,
    pub mode: CopyAccelerationStructureModeKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyMemoryToAccelerationStructureInfoKHR.html>
#[doc(alias = "VkCopyMemoryToAccelerationStructureInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CopyMemoryToAccelerationStructureInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src: DeviceOrHostAddressConstKHR,
    pub dst: AccelerationStructureKHR,
    pub mode: CopyAccelerationStructureModeKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyAccelerationStructureInfoKHR.html>
#[doc(alias = "VkCopyAccelerationStructureInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CopyAccelerationStructureInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src: AccelerationStructureKHR,
    pub dst: AccelerationStructureKHR,
    pub mode: CopyAccelerationStructureModeKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRayTracingShaderGroupCreateInfoKHR.html>
#[doc(alias = "VkRayTracingShaderGroupCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RayTracingShaderGroupCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub type_: RayTracingShaderGroupTypeKHR,
    pub general_shader: u32,
    pub closest_hit_shader: u32,
    pub any_hit_shader: u32,
    pub intersection_shader: u32,
    pub p_shader_group_capture_replay_handle: *const c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRayTracingPipelineInterfaceCreateInfoKHR.html>
#[doc(alias = "VkRayTracingPipelineInterfaceCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RayTracingPipelineInterfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_pipeline_ray_payload_size: u32,
    pub max_pipeline_ray_hit_attribute_size: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRayTracingPipelineCreateInfoKHR.html>
#[doc(alias = "VkRayTracingPipelineCreateInfoKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RayTracingPipelineCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage_count: u32,
    pub p_stages: *const PipelineShaderStageCreateInfo,
    pub group_count: u32,
    pub p_groups: *const RayTracingShaderGroupCreateInfoKHR,
    pub max_pipeline_ray_recursion_depth: u32,
    pub p_library_info: *const PipelineLibraryCreateInfoKHR,
    pub p_library_interface: *const RayTracingPipelineInterfaceCreateInfoKHR,
    pub p_dynamic_state: *const PipelineDynamicStateCreateInfo,
    pub layout: PipelineLayout,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRayTracingPipelineFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceRayTracingPipelineFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingPipelineFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_tracing_pipeline: Bool32,
    pub ray_tracing_pipeline_shader_group_handle_capture_replay: Bool32,
    pub ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: Bool32,
    pub ray_tracing_pipeline_trace_rays_indirect: Bool32,
    pub ray_traversal_primitive_culling: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRayTracingPipelinePropertiesKHR.html>
#[doc(alias = "VkPhysicalDeviceRayTracingPipelinePropertiesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingPipelinePropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_group_handle_size: u32,
    pub max_ray_recursion_depth: u32,
    pub max_shader_group_stride: u32,
    pub shader_group_base_alignment: u32,
    pub shader_group_handle_capture_replay_size: u32,
    pub max_ray_dispatch_invocation_count: u32,
    pub shader_group_handle_alignment: u32,
    pub max_ray_hit_attribute_size: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTraceRaysIndirectCommandKHR.html>
#[doc(alias = "VkTraceRaysIndirectCommandKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TraceRaysIndirectCommandKHR {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRayQueryFeaturesKHR.html>
#[doc(alias = "VkPhysicalDeviceRayQueryFeaturesKHR")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRayQueryFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_query: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMeshShaderFeaturesEXT.html>
#[doc(alias = "VkPhysicalDeviceMeshShaderFeaturesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMeshShaderFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub task_shader: Bool32,
    pub mesh_shader: Bool32,
    pub multiview_mesh_shader: Bool32,
    pub primitive_fragment_shading_rate_mesh_shader: Bool32,
    pub mesh_shader_queries: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMeshShaderPropertiesEXT.html>
#[doc(alias = "VkPhysicalDeviceMeshShaderPropertiesEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMeshShaderPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_task_work_group_total_count: u32,
    pub max_task_work_group_count: [u32; 3],
    pub max_task_work_group_invocations: u32,
    pub max_task_work_group_size: [u32; 3],
    pub max_task_payload_size: u32,
    pub max_task_shared_memory_size: u32,
    pub max_task_payload_and_shared_memory_size: u32,
    pub max_mesh_work_group_total_count: u32,
    pub max_mesh_work_group_count: [u32; 3],
    pub max_mesh_work_group_invocations: u32,
    pub max_mesh_work_group_size: [u32; 3],
    pub max_mesh_shared_memory_size: u32,
    pub max_mesh_payload_and_shared_memory_size: u32,
    pub max_mesh_output_memory_size: u32,
    pub max_mesh_payload_and_output_memory_size: u32,
    pub max_mesh_output_components: u32,
    pub max_mesh_output_vertices: u32,
    pub max_mesh_output_primitives: u32,
    pub max_mesh_output_layers: u32,
    pub max_mesh_multiview_view_count: u32,
    pub mesh_output_per_vertex_granularity: u32,
    pub mesh_output_per_primitive_granularity: u32,
    pub max_preferred_task_work_group_invocations: u32,
    pub max_preferred_mesh_work_group_invocations: u32,
    pub prefers_local_invocation_vertex_output: Bool32,
    pub prefers_local_invocation_primitive_output: Bool32,
    pub prefers_compact_vertex_output: Bool32,
    pub prefers_compact_primitive_output: Bool32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDrawMeshTasksIndirectCommandEXT.html>
#[doc(alias = "VkDrawMeshTasksIndirectCommandEXT")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DrawMeshTasksIndirectCommandEXT {
    pub group_count_x: u32,
    pub group_count_y: u32,
    pub group_count_z: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH264SpsVuiFlags.html>
#[doc(alias = "StdVideoH264SpsVuiFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H264SpsVuiFlags {
    pub aspect_ratio_info_present_flag: u32,
    pub overscan_info_present_flag: u32,
    pub overscan_appropriate_flag: u32,
    pub video_signal_type_present_flag: u32,
    pub video_full_range_flag: u32,
    pub color_description_present_flag: u32,
    pub chroma_loc_info_present_flag: u32,
    pub timing_info_present_flag: u32,
    pub fixed_frame_rate_flag: u32,
    pub bitstream_restriction_flag: u32,
    pub nal_hrd_parameters_present_flag: u32,
    pub vcl_hrd_parameters_present_flag: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH264HrdParameters.html>
#[doc(alias = "StdVideoH264HrdParameters")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H264HrdParameters {
    pub cpb_cnt_minus_1: u8,
    pub bit_rate_scale: u8,
    pub cpb_size_scale: u8,
    pub reserved_1: u8,
    pub bit_rate_value_minus_1: [u32; 32],
    pub cpb_size_value_minus_1: [u32; 32],
    pub cbr_flag: [u8; 32],
    pub initial_cpb_removal_delay_length_minus_1: u32,
    pub cpb_removal_delay_length_minus_1: u32,
    pub dpb_output_delay_length_minus_1: u32,
    pub time_offset_length: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH264SequenceParameterSetVui.html>
#[doc(alias = "StdVideoH264SequenceParameterSetVui")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H264SequenceParameterSetVui {
    pub flags: H264SpsVuiFlags,
    pub aspect_ratio_idc: H264AspectRatioIdc,
    pub sar_width: u16,
    pub sar_height: u16,
    pub video_format: u8,
    pub colour_primaries: u8,
    pub transfer_characteristics: u8,
    pub matrix_coefficients: u8,
    pub num_units_in_tick: u32,
    pub time_scale: u32,
    pub max_num_reorder_frames: u8,
    pub max_dec_frame_buffering: u8,
    pub chroma_sample_loc_type_top_field: u8,
    pub chroma_sample_loc_type_bottom_field: u8,
    pub reserved_1: u32,
    pub p_hrd_parameters: *const H264HrdParameters,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH264SpsFlags.html>
#[doc(alias = "StdVideoH264SpsFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H264SpsFlags {
    pub constraint_set_0_flag: u32,
    pub constraint_set_1_flag: u32,
    pub constraint_set_2_flag: u32,
    pub constraint_set_3_flag: u32,
    pub constraint_set_4_flag: u32,
    pub constraint_set_5_flag: u32,
    pub direct_8_x_8_inference_flag: u32,
    pub mb_adaptive_frame_field_flag: u32,
    pub frame_mbs_only_flag: u32,
    pub delta_pic_order_always_zero_flag: u32,
    pub separate_colour_plane_flag: u32,
    pub gaps_in_frame_num_value_allowed_flag: u32,
    pub qpprime_y_zero_transform_bypass_flag: u32,
    pub frame_cropping_flag: u32,
    pub seq_scaling_matrix_present_flag: u32,
    pub vui_parameters_present_flag: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH264ScalingLists.html>
#[doc(alias = "StdVideoH264ScalingLists")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H264ScalingLists {
    pub scaling_list_present_mask: u16,
    pub use_default_scaling_matrix_mask: u16,
    pub scaling_list_4_x_4: [[u8; 6]; 16],
    pub scaling_list_8_x_8: [[u8; 6]; 64],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH264SequenceParameterSet.html>
#[doc(alias = "StdVideoH264SequenceParameterSet")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H264SequenceParameterSet {
    pub flags: H264SpsFlags,
    pub profile_idc: H264ProfileIdc,
    pub level_idc: H264LevelIdc,
    pub chroma_format_idc: H264ChromaFormatIdc,
    pub seq_parameter_set_id: u8,
    pub bit_depth_luma_minus_8: u8,
    pub bit_depth_chroma_minus_8: u8,
    pub log_2_max_frame_num_minus_4: u8,
    pub pic_order_cnt_type: H264PocType,
    pub offset_for_non_ref_pic: i32,
    pub offset_for_top_to_bottom_field: i32,
    pub log_2_max_pic_order_cnt_lsb_minus_4: u8,
    pub num_ref_frames_in_pic_order_cnt_cycle: u8,
    pub max_num_ref_frames: u8,
    pub reserved_1: u8,
    pub pic_width_in_mbs_minus_1: u32,
    pub pic_height_in_map_units_minus_1: u32,
    pub frame_crop_left_offset: u32,
    pub frame_crop_right_offset: u32,
    pub frame_crop_top_offset: u32,
    pub frame_crop_bottom_offset: u32,
    pub reserved_2: u32,
    pub p_offset_for_ref_frame: *const i32,
    pub p_scaling_lists: *const H264ScalingLists,
    pub p_sequence_parameter_set_vui: *const H264SequenceParameterSetVui,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH264PpsFlags.html>
#[doc(alias = "StdVideoH264PpsFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H264PpsFlags {
    pub transform_8_x_8_mode_flag: u32,
    pub redundant_pic_cnt_present_flag: u32,
    pub constrained_intra_pred_flag: u32,
    pub deblocking_filter_control_present_flag: u32,
    pub weighted_pred_flag: u32,
    pub bottom_field_pic_order_in_frame_present_flag: u32,
    pub entropy_coding_mode_flag: u32,
    pub pic_scaling_matrix_present_flag: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH264PictureParameterSet.html>
#[doc(alias = "StdVideoH264PictureParameterSet")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H264PictureParameterSet {
    pub flags: H264PpsFlags,
    pub seq_parameter_set_id: u8,
    pub pic_parameter_set_id: u8,
    pub num_ref_idx_l_0_default_active_minus_1: u8,
    pub num_ref_idx_l_1_default_active_minus_1: u8,
    pub weighted_bipred_idc: H264WeightedBipredIdc,
    pub pic_init_qp_minus_26: i8,
    pub pic_init_qs_minus_26: i8,
    pub chroma_qp_index_offset: i8,
    pub second_chroma_qp_index_offset: i8,
    pub p_scaling_lists: *const H264ScalingLists,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoDecodeH264PictureInfoFlags.html>
#[doc(alias = "StdVideoDecodeH264PictureInfoFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DecodeH264PictureInfoFlags {
    pub field_pic_flag: u32,
    pub is_intra: u32,
    pub idr_pic_flag: u32,
    pub bottom_field_flag: u32,
    pub is_reference: u32,
    pub complementary_field_pair: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoDecodeH264PictureInfo.html>
#[doc(alias = "StdVideoDecodeH264PictureInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DecodeH264PictureInfo {
    pub flags: DecodeH264PictureInfoFlags,
    pub seq_parameter_set_id: u8,
    pub pic_parameter_set_id: u8,
    pub reserved_1: u8,
    pub reserved_2: u8,
    pub frame_num: u16,
    pub idr_pic_id: u16,
    pub pic_order_cnt: [i32; 2],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoDecodeH264ReferenceInfoFlags.html>
#[doc(alias = "StdVideoDecodeH264ReferenceInfoFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DecodeH264ReferenceInfoFlags {
    pub top_field_flag: u32,
    pub bottom_field_flag: u32,
    pub used_for_long_term_reference: u32,
    pub is_non_existing: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoDecodeH264ReferenceInfo.html>
#[doc(alias = "StdVideoDecodeH264ReferenceInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DecodeH264ReferenceInfo {
    pub flags: DecodeH264ReferenceInfoFlags,
    pub frame_num: u16,
    pub reserved: u16,
    pub pic_order_cnt: [i32; 2],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeH264WeightTableFlags.html>
#[doc(alias = "StdVideoEncodeH264WeightTableFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeH264WeightTableFlags {
    pub luma_weight_l_0_flag: u32,
    pub chroma_weight_l_0_flag: u32,
    pub luma_weight_l_1_flag: u32,
    pub chroma_weight_l_1_flag: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeH264WeightTable.html>
#[doc(alias = "StdVideoEncodeH264WeightTable")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeH264WeightTable {
    pub flags: EncodeH264WeightTableFlags,
    pub luma_log_2_weight_denom: u8,
    pub chroma_log_2_weight_denom: u8,
    pub luma_weight_l_0: [i8; 32],
    pub luma_offset_l_0: [i8; 32],
    pub chroma_weight_l_0: [[i8; 32]; 2],
    pub chroma_offset_l_0: [[i8; 32]; 2],
    pub luma_weight_l_1: [i8; 32],
    pub luma_offset_l_1: [i8; 32],
    pub chroma_weight_l_1: [[i8; 32]; 2],
    pub chroma_offset_l_1: [[i8; 32]; 2],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeH264SliceHeaderFlags.html>
#[doc(alias = "StdVideoEncodeH264SliceHeaderFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeH264SliceHeaderFlags {
    pub direct_spatial_mv_pred_flag: u32,
    pub num_ref_idx_active_override_flag: u32,
    pub reserved: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeH264PictureInfoFlags.html>
#[doc(alias = "StdVideoEncodeH264PictureInfoFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeH264PictureInfoFlags {
    pub idr_pic_flag: u32,
    pub is_reference: u32,
    pub no_output_of_prior_pics_flag: u32,
    pub long_term_reference_flag: u32,
    pub adaptive_ref_pic_marking_mode_flag: u32,
    pub reserved: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeH264ReferenceInfoFlags.html>
#[doc(alias = "StdVideoEncodeH264ReferenceInfoFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeH264ReferenceInfoFlags {
    pub used_for_long_term_reference: u32,
    pub reserved: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeH264ReferenceListsInfoFlags.html>
#[doc(alias = "StdVideoEncodeH264ReferenceListsInfoFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeH264ReferenceListsInfoFlags {
    pub ref_pic_list_modification_flag_l_0: u32,
    pub ref_pic_list_modification_flag_l_1: u32,
    pub reserved: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeH264RefListModEntry.html>
#[doc(alias = "StdVideoEncodeH264RefListModEntry")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeH264RefListModEntry {
    pub modification_of_pic_nums_idc: H264ModificationOfPicNumsIdc,
    pub abs_diff_pic_num_minus_1: u16,
    pub long_term_pic_num: u16,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeH264RefPicMarkingEntry.html>
#[doc(alias = "StdVideoEncodeH264RefPicMarkingEntry")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeH264RefPicMarkingEntry {
    pub memory_management_control_operation: H264MemMgmtControlOp,
    pub difference_of_pic_nums_minus_1: u16,
    pub long_term_pic_num: u16,
    pub long_term_frame_idx: u16,
    pub max_long_term_frame_idx_plus_1: u16,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeH264ReferenceListsInfo.html>
#[doc(alias = "StdVideoEncodeH264ReferenceListsInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeH264ReferenceListsInfo {
    pub flags: EncodeH264ReferenceListsInfoFlags,
    pub num_ref_idx_l_0_active_minus_1: u8,
    pub num_ref_idx_l_1_active_minus_1: u8,
    pub ref_pic_list_0: [u8; 32],
    pub ref_pic_list_1: [u8; 32],
    pub ref_list_0_mod_op_count: u8,
    pub ref_list_1_mod_op_count: u8,
    pub ref_pic_marking_op_count: u8,
    pub reserved_1: [u8; 7],
    pub p_ref_list_0_mod_operations: *const EncodeH264RefListModEntry,
    pub p_ref_list_1_mod_operations: *const EncodeH264RefListModEntry,
    pub p_ref_pic_marking_operations: *const EncodeH264RefPicMarkingEntry,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeH264PictureInfo.html>
#[doc(alias = "StdVideoEncodeH264PictureInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeH264PictureInfo {
    pub flags: EncodeH264PictureInfoFlags,
    pub seq_parameter_set_id: u8,
    pub pic_parameter_set_id: u8,
    pub idr_pic_id: u16,
    pub primary_pic_type: H264PictureType,
    pub frame_num: u32,
    pub pic_order_cnt: i32,
    pub temporal_id: u8,
    pub reserved_1: [u8; 3],
    pub p_ref_lists: *const EncodeH264ReferenceListsInfo,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeH264ReferenceInfo.html>
#[doc(alias = "StdVideoEncodeH264ReferenceInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeH264ReferenceInfo {
    pub flags: EncodeH264ReferenceInfoFlags,
    pub primary_pic_type: H264PictureType,
    pub frame_num: u32,
    pub pic_order_cnt: i32,
    pub long_term_pic_num: u16,
    pub long_term_frame_idx: u16,
    pub temporal_id: u8,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeH264SliceHeader.html>
#[doc(alias = "StdVideoEncodeH264SliceHeader")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeH264SliceHeader {
    pub flags: EncodeH264SliceHeaderFlags,
    pub first_mb_in_slice: u32,
    pub slice_type: H264SliceType,
    pub slice_alpha_c_0_offset_div_2: i8,
    pub slice_beta_offset_div_2: i8,
    pub slice_qp_delta: i8,
    pub reserved_1: u8,
    pub cabac_init_idc: H264CabacInitIdc,
    pub disable_deblocking_filter_idc: H264DisableDeblockingFilterIdc,
    pub p_weight_table: *const EncodeH264WeightTable,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265DecPicBufMgr.html>
#[doc(alias = "StdVideoH265DecPicBufMgr")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H265DecPicBufMgr {
    pub max_latency_increase_plus_1: [u32; 7],
    pub max_dec_pic_buffering_minus_1: [u8; 7],
    pub max_num_reorder_pics: [u8; 7],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265SubLayerHrdParameters.html>
#[doc(alias = "StdVideoH265SubLayerHrdParameters")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H265SubLayerHrdParameters {
    pub bit_rate_value_minus_1: [u32; 32],
    pub cpb_size_value_minus_1: [u32; 32],
    pub cpb_size_du_value_minus_1: [u32; 32],
    pub bit_rate_du_value_minus_1: [u32; 32],
    pub cbr_flag: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265HrdFlags.html>
#[doc(alias = "StdVideoH265HrdFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H265HrdFlags {
    pub nal_hrd_parameters_present_flag: u32,
    pub vcl_hrd_parameters_present_flag: u32,
    pub sub_pic_hrd_params_present_flag: u32,
    pub sub_pic_cpb_params_in_pic_timing_sei_flag: u32,
    pub fixed_pic_rate_general_flag: u32,
    pub fixed_pic_rate_within_cvs_flag: u32,
    pub low_delay_hrd_flag: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265HrdParameters.html>
#[doc(alias = "StdVideoH265HrdParameters")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H265HrdParameters {
    pub flags: H265HrdFlags,
    pub tick_divisor_minus_2: u8,
    pub du_cpb_removal_delay_increment_length_minus_1: u8,
    pub dpb_output_delay_du_length_minus_1: u8,
    pub bit_rate_scale: u8,
    pub cpb_size_scale: u8,
    pub cpb_size_du_scale: u8,
    pub initial_cpb_removal_delay_length_minus_1: u8,
    pub au_cpb_removal_delay_length_minus_1: u8,
    pub dpb_output_delay_length_minus_1: u8,
    pub cpb_cnt_minus_1: [u8; 7],
    pub elemental_duration_in_tc_minus_1: [u16; 7],
    pub reserved: [u16; 3],
    pub p_sub_layer_hrd_parameters_nal: *const H265SubLayerHrdParameters,
    pub p_sub_layer_hrd_parameters_vcl: *const H265SubLayerHrdParameters,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265VpsFlags.html>
#[doc(alias = "StdVideoH265VpsFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H265VpsFlags {
    pub vps_temporal_id_nesting_flag: u32,
    pub vps_sub_layer_ordering_info_present_flag: u32,
    pub vps_timing_info_present_flag: u32,
    pub vps_poc_proportional_to_timing_flag: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265ProfileTierLevelFlags.html>
#[doc(alias = "StdVideoH265ProfileTierLevelFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H265ProfileTierLevelFlags {
    pub general_tier_flag: u32,
    pub general_progressive_source_flag: u32,
    pub general_interlaced_source_flag: u32,
    pub general_non_packed_constraint_flag: u32,
    pub general_frame_only_constraint_flag: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265ProfileTierLevel.html>
#[doc(alias = "StdVideoH265ProfileTierLevel")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H265ProfileTierLevel {
    pub flags: H265ProfileTierLevelFlags,
    pub general_profile_idc: H265ProfileIdc,
    pub general_level_idc: H265LevelIdc,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265VideoParameterSet.html>
#[doc(alias = "StdVideoH265VideoParameterSet")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H265VideoParameterSet {
    pub flags: H265VpsFlags,
    pub vps_video_parameter_set_id: u8,
    pub vps_max_sub_layers_minus_1: u8,
    pub reserved_1: u8,
    pub reserved_2: u8,
    pub vps_num_units_in_tick: u32,
    pub vps_time_scale: u32,
    pub vps_num_ticks_poc_diff_one_minus_1: u32,
    pub reserved_3: u32,
    pub p_dec_pic_buf_mgr: *const H265DecPicBufMgr,
    pub p_hrd_parameters: *const H265HrdParameters,
    pub p_profile_tier_level: *const H265ProfileTierLevel,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265ScalingLists.html>
#[doc(alias = "StdVideoH265ScalingLists")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H265ScalingLists {
    pub scaling_list_4_x_4: [[u8; 6]; 16],
    pub scaling_list_8_x_8: [[u8; 6]; 64],
    pub scaling_list_16_x_16: [[u8; 6]; 64],
    pub scaling_list_32_x_32: [[u8; 2]; 64],
    pub scaling_list_dc_coef_16_x_16: [u8; 6],
    pub scaling_list_dc_coef_32_x_32: [u8; 2],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265SpsVuiFlags.html>
#[doc(alias = "StdVideoH265SpsVuiFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H265SpsVuiFlags {
    pub aspect_ratio_info_present_flag: u32,
    pub overscan_info_present_flag: u32,
    pub overscan_appropriate_flag: u32,
    pub video_signal_type_present_flag: u32,
    pub video_full_range_flag: u32,
    pub colour_description_present_flag: u32,
    pub chroma_loc_info_present_flag: u32,
    pub neutral_chroma_indication_flag: u32,
    pub field_seq_flag: u32,
    pub frame_field_info_present_flag: u32,
    pub default_display_window_flag: u32,
    pub vui_timing_info_present_flag: u32,
    pub vui_poc_proportional_to_timing_flag: u32,
    pub vui_hrd_parameters_present_flag: u32,
    pub bitstream_restriction_flag: u32,
    pub tiles_fixed_structure_flag: u32,
    pub motion_vectors_over_pic_boundaries_flag: u32,
    pub restricted_ref_pic_lists_flag: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265SequenceParameterSetVui.html>
#[doc(alias = "StdVideoH265SequenceParameterSetVui")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H265SequenceParameterSetVui {
    pub flags: H265SpsVuiFlags,
    pub aspect_ratio_idc: H265AspectRatioIdc,
    pub sar_width: u16,
    pub sar_height: u16,
    pub video_format: u8,
    pub colour_primaries: u8,
    pub transfer_characteristics: u8,
    pub matrix_coeffs: u8,
    pub chroma_sample_loc_type_top_field: u8,
    pub chroma_sample_loc_type_bottom_field: u8,
    pub reserved_1: u8,
    pub reserved_2: u8,
    pub def_disp_win_left_offset: u16,
    pub def_disp_win_right_offset: u16,
    pub def_disp_win_top_offset: u16,
    pub def_disp_win_bottom_offset: u16,
    pub vui_num_units_in_tick: u32,
    pub vui_time_scale: u32,
    pub vui_num_ticks_poc_diff_one_minus_1: u32,
    pub min_spatial_segmentation_idc: u16,
    pub reserved_3: u16,
    pub max_bytes_per_pic_denom: u8,
    pub max_bits_per_min_cu_denom: u8,
    pub log_2_max_mv_length_horizontal: u8,
    pub log_2_max_mv_length_vertical: u8,
    pub p_hrd_parameters: *const H265HrdParameters,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265PredictorPaletteEntries.html>
#[doc(alias = "StdVideoH265PredictorPaletteEntries")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H265PredictorPaletteEntries {
    pub predictor_palette_entries: [[u16; 3]; 128],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265SpsFlags.html>
#[doc(alias = "StdVideoH265SpsFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H265SpsFlags {
    pub sps_temporal_id_nesting_flag: u32,
    pub separate_colour_plane_flag: u32,
    pub conformance_window_flag: u32,
    pub sps_sub_layer_ordering_info_present_flag: u32,
    pub scaling_list_enabled_flag: u32,
    pub sps_scaling_list_data_present_flag: u32,
    pub amp_enabled_flag: u32,
    pub sample_adaptive_offset_enabled_flag: u32,
    pub pcm_enabled_flag: u32,
    pub pcm_loop_filter_disabled_flag: u32,
    pub long_term_ref_pics_present_flag: u32,
    pub sps_temporal_mvp_enabled_flag: u32,
    pub strong_intra_smoothing_enabled_flag: u32,
    pub vui_parameters_present_flag: u32,
    pub sps_extension_present_flag: u32,
    pub sps_range_extension_flag: u32,
    pub transform_skip_rotation_enabled_flag: u32,
    pub transform_skip_context_enabled_flag: u32,
    pub implicit_rdpcm_enabled_flag: u32,
    pub explicit_rdpcm_enabled_flag: u32,
    pub extended_precision_processing_flag: u32,
    pub intra_smoothing_disabled_flag: u32,
    pub high_precision_offsets_enabled_flag: u32,
    pub persistent_rice_adaptation_enabled_flag: u32,
    pub cabac_bypass_alignment_enabled_flag: u32,
    pub sps_scc_extension_flag: u32,
    pub sps_curr_pic_ref_enabled_flag: u32,
    pub palette_mode_enabled_flag: u32,
    pub sps_palette_predictor_initializers_present_flag: u32,
    pub intra_boundary_filtering_disabled_flag: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265ShortTermRefPicSetFlags.html>
#[doc(alias = "StdVideoH265ShortTermRefPicSetFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H265ShortTermRefPicSetFlags {
    pub inter_ref_pic_set_prediction_flag: u32,
    pub delta_rps_sign: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265ShortTermRefPicSet.html>
#[doc(alias = "StdVideoH265ShortTermRefPicSet")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H265ShortTermRefPicSet {
    pub flags: H265ShortTermRefPicSetFlags,
    pub delta_idx_minus_1: u32,
    pub use_delta_flag: u16,
    pub abs_delta_rps_minus_1: u16,
    pub used_by_curr_pic_flag: u16,
    pub used_by_curr_pic_s_0_flag: u16,
    pub used_by_curr_pic_s_1_flag: u16,
    pub reserved_1: u16,
    pub reserved_2: u8,
    pub reserved_3: u8,
    pub num_negative_pics: u8,
    pub num_positive_pics: u8,
    pub delta_poc_s_0_minus_1: [u16; 16],
    pub delta_poc_s_1_minus_1: [u16; 16],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265LongTermRefPicsSps.html>
#[doc(alias = "StdVideoH265LongTermRefPicsSps")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H265LongTermRefPicsSps {
    pub used_by_curr_pic_lt_sps_flag: u32,
    pub lt_ref_pic_poc_lsb_sps: [u32; 32],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265SequenceParameterSet.html>
#[doc(alias = "StdVideoH265SequenceParameterSet")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H265SequenceParameterSet {
    pub flags: H265SpsFlags,
    pub chroma_format_idc: H265ChromaFormatIdc,
    pub pic_width_in_luma_samples: u32,
    pub pic_height_in_luma_samples: u32,
    pub sps_video_parameter_set_id: u8,
    pub sps_max_sub_layers_minus_1: u8,
    pub sps_seq_parameter_set_id: u8,
    pub bit_depth_luma_minus_8: u8,
    pub bit_depth_chroma_minus_8: u8,
    pub log_2_max_pic_order_cnt_lsb_minus_4: u8,
    pub log_2_min_luma_coding_block_size_minus_3: u8,
    pub log_2_diff_max_min_luma_coding_block_size: u8,
    pub log_2_min_luma_transform_block_size_minus_2: u8,
    pub log_2_diff_max_min_luma_transform_block_size: u8,
    pub max_transform_hierarchy_depth_inter: u8,
    pub max_transform_hierarchy_depth_intra: u8,
    pub num_short_term_ref_pic_sets: u8,
    pub num_long_term_ref_pics_sps: u8,
    pub pcm_sample_bit_depth_luma_minus_1: u8,
    pub pcm_sample_bit_depth_chroma_minus_1: u8,
    pub log_2_min_pcm_luma_coding_block_size_minus_3: u8,
    pub log_2_diff_max_min_pcm_luma_coding_block_size: u8,
    pub reserved_1: u8,
    pub reserved_2: u8,
    pub palette_max_size: u8,
    pub delta_palette_max_predictor_size: u8,
    pub motion_vector_resolution_control_idc: u8,
    pub sps_num_palette_predictor_initializers_minus_1: u8,
    pub conf_win_left_offset: u32,
    pub conf_win_right_offset: u32,
    pub conf_win_top_offset: u32,
    pub conf_win_bottom_offset: u32,
    pub p_profile_tier_level: *const H265ProfileTierLevel,
    pub p_dec_pic_buf_mgr: *const H265DecPicBufMgr,
    pub p_scaling_lists: *const H265ScalingLists,
    pub p_short_term_ref_pic_set: *const H265ShortTermRefPicSet,
    pub p_long_term_ref_pics_sps: *const H265LongTermRefPicsSps,
    pub p_sequence_parameter_set_vui: *const H265SequenceParameterSetVui,
    pub p_predictor_palette_entries: *const H265PredictorPaletteEntries,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265PpsFlags.html>
#[doc(alias = "StdVideoH265PpsFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H265PpsFlags {
    pub dependent_slice_segments_enabled_flag: u32,
    pub output_flag_present_flag: u32,
    pub sign_data_hiding_enabled_flag: u32,
    pub cabac_init_present_flag: u32,
    pub constrained_intra_pred_flag: u32,
    pub transform_skip_enabled_flag: u32,
    pub cu_qp_delta_enabled_flag: u32,
    pub pps_slice_chroma_qp_offsets_present_flag: u32,
    pub weighted_pred_flag: u32,
    pub weighted_bipred_flag: u32,
    pub transquant_bypass_enabled_flag: u32,
    pub tiles_enabled_flag: u32,
    pub entropy_coding_sync_enabled_flag: u32,
    pub uniform_spacing_flag: u32,
    pub loop_filter_across_tiles_enabled_flag: u32,
    pub pps_loop_filter_across_slices_enabled_flag: u32,
    pub deblocking_filter_control_present_flag: u32,
    pub deblocking_filter_override_enabled_flag: u32,
    pub pps_deblocking_filter_disabled_flag: u32,
    pub pps_scaling_list_data_present_flag: u32,
    pub lists_modification_present_flag: u32,
    pub slice_segment_header_extension_present_flag: u32,
    pub pps_extension_present_flag: u32,
    pub cross_component_prediction_enabled_flag: u32,
    pub chroma_qp_offset_list_enabled_flag: u32,
    pub pps_curr_pic_ref_enabled_flag: u32,
    pub residual_adaptive_colour_transform_enabled_flag: u32,
    pub pps_slice_act_qp_offsets_present_flag: u32,
    pub pps_palette_predictor_initializers_present_flag: u32,
    pub monochrome_palette_flag: u32,
    pub pps_range_extension_flag: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265PictureParameterSet.html>
#[doc(alias = "StdVideoH265PictureParameterSet")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct H265PictureParameterSet {
    pub flags: H265PpsFlags,
    pub pps_pic_parameter_set_id: u8,
    pub pps_seq_parameter_set_id: u8,
    pub sps_video_parameter_set_id: u8,
    pub num_extra_slice_header_bits: u8,
    pub num_ref_idx_l_0_default_active_minus_1: u8,
    pub num_ref_idx_l_1_default_active_minus_1: u8,
    pub init_qp_minus_26: i8,
    pub diff_cu_qp_delta_depth: u8,
    pub pps_cb_qp_offset: i8,
    pub pps_cr_qp_offset: i8,
    pub pps_beta_offset_div_2: i8,
    pub pps_tc_offset_div_2: i8,
    pub log_2_parallel_merge_level_minus_2: u8,
    pub log_2_max_transform_skip_block_size_minus_2: u8,
    pub diff_cu_chroma_qp_offset_depth: u8,
    pub chroma_qp_offset_list_len_minus_1: u8,
    pub cb_qp_offset_list: [i8; 6],
    pub cr_qp_offset_list: [i8; 6],
    pub log_2_sao_offset_scale_luma: u8,
    pub log_2_sao_offset_scale_chroma: u8,
    pub pps_act_y_qp_offset_plus_5: i8,
    pub pps_act_cb_qp_offset_plus_5: i8,
    pub pps_act_cr_qp_offset_plus_3: i8,
    pub pps_num_palette_predictor_initializers: u8,
    pub luma_bit_depth_entry_minus_8: u8,
    pub chroma_bit_depth_entry_minus_8: u8,
    pub num_tile_columns_minus_1: u8,
    pub num_tile_rows_minus_1: u8,
    pub reserved_1: u8,
    pub reserved_2: u8,
    pub column_width_minus_1: [u16; 19],
    pub row_height_minus_1: [u16; 21],
    pub reserved_3: u32,
    pub p_scaling_lists: *const H265ScalingLists,
    pub p_predictor_palette_entries: *const H265PredictorPaletteEntries,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoDecodeH265PictureInfoFlags.html>
#[doc(alias = "StdVideoDecodeH265PictureInfoFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DecodeH265PictureInfoFlags {
    pub irap_pic_flag: u32,
    pub idr_pic_flag: u32,
    pub is_reference: u32,
    pub short_term_ref_pic_set_sps_flag: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoDecodeH265PictureInfo.html>
#[doc(alias = "StdVideoDecodeH265PictureInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DecodeH265PictureInfo {
    pub flags: DecodeH265PictureInfoFlags,
    pub sps_video_parameter_set_id: u8,
    pub pps_seq_parameter_set_id: u8,
    pub pps_pic_parameter_set_id: u8,
    pub num_delta_pocs_of_ref_rps_idx: u8,
    pub pic_order_cnt_val: i32,
    pub num_bits_for_st_ref_pic_set_in_slice: u16,
    pub reserved: u16,
    pub ref_pic_set_st_curr_before: [u8; 8],
    pub ref_pic_set_st_curr_after: [u8; 8],
    pub ref_pic_set_lt_curr: [u8; 8],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoDecodeH265ReferenceInfoFlags.html>
#[doc(alias = "StdVideoDecodeH265ReferenceInfoFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DecodeH265ReferenceInfoFlags {
    pub used_for_long_term_reference: u32,
    pub unused_for_reference: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoDecodeH265ReferenceInfo.html>
#[doc(alias = "StdVideoDecodeH265ReferenceInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DecodeH265ReferenceInfo {
    pub flags: DecodeH265ReferenceInfoFlags,
    pub pic_order_cnt_val: i32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeH265WeightTableFlags.html>
#[doc(alias = "StdVideoEncodeH265WeightTableFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeH265WeightTableFlags {
    pub luma_weight_l_0_flag: u16,
    pub chroma_weight_l_0_flag: u16,
    pub luma_weight_l_1_flag: u16,
    pub chroma_weight_l_1_flag: u16,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeH265WeightTable.html>
#[doc(alias = "StdVideoEncodeH265WeightTable")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeH265WeightTable {
    pub flags: EncodeH265WeightTableFlags,
    pub luma_log_2_weight_denom: u8,
    pub delta_chroma_log_2_weight_denom: i8,
    pub delta_luma_weight_l_0: [i8; 15],
    pub luma_offset_l_0: [i8; 15],
    pub delta_chroma_weight_l_0: [[i8; 15]; 2],
    pub delta_chroma_offset_l_0: [[i8; 15]; 2],
    pub delta_luma_weight_l_1: [i8; 15],
    pub luma_offset_l_1: [i8; 15],
    pub delta_chroma_weight_l_1: [[i8; 15]; 2],
    pub delta_chroma_offset_l_1: [[i8; 15]; 2],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeH265SliceSegmentHeaderFlags.html>
#[doc(alias = "StdVideoEncodeH265SliceSegmentHeaderFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeH265SliceSegmentHeaderFlags {
    pub first_slice_segment_in_pic_flag: u32,
    pub dependent_slice_segment_flag: u32,
    pub slice_sao_luma_flag: u32,
    pub slice_sao_chroma_flag: u32,
    pub num_ref_idx_active_override_flag: u32,
    pub mvd_l_1_zero_flag: u32,
    pub cabac_init_flag: u32,
    pub cu_chroma_qp_offset_enabled_flag: u32,
    pub deblocking_filter_override_flag: u32,
    pub slice_deblocking_filter_disabled_flag: u32,
    pub collocated_from_l_0_flag: u32,
    pub slice_loop_filter_across_slices_enabled_flag: u32,
    pub reserved: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeH265SliceSegmentHeader.html>
#[doc(alias = "StdVideoEncodeH265SliceSegmentHeader")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeH265SliceSegmentHeader {
    pub flags: EncodeH265SliceSegmentHeaderFlags,
    pub slice_type: H265SliceType,
    pub slice_segment_address: u32,
    pub collocated_ref_idx: u8,
    pub max_num_merge_cand: u8,
    pub slice_cb_qp_offset: i8,
    pub slice_cr_qp_offset: i8,
    pub slice_beta_offset_div_2: i8,
    pub slice_tc_offset_div_2: i8,
    pub slice_act_y_qp_offset: i8,
    pub slice_act_cb_qp_offset: i8,
    pub slice_act_cr_qp_offset: i8,
    pub slice_qp_delta: i8,
    pub reserved_1: u16,
    pub p_weight_table: *const EncodeH265WeightTable,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeH265ReferenceListsInfoFlags.html>
#[doc(alias = "StdVideoEncodeH265ReferenceListsInfoFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeH265ReferenceListsInfoFlags {
    pub ref_pic_list_modification_flag_l_0: u32,
    pub ref_pic_list_modification_flag_l_1: u32,
    pub reserved: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeH265ReferenceListsInfo.html>
#[doc(alias = "StdVideoEncodeH265ReferenceListsInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeH265ReferenceListsInfo {
    pub flags: EncodeH265ReferenceListsInfoFlags,
    pub num_ref_idx_l_0_active_minus_1: u8,
    pub num_ref_idx_l_1_active_minus_1: u8,
    pub ref_pic_list_0: [u8; 15],
    pub ref_pic_list_1: [u8; 15],
    pub list_entry_l_0: [u8; 15],
    pub list_entry_l_1: [u8; 15],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeH265PictureInfoFlags.html>
#[doc(alias = "StdVideoEncodeH265PictureInfoFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeH265PictureInfoFlags {
    pub is_reference: u32,
    pub irap_pic_flag: u32,
    pub used_for_long_term_reference: u32,
    pub discardable_flag: u32,
    pub cross_layer_bla_flag: u32,
    pub pic_output_flag: u32,
    pub no_output_of_prior_pics_flag: u32,
    pub short_term_ref_pic_set_sps_flag: u32,
    pub slice_temporal_mvp_enabled_flag: u32,
    pub reserved: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeH265LongTermRefPics.html>
#[doc(alias = "StdVideoEncodeH265LongTermRefPics")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeH265LongTermRefPics {
    pub num_long_term_sps: u8,
    pub num_long_term_pics: u8,
    pub lt_idx_sps: [u8; 32],
    pub poc_lsb_lt: [u8; 16],
    pub used_by_curr_pic_lt_flag: u16,
    pub delta_poc_msb_present_flag: [u8; 48],
    pub delta_poc_msb_cycle_lt: [u8; 48],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeH265PictureInfo.html>
#[doc(alias = "StdVideoEncodeH265PictureInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeH265PictureInfo {
    pub flags: EncodeH265PictureInfoFlags,
    pub pic_type: H265PictureType,
    pub sps_video_parameter_set_id: u8,
    pub pps_seq_parameter_set_id: u8,
    pub pps_pic_parameter_set_id: u8,
    pub short_term_ref_pic_set_idx: u8,
    pub pic_order_cnt_val: i32,
    pub temporal_id: u8,
    pub reserved_1: [u8; 7],
    pub p_ref_lists: *const EncodeH265ReferenceListsInfo,
    pub p_short_term_ref_pic_set: *const H265ShortTermRefPicSet,
    pub p_long_term_ref_pics: *const EncodeH265LongTermRefPics,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeH265ReferenceInfoFlags.html>
#[doc(alias = "StdVideoEncodeH265ReferenceInfoFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeH265ReferenceInfoFlags {
    pub used_for_long_term_reference: u32,
    pub unused_for_reference: u32,
    pub reserved: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeH265ReferenceInfo.html>
#[doc(alias = "StdVideoEncodeH265ReferenceInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeH265ReferenceInfo {
    pub flags: EncodeH265ReferenceInfoFlags,
    pub pic_type: H265PictureType,
    pub pic_order_cnt_val: i32,
    pub temporal_id: u8,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1ColorConfigFlags.html>
#[doc(alias = "StdVideoAV1ColorConfigFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AV1ColorConfigFlags {
    pub mono_chrome: u32,
    pub color_range: u32,
    pub separate_uv_delta_q: u32,
    pub color_description_present_flag: u32,
    pub reserved: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1ColorConfig.html>
#[doc(alias = "StdVideoAV1ColorConfig")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AV1ColorConfig {
    pub flags: AV1ColorConfigFlags,
    pub bit_depth: u8,
    pub subsampling_x: u8,
    pub subsampling_y: u8,
    pub reserved_1: u8,
    pub color_primaries: AV1ColorPrimaries,
    pub transfer_characteristics: AV1TransferCharacteristics,
    pub matrix_coefficients: AV1MatrixCoefficients,
    pub chroma_sample_position: AV1ChromaSamplePosition,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1TimingInfoFlags.html>
#[doc(alias = "StdVideoAV1TimingInfoFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AV1TimingInfoFlags {
    pub equal_picture_interval: u32,
    pub reserved: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1TimingInfo.html>
#[doc(alias = "StdVideoAV1TimingInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AV1TimingInfo {
    pub flags: AV1TimingInfoFlags,
    pub num_units_in_display_tick: u32,
    pub time_scale: u32,
    pub num_ticks_per_picture_minus_1: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1LoopFilterFlags.html>
#[doc(alias = "StdVideoAV1LoopFilterFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AV1LoopFilterFlags {
    pub loop_filter_delta_enabled: u32,
    pub loop_filter_delta_update: u32,
    pub reserved: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1LoopFilter.html>
#[doc(alias = "StdVideoAV1LoopFilter")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AV1LoopFilter {
    pub flags: AV1LoopFilterFlags,
    pub loop_filter_level: [u8; 4],
    pub loop_filter_sharpness: u8,
    pub update_ref_delta: u8,
    pub loop_filter_ref_deltas: [i8; 8],
    pub update_mode_delta: u8,
    pub loop_filter_mode_deltas: [i8; 2],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1QuantizationFlags.html>
#[doc(alias = "StdVideoAV1QuantizationFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AV1QuantizationFlags {
    pub using_qmatrix: u32,
    pub diff_uv_delta: u32,
    pub reserved: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1Quantization.html>
#[doc(alias = "StdVideoAV1Quantization")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AV1Quantization {
    pub flags: AV1QuantizationFlags,
    pub base_q_idx: u8,
    pub delta_qy_dc: i8,
    pub delta_qu_dc: i8,
    pub delta_qu_ac: i8,
    pub delta_qv_dc: i8,
    pub delta_qv_ac: i8,
    pub qm_y: u8,
    pub qm_u: u8,
    pub qm_v: u8,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1Segmentation.html>
#[doc(alias = "StdVideoAV1Segmentation")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AV1Segmentation {
    pub feature_enabled: [u8; 8],
    pub feature_data: [[i16; 8]; 8],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1TileInfoFlags.html>
#[doc(alias = "StdVideoAV1TileInfoFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AV1TileInfoFlags {
    pub uniform_tile_spacing_flag: u32,
    pub reserved: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1TileInfo.html>
#[doc(alias = "StdVideoAV1TileInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AV1TileInfo {
    pub flags: AV1TileInfoFlags,
    pub tile_cols: u8,
    pub tile_rows: u8,
    pub context_update_tile_id: u16,
    pub tile_size_bytes_minus_1: u8,
    pub reserved_1: [u8; 7],
    pub p_mi_col_starts: *const u16,
    pub p_mi_row_starts: *const u16,
    pub p_width_in_sbs_minus_1: *const u16,
    pub p_height_in_sbs_minus_1: *const u16,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1CDEF.html>
#[doc(alias = "StdVideoAV1CDEF")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AV1CDEF {
    pub cdef_damping_minus_3: u8,
    pub cdef_bits: u8,
    pub cdef_y_pri_strength: [u8; 8],
    pub cdef_y_sec_strength: [u8; 8],
    pub cdef_uv_pri_strength: [u8; 8],
    pub cdef_uv_sec_strength: [u8; 8],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1LoopRestoration.html>
#[doc(alias = "StdVideoAV1LoopRestoration")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AV1LoopRestoration {
    pub frame_restoration_type: [AV1FrameRestorationType; 3],
    pub loop_restoration_size: [u16; 3],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1GlobalMotion.html>
#[doc(alias = "StdVideoAV1GlobalMotion")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AV1GlobalMotion {
    pub gm_type: [u8; 8],
    pub gm_params: [[i32; 8]; 6],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1FilmGrainFlags.html>
#[doc(alias = "StdVideoAV1FilmGrainFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AV1FilmGrainFlags {
    pub chroma_scaling_from_luma: u32,
    pub overlap_flag: u32,
    pub clip_to_restricted_range: u32,
    pub update_grain: u32,
    pub reserved: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1FilmGrain.html>
#[doc(alias = "StdVideoAV1FilmGrain")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AV1FilmGrain {
    pub flags: AV1FilmGrainFlags,
    pub grain_scaling_minus_8: u8,
    pub ar_coeff_lag: u8,
    pub ar_coeff_shift_minus_6: u8,
    pub grain_scale_shift: u8,
    pub grain_seed: u16,
    pub film_grain_params_ref_idx: u8,
    pub num_y_points: u8,
    pub point_y_value: [u8; 14],
    pub point_y_scaling: [u8; 14],
    pub num_cb_points: u8,
    pub point_cb_value: [u8; 10],
    pub point_cb_scaling: [u8; 10],
    pub num_cr_points: u8,
    pub point_cr_value: [u8; 10],
    pub point_cr_scaling: [u8; 10],
    pub ar_coeffs_y_plus_128: [i8; 24],
    pub ar_coeffs_cb_plus_128: [i8; 25],
    pub ar_coeffs_cr_plus_128: [i8; 25],
    pub cb_mult: u8,
    pub cb_luma_mult: u8,
    pub cb_offset: u16,
    pub cr_mult: u8,
    pub cr_luma_mult: u8,
    pub cr_offset: u16,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1SequenceHeaderFlags.html>
#[doc(alias = "StdVideoAV1SequenceHeaderFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AV1SequenceHeaderFlags {
    pub still_picture: u32,
    pub reduced_still_picture_header: u32,
    pub use_128_x_128_superblock: u32,
    pub enable_filter_intra: u32,
    pub enable_intra_edge_filter: u32,
    pub enable_interintra_compound: u32,
    pub enable_masked_compound: u32,
    pub enable_warped_motion: u32,
    pub enable_dual_filter: u32,
    pub enable_order_hint: u32,
    pub enable_jnt_comp: u32,
    pub enable_ref_frame_mvs: u32,
    pub frame_id_numbers_present_flag: u32,
    pub enable_superres: u32,
    pub enable_cdef: u32,
    pub enable_restoration: u32,
    pub film_grain_params_present: u32,
    pub timing_info_present_flag: u32,
    pub initial_display_delay_present_flag: u32,
    pub reserved: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1SequenceHeader.html>
#[doc(alias = "StdVideoAV1SequenceHeader")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AV1SequenceHeader {
    pub flags: AV1SequenceHeaderFlags,
    pub seq_profile: AV1Profile,
    pub frame_width_bits_minus_1: u8,
    pub frame_height_bits_minus_1: u8,
    pub max_frame_width_minus_1: u16,
    pub max_frame_height_minus_1: u16,
    pub delta_frame_id_length_minus_2: u8,
    pub additional_frame_id_length_minus_1: u8,
    pub order_hint_bits_minus_1: u8,
    pub seq_force_integer_mv: u8,
    pub seq_force_screen_content_tools: u8,
    pub reserved_1: [u8; 5],
    pub p_color_config: *const AV1ColorConfig,
    pub p_timing_info: *const AV1TimingInfo,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoDecodeAV1PictureInfoFlags.html>
#[doc(alias = "StdVideoDecodeAV1PictureInfoFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DecodeAV1PictureInfoFlags {
    pub error_resilient_mode: u32,
    pub disable_cdf_update: u32,
    pub use_superres: u32,
    pub render_and_frame_size_different: u32,
    pub allow_screen_content_tools: u32,
    pub is_filter_switchable: u32,
    pub force_integer_mv: u32,
    pub frame_size_override_flag: u32,
    pub buffer_removal_time_present_flag: u32,
    pub allow_intrabc: u32,
    pub frame_refs_short_signaling: u32,
    pub allow_high_precision_mv: u32,
    pub is_motion_mode_switchable: u32,
    pub use_ref_frame_mvs: u32,
    pub disable_frame_end_update_cdf: u32,
    pub allow_warped_motion: u32,
    pub reduced_tx_set: u32,
    pub reference_select: u32,
    pub skip_mode_present: u32,
    pub delta_q_present: u32,
    pub delta_lf_present: u32,
    pub delta_lf_multi: u32,
    pub segmentation_enabled: u32,
    pub segmentation_update_map: u32,
    pub segmentation_temporal_update: u32,
    pub segmentation_update_data: u32,
    pub uses_lr: u32,
    pub uses_chroma_lr: u32,
    pub apply_grain: u32,
    pub reserved: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoDecodeAV1PictureInfo.html>
#[doc(alias = "StdVideoDecodeAV1PictureInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DecodeAV1PictureInfo {
    pub flags: DecodeAV1PictureInfoFlags,
    pub frame_type: AV1FrameType,
    pub current_frame_id: u32,
    pub order_hint: u8,
    pub primary_ref_frame: u8,
    pub refresh_frame_flags: u8,
    pub reserved_1: u8,
    pub interpolation_filter: AV1InterpolationFilter,
    pub tx_mode: AV1TxMode,
    pub delta_q_res: u8,
    pub delta_lf_res: u8,
    pub skip_mode_frame: [u8; 2],
    pub coded_denom: u8,
    pub reserved_2: [u8; 3],
    pub order_hints: [u8; 8],
    pub expected_frame_id: [u32; 8],
    pub p_tile_info: *const AV1TileInfo,
    pub p_quantization: *const AV1Quantization,
    pub p_segmentation: *const AV1Segmentation,
    pub p_loop_filter: *const AV1LoopFilter,
    pub p_cdef: *const AV1CDEF,
    pub p_loop_restoration: *const AV1LoopRestoration,
    pub p_global_motion: *const AV1GlobalMotion,
    pub p_film_grain: *const AV1FilmGrain,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoDecodeAV1ReferenceInfoFlags.html>
#[doc(alias = "StdVideoDecodeAV1ReferenceInfoFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DecodeAV1ReferenceInfoFlags {
    pub disable_frame_end_update_cdf: u32,
    pub segmentation_enabled: u32,
    pub reserved: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoDecodeAV1ReferenceInfo.html>
#[doc(alias = "StdVideoDecodeAV1ReferenceInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DecodeAV1ReferenceInfo {
    pub flags: DecodeAV1ReferenceInfoFlags,
    pub frame_type: u8,
    pub ref_frame_sign_bias: u8,
    pub order_hint: u8,
    pub saved_order_hints: [u8; 8],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeAV1DecoderModelInfo.html>
#[doc(alias = "StdVideoEncodeAV1DecoderModelInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeAV1DecoderModelInfo {
    pub buffer_delay_length_minus_1: u8,
    pub buffer_removal_time_length_minus_1: u8,
    pub frame_presentation_time_length_minus_1: u8,
    pub reserved_1: u8,
    pub num_units_in_decoding_tick: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeAV1ExtensionHeader.html>
#[doc(alias = "StdVideoEncodeAV1ExtensionHeader")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeAV1ExtensionHeader {
    pub temporal_id: u8,
    pub spatial_id: u8,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeAV1OperatingPointInfoFlags.html>
#[doc(alias = "StdVideoEncodeAV1OperatingPointInfoFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeAV1OperatingPointInfoFlags {
    pub decoder_model_present_for_this_op: u32,
    pub low_delay_mode_flag: u32,
    pub initial_display_delay_present_for_this_op: u32,
    pub reserved: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeAV1OperatingPointInfo.html>
#[doc(alias = "StdVideoEncodeAV1OperatingPointInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeAV1OperatingPointInfo {
    pub flags: EncodeAV1OperatingPointInfoFlags,
    pub operating_point_idc: u16,
    pub seq_level_idx: u8,
    pub seq_tier: u8,
    pub decoder_buffer_delay: u32,
    pub encoder_buffer_delay: u32,
    pub initial_display_delay_minus_1: u8,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeAV1PictureInfoFlags.html>
#[doc(alias = "StdVideoEncodeAV1PictureInfoFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeAV1PictureInfoFlags {
    pub error_resilient_mode: u32,
    pub disable_cdf_update: u32,
    pub use_superres: u32,
    pub render_and_frame_size_different: u32,
    pub allow_screen_content_tools: u32,
    pub is_filter_switchable: u32,
    pub force_integer_mv: u32,
    pub frame_size_override_flag: u32,
    pub buffer_removal_time_present_flag: u32,
    pub allow_intrabc: u32,
    pub frame_refs_short_signaling: u32,
    pub allow_high_precision_mv: u32,
    pub is_motion_mode_switchable: u32,
    pub use_ref_frame_mvs: u32,
    pub disable_frame_end_update_cdf: u32,
    pub allow_warped_motion: u32,
    pub reduced_tx_set: u32,
    pub skip_mode_present: u32,
    pub delta_q_present: u32,
    pub delta_lf_present: u32,
    pub delta_lf_multi: u32,
    pub segmentation_enabled: u32,
    pub segmentation_update_map: u32,
    pub segmentation_temporal_update: u32,
    pub segmentation_update_data: u32,
    pub uses_lr: u32,
    pub uses_chroma_lr: u32,
    pub show_frame: u32,
    pub showable_frame: u32,
    pub reserved: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeAV1PictureInfo.html>
#[doc(alias = "StdVideoEncodeAV1PictureInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeAV1PictureInfo {
    pub flags: EncodeAV1PictureInfoFlags,
    pub frame_type: AV1FrameType,
    pub frame_presentation_time: u32,
    pub current_frame_id: u32,
    pub order_hint: u8,
    pub primary_ref_frame: u8,
    pub refresh_frame_flags: u8,
    pub coded_denom: u8,
    pub render_width_minus_1: u16,
    pub render_height_minus_1: u16,
    pub interpolation_filter: AV1InterpolationFilter,
    pub tx_mode: AV1TxMode,
    pub delta_q_res: u8,
    pub delta_lf_res: u8,
    pub ref_order_hint: [u8; 8],
    pub ref_frame_idx: [i8; 7],
    pub reserved_1: [u8; 3],
    pub delta_frame_id_minus_1: [u32; 7],
    pub p_tile_info: *const AV1TileInfo,
    pub p_quantization: *const AV1Quantization,
    pub p_segmentation: *const AV1Segmentation,
    pub p_loop_filter: *const AV1LoopFilter,
    pub p_cdef: *const AV1CDEF,
    pub p_loop_restoration: *const AV1LoopRestoration,
    pub p_global_motion: *const AV1GlobalMotion,
    pub p_extension_header: *const EncodeAV1ExtensionHeader,
    pub p_buffer_removal_times: *const u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeAV1ReferenceInfoFlags.html>
#[doc(alias = "StdVideoEncodeAV1ReferenceInfoFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeAV1ReferenceInfoFlags {
    pub disable_frame_end_update_cdf: u32,
    pub segmentation_enabled: u32,
    pub reserved: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoEncodeAV1ReferenceInfo.html>
#[doc(alias = "StdVideoEncodeAV1ReferenceInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EncodeAV1ReferenceInfo {
    pub flags: EncodeAV1ReferenceInfoFlags,
    pub ref_frame_id: u32,
    pub frame_type: AV1FrameType,
    pub order_hint: u8,
    pub reserved_1: [u8; 3],
    pub p_extension_header: *const EncodeAV1ExtensionHeader,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoVP9ColorConfigFlags.html>
#[doc(alias = "StdVideoVP9ColorConfigFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VP9ColorConfigFlags {
    pub color_range: u32,
    pub reserved: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoVP9ColorConfig.html>
#[doc(alias = "StdVideoVP9ColorConfig")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VP9ColorConfig {
    pub flags: VP9ColorConfigFlags,
    pub bit_depth: u8,
    pub subsampling_x: u8,
    pub subsampling_y: u8,
    pub reserved_1: u8,
    pub color_space: VP9ColorSpace,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoVP9LoopFilterFlags.html>
#[doc(alias = "StdVideoVP9LoopFilterFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VP9LoopFilterFlags {
    pub loop_filter_delta_enabled: u32,
    pub loop_filter_delta_update: u32,
    pub reserved: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoVP9LoopFilter.html>
#[doc(alias = "StdVideoVP9LoopFilter")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VP9LoopFilter {
    pub flags: VP9LoopFilterFlags,
    pub loop_filter_level: u8,
    pub loop_filter_sharpness: u8,
    pub update_ref_delta: u8,
    pub loop_filter_ref_deltas: [i8; 4],
    pub update_mode_delta: u8,
    pub loop_filter_mode_deltas: [i8; 2],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoVP9SegmentationFlags.html>
#[doc(alias = "StdVideoVP9SegmentationFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VP9SegmentationFlags {
    pub segmentation_update_map: u32,
    pub segmentation_temporal_update: u32,
    pub segmentation_update_data: u32,
    pub segmentation_abs_or_delta_update: u32,
    pub reserved: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoVP9Segmentation.html>
#[doc(alias = "StdVideoVP9Segmentation")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VP9Segmentation {
    pub flags: VP9SegmentationFlags,
    pub segmentation_tree_probs: [u8; 7],
    pub segmentation_pred_prob: [u8; 3],
    pub feature_enabled: [u8; 8],
    pub feature_data: [[i16; 8]; 4],
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoDecodeVP9PictureInfoFlags.html>
#[doc(alias = "StdVideoDecodeVP9PictureInfoFlags")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DecodeVP9PictureInfoFlags {
    pub error_resilient_mode: u32,
    pub intra_only: u32,
    pub allow_high_precision_mv: u32,
    pub refresh_frame_context: u32,
    pub frame_parallel_decoding_mode: u32,
    pub segmentation_enabled: u32,
    pub show_frame: u32,
    pub use_prev_frame_mvs: u32,
    pub reserved: u32,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoDecodeVP9PictureInfo.html>
#[doc(alias = "StdVideoDecodeVP9PictureInfo")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DecodeVP9PictureInfo {
    pub flags: DecodeVP9PictureInfoFlags,
    pub profile: VP9Profile,
    pub frame_type: VP9FrameType,
    pub frame_context_idx: u8,
    pub reset_frame_context: u8,
    pub refresh_frame_flags: u8,
    pub ref_frame_sign_bias_mask: u8,
    pub interpolation_filter: VP9InterpolationFilter,
    pub base_q_idx: u8,
    pub delta_q_y_dc: i8,
    pub delta_q_uv_dc: i8,
    pub delta_q_uv_ac: i8,
    pub tile_cols_log_2: u8,
    pub tile_rows_log_2: u8,
    pub reserved_1: [u16; 3],
    pub p_color_config: *const VP9ColorConfig,
    pub p_loop_filter: *const VP9LoopFilter,
    pub p_segmentation: *const VP9Segmentation,
}
