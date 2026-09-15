// WARNING: AUTO GENERATED MODULE
#![allow(nonstandard_style)]
#![allow(unused_imports)]

use crate::manual::*;
use crate::platform::*;
use std::ffi::{c_char, c_int, c_uint, c_void};
bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlagBits.html>
    #[doc(alias = "VkFormatFeatureFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct FormatFeatureFlags: u32 {
        #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT")]
        const SAMPLED_IMAGE_BIT = 1;
        #[doc(alias = "VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT")]
        const STORAGE_IMAGE_BIT = 2;
        #[doc(alias = "VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT")]
        const STORAGE_IMAGE_ATOMIC_BIT = 4;
        #[doc(alias = "VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT")]
        const UNIFORM_TEXEL_BUFFER_BIT = 8;
        #[doc(alias = "VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT")]
        const STORAGE_TEXEL_BUFFER_BIT = 16;
        #[doc(alias = "VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT")]
        const STORAGE_TEXEL_BUFFER_ATOMIC_BIT = 32;
        #[doc(alias = "VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT")]
        const VERTEX_BUFFER_BIT = 64;
        #[doc(alias = "VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT")]
        const COLOR_ATTACHMENT_BIT = 128;
        #[doc(alias = "VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT")]
        const COLOR_ATTACHMENT_BLEND_BIT = 256;
        #[doc(alias = "VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT")]
        const DEPTH_STENCIL_ATTACHMENT_BIT = 512;
        #[doc(alias = "VK_FORMAT_FEATURE_BLIT_SRC_BIT")]
        const BLIT_SRC_BIT = 1024;
        #[doc(alias = "VK_FORMAT_FEATURE_BLIT_DST_BIT")]
        const BLIT_DST_BIT = 2048;
        #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT")]
        const SAMPLED_IMAGE_FILTER_LINEAR_BIT = 4096;
        #[doc(alias = "VK_FORMAT_FEATURE_TRANSFER_SRC_BIT")]
        const TRANSFER_SRC_BIT = 16384;
        #[doc(alias = "VK_FORMAT_FEATURE_TRANSFER_DST_BIT")]
        const TRANSFER_DST_BIT = 32768;
        #[doc(alias = "VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT")]
        const MIDPOINT_CHROMA_SAMPLES_BIT = 131072;
        #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT")]
        const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT = 262144;
        #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT")]
        const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT = 524288;
        #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT")]
        const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT = 1048576;
        #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT")]
        const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT = 2097152;
        #[doc(alias = "VK_FORMAT_FEATURE_DISJOINT_BIT")]
        const DISJOINT_BIT = 4194304;
        #[doc(alias = "VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT")]
        const COSITED_CHROMA_SAMPLES_BIT = 8388608;
        #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT")]
        const SAMPLED_IMAGE_FILTER_MINMAX_BIT = 65536;
        #[doc(alias = "VK_FORMAT_FEATURE_VIDEO_DECODE_OUTPUT_BIT_KHR")]
        const VIDEO_DECODE_OUTPUT_BIT_KHR = 33554432;
        #[doc(alias = "VK_FORMAT_FEATURE_VIDEO_DECODE_DPB_BIT_KHR")]
        const VIDEO_DECODE_DPB_BIT_KHR = 67108864;
        #[doc(alias = "VK_FORMAT_FEATURE_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR")]
        const ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR = 536870912;
        #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT")]
        const SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT = 8192;
        #[doc(alias = "VK_FORMAT_FEATURE_FRAGMENT_DENSITY_MAP_BIT_EXT")]
        const FRAGMENT_DENSITY_MAP_BIT_EXT = 16777216;
        #[doc(alias = "VK_FORMAT_FEATURE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
        const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 1073741824;
        #[doc(alias = "VK_FORMAT_FEATURE_VIDEO_ENCODE_INPUT_BIT_KHR")]
        const VIDEO_ENCODE_INPUT_BIT_KHR = 134217728;
        #[doc(alias = "VK_FORMAT_FEATURE_VIDEO_ENCODE_DPB_BIT_KHR")]
        const VIDEO_ENCODE_DPB_BIT_KHR = 268435456;
    }
}
impl FormatFeatureFlags {
    #[doc(alias = "VK_FORMAT_FEATURE_TRANSFER_SRC_BIT_KHR")]
    pub const TRANSFER_SRC_BIT_KHR: Self = Self::TRANSFER_SRC_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_TRANSFER_DST_BIT_KHR")]
    pub const TRANSFER_DST_BIT_KHR: Self = Self::TRANSFER_DST_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT_KHR")]
    pub const MIDPOINT_CHROMA_SAMPLES_BIT_KHR: Self = Self::MIDPOINT_CHROMA_SAMPLES_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR")]
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT;
    #[doc(
        alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR"
    )]
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT;
    #[doc(
        alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR"
    )]
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT;
    #[doc(
        alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR"
    )]
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR:
        Self = Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_DISJOINT_BIT_KHR")]
    pub const DISJOINT_BIT_KHR: Self = Self::DISJOINT_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT_KHR")]
    pub const COSITED_CHROMA_SAMPLES_BIT_KHR: Self = Self::COSITED_CHROMA_SAMPLES_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT_EXT")]
    pub const SAMPLED_IMAGE_FILTER_MINMAX_BIT_EXT: Self = Self::SAMPLED_IMAGE_FILTER_MINMAX_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG")]
    pub const SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG: Self = Self::SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCreateFlagBits.html>
    #[doc(alias = "VkImageCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ImageCreateFlags: u32 {
        #[doc(alias = "VK_IMAGE_CREATE_SPARSE_BINDING_BIT")]
        const SPARSE_BINDING_BIT = 1;
        #[doc(alias = "VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT")]
        const SPARSE_RESIDENCY_BIT = 2;
        #[doc(alias = "VK_IMAGE_CREATE_SPARSE_ALIASED_BIT")]
        const SPARSE_ALIASED_BIT = 4;
        #[doc(alias = "VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT")]
        const MUTABLE_FORMAT_BIT = 8;
        #[doc(alias = "VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT")]
        const CUBE_COMPATIBLE_BIT = 16;
        #[doc(alias = "VK_IMAGE_CREATE_ALIAS_BIT")]
        const ALIAS_BIT = 1024;
        #[doc(alias = "VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT")]
        const SPLIT_INSTANCE_BIND_REGIONS_BIT = 64;
        #[doc(alias = "VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT")]
        const _2D_ARRAY_COMPATIBLE_BIT = 32;
        #[doc(alias = "VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT")]
        const BLOCK_TEXEL_VIEW_COMPATIBLE_BIT = 128;
        #[doc(alias = "VK_IMAGE_CREATE_EXTENDED_USAGE_BIT")]
        const EXTENDED_USAGE_BIT = 256;
        #[doc(alias = "VK_IMAGE_CREATE_PROTECTED_BIT")]
        const PROTECTED_BIT = 2048;
        #[doc(alias = "VK_IMAGE_CREATE_DISJOINT_BIT")]
        const DISJOINT_BIT = 512;
        #[doc(alias = "VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV")]
        const CORNER_SAMPLED_BIT_NV = 8192;
        #[doc(alias = "VK_IMAGE_CREATE_DESCRIPTOR_HEAP_CAPTURE_REPLAY_BIT_EXT")]
        const DESCRIPTOR_HEAP_CAPTURE_REPLAY_BIT_EXT = 65536;
        #[doc(alias = "VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT")]
        const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT = 4096;
        #[doc(alias = "VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT")]
        const SUBSAMPLED_BIT_EXT = 16384;
        #[doc(alias = "VK_IMAGE_CREATE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT")]
        const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT = 262144;
        #[doc(alias = "VK_IMAGE_CREATE_2D_VIEW_COMPATIBLE_BIT_EXT")]
        const _2D_VIEW_COMPATIBLE_BIT_EXT = 131072;
        #[doc(alias = "VK_IMAGE_CREATE_VIDEO_PROFILE_INDEPENDENT_BIT_KHR")]
        const VIDEO_PROFILE_INDEPENDENT_BIT_KHR = 1048576;
        #[doc(alias = "VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_EXT")]
        const FRAGMENT_DENSITY_MAP_OFFSET_BIT_EXT = 32768;
        #[doc(alias = "VK_IMAGE_CREATE_ALIAS_SINGLE_LAYER_DESCRIPTOR_BIT_KHR")]
        const ALIAS_SINGLE_LAYER_DESCRIPTOR_BIT_KHR = 4194304;
    }
}
impl ImageCreateFlags {
    #[doc(alias = "VK_IMAGE_CREATE_ALIAS_BIT_KHR")]
    pub const ALIAS_BIT_KHR: Self = Self::ALIAS_BIT;
    #[doc(alias = "VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR")]
    pub const SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR: Self = Self::SPLIT_INSTANCE_BIND_REGIONS_BIT;
    #[doc(alias = "VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT_KHR")]
    pub const _2D_ARRAY_COMPATIBLE_BIT_KHR: Self = Self::_2D_ARRAY_COMPATIBLE_BIT;
    #[doc(alias = "VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR")]
    pub const BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR: Self = Self::BLOCK_TEXEL_VIEW_COMPATIBLE_BIT;
    #[doc(alias = "VK_IMAGE_CREATE_EXTENDED_USAGE_BIT_KHR")]
    pub const EXTENDED_USAGE_BIT_KHR: Self = Self::EXTENDED_USAGE_BIT;
    #[doc(alias = "VK_IMAGE_CREATE_DISJOINT_BIT_KHR")]
    pub const DISJOINT_BIT_KHR: Self = Self::DISJOINT_BIT;
    #[doc(alias = "VK_IMAGE_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT")]
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT: Self =
        Self::DESCRIPTOR_HEAP_CAPTURE_REPLAY_BIT_EXT;
    #[doc(alias = "VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM")]
    pub const FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM: Self =
        Self::FRAGMENT_DENSITY_MAP_OFFSET_BIT_EXT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSampleCountFlagBits.html>
    #[doc(alias = "VkSampleCountFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SampleCountFlags: u32 {
        #[doc(alias = "VK_SAMPLE_COUNT_1_BIT")]
        const _1_BIT = 1;
        #[doc(alias = "VK_SAMPLE_COUNT_2_BIT")]
        const _2_BIT = 2;
        #[doc(alias = "VK_SAMPLE_COUNT_4_BIT")]
        const _4_BIT = 4;
        #[doc(alias = "VK_SAMPLE_COUNT_8_BIT")]
        const _8_BIT = 8;
        #[doc(alias = "VK_SAMPLE_COUNT_16_BIT")]
        const _16_BIT = 16;
        #[doc(alias = "VK_SAMPLE_COUNT_32_BIT")]
        const _32_BIT = 32;
        #[doc(alias = "VK_SAMPLE_COUNT_64_BIT")]
        const _64_BIT = 64;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageUsageFlagBits.html>
    #[doc(alias = "VkImageUsageFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ImageUsageFlags: u32 {
        #[doc(alias = "VK_IMAGE_USAGE_TRANSFER_SRC_BIT")]
        const TRANSFER_SRC_BIT = 1;
        #[doc(alias = "VK_IMAGE_USAGE_TRANSFER_DST_BIT")]
        const TRANSFER_DST_BIT = 2;
        #[doc(alias = "VK_IMAGE_USAGE_SAMPLED_BIT")]
        const SAMPLED_BIT = 4;
        #[doc(alias = "VK_IMAGE_USAGE_STORAGE_BIT")]
        const STORAGE_BIT = 8;
        #[doc(alias = "VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT")]
        const COLOR_ATTACHMENT_BIT = 16;
        #[doc(alias = "VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT")]
        const DEPTH_STENCIL_ATTACHMENT_BIT = 32;
        #[doc(alias = "VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT")]
        const TRANSIENT_ATTACHMENT_BIT = 64;
        #[doc(alias = "VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT")]
        const INPUT_ATTACHMENT_BIT = 128;
        #[doc(alias = "VK_IMAGE_USAGE_HOST_TRANSFER_BIT")]
        const HOST_TRANSFER_BIT = 4194304;
        #[doc(alias = "VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR")]
        const VIDEO_DECODE_DST_BIT_KHR = 1024;
        #[doc(alias = "VK_IMAGE_USAGE_VIDEO_DECODE_SRC_BIT_KHR")]
        const VIDEO_DECODE_SRC_BIT_KHR = 2048;
        #[doc(alias = "VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR")]
        const VIDEO_DECODE_DPB_BIT_KHR = 4096;
        #[doc(alias = "VK_IMAGE_USAGE_FRAGMENT_DENSITY_MAP_BIT_EXT")]
        const FRAGMENT_DENSITY_MAP_BIT_EXT = 512;
        #[doc(alias = "VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
        const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 256;
        #[doc(alias = "VK_IMAGE_USAGE_VIDEO_ENCODE_DST_BIT_KHR")]
        const VIDEO_ENCODE_DST_BIT_KHR = 8192;
        #[doc(alias = "VK_IMAGE_USAGE_VIDEO_ENCODE_SRC_BIT_KHR")]
        const VIDEO_ENCODE_SRC_BIT_KHR = 16384;
        #[doc(alias = "VK_IMAGE_USAGE_VIDEO_ENCODE_DPB_BIT_KHR")]
        const VIDEO_ENCODE_DPB_BIT_KHR = 32768;
        #[doc(alias = "VK_IMAGE_USAGE_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT")]
        const ATTACHMENT_FEEDBACK_LOOP_BIT_EXT = 524288;
        #[doc(alias = "VK_IMAGE_USAGE_INVOCATION_MASK_BIT_HUAWEI")]
        const INVOCATION_MASK_BIT_HUAWEI = 262144;
        #[doc(alias = "VK_IMAGE_USAGE_SAMPLE_WEIGHT_BIT_QCOM")]
        const SAMPLE_WEIGHT_BIT_QCOM = 1048576;
        #[doc(alias = "VK_IMAGE_USAGE_SAMPLE_BLOCK_MATCH_BIT_QCOM")]
        const SAMPLE_BLOCK_MATCH_BIT_QCOM = 2097152;
        #[doc(alias = "VK_IMAGE_USAGE_TENSOR_ALIASING_BIT_ARM")]
        const TENSOR_ALIASING_BIT_ARM = 8388608;
        #[doc(alias = "VK_IMAGE_USAGE_TILE_MEMORY_BIT_QCOM")]
        const TILE_MEMORY_BIT_QCOM = 134217728;
        #[doc(alias = "VK_IMAGE_USAGE_VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR")]
        const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR = 33554432;
        #[doc(alias = "VK_IMAGE_USAGE_VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR")]
        const VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR = 67108864;
    }
}
impl ImageUsageFlags {
    #[doc(alias = "VK_IMAGE_USAGE_HOST_TRANSFER_BIT_EXT")]
    pub const HOST_TRANSFER_BIT_EXT: Self = Self::HOST_TRANSFER_BIT;
    #[doc(alias = "VK_IMAGE_USAGE_SHADING_RATE_IMAGE_BIT_NV")]
    pub const SHADING_RATE_IMAGE_BIT_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkInstanceCreateFlagBits.html>
    #[doc(alias = "VkInstanceCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct InstanceCreateFlags: u32 {
        #[doc(alias = "VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR")]
        const ENUMERATE_PORTABILITY_BIT_KHR = 1;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryHeapFlagBits.html>
    #[doc(alias = "VkMemoryHeapFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct MemoryHeapFlags: u32 {
        #[doc(alias = "VK_MEMORY_HEAP_DEVICE_LOCAL_BIT")]
        const DEVICE_LOCAL_BIT = 1;
        #[doc(alias = "VK_MEMORY_HEAP_MULTI_INSTANCE_BIT")]
        const MULTI_INSTANCE_BIT = 2;
        #[doc(alias = "VK_MEMORY_HEAP_TILE_MEMORY_BIT_QCOM")]
        const TILE_MEMORY_BIT_QCOM = 8;
    }
}
impl MemoryHeapFlags {
    #[doc(alias = "VK_MEMORY_HEAP_MULTI_INSTANCE_BIT_KHR")]
    pub const MULTI_INSTANCE_BIT_KHR: Self = Self::MULTI_INSTANCE_BIT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryPropertyFlagBits.html>
    #[doc(alias = "VkMemoryPropertyFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct MemoryPropertyFlags: u32 {
        #[doc(alias = "VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT")]
        const DEVICE_LOCAL_BIT = 1;
        #[doc(alias = "VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT")]
        const HOST_VISIBLE_BIT = 2;
        #[doc(alias = "VK_MEMORY_PROPERTY_HOST_COHERENT_BIT")]
        const HOST_COHERENT_BIT = 4;
        #[doc(alias = "VK_MEMORY_PROPERTY_HOST_CACHED_BIT")]
        const HOST_CACHED_BIT = 8;
        #[doc(alias = "VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT")]
        const LAZILY_ALLOCATED_BIT = 16;
        #[doc(alias = "VK_MEMORY_PROPERTY_PROTECTED_BIT")]
        const PROTECTED_BIT = 32;
        #[doc(alias = "VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD")]
        const DEVICE_COHERENT_BIT_AMD = 64;
        #[doc(alias = "VK_MEMORY_PROPERTY_DEVICE_UNCACHED_BIT_AMD")]
        const DEVICE_UNCACHED_BIT_AMD = 128;
        #[doc(alias = "VK_MEMORY_PROPERTY_RDMA_CAPABLE_BIT_NV")]
        const RDMA_CAPABLE_BIT_NV = 256;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFlagBits.html>
    #[doc(alias = "VkQueueFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct QueueFlags: u32 {
        #[doc(alias = "VK_QUEUE_GRAPHICS_BIT")]
        const GRAPHICS_BIT = 1;
        #[doc(alias = "VK_QUEUE_COMPUTE_BIT")]
        const COMPUTE_BIT = 2;
        #[doc(alias = "VK_QUEUE_TRANSFER_BIT")]
        const TRANSFER_BIT = 4;
        #[doc(alias = "VK_QUEUE_SPARSE_BINDING_BIT")]
        const SPARSE_BINDING_BIT = 8;
        #[doc(alias = "VK_QUEUE_PROTECTED_BIT")]
        const PROTECTED_BIT = 16;
        #[doc(alias = "VK_QUEUE_VIDEO_DECODE_BIT_KHR")]
        const VIDEO_DECODE_BIT_KHR = 32;
        #[doc(alias = "VK_QUEUE_VIDEO_ENCODE_BIT_KHR")]
        const VIDEO_ENCODE_BIT_KHR = 64;
        #[doc(alias = "VK_QUEUE_OPTICAL_FLOW_BIT_NV")]
        const OPTICAL_FLOW_BIT_NV = 256;
        #[doc(alias = "VK_QUEUE_DATA_GRAPH_BIT_ARM")]
        const DATA_GRAPH_BIT_ARM = 1024;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderStageFlagBits.html>
    #[doc(alias = "VkShaderStageFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ShaderStageFlags: u32 {
        #[doc(alias = "VK_SHADER_STAGE_VERTEX_BIT")]
        const VERTEX_BIT = 1;
        #[doc(alias = "VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT")]
        const TESSELLATION_CONTROL_BIT = 2;
        #[doc(alias = "VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT")]
        const TESSELLATION_EVALUATION_BIT = 4;
        #[doc(alias = "VK_SHADER_STAGE_GEOMETRY_BIT")]
        const GEOMETRY_BIT = 8;
        #[doc(alias = "VK_SHADER_STAGE_FRAGMENT_BIT")]
        const FRAGMENT_BIT = 16;
        #[doc(alias = "VK_SHADER_STAGE_COMPUTE_BIT")]
        const COMPUTE_BIT = 32;
        #[doc(alias = "VK_SHADER_STAGE_ALL_GRAPHICS")]
        const ALL_GRAPHICS = 31;
        #[doc(alias = "VK_SHADER_STAGE_ALL")]
        const ALL = 2147483647;
        #[doc(alias = "VK_SHADER_STAGE_RAYGEN_BIT_KHR")]
        const RAYGEN_BIT_KHR = 256;
        #[doc(alias = "VK_SHADER_STAGE_ANY_HIT_BIT_KHR")]
        const ANY_HIT_BIT_KHR = 512;
        #[doc(alias = "VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR")]
        const CLOSEST_HIT_BIT_KHR = 1024;
        #[doc(alias = "VK_SHADER_STAGE_MISS_BIT_KHR")]
        const MISS_BIT_KHR = 2048;
        #[doc(alias = "VK_SHADER_STAGE_INTERSECTION_BIT_KHR")]
        const INTERSECTION_BIT_KHR = 4096;
        #[doc(alias = "VK_SHADER_STAGE_CALLABLE_BIT_KHR")]
        const CALLABLE_BIT_KHR = 8192;
        #[doc(alias = "VK_SHADER_STAGE_TASK_BIT_EXT")]
        const TASK_BIT_EXT = 64;
        #[doc(alias = "VK_SHADER_STAGE_MESH_BIT_EXT")]
        const MESH_BIT_EXT = 128;
        #[doc(alias = "VK_SHADER_STAGE_SUBPASS_SHADING_BIT_HUAWEI")]
        const SUBPASS_SHADING_BIT_HUAWEI = 16384;
        #[doc(alias = "VK_SHADER_STAGE_CLUSTER_CULLING_BIT_HUAWEI")]
        const CLUSTER_CULLING_BIT_HUAWEI = 524288;
    }
}
impl ShaderStageFlags {
    #[doc(alias = "VK_SHADER_STAGE_RAYGEN_BIT_NV")]
    pub const RAYGEN_BIT_NV: Self = Self::RAYGEN_BIT_KHR;
    #[doc(alias = "VK_SHADER_STAGE_ANY_HIT_BIT_NV")]
    pub const ANY_HIT_BIT_NV: Self = Self::ANY_HIT_BIT_KHR;
    #[doc(alias = "VK_SHADER_STAGE_CLOSEST_HIT_BIT_NV")]
    pub const CLOSEST_HIT_BIT_NV: Self = Self::CLOSEST_HIT_BIT_KHR;
    #[doc(alias = "VK_SHADER_STAGE_MISS_BIT_NV")]
    pub const MISS_BIT_NV: Self = Self::MISS_BIT_KHR;
    #[doc(alias = "VK_SHADER_STAGE_INTERSECTION_BIT_NV")]
    pub const INTERSECTION_BIT_NV: Self = Self::INTERSECTION_BIT_KHR;
    #[doc(alias = "VK_SHADER_STAGE_CALLABLE_BIT_NV")]
    pub const CALLABLE_BIT_NV: Self = Self::CALLABLE_BIT_KHR;
    #[doc(alias = "VK_SHADER_STAGE_TASK_BIT_NV")]
    pub const TASK_BIT_NV: Self = Self::TASK_BIT_EXT;
    #[doc(alias = "VK_SHADER_STAGE_MESH_BIT_NV")]
    pub const MESH_BIT_NV: Self = Self::MESH_BIT_EXT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceQueueCreateFlagBits.html>
    #[doc(alias = "VkDeviceQueueCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DeviceQueueCreateFlags: u32 {
        #[doc(alias = "VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT")]
        const PROTECTED_BIT = 1;
        #[doc(alias = "VK_DEVICE_QUEUE_CREATE_INTERNALLY_SYNCHRONIZED_BIT_KHR")]
        const INTERNALLY_SYNCHRONIZED_BIT_KHR = 4;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineStageFlagBits.html>
    #[doc(alias = "VkPipelineStageFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PipelineStageFlags: u32 {
        #[doc(alias = "VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT")]
        const TOP_OF_PIPE_BIT = 1;
        #[doc(alias = "VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT")]
        const DRAW_INDIRECT_BIT = 2;
        #[doc(alias = "VK_PIPELINE_STAGE_VERTEX_INPUT_BIT")]
        const VERTEX_INPUT_BIT = 4;
        #[doc(alias = "VK_PIPELINE_STAGE_VERTEX_SHADER_BIT")]
        const VERTEX_SHADER_BIT = 8;
        #[doc(alias = "VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT")]
        const TESSELLATION_CONTROL_SHADER_BIT = 16;
        #[doc(alias = "VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT")]
        const TESSELLATION_EVALUATION_SHADER_BIT = 32;
        #[doc(alias = "VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT")]
        const GEOMETRY_SHADER_BIT = 64;
        #[doc(alias = "VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT")]
        const FRAGMENT_SHADER_BIT = 128;
        #[doc(alias = "VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT")]
        const EARLY_FRAGMENT_TESTS_BIT = 256;
        #[doc(alias = "VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT")]
        const LATE_FRAGMENT_TESTS_BIT = 512;
        #[doc(alias = "VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT")]
        const COLOR_ATTACHMENT_OUTPUT_BIT = 1024;
        #[doc(alias = "VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT")]
        const COMPUTE_SHADER_BIT = 2048;
        #[doc(alias = "VK_PIPELINE_STAGE_TRANSFER_BIT")]
        const TRANSFER_BIT = 4096;
        #[doc(alias = "VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT")]
        const BOTTOM_OF_PIPE_BIT = 8192;
        #[doc(alias = "VK_PIPELINE_STAGE_HOST_BIT")]
        const HOST_BIT = 16384;
        #[doc(alias = "VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT")]
        const ALL_GRAPHICS_BIT = 32768;
        #[doc(alias = "VK_PIPELINE_STAGE_ALL_COMMANDS_BIT")]
        const ALL_COMMANDS_BIT = 65536;
        #[doc(alias = "VK_PIPELINE_STAGE_NONE")]
        const NONE = 0;
        #[doc(alias = "VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT")]
        const TRANSFORM_FEEDBACK_BIT_EXT = 16777216;
        #[doc(alias = "VK_PIPELINE_STAGE_CONDITIONAL_RENDERING_BIT_EXT")]
        const CONDITIONAL_RENDERING_BIT_EXT = 262144;
        #[doc(alias = "VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR")]
        const ACCELERATION_STRUCTURE_BUILD_BIT_KHR = 33554432;
        #[doc(alias = "VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_KHR")]
        const RAY_TRACING_SHADER_BIT_KHR = 2097152;
        #[doc(alias = "VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT")]
        const FRAGMENT_DENSITY_PROCESS_BIT_EXT = 8388608;
        #[doc(alias = "VK_PIPELINE_STAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
        const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 4194304;
        #[doc(alias = "VK_PIPELINE_STAGE_TASK_SHADER_BIT_EXT")]
        const TASK_SHADER_BIT_EXT = 524288;
        #[doc(alias = "VK_PIPELINE_STAGE_MESH_SHADER_BIT_EXT")]
        const MESH_SHADER_BIT_EXT = 1048576;
        #[doc(alias = "VK_PIPELINE_STAGE_COMMAND_PREPROCESS_BIT_EXT")]
        const COMMAND_PREPROCESS_BIT_EXT = 131072;
    }
}
impl PipelineStageFlags {
    #[doc(alias = "VK_PIPELINE_STAGE_NONE_KHR")]
    pub const NONE_KHR: Self = Self::NONE;
    #[doc(alias = "VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_NV")]
    pub const ACCELERATION_STRUCTURE_BUILD_BIT_NV: Self =
        Self::ACCELERATION_STRUCTURE_BUILD_BIT_KHR;
    #[doc(alias = "VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_NV")]
    pub const RAY_TRACING_SHADER_BIT_NV: Self = Self::RAY_TRACING_SHADER_BIT_KHR;
    #[doc(alias = "VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV")]
    pub const SHADING_RATE_IMAGE_BIT_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR;
    #[doc(alias = "VK_PIPELINE_STAGE_TASK_SHADER_BIT_NV")]
    pub const TASK_SHADER_BIT_NV: Self = Self::TASK_SHADER_BIT_EXT;
    #[doc(alias = "VK_PIPELINE_STAGE_MESH_SHADER_BIT_NV")]
    pub const MESH_SHADER_BIT_NV: Self = Self::MESH_SHADER_BIT_EXT;
    #[doc(alias = "VK_PIPELINE_STAGE_COMMAND_PREPROCESS_BIT_NV")]
    pub const COMMAND_PREPROCESS_BIT_NV: Self = Self::COMMAND_PREPROCESS_BIT_EXT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryMapFlagBits.html>
    #[doc(alias = "VkMemoryMapFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct MemoryMapFlags: u32 {
        #[doc(alias = "VK_MEMORY_MAP_PLACED_BIT_EXT")]
        const PLACED_BIT_EXT = 1;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageAspectFlagBits.html>
    #[doc(alias = "VkImageAspectFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ImageAspectFlags: u32 {
        #[doc(alias = "VK_IMAGE_ASPECT_COLOR_BIT")]
        const COLOR_BIT = 1;
        #[doc(alias = "VK_IMAGE_ASPECT_DEPTH_BIT")]
        const DEPTH_BIT = 2;
        #[doc(alias = "VK_IMAGE_ASPECT_STENCIL_BIT")]
        const STENCIL_BIT = 4;
        #[doc(alias = "VK_IMAGE_ASPECT_METADATA_BIT")]
        const METADATA_BIT = 8;
        #[doc(alias = "VK_IMAGE_ASPECT_PLANE_0_BIT")]
        const PLANE_0_BIT = 16;
        #[doc(alias = "VK_IMAGE_ASPECT_PLANE_1_BIT")]
        const PLANE_1_BIT = 32;
        #[doc(alias = "VK_IMAGE_ASPECT_PLANE_2_BIT")]
        const PLANE_2_BIT = 64;
        #[doc(alias = "VK_IMAGE_ASPECT_NONE")]
        const NONE = 0;
        #[doc(alias = "VK_IMAGE_ASPECT_MEMORY_PLANE_0_BIT_EXT")]
        const MEMORY_PLANE_0_BIT_EXT = 128;
        #[doc(alias = "VK_IMAGE_ASPECT_MEMORY_PLANE_1_BIT_EXT")]
        const MEMORY_PLANE_1_BIT_EXT = 256;
        #[doc(alias = "VK_IMAGE_ASPECT_MEMORY_PLANE_2_BIT_EXT")]
        const MEMORY_PLANE_2_BIT_EXT = 512;
        #[doc(alias = "VK_IMAGE_ASPECT_MEMORY_PLANE_3_BIT_EXT")]
        const MEMORY_PLANE_3_BIT_EXT = 1024;
    }
}
impl ImageAspectFlags {
    #[doc(alias = "VK_IMAGE_ASPECT_PLANE_0_BIT_KHR")]
    pub const PLANE_0_BIT_KHR: Self = Self::PLANE_0_BIT;
    #[doc(alias = "VK_IMAGE_ASPECT_PLANE_1_BIT_KHR")]
    pub const PLANE_1_BIT_KHR: Self = Self::PLANE_1_BIT;
    #[doc(alias = "VK_IMAGE_ASPECT_PLANE_2_BIT_KHR")]
    pub const PLANE_2_BIT_KHR: Self = Self::PLANE_2_BIT;
    #[doc(alias = "VK_IMAGE_ASPECT_NONE_KHR")]
    pub const NONE_KHR: Self = Self::NONE;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageFormatFlagBits.html>
    #[doc(alias = "VkSparseImageFormatFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SparseImageFormatFlags: u32 {
        #[doc(alias = "VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT")]
        const SINGLE_MIPTAIL_BIT = 1;
        #[doc(alias = "VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT")]
        const ALIGNED_MIP_SIZE_BIT = 2;
        #[doc(alias = "VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT")]
        const NONSTANDARD_BLOCK_SIZE_BIT = 4;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseMemoryBindFlagBits.html>
    #[doc(alias = "VkSparseMemoryBindFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SparseMemoryBindFlags: u32 {
        #[doc(alias = "VK_SPARSE_MEMORY_BIND_METADATA_BIT")]
        const METADATA_BIT = 1;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceCreateFlagBits.html>
    #[doc(alias = "VkFenceCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct FenceCreateFlags: u32 {
        #[doc(alias = "VK_FENCE_CREATE_SIGNALED_BIT")]
        const SIGNALED_BIT = 1;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPoolCreateFlagBits.html>
    #[doc(alias = "VkQueryPoolCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct QueryPoolCreateFlags: u32 {
        #[doc(alias = "VK_QUERY_POOL_CREATE_RESET_BIT_KHR")]
        const RESET_BIT_KHR = 1;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPipelineStatisticFlagBits.html>
    #[doc(alias = "VkQueryPipelineStatisticFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct QueryPipelineStatisticFlags: u32 {
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT")]
        const INPUT_ASSEMBLY_VERTICES_BIT = 1;
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT")]
        const INPUT_ASSEMBLY_PRIMITIVES_BIT = 2;
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT")]
        const VERTEX_SHADER_INVOCATIONS_BIT = 4;
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT")]
        const GEOMETRY_SHADER_INVOCATIONS_BIT = 8;
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT")]
        const GEOMETRY_SHADER_PRIMITIVES_BIT = 16;
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT")]
        const CLIPPING_INVOCATIONS_BIT = 32;
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT")]
        const CLIPPING_PRIMITIVES_BIT = 64;
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT")]
        const FRAGMENT_SHADER_INVOCATIONS_BIT = 128;
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT")]
        const TESSELLATION_CONTROL_SHADER_PATCHES_BIT = 256;
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT")]
        const TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT = 512;
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT")]
        const COMPUTE_SHADER_INVOCATIONS_BIT = 1024;
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_TASK_SHADER_INVOCATIONS_BIT_EXT")]
        const TASK_SHADER_INVOCATIONS_BIT_EXT = 2048;
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_MESH_SHADER_INVOCATIONS_BIT_EXT")]
        const MESH_SHADER_INVOCATIONS_BIT_EXT = 4096;
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_CLUSTER_CULLING_SHADER_INVOCATIONS_BIT_HUAWEI")]
        const CLUSTER_CULLING_SHADER_INVOCATIONS_BIT_HUAWEI = 8192;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryResultFlagBits.html>
    #[doc(alias = "VkQueryResultFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct QueryResultFlags: u32 {
        #[doc(alias = "VK_QUERY_RESULT_64_BIT")]
        const _64_BIT = 1;
        #[doc(alias = "VK_QUERY_RESULT_WAIT_BIT")]
        const WAIT_BIT = 2;
        #[doc(alias = "VK_QUERY_RESULT_WITH_AVAILABILITY_BIT")]
        const WITH_AVAILABILITY_BIT = 4;
        #[doc(alias = "VK_QUERY_RESULT_PARTIAL_BIT")]
        const PARTIAL_BIT = 8;
        #[doc(alias = "VK_QUERY_RESULT_WITH_STATUS_BIT_KHR")]
        const WITH_STATUS_BIT_KHR = 16;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCreateFlagBits.html>
    #[doc(alias = "VkBufferCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct BufferCreateFlags: u32 {
        #[doc(alias = "VK_BUFFER_CREATE_SPARSE_BINDING_BIT")]
        const SPARSE_BINDING_BIT = 1;
        #[doc(alias = "VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT")]
        const SPARSE_RESIDENCY_BIT = 2;
        #[doc(alias = "VK_BUFFER_CREATE_SPARSE_ALIASED_BIT")]
        const SPARSE_ALIASED_BIT = 4;
        #[doc(alias = "VK_BUFFER_CREATE_PROTECTED_BIT")]
        const PROTECTED_BIT = 8;
        #[doc(alias = "VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT")]
        const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT = 16;
        #[doc(alias = "VK_BUFFER_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT")]
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT = 32;
        #[doc(alias = "VK_BUFFER_CREATE_VIDEO_PROFILE_INDEPENDENT_BIT_KHR")]
        const VIDEO_PROFILE_INDEPENDENT_BIT_KHR = 64;
    }
}
impl BufferCreateFlags {
    #[doc(alias = "VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT")]
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY_BIT;
    #[doc(alias = "VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR")]
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY_BIT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlagBits.html>
    #[doc(alias = "VkBufferUsageFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct BufferUsageFlags: u32 {
        #[doc(alias = "VK_BUFFER_USAGE_TRANSFER_SRC_BIT")]
        const TRANSFER_SRC_BIT = 1;
        #[doc(alias = "VK_BUFFER_USAGE_TRANSFER_DST_BIT")]
        const TRANSFER_DST_BIT = 2;
        #[doc(alias = "VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT")]
        const UNIFORM_TEXEL_BUFFER_BIT = 4;
        #[doc(alias = "VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT")]
        const STORAGE_TEXEL_BUFFER_BIT = 8;
        #[doc(alias = "VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT")]
        const UNIFORM_BUFFER_BIT = 16;
        #[doc(alias = "VK_BUFFER_USAGE_STORAGE_BUFFER_BIT")]
        const STORAGE_BUFFER_BIT = 32;
        #[doc(alias = "VK_BUFFER_USAGE_INDEX_BUFFER_BIT")]
        const INDEX_BUFFER_BIT = 64;
        #[doc(alias = "VK_BUFFER_USAGE_VERTEX_BUFFER_BIT")]
        const VERTEX_BUFFER_BIT = 128;
        #[doc(alias = "VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT")]
        const INDIRECT_BUFFER_BIT = 256;
        #[doc(alias = "VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT")]
        const SHADER_DEVICE_ADDRESS_BIT = 131072;
        #[doc(alias = "VK_BUFFER_USAGE_VIDEO_DECODE_SRC_BIT_KHR")]
        const VIDEO_DECODE_SRC_BIT_KHR = 8192;
        #[doc(alias = "VK_BUFFER_USAGE_VIDEO_DECODE_DST_BIT_KHR")]
        const VIDEO_DECODE_DST_BIT_KHR = 16384;
        #[doc(alias = "VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_BUFFER_BIT_EXT")]
        const TRANSFORM_FEEDBACK_BUFFER_BIT_EXT = 2048;
        #[doc(alias = "VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT")]
        const TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT = 4096;
        #[doc(alias = "VK_BUFFER_USAGE_CONDITIONAL_RENDERING_BIT_EXT")]
        const CONDITIONAL_RENDERING_BIT_EXT = 512;
        #[doc(alias = "VK_BUFFER_USAGE_EXECUTION_GRAPH_SCRATCH_BIT_AMDX")]
        const EXECUTION_GRAPH_SCRATCH_BIT_AMDX = 33554432;
        #[doc(alias = "VK_BUFFER_USAGE_DESCRIPTOR_HEAP_BIT_EXT")]
        const DESCRIPTOR_HEAP_BIT_EXT = 268435456;
        #[doc(alias = "VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR")]
        const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR = 524288;
        #[doc(alias = "VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_STORAGE_BIT_KHR")]
        const ACCELERATION_STRUCTURE_STORAGE_BIT_KHR = 1048576;
        #[doc(alias = "VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR")]
        const SHADER_BINDING_TABLE_BIT_KHR = 1024;
        #[doc(alias = "VK_BUFFER_USAGE_VIDEO_ENCODE_DST_BIT_KHR")]
        const VIDEO_ENCODE_DST_BIT_KHR = 32768;
        #[doc(alias = "VK_BUFFER_USAGE_VIDEO_ENCODE_SRC_BIT_KHR")]
        const VIDEO_ENCODE_SRC_BIT_KHR = 65536;
        #[doc(alias = "VK_BUFFER_USAGE_SAMPLER_DESCRIPTOR_BUFFER_BIT_EXT")]
        const SAMPLER_DESCRIPTOR_BUFFER_BIT_EXT = 2097152;
        #[doc(alias = "VK_BUFFER_USAGE_RESOURCE_DESCRIPTOR_BUFFER_BIT_EXT")]
        const RESOURCE_DESCRIPTOR_BUFFER_BIT_EXT = 4194304;
        #[doc(alias = "VK_BUFFER_USAGE_PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_BIT_EXT")]
        const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_BIT_EXT = 67108864;
        #[doc(alias = "VK_BUFFER_USAGE_MICROMAP_BUILD_INPUT_READ_ONLY_BIT_EXT")]
        const MICROMAP_BUILD_INPUT_READ_ONLY_BIT_EXT = 8388608;
        #[doc(alias = "VK_BUFFER_USAGE_MICROMAP_STORAGE_BIT_EXT")]
        const MICROMAP_STORAGE_BIT_EXT = 16777216;
        #[doc(alias = "VK_BUFFER_USAGE_TILE_MEMORY_BIT_QCOM")]
        const TILE_MEMORY_BIT_QCOM = 134217728;
    }
}
impl BufferUsageFlags {
    #[doc(alias = "VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_EXT")]
    pub const SHADER_DEVICE_ADDRESS_BIT_EXT: Self = Self::SHADER_DEVICE_ADDRESS_BIT;
    #[doc(alias = "VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_KHR")]
    pub const SHADER_DEVICE_ADDRESS_BIT_KHR: Self = Self::SHADER_DEVICE_ADDRESS_BIT;
    #[doc(alias = "VK_BUFFER_USAGE_RAY_TRACING_BIT_NV")]
    pub const RAY_TRACING_BIT_NV: Self = Self::SHADER_BINDING_TABLE_BIT_KHR;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewCreateFlagBits.html>
    #[doc(alias = "VkImageViewCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ImageViewCreateFlags: u32 {
        #[doc(alias = "VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT")]
        const FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT = 1;
        #[doc(alias = "VK_IMAGE_VIEW_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT")]
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT = 4;
        #[doc(alias = "VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT")]
        const FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT = 2;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlagBits.html>
    #[doc(alias = "VkAccessFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct AccessFlags: u32 {
        #[doc(alias = "VK_ACCESS_INDIRECT_COMMAND_READ_BIT")]
        const INDIRECT_COMMAND_READ_BIT = 1;
        #[doc(alias = "VK_ACCESS_INDEX_READ_BIT")]
        const INDEX_READ_BIT = 2;
        #[doc(alias = "VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT")]
        const VERTEX_ATTRIBUTE_READ_BIT = 4;
        #[doc(alias = "VK_ACCESS_UNIFORM_READ_BIT")]
        const UNIFORM_READ_BIT = 8;
        #[doc(alias = "VK_ACCESS_INPUT_ATTACHMENT_READ_BIT")]
        const INPUT_ATTACHMENT_READ_BIT = 16;
        #[doc(alias = "VK_ACCESS_SHADER_READ_BIT")]
        const SHADER_READ_BIT = 32;
        #[doc(alias = "VK_ACCESS_SHADER_WRITE_BIT")]
        const SHADER_WRITE_BIT = 64;
        #[doc(alias = "VK_ACCESS_COLOR_ATTACHMENT_READ_BIT")]
        const COLOR_ATTACHMENT_READ_BIT = 128;
        #[doc(alias = "VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT")]
        const COLOR_ATTACHMENT_WRITE_BIT = 256;
        #[doc(alias = "VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT")]
        const DEPTH_STENCIL_ATTACHMENT_READ_BIT = 512;
        #[doc(alias = "VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT")]
        const DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = 1024;
        #[doc(alias = "VK_ACCESS_TRANSFER_READ_BIT")]
        const TRANSFER_READ_BIT = 2048;
        #[doc(alias = "VK_ACCESS_TRANSFER_WRITE_BIT")]
        const TRANSFER_WRITE_BIT = 4096;
        #[doc(alias = "VK_ACCESS_HOST_READ_BIT")]
        const HOST_READ_BIT = 8192;
        #[doc(alias = "VK_ACCESS_HOST_WRITE_BIT")]
        const HOST_WRITE_BIT = 16384;
        #[doc(alias = "VK_ACCESS_MEMORY_READ_BIT")]
        const MEMORY_READ_BIT = 32768;
        #[doc(alias = "VK_ACCESS_MEMORY_WRITE_BIT")]
        const MEMORY_WRITE_BIT = 65536;
        #[doc(alias = "VK_ACCESS_NONE")]
        const NONE = 0;
        #[doc(alias = "VK_ACCESS_TRANSFORM_FEEDBACK_WRITE_BIT_EXT")]
        const TRANSFORM_FEEDBACK_WRITE_BIT_EXT = 33554432;
        #[doc(alias = "VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT")]
        const TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT = 67108864;
        #[doc(alias = "VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT")]
        const TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT = 134217728;
        #[doc(alias = "VK_ACCESS_CONDITIONAL_RENDERING_READ_BIT_EXT")]
        const CONDITIONAL_RENDERING_READ_BIT_EXT = 1048576;
        #[doc(alias = "VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT")]
        const COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT = 524288;
        #[doc(alias = "VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR")]
        const ACCELERATION_STRUCTURE_READ_BIT_KHR = 2097152;
        #[doc(alias = "VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR")]
        const ACCELERATION_STRUCTURE_WRITE_BIT_KHR = 4194304;
        #[doc(alias = "VK_ACCESS_FRAGMENT_DENSITY_MAP_READ_BIT_EXT")]
        const FRAGMENT_DENSITY_MAP_READ_BIT_EXT = 16777216;
        #[doc(alias = "VK_ACCESS_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR")]
        const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR = 8388608;
        #[doc(alias = "VK_ACCESS_COMMAND_PREPROCESS_READ_BIT_EXT")]
        const COMMAND_PREPROCESS_READ_BIT_EXT = 131072;
        #[doc(alias = "VK_ACCESS_COMMAND_PREPROCESS_WRITE_BIT_EXT")]
        const COMMAND_PREPROCESS_WRITE_BIT_EXT = 262144;
    }
}
impl AccessFlags {
    #[doc(alias = "VK_ACCESS_NONE_KHR")]
    pub const NONE_KHR: Self = Self::NONE;
    #[doc(alias = "VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_NV")]
    pub const ACCELERATION_STRUCTURE_READ_BIT_NV: Self = Self::ACCELERATION_STRUCTURE_READ_BIT_KHR;
    #[doc(alias = "VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_NV")]
    pub const ACCELERATION_STRUCTURE_WRITE_BIT_NV: Self =
        Self::ACCELERATION_STRUCTURE_WRITE_BIT_KHR;
    #[doc(alias = "VK_ACCESS_SHADING_RATE_IMAGE_READ_BIT_NV")]
    pub const SHADING_RATE_IMAGE_READ_BIT_NV: Self =
        Self::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR;
    #[doc(alias = "VK_ACCESS_COMMAND_PREPROCESS_READ_BIT_NV")]
    pub const COMMAND_PREPROCESS_READ_BIT_NV: Self = Self::COMMAND_PREPROCESS_READ_BIT_EXT;
    #[doc(alias = "VK_ACCESS_COMMAND_PREPROCESS_WRITE_BIT_NV")]
    pub const COMMAND_PREPROCESS_WRITE_BIT_NV: Self = Self::COMMAND_PREPROCESS_WRITE_BIT_EXT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDependencyFlagBits.html>
    #[doc(alias = "VkDependencyFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DependencyFlags: u32 {
        #[doc(alias = "VK_DEPENDENCY_BY_REGION_BIT")]
        const BY_REGION_BIT = 1;
        #[doc(alias = "VK_DEPENDENCY_DEVICE_GROUP_BIT")]
        const DEVICE_GROUP_BIT = 4;
        #[doc(alias = "VK_DEPENDENCY_VIEW_LOCAL_BIT")]
        const VIEW_LOCAL_BIT = 2;
        #[doc(alias = "VK_DEPENDENCY_FEEDBACK_LOOP_BIT_EXT")]
        const FEEDBACK_LOOP_BIT_EXT = 8;
        #[doc(alias = "VK_DEPENDENCY_QUEUE_FAMILY_OWNERSHIP_TRANSFER_USE_ALL_STAGES_BIT_KHR")]
        const QUEUE_FAMILY_OWNERSHIP_TRANSFER_USE_ALL_STAGES_BIT_KHR = 32;
        #[doc(alias = "VK_DEPENDENCY_ASYMMETRIC_EVENT_BIT_KHR")]
        const ASYMMETRIC_EVENT_BIT_KHR = 64;
    }
}
impl DependencyFlags {
    #[doc(alias = "VK_DEPENDENCY_DEVICE_GROUP_BIT_KHR")]
    pub const DEVICE_GROUP_BIT_KHR: Self = Self::DEVICE_GROUP_BIT;
    #[doc(alias = "VK_DEPENDENCY_VIEW_LOCAL_BIT_KHR")]
    pub const VIEW_LOCAL_BIT_KHR: Self = Self::VIEW_LOCAL_BIT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolCreateFlagBits.html>
    #[doc(alias = "VkCommandPoolCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct CommandPoolCreateFlags: u32 {
        #[doc(alias = "VK_COMMAND_POOL_CREATE_TRANSIENT_BIT")]
        const TRANSIENT_BIT = 1;
        #[doc(alias = "VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT")]
        const RESET_COMMAND_BUFFER_BIT = 2;
        #[doc(alias = "VK_COMMAND_POOL_CREATE_PROTECTED_BIT")]
        const PROTECTED_BIT = 4;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolResetFlagBits.html>
    #[doc(alias = "VkCommandPoolResetFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct CommandPoolResetFlags: u32 {
        #[doc(alias = "VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT")]
        const RELEASE_RESOURCES_BIT = 1;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryControlFlagBits.html>
    #[doc(alias = "VkQueryControlFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct QueryControlFlags: u32 {
        #[doc(alias = "VK_QUERY_CONTROL_PRECISE_BIT")]
        const PRECISE_BIT = 1;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferUsageFlagBits.html>
    #[doc(alias = "VkCommandBufferUsageFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct CommandBufferUsageFlags: u32 {
        #[doc(alias = "VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT")]
        const ONE_TIME_SUBMIT_BIT = 1;
        #[doc(alias = "VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT")]
        const RENDER_PASS_CONTINUE_BIT = 2;
        #[doc(alias = "VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT")]
        const SIMULTANEOUS_USE_BIT = 4;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferResetFlagBits.html>
    #[doc(alias = "VkCommandBufferResetFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct CommandBufferResetFlags: u32 {
        #[doc(alias = "VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT")]
        const RELEASE_RESOURCES_BIT = 1;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkEventCreateFlagBits.html>
    #[doc(alias = "VkEventCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct EventCreateFlags: u32 {
        #[doc(alias = "VK_EVENT_CREATE_DEVICE_ONLY_BIT")]
        const DEVICE_ONLY_BIT = 1;
    }
}
impl EventCreateFlags {
    #[doc(alias = "VK_EVENT_CREATE_DEVICE_ONLY_BIT_KHR")]
    pub const DEVICE_ONLY_BIT_KHR: Self = Self::DEVICE_ONLY_BIT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCacheCreateFlagBits.html>
    #[doc(alias = "VkPipelineCacheCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PipelineCacheCreateFlags: u32 {
        #[doc(alias = "VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT")]
        const EXTERNALLY_SYNCHRONIZED_BIT = 1;
        #[doc(alias = "VK_PIPELINE_CACHE_CREATE_INTERNALLY_SYNCHRONIZED_MERGE_BIT_KHR")]
        const INTERNALLY_SYNCHRONIZED_MERGE_BIT_KHR = 8;
    }
}
impl PipelineCacheCreateFlags {
    #[doc(alias = "VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT_EXT")]
    pub const EXTERNALLY_SYNCHRONIZED_BIT_EXT: Self = Self::EXTERNALLY_SYNCHRONIZED_BIT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlagBits.html>
    #[doc(alias = "VkPipelineCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PipelineCreateFlags: u32 {
        #[doc(alias = "VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT")]
        const DISABLE_OPTIMIZATION_BIT = 1;
        #[doc(alias = "VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT")]
        const ALLOW_DERIVATIVES_BIT = 2;
        #[doc(alias = "VK_PIPELINE_CREATE_DERIVATIVE_BIT")]
        const DERIVATIVE_BIT = 4;
        #[doc(alias = "VK_PIPELINE_CREATE_DISPATCH_BASE_BIT")]
        const DISPATCH_BASE_BIT = 16;
        #[doc(alias = "VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT")]
        const VIEW_INDEX_FROM_DEVICE_INDEX_BIT = 8;
        #[doc(alias = "VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT")]
        const FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT = 256;
        #[doc(alias = "VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT")]
        const EARLY_RETURN_ON_FAILURE_BIT = 512;
        #[doc(alias = "VK_PIPELINE_CREATE_NO_PROTECTED_ACCESS_BIT")]
        const NO_PROTECTED_ACCESS_BIT = 134217728;
        #[doc(alias = "VK_PIPELINE_CREATE_PROTECTED_ACCESS_ONLY_BIT")]
        const PROTECTED_ACCESS_ONLY_BIT = 1073741824;
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR")]
        const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR = 16384;
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR")]
        const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR = 32768;
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR")]
        const RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR = 65536;
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR")]
        const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR = 131072;
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR")]
        const RAY_TRACING_SKIP_TRIANGLES_BIT_KHR = 4096;
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR")]
        const RAY_TRACING_SKIP_AABBS_BIT_KHR = 8192;
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR")]
        const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR = 524288;
        #[doc(alias = "VK_PIPELINE_CREATE_DEFER_COMPILE_BIT_NV")]
        const DEFER_COMPILE_BIT_NV = 32;
        #[doc(alias = "VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT")]
        const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT = 4194304;
        #[doc(alias = "VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
        const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 2097152;
        #[doc(alias = "VK_PIPELINE_CREATE_CAPTURE_STATISTICS_BIT_KHR")]
        const CAPTURE_STATISTICS_BIT_KHR = 64;
        #[doc(alias = "VK_PIPELINE_CREATE_CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR")]
        const CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR = 128;
        #[doc(alias = "VK_PIPELINE_CREATE_INDIRECT_BINDABLE_BIT_NV")]
        const INDIRECT_BINDABLE_BIT_NV = 262144;
        #[doc(alias = "VK_PIPELINE_CREATE_LIBRARY_BIT_KHR")]
        const LIBRARY_BIT_KHR = 2048;
        #[doc(alias = "VK_PIPELINE_CREATE_DESCRIPTOR_BUFFER_BIT_EXT")]
        const DESCRIPTOR_BUFFER_BIT_EXT = 536870912;
        #[doc(alias = "VK_PIPELINE_CREATE_RETAIN_LINK_TIME_OPTIMIZATION_INFO_BIT_EXT")]
        const RETAIN_LINK_TIME_OPTIMIZATION_INFO_BIT_EXT = 8388608;
        #[doc(alias = "VK_PIPELINE_CREATE_LINK_TIME_OPTIMIZATION_BIT_EXT")]
        const LINK_TIME_OPTIMIZATION_BIT_EXT = 1024;
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_ALLOW_MOTION_BIT_NV")]
        const RAY_TRACING_ALLOW_MOTION_BIT_NV = 1048576;
        #[doc(alias = "VK_PIPELINE_CREATE_COLOR_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT")]
        const COLOR_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT = 33554432;
        #[doc(alias = "VK_PIPELINE_CREATE_DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT")]
        const DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT = 67108864;
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_DISPLACEMENT_MICROMAP_BIT_NV")]
        const RAY_TRACING_DISPLACEMENT_MICROMAP_BIT_NV = 268435456;
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_OPACITY_MICROMAP_BIT_KHR")]
        const RAY_TRACING_OPACITY_MICROMAP_BIT_KHR = 16777216;
    }
}
impl PipelineCreateFlags {
    #[doc(alias = "VK_PIPELINE_CREATE_DISPATCH_BASE")]
    pub const DISPATCH_BASE: Self = Self::DISPATCH_BASE_BIT;
    #[doc(alias = "VK_PIPELINE_CREATE_DISPATCH_BASE_BIT_KHR")]
    pub const DISPATCH_BASE_BIT_KHR: Self = Self::DISPATCH_BASE_BIT;
    #[doc(alias = "VK_PIPELINE_CREATE_DISPATCH_BASE_KHR")]
    pub const DISPATCH_BASE_KHR: Self = Self::DISPATCH_BASE_BIT;
    #[doc(alias = "VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR")]
    pub const VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR: Self = Self::VIEW_INDEX_FROM_DEVICE_INDEX_BIT;
    #[doc(alias = "VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT_EXT")]
    pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT_EXT: Self =
        Self::FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT;
    #[doc(alias = "VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT_EXT")]
    pub const EARLY_RETURN_ON_FAILURE_BIT_EXT: Self = Self::EARLY_RETURN_ON_FAILURE_BIT;
    #[doc(alias = "VK_PIPELINE_CREATE_NO_PROTECTED_ACCESS_BIT_EXT")]
    pub const NO_PROTECTED_ACCESS_BIT_EXT: Self = Self::NO_PROTECTED_ACCESS_BIT;
    #[doc(alias = "VK_PIPELINE_CREATE_PROTECTED_ACCESS_ONLY_BIT_EXT")]
    pub const PROTECTED_ACCESS_ONLY_BIT_EXT: Self = Self::PROTECTED_ACCESS_ONLY_BIT;
    #[doc(alias = "VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT")]
    pub const PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT: Self =
        Self::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT;
    #[doc(
        alias = "VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR"
    )]
    pub const PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: Self =
        Self::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR;
    #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_OPACITY_MICROMAP_BIT_EXT")]
    pub const RAY_TRACING_OPACITY_MICROMAP_BIT_EXT: Self =
        Self::RAY_TRACING_OPACITY_MICROMAP_BIT_KHR;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineLayoutCreateFlagBits.html>
    #[doc(alias = "VkPipelineLayoutCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PipelineLayoutCreateFlags: u32 {
        #[doc(alias = "VK_PIPELINE_LAYOUT_CREATE_INDEPENDENT_SETS_BIT_EXT")]
        const INDEPENDENT_SETS_BIT_EXT = 2;
        #[doc(alias = "VK_PIPELINE_LAYOUT_CREATE_NO_TASK_SHADER_BIT_KHR")]
        const NO_TASK_SHADER_BIT_KHR = 4;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineShaderStageCreateFlagBits.html>
    #[doc(alias = "VkPipelineShaderStageCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PipelineShaderStageCreateFlags: u32 {
        #[doc(alias = "VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT")]
        const ALLOW_VARYING_SUBGROUP_SIZE_BIT = 1;
        #[doc(alias = "VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT")]
        const REQUIRE_FULL_SUBGROUPS_BIT = 2;
    }
}
impl PipelineShaderStageCreateFlags {
    #[doc(alias = "VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT_EXT")]
    pub const ALLOW_VARYING_SUBGROUP_SIZE_BIT_EXT: Self = Self::ALLOW_VARYING_SUBGROUP_SIZE_BIT;
    #[doc(alias = "VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT_EXT")]
    pub const REQUIRE_FULL_SUBGROUPS_BIT_EXT: Self = Self::REQUIRE_FULL_SUBGROUPS_BIT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerCreateFlagBits.html>
    #[doc(alias = "VkSamplerCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SamplerCreateFlags: u32 {
        #[doc(alias = "VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT")]
        const SUBSAMPLED_BIT_EXT = 1;
        #[doc(alias = "VK_SAMPLER_CREATE_SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT")]
        const SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT = 2;
        #[doc(alias = "VK_SAMPLER_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT")]
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT = 8;
        #[doc(alias = "VK_SAMPLER_CREATE_NON_SEAMLESS_CUBE_MAP_BIT_EXT")]
        const NON_SEAMLESS_CUBE_MAP_BIT_EXT = 4;
        #[doc(alias = "VK_SAMPLER_CREATE_IMAGE_PROCESSING_BIT_QCOM")]
        const IMAGE_PROCESSING_BIT_QCOM = 16;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPoolCreateFlagBits.html>
    #[doc(alias = "VkDescriptorPoolCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DescriptorPoolCreateFlags: u32 {
        #[doc(alias = "VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT")]
        const FREE_DESCRIPTOR_SET_BIT = 1;
        #[doc(alias = "VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT")]
        const UPDATE_AFTER_BIND_BIT = 2;
        #[doc(alias = "VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_EXT")]
        const HOST_ONLY_BIT_EXT = 4;
        #[doc(alias = "VK_DESCRIPTOR_POOL_CREATE_ALLOW_OVERALLOCATION_SETS_BIT_NV")]
        const ALLOW_OVERALLOCATION_SETS_BIT_NV = 8;
        #[doc(alias = "VK_DESCRIPTOR_POOL_CREATE_ALLOW_OVERALLOCATION_POOLS_BIT_NV")]
        const ALLOW_OVERALLOCATION_POOLS_BIT_NV = 16;
    }
}
impl DescriptorPoolCreateFlags {
    #[doc(alias = "VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT_EXT")]
    pub const UPDATE_AFTER_BIND_BIT_EXT: Self = Self::UPDATE_AFTER_BIND_BIT;
    #[doc(alias = "VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE")]
    pub const HOST_ONLY_BIT_VALVE: Self = Self::HOST_ONLY_BIT_EXT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayoutCreateFlagBits.html>
    #[doc(alias = "VkDescriptorSetLayoutCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DescriptorSetLayoutCreateFlags: u32 {
        #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT")]
        const UPDATE_AFTER_BIND_POOL_BIT = 2;
        #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT")]
        const PUSH_DESCRIPTOR_BIT = 1;
        #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_DESCRIPTOR_BUFFER_BIT_EXT")]
        const DESCRIPTOR_BUFFER_BIT_EXT = 16;
        #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_EMBEDDED_IMMUTABLE_SAMPLERS_BIT_EXT")]
        const EMBEDDED_IMMUTABLE_SAMPLERS_BIT_EXT = 32;
        #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_INDIRECT_BINDABLE_BIT_NV")]
        const INDIRECT_BINDABLE_BIT_NV = 128;
        #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_EXT")]
        const HOST_ONLY_POOL_BIT_EXT = 4;
        #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_PER_STAGE_BIT_NV")]
        const PER_STAGE_BIT_NV = 64;
    }
}
impl DescriptorSetLayoutCreateFlags {
    #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT_EXT")]
    pub const UPDATE_AFTER_BIND_POOL_BIT_EXT: Self = Self::UPDATE_AFTER_BIND_POOL_BIT;
    #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR")]
    pub const PUSH_DESCRIPTOR_BIT_KHR: Self = Self::PUSH_DESCRIPTOR_BIT;
    #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_VALVE")]
    pub const HOST_ONLY_POOL_BIT_VALVE: Self = Self::HOST_ONLY_POOL_BIT_EXT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkColorComponentFlagBits.html>
    #[doc(alias = "VkColorComponentFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ColorComponentFlags: u32 {
        #[doc(alias = "VK_COLOR_COMPONENT_R_BIT")]
        const R_BIT = 1;
        #[doc(alias = "VK_COLOR_COMPONENT_G_BIT")]
        const G_BIT = 2;
        #[doc(alias = "VK_COLOR_COMPONENT_B_BIT")]
        const B_BIT = 4;
        #[doc(alias = "VK_COLOR_COMPONENT_A_BIT")]
        const A_BIT = 8;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCullModeFlagBits.html>
    #[doc(alias = "VkCullModeFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct CullModeFlags: u32 {
        #[doc(alias = "VK_CULL_MODE_NONE")]
        const NONE = 0;
        #[doc(alias = "VK_CULL_MODE_FRONT_BIT")]
        const FRONT_BIT = 1;
        #[doc(alias = "VK_CULL_MODE_BACK_BIT")]
        const BACK_BIT = 2;
        #[doc(alias = "VK_CULL_MODE_FRONT_AND_BACK")]
        const FRONT_AND_BACK = 3;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineColorBlendStateCreateFlagBits.html>
    #[doc(alias = "VkPipelineColorBlendStateCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PipelineColorBlendStateCreateFlags: u32 {
        #[doc(alias = "VK_PIPELINE_COLOR_BLEND_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_EXT")]
        const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_EXT = 1;
    }
}
impl PipelineColorBlendStateCreateFlags {
    #[doc(
        alias = "VK_PIPELINE_COLOR_BLEND_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_ARM"
    )]
    pub const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_EXT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineDepthStencilStateCreateFlagBits.html>
    #[doc(alias = "VkPipelineDepthStencilStateCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PipelineDepthStencilStateCreateFlags: u32 {
        #[doc(alias = "VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT")]
        const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT = 1;
        #[doc(alias = "VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT")]
        const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT = 2;
    }
}
impl PipelineDepthStencilStateCreateFlags {
    #[doc(
        alias = "VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM"
    )]
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT;
    #[doc(
        alias = "VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM"
    )]
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentDescriptionFlagBits.html>
    #[doc(alias = "VkAttachmentDescriptionFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct AttachmentDescriptionFlags: u32 {
        #[doc(alias = "VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT")]
        const MAY_ALIAS_BIT = 1;
        #[doc(alias = "VK_ATTACHMENT_DESCRIPTION_RESOLVE_SKIP_TRANSFER_FUNCTION_BIT_KHR")]
        const RESOLVE_SKIP_TRANSFER_FUNCTION_BIT_KHR = 2;
        #[doc(alias = "VK_ATTACHMENT_DESCRIPTION_RESOLVE_ENABLE_TRANSFER_FUNCTION_BIT_KHR")]
        const RESOLVE_ENABLE_TRANSFER_FUNCTION_BIT_KHR = 4;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebufferCreateFlagBits.html>
    #[doc(alias = "VkFramebufferCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct FramebufferCreateFlags: u32 {
        #[doc(alias = "VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT")]
        const IMAGELESS_BIT = 1;
    }
}
impl FramebufferCreateFlags {
    #[doc(alias = "VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT_KHR")]
    pub const IMAGELESS_BIT_KHR: Self = Self::IMAGELESS_BIT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassCreateFlagBits.html>
    #[doc(alias = "VkRenderPassCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct RenderPassCreateFlags: u32 {
        #[doc(alias = "VK_RENDER_PASS_CREATE_TRANSFORM_BIT_QCOM")]
        const TRANSFORM_BIT_QCOM = 2;
        #[doc(alias = "VK_RENDER_PASS_CREATE_PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE")]
        const PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE = 4;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassDescriptionFlagBits.html>
    #[doc(alias = "VkSubpassDescriptionFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SubpassDescriptionFlags: u32 {
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX")]
        const PER_VIEW_ATTRIBUTES_BIT_NVX = 1;
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_PER_VIEW_POSITION_X_ONLY_BIT_NVX")]
        const PER_VIEW_POSITION_X_ONLY_BIT_NVX = 2;
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_TILE_SHADING_APRON_BIT_QCOM")]
        const TILE_SHADING_APRON_BIT_QCOM = 256;
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_EXT")]
        const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_EXT = 16;
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT")]
        const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT = 32;
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT")]
        const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT = 64;
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_ENABLE_LEGACY_DITHERING_BIT_EXT")]
        const ENABLE_LEGACY_DITHERING_BIT_EXT = 128;
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_FRAGMENT_REGION_BIT_EXT")]
        const FRAGMENT_REGION_BIT_EXT = 4;
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_CUSTOM_RESOLVE_BIT_EXT")]
        const CUSTOM_RESOLVE_BIT_EXT = 8;
    }
}
impl SubpassDescriptionFlags {
    #[doc(alias = "VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_ARM")]
    pub const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_EXT;
    #[doc(alias = "VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM")]
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT;
    #[doc(alias = "VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM")]
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT;
    #[doc(alias = "VK_SUBPASS_DESCRIPTION_FRAGMENT_REGION_BIT_QCOM")]
    pub const FRAGMENT_REGION_BIT_QCOM: Self = Self::FRAGMENT_REGION_BIT_EXT;
    #[doc(alias = "VK_SUBPASS_DESCRIPTION_SHADER_RESOLVE_BIT_QCOM")]
    pub const SHADER_RESOLVE_BIT_QCOM: Self = Self::CUSTOM_RESOLVE_BIT_EXT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkStencilFaceFlagBits.html>
    #[doc(alias = "VkStencilFaceFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct StencilFaceFlags: u32 {
        #[doc(alias = "VK_STENCIL_FACE_FRONT_BIT")]
        const FRONT_BIT = 1;
        #[doc(alias = "VK_STENCIL_FACE_BACK_BIT")]
        const BACK_BIT = 2;
        #[doc(alias = "VK_STENCIL_FACE_FRONT_AND_BACK")]
        const FRONT_AND_BACK = 3;
    }
}
impl StencilFaceFlags {
    #[doc(alias = "VK_STENCIL_FRONT_AND_BACK")]
    pub const STENCIL_FRONT_AND_BACK: Self = Self::FRONT_AND_BACK;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubgroupFeatureFlagBits.html>
    #[doc(alias = "VkSubgroupFeatureFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SubgroupFeatureFlags: u32 {
        #[doc(alias = "VK_SUBGROUP_FEATURE_BASIC_BIT")]
        const BASIC_BIT = 1;
        #[doc(alias = "VK_SUBGROUP_FEATURE_VOTE_BIT")]
        const VOTE_BIT = 2;
        #[doc(alias = "VK_SUBGROUP_FEATURE_ARITHMETIC_BIT")]
        const ARITHMETIC_BIT = 4;
        #[doc(alias = "VK_SUBGROUP_FEATURE_BALLOT_BIT")]
        const BALLOT_BIT = 8;
        #[doc(alias = "VK_SUBGROUP_FEATURE_SHUFFLE_BIT")]
        const SHUFFLE_BIT = 16;
        #[doc(alias = "VK_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT")]
        const SHUFFLE_RELATIVE_BIT = 32;
        #[doc(alias = "VK_SUBGROUP_FEATURE_CLUSTERED_BIT")]
        const CLUSTERED_BIT = 64;
        #[doc(alias = "VK_SUBGROUP_FEATURE_QUAD_BIT")]
        const QUAD_BIT = 128;
        #[doc(alias = "VK_SUBGROUP_FEATURE_ROTATE_BIT")]
        const ROTATE_BIT = 512;
        #[doc(alias = "VK_SUBGROUP_FEATURE_ROTATE_CLUSTERED_BIT")]
        const ROTATE_CLUSTERED_BIT = 1024;
        #[doc(alias = "VK_SUBGROUP_FEATURE_PARTITIONED_BIT_EXT")]
        const PARTITIONED_BIT_EXT = 256;
    }
}
impl SubgroupFeatureFlags {
    #[doc(alias = "VK_SUBGROUP_FEATURE_ROTATE_BIT_KHR")]
    pub const ROTATE_BIT_KHR: Self = Self::ROTATE_BIT;
    #[doc(alias = "VK_SUBGROUP_FEATURE_ROTATE_CLUSTERED_BIT_KHR")]
    pub const ROTATE_CLUSTERED_BIT_KHR: Self = Self::ROTATE_CLUSTERED_BIT;
    #[doc(alias = "VK_SUBGROUP_FEATURE_PARTITIONED_BIT_NV")]
    pub const PARTITIONED_BIT_NV: Self = Self::PARTITIONED_BIT_EXT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPeerMemoryFeatureFlagBits.html>
    #[doc(alias = "VkPeerMemoryFeatureFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PeerMemoryFeatureFlags: u32 {
        #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT")]
        const COPY_SRC_BIT = 1;
        #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_DST_BIT")]
        const COPY_DST_BIT = 2;
        #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT")]
        const GENERIC_SRC_BIT = 4;
        #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT")]
        const GENERIC_DST_BIT = 8;
    }
}
pub type PeerMemoryFeatureFlagsKHR = PeerMemoryFeatureFlags;
impl PeerMemoryFeatureFlags {
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT_KHR")]
    pub const COPY_SRC_BIT_KHR: Self = Self::COPY_SRC_BIT;
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_DST_BIT_KHR")]
    pub const COPY_DST_BIT_KHR: Self = Self::COPY_DST_BIT;
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT_KHR")]
    pub const GENERIC_SRC_BIT_KHR: Self = Self::GENERIC_SRC_BIT;
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT_KHR")]
    pub const GENERIC_DST_BIT_KHR: Self = Self::GENERIC_DST_BIT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryAllocateFlagBits.html>
    #[doc(alias = "VkMemoryAllocateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct MemoryAllocateFlags: u32 {
        #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT")]
        const DEVICE_MASK_BIT = 1;
        #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT")]
        const DEVICE_ADDRESS_BIT = 2;
        #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT")]
        const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT = 4;
        #[doc(alias = "VK_MEMORY_ALLOCATE_ZERO_INITIALIZE_BIT_EXT")]
        const ZERO_INITIALIZE_BIT_EXT = 8;
    }
}
pub type MemoryAllocateFlagsKHR = MemoryAllocateFlags;
impl MemoryAllocateFlags {
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT_KHR")]
    pub const DEVICE_MASK_BIT_KHR: Self = Self::DEVICE_MASK_BIT;
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT_KHR")]
    pub const DEVICE_ADDRESS_BIT_KHR: Self = Self::DEVICE_ADDRESS_BIT;
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR")]
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY_BIT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryHandleTypeFlagBits.html>
    #[doc(alias = "VkExternalMemoryHandleTypeFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ExternalMemoryHandleTypeFlags: u32 {
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT")]
        const OPAQUE_FD_BIT = 1;
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT")]
        const OPAQUE_WIN32_BIT = 2;
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT")]
        const OPAQUE_WIN32_KMT_BIT = 4;
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT")]
        const D3D11_TEXTURE_BIT = 8;
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT")]
        const D3D11_TEXTURE_KMT_BIT = 16;
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT")]
        const D3D12_HEAP_BIT = 32;
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT")]
        const D3D12_RESOURCE_BIT = 64;
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT")]
        const DMA_BUF_BIT_EXT = 512;
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID")]
        const ANDROID_HARDWARE_BUFFER_BIT_ANDROID = 1024;
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT")]
        const HOST_ALLOCATION_BIT_EXT = 128;
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT")]
        const HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT = 256;
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA")]
        const ZIRCON_VMO_BIT_FUCHSIA = 2048;
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_RDMA_ADDRESS_BIT_NV")]
        const RDMA_ADDRESS_BIT_NV = 4096;
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OH_NATIVE_BUFFER_BIT_OHOS")]
        const OH_NATIVE_BUFFER_BIT_OHOS = 32768;
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_SCREEN_BUFFER_BIT_QNX")]
        const SCREEN_BUFFER_BIT_QNX = 16384;
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_MTLBUFFER_BIT_EXT")]
        const MTLBUFFER_BIT_EXT = 65536;
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_MTLTEXTURE_BIT_EXT")]
        const MTLTEXTURE_BIT_EXT = 131072;
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_MTLHEAP_BIT_EXT")]
        const MTLHEAP_BIT_EXT = 262144;
    }
}
pub type ExternalMemoryHandleTypeFlagsKHR = ExternalMemoryHandleTypeFlags;
impl ExternalMemoryHandleTypeFlags {
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR")]
    pub const OPAQUE_FD_BIT_KHR: Self = Self::OPAQUE_FD_BIT;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR")]
    pub const OPAQUE_WIN32_BIT_KHR: Self = Self::OPAQUE_WIN32_BIT;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR")]
    pub const OPAQUE_WIN32_KMT_BIT_KHR: Self = Self::OPAQUE_WIN32_KMT_BIT;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR")]
    pub const D3D11_TEXTURE_BIT_KHR: Self = Self::D3D11_TEXTURE_BIT;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR")]
    pub const D3D11_TEXTURE_KMT_BIT_KHR: Self = Self::D3D11_TEXTURE_KMT_BIT;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR")]
    pub const D3D12_HEAP_BIT_KHR: Self = Self::D3D12_HEAP_BIT;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR")]
    pub const D3D12_RESOURCE_BIT_KHR: Self = Self::D3D12_RESOURCE_BIT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryFeatureFlagBits.html>
    #[doc(alias = "VkExternalMemoryFeatureFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ExternalMemoryFeatureFlags: u32 {
        #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT")]
        const DEDICATED_ONLY_BIT = 1;
        #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT")]
        const EXPORTABLE_BIT = 2;
        #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT")]
        const IMPORTABLE_BIT = 4;
    }
}
pub type ExternalMemoryFeatureFlagsKHR = ExternalMemoryFeatureFlags;
impl ExternalMemoryFeatureFlags {
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_KHR")]
    pub const DEDICATED_ONLY_BIT_KHR: Self = Self::DEDICATED_ONLY_BIT;
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_KHR")]
    pub const EXPORTABLE_BIT_KHR: Self = Self::EXPORTABLE_BIT;
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_KHR")]
    pub const IMPORTABLE_BIT_KHR: Self = Self::IMPORTABLE_BIT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceHandleTypeFlagBits.html>
    #[doc(alias = "VkExternalFenceHandleTypeFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ExternalFenceHandleTypeFlags: u32 {
        #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT")]
        const OPAQUE_FD_BIT = 1;
        #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT")]
        const OPAQUE_WIN32_BIT = 2;
        #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT")]
        const OPAQUE_WIN32_KMT_BIT = 4;
        #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT")]
        const SYNC_FD_BIT = 8;
    }
}
pub type ExternalFenceHandleTypeFlagsKHR = ExternalFenceHandleTypeFlags;
impl ExternalFenceHandleTypeFlags {
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR")]
    pub const OPAQUE_FD_BIT_KHR: Self = Self::OPAQUE_FD_BIT;
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR")]
    pub const OPAQUE_WIN32_BIT_KHR: Self = Self::OPAQUE_WIN32_BIT;
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR")]
    pub const OPAQUE_WIN32_KMT_BIT_KHR: Self = Self::OPAQUE_WIN32_KMT_BIT;
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR")]
    pub const SYNC_FD_BIT_KHR: Self = Self::SYNC_FD_BIT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceFeatureFlagBits.html>
    #[doc(alias = "VkExternalFenceFeatureFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ExternalFenceFeatureFlags: u32 {
        #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT")]
        const EXPORTABLE_BIT = 1;
        #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT")]
        const IMPORTABLE_BIT = 2;
    }
}
pub type ExternalFenceFeatureFlagsKHR = ExternalFenceFeatureFlags;
impl ExternalFenceFeatureFlags {
    #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR")]
    pub const EXPORTABLE_BIT_KHR: Self = Self::EXPORTABLE_BIT;
    #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR")]
    pub const IMPORTABLE_BIT_KHR: Self = Self::IMPORTABLE_BIT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceImportFlagBits.html>
    #[doc(alias = "VkFenceImportFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct FenceImportFlags: u32 {
        #[doc(alias = "VK_FENCE_IMPORT_TEMPORARY_BIT")]
        const TEMPORARY_BIT = 1;
    }
}
pub type FenceImportFlagsKHR = FenceImportFlags;
impl FenceImportFlags {
    #[doc(alias = "VK_FENCE_IMPORT_TEMPORARY_BIT_KHR")]
    pub const TEMPORARY_BIT_KHR: Self = Self::TEMPORARY_BIT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreImportFlagBits.html>
    #[doc(alias = "VkSemaphoreImportFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SemaphoreImportFlags: u32 {
        #[doc(alias = "VK_SEMAPHORE_IMPORT_TEMPORARY_BIT")]
        const TEMPORARY_BIT = 1;
    }
}
pub type SemaphoreImportFlagsKHR = SemaphoreImportFlags;
impl SemaphoreImportFlags {
    #[doc(alias = "VK_SEMAPHORE_IMPORT_TEMPORARY_BIT_KHR")]
    pub const TEMPORARY_BIT_KHR: Self = Self::TEMPORARY_BIT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreHandleTypeFlagBits.html>
    #[doc(alias = "VkExternalSemaphoreHandleTypeFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ExternalSemaphoreHandleTypeFlags: u32 {
        #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT")]
        const OPAQUE_FD_BIT = 1;
        #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT")]
        const OPAQUE_WIN32_BIT = 2;
        #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT")]
        const OPAQUE_WIN32_KMT_BIT = 4;
        #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT")]
        const D3D12_FENCE_BIT = 8;
        #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT")]
        const SYNC_FD_BIT = 16;
        #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_ZIRCON_EVENT_BIT_FUCHSIA")]
        const ZIRCON_EVENT_BIT_FUCHSIA = 128;
    }
}
pub type ExternalSemaphoreHandleTypeFlagsKHR = ExternalSemaphoreHandleTypeFlags;
impl ExternalSemaphoreHandleTypeFlags {
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR")]
    pub const OPAQUE_FD_BIT_KHR: Self = Self::OPAQUE_FD_BIT;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR")]
    pub const OPAQUE_WIN32_BIT_KHR: Self = Self::OPAQUE_WIN32_BIT;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR")]
    pub const OPAQUE_WIN32_KMT_BIT_KHR: Self = Self::OPAQUE_WIN32_KMT_BIT;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT")]
    pub const D3D11_FENCE_BIT: Self = Self::D3D12_FENCE_BIT;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR")]
    pub const D3D12_FENCE_BIT_KHR: Self = Self::D3D12_FENCE_BIT;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT_KHR")]
    pub const SYNC_FD_BIT_KHR: Self = Self::SYNC_FD_BIT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreFeatureFlagBits.html>
    #[doc(alias = "VkExternalSemaphoreFeatureFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ExternalSemaphoreFeatureFlags: u32 {
        #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT")]
        const EXPORTABLE_BIT = 1;
        #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT")]
        const IMPORTABLE_BIT = 2;
    }
}
pub type ExternalSemaphoreFeatureFlagsKHR = ExternalSemaphoreFeatureFlags;
impl ExternalSemaphoreFeatureFlags {
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHR")]
    pub const EXPORTABLE_BIT_KHR: Self = Self::EXPORTABLE_BIT;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHR")]
    pub const IMPORTABLE_BIT_KHR: Self = Self::IMPORTABLE_BIT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkResolveModeFlagBits.html>
    #[doc(alias = "VkResolveModeFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ResolveModeFlags: u32 {
        #[doc(alias = "VK_RESOLVE_MODE_NONE")]
        const NONE = 0;
        #[doc(alias = "VK_RESOLVE_MODE_SAMPLE_ZERO_BIT")]
        const SAMPLE_ZERO_BIT = 1;
        #[doc(alias = "VK_RESOLVE_MODE_AVERAGE_BIT")]
        const AVERAGE_BIT = 2;
        #[doc(alias = "VK_RESOLVE_MODE_MIN_BIT")]
        const MIN_BIT = 4;
        #[doc(alias = "VK_RESOLVE_MODE_MAX_BIT")]
        const MAX_BIT = 8;
        #[doc(alias = "VK_RESOLVE_MODE_EXTERNAL_FORMAT_DOWNSAMPLE_BIT_ANDROID")]
        const EXTERNAL_FORMAT_DOWNSAMPLE_BIT_ANDROID = 16;
        #[doc(alias = "VK_RESOLVE_MODE_CUSTOM_BIT_EXT")]
        const CUSTOM_BIT_EXT = 32;
    }
}
pub type ResolveModeFlagsKHR = ResolveModeFlags;
impl ResolveModeFlags {
    #[doc(alias = "VK_RESOLVE_MODE_NONE_KHR")]
    pub const NONE_KHR: Self = Self::NONE;
    #[doc(alias = "VK_RESOLVE_MODE_SAMPLE_ZERO_BIT_KHR")]
    pub const SAMPLE_ZERO_BIT_KHR: Self = Self::SAMPLE_ZERO_BIT;
    #[doc(alias = "VK_RESOLVE_MODE_AVERAGE_BIT_KHR")]
    pub const AVERAGE_BIT_KHR: Self = Self::AVERAGE_BIT;
    #[doc(alias = "VK_RESOLVE_MODE_MIN_BIT_KHR")]
    pub const MIN_BIT_KHR: Self = Self::MIN_BIT;
    #[doc(alias = "VK_RESOLVE_MODE_MAX_BIT_KHR")]
    pub const MAX_BIT_KHR: Self = Self::MAX_BIT;
    #[doc(alias = "VK_RESOLVE_MODE_EXTERNAL_FORMAT_DOWNSAMPLE_ANDROID")]
    pub const EXTERNAL_FORMAT_DOWNSAMPLE_ANDROID: Self =
        Self::EXTERNAL_FORMAT_DOWNSAMPLE_BIT_ANDROID;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreWaitFlagBits.html>
    #[doc(alias = "VkSemaphoreWaitFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SemaphoreWaitFlags: u32 {
        #[doc(alias = "VK_SEMAPHORE_WAIT_ANY_BIT")]
        const ANY_BIT = 1;
    }
}
pub type SemaphoreWaitFlagsKHR = SemaphoreWaitFlags;
impl SemaphoreWaitFlags {
    #[doc(alias = "VK_SEMAPHORE_WAIT_ANY_BIT_KHR")]
    pub const ANY_BIT_KHR: Self = Self::ANY_BIT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorBindingFlagBits.html>
    #[doc(alias = "VkDescriptorBindingFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DescriptorBindingFlags: u32 {
        #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT")]
        const UPDATE_AFTER_BIND_BIT = 1;
        #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT")]
        const UPDATE_UNUSED_WHILE_PENDING_BIT = 2;
        #[doc(alias = "VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT")]
        const PARTIALLY_BOUND_BIT = 4;
        #[doc(alias = "VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT")]
        const VARIABLE_DESCRIPTOR_COUNT_BIT = 8;
    }
}
pub type DescriptorBindingFlagsEXT = DescriptorBindingFlags;
impl DescriptorBindingFlags {
    #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT_EXT")]
    pub const UPDATE_AFTER_BIND_BIT_EXT: Self = Self::UPDATE_AFTER_BIND_BIT;
    #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT_EXT")]
    pub const UPDATE_UNUSED_WHILE_PENDING_BIT_EXT: Self = Self::UPDATE_UNUSED_WHILE_PENDING_BIT;
    #[doc(alias = "VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT_EXT")]
    pub const PARTIALLY_BOUND_BIT_EXT: Self = Self::PARTIALLY_BOUND_BIT;
    #[doc(alias = "VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT_EXT")]
    pub const VARIABLE_DESCRIPTOR_COUNT_BIT_EXT: Self = Self::VARIABLE_DESCRIPTOR_COUNT_BIT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkToolPurposeFlagBits.html>
    #[doc(alias = "VkToolPurposeFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ToolPurposeFlags: u32 {
        #[doc(alias = "VK_TOOL_PURPOSE_VALIDATION_BIT")]
        const VALIDATION_BIT = 1;
        #[doc(alias = "VK_TOOL_PURPOSE_PROFILING_BIT")]
        const PROFILING_BIT = 2;
        #[doc(alias = "VK_TOOL_PURPOSE_TRACING_BIT")]
        const TRACING_BIT = 4;
        #[doc(alias = "VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT")]
        const ADDITIONAL_FEATURES_BIT = 8;
        #[doc(alias = "VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT")]
        const MODIFYING_FEATURES_BIT = 16;
        #[doc(alias = "VK_TOOL_PURPOSE_DEBUG_REPORTING_BIT_EXT")]
        const DEBUG_REPORTING_BIT_EXT = 32;
        #[doc(alias = "VK_TOOL_PURPOSE_DEBUG_MARKERS_BIT_EXT")]
        const DEBUG_MARKERS_BIT_EXT = 64;
    }
}
pub type ToolPurposeFlagsEXT = ToolPurposeFlags;
impl ToolPurposeFlags {
    #[doc(alias = "VK_TOOL_PURPOSE_VALIDATION_BIT_EXT")]
    pub const VALIDATION_BIT_EXT: Self = Self::VALIDATION_BIT;
    #[doc(alias = "VK_TOOL_PURPOSE_PROFILING_BIT_EXT")]
    pub const PROFILING_BIT_EXT: Self = Self::PROFILING_BIT;
    #[doc(alias = "VK_TOOL_PURPOSE_TRACING_BIT_EXT")]
    pub const TRACING_BIT_EXT: Self = Self::TRACING_BIT;
    #[doc(alias = "VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT_EXT")]
    pub const ADDITIONAL_FEATURES_BIT_EXT: Self = Self::ADDITIONAL_FEATURES_BIT;
    #[doc(alias = "VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT_EXT")]
    pub const MODIFYING_FEATURES_BIT_EXT: Self = Self::MODIFYING_FEATURES_BIT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlotCreateFlagBits.html>
    #[doc(alias = "VkPrivateDataSlotCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PrivateDataSlotCreateFlags: u32 {
        #[doc(alias = "VK_PRIVATE_DATA_SLOT_CREATE_BASE_OBJECT_HANDLE_BIT_NV")]
        const BASE_OBJECT_HANDLE_BIT_NV = 1;
    }
}
pub type PrivateDataSlotCreateFlagsEXT = PrivateDataSlotCreateFlags;

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineStageFlagBits2.html>
    #[doc(alias = "VkPipelineStageFlagBits2")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PipelineStageFlags2: u64 {
        #[doc(alias = "VK_PIPELINE_STAGE_2_NONE")]
        const NONE = 0;
        #[doc(alias = "VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT")]
        const TOP_OF_PIPE_BIT = 1;
        #[doc(alias = "VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT")]
        const DRAW_INDIRECT_BIT = 2;
        #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT")]
        const VERTEX_INPUT_BIT = 4;
        #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT")]
        const VERTEX_SHADER_BIT = 8;
        #[doc(alias = "VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT")]
        const TESSELLATION_CONTROL_SHADER_BIT = 16;
        #[doc(alias = "VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT")]
        const TESSELLATION_EVALUATION_SHADER_BIT = 32;
        #[doc(alias = "VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT")]
        const GEOMETRY_SHADER_BIT = 64;
        #[doc(alias = "VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT")]
        const FRAGMENT_SHADER_BIT = 128;
        #[doc(alias = "VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT")]
        const EARLY_FRAGMENT_TESTS_BIT = 256;
        #[doc(alias = "VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT")]
        const LATE_FRAGMENT_TESTS_BIT = 512;
        #[doc(alias = "VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT")]
        const COLOR_ATTACHMENT_OUTPUT_BIT = 1024;
        #[doc(alias = "VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT")]
        const COMPUTE_SHADER_BIT = 2048;
        #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT")]
        const ALL_TRANSFER_BIT = 4096;
        #[doc(alias = "VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT")]
        const BOTTOM_OF_PIPE_BIT = 8192;
        #[doc(alias = "VK_PIPELINE_STAGE_2_HOST_BIT")]
        const HOST_BIT = 16384;
        #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT")]
        const ALL_GRAPHICS_BIT = 32768;
        #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT")]
        const ALL_COMMANDS_BIT = 65536;
        #[doc(alias = "VK_PIPELINE_STAGE_2_COPY_BIT")]
        const COPY_BIT = 4294967296;
        #[doc(alias = "VK_PIPELINE_STAGE_2_RESOLVE_BIT")]
        const RESOLVE_BIT = 8589934592;
        #[doc(alias = "VK_PIPELINE_STAGE_2_BLIT_BIT")]
        const BLIT_BIT = 17179869184;
        #[doc(alias = "VK_PIPELINE_STAGE_2_CLEAR_BIT")]
        const CLEAR_BIT = 34359738368;
        #[doc(alias = "VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT")]
        const INDEX_INPUT_BIT = 68719476736;
        #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT")]
        const VERTEX_ATTRIBUTE_INPUT_BIT = 137438953472;
        #[doc(alias = "VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT")]
        const PRE_RASTERIZATION_SHADERS_BIT = 274877906944;
        #[doc(alias = "VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR")]
        const VIDEO_DECODE_BIT_KHR = 67108864;
        #[doc(alias = "VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR")]
        const VIDEO_ENCODE_BIT_KHR = 134217728;
        #[doc(alias = "VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT")]
        const TRANSFORM_FEEDBACK_BIT_EXT = 16777216;
        #[doc(alias = "VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT")]
        const CONDITIONAL_RENDERING_BIT_EXT = 262144;
        #[doc(alias = "VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_EXT")]
        const COMMAND_PREPROCESS_BIT_EXT = 131072;
        #[doc(alias = "VK_PIPELINE_STAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
        const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 4194304;
        #[doc(alias = "VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR")]
        const ACCELERATION_STRUCTURE_BUILD_BIT_KHR = 33554432;
        #[doc(alias = "VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR")]
        const RAY_TRACING_SHADER_BIT_KHR = 2097152;
        #[doc(alias = "VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT")]
        const FRAGMENT_DENSITY_PROCESS_BIT_EXT = 8388608;
        #[doc(alias = "VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_EXT")]
        const TASK_SHADER_BIT_EXT = 524288;
        #[doc(alias = "VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_EXT")]
        const MESH_SHADER_BIT_EXT = 1048576;
        #[doc(alias = "VK_PIPELINE_STAGE_2_SUBPASS_SHADER_BIT_HUAWEI")]
        const SUBPASS_SHADER_BIT_HUAWEI = 549755813888;
        #[doc(alias = "VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI")]
        const INVOCATION_MASK_BIT_HUAWEI = 1099511627776;
        #[doc(alias = "VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_COPY_BIT_KHR")]
        const ACCELERATION_STRUCTURE_COPY_BIT_KHR = 268435456;
        #[doc(alias = "VK_PIPELINE_STAGE_2_MICROMAP_BUILD_BIT_EXT")]
        const MICROMAP_BUILD_BIT_EXT = 1073741824;
        #[doc(alias = "VK_PIPELINE_STAGE_2_CLUSTER_CULLING_SHADER_BIT_HUAWEI")]
        const CLUSTER_CULLING_SHADER_BIT_HUAWEI = 2199023255552;
        #[doc(alias = "VK_PIPELINE_STAGE_2_OPTICAL_FLOW_BIT_NV")]
        const OPTICAL_FLOW_BIT_NV = 536870912;
        #[doc(alias = "VK_PIPELINE_STAGE_2_CONVERT_COOPERATIVE_VECTOR_MATRIX_BIT_NV")]
        const CONVERT_COOPERATIVE_VECTOR_MATRIX_BIT_NV = 17592186044416;
        #[doc(alias = "VK_PIPELINE_STAGE_2_DATA_GRAPH_BIT_ARM")]
        const DATA_GRAPH_BIT_ARM = 4398046511104;
        #[doc(alias = "VK_PIPELINE_STAGE_2_COPY_INDIRECT_BIT_KHR")]
        const COPY_INDIRECT_BIT_KHR = 70368744177664;
        #[doc(alias = "VK_PIPELINE_STAGE_2_MEMORY_DECOMPRESSION_BIT_EXT")]
        const MEMORY_DECOMPRESSION_BIT_EXT = 35184372088832;
    }
}
pub type PipelineStageFlags2KHR = PipelineStageFlags2;
impl PipelineStageFlags2 {
    #[doc(alias = "VK_PIPELINE_STAGE_2_NONE_KHR")]
    pub const NONE_KHR: Self = Self::NONE;
    #[doc(alias = "VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT_KHR")]
    pub const TOP_OF_PIPE_BIT_KHR: Self = Self::TOP_OF_PIPE_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT_KHR")]
    pub const DRAW_INDIRECT_BIT_KHR: Self = Self::DRAW_INDIRECT_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT_KHR")]
    pub const VERTEX_INPUT_BIT_KHR: Self = Self::VERTEX_INPUT_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT_KHR")]
    pub const VERTEX_SHADER_BIT_KHR: Self = Self::VERTEX_SHADER_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT_KHR")]
    pub const TESSELLATION_CONTROL_SHADER_BIT_KHR: Self = Self::TESSELLATION_CONTROL_SHADER_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT_KHR")]
    pub const TESSELLATION_EVALUATION_SHADER_BIT_KHR: Self =
        Self::TESSELLATION_EVALUATION_SHADER_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT_KHR")]
    pub const GEOMETRY_SHADER_BIT_KHR: Self = Self::GEOMETRY_SHADER_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT_KHR")]
    pub const FRAGMENT_SHADER_BIT_KHR: Self = Self::FRAGMENT_SHADER_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT_KHR")]
    pub const EARLY_FRAGMENT_TESTS_BIT_KHR: Self = Self::EARLY_FRAGMENT_TESTS_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT_KHR")]
    pub const LATE_FRAGMENT_TESTS_BIT_KHR: Self = Self::LATE_FRAGMENT_TESTS_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT_KHR")]
    pub const COLOR_ATTACHMENT_OUTPUT_BIT_KHR: Self = Self::COLOR_ATTACHMENT_OUTPUT_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT_KHR")]
    pub const COMPUTE_SHADER_BIT_KHR: Self = Self::COMPUTE_SHADER_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_TRANSFER_BIT")]
    pub const TRANSFER_BIT: Self = Self::ALL_TRANSFER_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT_KHR")]
    pub const ALL_TRANSFER_BIT_KHR: Self = Self::ALL_TRANSFER_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_TRANSFER_BIT_KHR")]
    pub const TRANSFER_BIT_KHR: Self = Self::ALL_TRANSFER_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT_KHR")]
    pub const BOTTOM_OF_PIPE_BIT_KHR: Self = Self::BOTTOM_OF_PIPE_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_HOST_BIT_KHR")]
    pub const HOST_BIT_KHR: Self = Self::HOST_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT_KHR")]
    pub const ALL_GRAPHICS_BIT_KHR: Self = Self::ALL_GRAPHICS_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT_KHR")]
    pub const ALL_COMMANDS_BIT_KHR: Self = Self::ALL_COMMANDS_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_COPY_BIT_KHR")]
    pub const COPY_BIT_KHR: Self = Self::COPY_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_RESOLVE_BIT_KHR")]
    pub const RESOLVE_BIT_KHR: Self = Self::RESOLVE_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_BLIT_BIT_KHR")]
    pub const BLIT_BIT_KHR: Self = Self::BLIT_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_CLEAR_BIT_KHR")]
    pub const CLEAR_BIT_KHR: Self = Self::CLEAR_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT_KHR")]
    pub const INDEX_INPUT_BIT_KHR: Self = Self::INDEX_INPUT_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT_KHR")]
    pub const VERTEX_ATTRIBUTE_INPUT_BIT_KHR: Self = Self::VERTEX_ATTRIBUTE_INPUT_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT_KHR")]
    pub const PRE_RASTERIZATION_SHADERS_BIT_KHR: Self = Self::PRE_RASTERIZATION_SHADERS_BIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV")]
    pub const COMMAND_PREPROCESS_BIT_NV: Self = Self::COMMAND_PREPROCESS_BIT_EXT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV")]
    pub const SHADING_RATE_IMAGE_BIT_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR;
    #[doc(alias = "VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_NV")]
    pub const ACCELERATION_STRUCTURE_BUILD_BIT_NV: Self =
        Self::ACCELERATION_STRUCTURE_BUILD_BIT_KHR;
    #[doc(alias = "VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_NV")]
    pub const RAY_TRACING_SHADER_BIT_NV: Self = Self::RAY_TRACING_SHADER_BIT_KHR;
    #[doc(alias = "VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV")]
    pub const TASK_SHADER_BIT_NV: Self = Self::TASK_SHADER_BIT_EXT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV")]
    pub const MESH_SHADER_BIT_NV: Self = Self::MESH_SHADER_BIT_EXT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI")]
    pub const SUBPASS_SHADING_BIT_HUAWEI: Self = Self::SUBPASS_SHADER_BIT_HUAWEI;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlagBits2.html>
    #[doc(alias = "VkAccessFlagBits2")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct AccessFlags2: u64 {
        #[doc(alias = "VK_ACCESS_2_NONE")]
        const NONE = 0;
        #[doc(alias = "VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT")]
        const INDIRECT_COMMAND_READ_BIT = 1;
        #[doc(alias = "VK_ACCESS_2_INDEX_READ_BIT")]
        const INDEX_READ_BIT = 2;
        #[doc(alias = "VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT")]
        const VERTEX_ATTRIBUTE_READ_BIT = 4;
        #[doc(alias = "VK_ACCESS_2_UNIFORM_READ_BIT")]
        const UNIFORM_READ_BIT = 8;
        #[doc(alias = "VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT")]
        const INPUT_ATTACHMENT_READ_BIT = 16;
        #[doc(alias = "VK_ACCESS_2_SHADER_READ_BIT")]
        const SHADER_READ_BIT = 32;
        #[doc(alias = "VK_ACCESS_2_SHADER_WRITE_BIT")]
        const SHADER_WRITE_BIT = 64;
        #[doc(alias = "VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT")]
        const COLOR_ATTACHMENT_READ_BIT = 128;
        #[doc(alias = "VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT")]
        const COLOR_ATTACHMENT_WRITE_BIT = 256;
        #[doc(alias = "VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT")]
        const DEPTH_STENCIL_ATTACHMENT_READ_BIT = 512;
        #[doc(alias = "VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT")]
        const DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = 1024;
        #[doc(alias = "VK_ACCESS_2_TRANSFER_READ_BIT")]
        const TRANSFER_READ_BIT = 2048;
        #[doc(alias = "VK_ACCESS_2_TRANSFER_WRITE_BIT")]
        const TRANSFER_WRITE_BIT = 4096;
        #[doc(alias = "VK_ACCESS_2_HOST_READ_BIT")]
        const HOST_READ_BIT = 8192;
        #[doc(alias = "VK_ACCESS_2_HOST_WRITE_BIT")]
        const HOST_WRITE_BIT = 16384;
        #[doc(alias = "VK_ACCESS_2_MEMORY_READ_BIT")]
        const MEMORY_READ_BIT = 32768;
        #[doc(alias = "VK_ACCESS_2_MEMORY_WRITE_BIT")]
        const MEMORY_WRITE_BIT = 65536;
        #[doc(alias = "VK_ACCESS_2_SHADER_SAMPLED_READ_BIT")]
        const SHADER_SAMPLED_READ_BIT = 4294967296;
        #[doc(alias = "VK_ACCESS_2_SHADER_STORAGE_READ_BIT")]
        const SHADER_STORAGE_READ_BIT = 8589934592;
        #[doc(alias = "VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT")]
        const SHADER_STORAGE_WRITE_BIT = 17179869184;
        #[doc(alias = "VK_ACCESS_2_VIDEO_DECODE_READ_BIT_KHR")]
        const VIDEO_DECODE_READ_BIT_KHR = 34359738368;
        #[doc(alias = "VK_ACCESS_2_VIDEO_DECODE_WRITE_BIT_KHR")]
        const VIDEO_DECODE_WRITE_BIT_KHR = 68719476736;
        #[doc(alias = "VK_ACCESS_2_SAMPLER_HEAP_READ_BIT_EXT")]
        const SAMPLER_HEAP_READ_BIT_EXT = 144115188075855872;
        #[doc(alias = "VK_ACCESS_2_RESOURCE_HEAP_READ_BIT_EXT")]
        const RESOURCE_HEAP_READ_BIT_EXT = 288230376151711744;
        #[doc(alias = "VK_ACCESS_2_VIDEO_ENCODE_READ_BIT_KHR")]
        const VIDEO_ENCODE_READ_BIT_KHR = 137438953472;
        #[doc(alias = "VK_ACCESS_2_VIDEO_ENCODE_WRITE_BIT_KHR")]
        const VIDEO_ENCODE_WRITE_BIT_KHR = 274877906944;
        #[doc(alias = "VK_ACCESS_2_SHADER_TILE_ATTACHMENT_READ_BIT_QCOM")]
        const SHADER_TILE_ATTACHMENT_READ_BIT_QCOM = 2251799813685248;
        #[doc(alias = "VK_ACCESS_2_SHADER_TILE_ATTACHMENT_WRITE_BIT_QCOM")]
        const SHADER_TILE_ATTACHMENT_WRITE_BIT_QCOM = 4503599627370496;
        #[doc(alias = "VK_ACCESS_2_TRANSFORM_FEEDBACK_WRITE_BIT_EXT")]
        const TRANSFORM_FEEDBACK_WRITE_BIT_EXT = 33554432;
        #[doc(alias = "VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT")]
        const TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT = 67108864;
        #[doc(alias = "VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT")]
        const TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT = 134217728;
        #[doc(alias = "VK_ACCESS_2_CONDITIONAL_RENDERING_READ_BIT_EXT")]
        const CONDITIONAL_RENDERING_READ_BIT_EXT = 1048576;
        #[doc(alias = "VK_ACCESS_2_COMMAND_PREPROCESS_READ_BIT_EXT")]
        const COMMAND_PREPROCESS_READ_BIT_EXT = 131072;
        #[doc(alias = "VK_ACCESS_2_COMMAND_PREPROCESS_WRITE_BIT_EXT")]
        const COMMAND_PREPROCESS_WRITE_BIT_EXT = 262144;
        #[doc(alias = "VK_ACCESS_2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR")]
        const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR = 8388608;
        #[doc(alias = "VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR")]
        const ACCELERATION_STRUCTURE_READ_BIT_KHR = 2097152;
        #[doc(alias = "VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_KHR")]
        const ACCELERATION_STRUCTURE_WRITE_BIT_KHR = 4194304;
        #[doc(alias = "VK_ACCESS_2_FRAGMENT_DENSITY_MAP_READ_BIT_EXT")]
        const FRAGMENT_DENSITY_MAP_READ_BIT_EXT = 16777216;
        #[doc(alias = "VK_ACCESS_2_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT")]
        const COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT = 524288;
        #[doc(alias = "VK_ACCESS_2_DESCRIPTOR_BUFFER_READ_BIT_EXT")]
        const DESCRIPTOR_BUFFER_READ_BIT_EXT = 2199023255552;
        #[doc(alias = "VK_ACCESS_2_INVOCATION_MASK_READ_BIT_HUAWEI")]
        const INVOCATION_MASK_READ_BIT_HUAWEI = 549755813888;
        #[doc(alias = "VK_ACCESS_2_SHADER_BINDING_TABLE_READ_BIT_KHR")]
        const SHADER_BINDING_TABLE_READ_BIT_KHR = 1099511627776;
        #[doc(alias = "VK_ACCESS_2_MICROMAP_READ_BIT_EXT")]
        const MICROMAP_READ_BIT_EXT = 17592186044416;
        #[doc(alias = "VK_ACCESS_2_MICROMAP_WRITE_BIT_EXT")]
        const MICROMAP_WRITE_BIT_EXT = 35184372088832;
        #[doc(alias = "VK_ACCESS_2_OPTICAL_FLOW_READ_BIT_NV")]
        const OPTICAL_FLOW_READ_BIT_NV = 4398046511104;
        #[doc(alias = "VK_ACCESS_2_OPTICAL_FLOW_WRITE_BIT_NV")]
        const OPTICAL_FLOW_WRITE_BIT_NV = 8796093022208;
        #[doc(alias = "VK_ACCESS_2_DATA_GRAPH_READ_BIT_ARM")]
        const DATA_GRAPH_READ_BIT_ARM = 140737488355328;
        #[doc(alias = "VK_ACCESS_2_DATA_GRAPH_WRITE_BIT_ARM")]
        const DATA_GRAPH_WRITE_BIT_ARM = 281474976710656;
        #[doc(alias = "VK_ACCESS_2_MEMORY_DECOMPRESSION_READ_BIT_EXT")]
        const MEMORY_DECOMPRESSION_READ_BIT_EXT = 36028797018963968;
        #[doc(alias = "VK_ACCESS_2_MEMORY_DECOMPRESSION_WRITE_BIT_EXT")]
        const MEMORY_DECOMPRESSION_WRITE_BIT_EXT = 72057594037927936;
    }
}
pub type AccessFlags2KHR = AccessFlags2;
impl AccessFlags2 {
    #[doc(alias = "VK_ACCESS_2_NONE_KHR")]
    pub const NONE_KHR: Self = Self::NONE;
    #[doc(alias = "VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT_KHR")]
    pub const INDIRECT_COMMAND_READ_BIT_KHR: Self = Self::INDIRECT_COMMAND_READ_BIT;
    #[doc(alias = "VK_ACCESS_2_INDEX_READ_BIT_KHR")]
    pub const INDEX_READ_BIT_KHR: Self = Self::INDEX_READ_BIT;
    #[doc(alias = "VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT_KHR")]
    pub const VERTEX_ATTRIBUTE_READ_BIT_KHR: Self = Self::VERTEX_ATTRIBUTE_READ_BIT;
    #[doc(alias = "VK_ACCESS_2_UNIFORM_READ_BIT_KHR")]
    pub const UNIFORM_READ_BIT_KHR: Self = Self::UNIFORM_READ_BIT;
    #[doc(alias = "VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT_KHR")]
    pub const INPUT_ATTACHMENT_READ_BIT_KHR: Self = Self::INPUT_ATTACHMENT_READ_BIT;
    #[doc(alias = "VK_ACCESS_2_SHADER_READ_BIT_KHR")]
    pub const SHADER_READ_BIT_KHR: Self = Self::SHADER_READ_BIT;
    #[doc(alias = "VK_ACCESS_2_SHADER_WRITE_BIT_KHR")]
    pub const SHADER_WRITE_BIT_KHR: Self = Self::SHADER_WRITE_BIT;
    #[doc(alias = "VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT_KHR")]
    pub const COLOR_ATTACHMENT_READ_BIT_KHR: Self = Self::COLOR_ATTACHMENT_READ_BIT;
    #[doc(alias = "VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT_KHR")]
    pub const COLOR_ATTACHMENT_WRITE_BIT_KHR: Self = Self::COLOR_ATTACHMENT_WRITE_BIT;
    #[doc(alias = "VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT_KHR")]
    pub const DEPTH_STENCIL_ATTACHMENT_READ_BIT_KHR: Self = Self::DEPTH_STENCIL_ATTACHMENT_READ_BIT;
    #[doc(alias = "VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT_KHR")]
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE_BIT_KHR: Self =
        Self::DEPTH_STENCIL_ATTACHMENT_WRITE_BIT;
    #[doc(alias = "VK_ACCESS_2_TRANSFER_READ_BIT_KHR")]
    pub const TRANSFER_READ_BIT_KHR: Self = Self::TRANSFER_READ_BIT;
    #[doc(alias = "VK_ACCESS_2_TRANSFER_WRITE_BIT_KHR")]
    pub const TRANSFER_WRITE_BIT_KHR: Self = Self::TRANSFER_WRITE_BIT;
    #[doc(alias = "VK_ACCESS_2_HOST_READ_BIT_KHR")]
    pub const HOST_READ_BIT_KHR: Self = Self::HOST_READ_BIT;
    #[doc(alias = "VK_ACCESS_2_HOST_WRITE_BIT_KHR")]
    pub const HOST_WRITE_BIT_KHR: Self = Self::HOST_WRITE_BIT;
    #[doc(alias = "VK_ACCESS_2_MEMORY_READ_BIT_KHR")]
    pub const MEMORY_READ_BIT_KHR: Self = Self::MEMORY_READ_BIT;
    #[doc(alias = "VK_ACCESS_2_MEMORY_WRITE_BIT_KHR")]
    pub const MEMORY_WRITE_BIT_KHR: Self = Self::MEMORY_WRITE_BIT;
    #[doc(alias = "VK_ACCESS_2_SHADER_SAMPLED_READ_BIT_KHR")]
    pub const SHADER_SAMPLED_READ_BIT_KHR: Self = Self::SHADER_SAMPLED_READ_BIT;
    #[doc(alias = "VK_ACCESS_2_SHADER_STORAGE_READ_BIT_KHR")]
    pub const SHADER_STORAGE_READ_BIT_KHR: Self = Self::SHADER_STORAGE_READ_BIT;
    #[doc(alias = "VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT_KHR")]
    pub const SHADER_STORAGE_WRITE_BIT_KHR: Self = Self::SHADER_STORAGE_WRITE_BIT;
    #[doc(alias = "VK_ACCESS_2_COMMAND_PREPROCESS_READ_BIT_NV")]
    pub const COMMAND_PREPROCESS_READ_BIT_NV: Self = Self::COMMAND_PREPROCESS_READ_BIT_EXT;
    #[doc(alias = "VK_ACCESS_2_COMMAND_PREPROCESS_WRITE_BIT_NV")]
    pub const COMMAND_PREPROCESS_WRITE_BIT_NV: Self = Self::COMMAND_PREPROCESS_WRITE_BIT_EXT;
    #[doc(alias = "VK_ACCESS_2_SHADING_RATE_IMAGE_READ_BIT_NV")]
    pub const SHADING_RATE_IMAGE_READ_BIT_NV: Self =
        Self::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR;
    #[doc(alias = "VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_NV")]
    pub const ACCELERATION_STRUCTURE_READ_BIT_NV: Self = Self::ACCELERATION_STRUCTURE_READ_BIT_KHR;
    #[doc(alias = "VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_NV")]
    pub const ACCELERATION_STRUCTURE_WRITE_BIT_NV: Self =
        Self::ACCELERATION_STRUCTURE_WRITE_BIT_KHR;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubmitFlagBits.html>
    #[doc(alias = "VkSubmitFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SubmitFlags: u32 {
        #[doc(alias = "VK_SUBMIT_PROTECTED_BIT")]
        const PROTECTED_BIT = 1;
    }
}
pub type SubmitFlagsKHR = SubmitFlags;
impl SubmitFlags {
    #[doc(alias = "VK_SUBMIT_PROTECTED_BIT_KHR")]
    pub const PROTECTED_BIT_KHR: Self = Self::PROTECTED_BIT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlagBits2.html>
    #[doc(alias = "VkFormatFeatureFlagBits2")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct FormatFeatureFlags2: u64 {
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT")]
        const SAMPLED_IMAGE_BIT = 1;
        #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT")]
        const STORAGE_IMAGE_BIT = 2;
        #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT")]
        const STORAGE_IMAGE_ATOMIC_BIT = 4;
        #[doc(alias = "VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT")]
        const UNIFORM_TEXEL_BUFFER_BIT = 8;
        #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT")]
        const STORAGE_TEXEL_BUFFER_BIT = 16;
        #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT")]
        const STORAGE_TEXEL_BUFFER_ATOMIC_BIT = 32;
        #[doc(alias = "VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT")]
        const VERTEX_BUFFER_BIT = 64;
        #[doc(alias = "VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT")]
        const COLOR_ATTACHMENT_BIT = 128;
        #[doc(alias = "VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT")]
        const COLOR_ATTACHMENT_BLEND_BIT = 256;
        #[doc(alias = "VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT")]
        const DEPTH_STENCIL_ATTACHMENT_BIT = 512;
        #[doc(alias = "VK_FORMAT_FEATURE_2_BLIT_SRC_BIT")]
        const BLIT_SRC_BIT = 1024;
        #[doc(alias = "VK_FORMAT_FEATURE_2_BLIT_DST_BIT")]
        const BLIT_DST_BIT = 2048;
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT")]
        const SAMPLED_IMAGE_FILTER_LINEAR_BIT = 4096;
        #[doc(alias = "VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT")]
        const TRANSFER_SRC_BIT = 16384;
        #[doc(alias = "VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT")]
        const TRANSFER_DST_BIT = 32768;
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT")]
        const SAMPLED_IMAGE_FILTER_MINMAX_BIT = 65536;
        #[doc(alias = "VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT")]
        const MIDPOINT_CHROMA_SAMPLES_BIT = 131072;
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT")]
        const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT = 262144;
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT")]
        const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT = 524288;
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT")]
        const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT = 1048576;
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT")]
        const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT = 2097152;
        #[doc(alias = "VK_FORMAT_FEATURE_2_DISJOINT_BIT")]
        const DISJOINT_BIT = 4194304;
        #[doc(alias = "VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT")]
        const COSITED_CHROMA_SAMPLES_BIT = 8388608;
        #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT")]
        const STORAGE_READ_WITHOUT_FORMAT_BIT = 2147483648;
        #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT")]
        const STORAGE_WRITE_WITHOUT_FORMAT_BIT = 4294967296;
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT")]
        const SAMPLED_IMAGE_DEPTH_COMPARISON_BIT = 8589934592;
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT")]
        const SAMPLED_IMAGE_FILTER_CUBIC_BIT = 8192;
        #[doc(alias = "VK_FORMAT_FEATURE_2_HOST_IMAGE_TRANSFER_BIT")]
        const HOST_IMAGE_TRANSFER_BIT = 70368744177664;
        #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_DECODE_OUTPUT_BIT_KHR")]
        const VIDEO_DECODE_OUTPUT_BIT_KHR = 33554432;
        #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_DECODE_DPB_BIT_KHR")]
        const VIDEO_DECODE_DPB_BIT_KHR = 67108864;
        #[doc(alias = "VK_FORMAT_FEATURE_2_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR")]
        const ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR = 536870912;
        #[doc(alias = "VK_FORMAT_FEATURE_2_FRAGMENT_DENSITY_MAP_BIT_EXT")]
        const FRAGMENT_DENSITY_MAP_BIT_EXT = 16777216;
        #[doc(alias = "VK_FORMAT_FEATURE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
        const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 1073741824;
        #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_ENCODE_INPUT_BIT_KHR")]
        const VIDEO_ENCODE_INPUT_BIT_KHR = 134217728;
        #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_ENCODE_DPB_BIT_KHR")]
        const VIDEO_ENCODE_DPB_BIT_KHR = 268435456;
        #[doc(alias = "VK_FORMAT_FEATURE_2_BLOCK_MATCHING_SXD_BIT_QCOM")]
        const BLOCK_MATCHING_SXD_BIT_QCOM = 17592186044416;
        #[doc(alias = "VK_FORMAT_FEATURE_2_ACCELERATION_STRUCTURE_RADIUS_BUFFER_BIT_NV")]
        const ACCELERATION_STRUCTURE_RADIUS_BUFFER_BIT_NV = 2251799813685248;
        #[doc(alias = "VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV")]
        const LINEAR_COLOR_ATTACHMENT_BIT_NV = 274877906944;
        #[doc(alias = "VK_FORMAT_FEATURE_2_WEIGHT_IMAGE_BIT_QCOM")]
        const WEIGHT_IMAGE_BIT_QCOM = 17179869184;
        #[doc(alias = "VK_FORMAT_FEATURE_2_WEIGHT_SAMPLED_IMAGE_BIT_QCOM")]
        const WEIGHT_SAMPLED_IMAGE_BIT_QCOM = 34359738368;
        #[doc(alias = "VK_FORMAT_FEATURE_2_BLOCK_MATCHING_BIT_QCOM")]
        const BLOCK_MATCHING_BIT_QCOM = 68719476736;
        #[doc(alias = "VK_FORMAT_FEATURE_2_BOX_FILTER_SAMPLED_BIT_QCOM")]
        const BOX_FILTER_SAMPLED_BIT_QCOM = 137438953472;
        #[doc(alias = "VK_FORMAT_FEATURE_2_TENSOR_SHADER_BIT_ARM")]
        const TENSOR_SHADER_BIT_ARM = 549755813888;
        #[doc(alias = "VK_FORMAT_FEATURE_2_TENSOR_IMAGE_ALIASING_BIT_ARM")]
        const TENSOR_IMAGE_ALIASING_BIT_ARM = 8796093022208;
        #[doc(alias = "VK_FORMAT_FEATURE_2_OPTICAL_FLOW_IMAGE_BIT_NV")]
        const OPTICAL_FLOW_IMAGE_BIT_NV = 1099511627776;
        #[doc(alias = "VK_FORMAT_FEATURE_2_OPTICAL_FLOW_VECTOR_BIT_NV")]
        const OPTICAL_FLOW_VECTOR_BIT_NV = 2199023255552;
        #[doc(alias = "VK_FORMAT_FEATURE_2_OPTICAL_FLOW_COST_BIT_NV")]
        const OPTICAL_FLOW_COST_BIT_NV = 4398046511104;
        #[doc(alias = "VK_FORMAT_FEATURE_2_TENSOR_DATA_GRAPH_BIT_ARM")]
        const TENSOR_DATA_GRAPH_BIT_ARM = 281474976710656;
        #[doc(alias = "VK_FORMAT_FEATURE_2_COPY_IMAGE_INDIRECT_DST_BIT_KHR")]
        const COPY_IMAGE_INDIRECT_DST_BIT_KHR = 576460752303423488;
        #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR")]
        const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR = 562949953421312;
        #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR")]
        const VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR = 1125899906842624;
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_2D_BIT_IMG")]
        const SAMPLED_IMAGE_FILTER_LINEAR_2D_BIT_IMG = 35184372088832;
        #[doc(alias = "VK_FORMAT_FEATURE_2_DEPTH_COPY_ON_COMPUTE_QUEUE_BIT_KHR")]
        const DEPTH_COPY_ON_COMPUTE_QUEUE_BIT_KHR = 4503599627370496;
        #[doc(alias = "VK_FORMAT_FEATURE_2_DEPTH_COPY_ON_TRANSFER_QUEUE_BIT_KHR")]
        const DEPTH_COPY_ON_TRANSFER_QUEUE_BIT_KHR = 9007199254740992;
        #[doc(alias = "VK_FORMAT_FEATURE_2_STENCIL_COPY_ON_COMPUTE_QUEUE_BIT_KHR")]
        const STENCIL_COPY_ON_COMPUTE_QUEUE_BIT_KHR = 18014398509481984;
        #[doc(alias = "VK_FORMAT_FEATURE_2_STENCIL_COPY_ON_TRANSFER_QUEUE_BIT_KHR")]
        const STENCIL_COPY_ON_TRANSFER_QUEUE_BIT_KHR = 36028797018963968;
        #[doc(alias = "VK_FORMAT_FEATURE_2_DATA_GRAPH_OPTICAL_FLOW_IMAGE_BIT_ARM")]
        const DATA_GRAPH_OPTICAL_FLOW_IMAGE_BIT_ARM = 72057594037927936;
        #[doc(alias = "VK_FORMAT_FEATURE_2_DATA_GRAPH_OPTICAL_FLOW_VECTOR_BIT_ARM")]
        const DATA_GRAPH_OPTICAL_FLOW_VECTOR_BIT_ARM = 144115188075855872;
        #[doc(alias = "VK_FORMAT_FEATURE_2_DATA_GRAPH_OPTICAL_FLOW_COST_BIT_ARM")]
        const DATA_GRAPH_OPTICAL_FLOW_COST_BIT_ARM = 288230376151711744;
    }
}
pub type FormatFeatureFlags2KHR = FormatFeatureFlags2;
impl FormatFeatureFlags2 {
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT_KHR")]
    pub const SAMPLED_IMAGE_BIT_KHR: Self = Self::SAMPLED_IMAGE_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT_KHR")]
    pub const STORAGE_IMAGE_BIT_KHR: Self = Self::STORAGE_IMAGE_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT_KHR")]
    pub const STORAGE_IMAGE_ATOMIC_BIT_KHR: Self = Self::STORAGE_IMAGE_ATOMIC_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT_KHR")]
    pub const UNIFORM_TEXEL_BUFFER_BIT_KHR: Self = Self::UNIFORM_TEXEL_BUFFER_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT_KHR")]
    pub const STORAGE_TEXEL_BUFFER_BIT_KHR: Self = Self::STORAGE_TEXEL_BUFFER_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT_KHR")]
    pub const STORAGE_TEXEL_BUFFER_ATOMIC_BIT_KHR: Self = Self::STORAGE_TEXEL_BUFFER_ATOMIC_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT_KHR")]
    pub const VERTEX_BUFFER_BIT_KHR: Self = Self::VERTEX_BUFFER_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT_KHR")]
    pub const COLOR_ATTACHMENT_BIT_KHR: Self = Self::COLOR_ATTACHMENT_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT_KHR")]
    pub const COLOR_ATTACHMENT_BLEND_BIT_KHR: Self = Self::COLOR_ATTACHMENT_BLEND_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT_KHR")]
    pub const DEPTH_STENCIL_ATTACHMENT_BIT_KHR: Self = Self::DEPTH_STENCIL_ATTACHMENT_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_BLIT_SRC_BIT_KHR")]
    pub const BLIT_SRC_BIT_KHR: Self = Self::BLIT_SRC_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_BLIT_DST_BIT_KHR")]
    pub const BLIT_DST_BIT_KHR: Self = Self::BLIT_DST_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT_KHR")]
    pub const SAMPLED_IMAGE_FILTER_LINEAR_BIT_KHR: Self = Self::SAMPLED_IMAGE_FILTER_LINEAR_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT_KHR")]
    pub const TRANSFER_SRC_BIT_KHR: Self = Self::TRANSFER_SRC_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT_KHR")]
    pub const TRANSFER_DST_BIT_KHR: Self = Self::TRANSFER_DST_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT_KHR")]
    pub const SAMPLED_IMAGE_FILTER_MINMAX_BIT_KHR: Self = Self::SAMPLED_IMAGE_FILTER_MINMAX_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT_KHR")]
    pub const MIDPOINT_CHROMA_SAMPLES_BIT_KHR: Self = Self::MIDPOINT_CHROMA_SAMPLES_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR")]
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT;
    #[doc(
        alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR"
    )]
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT;
    #[doc(
        alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR"
    )]
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT;
    #[doc(
        alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR"
    )]
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR:
        Self = Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_DISJOINT_BIT_KHR")]
    pub const DISJOINT_BIT_KHR: Self = Self::DISJOINT_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT_KHR")]
    pub const COSITED_CHROMA_SAMPLES_BIT_KHR: Self = Self::COSITED_CHROMA_SAMPLES_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT_KHR")]
    pub const STORAGE_READ_WITHOUT_FORMAT_BIT_KHR: Self = Self::STORAGE_READ_WITHOUT_FORMAT_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT_KHR")]
    pub const STORAGE_WRITE_WITHOUT_FORMAT_BIT_KHR: Self = Self::STORAGE_WRITE_WITHOUT_FORMAT_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT_KHR")]
    pub const SAMPLED_IMAGE_DEPTH_COMPARISON_BIT_KHR: Self =
        Self::SAMPLED_IMAGE_DEPTH_COMPARISON_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT")]
    pub const SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT: Self = Self::SAMPLED_IMAGE_FILTER_CUBIC_BIT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_HOST_IMAGE_TRANSFER_BIT_EXT")]
    pub const HOST_IMAGE_TRANSFER_BIT_EXT: Self = Self::HOST_IMAGE_TRANSFER_BIT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreationFeedbackFlagBits.html>
    #[doc(alias = "VkPipelineCreationFeedbackFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PipelineCreationFeedbackFlags: u32 {
        #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT")]
        const VALID_BIT = 1;
        #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT")]
        const APPLICATION_PIPELINE_CACHE_HIT_BIT = 2;
        #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT")]
        const BASE_PIPELINE_ACCELERATION_BIT = 4;
    }
}
pub type PipelineCreationFeedbackFlagsEXT = PipelineCreationFeedbackFlags;
impl PipelineCreationFeedbackFlags {
    #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT_EXT")]
    pub const VALID_BIT_EXT: Self = Self::VALID_BIT;
    #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT_EXT")]
    pub const APPLICATION_PIPELINE_CACHE_HIT_BIT_EXT: Self =
        Self::APPLICATION_PIPELINE_CACHE_HIT_BIT;
    #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT_EXT")]
    pub const BASE_PIPELINE_ACCELERATION_BIT_EXT: Self = Self::BASE_PIPELINE_ACCELERATION_BIT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingFlagBits.html>
    #[doc(alias = "VkRenderingFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct RenderingFlags: u32 {
        #[doc(alias = "VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT")]
        const CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT = 1;
        #[doc(alias = "VK_RENDERING_SUSPENDING_BIT")]
        const SUSPENDING_BIT = 2;
        #[doc(alias = "VK_RENDERING_RESUMING_BIT")]
        const RESUMING_BIT = 4;
        #[doc(alias = "VK_RENDERING_ENABLE_LEGACY_DITHERING_BIT_EXT")]
        const ENABLE_LEGACY_DITHERING_BIT_EXT = 8;
        #[doc(alias = "VK_RENDERING_CONTENTS_INLINE_BIT_KHR")]
        const CONTENTS_INLINE_BIT_KHR = 16;
        #[doc(alias = "VK_RENDERING_PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE")]
        const PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE = 32;
        #[doc(alias = "VK_RENDERING_FRAGMENT_REGION_BIT_EXT")]
        const FRAGMENT_REGION_BIT_EXT = 64;
        #[doc(alias = "VK_RENDERING_CUSTOM_RESOLVE_BIT_EXT")]
        const CUSTOM_RESOLVE_BIT_EXT = 128;
        #[doc(alias = "VK_RENDERING_LOCAL_READ_CONCURRENT_ACCESS_CONTROL_BIT_KHR")]
        const LOCAL_READ_CONCURRENT_ACCESS_CONTROL_BIT_KHR = 256;
    }
}
pub type RenderingFlagsKHR = RenderingFlags;
impl RenderingFlags {
    #[doc(alias = "VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT_KHR")]
    pub const CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT_KHR: Self =
        Self::CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT;
    #[doc(alias = "VK_RENDERING_SUSPENDING_BIT_KHR")]
    pub const SUSPENDING_BIT_KHR: Self = Self::SUSPENDING_BIT;
    #[doc(alias = "VK_RENDERING_RESUMING_BIT_KHR")]
    pub const RESUMING_BIT_KHR: Self = Self::RESUMING_BIT;
    #[doc(alias = "VK_RENDERING_CONTENTS_INLINE_BIT_EXT")]
    pub const CONTENTS_INLINE_BIT_EXT: Self = Self::CONTENTS_INLINE_BIT_KHR;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryUnmapFlagBits.html>
    #[doc(alias = "VkMemoryUnmapFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct MemoryUnmapFlags: u32 {
        #[doc(alias = "VK_MEMORY_UNMAP_RESERVE_BIT_EXT")]
        const RESERVE_BIT_EXT = 1;
    }
}
pub type MemoryUnmapFlagsKHR = MemoryUnmapFlags;

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlagBits2.html>
    #[doc(alias = "VkBufferUsageFlagBits2")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct BufferUsageFlags2: u64 {
        #[doc(alias = "VK_BUFFER_USAGE_2_TRANSFER_SRC_BIT")]
        const TRANSFER_SRC_BIT = 1;
        #[doc(alias = "VK_BUFFER_USAGE_2_TRANSFER_DST_BIT")]
        const TRANSFER_DST_BIT = 2;
        #[doc(alias = "VK_BUFFER_USAGE_2_UNIFORM_TEXEL_BUFFER_BIT")]
        const UNIFORM_TEXEL_BUFFER_BIT = 4;
        #[doc(alias = "VK_BUFFER_USAGE_2_STORAGE_TEXEL_BUFFER_BIT")]
        const STORAGE_TEXEL_BUFFER_BIT = 8;
        #[doc(alias = "VK_BUFFER_USAGE_2_UNIFORM_BUFFER_BIT")]
        const UNIFORM_BUFFER_BIT = 16;
        #[doc(alias = "VK_BUFFER_USAGE_2_STORAGE_BUFFER_BIT")]
        const STORAGE_BUFFER_BIT = 32;
        #[doc(alias = "VK_BUFFER_USAGE_2_INDEX_BUFFER_BIT")]
        const INDEX_BUFFER_BIT = 64;
        #[doc(alias = "VK_BUFFER_USAGE_2_VERTEX_BUFFER_BIT")]
        const VERTEX_BUFFER_BIT = 128;
        #[doc(alias = "VK_BUFFER_USAGE_2_INDIRECT_BUFFER_BIT")]
        const INDIRECT_BUFFER_BIT = 256;
        #[doc(alias = "VK_BUFFER_USAGE_2_SHADER_DEVICE_ADDRESS_BIT")]
        const SHADER_DEVICE_ADDRESS_BIT = 131072;
        #[doc(alias = "VK_BUFFER_USAGE_2_EXECUTION_GRAPH_SCRATCH_BIT_AMDX")]
        const EXECUTION_GRAPH_SCRATCH_BIT_AMDX = 33554432;
        #[doc(alias = "VK_BUFFER_USAGE_2_DESCRIPTOR_HEAP_BIT_EXT")]
        const DESCRIPTOR_HEAP_BIT_EXT = 268435456;
        #[doc(alias = "VK_BUFFER_USAGE_2_MICROMAP_BUILD_INPUT_READ_ONLY_BIT_EXT")]
        const MICROMAP_BUILD_INPUT_READ_ONLY_BIT_EXT = 8388608;
        #[doc(alias = "VK_BUFFER_USAGE_2_MICROMAP_STORAGE_BIT_EXT")]
        const MICROMAP_STORAGE_BIT_EXT = 16777216;
        #[doc(alias = "VK_BUFFER_USAGE_2_CONDITIONAL_RENDERING_BIT_EXT")]
        const CONDITIONAL_RENDERING_BIT_EXT = 512;
        #[doc(alias = "VK_BUFFER_USAGE_2_SHADER_BINDING_TABLE_BIT_KHR")]
        const SHADER_BINDING_TABLE_BIT_KHR = 1024;
        #[doc(alias = "VK_BUFFER_USAGE_2_TRANSFORM_FEEDBACK_BUFFER_BIT_EXT")]
        const TRANSFORM_FEEDBACK_BUFFER_BIT_EXT = 2048;
        #[doc(alias = "VK_BUFFER_USAGE_2_TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT")]
        const TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT = 4096;
        #[doc(alias = "VK_BUFFER_USAGE_2_VIDEO_DECODE_SRC_BIT_KHR")]
        const VIDEO_DECODE_SRC_BIT_KHR = 8192;
        #[doc(alias = "VK_BUFFER_USAGE_2_VIDEO_DECODE_DST_BIT_KHR")]
        const VIDEO_DECODE_DST_BIT_KHR = 16384;
        #[doc(alias = "VK_BUFFER_USAGE_2_VIDEO_ENCODE_DST_BIT_KHR")]
        const VIDEO_ENCODE_DST_BIT_KHR = 32768;
        #[doc(alias = "VK_BUFFER_USAGE_2_VIDEO_ENCODE_SRC_BIT_KHR")]
        const VIDEO_ENCODE_SRC_BIT_KHR = 65536;
        #[doc(alias = "VK_BUFFER_USAGE_2_ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR")]
        const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR = 524288;
        #[doc(alias = "VK_BUFFER_USAGE_2_ACCELERATION_STRUCTURE_STORAGE_BIT_KHR")]
        const ACCELERATION_STRUCTURE_STORAGE_BIT_KHR = 1048576;
        #[doc(alias = "VK_BUFFER_USAGE_2_SAMPLER_DESCRIPTOR_BUFFER_BIT_EXT")]
        const SAMPLER_DESCRIPTOR_BUFFER_BIT_EXT = 2097152;
        #[doc(alias = "VK_BUFFER_USAGE_2_RESOURCE_DESCRIPTOR_BUFFER_BIT_EXT")]
        const RESOURCE_DESCRIPTOR_BUFFER_BIT_EXT = 4194304;
        #[doc(alias = "VK_BUFFER_USAGE_2_PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_BIT_EXT")]
        const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_BIT_EXT = 67108864;
        #[doc(alias = "VK_BUFFER_USAGE_2_COMPRESSED_DATA_DGF1_BIT_AMDX")]
        const COMPRESSED_DATA_DGF1_BIT_AMDX = 8589934592;
        #[doc(alias = "VK_BUFFER_USAGE_2_DATA_GRAPH_FOREIGN_DESCRIPTOR_BIT_ARM")]
        const DATA_GRAPH_FOREIGN_DESCRIPTOR_BIT_ARM = 536870912;
        #[doc(alias = "VK_BUFFER_USAGE_2_TILE_MEMORY_BIT_QCOM")]
        const TILE_MEMORY_BIT_QCOM = 134217728;
        #[doc(alias = "VK_BUFFER_USAGE_2_MEMORY_DECOMPRESSION_BIT_EXT")]
        const MEMORY_DECOMPRESSION_BIT_EXT = 4294967296;
        #[doc(alias = "VK_BUFFER_USAGE_2_PREPROCESS_BUFFER_BIT_EXT")]
        const PREPROCESS_BUFFER_BIT_EXT = 2147483648;
    }
}
pub type BufferUsageFlags2KHR = BufferUsageFlags2;
impl BufferUsageFlags2 {
    #[doc(alias = "VK_BUFFER_USAGE_2_TRANSFER_SRC_BIT_KHR")]
    pub const TRANSFER_SRC_BIT_KHR: Self = Self::TRANSFER_SRC_BIT;
    #[doc(alias = "VK_BUFFER_USAGE_2_TRANSFER_DST_BIT_KHR")]
    pub const TRANSFER_DST_BIT_KHR: Self = Self::TRANSFER_DST_BIT;
    #[doc(alias = "VK_BUFFER_USAGE_2_UNIFORM_TEXEL_BUFFER_BIT_KHR")]
    pub const UNIFORM_TEXEL_BUFFER_BIT_KHR: Self = Self::UNIFORM_TEXEL_BUFFER_BIT;
    #[doc(alias = "VK_BUFFER_USAGE_2_STORAGE_TEXEL_BUFFER_BIT_KHR")]
    pub const STORAGE_TEXEL_BUFFER_BIT_KHR: Self = Self::STORAGE_TEXEL_BUFFER_BIT;
    #[doc(alias = "VK_BUFFER_USAGE_2_UNIFORM_BUFFER_BIT_KHR")]
    pub const UNIFORM_BUFFER_BIT_KHR: Self = Self::UNIFORM_BUFFER_BIT;
    #[doc(alias = "VK_BUFFER_USAGE_2_STORAGE_BUFFER_BIT_KHR")]
    pub const STORAGE_BUFFER_BIT_KHR: Self = Self::STORAGE_BUFFER_BIT;
    #[doc(alias = "VK_BUFFER_USAGE_2_INDEX_BUFFER_BIT_KHR")]
    pub const INDEX_BUFFER_BIT_KHR: Self = Self::INDEX_BUFFER_BIT;
    #[doc(alias = "VK_BUFFER_USAGE_2_VERTEX_BUFFER_BIT_KHR")]
    pub const VERTEX_BUFFER_BIT_KHR: Self = Self::VERTEX_BUFFER_BIT;
    #[doc(alias = "VK_BUFFER_USAGE_2_INDIRECT_BUFFER_BIT_KHR")]
    pub const INDIRECT_BUFFER_BIT_KHR: Self = Self::INDIRECT_BUFFER_BIT;
    #[doc(alias = "VK_BUFFER_USAGE_2_SHADER_DEVICE_ADDRESS_BIT_KHR")]
    pub const SHADER_DEVICE_ADDRESS_BIT_KHR: Self = Self::SHADER_DEVICE_ADDRESS_BIT;
    #[doc(alias = "VK_BUFFER_USAGE_2_RAY_TRACING_BIT_NV")]
    pub const RAY_TRACING_BIT_NV: Self = Self::SHADER_BINDING_TABLE_BIT_KHR;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkHostImageCopyFlagBits.html>
    #[doc(alias = "VkHostImageCopyFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct HostImageCopyFlags: u32 {
        #[doc(alias = "VK_HOST_IMAGE_COPY_MEMCPY_BIT")]
        const MEMCPY_BIT = 1;
    }
}
pub type HostImageCopyFlagsEXT = HostImageCopyFlags;
impl HostImageCopyFlags {
    #[doc(alias = "VK_HOST_IMAGE_COPY_MEMCPY")]
    pub const MEMCPY: Self = Self::MEMCPY_BIT;
    #[doc(alias = "VK_HOST_IMAGE_COPY_MEMCPY_BIT_EXT")]
    pub const MEMCPY_BIT_EXT: Self = Self::MEMCPY_BIT;
    #[doc(alias = "VK_HOST_IMAGE_COPY_MEMCPY_EXT")]
    pub const MEMCPY_EXT: Self = Self::MEMCPY_BIT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlagBits2.html>
    #[doc(alias = "VkPipelineCreateFlagBits2")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PipelineCreateFlags2: u64 {
        #[doc(alias = "VK_PIPELINE_CREATE_2_DISABLE_OPTIMIZATION_BIT")]
        const DISABLE_OPTIMIZATION_BIT = 1;
        #[doc(alias = "VK_PIPELINE_CREATE_2_ALLOW_DERIVATIVES_BIT")]
        const ALLOW_DERIVATIVES_BIT = 2;
        #[doc(alias = "VK_PIPELINE_CREATE_2_DERIVATIVE_BIT")]
        const DERIVATIVE_BIT = 4;
        #[doc(alias = "VK_PIPELINE_CREATE_2_VIEW_INDEX_FROM_DEVICE_INDEX_BIT")]
        const VIEW_INDEX_FROM_DEVICE_INDEX_BIT = 8;
        #[doc(alias = "VK_PIPELINE_CREATE_2_DISPATCH_BASE_BIT")]
        const DISPATCH_BASE_BIT = 16;
        #[doc(alias = "VK_PIPELINE_CREATE_2_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT")]
        const FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT = 256;
        #[doc(alias = "VK_PIPELINE_CREATE_2_EARLY_RETURN_ON_FAILURE_BIT")]
        const EARLY_RETURN_ON_FAILURE_BIT = 512;
        #[doc(alias = "VK_PIPELINE_CREATE_2_NO_PROTECTED_ACCESS_BIT")]
        const NO_PROTECTED_ACCESS_BIT = 134217728;
        #[doc(alias = "VK_PIPELINE_CREATE_2_PROTECTED_ACCESS_ONLY_BIT")]
        const PROTECTED_ACCESS_ONLY_BIT = 1073741824;
        #[doc(alias = "VK_PIPELINE_CREATE_2_EXECUTION_GRAPH_BIT_AMDX")]
        const EXECUTION_GRAPH_BIT_AMDX = 4294967296;
        #[doc(alias = "VK_PIPELINE_CREATE_2_DESCRIPTOR_HEAP_BIT_EXT")]
        const DESCRIPTOR_HEAP_BIT_EXT = 68719476736;
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_ALLOW_SPHERES_AND_LINEAR_SWEPT_SPHERES_BIT_NV")]
        const RAY_TRACING_ALLOW_SPHERES_AND_LINEAR_SWEPT_SPHERES_BIT_NV = 8589934592;
        #[doc(alias = "VK_PIPELINE_CREATE_2_ENABLE_LEGACY_DITHERING_BIT_EXT")]
        const ENABLE_LEGACY_DITHERING_BIT_EXT = 17179869184;
        #[doc(alias = "VK_PIPELINE_CREATE_2_DEFER_COMPILE_BIT_NV")]
        const DEFER_COMPILE_BIT_NV = 32;
        #[doc(alias = "VK_PIPELINE_CREATE_2_CAPTURE_STATISTICS_BIT_KHR")]
        const CAPTURE_STATISTICS_BIT_KHR = 64;
        #[doc(alias = "VK_PIPELINE_CREATE_2_CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR")]
        const CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR = 128;
        #[doc(alias = "VK_PIPELINE_CREATE_2_LINK_TIME_OPTIMIZATION_BIT_EXT")]
        const LINK_TIME_OPTIMIZATION_BIT_EXT = 1024;
        #[doc(alias = "VK_PIPELINE_CREATE_2_RETAIN_LINK_TIME_OPTIMIZATION_INFO_BIT_EXT")]
        const RETAIN_LINK_TIME_OPTIMIZATION_INFO_BIT_EXT = 8388608;
        #[doc(alias = "VK_PIPELINE_CREATE_2_LIBRARY_BIT_KHR")]
        const LIBRARY_BIT_KHR = 2048;
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR")]
        const RAY_TRACING_SKIP_TRIANGLES_BIT_KHR = 4096;
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_SKIP_AABBS_BIT_KHR")]
        const RAY_TRACING_SKIP_AABBS_BIT_KHR = 8192;
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR")]
        const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR = 16384;
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR")]
        const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR = 32768;
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR")]
        const RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR = 65536;
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR")]
        const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR = 131072;
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR")]
        const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR = 524288;
        #[doc(alias = "VK_PIPELINE_CREATE_2_INDIRECT_BINDABLE_BIT_NV")]
        const INDIRECT_BINDABLE_BIT_NV = 262144;
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_ALLOW_MOTION_BIT_NV")]
        const RAY_TRACING_ALLOW_MOTION_BIT_NV = 1048576;
        #[doc(alias = "VK_PIPELINE_CREATE_2_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
        const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 2097152;
        #[doc(alias = "VK_PIPELINE_CREATE_2_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT")]
        const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT = 4194304;
        #[doc(alias = "VK_PIPELINE_CREATE_2_COLOR_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT")]
        const COLOR_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT = 33554432;
        #[doc(alias = "VK_PIPELINE_CREATE_2_DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT")]
        const DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT = 67108864;
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_DISPLACEMENT_MICROMAP_BIT_NV")]
        const RAY_TRACING_DISPLACEMENT_MICROMAP_BIT_NV = 268435456;
        #[doc(alias = "VK_PIPELINE_CREATE_2_DESCRIPTOR_BUFFER_BIT_EXT")]
        const DESCRIPTOR_BUFFER_BIT_EXT = 536870912;
        #[doc(alias = "VK_PIPELINE_CREATE_2_DISALLOW_OPACITY_MICROMAP_BIT_ARM")]
        const DISALLOW_OPACITY_MICROMAP_BIT_ARM = 137438953472;
        #[doc(alias = "VK_PIPELINE_CREATE_2_INSTRUMENT_SHADERS_BIT_ARM")]
        const INSTRUMENT_SHADERS_BIT_ARM = 549755813888;
        #[doc(alias = "VK_PIPELINE_CREATE_2_CAPTURE_DATA_BIT_KHR")]
        const CAPTURE_DATA_BIT_KHR = 2147483648;
        #[doc(alias = "VK_PIPELINE_CREATE_2_INDIRECT_BINDABLE_BIT_EXT")]
        const INDIRECT_BINDABLE_BIT_EXT = 274877906944;
        #[doc(alias = "VK_PIPELINE_CREATE_2_PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE")]
        const PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE = 1099511627776;
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_OPACITY_MICROMAP_BIT_KHR")]
        const RAY_TRACING_OPACITY_MICROMAP_BIT_KHR = 16777216;
        #[doc(alias = "VK_PIPELINE_CREATE_2_OPACITY_MICROMAP_DISALLOW_MIXED_SPECIAL_INDEX_BIT_KHR")]
        const OPACITY_MICROMAP_DISALLOW_MIXED_SPECIAL_INDEX_BIT_KHR = 2199023255552;
        #[doc(alias = "VK_PIPELINE_CREATE_2_64_BIT_INDEXING_BIT_EXT")]
        const _64_BIT_INDEXING_BIT_EXT = 8796093022208;
    }
}
pub type PipelineCreateFlags2KHR = PipelineCreateFlags2;
impl PipelineCreateFlags2 {
    #[doc(alias = "VK_PIPELINE_CREATE_2_DISABLE_OPTIMIZATION_BIT_KHR")]
    pub const DISABLE_OPTIMIZATION_BIT_KHR: Self = Self::DISABLE_OPTIMIZATION_BIT;
    #[doc(alias = "VK_PIPELINE_CREATE_2_ALLOW_DERIVATIVES_BIT_KHR")]
    pub const ALLOW_DERIVATIVES_BIT_KHR: Self = Self::ALLOW_DERIVATIVES_BIT;
    #[doc(alias = "VK_PIPELINE_CREATE_2_DERIVATIVE_BIT_KHR")]
    pub const DERIVATIVE_BIT_KHR: Self = Self::DERIVATIVE_BIT;
    #[doc(alias = "VK_PIPELINE_CREATE_2_VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR")]
    pub const VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR: Self = Self::VIEW_INDEX_FROM_DEVICE_INDEX_BIT;
    #[doc(alias = "VK_PIPELINE_CREATE_2_DISPATCH_BASE_BIT_KHR")]
    pub const DISPATCH_BASE_BIT_KHR: Self = Self::DISPATCH_BASE_BIT;
    #[doc(alias = "VK_PIPELINE_CREATE_2_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT_KHR")]
    pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT_KHR: Self =
        Self::FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT;
    #[doc(alias = "VK_PIPELINE_CREATE_2_EARLY_RETURN_ON_FAILURE_BIT_KHR")]
    pub const EARLY_RETURN_ON_FAILURE_BIT_KHR: Self = Self::EARLY_RETURN_ON_FAILURE_BIT;
    #[doc(alias = "VK_PIPELINE_CREATE_2_NO_PROTECTED_ACCESS_BIT_EXT")]
    pub const NO_PROTECTED_ACCESS_BIT_EXT: Self = Self::NO_PROTECTED_ACCESS_BIT;
    #[doc(alias = "VK_PIPELINE_CREATE_2_PROTECTED_ACCESS_ONLY_BIT_EXT")]
    pub const PROTECTED_ACCESS_ONLY_BIT_EXT: Self = Self::PROTECTED_ACCESS_ONLY_BIT;
    #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_SKIP_BUILT_IN_PRIMITIVES_BIT_KHR")]
    pub const RAY_TRACING_SKIP_BUILT_IN_PRIMITIVES_BIT_KHR: Self =
        Self::RAY_TRACING_SKIP_TRIANGLES_BIT_KHR;
    #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_OPACITY_MICROMAP_BIT_EXT")]
    pub const RAY_TRACING_OPACITY_MICROMAP_BIT_EXT: Self =
        Self::RAY_TRACING_OPACITY_MICROMAP_BIT_KHR;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceTransformFlagBitsKHR.html>
    #[doc(alias = "VkSurfaceTransformFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SurfaceTransformFlagsKHR: u32 {
        #[doc(alias = "VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR")]
        const IDENTITY_BIT_KHR = 1;
        #[doc(alias = "VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR")]
        const ROTATE_90_BIT_KHR = 2;
        #[doc(alias = "VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR")]
        const ROTATE_180_BIT_KHR = 4;
        #[doc(alias = "VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR")]
        const ROTATE_270_BIT_KHR = 8;
        #[doc(alias = "VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR")]
        const HORIZONTAL_MIRROR_BIT_KHR = 16;
        #[doc(alias = "VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR")]
        const HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR = 32;
        #[doc(alias = "VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR")]
        const HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR = 64;
        #[doc(alias = "VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR")]
        const HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR = 128;
        #[doc(alias = "VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR")]
        const INHERIT_BIT_KHR = 256;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCompositeAlphaFlagBitsKHR.html>
    #[doc(alias = "VkCompositeAlphaFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct CompositeAlphaFlagsKHR: u32 {
        #[doc(alias = "VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR")]
        const OPAQUE_BIT_KHR = 1;
        #[doc(alias = "VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR")]
        const PRE_MULTIPLIED_BIT_KHR = 2;
        #[doc(alias = "VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR")]
        const POST_MULTIPLIED_BIT_KHR = 4;
        #[doc(alias = "VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR")]
        const INHERIT_BIT_KHR = 8;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainCreateFlagBitsKHR.html>
    #[doc(alias = "VkSwapchainCreateFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SwapchainCreateFlagsKHR: u32 {
        #[doc(alias = "VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR")]
        const SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR = 1;
        #[doc(alias = "VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR")]
        const PROTECTED_BIT_KHR = 2;
        #[doc(alias = "VK_SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR")]
        const MUTABLE_FORMAT_BIT_KHR = 4;
        #[doc(alias = "VK_SWAPCHAIN_CREATE_PRESENT_TIMING_BIT_EXT")]
        const PRESENT_TIMING_BIT_EXT = 512;
        #[doc(alias = "VK_SWAPCHAIN_CREATE_PRESENT_ID_2_BIT_KHR")]
        const PRESENT_ID_2_BIT_KHR = 64;
        #[doc(alias = "VK_SWAPCHAIN_CREATE_PRESENT_WAIT_2_BIT_KHR")]
        const PRESENT_WAIT_2_BIT_KHR = 128;
        #[doc(alias = "VK_SWAPCHAIN_CREATE_DEFERRED_MEMORY_ALLOCATION_BIT_KHR")]
        const DEFERRED_MEMORY_ALLOCATION_BIT_KHR = 8;
        #[doc(alias = "VK_SWAPCHAIN_CREATE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT")]
        const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT = 256;
    }
}
impl SwapchainCreateFlagsKHR {
    #[doc(alias = "VK_SWAPCHAIN_CREATE_DEFERRED_MEMORY_ALLOCATION_BIT_EXT")]
    pub const DEFERRED_MEMORY_ALLOCATION_BIT_EXT: Self = Self::DEFERRED_MEMORY_ALLOCATION_BIT_KHR;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupPresentModeFlagBitsKHR.html>
    #[doc(alias = "VkDeviceGroupPresentModeFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DeviceGroupPresentModeFlagsKHR: u32 {
        #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR")]
        const LOCAL_BIT_KHR = 1;
        #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR")]
        const REMOTE_BIT_KHR = 2;
        #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR")]
        const SUM_BIT_KHR = 4;
        #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR")]
        const LOCAL_MULTI_DEVICE_BIT_KHR = 8;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayPlaneAlphaFlagBitsKHR.html>
    #[doc(alias = "VkDisplayPlaneAlphaFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DisplayPlaneAlphaFlagsKHR: u32 {
        #[doc(alias = "VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR")]
        const OPAQUE_BIT_KHR = 1;
        #[doc(alias = "VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR")]
        const GLOBAL_BIT_KHR = 2;
        #[doc(alias = "VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR")]
        const PER_PIXEL_BIT_KHR = 4;
        #[doc(alias = "VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR")]
        const PER_PIXEL_PREMULTIPLIED_BIT_KHR = 8;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoCodecOperationFlagBitsKHR.html>
    #[doc(alias = "VkVideoCodecOperationFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoCodecOperationFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_CODEC_OPERATION_NONE_KHR")]
        const NONE_KHR = 0;
        #[doc(alias = "VK_VIDEO_CODEC_OPERATION_ENCODE_H264_BIT_KHR")]
        const ENCODE_H264_BIT_KHR = 65536;
        #[doc(alias = "VK_VIDEO_CODEC_OPERATION_ENCODE_H265_BIT_KHR")]
        const ENCODE_H265_BIT_KHR = 131072;
        #[doc(alias = "VK_VIDEO_CODEC_OPERATION_DECODE_H264_BIT_KHR")]
        const DECODE_H264_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_CODEC_OPERATION_DECODE_H265_BIT_KHR")]
        const DECODE_H265_BIT_KHR = 2;
        #[doc(alias = "VK_VIDEO_CODEC_OPERATION_DECODE_AV1_BIT_KHR")]
        const DECODE_AV1_BIT_KHR = 4;
        #[doc(alias = "VK_VIDEO_CODEC_OPERATION_ENCODE_AV1_BIT_KHR")]
        const ENCODE_AV1_BIT_KHR = 262144;
        #[doc(alias = "VK_VIDEO_CODEC_OPERATION_DECODE_VP9_BIT_KHR")]
        const DECODE_VP9_BIT_KHR = 8;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoChromaSubsamplingFlagBitsKHR.html>
    #[doc(alias = "VkVideoChromaSubsamplingFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoChromaSubsamplingFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_INVALID_KHR")]
        const INVALID_KHR = 0;
        #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_MONOCHROME_BIT_KHR")]
        const MONOCHROME_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_420_BIT_KHR")]
        const _420_BIT_KHR = 2;
        #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_422_BIT_KHR")]
        const _422_BIT_KHR = 4;
        #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_444_BIT_KHR")]
        const _444_BIT_KHR = 8;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoComponentBitDepthFlagBitsKHR.html>
    #[doc(alias = "VkVideoComponentBitDepthFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoComponentBitDepthFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_COMPONENT_BIT_DEPTH_INVALID_KHR")]
        const INVALID_KHR = 0;
        #[doc(alias = "VK_VIDEO_COMPONENT_BIT_DEPTH_8_BIT_KHR")]
        const _8_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_COMPONENT_BIT_DEPTH_10_BIT_KHR")]
        const _10_BIT_KHR = 4;
        #[doc(alias = "VK_VIDEO_COMPONENT_BIT_DEPTH_12_BIT_KHR")]
        const _12_BIT_KHR = 16;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoCapabilityFlagBitsKHR.html>
    #[doc(alias = "VkVideoCapabilityFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoCapabilityFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_CAPABILITY_PROTECTED_CONTENT_BIT_KHR")]
        const PROTECTED_CONTENT_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_CAPABILITY_SEPARATE_REFERENCE_IMAGES_BIT_KHR")]
        const SEPARATE_REFERENCE_IMAGES_BIT_KHR = 2;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionCreateFlagBitsKHR.html>
    #[doc(alias = "VkVideoSessionCreateFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoSessionCreateFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_SESSION_CREATE_PROTECTED_CONTENT_BIT_KHR")]
        const PROTECTED_CONTENT_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_SESSION_CREATE_ALLOW_ENCODE_PARAMETER_OPTIMIZATIONS_BIT_KHR")]
        const ALLOW_ENCODE_PARAMETER_OPTIMIZATIONS_BIT_KHR = 2;
        #[doc(alias = "VK_VIDEO_SESSION_CREATE_INLINE_QUERIES_BIT_KHR")]
        const INLINE_QUERIES_BIT_KHR = 4;
        #[doc(alias = "VK_VIDEO_SESSION_CREATE_ALLOW_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR")]
        const ALLOW_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR = 8;
        #[doc(alias = "VK_VIDEO_SESSION_CREATE_ALLOW_ENCODE_EMPHASIS_MAP_BIT_KHR")]
        const ALLOW_ENCODE_EMPHASIS_MAP_BIT_KHR = 16;
        #[doc(alias = "VK_VIDEO_SESSION_CREATE_INLINE_SESSION_PARAMETERS_BIT_KHR")]
        const INLINE_SESSION_PARAMETERS_BIT_KHR = 32;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionParametersCreateFlagBitsKHR.html>
    #[doc(alias = "VkVideoSessionParametersCreateFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoSessionParametersCreateFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_SESSION_PARAMETERS_CREATE_QUANTIZATION_MAP_COMPATIBLE_BIT_KHR")]
        const QUANTIZATION_MAP_COMPATIBLE_BIT_KHR = 1;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoCodingControlFlagBitsKHR.html>
    #[doc(alias = "VkVideoCodingControlFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoCodingControlFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_CODING_CONTROL_RESET_BIT_KHR")]
        const RESET_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_CODING_CONTROL_ENCODE_RATE_CONTROL_BIT_KHR")]
        const ENCODE_RATE_CONTROL_BIT_KHR = 2;
        #[doc(alias = "VK_VIDEO_CODING_CONTROL_ENCODE_QUALITY_LEVEL_BIT_KHR")]
        const ENCODE_QUALITY_LEVEL_BIT_KHR = 4;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeCapabilityFlagBitsKHR.html>
    #[doc(alias = "VkVideoDecodeCapabilityFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoDecodeCapabilityFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_COINCIDE_BIT_KHR")]
        const DPB_AND_OUTPUT_COINCIDE_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_DISTINCT_BIT_KHR")]
        const DPB_AND_OUTPUT_DISTINCT_BIT_KHR = 2;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeUsageFlagBitsKHR.html>
    #[doc(alias = "VkVideoDecodeUsageFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoDecodeUsageFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_DECODE_USAGE_DEFAULT_KHR")]
        const DEFAULT_KHR = 0;
        #[doc(alias = "VK_VIDEO_DECODE_USAGE_TRANSCODING_BIT_KHR")]
        const TRANSCODING_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_DECODE_USAGE_OFFLINE_BIT_KHR")]
        const OFFLINE_BIT_KHR = 2;
        #[doc(alias = "VK_VIDEO_DECODE_USAGE_STREAMING_BIT_KHR")]
        const STREAMING_BIT_KHR = 4;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264CapabilityFlagBitsKHR.html>
    #[doc(alias = "VkVideoEncodeH264CapabilityFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeH264CapabilityFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_HRD_COMPLIANCE_BIT_KHR")]
        const VIDEO_ENCODE_H264_CAPABILITY_HRD_COMPLIANCE_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_PREDICTION_WEIGHT_TABLE_GENERATED_BIT_KHR")]
        const VIDEO_ENCODE_H264_CAPABILITY_PREDICTION_WEIGHT_TABLE_GENERATED_BIT_KHR = 2;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_ROW_UNALIGNED_SLICE_BIT_KHR")]
        const VIDEO_ENCODE_H264_CAPABILITY_ROW_UNALIGNED_SLICE_BIT_KHR = 4;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_DIFFERENT_SLICE_TYPE_BIT_KHR")]
        const VIDEO_ENCODE_H264_CAPABILITY_DIFFERENT_SLICE_TYPE_BIT_KHR = 8;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L0_LIST_BIT_KHR")]
        const VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L0_LIST_BIT_KHR = 16;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L1_LIST_BIT_KHR")]
        const VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L1_LIST_BIT_KHR = 32;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_PER_PICTURE_TYPE_MIN_MAX_QP_BIT_KHR")]
        const VIDEO_ENCODE_H264_CAPABILITY_PER_PICTURE_TYPE_MIN_MAX_QP_BIT_KHR = 64;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_PER_SLICE_CONSTANT_QP_BIT_KHR")]
        const VIDEO_ENCODE_H264_CAPABILITY_PER_SLICE_CONSTANT_QP_BIT_KHR = 128;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_GENERATE_PREFIX_NALU_BIT_KHR")]
        const VIDEO_ENCODE_H264_CAPABILITY_GENERATE_PREFIX_NALU_BIT_KHR = 256;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_B_PICTURE_INTRA_REFRESH_BIT_KHR")]
        const VIDEO_ENCODE_H264_CAPABILITY_B_PICTURE_INTRA_REFRESH_BIT_KHR = 1024;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_MB_QP_DIFF_WRAPAROUND_BIT_KHR")]
        const VIDEO_ENCODE_H264_CAPABILITY_MB_QP_DIFF_WRAPAROUND_BIT_KHR = 512;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264StdFlagBitsKHR.html>
    #[doc(alias = "VkVideoEncodeH264StdFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeH264StdFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_SEPARATE_COLOR_PLANE_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_SEPARATE_COLOR_PLANE_FLAG_SET_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_BIT_KHR = 2;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_SCALING_MATRIX_PRESENT_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_SCALING_MATRIX_PRESENT_FLAG_SET_BIT_KHR = 4;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_CHROMA_QP_INDEX_OFFSET_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_CHROMA_QP_INDEX_OFFSET_BIT_KHR = 8;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_SECOND_CHROMA_QP_INDEX_OFFSET_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_SECOND_CHROMA_QP_INDEX_OFFSET_BIT_KHR = 16;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_PIC_INIT_QP_MINUS26_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_PIC_INIT_QP_MINUS26_BIT_KHR = 32;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_WEIGHTED_PRED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_WEIGHTED_PRED_FLAG_SET_BIT_KHR = 64;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_WEIGHTED_BIPRED_IDC_EXPLICIT_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_WEIGHTED_BIPRED_IDC_EXPLICIT_BIT_KHR = 128;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_WEIGHTED_BIPRED_IDC_IMPLICIT_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_WEIGHTED_BIPRED_IDC_IMPLICIT_BIT_KHR = 256;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_TRANSFORM_8X8_MODE_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_TRANSFORM_8X8_MODE_FLAG_SET_BIT_KHR = 512;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_BIT_KHR = 1024;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_ENTROPY_CODING_MODE_FLAG_UNSET_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_ENTROPY_CODING_MODE_FLAG_UNSET_BIT_KHR = 2048;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_ENTROPY_CODING_MODE_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_ENTROPY_CODING_MODE_FLAG_SET_BIT_KHR = 4096;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_DIRECT_8X8_INFERENCE_FLAG_UNSET_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_DIRECT_8X8_INFERENCE_FLAG_UNSET_BIT_KHR = 8192;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_CONSTRAINED_INTRA_PRED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_CONSTRAINED_INTRA_PRED_FLAG_SET_BIT_KHR = 16384;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_DISABLED_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_DISABLED_BIT_KHR = 32768;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_ENABLED_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_ENABLED_BIT_KHR = 65536;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_PARTIAL_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_PARTIAL_BIT_KHR = 131072;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_SLICE_QP_DELTA_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_SLICE_QP_DELTA_BIT_KHR = 524288;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_DIFFERENT_SLICE_QP_DELTA_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_DIFFERENT_SLICE_QP_DELTA_BIT_KHR = 1048576;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264RateControlFlagBitsKHR.html>
    #[doc(alias = "VkVideoEncodeH264RateControlFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeH264RateControlFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_ENCODE_H264_RATE_CONTROL_ATTEMPT_HRD_COMPLIANCE_BIT_KHR")]
        const VIDEO_ENCODE_H264_RATE_CONTROL_ATTEMPT_HRD_COMPLIANCE_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_RATE_CONTROL_REGULAR_GOP_BIT_KHR")]
        const VIDEO_ENCODE_H264_RATE_CONTROL_REGULAR_GOP_BIT_KHR = 2;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR")]
        const VIDEO_ENCODE_H264_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR = 4;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR")]
        const VIDEO_ENCODE_H264_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR = 8;
        #[doc(alias = "VK_VIDEO_ENCODE_H264_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_BIT_KHR")]
        const VIDEO_ENCODE_H264_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_BIT_KHR = 16;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265CapabilityFlagBitsKHR.html>
    #[doc(alias = "VkVideoEncodeH265CapabilityFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeH265CapabilityFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_HRD_COMPLIANCE_BIT_KHR")]
        const VIDEO_ENCODE_H265_CAPABILITY_HRD_COMPLIANCE_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_PREDICTION_WEIGHT_TABLE_GENERATED_BIT_KHR")]
        const VIDEO_ENCODE_H265_CAPABILITY_PREDICTION_WEIGHT_TABLE_GENERATED_BIT_KHR = 2;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_ROW_UNALIGNED_SLICE_SEGMENT_BIT_KHR")]
        const VIDEO_ENCODE_H265_CAPABILITY_ROW_UNALIGNED_SLICE_SEGMENT_BIT_KHR = 4;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_DIFFERENT_SLICE_SEGMENT_TYPE_BIT_KHR")]
        const VIDEO_ENCODE_H265_CAPABILITY_DIFFERENT_SLICE_SEGMENT_TYPE_BIT_KHR = 8;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L0_LIST_BIT_KHR")]
        const VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L0_LIST_BIT_KHR = 16;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L1_LIST_BIT_KHR")]
        const VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L1_LIST_BIT_KHR = 32;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_PER_PICTURE_TYPE_MIN_MAX_QP_BIT_KHR")]
        const VIDEO_ENCODE_H265_CAPABILITY_PER_PICTURE_TYPE_MIN_MAX_QP_BIT_KHR = 64;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_PER_SLICE_SEGMENT_CONSTANT_QP_BIT_KHR")]
        const VIDEO_ENCODE_H265_CAPABILITY_PER_SLICE_SEGMENT_CONSTANT_QP_BIT_KHR = 128;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_TILES_PER_SLICE_SEGMENT_BIT_KHR")]
        const VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_TILES_PER_SLICE_SEGMENT_BIT_KHR = 256;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_SLICE_SEGMENTS_PER_TILE_BIT_KHR")]
        const VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_SLICE_SEGMENTS_PER_TILE_BIT_KHR = 512;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_B_PICTURE_INTRA_REFRESH_BIT_KHR")]
        const VIDEO_ENCODE_H265_CAPABILITY_B_PICTURE_INTRA_REFRESH_BIT_KHR = 2048;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_CU_QP_DIFF_WRAPAROUND_BIT_KHR")]
        const VIDEO_ENCODE_H265_CAPABILITY_CU_QP_DIFF_WRAPAROUND_BIT_KHR = 1024;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265StdFlagBitsKHR.html>
    #[doc(alias = "VkVideoEncodeH265StdFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeH265StdFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_SEPARATE_COLOR_PLANE_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_SEPARATE_COLOR_PLANE_FLAG_SET_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET_BIT_KHR = 2;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_SCALING_LIST_DATA_PRESENT_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_SCALING_LIST_DATA_PRESENT_FLAG_SET_BIT_KHR = 4;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_PCM_ENABLED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_PCM_ENABLED_FLAG_SET_BIT_KHR = 8;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_SPS_TEMPORAL_MVP_ENABLED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_SPS_TEMPORAL_MVP_ENABLED_FLAG_SET_BIT_KHR = 16;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_INIT_QP_MINUS26_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_INIT_QP_MINUS26_BIT_KHR = 32;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_WEIGHTED_PRED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_WEIGHTED_PRED_FLAG_SET_BIT_KHR = 64;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_WEIGHTED_BIPRED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_WEIGHTED_BIPRED_FLAG_SET_BIT_KHR = 128;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_LOG2_PARALLEL_MERGE_LEVEL_MINUS2_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_LOG2_PARALLEL_MERGE_LEVEL_MINUS2_BIT_KHR = 256;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_SIGN_DATA_HIDING_ENABLED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_SIGN_DATA_HIDING_ENABLED_FLAG_SET_BIT_KHR = 512;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_TRANSFORM_SKIP_ENABLED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_TRANSFORM_SKIP_ENABLED_FLAG_SET_BIT_KHR = 1024;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_TRANSFORM_SKIP_ENABLED_FLAG_UNSET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_TRANSFORM_SKIP_ENABLED_FLAG_UNSET_BIT_KHR = 2048;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET_BIT_KHR = 4096;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_TRANSQUANT_BYPASS_ENABLED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_TRANSQUANT_BYPASS_ENABLED_FLAG_SET_BIT_KHR = 8192;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_CONSTRAINED_INTRA_PRED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_CONSTRAINED_INTRA_PRED_FLAG_SET_BIT_KHR = 16384;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_ENTROPY_CODING_SYNC_ENABLED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_ENTROPY_CODING_SYNC_ENABLED_FLAG_SET_BIT_KHR = 32768;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET_BIT_KHR = 65536;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET_BIT_KHR = 131072;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_DEPENDENT_SLICE_SEGMENT_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_DEPENDENT_SLICE_SEGMENT_FLAG_SET_BIT_KHR = 262144;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_SLICE_QP_DELTA_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_SLICE_QP_DELTA_BIT_KHR = 524288;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_DIFFERENT_SLICE_QP_DELTA_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_DIFFERENT_SLICE_QP_DELTA_BIT_KHR = 1048576;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265CtbSizeFlagBitsKHR.html>
    #[doc(alias = "VkVideoEncodeH265CtbSizeFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeH265CtbSizeFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CTB_SIZE_16_BIT_KHR")]
        const VIDEO_ENCODE_H265_CTB_SIZE_16_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CTB_SIZE_32_BIT_KHR")]
        const VIDEO_ENCODE_H265_CTB_SIZE_32_BIT_KHR = 2;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CTB_SIZE_64_BIT_KHR")]
        const VIDEO_ENCODE_H265_CTB_SIZE_64_BIT_KHR = 4;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265TransformBlockSizeFlagBitsKHR.html>
    #[doc(alias = "VkVideoEncodeH265TransformBlockSizeFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeH265TransformBlockSizeFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_4_BIT_KHR")]
        const VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_4_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_8_BIT_KHR")]
        const VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_8_BIT_KHR = 2;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_16_BIT_KHR")]
        const VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_16_BIT_KHR = 4;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_32_BIT_KHR")]
        const VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_32_BIT_KHR = 8;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265RateControlFlagBitsKHR.html>
    #[doc(alias = "VkVideoEncodeH265RateControlFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeH265RateControlFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_ENCODE_H265_RATE_CONTROL_ATTEMPT_HRD_COMPLIANCE_BIT_KHR")]
        const VIDEO_ENCODE_H265_RATE_CONTROL_ATTEMPT_HRD_COMPLIANCE_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_RATE_CONTROL_REGULAR_GOP_BIT_KHR")]
        const VIDEO_ENCODE_H265_RATE_CONTROL_REGULAR_GOP_BIT_KHR = 2;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR")]
        const VIDEO_ENCODE_H265_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR = 4;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR")]
        const VIDEO_ENCODE_H265_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR = 8;
        #[doc(alias = "VK_VIDEO_ENCODE_H265_RATE_CONTROL_TEMPORAL_SUB_LAYER_PATTERN_DYADIC_BIT_KHR")]
        const VIDEO_ENCODE_H265_RATE_CONTROL_TEMPORAL_SUB_LAYER_PATTERN_DYADIC_BIT_KHR = 16;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeH264PictureLayoutFlagBitsKHR.html>
    #[doc(alias = "VkVideoDecodeH264PictureLayoutFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoDecodeH264PictureLayoutFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_PROGRESSIVE_KHR")]
        const VIDEO_DECODE_H264_PICTURE_LAYOUT_PROGRESSIVE_KHR = 0;
        #[doc(alias = "VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_INTERLEAVED_LINES_BIT_KHR")]
        const VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_INTERLEAVED_LINES_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_SEPARATE_PLANES_BIT_KHR")]
        const VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_SEPARATE_PLANES_BIT_KHR = 2;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterDescriptionFlagBitsKHR.html>
    #[doc(alias = "VkPerformanceCounterDescriptionFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PerformanceCounterDescriptionFlagsKHR: u32 {
        #[doc(alias = "VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR")]
        const PERFORMANCE_IMPACTING_BIT_KHR = 1;
        #[doc(alias = "VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR")]
        const CONCURRENTLY_IMPACTED_BIT_KHR = 2;
    }
}
impl PerformanceCounterDescriptionFlagsKHR {
    #[doc(alias = "VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_KHR")]
    pub const PERFORMANCE_IMPACTING_KHR: Self = Self::PERFORMANCE_IMPACTING_BIT_KHR;
    #[doc(alias = "VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_KHR")]
    pub const CONCURRENTLY_IMPACTED_KHR: Self = Self::CONCURRENTLY_IMPACTED_BIT_KHR;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAcquireProfilingLockFlagBitsKHR.html>
    #[doc(alias = "VkAcquireProfilingLockFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct AcquireProfilingLockFlagsKHR: u32 {
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeFlagBitsKHR.html>
    #[doc(alias = "VkVideoEncodeFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_ENCODE_INTRA_REFRESH_BIT_KHR")]
        const INTRA_REFRESH_BIT_KHR = 4;
        #[doc(alias = "VK_VIDEO_ENCODE_WITH_QUANTIZATION_DELTA_MAP_BIT_KHR")]
        const WITH_QUANTIZATION_DELTA_MAP_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_ENCODE_WITH_EMPHASIS_MAP_BIT_KHR")]
        const WITH_EMPHASIS_MAP_BIT_KHR = 2;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeCapabilityFlagBitsKHR.html>
    #[doc(alias = "VkVideoEncodeCapabilityFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeCapabilityFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_ENCODE_CAPABILITY_PRECEDING_EXTERNALLY_ENCODED_BYTES_BIT_KHR")]
        const PRECEDING_EXTERNALLY_ENCODED_BYTES_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_ENCODE_CAPABILITY_INSUFFICIENT_BITSTREAM_BUFFER_RANGE_DETECTION_BIT_KHR")]
        const INSUFFICIENT_BITSTREAM_BUFFER_RANGE_DETECTION_BIT_KHR = 2;
        #[doc(alias = "VK_VIDEO_ENCODE_CAPABILITY_QUANTIZATION_DELTA_MAP_BIT_KHR")]
        const QUANTIZATION_DELTA_MAP_BIT_KHR = 4;
        #[doc(alias = "VK_VIDEO_ENCODE_CAPABILITY_EMPHASIS_MAP_BIT_KHR")]
        const EMPHASIS_MAP_BIT_KHR = 8;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRateControlModeFlagBitsKHR.html>
    #[doc(alias = "VkVideoEncodeRateControlModeFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeRateControlModeFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_ENCODE_RATE_CONTROL_MODE_DEFAULT_KHR")]
        const DEFAULT_KHR = 0;
        #[doc(alias = "VK_VIDEO_ENCODE_RATE_CONTROL_MODE_DISABLED_BIT_KHR")]
        const DISABLED_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_ENCODE_RATE_CONTROL_MODE_CBR_BIT_KHR")]
        const CBR_BIT_KHR = 2;
        #[doc(alias = "VK_VIDEO_ENCODE_RATE_CONTROL_MODE_VBR_BIT_KHR")]
        const VBR_BIT_KHR = 4;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeFeedbackFlagBitsKHR.html>
    #[doc(alias = "VkVideoEncodeFeedbackFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeFeedbackFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_BITSTREAM_BUFFER_OFFSET_BIT_KHR")]
        const BITSTREAM_BUFFER_OFFSET_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_BITSTREAM_BYTES_WRITTEN_BIT_KHR")]
        const BITSTREAM_BYTES_WRITTEN_BIT_KHR = 2;
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_BITSTREAM_HAS_OVERRIDES_BIT_KHR")]
        const BITSTREAM_HAS_OVERRIDES_BIT_KHR = 4;
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_AVERAGE_QUANTIZATION_BIT_KHR")]
        const AVERAGE_QUANTIZATION_BIT_KHR = 8;
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_MIN_QUANTIZATION_BIT_KHR")]
        const MIN_QUANTIZATION_BIT_KHR = 16;
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_MAX_QUANTIZATION_BIT_KHR")]
        const MAX_QUANTIZATION_BIT_KHR = 32;
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_INTRA_PIXELS_BIT_KHR")]
        const INTRA_PIXELS_BIT_KHR = 64;
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_INTER_PIXELS_BIT_KHR")]
        const INTER_PIXELS_BIT_KHR = 128;
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_SKIPPED_PIXELS_BIT_KHR")]
        const SKIPPED_PIXELS_BIT_KHR = 256;
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_PICTURE_PARTITION_COUNT_BIT_KHR")]
        const PICTURE_PARTITION_COUNT_BIT_KHR = 512;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeUsageFlagBitsKHR.html>
    #[doc(alias = "VkVideoEncodeUsageFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeUsageFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_ENCODE_USAGE_DEFAULT_KHR")]
        const DEFAULT_KHR = 0;
        #[doc(alias = "VK_VIDEO_ENCODE_USAGE_TRANSCODING_BIT_KHR")]
        const TRANSCODING_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_ENCODE_USAGE_STREAMING_BIT_KHR")]
        const STREAMING_BIT_KHR = 2;
        #[doc(alias = "VK_VIDEO_ENCODE_USAGE_RECORDING_BIT_KHR")]
        const RECORDING_BIT_KHR = 4;
        #[doc(alias = "VK_VIDEO_ENCODE_USAGE_CONFERENCING_BIT_KHR")]
        const CONFERENCING_BIT_KHR = 8;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeContentFlagBitsKHR.html>
    #[doc(alias = "VkVideoEncodeContentFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeContentFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_ENCODE_CONTENT_DEFAULT_KHR")]
        const DEFAULT_KHR = 0;
        #[doc(alias = "VK_VIDEO_ENCODE_CONTENT_CAMERA_BIT_KHR")]
        const CAMERA_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_ENCODE_CONTENT_DESKTOP_BIT_KHR")]
        const DESKTOP_BIT_KHR = 2;
        #[doc(alias = "VK_VIDEO_ENCODE_CONTENT_RENDERED_BIT_KHR")]
        const RENDERED_BIT_KHR = 4;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAddressCommandFlagBitsKHR.html>
    #[doc(alias = "VkAddressCommandFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct AddressCommandFlagsKHR: u32 {
        #[doc(alias = "VK_ADDRESS_COMMAND_PROTECTED_BIT_KHR")]
        const PROTECTED_BIT_KHR = 1;
        #[doc(alias = "VK_ADDRESS_COMMAND_FULLY_BOUND_BIT_KHR")]
        const FULLY_BOUND_BIT_KHR = 2;
        #[doc(alias = "VK_ADDRESS_COMMAND_STORAGE_BUFFER_USAGE_BIT_KHR")]
        const STORAGE_BUFFER_USAGE_BIT_KHR = 4;
        #[doc(alias = "VK_ADDRESS_COMMAND_UNKNOWN_STORAGE_BUFFER_USAGE_BIT_KHR")]
        const UNKNOWN_STORAGE_BUFFER_USAGE_BIT_KHR = 8;
        #[doc(alias = "VK_ADDRESS_COMMAND_TRANSFORM_FEEDBACK_BUFFER_USAGE_BIT_KHR")]
        const TRANSFORM_FEEDBACK_BUFFER_USAGE_BIT_KHR = 16;
        #[doc(alias = "VK_ADDRESS_COMMAND_UNKNOWN_TRANSFORM_FEEDBACK_BUFFER_USAGE_BIT_KHR")]
        const UNKNOWN_TRANSFORM_FEEDBACK_BUFFER_USAGE_BIT_KHR = 32;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkConditionalRenderingFlagBitsEXT.html>
    #[doc(alias = "VkConditionalRenderingFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ConditionalRenderingFlagsEXT: u32 {
        #[doc(alias = "VK_CONDITIONAL_RENDERING_INVERTED_BIT_EXT")]
        const INVERTED_BIT_EXT = 1;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureCreateFlagBitsKHR.html>
    #[doc(alias = "VkAccelerationStructureCreateFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct AccelerationStructureCreateFlagsKHR: u32 {
        #[doc(alias = "VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR")]
        const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR = 1;
        #[doc(alias = "VK_ACCELERATION_STRUCTURE_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT")]
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT = 8;
        #[doc(alias = "VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV")]
        const MOTION_BIT_NV = 4;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentScalingFlagBitsKHR.html>
    #[doc(alias = "VkPresentScalingFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PresentScalingFlagsKHR: u32 {
        #[doc(alias = "VK_PRESENT_SCALING_ONE_TO_ONE_BIT_KHR")]
        const ONE_TO_ONE_BIT_KHR = 1;
        #[doc(alias = "VK_PRESENT_SCALING_ASPECT_RATIO_STRETCH_BIT_KHR")]
        const ASPECT_RATIO_STRETCH_BIT_KHR = 2;
        #[doc(alias = "VK_PRESENT_SCALING_STRETCH_BIT_KHR")]
        const STRETCH_BIT_KHR = 4;
    }
}
pub type PresentScalingFlagsEXT = PresentScalingFlagsKHR;
impl PresentScalingFlagsKHR {
    #[doc(alias = "VK_PRESENT_SCALING_ONE_TO_ONE_BIT_EXT")]
    pub const ONE_TO_ONE_BIT_EXT: Self = Self::ONE_TO_ONE_BIT_KHR;
    #[doc(alias = "VK_PRESENT_SCALING_ASPECT_RATIO_STRETCH_BIT_EXT")]
    pub const ASPECT_RATIO_STRETCH_BIT_EXT: Self = Self::ASPECT_RATIO_STRETCH_BIT_KHR;
    #[doc(alias = "VK_PRESENT_SCALING_STRETCH_BIT_EXT")]
    pub const STRETCH_BIT_EXT: Self = Self::STRETCH_BIT_KHR;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentGravityFlagBitsKHR.html>
    #[doc(alias = "VkPresentGravityFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PresentGravityFlagsKHR: u32 {
        #[doc(alias = "VK_PRESENT_GRAVITY_MIN_BIT_KHR")]
        const MIN_BIT_KHR = 1;
        #[doc(alias = "VK_PRESENT_GRAVITY_MAX_BIT_KHR")]
        const MAX_BIT_KHR = 2;
        #[doc(alias = "VK_PRESENT_GRAVITY_CENTERED_BIT_KHR")]
        const CENTERED_BIT_KHR = 4;
    }
}
pub type PresentGravityFlagsEXT = PresentGravityFlagsKHR;
impl PresentGravityFlagsKHR {
    #[doc(alias = "VK_PRESENT_GRAVITY_MIN_BIT_EXT")]
    pub const MIN_BIT_EXT: Self = Self::MIN_BIT_KHR;
    #[doc(alias = "VK_PRESENT_GRAVITY_MAX_BIT_EXT")]
    pub const MAX_BIT_EXT: Self = Self::MAX_BIT_KHR;
    #[doc(alias = "VK_PRESENT_GRAVITY_CENTERED_BIT_EXT")]
    pub const CENTERED_BIT_EXT: Self = Self::CENTERED_BIT_KHR;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1CapabilityFlagBitsKHR.html>
    #[doc(alias = "VkVideoEncodeAV1CapabilityFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeAV1CapabilityFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_CAPABILITY_PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_BIT_KHR")]
        const VIDEO_ENCODE_AV1_CAPABILITY_PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_CAPABILITY_GENERATE_OBU_EXTENSION_HEADER_BIT_KHR")]
        const VIDEO_ENCODE_AV1_CAPABILITY_GENERATE_OBU_EXTENSION_HEADER_BIT_KHR = 2;
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_CAPABILITY_PRIMARY_REFERENCE_CDF_ONLY_BIT_KHR")]
        const VIDEO_ENCODE_AV1_CAPABILITY_PRIMARY_REFERENCE_CDF_ONLY_BIT_KHR = 4;
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_CAPABILITY_FRAME_SIZE_OVERRIDE_BIT_KHR")]
        const VIDEO_ENCODE_AV1_CAPABILITY_FRAME_SIZE_OVERRIDE_BIT_KHR = 8;
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_CAPABILITY_MOTION_VECTOR_SCALING_BIT_KHR")]
        const VIDEO_ENCODE_AV1_CAPABILITY_MOTION_VECTOR_SCALING_BIT_KHR = 16;
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_CAPABILITY_COMPOUND_PREDICTION_INTRA_REFRESH_BIT_KHR")]
        const VIDEO_ENCODE_AV1_CAPABILITY_COMPOUND_PREDICTION_INTRA_REFRESH_BIT_KHR = 32;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1StdFlagBitsKHR.html>
    #[doc(alias = "VkVideoEncodeAV1StdFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeAV1StdFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_STD_UNIFORM_TILE_SPACING_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_AV1_STD_UNIFORM_TILE_SPACING_FLAG_SET_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_STD_SKIP_MODE_PRESENT_UNSET_BIT_KHR")]
        const VIDEO_ENCODE_AV1_STD_SKIP_MODE_PRESENT_UNSET_BIT_KHR = 2;
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_STD_PRIMARY_REF_FRAME_BIT_KHR")]
        const VIDEO_ENCODE_AV1_STD_PRIMARY_REF_FRAME_BIT_KHR = 4;
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_STD_DELTA_Q_BIT_KHR")]
        const VIDEO_ENCODE_AV1_STD_DELTA_Q_BIT_KHR = 8;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1SuperblockSizeFlagBitsKHR.html>
    #[doc(alias = "VkVideoEncodeAV1SuperblockSizeFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeAV1SuperblockSizeFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_64_BIT_KHR")]
        const VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_64_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_128_BIT_KHR")]
        const VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_128_BIT_KHR = 2;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1RateControlFlagBitsKHR.html>
    #[doc(alias = "VkVideoEncodeAV1RateControlFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeAV1RateControlFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_RATE_CONTROL_REGULAR_GOP_BIT_KHR")]
        const VIDEO_ENCODE_AV1_RATE_CONTROL_REGULAR_GOP_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_BIT_KHR")]
        const VIDEO_ENCODE_AV1_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_BIT_KHR = 2;
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR")]
        const VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR = 4;
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR")]
        const VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR = 8;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAddressCopyFlagBitsKHR.html>
    #[doc(alias = "VkAddressCopyFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct AddressCopyFlagsKHR: u32 {
        #[doc(alias = "VK_ADDRESS_COPY_DEVICE_LOCAL_BIT_KHR")]
        const DEVICE_LOCAL_BIT_KHR = 1;
        #[doc(alias = "VK_ADDRESS_COPY_SPARSE_BIT_KHR")]
        const SPARSE_BIT_KHR = 2;
        #[doc(alias = "VK_ADDRESS_COPY_PROTECTED_BIT_KHR")]
        const PROTECTED_BIT_KHR = 4;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeIntraRefreshModeFlagBitsKHR.html>
    #[doc(alias = "VkVideoEncodeIntraRefreshModeFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeIntraRefreshModeFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_NONE_KHR")]
        const NONE_KHR = 0;
        #[doc(alias = "VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_PER_PICTURE_PARTITION_BIT_KHR")]
        const PER_PICTURE_PARTITION_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_BLOCK_BASED_BIT_KHR")]
        const BLOCK_BASED_BIT_KHR = 2;
        #[doc(alias = "VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_BLOCK_ROW_BASED_BIT_KHR")]
        const BLOCK_ROW_BASED_BIT_KHR = 4;
        #[doc(alias = "VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_BLOCK_COLUMN_BASED_BIT_KHR")]
        const BLOCK_COLUMN_BASED_BIT_KHR = 8;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceFaultFlagBitsKHR.html>
    #[doc(alias = "VkDeviceFaultFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DeviceFaultFlagsKHR: u32 {
        #[doc(alias = "VK_DEVICE_FAULT_FLAG_DEVICE_LOST_KHR")]
        const FLAG_DEVICE_LOST_KHR = 1;
        #[doc(alias = "VK_DEVICE_FAULT_FLAG_MEMORY_ADDRESS_KHR")]
        const FLAG_MEMORY_ADDRESS_KHR = 2;
        #[doc(alias = "VK_DEVICE_FAULT_FLAG_INSTRUCTION_ADDRESS_KHR")]
        const FLAG_INSTRUCTION_ADDRESS_KHR = 4;
        #[doc(alias = "VK_DEVICE_FAULT_FLAG_VENDOR_KHR")]
        const FLAG_VENDOR_KHR = 8;
        #[doc(alias = "VK_DEVICE_FAULT_FLAG_WATCHDOG_TIMEOUT_KHR")]
        const FLAG_WATCHDOG_TIMEOUT_KHR = 16;
        #[doc(alias = "VK_DEVICE_FAULT_FLAG_OVERFLOW_KHR")]
        const FLAG_OVERFLOW_KHR = 32;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlagBits3KHR.html>
    #[doc(alias = "VkAccessFlagBits3KHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct AccessFlags3KHR: u64 {
        #[doc(alias = "VK_ACCESS_3_NONE_KHR")]
        const NONE_KHR = 0;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodePerPartitionFeedbackFlagBitsKHR.html>
    #[doc(alias = "VkVideoEncodePerPartitionFeedbackFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodePerPartitionFeedbackFlagsKHR: u32 {
        #[doc(alias = "VK_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_STATUS_BIT_KHR")]
        const STATUS_BIT_KHR = 1;
        #[doc(alias = "VK_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_BITSTREAM_BUFFER_OFFSET_BIT_KHR")]
        const BITSTREAM_BUFFER_OFFSET_BIT_KHR = 2;
        #[doc(alias = "VK_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_BITSTREAM_BYTES_WRITTEN_BIT_KHR")]
        const BITSTREAM_BYTES_WRITTEN_BIT_KHR = 4;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingAttachmentFlagBitsKHR.html>
    #[doc(alias = "VkRenderingAttachmentFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct RenderingAttachmentFlagsKHR: u32 {
        #[doc(alias = "VK_RENDERING_ATTACHMENT_INPUT_ATTACHMENT_FEEDBACK_BIT_KHR")]
        const INPUT_ATTACHMENT_FEEDBACK_BIT_KHR = 1;
        #[doc(alias = "VK_RENDERING_ATTACHMENT_RESOLVE_SKIP_TRANSFER_FUNCTION_BIT_KHR")]
        const RESOLVE_SKIP_TRANSFER_FUNCTION_BIT_KHR = 2;
        #[doc(alias = "VK_RENDERING_ATTACHMENT_RESOLVE_ENABLE_TRANSFER_FUNCTION_BIT_KHR")]
        const RESOLVE_ENABLE_TRANSFER_FUNCTION_BIT_KHR = 4;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkResolveImageFlagBitsKHR.html>
    #[doc(alias = "VkResolveImageFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ResolveImageFlagsKHR: u32 {
        #[doc(alias = "VK_RESOLVE_IMAGE_SKIP_TRANSFER_FUNCTION_BIT_KHR")]
        const SKIP_TRANSFER_FUNCTION_BIT_KHR = 1;
        #[doc(alias = "VK_RESOLVE_IMAGE_ENABLE_TRANSFER_FUNCTION_BIT_KHR")]
        const ENABLE_TRANSFER_FUNCTION_BIT_KHR = 2;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlagBits4KHR.html>
    #[doc(alias = "VkFormatFeatureFlagBits4KHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct FormatFeatureFlags4KHR: u64 {
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageUsageFlagBits2KHR.html>
    #[doc(alias = "VkImageUsageFlagBits2KHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ImageUsageFlags2KHR: u64 {
        #[doc(alias = "VK_IMAGE_USAGE_2_TRANSFER_SRC_BIT_KHR")]
        const TRANSFER_SRC_BIT_KHR = 1;
        #[doc(alias = "VK_IMAGE_USAGE_2_TRANSFER_DST_BIT_KHR")]
        const TRANSFER_DST_BIT_KHR = 2;
        #[doc(alias = "VK_IMAGE_USAGE_2_SAMPLED_BIT_KHR")]
        const SAMPLED_BIT_KHR = 4;
        #[doc(alias = "VK_IMAGE_USAGE_2_STORAGE_BIT_KHR")]
        const STORAGE_BIT_KHR = 8;
        #[doc(alias = "VK_IMAGE_USAGE_2_COLOR_ATTACHMENT_BIT_KHR")]
        const COLOR_ATTACHMENT_BIT_KHR = 16;
        #[doc(alias = "VK_IMAGE_USAGE_2_DEPTH_STENCIL_ATTACHMENT_BIT_KHR")]
        const DEPTH_STENCIL_ATTACHMENT_BIT_KHR = 32;
        #[doc(alias = "VK_IMAGE_USAGE_2_TRANSIENT_ATTACHMENT_BIT_KHR")]
        const TRANSIENT_ATTACHMENT_BIT_KHR = 64;
        #[doc(alias = "VK_IMAGE_USAGE_2_INPUT_ATTACHMENT_BIT_KHR")]
        const INPUT_ATTACHMENT_BIT_KHR = 128;
        #[doc(alias = "VK_IMAGE_USAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
        const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 256;
        #[doc(alias = "VK_IMAGE_USAGE_2_FRAGMENT_DENSITY_MAP_BIT_EXT")]
        const FRAGMENT_DENSITY_MAP_BIT_EXT = 512;
        #[doc(alias = "VK_IMAGE_USAGE_2_VIDEO_DECODE_DST_BIT_KHR")]
        const VIDEO_DECODE_DST_BIT_KHR = 1024;
        #[doc(alias = "VK_IMAGE_USAGE_2_VIDEO_DECODE_SRC_BIT_KHR")]
        const VIDEO_DECODE_SRC_BIT_KHR = 2048;
        #[doc(alias = "VK_IMAGE_USAGE_2_VIDEO_DECODE_DPB_BIT_KHR")]
        const VIDEO_DECODE_DPB_BIT_KHR = 4096;
        #[doc(alias = "VK_IMAGE_USAGE_2_VIDEO_ENCODE_DST_BIT_KHR")]
        const VIDEO_ENCODE_DST_BIT_KHR = 8192;
        #[doc(alias = "VK_IMAGE_USAGE_2_VIDEO_ENCODE_SRC_BIT_KHR")]
        const VIDEO_ENCODE_SRC_BIT_KHR = 16384;
        #[doc(alias = "VK_IMAGE_USAGE_2_VIDEO_ENCODE_DPB_BIT_KHR")]
        const VIDEO_ENCODE_DPB_BIT_KHR = 32768;
        #[doc(alias = "VK_IMAGE_USAGE_2_INVOCATION_MASK_BIT_HUAWEI")]
        const INVOCATION_MASK_BIT_HUAWEI = 262144;
        #[doc(alias = "VK_IMAGE_USAGE_2_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT")]
        const ATTACHMENT_FEEDBACK_LOOP_BIT_EXT = 524288;
        #[doc(alias = "VK_IMAGE_USAGE_2_SAMPLE_WEIGHT_BIT_QCOM")]
        const SAMPLE_WEIGHT_BIT_QCOM = 1048576;
        #[doc(alias = "VK_IMAGE_USAGE_2_SAMPLE_BLOCK_MATCH_BIT_QCOM")]
        const SAMPLE_BLOCK_MATCH_BIT_QCOM = 2097152;
        #[doc(alias = "VK_IMAGE_USAGE_2_HOST_TRANSFER_BIT_KHR")]
        const HOST_TRANSFER_BIT_KHR = 4194304;
        #[doc(alias = "VK_IMAGE_USAGE_2_TENSOR_ALIASING_BIT_ARM")]
        const TENSOR_ALIASING_BIT_ARM = 8388608;
        #[doc(alias = "VK_IMAGE_USAGE_2_VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR")]
        const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR = 33554432;
        #[doc(alias = "VK_IMAGE_USAGE_2_VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR")]
        const VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR = 67108864;
        #[doc(alias = "VK_IMAGE_USAGE_2_TILE_MEMORY_BIT_QCOM")]
        const TILE_MEMORY_BIT_QCOM = 134217728;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCreateFlagBits2KHR.html>
    #[doc(alias = "VkImageCreateFlagBits2KHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ImageCreateFlags2KHR: u64 {
        #[doc(alias = "VK_IMAGE_CREATE_2_SPARSE_BINDING_BIT_KHR")]
        const SPARSE_BINDING_BIT_KHR = 1;
        #[doc(alias = "VK_IMAGE_CREATE_2_SPARSE_RESIDENCY_BIT_KHR")]
        const SPARSE_RESIDENCY_BIT_KHR = 2;
        #[doc(alias = "VK_IMAGE_CREATE_2_SPARSE_ALIASED_BIT_KHR")]
        const SPARSE_ALIASED_BIT_KHR = 4;
        #[doc(alias = "VK_IMAGE_CREATE_2_MUTABLE_FORMAT_BIT_KHR")]
        const MUTABLE_FORMAT_BIT_KHR = 8;
        #[doc(alias = "VK_IMAGE_CREATE_2_CUBE_COMPATIBLE_BIT_KHR")]
        const CUBE_COMPATIBLE_BIT_KHR = 16;
        #[doc(alias = "VK_IMAGE_CREATE_2_ALIAS_SINGLE_LAYER_DESCRIPTOR_BIT_KHR")]
        const ALIAS_SINGLE_LAYER_DESCRIPTOR_BIT_KHR = 4194304;
        #[doc(alias = "VK_IMAGE_CREATE_2_2D_ARRAY_COMPATIBLE_BIT_KHR")]
        const _2D_ARRAY_COMPATIBLE_BIT_KHR = 32;
        #[doc(alias = "VK_IMAGE_CREATE_2_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR")]
        const SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR = 64;
        #[doc(alias = "VK_IMAGE_CREATE_2_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR")]
        const BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR = 128;
        #[doc(alias = "VK_IMAGE_CREATE_2_EXTENDED_USAGE_BIT_KHR")]
        const EXTENDED_USAGE_BIT_KHR = 256;
        #[doc(alias = "VK_IMAGE_CREATE_2_DISJOINT_BIT_KHR")]
        const DISJOINT_BIT_KHR = 512;
        #[doc(alias = "VK_IMAGE_CREATE_2_ALIAS_BIT_KHR")]
        const ALIAS_BIT_KHR = 1024;
        #[doc(alias = "VK_IMAGE_CREATE_2_PROTECTED_BIT_KHR")]
        const PROTECTED_BIT_KHR = 2048;
        #[doc(alias = "VK_IMAGE_CREATE_2_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT")]
        const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT = 4096;
        #[doc(alias = "VK_IMAGE_CREATE_2_CORNER_SAMPLED_BIT_NV")]
        const CORNER_SAMPLED_BIT_NV = 8192;
        #[doc(alias = "VK_IMAGE_CREATE_2_SUBSAMPLED_BIT_EXT")]
        const SUBSAMPLED_BIT_EXT = 16384;
        #[doc(alias = "VK_IMAGE_CREATE_2_FRAGMENT_DENSITY_MAP_OFFSET_BIT_EXT")]
        const FRAGMENT_DENSITY_MAP_OFFSET_BIT_EXT = 32768;
        #[doc(alias = "VK_IMAGE_CREATE_2_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT")]
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT = 65536;
        #[doc(alias = "VK_IMAGE_CREATE_2_2D_VIEW_COMPATIBLE_BIT_EXT")]
        const _2D_VIEW_COMPATIBLE_BIT_EXT = 131072;
        #[doc(alias = "VK_IMAGE_CREATE_2_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT")]
        const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT = 262144;
        #[doc(alias = "VK_IMAGE_CREATE_2_VIDEO_PROFILE_INDEPENDENT_BIT_KHR")]
        const VIDEO_PROFILE_INDEPENDENT_BIT_KHR = 1048576;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugReportFlagBitsEXT.html>
    #[doc(alias = "VkDebugReportFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DebugReportFlagsEXT: u32 {
        #[doc(alias = "VK_DEBUG_REPORT_INFORMATION_BIT_EXT")]
        const INFORMATION_BIT_EXT = 1;
        #[doc(alias = "VK_DEBUG_REPORT_WARNING_BIT_EXT")]
        const WARNING_BIT_EXT = 2;
        #[doc(alias = "VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT")]
        const PERFORMANCE_WARNING_BIT_EXT = 4;
        #[doc(alias = "VK_DEBUG_REPORT_ERROR_BIT_EXT")]
        const ERROR_BIT_EXT = 8;
        #[doc(alias = "VK_DEBUG_REPORT_DEBUG_BIT_EXT")]
        const DEBUG_BIT_EXT = 16;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryHandleTypeFlagBitsNV.html>
    #[doc(alias = "VkExternalMemoryHandleTypeFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ExternalMemoryHandleTypeFlagsNV: u32 {
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV")]
        const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV = 1;
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV")]
        const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV = 2;
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV")]
        const EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV = 4;
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV")]
        const EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV = 8;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryFeatureFlagBitsNV.html>
    #[doc(alias = "VkExternalMemoryFeatureFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ExternalMemoryFeatureFlagsNV: u32 {
        #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV")]
        const EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV = 1;
        #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV")]
        const EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV = 2;
        #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV")]
        const EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV = 4;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCounterFlagBitsEXT.html>
    #[doc(alias = "VkSurfaceCounterFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SurfaceCounterFlagsEXT: u32 {
        #[doc(alias = "VK_SURFACE_COUNTER_VBLANK_BIT_EXT")]
        const VBLANK_BIT_EXT = 1;
    }
}
impl SurfaceCounterFlagsEXT {
    #[doc(alias = "VK_SURFACE_COUNTER_VBLANK_EXT")]
    pub const VBLANK_EXT: Self = Self::VBLANK_BIT_EXT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessageSeverityFlagBitsEXT.html>
    #[doc(alias = "VkDebugUtilsMessageSeverityFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DebugUtilsMessageSeverityFlagsEXT: u32 {
        #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT")]
        const VERBOSE_BIT_EXT = 1;
        #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT")]
        const INFO_BIT_EXT = 16;
        #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT")]
        const WARNING_BIT_EXT = 256;
        #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT")]
        const ERROR_BIT_EXT = 4096;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessageTypeFlagBitsEXT.html>
    #[doc(alias = "VkDebugUtilsMessageTypeFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DebugUtilsMessageTypeFlagsEXT: u32 {
        #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT")]
        const GENERAL_BIT_EXT = 1;
        #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT")]
        const VALIDATION_BIT_EXT = 2;
        #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT")]
        const PERFORMANCE_BIT_EXT = 4;
        #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_TYPE_DEVICE_ADDRESS_BINDING_BIT_EXT")]
        const DEVICE_ADDRESS_BINDING_BIT_EXT = 8;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaSqShaderStageFlagBitsAMD.html>
    #[doc(alias = "VkGpaSqShaderStageFlagBitsAMD")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct GpaSqShaderStageFlagsAMD: u32 {
        #[doc(alias = "VK_GPA_SQ_SHADER_STAGE_PS_BIT_AMD")]
        const PS_BIT_AMD = 1;
        #[doc(alias = "VK_GPA_SQ_SHADER_STAGE_VS_BIT_AMD")]
        const VS_BIT_AMD = 2;
        #[doc(alias = "VK_GPA_SQ_SHADER_STAGE_GS_BIT_AMD")]
        const GS_BIT_AMD = 4;
        #[doc(alias = "VK_GPA_SQ_SHADER_STAGE_ES_BIT_AMD")]
        const ES_BIT_AMD = 8;
        #[doc(alias = "VK_GPA_SQ_SHADER_STAGE_HS_BIT_AMD")]
        const HS_BIT_AMD = 16;
        #[doc(alias = "VK_GPA_SQ_SHADER_STAGE_LS_BIT_AMD")]
        const LS_BIT_AMD = 32;
        #[doc(alias = "VK_GPA_SQ_SHADER_STAGE_CS_BIT_AMD")]
        const CS_BIT_AMD = 64;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorViewCreateFlagBitsARM.html>
    #[doc(alias = "VkTensorViewCreateFlagBitsARM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct TensorViewCreateFlagsARM: u64 {
        #[doc(alias = "VK_TENSOR_VIEW_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_ARM")]
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_ARM = 1;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSpirvResourceTypeFlagBitsEXT.html>
    #[doc(alias = "VkSpirvResourceTypeFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SpirvResourceTypeFlagsEXT: u32 {
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_ALL_EXT")]
        const ALL_EXT = 2147483647;
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_SAMPLER_BIT_EXT")]
        const SAMPLER_BIT_EXT = 1;
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_SAMPLED_IMAGE_BIT_EXT")]
        const SAMPLED_IMAGE_BIT_EXT = 2;
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_READ_ONLY_IMAGE_BIT_EXT")]
        const READ_ONLY_IMAGE_BIT_EXT = 4;
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_READ_WRITE_IMAGE_BIT_EXT")]
        const READ_WRITE_IMAGE_BIT_EXT = 8;
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_COMBINED_SAMPLED_IMAGE_BIT_EXT")]
        const COMBINED_SAMPLED_IMAGE_BIT_EXT = 16;
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_UNIFORM_BUFFER_BIT_EXT")]
        const UNIFORM_BUFFER_BIT_EXT = 32;
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_READ_ONLY_STORAGE_BUFFER_BIT_EXT")]
        const READ_ONLY_STORAGE_BUFFER_BIT_EXT = 64;
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_READ_WRITE_STORAGE_BUFFER_BIT_EXT")]
        const READ_WRITE_STORAGE_BUFFER_BIT_EXT = 128;
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_ACCELERATION_STRUCTURE_BIT_EXT")]
        const ACCELERATION_STRUCTURE_BIT_EXT = 256;
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_TENSOR_BIT_ARM")]
        const TENSOR_BIT_ARM = 512;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryFlagBitsKHR.html>
    #[doc(alias = "VkGeometryFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct GeometryFlagsKHR: u32 {
        #[doc(alias = "VK_GEOMETRY_OPAQUE_BIT_KHR")]
        const OPAQUE_BIT_KHR = 1;
        #[doc(alias = "VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR")]
        const NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR = 2;
    }
}
pub type GeometryFlagsNV = GeometryFlagsKHR;
impl GeometryFlagsKHR {
    #[doc(alias = "VK_GEOMETRY_OPAQUE_BIT_NV")]
    pub const OPAQUE_BIT_NV: Self = Self::OPAQUE_BIT_KHR;
    #[doc(alias = "VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_NV")]
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_NV: Self =
        Self::NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryInstanceFlagBitsKHR.html>
    #[doc(alias = "VkGeometryInstanceFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct GeometryInstanceFlagsKHR: u32 {
        #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR")]
        const TRIANGLE_FACING_CULL_DISABLE_BIT_KHR = 1;
        #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR")]
        const TRIANGLE_FLIP_FACING_BIT_KHR = 2;
        #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR")]
        const FORCE_OPAQUE_BIT_KHR = 4;
        #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR")]
        const FORCE_NO_OPAQUE_BIT_KHR = 8;
        #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_OPACITY_MICROMAP_2_STATE_BIT_KHR")]
        const FORCE_OPACITY_MICROMAP_2_STATE_BIT_KHR = 16;
        #[doc(alias = "VK_GEOMETRY_INSTANCE_DISABLE_OPACITY_MICROMAPS_BIT_KHR")]
        const DISABLE_OPACITY_MICROMAPS_BIT_KHR = 32;
    }
}
pub type GeometryInstanceFlagsNV = GeometryInstanceFlagsKHR;
impl GeometryInstanceFlagsKHR {
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_CULL_DISABLE_BIT_NV")]
    pub const TRIANGLE_CULL_DISABLE_BIT_NV: Self = Self::TRIANGLE_FACING_CULL_DISABLE_BIT_KHR;
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR")]
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR: Self = Self::TRIANGLE_FLIP_FACING_BIT_KHR;
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_NV")]
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_NV: Self = Self::TRIANGLE_FLIP_FACING_BIT_KHR;
    #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_NV")]
    pub const FORCE_OPAQUE_BIT_NV: Self = Self::FORCE_OPAQUE_BIT_KHR;
    #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_NV")]
    pub const FORCE_NO_OPAQUE_BIT_NV: Self = Self::FORCE_NO_OPAQUE_BIT_KHR;
    #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_OPACITY_MICROMAP_2_STATE_BIT_EXT")]
    pub const FORCE_OPACITY_MICROMAP_2_STATE_BIT_EXT: Self =
        Self::FORCE_OPACITY_MICROMAP_2_STATE_BIT_KHR;
    #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_OPACITY_MICROMAP_2_STATE_EXT")]
    pub const FORCE_OPACITY_MICROMAP_2_STATE_EXT: Self =
        Self::FORCE_OPACITY_MICROMAP_2_STATE_BIT_KHR;
    #[doc(alias = "VK_GEOMETRY_INSTANCE_DISABLE_OPACITY_MICROMAPS_BIT_EXT")]
    pub const DISABLE_OPACITY_MICROMAPS_BIT_EXT: Self = Self::DISABLE_OPACITY_MICROMAPS_BIT_KHR;
    #[doc(alias = "VK_GEOMETRY_INSTANCE_DISABLE_OPACITY_MICROMAPS_EXT")]
    pub const DISABLE_OPACITY_MICROMAPS_EXT: Self = Self::DISABLE_OPACITY_MICROMAPS_BIT_KHR;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildAccelerationStructureFlagBitsKHR.html>
    #[doc(alias = "VkBuildAccelerationStructureFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct BuildAccelerationStructureFlagsKHR: u32 {
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR")]
        const ALLOW_UPDATE_BIT_KHR = 1;
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR")]
        const ALLOW_COMPACTION_BIT_KHR = 2;
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR")]
        const PREFER_FAST_TRACE_BIT_KHR = 4;
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR")]
        const PREFER_FAST_BUILD_BIT_KHR = 8;
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR")]
        const LOW_MEMORY_BIT_KHR = 16;
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV")]
        const MOTION_BIT_NV = 32;
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_OPACITY_MICROMAP_DATA_UPDATE_BIT_EXT")]
        const ALLOW_OPACITY_MICROMAP_DATA_UPDATE_BIT_EXT = 256;
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_DISPLACEMENT_MICROMAP_UPDATE_BIT_NV")]
        const ALLOW_DISPLACEMENT_MICROMAP_UPDATE_BIT_NV = 512;
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_DATA_ACCESS_BIT_KHR")]
        const ALLOW_DATA_ACCESS_BIT_KHR = 2048;
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_CLUSTER_OPACITY_MICROMAPS_BIT_NV")]
        const ALLOW_CLUSTER_OPACITY_MICROMAPS_BIT_NV = 4096;
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_OPACITY_MICROMAP_UPDATE_BIT_KHR")]
        const ALLOW_OPACITY_MICROMAP_UPDATE_BIT_KHR = 64;
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_DISABLE_OPACITY_MICROMAPS_BIT_KHR")]
        const ALLOW_DISABLE_OPACITY_MICROMAPS_BIT_KHR = 128;
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_MICROMAP_LOSSY_BIT_KHR")]
        const MICROMAP_LOSSY_BIT_KHR = 1024;
    }
}
pub type BuildAccelerationStructureFlagsNV = BuildAccelerationStructureFlagsKHR;
impl BuildAccelerationStructureFlagsKHR {
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_NV")]
    pub const ALLOW_UPDATE_BIT_NV: Self = Self::ALLOW_UPDATE_BIT_KHR;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_NV")]
    pub const ALLOW_COMPACTION_BIT_NV: Self = Self::ALLOW_COMPACTION_BIT_KHR;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_NV")]
    pub const PREFER_FAST_TRACE_BIT_NV: Self = Self::PREFER_FAST_TRACE_BIT_KHR;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_NV")]
    pub const PREFER_FAST_BUILD_BIT_NV: Self = Self::PREFER_FAST_BUILD_BIT_KHR;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_NV")]
    pub const LOW_MEMORY_BIT_NV: Self = Self::LOW_MEMORY_BIT_KHR;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT")]
    pub const ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT: Self =
        Self::ALLOW_OPACITY_MICROMAP_DATA_UPDATE_BIT_EXT;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV")]
    pub const ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV: Self =
        Self::ALLOW_DISPLACEMENT_MICROMAP_UPDATE_BIT_NV;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_DATA_ACCESS_KHR")]
    pub const ALLOW_DATA_ACCESS_KHR: Self = Self::ALLOW_DATA_ACCESS_BIT_KHR;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_OPACITY_MICROMAP_UPDATE_BIT_EXT")]
    pub const ALLOW_OPACITY_MICROMAP_UPDATE_BIT_EXT: Self =
        Self::ALLOW_OPACITY_MICROMAP_UPDATE_BIT_KHR;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_OPACITY_MICROMAP_UPDATE_EXT")]
    pub const ALLOW_OPACITY_MICROMAP_UPDATE_EXT: Self = Self::ALLOW_OPACITY_MICROMAP_UPDATE_BIT_KHR;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_DISABLE_OPACITY_MICROMAPS_BIT_EXT")]
    pub const ALLOW_DISABLE_OPACITY_MICROMAPS_BIT_EXT: Self =
        Self::ALLOW_DISABLE_OPACITY_MICROMAPS_BIT_KHR;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_DISABLE_OPACITY_MICROMAPS_EXT")]
    pub const ALLOW_DISABLE_OPACITY_MICROMAPS_EXT: Self =
        Self::ALLOW_DISABLE_OPACITY_MICROMAPS_BIT_KHR;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCompilerControlFlagBitsAMD.html>
    #[doc(alias = "VkPipelineCompilerControlFlagBitsAMD")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PipelineCompilerControlFlagsAMD: u32 {
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentStageFlagBitsEXT.html>
    #[doc(alias = "VkPresentStageFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PresentStageFlagsEXT: u32 {
        #[doc(alias = "VK_PRESENT_STAGE_QUEUE_OPERATIONS_END_BIT_EXT")]
        const QUEUE_OPERATIONS_END_BIT_EXT = 1;
        #[doc(alias = "VK_PRESENT_STAGE_REQUEST_DEQUEUED_BIT_EXT")]
        const REQUEST_DEQUEUED_BIT_EXT = 2;
        #[doc(alias = "VK_PRESENT_STAGE_IMAGE_FIRST_PIXEL_OUT_BIT_EXT")]
        const IMAGE_FIRST_PIXEL_OUT_BIT_EXT = 4;
        #[doc(alias = "VK_PRESENT_STAGE_IMAGE_FIRST_PIXEL_VISIBLE_BIT_EXT")]
        const IMAGE_FIRST_PIXEL_VISIBLE_BIT_EXT = 8;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPastPresentationTimingFlagBitsEXT.html>
    #[doc(alias = "VkPastPresentationTimingFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PastPresentationTimingFlagsEXT: u32 {
        #[doc(alias = "VK_PAST_PRESENTATION_TIMING_ALLOW_PARTIAL_RESULTS_BIT_EXT")]
        const ALLOW_PARTIAL_RESULTS_BIT_EXT = 1;
        #[doc(alias = "VK_PAST_PRESENTATION_TIMING_ALLOW_OUT_OF_ORDER_RESULTS_BIT_EXT")]
        const ALLOW_OUT_OF_ORDER_RESULTS_BIT_EXT = 2;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentTimingInfoFlagBitsEXT.html>
    #[doc(alias = "VkPresentTimingInfoFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PresentTimingInfoFlagsEXT: u32 {
        #[doc(alias = "VK_PRESENT_TIMING_INFO_PRESENT_AT_RELATIVE_TIME_BIT_EXT")]
        const PRESENT_AT_RELATIVE_TIME_BIT_EXT = 1;
        #[doc(alias = "VK_PRESENT_TIMING_INFO_PRESENT_AT_NEAREST_REFRESH_CYCLE_BIT_EXT")]
        const PRESENT_AT_NEAREST_REFRESH_CYCLE_BIT_EXT = 2;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderCorePropertiesFlagBitsAMD.html>
    #[doc(alias = "VkShaderCorePropertiesFlagBitsAMD")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ShaderCorePropertiesFlagsAMD: u32 {
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectStateFlagBitsNV.html>
    #[doc(alias = "VkIndirectStateFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct IndirectStateFlagsNV: u32 {
        #[doc(alias = "VK_INDIRECT_STATE_FLAG_FRONTFACE_BIT_NV")]
        const INDIRECT_STATE_FLAG_FRONTFACE_BIT_NV = 1;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutUsageFlagBitsNV.html>
    #[doc(alias = "VkIndirectCommandsLayoutUsageFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct IndirectCommandsLayoutUsageFlagsNV: u32 {
        #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV")]
        const INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV = 1;
        #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV")]
        const INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV = 2;
        #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV")]
        const INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV = 4;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceDiagnosticsConfigFlagBitsNV.html>
    #[doc(alias = "VkDeviceDiagnosticsConfigFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DeviceDiagnosticsConfigFlagsNV: u32 {
        #[doc(alias = "VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_DEBUG_INFO_BIT_NV")]
        const DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_DEBUG_INFO_BIT_NV = 1;
        #[doc(alias = "VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_RESOURCE_TRACKING_BIT_NV")]
        const DEVICE_DIAGNOSTICS_CONFIG_ENABLE_RESOURCE_TRACKING_BIT_NV = 2;
        #[doc(alias = "VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_AUTOMATIC_CHECKPOINTS_BIT_NV")]
        const DEVICE_DIAGNOSTICS_CONFIG_ENABLE_AUTOMATIC_CHECKPOINTS_BIT_NV = 4;
        #[doc(alias = "VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_ERROR_REPORTING_BIT_NV")]
        const DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_ERROR_REPORTING_BIT_NV = 8;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTileShadingRenderPassFlagBitsQCOM.html>
    #[doc(alias = "VkTileShadingRenderPassFlagBitsQCOM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct TileShadingRenderPassFlagsQCOM: u32 {
        #[doc(alias = "VK_TILE_SHADING_RENDER_PASS_ENABLE_BIT_QCOM")]
        const TILE_SHADING_RENDER_PASS_ENABLE_BIT_QCOM = 1;
        #[doc(alias = "VK_TILE_SHADING_RENDER_PASS_PER_TILE_EXECUTION_BIT_QCOM")]
        const TILE_SHADING_RENDER_PASS_PER_TILE_EXECUTION_BIT_QCOM = 2;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExportMetalObjectTypeFlagBitsEXT.html>
    #[doc(alias = "VkExportMetalObjectTypeFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ExportMetalObjectTypeFlagsEXT: u32 {
        #[doc(alias = "VK_EXPORT_METAL_OBJECT_TYPE_METAL_DEVICE_BIT_EXT")]
        const METAL_DEVICE_BIT_EXT = 1;
        #[doc(alias = "VK_EXPORT_METAL_OBJECT_TYPE_METAL_COMMAND_QUEUE_BIT_EXT")]
        const METAL_COMMAND_QUEUE_BIT_EXT = 2;
        #[doc(alias = "VK_EXPORT_METAL_OBJECT_TYPE_METAL_BUFFER_BIT_EXT")]
        const METAL_BUFFER_BIT_EXT = 4;
        #[doc(alias = "VK_EXPORT_METAL_OBJECT_TYPE_METAL_TEXTURE_BIT_EXT")]
        const METAL_TEXTURE_BIT_EXT = 8;
        #[doc(alias = "VK_EXPORT_METAL_OBJECT_TYPE_METAL_IOSURFACE_BIT_EXT")]
        const METAL_IOSURFACE_BIT_EXT = 16;
        #[doc(alias = "VK_EXPORT_METAL_OBJECT_TYPE_METAL_SHARED_EVENT_BIT_EXT")]
        const METAL_SHARED_EVENT_BIT_EXT = 32;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGraphicsPipelineLibraryFlagBitsEXT.html>
    #[doc(alias = "VkGraphicsPipelineLibraryFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct GraphicsPipelineLibraryFlagsEXT: u32 {
        #[doc(alias = "VK_GRAPHICS_PIPELINE_LIBRARY_VERTEX_INPUT_INTERFACE_BIT_EXT")]
        const VERTEX_INPUT_INTERFACE_BIT_EXT = 1;
        #[doc(alias = "VK_GRAPHICS_PIPELINE_LIBRARY_PRE_RASTERIZATION_SHADERS_BIT_EXT")]
        const PRE_RASTERIZATION_SHADERS_BIT_EXT = 2;
        #[doc(alias = "VK_GRAPHICS_PIPELINE_LIBRARY_FRAGMENT_SHADER_BIT_EXT")]
        const FRAGMENT_SHADER_BIT_EXT = 4;
        #[doc(alias = "VK_GRAPHICS_PIPELINE_LIBRARY_FRAGMENT_OUTPUT_INTERFACE_BIT_EXT")]
        const FRAGMENT_OUTPUT_INTERFACE_BIT_EXT = 8;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCompressionFlagBitsEXT.html>
    #[doc(alias = "VkImageCompressionFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ImageCompressionFlagsEXT: u32 {
        #[doc(alias = "VK_IMAGE_COMPRESSION_DEFAULT_EXT")]
        const DEFAULT_EXT = 0;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_DEFAULT_EXT")]
        const FIXED_RATE_DEFAULT_EXT = 1;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_EXPLICIT_EXT")]
        const FIXED_RATE_EXPLICIT_EXT = 2;
        #[doc(alias = "VK_IMAGE_COMPRESSION_DISABLED_EXT")]
        const DISABLED_EXT = 4;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCompressionFixedRateFlagBitsEXT.html>
    #[doc(alias = "VkImageCompressionFixedRateFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ImageCompressionFixedRateFlagsEXT: u32 {
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_NONE_EXT")]
        const NONE_EXT = 0;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_1BPC_BIT_EXT")]
        const _1BPC_BIT_EXT = 1;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_2BPC_BIT_EXT")]
        const _2BPC_BIT_EXT = 2;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_3BPC_BIT_EXT")]
        const _3BPC_BIT_EXT = 4;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_4BPC_BIT_EXT")]
        const _4BPC_BIT_EXT = 8;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_5BPC_BIT_EXT")]
        const _5BPC_BIT_EXT = 16;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_6BPC_BIT_EXT")]
        const _6BPC_BIT_EXT = 32;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_7BPC_BIT_EXT")]
        const _7BPC_BIT_EXT = 64;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_8BPC_BIT_EXT")]
        const _8BPC_BIT_EXT = 128;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_9BPC_BIT_EXT")]
        const _9BPC_BIT_EXT = 256;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_10BPC_BIT_EXT")]
        const _10BPC_BIT_EXT = 512;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_11BPC_BIT_EXT")]
        const _11BPC_BIT_EXT = 1024;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_12BPC_BIT_EXT")]
        const _12BPC_BIT_EXT = 2048;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_13BPC_BIT_EXT")]
        const _13BPC_BIT_EXT = 4096;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_14BPC_BIT_EXT")]
        const _14BPC_BIT_EXT = 8192;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_15BPC_BIT_EXT")]
        const _15BPC_BIT_EXT = 16384;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_16BPC_BIT_EXT")]
        const _16BPC_BIT_EXT = 32768;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_17BPC_BIT_EXT")]
        const _17BPC_BIT_EXT = 65536;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_18BPC_BIT_EXT")]
        const _18BPC_BIT_EXT = 131072;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_19BPC_BIT_EXT")]
        const _19BPC_BIT_EXT = 262144;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_20BPC_BIT_EXT")]
        const _20BPC_BIT_EXT = 524288;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_21BPC_BIT_EXT")]
        const _21BPC_BIT_EXT = 1048576;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_22BPC_BIT_EXT")]
        const _22BPC_BIT_EXT = 2097152;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_23BPC_BIT_EXT")]
        const _23BPC_BIT_EXT = 4194304;
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_24BPC_BIT_EXT")]
        const _24BPC_BIT_EXT = 8388608;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceAddressBindingFlagBitsEXT.html>
    #[doc(alias = "VkDeviceAddressBindingFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DeviceAddressBindingFlagsEXT: u32 {
        #[doc(alias = "VK_DEVICE_ADDRESS_BINDING_INTERNAL_OBJECT_BIT_EXT")]
        const INTERNAL_OBJECT_BIT_EXT = 1;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageConstraintsInfoFlagBitsFUCHSIA.html>
    #[doc(alias = "VkImageConstraintsInfoFlagBitsFUCHSIA")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ImageConstraintsInfoFlagsFUCHSIA: u32 {
        #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_RARELY_FUCHSIA")]
        const IMAGE_CONSTRAINTS_INFO_CPU_READ_RARELY_FUCHSIA = 1;
        #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_OFTEN_FUCHSIA")]
        const IMAGE_CONSTRAINTS_INFO_CPU_READ_OFTEN_FUCHSIA = 2;
        #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_RARELY_FUCHSIA")]
        const IMAGE_CONSTRAINTS_INFO_CPU_WRITE_RARELY_FUCHSIA = 4;
        #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_OFTEN_FUCHSIA")]
        const IMAGE_CONSTRAINTS_INFO_CPU_WRITE_OFTEN_FUCHSIA = 8;
        #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_PROTECTED_OPTIONAL_FUCHSIA")]
        const IMAGE_CONSTRAINTS_INFO_PROTECTED_OPTIONAL_FUCHSIA = 16;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFrameBoundaryFlagBitsEXT.html>
    #[doc(alias = "VkFrameBoundaryFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct FrameBoundaryFlagsEXT: u32 {
        #[doc(alias = "VK_FRAME_BOUNDARY_FRAME_END_BIT_EXT")]
        const FRAME_END_BIT_EXT = 1;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbModelConversionFlagBitsVALVE.html>
    #[doc(alias = "VkVideoEncodeRgbModelConversionFlagBitsVALVE")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeRgbModelConversionFlagsVALVE: u32 {
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_RGB_IDENTITY_BIT_VALVE")]
        const VIDEO_ENCODE_RGB_MODEL_CONVERSION_RGB_IDENTITY_BIT_VALVE = 1;
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_IDENTITY_BIT_VALVE")]
        const VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_IDENTITY_BIT_VALVE = 2;
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_709_BIT_VALVE")]
        const VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_709_BIT_VALVE = 4;
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_601_BIT_VALVE")]
        const VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_601_BIT_VALVE = 8;
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_2020_BIT_VALVE")]
        const VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_2020_BIT_VALVE = 16;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbRangeCompressionFlagBitsVALVE.html>
    #[doc(alias = "VkVideoEncodeRgbRangeCompressionFlagBitsVALVE")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeRgbRangeCompressionFlagsVALVE: u32 {
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_RANGE_COMPRESSION_FULL_RANGE_BIT_VALVE")]
        const VIDEO_ENCODE_RGB_RANGE_COMPRESSION_FULL_RANGE_BIT_VALVE = 1;
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_RANGE_COMPRESSION_NARROW_RANGE_BIT_VALVE")]
        const VIDEO_ENCODE_RGB_RANGE_COMPRESSION_NARROW_RANGE_BIT_VALVE = 2;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbChromaOffsetFlagBitsVALVE.html>
    #[doc(alias = "VkVideoEncodeRgbChromaOffsetFlagBitsVALVE")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeRgbChromaOffsetFlagsVALVE: u32 {
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_CHROMA_OFFSET_COSITED_EVEN_BIT_VALVE")]
        const VIDEO_ENCODE_RGB_CHROMA_OFFSET_COSITED_EVEN_BIT_VALVE = 1;
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_CHROMA_OFFSET_MIDPOINT_BIT_VALVE")]
        const VIDEO_ENCODE_RGB_CHROMA_OFFSET_MIDPOINT_BIT_VALVE = 2;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildMicromapFlagBitsEXT.html>
    #[doc(alias = "VkBuildMicromapFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct BuildMicromapFlagsEXT: u32 {
        #[doc(alias = "VK_BUILD_MICROMAP_PREFER_FAST_TRACE_BIT_EXT")]
        const PREFER_FAST_TRACE_BIT_EXT = 1;
        #[doc(alias = "VK_BUILD_MICROMAP_PREFER_FAST_BUILD_BIT_EXT")]
        const PREFER_FAST_BUILD_BIT_EXT = 2;
        #[doc(alias = "VK_BUILD_MICROMAP_ALLOW_COMPACTION_BIT_EXT")]
        const ALLOW_COMPACTION_BIT_EXT = 4;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapCreateFlagBitsEXT.html>
    #[doc(alias = "VkMicromapCreateFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct MicromapCreateFlagsEXT: u32 {
        #[doc(alias = "VK_MICROMAP_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT")]
        const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT = 1;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSchedulingControlsFlagBitsARM.html>
    #[doc(alias = "VkPhysicalDeviceSchedulingControlsFlagBitsARM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PhysicalDeviceSchedulingControlsFlagsARM: u64 {
        #[doc(alias = "VK_PHYSICAL_DEVICE_SCHEDULING_CONTROLS_SHADER_CORE_COUNT_ARM")]
        const SHADER_CORE_COUNT_ARM = 1;
        #[doc(alias = "VK_PHYSICAL_DEVICE_SCHEDULING_CONTROLS_DISPATCH_PARAMETERS_ARM")]
        const DISPATCH_PARAMETERS_ARM = 2;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryDecompressionMethodFlagBitsEXT.html>
    #[doc(alias = "VkMemoryDecompressionMethodFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct MemoryDecompressionMethodFlagsEXT: u64 {
        #[doc(alias = "VK_MEMORY_DECOMPRESSION_METHOD_GDEFLATE_1_0_BIT_EXT")]
        const GDEFLATE_1_0_BIT_EXT = 1;
    }
}
pub type MemoryDecompressionMethodFlagsNV = MemoryDecompressionMethodFlagsEXT;
impl MemoryDecompressionMethodFlagsEXT {
    #[doc(alias = "VK_MEMORY_DECOMPRESSION_METHOD_GDEFLATE_1_0_BIT_NV")]
    pub const GDEFLATE_1_0_BIT_NV: Self = Self::GDEFLATE_1_0_BIT_EXT;
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorCreateFlagBitsARM.html>
    #[doc(alias = "VkTensorCreateFlagBitsARM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct TensorCreateFlagsARM: u64 {
        #[doc(alias = "VK_TENSOR_CREATE_MUTABLE_FORMAT_BIT_ARM")]
        const MUTABLE_FORMAT_BIT_ARM = 1;
        #[doc(alias = "VK_TENSOR_CREATE_PROTECTED_BIT_ARM")]
        const PROTECTED_BIT_ARM = 2;
        #[doc(alias = "VK_TENSOR_CREATE_DESCRIPTOR_HEAP_CAPTURE_REPLAY_BIT_ARM")]
        const DESCRIPTOR_HEAP_CAPTURE_REPLAY_BIT_ARM = 8;
        #[doc(alias = "VK_TENSOR_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_ARM")]
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_ARM = 4;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorUsageFlagBitsARM.html>
    #[doc(alias = "VkTensorUsageFlagBitsARM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct TensorUsageFlagsARM: u64 {
        #[doc(alias = "VK_TENSOR_USAGE_SHADER_BIT_ARM")]
        const SHADER_BIT_ARM = 2;
        #[doc(alias = "VK_TENSOR_USAGE_TRANSFER_SRC_BIT_ARM")]
        const TRANSFER_SRC_BIT_ARM = 4;
        #[doc(alias = "VK_TENSOR_USAGE_TRANSFER_DST_BIT_ARM")]
        const TRANSFER_DST_BIT_ARM = 8;
        #[doc(alias = "VK_TENSOR_USAGE_IMAGE_ALIASING_BIT_ARM")]
        const IMAGE_ALIASING_BIT_ARM = 16;
        #[doc(alias = "VK_TENSOR_USAGE_DATA_GRAPH_BIT_ARM")]
        const DATA_GRAPH_BIT_ARM = 32;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowGridSizeFlagBitsNV.html>
    #[doc(alias = "VkOpticalFlowGridSizeFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct OpticalFlowGridSizeFlagsNV: u32 {
        #[doc(alias = "VK_OPTICAL_FLOW_GRID_SIZE_UNKNOWN_NV")]
        const OPTICAL_FLOW_GRID_SIZE_UNKNOWN_NV = 0;
        #[doc(alias = "VK_OPTICAL_FLOW_GRID_SIZE_1X1_BIT_NV")]
        const OPTICAL_FLOW_GRID_SIZE_1X1_BIT_NV = 1;
        #[doc(alias = "VK_OPTICAL_FLOW_GRID_SIZE_2X2_BIT_NV")]
        const OPTICAL_FLOW_GRID_SIZE_2X2_BIT_NV = 2;
        #[doc(alias = "VK_OPTICAL_FLOW_GRID_SIZE_4X4_BIT_NV")]
        const OPTICAL_FLOW_GRID_SIZE_4X4_BIT_NV = 4;
        #[doc(alias = "VK_OPTICAL_FLOW_GRID_SIZE_8X8_BIT_NV")]
        const OPTICAL_FLOW_GRID_SIZE_8X8_BIT_NV = 8;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowUsageFlagBitsNV.html>
    #[doc(alias = "VkOpticalFlowUsageFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct OpticalFlowUsageFlagsNV: u32 {
        #[doc(alias = "VK_OPTICAL_FLOW_USAGE_UNKNOWN_NV")]
        const OPTICAL_FLOW_USAGE_UNKNOWN_NV = 0;
        #[doc(alias = "VK_OPTICAL_FLOW_USAGE_INPUT_BIT_NV")]
        const OPTICAL_FLOW_USAGE_INPUT_BIT_NV = 1;
        #[doc(alias = "VK_OPTICAL_FLOW_USAGE_OUTPUT_BIT_NV")]
        const OPTICAL_FLOW_USAGE_OUTPUT_BIT_NV = 2;
        #[doc(alias = "VK_OPTICAL_FLOW_USAGE_HINT_BIT_NV")]
        const OPTICAL_FLOW_USAGE_HINT_BIT_NV = 4;
        #[doc(alias = "VK_OPTICAL_FLOW_USAGE_COST_BIT_NV")]
        const OPTICAL_FLOW_USAGE_COST_BIT_NV = 8;
        #[doc(alias = "VK_OPTICAL_FLOW_USAGE_GLOBAL_FLOW_BIT_NV")]
        const OPTICAL_FLOW_USAGE_GLOBAL_FLOW_BIT_NV = 16;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowSessionCreateFlagBitsNV.html>
    #[doc(alias = "VkOpticalFlowSessionCreateFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct OpticalFlowSessionCreateFlagsNV: u32 {
        #[doc(alias = "VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_HINT_BIT_NV")]
        const OPTICAL_FLOW_SESSION_CREATE_ENABLE_HINT_BIT_NV = 1;
        #[doc(alias = "VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_COST_BIT_NV")]
        const OPTICAL_FLOW_SESSION_CREATE_ENABLE_COST_BIT_NV = 2;
        #[doc(alias = "VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_GLOBAL_FLOW_BIT_NV")]
        const OPTICAL_FLOW_SESSION_CREATE_ENABLE_GLOBAL_FLOW_BIT_NV = 4;
        #[doc(alias = "VK_OPTICAL_FLOW_SESSION_CREATE_ALLOW_REGIONS_BIT_NV")]
        const OPTICAL_FLOW_SESSION_CREATE_ALLOW_REGIONS_BIT_NV = 8;
        #[doc(alias = "VK_OPTICAL_FLOW_SESSION_CREATE_BOTH_DIRECTIONS_BIT_NV")]
        const OPTICAL_FLOW_SESSION_CREATE_BOTH_DIRECTIONS_BIT_NV = 16;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowExecuteFlagBitsNV.html>
    #[doc(alias = "VkOpticalFlowExecuteFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct OpticalFlowExecuteFlagsNV: u32 {
        #[doc(alias = "VK_OPTICAL_FLOW_EXECUTE_DISABLE_TEMPORAL_HINTS_BIT_NV")]
        const OPTICAL_FLOW_EXECUTE_DISABLE_TEMPORAL_HINTS_BIT_NV = 1;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderCreateFlagBitsEXT.html>
    #[doc(alias = "VkShaderCreateFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ShaderCreateFlagsEXT: u32 {
        #[doc(alias = "VK_SHADER_CREATE_LINK_STAGE_BIT_EXT")]
        const LINK_STAGE_BIT_EXT = 1;
        #[doc(alias = "VK_SHADER_CREATE_DESCRIPTOR_HEAP_BIT_EXT")]
        const DESCRIPTOR_HEAP_BIT_EXT = 1024;
        #[doc(alias = "VK_SHADER_CREATE_INSTRUMENT_SHADER_BIT_ARM")]
        const INSTRUMENT_SHADER_BIT_ARM = 2048;
        #[doc(alias = "VK_SHADER_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT_EXT")]
        const ALLOW_VARYING_SUBGROUP_SIZE_BIT_EXT = 2;
        #[doc(alias = "VK_SHADER_CREATE_REQUIRE_FULL_SUBGROUPS_BIT_EXT")]
        const REQUIRE_FULL_SUBGROUPS_BIT_EXT = 4;
        #[doc(alias = "VK_SHADER_CREATE_NO_TASK_SHADER_BIT_EXT")]
        const NO_TASK_SHADER_BIT_EXT = 8;
        #[doc(alias = "VK_SHADER_CREATE_DISPATCH_BASE_BIT_EXT")]
        const DISPATCH_BASE_BIT_EXT = 16;
        #[doc(alias = "VK_SHADER_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_EXT")]
        const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_EXT = 32;
        #[doc(alias = "VK_SHADER_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT")]
        const FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT = 64;
        #[doc(alias = "VK_SHADER_CREATE_INDIRECT_BINDABLE_BIT_EXT")]
        const INDIRECT_BINDABLE_BIT_EXT = 128;
        #[doc(alias = "VK_SHADER_CREATE_OPACITY_MICROMAP_DISALLOW_MIXED_SPECIAL_INDEX_BIT_EXT")]
        const OPACITY_MICROMAP_DISALLOW_MIXED_SPECIAL_INDEX_BIT_EXT = 4096;
        #[doc(alias = "VK_SHADER_CREATE_64_BIT_INDEXING_BIT_EXT")]
        const _64_BIT_INDEXING_BIT_EXT = 32768;
        #[doc(alias = "VK_SHADER_CREATE_INDEPENDENT_SETS_BIT_KHR")]
        const INDEPENDENT_SETS_BIT_KHR = 262144;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionCreateFlagBitsARM.html>
    #[doc(alias = "VkDataGraphPipelineSessionCreateFlagBitsARM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DataGraphPipelineSessionCreateFlagsARM: u64 {
        #[doc(alias = "VK_DATA_GRAPH_PIPELINE_SESSION_CREATE_PROTECTED_BIT_ARM")]
        const PROTECTED_BIT_ARM = 1;
        #[doc(alias = "VK_DATA_GRAPH_PIPELINE_SESSION_CREATE_OPTICAL_FLOW_CACHE_BIT_ARM")]
        const OPTICAL_FLOW_CACHE_BIT_ARM = 2;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineDispatchFlagBitsARM.html>
    #[doc(alias = "VkDataGraphPipelineDispatchFlagBitsARM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DataGraphPipelineDispatchFlagsARM: u64 {
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphTOSAQualityFlagBitsARM.html>
    #[doc(alias = "VkDataGraphTOSAQualityFlagBitsARM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DataGraphTOSAQualityFlagsARM: u32 {
        #[doc(alias = "VK_DATA_GRAPH_TOSA_QUALITY_ACCELERATED_ARM")]
        const ACCELERATED_ARM = 1;
        #[doc(alias = "VK_DATA_GRAPH_TOSA_QUALITY_CONFORMANT_ARM")]
        const CONFORMANT_ARM = 2;
        #[doc(alias = "VK_DATA_GRAPH_TOSA_QUALITY_EXPERIMENTAL_ARM")]
        const EXPERIMENTAL_ARM = 4;
        #[doc(alias = "VK_DATA_GRAPH_TOSA_QUALITY_DEPRECATED_ARM")]
        const DEPRECATED_ARM = 8;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureAddressResolutionFlagBitsNV.html>
    #[doc(alias = "VkClusterAccelerationStructureAddressResolutionFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ClusterAccelerationStructureAddressResolutionFlagsNV: u32 {
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_NONE_NV")]
        const CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_NONE_NV = 0;
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_IMPLICIT_DATA_BIT_NV")]
        const CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_IMPLICIT_DATA_BIT_NV = 1;
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SCRATCH_DATA_BIT_NV")]
        const CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SCRATCH_DATA_BIT_NV = 2;
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_ADDRESS_ARRAY_BIT_NV")]
        const CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_ADDRESS_ARRAY_BIT_NV = 4;
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_SIZES_ARRAY_BIT_NV")]
        const CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_SIZES_ARRAY_BIT_NV = 8;
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SRC_INFOS_ARRAY_BIT_NV")]
        const CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SRC_INFOS_ARRAY_BIT_NV = 16;
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SRC_INFOS_COUNT_BIT_NV")]
        const CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SRC_INFOS_COUNT_BIT_NV = 32;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureClusterFlagBitsNV.html>
    #[doc(alias = "VkClusterAccelerationStructureClusterFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ClusterAccelerationStructureClusterFlagsNV: u32 {
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_CLUSTER_ALLOW_DISABLE_OPACITY_MICROMAPS_NV")]
        const CLUSTER_ACCELERATION_STRUCTURE_CLUSTER_ALLOW_DISABLE_OPACITY_MICROMAPS_NV = 1;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureGeometryFlagBitsNV.html>
    #[doc(alias = "VkClusterAccelerationStructureGeometryFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ClusterAccelerationStructureGeometryFlagsNV: u32 {
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_CULL_DISABLE_BIT_NV")]
        const CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_CULL_DISABLE_BIT_NV = 1;
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_NO_DUPLICATE_ANYHIT_INVOCATION_BIT_NV")]
        const CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_NO_DUPLICATE_ANYHIT_INVOCATION_BIT_NV = 2;
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_OPAQUE_BIT_NV")]
        const CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_OPAQUE_BIT_NV = 4;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureIndexFormatFlagBitsNV.html>
    #[doc(alias = "VkClusterAccelerationStructureIndexFormatFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ClusterAccelerationStructureIndexFormatFlagsNV: u32 {
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_8BIT_NV")]
        const CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_8BIT_NV = 1;
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_16BIT_NV")]
        const CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_16BIT_NV = 2;
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_32BIT_NV")]
        const CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_32BIT_NV = 4;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPartitionedAccelerationStructureInstanceFlagBitsNV.html>
    #[doc(alias = "VkPartitionedAccelerationStructureInstanceFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PartitionedAccelerationStructureInstanceFlagsNV: u32 {
        #[doc(alias = "VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_TRIANGLE_FACING_CULL_DISABLE_BIT_NV")]
        const PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_TRIANGLE_FACING_CULL_DISABLE_BIT_NV = 1;
        #[doc(alias = "VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_TRIANGLE_FLIP_FACING_BIT_NV")]
        const PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_TRIANGLE_FLIP_FACING_BIT_NV = 2;
        #[doc(alias = "VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_FORCE_OPAQUE_BIT_NV")]
        const PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_FORCE_OPAQUE_BIT_NV = 4;
        #[doc(alias = "VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_FORCE_NO_OPAQUE_BIT_NV")]
        const PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_FORCE_NO_OPAQUE_BIT_NV = 8;
        #[doc(alias = "VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV")]
        const PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV = 16;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsInputModeFlagBitsEXT.html>
    #[doc(alias = "VkIndirectCommandsInputModeFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct IndirectCommandsInputModeFlagsEXT: u32 {
        #[doc(alias = "VK_INDIRECT_COMMANDS_INPUT_MODE_VULKAN_INDEX_BUFFER_EXT")]
        const VULKAN_INDEX_BUFFER_EXT = 1;
        #[doc(alias = "VK_INDIRECT_COMMANDS_INPUT_MODE_DXGI_INDEX_BUFFER_EXT")]
        const DXGI_INDEX_BUFFER_EXT = 2;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutUsageFlagBitsEXT.html>
    #[doc(alias = "VkIndirectCommandsLayoutUsageFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct IndirectCommandsLayoutUsageFlagsEXT: u32 {
        #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_EXT")]
        const EXPLICIT_PREPROCESS_BIT_EXT = 1;
        #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_EXT")]
        const UNORDERED_SEQUENCES_BIT_EXT = 2;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowGridSizeFlagBitsARM.html>
    #[doc(alias = "VkDataGraphOpticalFlowGridSizeFlagBitsARM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DataGraphOpticalFlowGridSizeFlagsARM: u32 {
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_UNKNOWN_ARM")]
        const UNKNOWN_ARM = 0;
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_1X1_BIT_ARM")]
        const _1X1_BIT_ARM = 1;
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_2X2_BIT_ARM")]
        const _2X2_BIT_ARM = 2;
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_4X4_BIT_ARM")]
        const _4X4_BIT_ARM = 4;
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_8X8_BIT_ARM")]
        const _8X8_BIT_ARM = 8;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowCreateFlagBitsARM.html>
    #[doc(alias = "VkDataGraphOpticalFlowCreateFlagBitsARM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DataGraphOpticalFlowCreateFlagsARM: u32 {
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_CREATE_ENABLE_HINT_BIT_ARM")]
        const ENABLE_HINT_BIT_ARM = 1;
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_CREATE_ENABLE_COST_BIT_ARM")]
        const ENABLE_COST_BIT_ARM = 2;
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_CREATE_RESERVED_30_BIT_ARM")]
        const RESERVED_30_BIT_ARM = 1073741824;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowImageUsageFlagBitsARM.html>
    #[doc(alias = "VkDataGraphOpticalFlowImageUsageFlagBitsARM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DataGraphOpticalFlowImageUsageFlagsARM: u32 {
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_UNKNOWN_ARM")]
        const UNKNOWN_ARM = 0;
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_INPUT_BIT_ARM")]
        const INPUT_BIT_ARM = 1;
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_OUTPUT_BIT_ARM")]
        const OUTPUT_BIT_ARM = 2;
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_HINT_BIT_ARM")]
        const HINT_BIT_ARM = 4;
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_COST_BIT_ARM")]
        const COST_BIT_ARM = 8;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowExecuteFlagBitsARM.html>
    #[doc(alias = "VkDataGraphOpticalFlowExecuteFlagBitsARM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DataGraphOpticalFlowExecuteFlagsARM: u32 {
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_DISABLE_TEMPORAL_HINTS_BIT_ARM")]
        const DISABLE_TEMPORAL_HINTS_BIT_ARM = 1;
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_INPUT_UNCHANGED_BIT_ARM")]
        const INPUT_UNCHANGED_BIT_ARM = 2;
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_REFERENCE_UNCHANGED_BIT_ARM")]
        const REFERENCE_UNCHANGED_BIT_ARM = 4;
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_INPUT_IS_PREVIOUS_REFERENCE_BIT_ARM")]
        const INPUT_IS_PREVIOUS_REFERENCE_BIT_ARM = 8;
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_REFERENCE_IS_PREVIOUS_INPUT_BIT_ARM")]
        const REFERENCE_IS_PREVIOUS_INPUT_BIT_ARM = 16;
    }
}

bitflags::bitflags! {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCooperativeMatrixFlagBitsEXT.html>
    #[doc(alias = "VkCooperativeMatrixFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct CooperativeMatrixFlagsEXT: u32 {
        #[doc(alias = "VK_COOPERATIVE_MATRIX_SATURATING_ACCUMULATION_BIT_EXT")]
        const SATURATING_ACCUMULATION_BIT_EXT = 1;
    }
}
