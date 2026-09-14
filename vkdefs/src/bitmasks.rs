 // WARNING: AUTO GENERATED MODULE
#![allow(nonstandard_style)]
#![allow(unused_imports)]

use std::ffi::{c_void, c_int, c_uint, c_char};
use crate::inner::*;
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct FormatFeatureFlags: u32 {
        const SAMPLED_IMAGE_BIT = 1;
        const STORAGE_IMAGE_BIT = 2;
        const STORAGE_IMAGE_ATOMIC_BIT = 4;
        const UNIFORM_TEXEL_BUFFER_BIT = 8;
        const STORAGE_TEXEL_BUFFER_BIT = 16;
        const STORAGE_TEXEL_BUFFER_ATOMIC_BIT = 32;
        const VERTEX_BUFFER_BIT = 64;
        const COLOR_ATTACHMENT_BIT = 128;
        const COLOR_ATTACHMENT_BLEND_BIT = 256;
        const DEPTH_STENCIL_ATTACHMENT_BIT = 512;
        const BLIT_SRC_BIT = 1024;
        const BLIT_DST_BIT = 2048;
        const SAMPLED_IMAGE_FILTER_LINEAR_BIT = 4096;
        const TRANSFER_SRC_BIT = 16384;
        const TRANSFER_DST_BIT = 32768;
        const MIDPOINT_CHROMA_SAMPLES_BIT = 131072;
        const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT = 262144;
        const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT = 524288;
        const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT = 1048576;
        const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT = 2097152;
        const DISJOINT_BIT = 4194304;
        const COSITED_CHROMA_SAMPLES_BIT = 8388608;
        const SAMPLED_IMAGE_FILTER_MINMAX_BIT = 65536;
        const VIDEO_DECODE_OUTPUT_BIT_KHR = 33554432;
        const VIDEO_DECODE_DPB_BIT_KHR = 67108864;
        const ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR = 536870912;
        const SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT = 8192;
        const FRAGMENT_DENSITY_MAP_BIT_EXT = 16777216;
        const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 1073741824;
        const VIDEO_ENCODE_INPUT_BIT_KHR = 134217728;
        const VIDEO_ENCODE_DPB_BIT_KHR = 268435456;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCreateFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ImageCreateFlags: u32 {
        const SPARSE_BINDING_BIT = 1;
        const SPARSE_RESIDENCY_BIT = 2;
        const SPARSE_ALIASED_BIT = 4;
        const MUTABLE_FORMAT_BIT = 8;
        const CUBE_COMPATIBLE_BIT = 16;
        const ALIAS_BIT = 1024;
        const SPLIT_INSTANCE_BIND_REGIONS_BIT = 64;
        const _2D_ARRAY_COMPATIBLE_BIT = 32;
        const BLOCK_TEXEL_VIEW_COMPATIBLE_BIT = 128;
        const EXTENDED_USAGE_BIT = 256;
        const PROTECTED_BIT = 2048;
        const DISJOINT_BIT = 512;
        const CORNER_SAMPLED_BIT_NV = 8192;
        const DESCRIPTOR_HEAP_CAPTURE_REPLAY_BIT_EXT = 65536;
        const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT = 4096;
        const SUBSAMPLED_BIT_EXT = 16384;
        const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT = 262144;
        const _2D_VIEW_COMPATIBLE_BIT_EXT = 131072;
        const VIDEO_PROFILE_INDEPENDENT_BIT_KHR = 1048576;
        const FRAGMENT_DENSITY_MAP_OFFSET_BIT_EXT = 32768;
        const ALIAS_SINGLE_LAYER_DESCRIPTOR_BIT_KHR = 4194304;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSampleCountFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct SampleCountFlags: u32 {
        const _1_BIT = 1;
        const _2_BIT = 2;
        const _4_BIT = 4;
        const _8_BIT = 8;
        const _16_BIT = 16;
        const _32_BIT = 32;
        const _64_BIT = 64;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageUsageFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ImageUsageFlags: u32 {
        const TRANSFER_SRC_BIT = 1;
        const TRANSFER_DST_BIT = 2;
        const SAMPLED_BIT = 4;
        const STORAGE_BIT = 8;
        const COLOR_ATTACHMENT_BIT = 16;
        const DEPTH_STENCIL_ATTACHMENT_BIT = 32;
        const TRANSIENT_ATTACHMENT_BIT = 64;
        const INPUT_ATTACHMENT_BIT = 128;
        const HOST_TRANSFER_BIT = 4194304;
        const VIDEO_DECODE_DST_BIT_KHR = 1024;
        const VIDEO_DECODE_SRC_BIT_KHR = 2048;
        const VIDEO_DECODE_DPB_BIT_KHR = 4096;
        const FRAGMENT_DENSITY_MAP_BIT_EXT = 512;
        const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 256;
        const VIDEO_ENCODE_DST_BIT_KHR = 8192;
        const VIDEO_ENCODE_SRC_BIT_KHR = 16384;
        const VIDEO_ENCODE_DPB_BIT_KHR = 32768;
        const ATTACHMENT_FEEDBACK_LOOP_BIT_EXT = 524288;
        const INVOCATION_MASK_BIT_HUAWEI = 262144;
        const SAMPLE_WEIGHT_BIT_QCOM = 1048576;
        const SAMPLE_BLOCK_MATCH_BIT_QCOM = 2097152;
        const TENSOR_ALIASING_BIT_ARM = 8388608;
        const TILE_MEMORY_BIT_QCOM = 134217728;
        const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR = 33554432;
        const VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR = 67108864;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkInstanceCreateFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct InstanceCreateFlags: u32 {
        const ENUMERATE_PORTABILITY_BIT_KHR = 1;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryHeapFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct MemoryHeapFlags: u32 {
        const DEVICE_LOCAL_BIT = 1;
        const MULTI_INSTANCE_BIT = 2;
        const TILE_MEMORY_BIT_QCOM = 8;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryPropertyFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct MemoryPropertyFlags: u32 {
        const DEVICE_LOCAL_BIT = 1;
        const HOST_VISIBLE_BIT = 2;
        const HOST_COHERENT_BIT = 4;
        const HOST_CACHED_BIT = 8;
        const LAZILY_ALLOCATED_BIT = 16;
        const PROTECTED_BIT = 32;
        const DEVICE_COHERENT_BIT_AMD = 64;
        const DEVICE_UNCACHED_BIT_AMD = 128;
        const RDMA_CAPABLE_BIT_NV = 256;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct QueueFlags: u32 {
        const GRAPHICS_BIT = 1;
        const COMPUTE_BIT = 2;
        const TRANSFER_BIT = 4;
        const SPARSE_BINDING_BIT = 8;
        const PROTECTED_BIT = 16;
        const VIDEO_DECODE_BIT_KHR = 32;
        const VIDEO_ENCODE_BIT_KHR = 64;
        const OPTICAL_FLOW_BIT_NV = 256;
        const DATA_GRAPH_BIT_ARM = 1024;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderStageFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ShaderStageFlags: u32 {
        const VERTEX_BIT = 1;
        const TESSELLATION_CONTROL_BIT = 2;
        const TESSELLATION_EVALUATION_BIT = 4;
        const GEOMETRY_BIT = 8;
        const FRAGMENT_BIT = 16;
        const COMPUTE_BIT = 32;
        const ALL_GRAPHICS = 31;
        const ALL = 2147483647;
        const RAYGEN_BIT_KHR = 256;
        const ANY_HIT_BIT_KHR = 512;
        const CLOSEST_HIT_BIT_KHR = 1024;
        const MISS_BIT_KHR = 2048;
        const INTERSECTION_BIT_KHR = 4096;
        const CALLABLE_BIT_KHR = 8192;
        const TASK_BIT_EXT = 64;
        const MESH_BIT_EXT = 128;
        const SUBPASS_SHADING_BIT_HUAWEI = 16384;
        const CLUSTER_CULLING_BIT_HUAWEI = 524288;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceQueueCreateFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct DeviceQueueCreateFlags: u32 {
        const PROTECTED_BIT = 1;
        const INTERNALLY_SYNCHRONIZED_BIT_KHR = 4;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineStageFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PipelineStageFlags: u32 {
        const TOP_OF_PIPE_BIT = 1;
        const DRAW_INDIRECT_BIT = 2;
        const VERTEX_INPUT_BIT = 4;
        const VERTEX_SHADER_BIT = 8;
        const TESSELLATION_CONTROL_SHADER_BIT = 16;
        const TESSELLATION_EVALUATION_SHADER_BIT = 32;
        const GEOMETRY_SHADER_BIT = 64;
        const FRAGMENT_SHADER_BIT = 128;
        const EARLY_FRAGMENT_TESTS_BIT = 256;
        const LATE_FRAGMENT_TESTS_BIT = 512;
        const COLOR_ATTACHMENT_OUTPUT_BIT = 1024;
        const COMPUTE_SHADER_BIT = 2048;
        const TRANSFER_BIT = 4096;
        const BOTTOM_OF_PIPE_BIT = 8192;
        const HOST_BIT = 16384;
        const ALL_GRAPHICS_BIT = 32768;
        const ALL_COMMANDS_BIT = 65536;
        const NONE = 0;
        const TRANSFORM_FEEDBACK_BIT_EXT = 16777216;
        const CONDITIONAL_RENDERING_BIT_EXT = 262144;
        const ACCELERATION_STRUCTURE_BUILD_BIT_KHR = 33554432;
        const RAY_TRACING_SHADER_BIT_KHR = 2097152;
        const FRAGMENT_DENSITY_PROCESS_BIT_EXT = 8388608;
        const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 4194304;
        const TASK_SHADER_BIT_EXT = 524288;
        const MESH_SHADER_BIT_EXT = 1048576;
        const COMMAND_PREPROCESS_BIT_EXT = 131072;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryMapFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct MemoryMapFlags: u32 {
        const PLACED_BIT_EXT = 1;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageAspectFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ImageAspectFlags: u32 {
        const COLOR_BIT = 1;
        const DEPTH_BIT = 2;
        const STENCIL_BIT = 4;
        const METADATA_BIT = 8;
        const PLANE_0_BIT = 16;
        const PLANE_1_BIT = 32;
        const PLANE_2_BIT = 64;
        const NONE = 0;
        const MEMORY_PLANE_0_BIT_EXT = 128;
        const MEMORY_PLANE_1_BIT_EXT = 256;
        const MEMORY_PLANE_2_BIT_EXT = 512;
        const MEMORY_PLANE_3_BIT_EXT = 1024;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageFormatFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct SparseImageFormatFlags: u32 {
        const SINGLE_MIPTAIL_BIT = 1;
        const ALIGNED_MIP_SIZE_BIT = 2;
        const NONSTANDARD_BLOCK_SIZE_BIT = 4;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseMemoryBindFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct SparseMemoryBindFlags: u32 {
        const METADATA_BIT = 1;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceCreateFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct FenceCreateFlags: u32 {
        const SIGNALED_BIT = 1;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPoolCreateFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct QueryPoolCreateFlags: u32 {
        const RESET_BIT_KHR = 1;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPipelineStatisticFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct QueryPipelineStatisticFlags: u32 {
        const INPUT_ASSEMBLY_VERTICES_BIT = 1;
        const INPUT_ASSEMBLY_PRIMITIVES_BIT = 2;
        const VERTEX_SHADER_INVOCATIONS_BIT = 4;
        const GEOMETRY_SHADER_INVOCATIONS_BIT = 8;
        const GEOMETRY_SHADER_PRIMITIVES_BIT = 16;
        const CLIPPING_INVOCATIONS_BIT = 32;
        const CLIPPING_PRIMITIVES_BIT = 64;
        const FRAGMENT_SHADER_INVOCATIONS_BIT = 128;
        const TESSELLATION_CONTROL_SHADER_PATCHES_BIT = 256;
        const TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT = 512;
        const COMPUTE_SHADER_INVOCATIONS_BIT = 1024;
        const TASK_SHADER_INVOCATIONS_BIT_EXT = 2048;
        const MESH_SHADER_INVOCATIONS_BIT_EXT = 4096;
        const CLUSTER_CULLING_SHADER_INVOCATIONS_BIT_HUAWEI = 8192;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryResultFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct QueryResultFlags: u32 {
        const _64_BIT = 1;
        const WAIT_BIT = 2;
        const WITH_AVAILABILITY_BIT = 4;
        const PARTIAL_BIT = 8;
        const WITH_STATUS_BIT_KHR = 16;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCreateFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct BufferCreateFlags: u32 {
        const SPARSE_BINDING_BIT = 1;
        const SPARSE_RESIDENCY_BIT = 2;
        const SPARSE_ALIASED_BIT = 4;
        const PROTECTED_BIT = 8;
        const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT = 16;
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT = 32;
        const VIDEO_PROFILE_INDEPENDENT_BIT_KHR = 64;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct BufferUsageFlags: u32 {
        const TRANSFER_SRC_BIT = 1;
        const TRANSFER_DST_BIT = 2;
        const UNIFORM_TEXEL_BUFFER_BIT = 4;
        const STORAGE_TEXEL_BUFFER_BIT = 8;
        const UNIFORM_BUFFER_BIT = 16;
        const STORAGE_BUFFER_BIT = 32;
        const INDEX_BUFFER_BIT = 64;
        const VERTEX_BUFFER_BIT = 128;
        const INDIRECT_BUFFER_BIT = 256;
        const SHADER_DEVICE_ADDRESS_BIT = 131072;
        const VIDEO_DECODE_SRC_BIT_KHR = 8192;
        const VIDEO_DECODE_DST_BIT_KHR = 16384;
        const TRANSFORM_FEEDBACK_BUFFER_BIT_EXT = 2048;
        const TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT = 4096;
        const CONDITIONAL_RENDERING_BIT_EXT = 512;
        const EXECUTION_GRAPH_SCRATCH_BIT_AMDX = 33554432;
        const DESCRIPTOR_HEAP_BIT_EXT = 268435456;
        const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR = 524288;
        const ACCELERATION_STRUCTURE_STORAGE_BIT_KHR = 1048576;
        const SHADER_BINDING_TABLE_BIT_KHR = 1024;
        const VIDEO_ENCODE_DST_BIT_KHR = 32768;
        const VIDEO_ENCODE_SRC_BIT_KHR = 65536;
        const SAMPLER_DESCRIPTOR_BUFFER_BIT_EXT = 2097152;
        const RESOURCE_DESCRIPTOR_BUFFER_BIT_EXT = 4194304;
        const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_BIT_EXT = 67108864;
        const MICROMAP_BUILD_INPUT_READ_ONLY_BIT_EXT = 8388608;
        const MICROMAP_STORAGE_BIT_EXT = 16777216;
        const TILE_MEMORY_BIT_QCOM = 134217728;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewCreateFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ImageViewCreateFlags: u32 {
        const FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT = 1;
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT = 4;
        const FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT = 2;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct AccessFlags: u32 {
        const INDIRECT_COMMAND_READ_BIT = 1;
        const INDEX_READ_BIT = 2;
        const VERTEX_ATTRIBUTE_READ_BIT = 4;
        const UNIFORM_READ_BIT = 8;
        const INPUT_ATTACHMENT_READ_BIT = 16;
        const SHADER_READ_BIT = 32;
        const SHADER_WRITE_BIT = 64;
        const COLOR_ATTACHMENT_READ_BIT = 128;
        const COLOR_ATTACHMENT_WRITE_BIT = 256;
        const DEPTH_STENCIL_ATTACHMENT_READ_BIT = 512;
        const DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = 1024;
        const TRANSFER_READ_BIT = 2048;
        const TRANSFER_WRITE_BIT = 4096;
        const HOST_READ_BIT = 8192;
        const HOST_WRITE_BIT = 16384;
        const MEMORY_READ_BIT = 32768;
        const MEMORY_WRITE_BIT = 65536;
        const NONE = 0;
        const TRANSFORM_FEEDBACK_WRITE_BIT_EXT = 33554432;
        const TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT = 67108864;
        const TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT = 134217728;
        const CONDITIONAL_RENDERING_READ_BIT_EXT = 1048576;
        const COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT = 524288;
        const ACCELERATION_STRUCTURE_READ_BIT_KHR = 2097152;
        const ACCELERATION_STRUCTURE_WRITE_BIT_KHR = 4194304;
        const FRAGMENT_DENSITY_MAP_READ_BIT_EXT = 16777216;
        const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR = 8388608;
        const COMMAND_PREPROCESS_READ_BIT_EXT = 131072;
        const COMMAND_PREPROCESS_WRITE_BIT_EXT = 262144;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDependencyFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct DependencyFlags: u32 {
        const BY_REGION_BIT = 1;
        const DEVICE_GROUP_BIT = 4;
        const VIEW_LOCAL_BIT = 2;
        const FEEDBACK_LOOP_BIT_EXT = 8;
        const QUEUE_FAMILY_OWNERSHIP_TRANSFER_USE_ALL_STAGES_BIT_KHR = 32;
        const ASYMMETRIC_EVENT_BIT_KHR = 64;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolCreateFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct CommandPoolCreateFlags: u32 {
        const TRANSIENT_BIT = 1;
        const RESET_COMMAND_BUFFER_BIT = 2;
        const PROTECTED_BIT = 4;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolResetFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct CommandPoolResetFlags: u32 {
        const RELEASE_RESOURCES_BIT = 1;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryControlFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct QueryControlFlags: u32 {
        const PRECISE_BIT = 1;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferUsageFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct CommandBufferUsageFlags: u32 {
        const ONE_TIME_SUBMIT_BIT = 1;
        const RENDER_PASS_CONTINUE_BIT = 2;
        const SIMULTANEOUS_USE_BIT = 4;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferResetFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct CommandBufferResetFlags: u32 {
        const RELEASE_RESOURCES_BIT = 1;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkEventCreateFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct EventCreateFlags: u32 {
        const DEVICE_ONLY_BIT = 1;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCacheCreateFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PipelineCacheCreateFlags: u32 {
        const EXTERNALLY_SYNCHRONIZED_BIT = 1;
        const INTERNALLY_SYNCHRONIZED_MERGE_BIT_KHR = 8;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PipelineCreateFlags: u32 {
        const DISABLE_OPTIMIZATION_BIT = 1;
        const ALLOW_DERIVATIVES_BIT = 2;
        const DERIVATIVE_BIT = 4;
        const DISPATCH_BASE_BIT = 16;
        const VIEW_INDEX_FROM_DEVICE_INDEX_BIT = 8;
        const FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT = 256;
        const EARLY_RETURN_ON_FAILURE_BIT = 512;
        const NO_PROTECTED_ACCESS_BIT = 134217728;
        const PROTECTED_ACCESS_ONLY_BIT = 1073741824;
        const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR = 16384;
        const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR = 32768;
        const RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR = 65536;
        const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR = 131072;
        const RAY_TRACING_SKIP_TRIANGLES_BIT_KHR = 4096;
        const RAY_TRACING_SKIP_AABBS_BIT_KHR = 8192;
        const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR = 524288;
        const DEFER_COMPILE_BIT_NV = 32;
        const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT = 4194304;
        const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 2097152;
        const CAPTURE_STATISTICS_BIT_KHR = 64;
        const CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR = 128;
        const INDIRECT_BINDABLE_BIT_NV = 262144;
        const LIBRARY_BIT_KHR = 2048;
        const DESCRIPTOR_BUFFER_BIT_EXT = 536870912;
        const RETAIN_LINK_TIME_OPTIMIZATION_INFO_BIT_EXT = 8388608;
        const LINK_TIME_OPTIMIZATION_BIT_EXT = 1024;
        const RAY_TRACING_ALLOW_MOTION_BIT_NV = 1048576;
        const COLOR_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT = 33554432;
        const DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT = 67108864;
        const RAY_TRACING_DISPLACEMENT_MICROMAP_BIT_NV = 268435456;
        const RAY_TRACING_OPACITY_MICROMAP_BIT_KHR = 16777216;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineLayoutCreateFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PipelineLayoutCreateFlags: u32 {
        const INDEPENDENT_SETS_BIT_EXT = 2;
        const NO_TASK_SHADER_BIT_KHR = 4;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineShaderStageCreateFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PipelineShaderStageCreateFlags: u32 {
        const ALLOW_VARYING_SUBGROUP_SIZE_BIT = 1;
        const REQUIRE_FULL_SUBGROUPS_BIT = 2;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerCreateFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct SamplerCreateFlags: u32 {
        const SUBSAMPLED_BIT_EXT = 1;
        const SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT = 2;
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT = 8;
        const NON_SEAMLESS_CUBE_MAP_BIT_EXT = 4;
        const IMAGE_PROCESSING_BIT_QCOM = 16;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPoolCreateFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct DescriptorPoolCreateFlags: u32 {
        const FREE_DESCRIPTOR_SET_BIT = 1;
        const UPDATE_AFTER_BIND_BIT = 2;
        const HOST_ONLY_BIT_EXT = 4;
        const ALLOW_OVERALLOCATION_SETS_BIT_NV = 8;
        const ALLOW_OVERALLOCATION_POOLS_BIT_NV = 16;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayoutCreateFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct DescriptorSetLayoutCreateFlags: u32 {
        const UPDATE_AFTER_BIND_POOL_BIT = 2;
        const PUSH_DESCRIPTOR_BIT = 1;
        const DESCRIPTOR_BUFFER_BIT_EXT = 16;
        const EMBEDDED_IMMUTABLE_SAMPLERS_BIT_EXT = 32;
        const INDIRECT_BINDABLE_BIT_NV = 128;
        const HOST_ONLY_POOL_BIT_EXT = 4;
        const PER_STAGE_BIT_NV = 64;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkColorComponentFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ColorComponentFlags: u32 {
        const R_BIT = 1;
        const G_BIT = 2;
        const B_BIT = 4;
        const A_BIT = 8;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCullModeFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct CullModeFlags: u32 {
        const NONE = 0;
        const FRONT_BIT = 1;
        const BACK_BIT = 2;
        const FRONT_AND_BACK = 3;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineColorBlendStateCreateFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PipelineColorBlendStateCreateFlags: u32 {
        const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_EXT = 1;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineDepthStencilStateCreateFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PipelineDepthStencilStateCreateFlags: u32 {
        const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT = 1;
        const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT = 2;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentDescriptionFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct AttachmentDescriptionFlags: u32 {
        const MAY_ALIAS_BIT = 1;
        const RESOLVE_SKIP_TRANSFER_FUNCTION_BIT_KHR = 2;
        const RESOLVE_ENABLE_TRANSFER_FUNCTION_BIT_KHR = 4;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebufferCreateFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct FramebufferCreateFlags: u32 {
        const IMAGELESS_BIT = 1;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassCreateFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct RenderPassCreateFlags: u32 {
        const TRANSFORM_BIT_QCOM = 2;
        const PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE = 4;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassDescriptionFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct SubpassDescriptionFlags: u32 {
        const PER_VIEW_ATTRIBUTES_BIT_NVX = 1;
        const PER_VIEW_POSITION_X_ONLY_BIT_NVX = 2;
        const TILE_SHADING_APRON_BIT_QCOM = 256;
        const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_EXT = 16;
        const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT = 32;
        const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT = 64;
        const ENABLE_LEGACY_DITHERING_BIT_EXT = 128;
        const FRAGMENT_REGION_BIT_EXT = 4;
        const CUSTOM_RESOLVE_BIT_EXT = 8;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkStencilFaceFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct StencilFaceFlags: u32 {
        const FRONT_BIT = 1;
        const BACK_BIT = 2;
        const FRONT_AND_BACK = 3;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubgroupFeatureFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct SubgroupFeatureFlags: u32 {
        const BASIC_BIT = 1;
        const VOTE_BIT = 2;
        const ARITHMETIC_BIT = 4;
        const BALLOT_BIT = 8;
        const SHUFFLE_BIT = 16;
        const SHUFFLE_RELATIVE_BIT = 32;
        const CLUSTERED_BIT = 64;
        const QUAD_BIT = 128;
        const ROTATE_BIT = 512;
        const ROTATE_CLUSTERED_BIT = 1024;
        const PARTITIONED_BIT_EXT = 256;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPeerMemoryFeatureFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PeerMemoryFeatureFlags: u32 {
        const COPY_SRC_BIT = 1;
        const COPY_DST_BIT = 2;
        const GENERIC_SRC_BIT = 4;
        const GENERIC_DST_BIT = 8;
    }
}
pub type PeerMemoryFeatureFlagsKHR = PeerMemoryFeatureFlags;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryAllocateFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct MemoryAllocateFlags: u32 {
        const DEVICE_MASK_BIT = 1;
        const DEVICE_ADDRESS_BIT = 2;
        const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT = 4;
        const ZERO_INITIALIZE_BIT_EXT = 8;
    }
}
pub type MemoryAllocateFlagsKHR = MemoryAllocateFlags;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryHandleTypeFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ExternalMemoryHandleTypeFlags: u32 {
        const OPAQUE_FD_BIT = 1;
        const OPAQUE_WIN32_BIT = 2;
        const OPAQUE_WIN32_KMT_BIT = 4;
        const D3D11_TEXTURE_BIT = 8;
        const D3D11_TEXTURE_KMT_BIT = 16;
        const D3D12_HEAP_BIT = 32;
        const D3D12_RESOURCE_BIT = 64;
        const DMA_BUF_BIT_EXT = 512;
        const ANDROID_HARDWARE_BUFFER_BIT_ANDROID = 1024;
        const HOST_ALLOCATION_BIT_EXT = 128;
        const HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT = 256;
        const ZIRCON_VMO_BIT_FUCHSIA = 2048;
        const RDMA_ADDRESS_BIT_NV = 4096;
        const OH_NATIVE_BUFFER_BIT_OHOS = 32768;
        const SCREEN_BUFFER_BIT_QNX = 16384;
        const MTLBUFFER_BIT_EXT = 65536;
        const MTLTEXTURE_BIT_EXT = 131072;
        const MTLHEAP_BIT_EXT = 262144;
    }
}
pub type ExternalMemoryHandleTypeFlagsKHR = ExternalMemoryHandleTypeFlags;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryFeatureFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ExternalMemoryFeatureFlags: u32 {
        const DEDICATED_ONLY_BIT = 1;
        const EXPORTABLE_BIT = 2;
        const IMPORTABLE_BIT = 4;
    }
}
pub type ExternalMemoryFeatureFlagsKHR = ExternalMemoryFeatureFlags;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceHandleTypeFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ExternalFenceHandleTypeFlags: u32 {
        const OPAQUE_FD_BIT = 1;
        const OPAQUE_WIN32_BIT = 2;
        const OPAQUE_WIN32_KMT_BIT = 4;
        const SYNC_FD_BIT = 8;
    }
}
pub type ExternalFenceHandleTypeFlagsKHR = ExternalFenceHandleTypeFlags;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceFeatureFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ExternalFenceFeatureFlags: u32 {
        const EXPORTABLE_BIT = 1;
        const IMPORTABLE_BIT = 2;
    }
}
pub type ExternalFenceFeatureFlagsKHR = ExternalFenceFeatureFlags;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceImportFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct FenceImportFlags: u32 {
        const TEMPORARY_BIT = 1;
    }
}
pub type FenceImportFlagsKHR = FenceImportFlags;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreImportFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct SemaphoreImportFlags: u32 {
        const TEMPORARY_BIT = 1;
    }
}
pub type SemaphoreImportFlagsKHR = SemaphoreImportFlags;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreHandleTypeFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ExternalSemaphoreHandleTypeFlags: u32 {
        const OPAQUE_FD_BIT = 1;
        const OPAQUE_WIN32_BIT = 2;
        const OPAQUE_WIN32_KMT_BIT = 4;
        const D3D12_FENCE_BIT = 8;
        const SYNC_FD_BIT = 16;
        const ZIRCON_EVENT_BIT_FUCHSIA = 128;
    }
}
pub type ExternalSemaphoreHandleTypeFlagsKHR = ExternalSemaphoreHandleTypeFlags;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreFeatureFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ExternalSemaphoreFeatureFlags: u32 {
        const EXPORTABLE_BIT = 1;
        const IMPORTABLE_BIT = 2;
    }
}
pub type ExternalSemaphoreFeatureFlagsKHR = ExternalSemaphoreFeatureFlags;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkResolveModeFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ResolveModeFlags: u32 {
        const NONE = 0;
        const SAMPLE_ZERO_BIT = 1;
        const AVERAGE_BIT = 2;
        const MIN_BIT = 4;
        const MAX_BIT = 8;
        const EXTERNAL_FORMAT_DOWNSAMPLE_BIT_ANDROID = 16;
        const CUSTOM_BIT_EXT = 32;
    }
}
pub type ResolveModeFlagsKHR = ResolveModeFlags;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreWaitFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct SemaphoreWaitFlags: u32 {
        const ANY_BIT = 1;
    }
}
pub type SemaphoreWaitFlagsKHR = SemaphoreWaitFlags;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorBindingFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct DescriptorBindingFlags: u32 {
        const UPDATE_AFTER_BIND_BIT = 1;
        const UPDATE_UNUSED_WHILE_PENDING_BIT = 2;
        const PARTIALLY_BOUND_BIT = 4;
        const VARIABLE_DESCRIPTOR_COUNT_BIT = 8;
    }
}
pub type DescriptorBindingFlagsEXT = DescriptorBindingFlags;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkToolPurposeFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ToolPurposeFlags: u32 {
        const VALIDATION_BIT = 1;
        const PROFILING_BIT = 2;
        const TRACING_BIT = 4;
        const ADDITIONAL_FEATURES_BIT = 8;
        const MODIFYING_FEATURES_BIT = 16;
        const DEBUG_REPORTING_BIT_EXT = 32;
        const DEBUG_MARKERS_BIT_EXT = 64;
    }
}
pub type ToolPurposeFlagsEXT = ToolPurposeFlags;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlotCreateFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PrivateDataSlotCreateFlags: u32 {
        const BASE_OBJECT_HANDLE_BIT_NV = 1;
    }
}
pub type PrivateDataSlotCreateFlagsEXT = PrivateDataSlotCreateFlags;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineStageFlagBits2.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PipelineStageFlags2: u64 {
        const NONE = 0;
        const TOP_OF_PIPE_BIT = 1;
        const DRAW_INDIRECT_BIT = 2;
        const VERTEX_INPUT_BIT = 4;
        const VERTEX_SHADER_BIT = 8;
        const TESSELLATION_CONTROL_SHADER_BIT = 16;
        const TESSELLATION_EVALUATION_SHADER_BIT = 32;
        const GEOMETRY_SHADER_BIT = 64;
        const FRAGMENT_SHADER_BIT = 128;
        const EARLY_FRAGMENT_TESTS_BIT = 256;
        const LATE_FRAGMENT_TESTS_BIT = 512;
        const COLOR_ATTACHMENT_OUTPUT_BIT = 1024;
        const COMPUTE_SHADER_BIT = 2048;
        const ALL_TRANSFER_BIT = 4096;
        const BOTTOM_OF_PIPE_BIT = 8192;
        const HOST_BIT = 16384;
        const ALL_GRAPHICS_BIT = 32768;
        const ALL_COMMANDS_BIT = 65536;
        const COPY_BIT = 4294967296;
        const RESOLVE_BIT = 8589934592;
        const BLIT_BIT = 17179869184;
        const CLEAR_BIT = 34359738368;
        const INDEX_INPUT_BIT = 68719476736;
        const VERTEX_ATTRIBUTE_INPUT_BIT = 137438953472;
        const PRE_RASTERIZATION_SHADERS_BIT = 274877906944;
        const VIDEO_DECODE_BIT_KHR = 67108864;
        const VIDEO_ENCODE_BIT_KHR = 134217728;
        const TRANSFORM_FEEDBACK_BIT_EXT = 16777216;
        const CONDITIONAL_RENDERING_BIT_EXT = 262144;
        const COMMAND_PREPROCESS_BIT_EXT = 131072;
        const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 4194304;
        const ACCELERATION_STRUCTURE_BUILD_BIT_KHR = 33554432;
        const RAY_TRACING_SHADER_BIT_KHR = 2097152;
        const FRAGMENT_DENSITY_PROCESS_BIT_EXT = 8388608;
        const TASK_SHADER_BIT_EXT = 524288;
        const MESH_SHADER_BIT_EXT = 1048576;
        const SUBPASS_SHADER_BIT_HUAWEI = 549755813888;
        const INVOCATION_MASK_BIT_HUAWEI = 1099511627776;
        const ACCELERATION_STRUCTURE_COPY_BIT_KHR = 268435456;
        const MICROMAP_BUILD_BIT_EXT = 1073741824;
        const CLUSTER_CULLING_SHADER_BIT_HUAWEI = 2199023255552;
        const OPTICAL_FLOW_BIT_NV = 536870912;
        const CONVERT_COOPERATIVE_VECTOR_MATRIX_BIT_NV = 17592186044416;
        const DATA_GRAPH_BIT_ARM = 4398046511104;
        const COPY_INDIRECT_BIT_KHR = 70368744177664;
        const MEMORY_DECOMPRESSION_BIT_EXT = 35184372088832;
    }
}
pub type PipelineStageFlags2KHR = PipelineStageFlags2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlagBits2.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct AccessFlags2: u64 {
        const NONE = 0;
        const INDIRECT_COMMAND_READ_BIT = 1;
        const INDEX_READ_BIT = 2;
        const VERTEX_ATTRIBUTE_READ_BIT = 4;
        const UNIFORM_READ_BIT = 8;
        const INPUT_ATTACHMENT_READ_BIT = 16;
        const SHADER_READ_BIT = 32;
        const SHADER_WRITE_BIT = 64;
        const COLOR_ATTACHMENT_READ_BIT = 128;
        const COLOR_ATTACHMENT_WRITE_BIT = 256;
        const DEPTH_STENCIL_ATTACHMENT_READ_BIT = 512;
        const DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = 1024;
        const TRANSFER_READ_BIT = 2048;
        const TRANSFER_WRITE_BIT = 4096;
        const HOST_READ_BIT = 8192;
        const HOST_WRITE_BIT = 16384;
        const MEMORY_READ_BIT = 32768;
        const MEMORY_WRITE_BIT = 65536;
        const SHADER_SAMPLED_READ_BIT = 4294967296;
        const SHADER_STORAGE_READ_BIT = 8589934592;
        const SHADER_STORAGE_WRITE_BIT = 17179869184;
        const VIDEO_DECODE_READ_BIT_KHR = 34359738368;
        const VIDEO_DECODE_WRITE_BIT_KHR = 68719476736;
        const SAMPLER_HEAP_READ_BIT_EXT = 144115188075855872;
        const RESOURCE_HEAP_READ_BIT_EXT = 288230376151711744;
        const VIDEO_ENCODE_READ_BIT_KHR = 137438953472;
        const VIDEO_ENCODE_WRITE_BIT_KHR = 274877906944;
        const SHADER_TILE_ATTACHMENT_READ_BIT_QCOM = 2251799813685248;
        const SHADER_TILE_ATTACHMENT_WRITE_BIT_QCOM = 4503599627370496;
        const TRANSFORM_FEEDBACK_WRITE_BIT_EXT = 33554432;
        const TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT = 67108864;
        const TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT = 134217728;
        const CONDITIONAL_RENDERING_READ_BIT_EXT = 1048576;
        const COMMAND_PREPROCESS_READ_BIT_EXT = 131072;
        const COMMAND_PREPROCESS_WRITE_BIT_EXT = 262144;
        const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR = 8388608;
        const ACCELERATION_STRUCTURE_READ_BIT_KHR = 2097152;
        const ACCELERATION_STRUCTURE_WRITE_BIT_KHR = 4194304;
        const FRAGMENT_DENSITY_MAP_READ_BIT_EXT = 16777216;
        const COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT = 524288;
        const DESCRIPTOR_BUFFER_READ_BIT_EXT = 2199023255552;
        const INVOCATION_MASK_READ_BIT_HUAWEI = 549755813888;
        const SHADER_BINDING_TABLE_READ_BIT_KHR = 1099511627776;
        const MICROMAP_READ_BIT_EXT = 17592186044416;
        const MICROMAP_WRITE_BIT_EXT = 35184372088832;
        const OPTICAL_FLOW_READ_BIT_NV = 4398046511104;
        const OPTICAL_FLOW_WRITE_BIT_NV = 8796093022208;
        const DATA_GRAPH_READ_BIT_ARM = 140737488355328;
        const DATA_GRAPH_WRITE_BIT_ARM = 281474976710656;
        const MEMORY_DECOMPRESSION_READ_BIT_EXT = 36028797018963968;
        const MEMORY_DECOMPRESSION_WRITE_BIT_EXT = 72057594037927936;
    }
}
pub type AccessFlags2KHR = AccessFlags2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubmitFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct SubmitFlags: u32 {
        const PROTECTED_BIT = 1;
    }
}
pub type SubmitFlagsKHR = SubmitFlags;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlagBits2.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct FormatFeatureFlags2: u64 {
        const SAMPLED_IMAGE_BIT = 1;
        const STORAGE_IMAGE_BIT = 2;
        const STORAGE_IMAGE_ATOMIC_BIT = 4;
        const UNIFORM_TEXEL_BUFFER_BIT = 8;
        const STORAGE_TEXEL_BUFFER_BIT = 16;
        const STORAGE_TEXEL_BUFFER_ATOMIC_BIT = 32;
        const VERTEX_BUFFER_BIT = 64;
        const COLOR_ATTACHMENT_BIT = 128;
        const COLOR_ATTACHMENT_BLEND_BIT = 256;
        const DEPTH_STENCIL_ATTACHMENT_BIT = 512;
        const BLIT_SRC_BIT = 1024;
        const BLIT_DST_BIT = 2048;
        const SAMPLED_IMAGE_FILTER_LINEAR_BIT = 4096;
        const TRANSFER_SRC_BIT = 16384;
        const TRANSFER_DST_BIT = 32768;
        const SAMPLED_IMAGE_FILTER_MINMAX_BIT = 65536;
        const MIDPOINT_CHROMA_SAMPLES_BIT = 131072;
        const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT = 262144;
        const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT = 524288;
        const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT = 1048576;
        const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT = 2097152;
        const DISJOINT_BIT = 4194304;
        const COSITED_CHROMA_SAMPLES_BIT = 8388608;
        const STORAGE_READ_WITHOUT_FORMAT_BIT = 2147483648;
        const STORAGE_WRITE_WITHOUT_FORMAT_BIT = 4294967296;
        const SAMPLED_IMAGE_DEPTH_COMPARISON_BIT = 8589934592;
        const SAMPLED_IMAGE_FILTER_CUBIC_BIT = 8192;
        const HOST_IMAGE_TRANSFER_BIT = 70368744177664;
        const VIDEO_DECODE_OUTPUT_BIT_KHR = 33554432;
        const VIDEO_DECODE_DPB_BIT_KHR = 67108864;
        const ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR = 536870912;
        const FRAGMENT_DENSITY_MAP_BIT_EXT = 16777216;
        const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 1073741824;
        const VIDEO_ENCODE_INPUT_BIT_KHR = 134217728;
        const VIDEO_ENCODE_DPB_BIT_KHR = 268435456;
        const BLOCK_MATCHING_SXD_BIT_QCOM = 17592186044416;
        const ACCELERATION_STRUCTURE_RADIUS_BUFFER_BIT_NV = 2251799813685248;
        const LINEAR_COLOR_ATTACHMENT_BIT_NV = 274877906944;
        const WEIGHT_IMAGE_BIT_QCOM = 17179869184;
        const WEIGHT_SAMPLED_IMAGE_BIT_QCOM = 34359738368;
        const BLOCK_MATCHING_BIT_QCOM = 68719476736;
        const BOX_FILTER_SAMPLED_BIT_QCOM = 137438953472;
        const TENSOR_SHADER_BIT_ARM = 549755813888;
        const TENSOR_IMAGE_ALIASING_BIT_ARM = 8796093022208;
        const OPTICAL_FLOW_IMAGE_BIT_NV = 1099511627776;
        const OPTICAL_FLOW_VECTOR_BIT_NV = 2199023255552;
        const OPTICAL_FLOW_COST_BIT_NV = 4398046511104;
        const TENSOR_DATA_GRAPH_BIT_ARM = 281474976710656;
        const COPY_IMAGE_INDIRECT_DST_BIT_KHR = 576460752303423488;
        const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR = 562949953421312;
        const VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR = 1125899906842624;
        const SAMPLED_IMAGE_FILTER_LINEAR_2D_BIT_IMG = 35184372088832;
        const DEPTH_COPY_ON_COMPUTE_QUEUE_BIT_KHR = 4503599627370496;
        const DEPTH_COPY_ON_TRANSFER_QUEUE_BIT_KHR = 9007199254740992;
        const STENCIL_COPY_ON_COMPUTE_QUEUE_BIT_KHR = 18014398509481984;
        const STENCIL_COPY_ON_TRANSFER_QUEUE_BIT_KHR = 36028797018963968;
        const DATA_GRAPH_OPTICAL_FLOW_IMAGE_BIT_ARM = 72057594037927936;
        const DATA_GRAPH_OPTICAL_FLOW_VECTOR_BIT_ARM = 144115188075855872;
        const DATA_GRAPH_OPTICAL_FLOW_COST_BIT_ARM = 288230376151711744;
    }
}
pub type FormatFeatureFlags2KHR = FormatFeatureFlags2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreationFeedbackFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PipelineCreationFeedbackFlags: u32 {
        const VALID_BIT = 1;
        const APPLICATION_PIPELINE_CACHE_HIT_BIT = 2;
        const BASE_PIPELINE_ACCELERATION_BIT = 4;
    }
}
pub type PipelineCreationFeedbackFlagsEXT = PipelineCreationFeedbackFlags;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct RenderingFlags: u32 {
        const CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT = 1;
        const SUSPENDING_BIT = 2;
        const RESUMING_BIT = 4;
        const ENABLE_LEGACY_DITHERING_BIT_EXT = 8;
        const CONTENTS_INLINE_BIT_KHR = 16;
        const PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE = 32;
        const FRAGMENT_REGION_BIT_EXT = 64;
        const CUSTOM_RESOLVE_BIT_EXT = 128;
        const LOCAL_READ_CONCURRENT_ACCESS_CONTROL_BIT_KHR = 256;
    }
}
pub type RenderingFlagsKHR = RenderingFlags;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryUnmapFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct MemoryUnmapFlags: u32 {
        const RESERVE_BIT_EXT = 1;
    }
}
pub type MemoryUnmapFlagsKHR = MemoryUnmapFlags;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlagBits2.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct BufferUsageFlags2: u64 {
        const TRANSFER_SRC_BIT = 1;
        const TRANSFER_DST_BIT = 2;
        const UNIFORM_TEXEL_BUFFER_BIT = 4;
        const STORAGE_TEXEL_BUFFER_BIT = 8;
        const UNIFORM_BUFFER_BIT = 16;
        const STORAGE_BUFFER_BIT = 32;
        const INDEX_BUFFER_BIT = 64;
        const VERTEX_BUFFER_BIT = 128;
        const INDIRECT_BUFFER_BIT = 256;
        const SHADER_DEVICE_ADDRESS_BIT = 131072;
        const EXECUTION_GRAPH_SCRATCH_BIT_AMDX = 33554432;
        const DESCRIPTOR_HEAP_BIT_EXT = 268435456;
        const MICROMAP_BUILD_INPUT_READ_ONLY_BIT_EXT = 8388608;
        const MICROMAP_STORAGE_BIT_EXT = 16777216;
        const CONDITIONAL_RENDERING_BIT_EXT = 512;
        const SHADER_BINDING_TABLE_BIT_KHR = 1024;
        const TRANSFORM_FEEDBACK_BUFFER_BIT_EXT = 2048;
        const TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT = 4096;
        const VIDEO_DECODE_SRC_BIT_KHR = 8192;
        const VIDEO_DECODE_DST_BIT_KHR = 16384;
        const VIDEO_ENCODE_DST_BIT_KHR = 32768;
        const VIDEO_ENCODE_SRC_BIT_KHR = 65536;
        const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR = 524288;
        const ACCELERATION_STRUCTURE_STORAGE_BIT_KHR = 1048576;
        const SAMPLER_DESCRIPTOR_BUFFER_BIT_EXT = 2097152;
        const RESOURCE_DESCRIPTOR_BUFFER_BIT_EXT = 4194304;
        const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_BIT_EXT = 67108864;
        const COMPRESSED_DATA_DGF1_BIT_AMDX = 8589934592;
        const DATA_GRAPH_FOREIGN_DESCRIPTOR_BIT_ARM = 536870912;
        const TILE_MEMORY_BIT_QCOM = 134217728;
        const MEMORY_DECOMPRESSION_BIT_EXT = 4294967296;
        const PREPROCESS_BUFFER_BIT_EXT = 2147483648;
    }
}
pub type BufferUsageFlags2KHR = BufferUsageFlags2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkHostImageCopyFlagBits.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct HostImageCopyFlags: u32 {
        const MEMCPY_BIT = 1;
    }
}
pub type HostImageCopyFlagsEXT = HostImageCopyFlags;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlagBits2.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PipelineCreateFlags2: u64 {
        const DISABLE_OPTIMIZATION_BIT = 1;
        const ALLOW_DERIVATIVES_BIT = 2;
        const DERIVATIVE_BIT = 4;
        const VIEW_INDEX_FROM_DEVICE_INDEX_BIT = 8;
        const DISPATCH_BASE_BIT = 16;
        const FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT = 256;
        const EARLY_RETURN_ON_FAILURE_BIT = 512;
        const NO_PROTECTED_ACCESS_BIT = 134217728;
        const PROTECTED_ACCESS_ONLY_BIT = 1073741824;
        const EXECUTION_GRAPH_BIT_AMDX = 4294967296;
        const DESCRIPTOR_HEAP_BIT_EXT = 68719476736;
        const RAY_TRACING_ALLOW_SPHERES_AND_LINEAR_SWEPT_SPHERES_BIT_NV = 8589934592;
        const ENABLE_LEGACY_DITHERING_BIT_EXT = 17179869184;
        const DEFER_COMPILE_BIT_NV = 32;
        const CAPTURE_STATISTICS_BIT_KHR = 64;
        const CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR = 128;
        const LINK_TIME_OPTIMIZATION_BIT_EXT = 1024;
        const RETAIN_LINK_TIME_OPTIMIZATION_INFO_BIT_EXT = 8388608;
        const LIBRARY_BIT_KHR = 2048;
        const RAY_TRACING_SKIP_TRIANGLES_BIT_KHR = 4096;
        const RAY_TRACING_SKIP_AABBS_BIT_KHR = 8192;
        const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR = 16384;
        const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR = 32768;
        const RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR = 65536;
        const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR = 131072;
        const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR = 524288;
        const INDIRECT_BINDABLE_BIT_NV = 262144;
        const RAY_TRACING_ALLOW_MOTION_BIT_NV = 1048576;
        const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 2097152;
        const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT = 4194304;
        const COLOR_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT = 33554432;
        const DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT = 67108864;
        const RAY_TRACING_DISPLACEMENT_MICROMAP_BIT_NV = 268435456;
        const DESCRIPTOR_BUFFER_BIT_EXT = 536870912;
        const DISALLOW_OPACITY_MICROMAP_BIT_ARM = 137438953472;
        const INSTRUMENT_SHADERS_BIT_ARM = 549755813888;
        const CAPTURE_DATA_BIT_KHR = 2147483648;
        const INDIRECT_BINDABLE_BIT_EXT = 274877906944;
        const PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE = 1099511627776;
        const RAY_TRACING_OPACITY_MICROMAP_BIT_KHR = 16777216;
        const OPACITY_MICROMAP_DISALLOW_MIXED_SPECIAL_INDEX_BIT_KHR = 2199023255552;
        const _64_BIT_INDEXING_BIT_EXT = 8796093022208;
    }
}
pub type PipelineCreateFlags2KHR = PipelineCreateFlags2;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceTransformFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct SurfaceTransformFlagsKHR: u32 {
        const IDENTITY_BIT_KHR = 1;
        const ROTATE_90_BIT_KHR = 2;
        const ROTATE_180_BIT_KHR = 4;
        const ROTATE_270_BIT_KHR = 8;
        const HORIZONTAL_MIRROR_BIT_KHR = 16;
        const HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR = 32;
        const HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR = 64;
        const HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR = 128;
        const INHERIT_BIT_KHR = 256;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCompositeAlphaFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct CompositeAlphaFlagsKHR: u32 {
        const OPAQUE_BIT_KHR = 1;
        const PRE_MULTIPLIED_BIT_KHR = 2;
        const POST_MULTIPLIED_BIT_KHR = 4;
        const INHERIT_BIT_KHR = 8;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainCreateFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct SwapchainCreateFlagsKHR: u32 {
        const SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR = 1;
        const PROTECTED_BIT_KHR = 2;
        const MUTABLE_FORMAT_BIT_KHR = 4;
        const PRESENT_TIMING_BIT_EXT = 512;
        const PRESENT_ID_2_BIT_KHR = 64;
        const PRESENT_WAIT_2_BIT_KHR = 128;
        const DEFERRED_MEMORY_ALLOCATION_BIT_KHR = 8;
        const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT = 256;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupPresentModeFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct DeviceGroupPresentModeFlagsKHR: u32 {
        const LOCAL_BIT_KHR = 1;
        const REMOTE_BIT_KHR = 2;
        const SUM_BIT_KHR = 4;
        const LOCAL_MULTI_DEVICE_BIT_KHR = 8;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayPlaneAlphaFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct DisplayPlaneAlphaFlagsKHR: u32 {
        const OPAQUE_BIT_KHR = 1;
        const GLOBAL_BIT_KHR = 2;
        const PER_PIXEL_BIT_KHR = 4;
        const PER_PIXEL_PREMULTIPLIED_BIT_KHR = 8;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoCodecOperationFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoCodecOperationFlagsKHR: u32 {
        const NONE_KHR = 0;
        const ENCODE_H264_BIT_KHR = 65536;
        const ENCODE_H265_BIT_KHR = 131072;
        const DECODE_H264_BIT_KHR = 1;
        const DECODE_H265_BIT_KHR = 2;
        const DECODE_AV1_BIT_KHR = 4;
        const ENCODE_AV1_BIT_KHR = 262144;
        const DECODE_VP9_BIT_KHR = 8;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoChromaSubsamplingFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoChromaSubsamplingFlagsKHR: u32 {
        const INVALID_KHR = 0;
        const MONOCHROME_BIT_KHR = 1;
        const _420_BIT_KHR = 2;
        const _422_BIT_KHR = 4;
        const _444_BIT_KHR = 8;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoComponentBitDepthFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoComponentBitDepthFlagsKHR: u32 {
        const INVALID_KHR = 0;
        const _8_BIT_KHR = 1;
        const _10_BIT_KHR = 4;
        const _12_BIT_KHR = 16;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoCapabilityFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoCapabilityFlagsKHR: u32 {
        const PROTECTED_CONTENT_BIT_KHR = 1;
        const SEPARATE_REFERENCE_IMAGES_BIT_KHR = 2;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionCreateFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoSessionCreateFlagsKHR: u32 {
        const PROTECTED_CONTENT_BIT_KHR = 1;
        const ALLOW_ENCODE_PARAMETER_OPTIMIZATIONS_BIT_KHR = 2;
        const INLINE_QUERIES_BIT_KHR = 4;
        const ALLOW_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR = 8;
        const ALLOW_ENCODE_EMPHASIS_MAP_BIT_KHR = 16;
        const INLINE_SESSION_PARAMETERS_BIT_KHR = 32;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionParametersCreateFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoSessionParametersCreateFlagsKHR: u32 {
        const QUANTIZATION_MAP_COMPATIBLE_BIT_KHR = 1;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoCodingControlFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoCodingControlFlagsKHR: u32 {
        const RESET_BIT_KHR = 1;
        const ENCODE_RATE_CONTROL_BIT_KHR = 2;
        const ENCODE_QUALITY_LEVEL_BIT_KHR = 4;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeCapabilityFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoDecodeCapabilityFlagsKHR: u32 {
        const DPB_AND_OUTPUT_COINCIDE_BIT_KHR = 1;
        const DPB_AND_OUTPUT_DISTINCT_BIT_KHR = 2;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeUsageFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoDecodeUsageFlagsKHR: u32 {
        const DEFAULT_KHR = 0;
        const TRANSCODING_BIT_KHR = 1;
        const OFFLINE_BIT_KHR = 2;
        const STREAMING_BIT_KHR = 4;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264CapabilityFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoEncodeH264CapabilityFlagsKHR: u32 {
        const VIDEO_ENCODE_H264_CAPABILITY_HRD_COMPLIANCE_BIT_KHR = 1;
        const VIDEO_ENCODE_H264_CAPABILITY_PREDICTION_WEIGHT_TABLE_GENERATED_BIT_KHR = 2;
        const VIDEO_ENCODE_H264_CAPABILITY_ROW_UNALIGNED_SLICE_BIT_KHR = 4;
        const VIDEO_ENCODE_H264_CAPABILITY_DIFFERENT_SLICE_TYPE_BIT_KHR = 8;
        const VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L0_LIST_BIT_KHR = 16;
        const VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L1_LIST_BIT_KHR = 32;
        const VIDEO_ENCODE_H264_CAPABILITY_PER_PICTURE_TYPE_MIN_MAX_QP_BIT_KHR = 64;
        const VIDEO_ENCODE_H264_CAPABILITY_PER_SLICE_CONSTANT_QP_BIT_KHR = 128;
        const VIDEO_ENCODE_H264_CAPABILITY_GENERATE_PREFIX_NALU_BIT_KHR = 256;
        const VIDEO_ENCODE_H264_CAPABILITY_B_PICTURE_INTRA_REFRESH_BIT_KHR = 1024;
        const VIDEO_ENCODE_H264_CAPABILITY_MB_QP_DIFF_WRAPAROUND_BIT_KHR = 512;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264StdFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoEncodeH264StdFlagsKHR: u32 {
        const VIDEO_ENCODE_H264_STD_SEPARATE_COLOR_PLANE_FLAG_SET_BIT_KHR = 1;
        const VIDEO_ENCODE_H264_STD_QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_BIT_KHR = 2;
        const VIDEO_ENCODE_H264_STD_SCALING_MATRIX_PRESENT_FLAG_SET_BIT_KHR = 4;
        const VIDEO_ENCODE_H264_STD_CHROMA_QP_INDEX_OFFSET_BIT_KHR = 8;
        const VIDEO_ENCODE_H264_STD_SECOND_CHROMA_QP_INDEX_OFFSET_BIT_KHR = 16;
        const VIDEO_ENCODE_H264_STD_PIC_INIT_QP_MINUS26_BIT_KHR = 32;
        const VIDEO_ENCODE_H264_STD_WEIGHTED_PRED_FLAG_SET_BIT_KHR = 64;
        const VIDEO_ENCODE_H264_STD_WEIGHTED_BIPRED_IDC_EXPLICIT_BIT_KHR = 128;
        const VIDEO_ENCODE_H264_STD_WEIGHTED_BIPRED_IDC_IMPLICIT_BIT_KHR = 256;
        const VIDEO_ENCODE_H264_STD_TRANSFORM_8X8_MODE_FLAG_SET_BIT_KHR = 512;
        const VIDEO_ENCODE_H264_STD_DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_BIT_KHR = 1024;
        const VIDEO_ENCODE_H264_STD_ENTROPY_CODING_MODE_FLAG_UNSET_BIT_KHR = 2048;
        const VIDEO_ENCODE_H264_STD_ENTROPY_CODING_MODE_FLAG_SET_BIT_KHR = 4096;
        const VIDEO_ENCODE_H264_STD_DIRECT_8X8_INFERENCE_FLAG_UNSET_BIT_KHR = 8192;
        const VIDEO_ENCODE_H264_STD_CONSTRAINED_INTRA_PRED_FLAG_SET_BIT_KHR = 16384;
        const VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_DISABLED_BIT_KHR = 32768;
        const VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_ENABLED_BIT_KHR = 65536;
        const VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_PARTIAL_BIT_KHR = 131072;
        const VIDEO_ENCODE_H264_STD_SLICE_QP_DELTA_BIT_KHR = 524288;
        const VIDEO_ENCODE_H264_STD_DIFFERENT_SLICE_QP_DELTA_BIT_KHR = 1048576;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264RateControlFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoEncodeH264RateControlFlagsKHR: u32 {
        const VIDEO_ENCODE_H264_RATE_CONTROL_ATTEMPT_HRD_COMPLIANCE_BIT_KHR = 1;
        const VIDEO_ENCODE_H264_RATE_CONTROL_REGULAR_GOP_BIT_KHR = 2;
        const VIDEO_ENCODE_H264_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR = 4;
        const VIDEO_ENCODE_H264_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR = 8;
        const VIDEO_ENCODE_H264_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_BIT_KHR = 16;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265CapabilityFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoEncodeH265CapabilityFlagsKHR: u32 {
        const VIDEO_ENCODE_H265_CAPABILITY_HRD_COMPLIANCE_BIT_KHR = 1;
        const VIDEO_ENCODE_H265_CAPABILITY_PREDICTION_WEIGHT_TABLE_GENERATED_BIT_KHR = 2;
        const VIDEO_ENCODE_H265_CAPABILITY_ROW_UNALIGNED_SLICE_SEGMENT_BIT_KHR = 4;
        const VIDEO_ENCODE_H265_CAPABILITY_DIFFERENT_SLICE_SEGMENT_TYPE_BIT_KHR = 8;
        const VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L0_LIST_BIT_KHR = 16;
        const VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L1_LIST_BIT_KHR = 32;
        const VIDEO_ENCODE_H265_CAPABILITY_PER_PICTURE_TYPE_MIN_MAX_QP_BIT_KHR = 64;
        const VIDEO_ENCODE_H265_CAPABILITY_PER_SLICE_SEGMENT_CONSTANT_QP_BIT_KHR = 128;
        const VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_TILES_PER_SLICE_SEGMENT_BIT_KHR = 256;
        const VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_SLICE_SEGMENTS_PER_TILE_BIT_KHR = 512;
        const VIDEO_ENCODE_H265_CAPABILITY_B_PICTURE_INTRA_REFRESH_BIT_KHR = 2048;
        const VIDEO_ENCODE_H265_CAPABILITY_CU_QP_DIFF_WRAPAROUND_BIT_KHR = 1024;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265StdFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoEncodeH265StdFlagsKHR: u32 {
        const VIDEO_ENCODE_H265_STD_SEPARATE_COLOR_PLANE_FLAG_SET_BIT_KHR = 1;
        const VIDEO_ENCODE_H265_STD_SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET_BIT_KHR = 2;
        const VIDEO_ENCODE_H265_STD_SCALING_LIST_DATA_PRESENT_FLAG_SET_BIT_KHR = 4;
        const VIDEO_ENCODE_H265_STD_PCM_ENABLED_FLAG_SET_BIT_KHR = 8;
        const VIDEO_ENCODE_H265_STD_SPS_TEMPORAL_MVP_ENABLED_FLAG_SET_BIT_KHR = 16;
        const VIDEO_ENCODE_H265_STD_INIT_QP_MINUS26_BIT_KHR = 32;
        const VIDEO_ENCODE_H265_STD_WEIGHTED_PRED_FLAG_SET_BIT_KHR = 64;
        const VIDEO_ENCODE_H265_STD_WEIGHTED_BIPRED_FLAG_SET_BIT_KHR = 128;
        const VIDEO_ENCODE_H265_STD_LOG2_PARALLEL_MERGE_LEVEL_MINUS2_BIT_KHR = 256;
        const VIDEO_ENCODE_H265_STD_SIGN_DATA_HIDING_ENABLED_FLAG_SET_BIT_KHR = 512;
        const VIDEO_ENCODE_H265_STD_TRANSFORM_SKIP_ENABLED_FLAG_SET_BIT_KHR = 1024;
        const VIDEO_ENCODE_H265_STD_TRANSFORM_SKIP_ENABLED_FLAG_UNSET_BIT_KHR = 2048;
        const VIDEO_ENCODE_H265_STD_PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET_BIT_KHR = 4096;
        const VIDEO_ENCODE_H265_STD_TRANSQUANT_BYPASS_ENABLED_FLAG_SET_BIT_KHR = 8192;
        const VIDEO_ENCODE_H265_STD_CONSTRAINED_INTRA_PRED_FLAG_SET_BIT_KHR = 16384;
        const VIDEO_ENCODE_H265_STD_ENTROPY_CODING_SYNC_ENABLED_FLAG_SET_BIT_KHR = 32768;
        const VIDEO_ENCODE_H265_STD_DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET_BIT_KHR = 65536;
        const VIDEO_ENCODE_H265_STD_DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET_BIT_KHR = 131072;
        const VIDEO_ENCODE_H265_STD_DEPENDENT_SLICE_SEGMENT_FLAG_SET_BIT_KHR = 262144;
        const VIDEO_ENCODE_H265_STD_SLICE_QP_DELTA_BIT_KHR = 524288;
        const VIDEO_ENCODE_H265_STD_DIFFERENT_SLICE_QP_DELTA_BIT_KHR = 1048576;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265CtbSizeFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoEncodeH265CtbSizeFlagsKHR: u32 {
        const VIDEO_ENCODE_H265_CTB_SIZE_16_BIT_KHR = 1;
        const VIDEO_ENCODE_H265_CTB_SIZE_32_BIT_KHR = 2;
        const VIDEO_ENCODE_H265_CTB_SIZE_64_BIT_KHR = 4;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265TransformBlockSizeFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoEncodeH265TransformBlockSizeFlagsKHR: u32 {
        const VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_4_BIT_KHR = 1;
        const VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_8_BIT_KHR = 2;
        const VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_16_BIT_KHR = 4;
        const VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_32_BIT_KHR = 8;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265RateControlFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoEncodeH265RateControlFlagsKHR: u32 {
        const VIDEO_ENCODE_H265_RATE_CONTROL_ATTEMPT_HRD_COMPLIANCE_BIT_KHR = 1;
        const VIDEO_ENCODE_H265_RATE_CONTROL_REGULAR_GOP_BIT_KHR = 2;
        const VIDEO_ENCODE_H265_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR = 4;
        const VIDEO_ENCODE_H265_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR = 8;
        const VIDEO_ENCODE_H265_RATE_CONTROL_TEMPORAL_SUB_LAYER_PATTERN_DYADIC_BIT_KHR = 16;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeH264PictureLayoutFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoDecodeH264PictureLayoutFlagsKHR: u32 {
        const VIDEO_DECODE_H264_PICTURE_LAYOUT_PROGRESSIVE_KHR = 0;
        const VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_INTERLEAVED_LINES_BIT_KHR = 1;
        const VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_SEPARATE_PLANES_BIT_KHR = 2;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterDescriptionFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PerformanceCounterDescriptionFlagsKHR: u32 {
        const PERFORMANCE_IMPACTING_BIT_KHR = 1;
        const CONCURRENTLY_IMPACTED_BIT_KHR = 2;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAcquireProfilingLockFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct AcquireProfilingLockFlagsKHR: u32 {
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoEncodeFlagsKHR: u32 {
        const INTRA_REFRESH_BIT_KHR = 4;
        const WITH_QUANTIZATION_DELTA_MAP_BIT_KHR = 1;
        const WITH_EMPHASIS_MAP_BIT_KHR = 2;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeCapabilityFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoEncodeCapabilityFlagsKHR: u32 {
        const PRECEDING_EXTERNALLY_ENCODED_BYTES_BIT_KHR = 1;
        const INSUFFICIENT_BITSTREAM_BUFFER_RANGE_DETECTION_BIT_KHR = 2;
        const QUANTIZATION_DELTA_MAP_BIT_KHR = 4;
        const EMPHASIS_MAP_BIT_KHR = 8;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRateControlModeFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoEncodeRateControlModeFlagsKHR: u32 {
        const DEFAULT_KHR = 0;
        const DISABLED_BIT_KHR = 1;
        const CBR_BIT_KHR = 2;
        const VBR_BIT_KHR = 4;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeFeedbackFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoEncodeFeedbackFlagsKHR: u32 {
        const BITSTREAM_BUFFER_OFFSET_BIT_KHR = 1;
        const BITSTREAM_BYTES_WRITTEN_BIT_KHR = 2;
        const BITSTREAM_HAS_OVERRIDES_BIT_KHR = 4;
        const AVERAGE_QUANTIZATION_BIT_KHR = 8;
        const MIN_QUANTIZATION_BIT_KHR = 16;
        const MAX_QUANTIZATION_BIT_KHR = 32;
        const INTRA_PIXELS_BIT_KHR = 64;
        const INTER_PIXELS_BIT_KHR = 128;
        const SKIPPED_PIXELS_BIT_KHR = 256;
        const PICTURE_PARTITION_COUNT_BIT_KHR = 512;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeUsageFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoEncodeUsageFlagsKHR: u32 {
        const DEFAULT_KHR = 0;
        const TRANSCODING_BIT_KHR = 1;
        const STREAMING_BIT_KHR = 2;
        const RECORDING_BIT_KHR = 4;
        const CONFERENCING_BIT_KHR = 8;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeContentFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoEncodeContentFlagsKHR: u32 {
        const DEFAULT_KHR = 0;
        const CAMERA_BIT_KHR = 1;
        const DESKTOP_BIT_KHR = 2;
        const RENDERED_BIT_KHR = 4;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAddressCommandFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct AddressCommandFlagsKHR: u32 {
        const PROTECTED_BIT_KHR = 1;
        const FULLY_BOUND_BIT_KHR = 2;
        const STORAGE_BUFFER_USAGE_BIT_KHR = 4;
        const UNKNOWN_STORAGE_BUFFER_USAGE_BIT_KHR = 8;
        const TRANSFORM_FEEDBACK_BUFFER_USAGE_BIT_KHR = 16;
        const UNKNOWN_TRANSFORM_FEEDBACK_BUFFER_USAGE_BIT_KHR = 32;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkConditionalRenderingFlagBitsEXT.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ConditionalRenderingFlagsEXT: u32 {
        const INVERTED_BIT_EXT = 1;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureCreateFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct AccelerationStructureCreateFlagsKHR: u32 {
        const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR = 1;
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT = 8;
        const MOTION_BIT_NV = 4;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentScalingFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PresentScalingFlagsKHR: u32 {
        const ONE_TO_ONE_BIT_KHR = 1;
        const ASPECT_RATIO_STRETCH_BIT_KHR = 2;
        const STRETCH_BIT_KHR = 4;
    }
}
pub type PresentScalingFlagsEXT = PresentScalingFlagsKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentGravityFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PresentGravityFlagsKHR: u32 {
        const MIN_BIT_KHR = 1;
        const MAX_BIT_KHR = 2;
        const CENTERED_BIT_KHR = 4;
    }
}
pub type PresentGravityFlagsEXT = PresentGravityFlagsKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1CapabilityFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoEncodeAV1CapabilityFlagsKHR: u32 {
        const VIDEO_ENCODE_AV1_CAPABILITY_PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_BIT_KHR = 1;
        const VIDEO_ENCODE_AV1_CAPABILITY_GENERATE_OBU_EXTENSION_HEADER_BIT_KHR = 2;
        const VIDEO_ENCODE_AV1_CAPABILITY_PRIMARY_REFERENCE_CDF_ONLY_BIT_KHR = 4;
        const VIDEO_ENCODE_AV1_CAPABILITY_FRAME_SIZE_OVERRIDE_BIT_KHR = 8;
        const VIDEO_ENCODE_AV1_CAPABILITY_MOTION_VECTOR_SCALING_BIT_KHR = 16;
        const VIDEO_ENCODE_AV1_CAPABILITY_COMPOUND_PREDICTION_INTRA_REFRESH_BIT_KHR = 32;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1StdFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoEncodeAV1StdFlagsKHR: u32 {
        const VIDEO_ENCODE_AV1_STD_UNIFORM_TILE_SPACING_FLAG_SET_BIT_KHR = 1;
        const VIDEO_ENCODE_AV1_STD_SKIP_MODE_PRESENT_UNSET_BIT_KHR = 2;
        const VIDEO_ENCODE_AV1_STD_PRIMARY_REF_FRAME_BIT_KHR = 4;
        const VIDEO_ENCODE_AV1_STD_DELTA_Q_BIT_KHR = 8;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1SuperblockSizeFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoEncodeAV1SuperblockSizeFlagsKHR: u32 {
        const VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_64_BIT_KHR = 1;
        const VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_128_BIT_KHR = 2;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1RateControlFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoEncodeAV1RateControlFlagsKHR: u32 {
        const VIDEO_ENCODE_AV1_RATE_CONTROL_REGULAR_GOP_BIT_KHR = 1;
        const VIDEO_ENCODE_AV1_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_BIT_KHR = 2;
        const VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR = 4;
        const VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR = 8;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAddressCopyFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct AddressCopyFlagsKHR: u32 {
        const DEVICE_LOCAL_BIT_KHR = 1;
        const SPARSE_BIT_KHR = 2;
        const PROTECTED_BIT_KHR = 4;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeIntraRefreshModeFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoEncodeIntraRefreshModeFlagsKHR: u32 {
        const NONE_KHR = 0;
        const PER_PICTURE_PARTITION_BIT_KHR = 1;
        const BLOCK_BASED_BIT_KHR = 2;
        const BLOCK_ROW_BASED_BIT_KHR = 4;
        const BLOCK_COLUMN_BASED_BIT_KHR = 8;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceFaultFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct DeviceFaultFlagsKHR: u32 {
        const FLAG_DEVICE_LOST_KHR = 1;
        const FLAG_MEMORY_ADDRESS_KHR = 2;
        const FLAG_INSTRUCTION_ADDRESS_KHR = 4;
        const FLAG_VENDOR_KHR = 8;
        const FLAG_WATCHDOG_TIMEOUT_KHR = 16;
        const FLAG_OVERFLOW_KHR = 32;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlagBits3KHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct AccessFlags3KHR: u64 {
        const NONE_KHR = 0;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodePerPartitionFeedbackFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoEncodePerPartitionFeedbackFlagsKHR: u32 {
        const STATUS_BIT_KHR = 1;
        const BITSTREAM_BUFFER_OFFSET_BIT_KHR = 2;
        const BITSTREAM_BYTES_WRITTEN_BIT_KHR = 4;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingAttachmentFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct RenderingAttachmentFlagsKHR: u32 {
        const INPUT_ATTACHMENT_FEEDBACK_BIT_KHR = 1;
        const RESOLVE_SKIP_TRANSFER_FUNCTION_BIT_KHR = 2;
        const RESOLVE_ENABLE_TRANSFER_FUNCTION_BIT_KHR = 4;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkResolveImageFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ResolveImageFlagsKHR: u32 {
        const SKIP_TRANSFER_FUNCTION_BIT_KHR = 1;
        const ENABLE_TRANSFER_FUNCTION_BIT_KHR = 2;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlagBits4KHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct FormatFeatureFlags4KHR: u64 {
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageUsageFlagBits2KHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ImageUsageFlags2KHR: u64 {
        const TRANSFER_SRC_BIT_KHR = 1;
        const TRANSFER_DST_BIT_KHR = 2;
        const SAMPLED_BIT_KHR = 4;
        const STORAGE_BIT_KHR = 8;
        const COLOR_ATTACHMENT_BIT_KHR = 16;
        const DEPTH_STENCIL_ATTACHMENT_BIT_KHR = 32;
        const TRANSIENT_ATTACHMENT_BIT_KHR = 64;
        const INPUT_ATTACHMENT_BIT_KHR = 128;
        const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 256;
        const FRAGMENT_DENSITY_MAP_BIT_EXT = 512;
        const VIDEO_DECODE_DST_BIT_KHR = 1024;
        const VIDEO_DECODE_SRC_BIT_KHR = 2048;
        const VIDEO_DECODE_DPB_BIT_KHR = 4096;
        const VIDEO_ENCODE_DST_BIT_KHR = 8192;
        const VIDEO_ENCODE_SRC_BIT_KHR = 16384;
        const VIDEO_ENCODE_DPB_BIT_KHR = 32768;
        const INVOCATION_MASK_BIT_HUAWEI = 262144;
        const ATTACHMENT_FEEDBACK_LOOP_BIT_EXT = 524288;
        const SAMPLE_WEIGHT_BIT_QCOM = 1048576;
        const SAMPLE_BLOCK_MATCH_BIT_QCOM = 2097152;
        const HOST_TRANSFER_BIT_KHR = 4194304;
        const TENSOR_ALIASING_BIT_ARM = 8388608;
        const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR = 33554432;
        const VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR = 67108864;
        const TILE_MEMORY_BIT_QCOM = 134217728;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCreateFlagBits2KHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ImageCreateFlags2KHR: u64 {
        const SPARSE_BINDING_BIT_KHR = 1;
        const SPARSE_RESIDENCY_BIT_KHR = 2;
        const SPARSE_ALIASED_BIT_KHR = 4;
        const MUTABLE_FORMAT_BIT_KHR = 8;
        const CUBE_COMPATIBLE_BIT_KHR = 16;
        const ALIAS_SINGLE_LAYER_DESCRIPTOR_BIT_KHR = 4194304;
        const _2D_ARRAY_COMPATIBLE_BIT_KHR = 32;
        const SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR = 64;
        const BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR = 128;
        const EXTENDED_USAGE_BIT_KHR = 256;
        const DISJOINT_BIT_KHR = 512;
        const ALIAS_BIT_KHR = 1024;
        const PROTECTED_BIT_KHR = 2048;
        const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT = 4096;
        const CORNER_SAMPLED_BIT_NV = 8192;
        const SUBSAMPLED_BIT_EXT = 16384;
        const FRAGMENT_DENSITY_MAP_OFFSET_BIT_EXT = 32768;
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT = 65536;
        const _2D_VIEW_COMPATIBLE_BIT_EXT = 131072;
        const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT = 262144;
        const VIDEO_PROFILE_INDEPENDENT_BIT_KHR = 1048576;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugReportFlagBitsEXT.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct DebugReportFlagsEXT: u32 {
        const INFORMATION_BIT_EXT = 1;
        const WARNING_BIT_EXT = 2;
        const PERFORMANCE_WARNING_BIT_EXT = 4;
        const ERROR_BIT_EXT = 8;
        const DEBUG_BIT_EXT = 16;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryHandleTypeFlagBitsNV.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ExternalMemoryHandleTypeFlagsNV: u32 {
        const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV = 1;
        const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV = 2;
        const EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV = 4;
        const EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV = 8;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryFeatureFlagBitsNV.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ExternalMemoryFeatureFlagsNV: u32 {
        const EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV = 1;
        const EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV = 2;
        const EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV = 4;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCounterFlagBitsEXT.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct SurfaceCounterFlagsEXT: u32 {
        const VBLANK_BIT_EXT = 1;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessageSeverityFlagBitsEXT.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct DebugUtilsMessageSeverityFlagsEXT: u32 {
        const VERBOSE_BIT_EXT = 1;
        const INFO_BIT_EXT = 16;
        const WARNING_BIT_EXT = 256;
        const ERROR_BIT_EXT = 4096;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessageTypeFlagBitsEXT.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct DebugUtilsMessageTypeFlagsEXT: u32 {
        const GENERAL_BIT_EXT = 1;
        const VALIDATION_BIT_EXT = 2;
        const PERFORMANCE_BIT_EXT = 4;
        const DEVICE_ADDRESS_BINDING_BIT_EXT = 8;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaSqShaderStageFlagBitsAMD.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct GpaSqShaderStageFlagsAMD: u32 {
        const PS_BIT_AMD = 1;
        const VS_BIT_AMD = 2;
        const GS_BIT_AMD = 4;
        const ES_BIT_AMD = 8;
        const HS_BIT_AMD = 16;
        const LS_BIT_AMD = 32;
        const CS_BIT_AMD = 64;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorViewCreateFlagBitsARM.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct TensorViewCreateFlagsARM: u64 {
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_ARM = 1;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSpirvResourceTypeFlagBitsEXT.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct SpirvResourceTypeFlagsEXT: u32 {
        const ALL_EXT = 2147483647;
        const SAMPLER_BIT_EXT = 1;
        const SAMPLED_IMAGE_BIT_EXT = 2;
        const READ_ONLY_IMAGE_BIT_EXT = 4;
        const READ_WRITE_IMAGE_BIT_EXT = 8;
        const COMBINED_SAMPLED_IMAGE_BIT_EXT = 16;
        const UNIFORM_BUFFER_BIT_EXT = 32;
        const READ_ONLY_STORAGE_BUFFER_BIT_EXT = 64;
        const READ_WRITE_STORAGE_BUFFER_BIT_EXT = 128;
        const ACCELERATION_STRUCTURE_BIT_EXT = 256;
        const TENSOR_BIT_ARM = 512;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct GeometryFlagsKHR: u32 {
        const OPAQUE_BIT_KHR = 1;
        const NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR = 2;
    }
}
pub type GeometryFlagsNV = GeometryFlagsKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryInstanceFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct GeometryInstanceFlagsKHR: u32 {
        const TRIANGLE_FACING_CULL_DISABLE_BIT_KHR = 1;
        const TRIANGLE_FLIP_FACING_BIT_KHR = 2;
        const FORCE_OPAQUE_BIT_KHR = 4;
        const FORCE_NO_OPAQUE_BIT_KHR = 8;
        const FORCE_OPACITY_MICROMAP_2_STATE_BIT_KHR = 16;
        const DISABLE_OPACITY_MICROMAPS_BIT_KHR = 32;
    }
}
pub type GeometryInstanceFlagsNV = GeometryInstanceFlagsKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildAccelerationStructureFlagBitsKHR.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct BuildAccelerationStructureFlagsKHR: u32 {
        const ALLOW_UPDATE_BIT_KHR = 1;
        const ALLOW_COMPACTION_BIT_KHR = 2;
        const PREFER_FAST_TRACE_BIT_KHR = 4;
        const PREFER_FAST_BUILD_BIT_KHR = 8;
        const LOW_MEMORY_BIT_KHR = 16;
        const MOTION_BIT_NV = 32;
        const ALLOW_OPACITY_MICROMAP_DATA_UPDATE_BIT_EXT = 256;
        const ALLOW_DISPLACEMENT_MICROMAP_UPDATE_BIT_NV = 512;
        const ALLOW_DATA_ACCESS_BIT_KHR = 2048;
        const ALLOW_CLUSTER_OPACITY_MICROMAPS_BIT_NV = 4096;
        const ALLOW_OPACITY_MICROMAP_UPDATE_BIT_KHR = 64;
        const ALLOW_DISABLE_OPACITY_MICROMAPS_BIT_KHR = 128;
        const MICROMAP_LOSSY_BIT_KHR = 1024;
    }
}
pub type BuildAccelerationStructureFlagsNV = BuildAccelerationStructureFlagsKHR;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCompilerControlFlagBitsAMD.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PipelineCompilerControlFlagsAMD: u32 {
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentStageFlagBitsEXT.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PresentStageFlagsEXT: u32 {
        const QUEUE_OPERATIONS_END_BIT_EXT = 1;
        const REQUEST_DEQUEUED_BIT_EXT = 2;
        const IMAGE_FIRST_PIXEL_OUT_BIT_EXT = 4;
        const IMAGE_FIRST_PIXEL_VISIBLE_BIT_EXT = 8;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPastPresentationTimingFlagBitsEXT.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PastPresentationTimingFlagsEXT: u32 {
        const ALLOW_PARTIAL_RESULTS_BIT_EXT = 1;
        const ALLOW_OUT_OF_ORDER_RESULTS_BIT_EXT = 2;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentTimingInfoFlagBitsEXT.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PresentTimingInfoFlagsEXT: u32 {
        const PRESENT_AT_RELATIVE_TIME_BIT_EXT = 1;
        const PRESENT_AT_NEAREST_REFRESH_CYCLE_BIT_EXT = 2;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderCorePropertiesFlagBitsAMD.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ShaderCorePropertiesFlagsAMD: u32 {
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectStateFlagBitsNV.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct IndirectStateFlagsNV: u32 {
        const INDIRECT_STATE_FLAG_FRONTFACE_BIT_NV = 1;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutUsageFlagBitsNV.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct IndirectCommandsLayoutUsageFlagsNV: u32 {
        const INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV = 1;
        const INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV = 2;
        const INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV = 4;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceDiagnosticsConfigFlagBitsNV.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct DeviceDiagnosticsConfigFlagsNV: u32 {
        const DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_DEBUG_INFO_BIT_NV = 1;
        const DEVICE_DIAGNOSTICS_CONFIG_ENABLE_RESOURCE_TRACKING_BIT_NV = 2;
        const DEVICE_DIAGNOSTICS_CONFIG_ENABLE_AUTOMATIC_CHECKPOINTS_BIT_NV = 4;
        const DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_ERROR_REPORTING_BIT_NV = 8;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTileShadingRenderPassFlagBitsQCOM.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct TileShadingRenderPassFlagsQCOM: u32 {
        const TILE_SHADING_RENDER_PASS_ENABLE_BIT_QCOM = 1;
        const TILE_SHADING_RENDER_PASS_PER_TILE_EXECUTION_BIT_QCOM = 2;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExportMetalObjectTypeFlagBitsEXT.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ExportMetalObjectTypeFlagsEXT: u32 {
        const METAL_DEVICE_BIT_EXT = 1;
        const METAL_COMMAND_QUEUE_BIT_EXT = 2;
        const METAL_BUFFER_BIT_EXT = 4;
        const METAL_TEXTURE_BIT_EXT = 8;
        const METAL_IOSURFACE_BIT_EXT = 16;
        const METAL_SHARED_EVENT_BIT_EXT = 32;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGraphicsPipelineLibraryFlagBitsEXT.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct GraphicsPipelineLibraryFlagsEXT: u32 {
        const VERTEX_INPUT_INTERFACE_BIT_EXT = 1;
        const PRE_RASTERIZATION_SHADERS_BIT_EXT = 2;
        const FRAGMENT_SHADER_BIT_EXT = 4;
        const FRAGMENT_OUTPUT_INTERFACE_BIT_EXT = 8;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCompressionFlagBitsEXT.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ImageCompressionFlagsEXT: u32 {
        const DEFAULT_EXT = 0;
        const FIXED_RATE_DEFAULT_EXT = 1;
        const FIXED_RATE_EXPLICIT_EXT = 2;
        const DISABLED_EXT = 4;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCompressionFixedRateFlagBitsEXT.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ImageCompressionFixedRateFlagsEXT: u32 {
        const NONE_EXT = 0;
        const _1BPC_BIT_EXT = 1;
        const _2BPC_BIT_EXT = 2;
        const _3BPC_BIT_EXT = 4;
        const _4BPC_BIT_EXT = 8;
        const _5BPC_BIT_EXT = 16;
        const _6BPC_BIT_EXT = 32;
        const _7BPC_BIT_EXT = 64;
        const _8BPC_BIT_EXT = 128;
        const _9BPC_BIT_EXT = 256;
        const _10BPC_BIT_EXT = 512;
        const _11BPC_BIT_EXT = 1024;
        const _12BPC_BIT_EXT = 2048;
        const _13BPC_BIT_EXT = 4096;
        const _14BPC_BIT_EXT = 8192;
        const _15BPC_BIT_EXT = 16384;
        const _16BPC_BIT_EXT = 32768;
        const _17BPC_BIT_EXT = 65536;
        const _18BPC_BIT_EXT = 131072;
        const _19BPC_BIT_EXT = 262144;
        const _20BPC_BIT_EXT = 524288;
        const _21BPC_BIT_EXT = 1048576;
        const _22BPC_BIT_EXT = 2097152;
        const _23BPC_BIT_EXT = 4194304;
        const _24BPC_BIT_EXT = 8388608;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceAddressBindingFlagBitsEXT.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct DeviceAddressBindingFlagsEXT: u32 {
        const INTERNAL_OBJECT_BIT_EXT = 1;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageConstraintsInfoFlagBitsFUCHSIA.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ImageConstraintsInfoFlagsFUCHSIA: u32 {
        const IMAGE_CONSTRAINTS_INFO_CPU_READ_RARELY_FUCHSIA = 1;
        const IMAGE_CONSTRAINTS_INFO_CPU_READ_OFTEN_FUCHSIA = 2;
        const IMAGE_CONSTRAINTS_INFO_CPU_WRITE_RARELY_FUCHSIA = 4;
        const IMAGE_CONSTRAINTS_INFO_CPU_WRITE_OFTEN_FUCHSIA = 8;
        const IMAGE_CONSTRAINTS_INFO_PROTECTED_OPTIONAL_FUCHSIA = 16;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFrameBoundaryFlagBitsEXT.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct FrameBoundaryFlagsEXT: u32 {
        const FRAME_END_BIT_EXT = 1;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbModelConversionFlagBitsVALVE.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoEncodeRgbModelConversionFlagsVALVE: u32 {
        const VIDEO_ENCODE_RGB_MODEL_CONVERSION_RGB_IDENTITY_BIT_VALVE = 1;
        const VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_IDENTITY_BIT_VALVE = 2;
        const VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_709_BIT_VALVE = 4;
        const VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_601_BIT_VALVE = 8;
        const VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_2020_BIT_VALVE = 16;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbRangeCompressionFlagBitsVALVE.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoEncodeRgbRangeCompressionFlagsVALVE: u32 {
        const VIDEO_ENCODE_RGB_RANGE_COMPRESSION_FULL_RANGE_BIT_VALVE = 1;
        const VIDEO_ENCODE_RGB_RANGE_COMPRESSION_NARROW_RANGE_BIT_VALVE = 2;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbChromaOffsetFlagBitsVALVE.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct VideoEncodeRgbChromaOffsetFlagsVALVE: u32 {
        const VIDEO_ENCODE_RGB_CHROMA_OFFSET_COSITED_EVEN_BIT_VALVE = 1;
        const VIDEO_ENCODE_RGB_CHROMA_OFFSET_MIDPOINT_BIT_VALVE = 2;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildMicromapFlagBitsEXT.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct BuildMicromapFlagsEXT: u32 {
        const PREFER_FAST_TRACE_BIT_EXT = 1;
        const PREFER_FAST_BUILD_BIT_EXT = 2;
        const ALLOW_COMPACTION_BIT_EXT = 4;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapCreateFlagBitsEXT.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct MicromapCreateFlagsEXT: u32 {
        const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT = 1;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSchedulingControlsFlagBitsARM.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PhysicalDeviceSchedulingControlsFlagsARM: u64 {
        const SHADER_CORE_COUNT_ARM = 1;
        const DISPATCH_PARAMETERS_ARM = 2;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryDecompressionMethodFlagBitsEXT.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct MemoryDecompressionMethodFlagsEXT: u64 {
        const GDEFLATE_1_0_BIT_EXT = 1;
    }
}
pub type MemoryDecompressionMethodFlagsNV = MemoryDecompressionMethodFlagsEXT;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorCreateFlagBitsARM.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct TensorCreateFlagsARM: u64 {
        const MUTABLE_FORMAT_BIT_ARM = 1;
        const PROTECTED_BIT_ARM = 2;
        const DESCRIPTOR_HEAP_CAPTURE_REPLAY_BIT_ARM = 8;
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_ARM = 4;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorUsageFlagBitsARM.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct TensorUsageFlagsARM: u64 {
        const SHADER_BIT_ARM = 2;
        const TRANSFER_SRC_BIT_ARM = 4;
        const TRANSFER_DST_BIT_ARM = 8;
        const IMAGE_ALIASING_BIT_ARM = 16;
        const DATA_GRAPH_BIT_ARM = 32;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowGridSizeFlagBitsNV.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct OpticalFlowGridSizeFlagsNV: u32 {
        const OPTICAL_FLOW_GRID_SIZE_UNKNOWN_NV = 0;
        const OPTICAL_FLOW_GRID_SIZE_1X1_BIT_NV = 1;
        const OPTICAL_FLOW_GRID_SIZE_2X2_BIT_NV = 2;
        const OPTICAL_FLOW_GRID_SIZE_4X4_BIT_NV = 4;
        const OPTICAL_FLOW_GRID_SIZE_8X8_BIT_NV = 8;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowUsageFlagBitsNV.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct OpticalFlowUsageFlagsNV: u32 {
        const OPTICAL_FLOW_USAGE_UNKNOWN_NV = 0;
        const OPTICAL_FLOW_USAGE_INPUT_BIT_NV = 1;
        const OPTICAL_FLOW_USAGE_OUTPUT_BIT_NV = 2;
        const OPTICAL_FLOW_USAGE_HINT_BIT_NV = 4;
        const OPTICAL_FLOW_USAGE_COST_BIT_NV = 8;
        const OPTICAL_FLOW_USAGE_GLOBAL_FLOW_BIT_NV = 16;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowSessionCreateFlagBitsNV.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct OpticalFlowSessionCreateFlagsNV: u32 {
        const OPTICAL_FLOW_SESSION_CREATE_ENABLE_HINT_BIT_NV = 1;
        const OPTICAL_FLOW_SESSION_CREATE_ENABLE_COST_BIT_NV = 2;
        const OPTICAL_FLOW_SESSION_CREATE_ENABLE_GLOBAL_FLOW_BIT_NV = 4;
        const OPTICAL_FLOW_SESSION_CREATE_ALLOW_REGIONS_BIT_NV = 8;
        const OPTICAL_FLOW_SESSION_CREATE_BOTH_DIRECTIONS_BIT_NV = 16;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowExecuteFlagBitsNV.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct OpticalFlowExecuteFlagsNV: u32 {
        const OPTICAL_FLOW_EXECUTE_DISABLE_TEMPORAL_HINTS_BIT_NV = 1;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderCreateFlagBitsEXT.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ShaderCreateFlagsEXT: u32 {
        const LINK_STAGE_BIT_EXT = 1;
        const DESCRIPTOR_HEAP_BIT_EXT = 1024;
        const INSTRUMENT_SHADER_BIT_ARM = 2048;
        const ALLOW_VARYING_SUBGROUP_SIZE_BIT_EXT = 2;
        const REQUIRE_FULL_SUBGROUPS_BIT_EXT = 4;
        const NO_TASK_SHADER_BIT_EXT = 8;
        const DISPATCH_BASE_BIT_EXT = 16;
        const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_EXT = 32;
        const FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT = 64;
        const INDIRECT_BINDABLE_BIT_EXT = 128;
        const OPACITY_MICROMAP_DISALLOW_MIXED_SPECIAL_INDEX_BIT_EXT = 4096;
        const _64_BIT_INDEXING_BIT_EXT = 32768;
        const INDEPENDENT_SETS_BIT_KHR = 262144;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionCreateFlagBitsARM.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct DataGraphPipelineSessionCreateFlagsARM: u64 {
        const PROTECTED_BIT_ARM = 1;
        const OPTICAL_FLOW_CACHE_BIT_ARM = 2;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineDispatchFlagBitsARM.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct DataGraphPipelineDispatchFlagsARM: u64 {
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphTOSAQualityFlagBitsARM.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct DataGraphTOSAQualityFlagsARM: u32 {
        const ACCELERATED_ARM = 1;
        const CONFORMANT_ARM = 2;
        const EXPERIMENTAL_ARM = 4;
        const DEPRECATED_ARM = 8;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureAddressResolutionFlagBitsNV.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ClusterAccelerationStructureAddressResolutionFlagsNV: u32 {
        const CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_NONE_NV = 0;
        const CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_IMPLICIT_DATA_BIT_NV = 1;
        const CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SCRATCH_DATA_BIT_NV = 2;
        const CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_ADDRESS_ARRAY_BIT_NV = 4;
        const CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_SIZES_ARRAY_BIT_NV = 8;
        const CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SRC_INFOS_ARRAY_BIT_NV = 16;
        const CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SRC_INFOS_COUNT_BIT_NV = 32;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureClusterFlagBitsNV.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ClusterAccelerationStructureClusterFlagsNV: u32 {
        const CLUSTER_ACCELERATION_STRUCTURE_CLUSTER_ALLOW_DISABLE_OPACITY_MICROMAPS_NV = 1;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureGeometryFlagBitsNV.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ClusterAccelerationStructureGeometryFlagsNV: u32 {
        const CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_CULL_DISABLE_BIT_NV = 1;
        const CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_NO_DUPLICATE_ANYHIT_INVOCATION_BIT_NV = 2;
        const CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_OPAQUE_BIT_NV = 4;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureIndexFormatFlagBitsNV.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ClusterAccelerationStructureIndexFormatFlagsNV: u32 {
        const CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_8BIT_NV = 1;
        const CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_16BIT_NV = 2;
        const CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_32BIT_NV = 4;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPartitionedAccelerationStructureInstanceFlagBitsNV.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PartitionedAccelerationStructureInstanceFlagsNV: u32 {
        const PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_TRIANGLE_FACING_CULL_DISABLE_BIT_NV = 1;
        const PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_TRIANGLE_FLIP_FACING_BIT_NV = 2;
        const PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_FORCE_OPAQUE_BIT_NV = 4;
        const PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_FORCE_NO_OPAQUE_BIT_NV = 8;
        const PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV = 16;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsInputModeFlagBitsEXT.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct IndirectCommandsInputModeFlagsEXT: u32 {
        const VULKAN_INDEX_BUFFER_EXT = 1;
        const DXGI_INDEX_BUFFER_EXT = 2;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutUsageFlagBitsEXT.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct IndirectCommandsLayoutUsageFlagsEXT: u32 {
        const EXPLICIT_PREPROCESS_BIT_EXT = 1;
        const UNORDERED_SEQUENCES_BIT_EXT = 2;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowGridSizeFlagBitsARM.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct DataGraphOpticalFlowGridSizeFlagsARM: u32 {
        const UNKNOWN_ARM = 0;
        const _1X1_BIT_ARM = 1;
        const _2X2_BIT_ARM = 2;
        const _4X4_BIT_ARM = 4;
        const _8X8_BIT_ARM = 8;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowCreateFlagBitsARM.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct DataGraphOpticalFlowCreateFlagsARM: u32 {
        const ENABLE_HINT_BIT_ARM = 1;
        const ENABLE_COST_BIT_ARM = 2;
        const RESERVED_30_BIT_ARM = 1073741824;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowImageUsageFlagBitsARM.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct DataGraphOpticalFlowImageUsageFlagsARM: u32 {
        const UNKNOWN_ARM = 0;
        const INPUT_BIT_ARM = 1;
        const OUTPUT_BIT_ARM = 2;
        const HINT_BIT_ARM = 4;
        const COST_BIT_ARM = 8;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowExecuteFlagBitsARM.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct DataGraphOpticalFlowExecuteFlagsARM: u32 {
        const DISABLE_TEMPORAL_HINTS_BIT_ARM = 1;
        const INPUT_UNCHANGED_BIT_ARM = 2;
        const REFERENCE_UNCHANGED_BIT_ARM = 4;
        const INPUT_IS_PREVIOUS_REFERENCE_BIT_ARM = 8;
        const REFERENCE_IS_PREVIOUS_INPUT_BIT_ARM = 16;
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCooperativeMatrixFlagBitsEXT.html>
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct CooperativeMatrixFlagsEXT: u32 {
        const SATURATING_ACCUMULATION_BIT_EXT = 1;
    }
}
