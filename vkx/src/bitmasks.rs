// WARNING: AUTO GENERATED MODULE
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::loader::*;
use crate::platform::*;
use std::ffi::{c_char, c_int, c_uint, c_void};

use crate::enums::*;

bitflags::bitflags! {
    /// [`VkFormatFeatureFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlagBits.html)
    ///
    #[doc(alias = "VkFormatFeatureFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct FormatFeatureFlags: u32 {
        #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT")]
        const SAMPLED_IMAGE = 1;
        #[doc(alias = "VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT")]
        const STORAGE_IMAGE = 2;
        #[doc(alias = "VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT")]
        const STORAGE_IMAGE_ATOMIC = 4;
        #[doc(alias = "VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT")]
        const UNIFORM_TEXEL_BUFFER = 8;
        #[doc(alias = "VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT")]
        const STORAGE_TEXEL_BUFFER = 16;
        #[doc(alias = "VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT")]
        const STORAGE_TEXEL_BUFFER_ATOMIC = 32;
        #[doc(alias = "VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT")]
        const VERTEX_BUFFER = 64;
        #[doc(alias = "VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT")]
        const COLOR_ATTACHMENT = 128;
        #[doc(alias = "VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT")]
        const COLOR_ATTACHMENT_BLEND = 256;
        #[doc(alias = "VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT")]
        const DEPTH_STENCIL_ATTACHMENT = 512;
        #[doc(alias = "VK_FORMAT_FEATURE_BLIT_SRC_BIT")]
        const BLIT_SRC = 1024;
        #[doc(alias = "VK_FORMAT_FEATURE_BLIT_DST_BIT")]
        const BLIT_DST = 2048;
        #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT")]
        const SAMPLED_IMAGE_FILTER_LINEAR = 4096;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_Maintenance1`](Extensions::KHR_Maintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_TRANSFER_SRC_BIT")]
        const TRANSFER_SRC = 16384;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_Maintenance1`](Extensions::KHR_Maintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_TRANSFER_DST_BIT")]
        const TRANSFER_DST = 32768;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_SamplerYcbcrConversion`](Extensions::KHR_SamplerYcbcrConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT")]
        const MIDPOINT_CHROMA_SAMPLES = 131072;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_SamplerYcbcrConversion`](Extensions::KHR_SamplerYcbcrConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT")]
        const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER = 262144;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_SamplerYcbcrConversion`](Extensions::KHR_SamplerYcbcrConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT")]
        const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER = 524288;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_SamplerYcbcrConversion`](Extensions::KHR_SamplerYcbcrConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT")]
        const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT = 1048576;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_SamplerYcbcrConversion`](Extensions::KHR_SamplerYcbcrConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT")]
        const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE = 2097152;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_SamplerYcbcrConversion`](Extensions::KHR_SamplerYcbcrConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_DISJOINT_BIT")]
        const DISJOINT = 4194304;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_SamplerYcbcrConversion`](Extensions::KHR_SamplerYcbcrConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT")]
        const COSITED_CHROMA_SAMPLES = 8388608;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`EXT_SamplerFilterMinmax`](Extensions::EXT_SamplerFilterMinmax)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT")]
        const SAMPLED_IMAGE_FILTER_MINMAX = 65536;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extensions::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_VIDEO_DECODE_OUTPUT_BIT_KHR")]
        const VIDEO_DECODE_OUTPUT_KHR = 33554432;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extensions::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_VIDEO_DECODE_DPB_BIT_KHR")]
        const VIDEO_DECODE_DPB_KHR = 67108864;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_AccelerationStructure`](Extensions::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR")]
        const ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR = 536870912;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`IMG_FilterCubic`](Extensions::IMG_FilterCubic)
        /// - Extension [`EXT_FilterCubic`](Extensions::EXT_FilterCubic)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT")]
        const SAMPLED_IMAGE_FILTER_CUBIC_EXT = 8192;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_FragmentDensityMap`](Extensions::EXT_FragmentDensityMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_FRAGMENT_DENSITY_MAP_BIT_EXT")]
        const FRAGMENT_DENSITY_MAP_EXT = 16777216;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_FragmentShadingRate`](Extensions::KHR_FragmentShadingRate)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
        const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = 1073741824;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_VIDEO_ENCODE_INPUT_BIT_KHR")]
        const VIDEO_ENCODE_INPUT_KHR = 134217728;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_VIDEO_ENCODE_DPB_BIT_KHR")]
        const VIDEO_ENCODE_DPB_KHR = 268435456;
    }
}
impl FormatFeatureFlags {
    /// [`VK_FORMAT_FEATURE_TRANSFER_SRC_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_TRANSFER_SRC_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_TRANSFER_SRC_BIT_KHR")]
    pub const TRANSFER_SRC_KHR: Self = Self::TRANSFER_SRC;
    /// [`VK_FORMAT_FEATURE_TRANSFER_DST_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_TRANSFER_DST_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_TRANSFER_DST_BIT_KHR")]
    pub const TRANSFER_DST_KHR: Self = Self::TRANSFER_DST;
    /// [`VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT_KHR")]
    pub const MIDPOINT_CHROMA_SAMPLES_KHR: Self = Self::MIDPOINT_CHROMA_SAMPLES;
    /// [`VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR")]
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER;
    /// [`VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR.html)
    ///
    #[doc(
        alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR"
    )]
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER;
    /// [`VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR.html)
    ///
    #[doc(
        alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR"
    )]
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT;
    /// [`VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR.html)
    ///
    #[doc(
        alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR"
    )]
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE;
    /// [`VK_FORMAT_FEATURE_DISJOINT_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_DISJOINT_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_DISJOINT_BIT_KHR")]
    pub const DISJOINT_KHR: Self = Self::DISJOINT;
    /// [`VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT_KHR")]
    pub const COSITED_CHROMA_SAMPLES_KHR: Self = Self::COSITED_CHROMA_SAMPLES;
    /// [`VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT_EXT.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT_EXT")]
    pub const SAMPLED_IMAGE_FILTER_MINMAX_EXT: Self = Self::SAMPLED_IMAGE_FILTER_MINMAX;
    /// [`VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG")]
    pub const SAMPLED_IMAGE_FILTER_CUBIC_IMG: Self = Self::SAMPLED_IMAGE_FILTER_CUBIC_EXT;
}

bitflags::bitflags! {
    /// [`VkImageCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCreateFlagBits.html)
    ///
    #[doc(alias = "VkImageCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ImageCreateFlags: u32 {
        #[doc(alias = "VK_IMAGE_CREATE_SPARSE_BINDING_BIT")]
        const SPARSE_BINDING = 1;
        #[doc(alias = "VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT")]
        const SPARSE_RESIDENCY = 2;
        #[doc(alias = "VK_IMAGE_CREATE_SPARSE_ALIASED_BIT")]
        const SPARSE_ALIASED = 4;
        #[doc(alias = "VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT")]
        const MUTABLE_FORMAT = 8;
        #[doc(alias = "VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT")]
        const CUBE_COMPATIBLE = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_BindMemory2`](Extensions::KHR_BindMemory2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_ALIAS_BIT")]
        const ALIAS = 1024;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_DeviceGroup`](Extensions::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT")]
        const SPLIT_INSTANCE_BIND_REGIONS = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_Maintenance1`](Extensions::KHR_Maintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT")]
        const _2D_ARRAY_COMPATIBLE = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_Maintenance2`](Extensions::KHR_Maintenance2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT")]
        const BLOCK_TEXEL_VIEW_COMPATIBLE = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_Maintenance2`](Extensions::KHR_Maintenance2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_EXTENDED_USAGE_BIT")]
        const EXTENDED_USAGE = 256;
        #[doc(alias = "VK_IMAGE_CREATE_PROTECTED_BIT")]
        const PROTECTED = 2048;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_SamplerYcbcrConversion`](Extensions::KHR_SamplerYcbcrConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_DISJOINT_BIT")]
        const DISJOINT = 512;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_CornerSampledImage`](Extensions::NV_CornerSampledImage)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV")]
        const CORNER_SAMPLED_NV = 8192;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        /// - Extension [`EXT_DescriptorBuffer`](Extensions::EXT_DescriptorBuffer)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_DESCRIPTOR_HEAP_CAPTURE_REPLAY_BIT_EXT")]
        const DESCRIPTOR_HEAP_CAPTURE_REPLAY_EXT = 65536;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_SampleLocations`](Extensions::EXT_SampleLocations)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT")]
        const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT = 4096;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_FragmentDensityMap`](Extensions::EXT_FragmentDensityMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT")]
        const SUBSAMPLED_EXT = 16384;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MultisampledRenderToSingleSampled`](Extensions::EXT_MultisampledRenderToSingleSampled)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT")]
        const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT = 262144;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_Image2DViewOf3D`](Extensions::EXT_Image2DViewOf3D)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2D_VIEW_COMPATIBLE_BIT_EXT")]
        const _2D_VIEW_COMPATIBLE_EXT = 131072;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoMaintenance1`](Extensions::KHR_VideoMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_VIDEO_PROFILE_INDEPENDENT_BIT_KHR")]
        const VIDEO_PROFILE_INDEPENDENT_KHR = 1048576;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_FragmentDensityMapOffset`](Extensions::QCOM_FragmentDensityMapOffset)
        /// - Extension [`EXT_FragmentDensityMapOffset`](Extensions::EXT_FragmentDensityMapOffset)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_EXT")]
        const FRAGMENT_DENSITY_MAP_OFFSET_EXT = 32768;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance11`](Extensions::KHR_Maintenance11)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_ALIAS_SINGLE_LAYER_DESCRIPTOR_BIT_KHR")]
        const ALIAS_SINGLE_LAYER_DESCRIPTOR_KHR = 4194304;
    }
}
impl ImageCreateFlags {
    /// [`VK_IMAGE_CREATE_ALIAS_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMAGE_CREATE_ALIAS_BIT_KHR.html)
    ///
    #[doc(alias = "VK_IMAGE_CREATE_ALIAS_BIT_KHR")]
    pub const ALIAS_KHR: Self = Self::ALIAS;
    /// [`VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR.html)
    ///
    #[doc(alias = "VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR")]
    pub const SPLIT_INSTANCE_BIND_REGIONS_KHR: Self = Self::SPLIT_INSTANCE_BIND_REGIONS;
    /// [`VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT_KHR")]
    pub const _2D_ARRAY_COMPATIBLE_KHR: Self = Self::_2D_ARRAY_COMPATIBLE;
    /// [`VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR")]
    pub const BLOCK_TEXEL_VIEW_COMPATIBLE_KHR: Self = Self::BLOCK_TEXEL_VIEW_COMPATIBLE;
    /// [`VK_IMAGE_CREATE_EXTENDED_USAGE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMAGE_CREATE_EXTENDED_USAGE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_IMAGE_CREATE_EXTENDED_USAGE_BIT_KHR")]
    pub const EXTENDED_USAGE_KHR: Self = Self::EXTENDED_USAGE;
    /// [`VK_IMAGE_CREATE_DISJOINT_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMAGE_CREATE_DISJOINT_BIT_KHR.html)
    ///
    #[doc(alias = "VK_IMAGE_CREATE_DISJOINT_BIT_KHR")]
    pub const DISJOINT_KHR: Self = Self::DISJOINT;
    /// [`VK_IMAGE_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMAGE_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT.html)
    ///
    #[doc(alias = "VK_IMAGE_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT")]
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self::DESCRIPTOR_HEAP_CAPTURE_REPLAY_EXT;
    /// [`VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM.html)
    ///
    #[doc(alias = "VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM")]
    pub const FRAGMENT_DENSITY_MAP_OFFSET_QCOM: Self = Self::FRAGMENT_DENSITY_MAP_OFFSET_EXT;
}

bitflags::bitflags! {
    /// [`VkSampleCountFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSampleCountFlagBits.html)
    ///
    #[doc(alias = "VkSampleCountFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SampleCountFlags: u32 {
        #[doc(alias = "VK_SAMPLE_COUNT_1_BIT")]
        const _1 = 1;
        #[doc(alias = "VK_SAMPLE_COUNT_2_BIT")]
        const _2 = 2;
        #[doc(alias = "VK_SAMPLE_COUNT_4_BIT")]
        const _4 = 4;
        #[doc(alias = "VK_SAMPLE_COUNT_8_BIT")]
        const _8 = 8;
        #[doc(alias = "VK_SAMPLE_COUNT_16_BIT")]
        const _16 = 16;
        #[doc(alias = "VK_SAMPLE_COUNT_32_BIT")]
        const _32 = 32;
        #[doc(alias = "VK_SAMPLE_COUNT_64_BIT")]
        const _64 = 64;
    }
}

bitflags::bitflags! {
    /// [`VkImageUsageFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageUsageFlagBits.html)
    ///
    #[doc(alias = "VkImageUsageFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ImageUsageFlags: u32 {
        #[doc(alias = "VK_IMAGE_USAGE_TRANSFER_SRC_BIT")]
        const TRANSFER_SRC = 1;
        #[doc(alias = "VK_IMAGE_USAGE_TRANSFER_DST_BIT")]
        const TRANSFER_DST = 2;
        #[doc(alias = "VK_IMAGE_USAGE_SAMPLED_BIT")]
        const SAMPLED = 4;
        #[doc(alias = "VK_IMAGE_USAGE_STORAGE_BIT")]
        const STORAGE = 8;
        #[doc(alias = "VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT")]
        const COLOR_ATTACHMENT = 16;
        #[doc(alias = "VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT")]
        const DEPTH_STENCIL_ATTACHMENT = 32;
        #[doc(alias = "VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT")]
        const TRANSIENT_ATTACHMENT = 64;
        #[doc(alias = "VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT")]
        const INPUT_ATTACHMENT = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`EXT_HostImageCopy`](Extensions::EXT_HostImageCopy)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_HOST_TRANSFER_BIT")]
        const HOST_TRANSFER = 4194304;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extensions::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR")]
        const VIDEO_DECODE_DST_KHR = 1024;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extensions::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_VIDEO_DECODE_SRC_BIT_KHR")]
        const VIDEO_DECODE_SRC_KHR = 2048;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extensions::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR")]
        const VIDEO_DECODE_DPB_KHR = 4096;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_FragmentDensityMap`](Extensions::EXT_FragmentDensityMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_FRAGMENT_DENSITY_MAP_BIT_EXT")]
        const FRAGMENT_DENSITY_MAP_EXT = 512;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_FragmentShadingRate`](Extensions::KHR_FragmentShadingRate)
        /// - Extension [`NV_ShadingRateImage`](Extensions::NV_ShadingRateImage)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
        const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = 256;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_VIDEO_ENCODE_DST_BIT_KHR")]
        const VIDEO_ENCODE_DST_KHR = 8192;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_VIDEO_ENCODE_SRC_BIT_KHR")]
        const VIDEO_ENCODE_SRC_KHR = 16384;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_VIDEO_ENCODE_DPB_BIT_KHR")]
        const VIDEO_ENCODE_DPB_KHR = 32768;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_AttachmentFeedbackLoopLayout`](Extensions::EXT_AttachmentFeedbackLoopLayout)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT")]
        const ATTACHMENT_FEEDBACK_LOOP_EXT = 524288;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`HUAWEI_InvocationMask`](Extensions::HUAWEI_InvocationMask)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_INVOCATION_MASK_BIT_HUAWEI")]
        const INVOCATION_MASK_HUAWEI = 262144;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_ImageProcessing`](Extensions::QCOM_ImageProcessing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_SAMPLE_WEIGHT_BIT_QCOM")]
        const SAMPLE_WEIGHT_QCOM = 1048576;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_ImageProcessing`](Extensions::QCOM_ImageProcessing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_SAMPLE_BLOCK_MATCH_BIT_QCOM")]
        const SAMPLE_BLOCK_MATCH_QCOM = 2097152;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_Tensors`](Extensions::ARM_Tensors)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_TENSOR_ALIASING_BIT_ARM")]
        const TENSOR_ALIASING_ARM = 8388608;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_TileMemoryHeap`](Extensions::QCOM_TileMemoryHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_TILE_MEMORY_BIT_QCOM")]
        const TILE_MEMORY_QCOM = 134217728;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extensions::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR")]
        const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR = 33554432;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extensions::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR")]
        const VIDEO_ENCODE_EMPHASIS_MAP_KHR = 67108864;
    }
}
impl ImageUsageFlags {
    /// [`VK_IMAGE_USAGE_HOST_TRANSFER_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMAGE_USAGE_HOST_TRANSFER_BIT_EXT.html)
    ///
    #[doc(alias = "VK_IMAGE_USAGE_HOST_TRANSFER_BIT_EXT")]
    pub const HOST_TRANSFER_EXT: Self = Self::HOST_TRANSFER;
    /// [`VK_IMAGE_USAGE_SHADING_RATE_IMAGE_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMAGE_USAGE_SHADING_RATE_IMAGE_BIT_NV.html)
    ///
    #[doc(alias = "VK_IMAGE_USAGE_SHADING_RATE_IMAGE_BIT_NV")]
    pub const SHADING_RATE_IMAGE_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
}

bitflags::bitflags! {
    /// [`VkInstanceCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkInstanceCreateFlagBits.html)
    ///
    #[doc(alias = "VkInstanceCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct InstanceCreateFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_PortabilityEnumeration`](Extensions::KHR_PortabilityEnumeration)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR")]
        const ENUMERATE_PORTABILITY_KHR = 1;
    }
}

bitflags::bitflags! {
    /// [`VkMemoryHeapFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryHeapFlagBits.html)
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkMemoryHeapFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct MemoryHeapFlags: u32 {
        #[doc(alias = "VK_MEMORY_HEAP_DEVICE_LOCAL_BIT")]
        const DEVICE_LOCAL = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_DeviceGroupCreation`](Extensions::KHR_DeviceGroupCreation)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MEMORY_HEAP_MULTI_INSTANCE_BIT")]
        const MULTI_INSTANCE = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_TileMemoryHeap`](Extensions::QCOM_TileMemoryHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MEMORY_HEAP_TILE_MEMORY_BIT_QCOM")]
        const TILE_MEMORY_QCOM = 8;
    }
}
impl MemoryHeapFlags {
    /// [`VK_MEMORY_HEAP_MULTI_INSTANCE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MEMORY_HEAP_MULTI_INSTANCE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_MEMORY_HEAP_MULTI_INSTANCE_BIT_KHR")]
    pub const MULTI_INSTANCE_KHR: Self = Self::MULTI_INSTANCE;
}

bitflags::bitflags! {
    /// [`VkMemoryPropertyFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryPropertyFlagBits.html)
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkMemoryPropertyFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct MemoryPropertyFlags: u32 {
        #[doc(alias = "VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT")]
        const DEVICE_LOCAL = 1;
        #[doc(alias = "VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT")]
        const HOST_VISIBLE = 2;
        #[doc(alias = "VK_MEMORY_PROPERTY_HOST_COHERENT_BIT")]
        const HOST_COHERENT = 4;
        #[doc(alias = "VK_MEMORY_PROPERTY_HOST_CACHED_BIT")]
        const HOST_CACHED = 8;
        #[doc(alias = "VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT")]
        const LAZILY_ALLOCATED = 16;
        #[doc(alias = "VK_MEMORY_PROPERTY_PROTECTED_BIT")]
        const PROTECTED = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMD_DeviceCoherentMemory`](Extensions::AMD_DeviceCoherentMemory)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD")]
        const DEVICE_COHERENT_AMD = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMD_DeviceCoherentMemory`](Extensions::AMD_DeviceCoherentMemory)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MEMORY_PROPERTY_DEVICE_UNCACHED_BIT_AMD")]
        const DEVICE_UNCACHED_AMD = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ExternalMemoryRdma`](Extensions::NV_ExternalMemoryRdma)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MEMORY_PROPERTY_RDMA_CAPABLE_BIT_NV")]
        const RDMA_CAPABLE_NV = 256;
    }
}

bitflags::bitflags! {
    /// [`VkQueueFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFlagBits.html)
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkQueueFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct QueueFlags: u32 {
        #[doc(alias = "VK_QUEUE_GRAPHICS_BIT")]
        const GRAPHICS = 1;
        #[doc(alias = "VK_QUEUE_COMPUTE_BIT")]
        const COMPUTE = 2;
        #[doc(alias = "VK_QUEUE_TRANSFER_BIT")]
        const TRANSFER = 4;
        #[doc(alias = "VK_QUEUE_SPARSE_BINDING_BIT")]
        const SPARSE_BINDING = 8;
        #[doc(alias = "VK_QUEUE_PROTECTED_BIT")]
        const PROTECTED = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extensions::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_QUEUE_VIDEO_DECODE_BIT_KHR")]
        const VIDEO_DECODE_KHR = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_QUEUE_VIDEO_ENCODE_BIT_KHR")]
        const VIDEO_ENCODE_KHR = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_QUEUE_OPTICAL_FLOW_BIT_NV")]
        const OPTICAL_FLOW_NV = 256;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraph`](Extensions::ARM_DataGraph)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_QUEUE_DATA_GRAPH_BIT_ARM")]
        const DATA_GRAPH_ARM = 1024;
    }
}

bitflags::bitflags! {
    /// [`VkShaderStageFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderStageFlagBits.html)
    ///
    #[doc(alias = "VkShaderStageFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ShaderStageFlags: u32 {
        #[doc(alias = "VK_SHADER_STAGE_VERTEX_BIT")]
        const VERTEX = 1;
        #[doc(alias = "VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT")]
        const TESSELLATION_CONTROL = 2;
        #[doc(alias = "VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT")]
        const TESSELLATION_EVALUATION = 4;
        #[doc(alias = "VK_SHADER_STAGE_GEOMETRY_BIT")]
        const GEOMETRY = 8;
        #[doc(alias = "VK_SHADER_STAGE_FRAGMENT_BIT")]
        const FRAGMENT = 16;
        #[doc(alias = "VK_SHADER_STAGE_COMPUTE_BIT")]
        const COMPUTE = 32;
        #[doc(alias = "VK_SHADER_STAGE_ALL_GRAPHICS")]
        const ALL_GRAPHICS = 31;
        #[doc(alias = "VK_SHADER_STAGE_ALL")]
        const ALL = 2147483647;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_STAGE_RAYGEN_BIT_KHR")]
        const RAYGEN_KHR = 256;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_STAGE_ANY_HIT_BIT_KHR")]
        const ANY_HIT_KHR = 512;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR")]
        const CLOSEST_HIT_KHR = 1024;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_STAGE_MISS_BIT_KHR")]
        const MISS_KHR = 2048;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_STAGE_INTERSECTION_BIT_KHR")]
        const INTERSECTION_KHR = 4096;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_STAGE_CALLABLE_BIT_KHR")]
        const CALLABLE_KHR = 8192;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_MeshShader`](Extensions::NV_MeshShader)
        /// - Extension [`EXT_MeshShader`](Extensions::EXT_MeshShader)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_STAGE_TASK_BIT_EXT")]
        const TASK_EXT = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_MeshShader`](Extensions::NV_MeshShader)
        /// - Extension [`EXT_MeshShader`](Extensions::EXT_MeshShader)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_STAGE_MESH_BIT_EXT")]
        const MESH_EXT = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`HUAWEI_SubpassShading`](Extensions::HUAWEI_SubpassShading)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_STAGE_SUBPASS_SHADING_BIT_HUAWEI")]
        const SUBPASS_SHADING_HUAWEI = 16384;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`HUAWEI_ClusterCullingShader`](Extensions::HUAWEI_ClusterCullingShader)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_STAGE_CLUSTER_CULLING_BIT_HUAWEI")]
        const CLUSTER_CULLING_HUAWEI = 524288;
    }
}
impl ShaderStageFlags {
    /// [`VK_SHADER_STAGE_RAYGEN_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SHADER_STAGE_RAYGEN_BIT_NV.html)
    ///
    #[doc(alias = "VK_SHADER_STAGE_RAYGEN_BIT_NV")]
    pub const RAYGEN_NV: Self = Self::RAYGEN_KHR;
    /// [`VK_SHADER_STAGE_ANY_HIT_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SHADER_STAGE_ANY_HIT_BIT_NV.html)
    ///
    #[doc(alias = "VK_SHADER_STAGE_ANY_HIT_BIT_NV")]
    pub const ANY_HIT_NV: Self = Self::ANY_HIT_KHR;
    /// [`VK_SHADER_STAGE_CLOSEST_HIT_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SHADER_STAGE_CLOSEST_HIT_BIT_NV.html)
    ///
    #[doc(alias = "VK_SHADER_STAGE_CLOSEST_HIT_BIT_NV")]
    pub const CLOSEST_HIT_NV: Self = Self::CLOSEST_HIT_KHR;
    /// [`VK_SHADER_STAGE_MISS_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SHADER_STAGE_MISS_BIT_NV.html)
    ///
    #[doc(alias = "VK_SHADER_STAGE_MISS_BIT_NV")]
    pub const MISS_NV: Self = Self::MISS_KHR;
    /// [`VK_SHADER_STAGE_INTERSECTION_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SHADER_STAGE_INTERSECTION_BIT_NV.html)
    ///
    #[doc(alias = "VK_SHADER_STAGE_INTERSECTION_BIT_NV")]
    pub const INTERSECTION_NV: Self = Self::INTERSECTION_KHR;
    /// [`VK_SHADER_STAGE_CALLABLE_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SHADER_STAGE_CALLABLE_BIT_NV.html)
    ///
    #[doc(alias = "VK_SHADER_STAGE_CALLABLE_BIT_NV")]
    pub const CALLABLE_NV: Self = Self::CALLABLE_KHR;
    /// [`VK_SHADER_STAGE_TASK_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SHADER_STAGE_TASK_BIT_NV.html)
    ///
    #[doc(alias = "VK_SHADER_STAGE_TASK_BIT_NV")]
    pub const TASK_NV: Self = Self::TASK_EXT;
    /// [`VK_SHADER_STAGE_MESH_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SHADER_STAGE_MESH_BIT_NV.html)
    ///
    #[doc(alias = "VK_SHADER_STAGE_MESH_BIT_NV")]
    pub const MESH_NV: Self = Self::MESH_EXT;
}

bitflags::bitflags! {
    /// [`VkDeviceQueueCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceQueueCreateFlagBits.html)
    ///
    #[doc(alias = "VkDeviceQueueCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DeviceQueueCreateFlags: u32 {
        #[doc(alias = "VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT")]
        const PROTECTED = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_InternallySynchronizedQueues`](Extensions::KHR_InternallySynchronizedQueues)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_QUEUE_CREATE_INTERNALLY_SYNCHRONIZED_BIT_KHR")]
        const INTERNALLY_SYNCHRONIZED_KHR = 4;
    }
}

bitflags::bitflags! {
    /// [`VkPipelineStageFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineStageFlagBits.html)
    ///
    #[doc(alias = "VkPipelineStageFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PipelineStageFlags: u32 {
        #[doc(alias = "VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT")]
        const TOP_OF_PIPE = 1;
        #[doc(alias = "VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT")]
        const DRAW_INDIRECT = 2;
        #[doc(alias = "VK_PIPELINE_STAGE_VERTEX_INPUT_BIT")]
        const VERTEX_INPUT = 4;
        #[doc(alias = "VK_PIPELINE_STAGE_VERTEX_SHADER_BIT")]
        const VERTEX_SHADER = 8;
        #[doc(alias = "VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT")]
        const TESSELLATION_CONTROL_SHADER = 16;
        #[doc(alias = "VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT")]
        const TESSELLATION_EVALUATION_SHADER = 32;
        #[doc(alias = "VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT")]
        const GEOMETRY_SHADER = 64;
        #[doc(alias = "VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT")]
        const FRAGMENT_SHADER = 128;
        #[doc(alias = "VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT")]
        const EARLY_FRAGMENT_TESTS = 256;
        #[doc(alias = "VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT")]
        const LATE_FRAGMENT_TESTS = 512;
        #[doc(alias = "VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT")]
        const COLOR_ATTACHMENT_OUTPUT = 1024;
        #[doc(alias = "VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT")]
        const COMPUTE_SHADER = 2048;
        #[doc(alias = "VK_PIPELINE_STAGE_TRANSFER_BIT")]
        const TRANSFER = 4096;
        #[doc(alias = "VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT")]
        const BOTTOM_OF_PIPE = 8192;
        #[doc(alias = "VK_PIPELINE_STAGE_HOST_BIT")]
        const HOST = 16384;
        #[doc(alias = "VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT")]
        const ALL_GRAPHICS = 32768;
        #[doc(alias = "VK_PIPELINE_STAGE_ALL_COMMANDS_BIT")]
        const ALL_COMMANDS = 65536;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_NONE")]
        const NONE = 0;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_TransformFeedback`](Extensions::EXT_TransformFeedback)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT")]
        const TRANSFORM_FEEDBACK_EXT = 16777216;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ConditionalRendering`](Extensions::EXT_ConditionalRendering)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_CONDITIONAL_RENDERING_BIT_EXT")]
        const CONDITIONAL_RENDERING_EXT = 262144;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extensions::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR")]
        const ACCELERATION_STRUCTURE_BUILD_KHR = 33554432;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_KHR")]
        const RAY_TRACING_SHADER_KHR = 2097152;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_FragmentDensityMap`](Extensions::EXT_FragmentDensityMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT")]
        const FRAGMENT_DENSITY_PROCESS_EXT = 8388608;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_FragmentShadingRate`](Extensions::KHR_FragmentShadingRate)
        /// - Extension [`NV_ShadingRateImage`](Extensions::NV_ShadingRateImage)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
        const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = 4194304;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_MeshShader`](Extensions::NV_MeshShader)
        /// - Extension [`EXT_MeshShader`](Extensions::EXT_MeshShader)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_TASK_SHADER_BIT_EXT")]
        const TASK_SHADER_EXT = 524288;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_MeshShader`](Extensions::NV_MeshShader)
        /// - Extension [`EXT_MeshShader`](Extensions::EXT_MeshShader)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_MESH_SHADER_BIT_EXT")]
        const MESH_SHADER_EXT = 1048576;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceGeneratedCommands`](Extensions::NV_DeviceGeneratedCommands)
        /// - Extension [`EXT_DeviceGeneratedCommands`](Extensions::EXT_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_COMMAND_PREPROCESS_BIT_EXT")]
        const COMMAND_PREPROCESS_EXT = 131072;
    }
}
impl PipelineStageFlags {
    /// [`VK_PIPELINE_STAGE_NONE_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_NONE_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_NONE_KHR")]
    pub const NONE_KHR: Self = Self::NONE;
    /// [`VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_NV.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_NV")]
    pub const ACCELERATION_STRUCTURE_BUILD_NV: Self = Self::ACCELERATION_STRUCTURE_BUILD_KHR;
    /// [`VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_NV.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_NV")]
    pub const RAY_TRACING_SHADER_NV: Self = Self::RAY_TRACING_SHADER_KHR;
    /// [`VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV")]
    pub const SHADING_RATE_IMAGE_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
    /// [`VK_PIPELINE_STAGE_TASK_SHADER_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_TASK_SHADER_BIT_NV.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_TASK_SHADER_BIT_NV")]
    pub const TASK_SHADER_NV: Self = Self::TASK_SHADER_EXT;
    /// [`VK_PIPELINE_STAGE_MESH_SHADER_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_MESH_SHADER_BIT_NV.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_MESH_SHADER_BIT_NV")]
    pub const MESH_SHADER_NV: Self = Self::MESH_SHADER_EXT;
    /// [`VK_PIPELINE_STAGE_COMMAND_PREPROCESS_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_COMMAND_PREPROCESS_BIT_NV.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_COMMAND_PREPROCESS_BIT_NV")]
    pub const COMMAND_PREPROCESS_NV: Self = Self::COMMAND_PREPROCESS_EXT;
}

bitflags::bitflags! {
    /// [`VkMemoryMapFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryMapFlagBits.html)
    ///
    #[doc(alias = "VkMemoryMapFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct MemoryMapFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MapMemoryPlaced`](Extensions::EXT_MapMemoryPlaced)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MEMORY_MAP_PLACED_BIT_EXT")]
        const PLACED_EXT = 1;
    }
}

bitflags::bitflags! {
    /// [`VkImageAspectFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageAspectFlagBits.html)
    ///
    #[doc(alias = "VkImageAspectFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ImageAspectFlags: u32 {
        #[doc(alias = "VK_IMAGE_ASPECT_COLOR_BIT")]
        const COLOR = 1;
        #[doc(alias = "VK_IMAGE_ASPECT_DEPTH_BIT")]
        const DEPTH = 2;
        #[doc(alias = "VK_IMAGE_ASPECT_STENCIL_BIT")]
        const STENCIL = 4;
        #[doc(alias = "VK_IMAGE_ASPECT_METADATA_BIT")]
        const METADATA = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_SamplerYcbcrConversion`](Extensions::KHR_SamplerYcbcrConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_ASPECT_PLANE_0_BIT")]
        const PLANE_0 = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_SamplerYcbcrConversion`](Extensions::KHR_SamplerYcbcrConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_ASPECT_PLANE_1_BIT")]
        const PLANE_1 = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_SamplerYcbcrConversion`](Extensions::KHR_SamplerYcbcrConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_ASPECT_PLANE_2_BIT")]
        const PLANE_2 = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Maintenance4`](Extensions::KHR_Maintenance4)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_ASPECT_NONE")]
        const NONE = 0;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageDrmFormatModifier`](Extensions::EXT_ImageDrmFormatModifier)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_ASPECT_MEMORY_PLANE_0_BIT_EXT")]
        const MEMORY_PLANE_0_EXT = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageDrmFormatModifier`](Extensions::EXT_ImageDrmFormatModifier)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_ASPECT_MEMORY_PLANE_1_BIT_EXT")]
        const MEMORY_PLANE_1_EXT = 256;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageDrmFormatModifier`](Extensions::EXT_ImageDrmFormatModifier)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_ASPECT_MEMORY_PLANE_2_BIT_EXT")]
        const MEMORY_PLANE_2_EXT = 512;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageDrmFormatModifier`](Extensions::EXT_ImageDrmFormatModifier)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_ASPECT_MEMORY_PLANE_3_BIT_EXT")]
        const MEMORY_PLANE_3_EXT = 1024;
    }
}
impl ImageAspectFlags {
    /// [`VK_IMAGE_ASPECT_PLANE_0_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMAGE_ASPECT_PLANE_0_BIT_KHR.html)
    ///
    #[doc(alias = "VK_IMAGE_ASPECT_PLANE_0_BIT_KHR")]
    pub const PLANE_0_KHR: Self = Self::PLANE_0;
    /// [`VK_IMAGE_ASPECT_PLANE_1_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMAGE_ASPECT_PLANE_1_BIT_KHR.html)
    ///
    #[doc(alias = "VK_IMAGE_ASPECT_PLANE_1_BIT_KHR")]
    pub const PLANE_1_KHR: Self = Self::PLANE_1;
    /// [`VK_IMAGE_ASPECT_PLANE_2_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMAGE_ASPECT_PLANE_2_BIT_KHR.html)
    ///
    #[doc(alias = "VK_IMAGE_ASPECT_PLANE_2_BIT_KHR")]
    pub const PLANE_2_KHR: Self = Self::PLANE_2;
    /// [`VK_IMAGE_ASPECT_NONE_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMAGE_ASPECT_NONE_KHR.html)
    ///
    #[doc(alias = "VK_IMAGE_ASPECT_NONE_KHR")]
    pub const NONE_KHR: Self = Self::NONE;
}

bitflags::bitflags! {
    /// [`VkSparseImageFormatFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageFormatFlagBits.html)
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkSparseImageFormatFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SparseImageFormatFlags: u32 {
        #[doc(alias = "VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT")]
        const SINGLE_MIPTAIL = 1;
        #[doc(alias = "VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT")]
        const ALIGNED_MIP_SIZE = 2;
        #[doc(alias = "VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT")]
        const NONSTANDARD_BLOCK_SIZE = 4;
    }
}

bitflags::bitflags! {
    /// [`VkSparseMemoryBindFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseMemoryBindFlagBits.html)
    ///
    #[doc(alias = "VkSparseMemoryBindFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SparseMemoryBindFlags: u32 {
        #[doc(alias = "VK_SPARSE_MEMORY_BIND_METADATA_BIT")]
        const METADATA = 1;
    }
}

bitflags::bitflags! {
    /// [`VkFenceCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceCreateFlagBits.html)
    ///
    #[doc(alias = "VkFenceCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct FenceCreateFlags: u32 {
        #[doc(alias = "VK_FENCE_CREATE_SIGNALED_BIT")]
        const SIGNALED = 1;
    }
}

bitflags::bitflags! {
    /// [`VkQueryPoolCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPoolCreateFlagBits.html)
    ///
    #[doc(alias = "VkQueryPoolCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct QueryPoolCreateFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance9`](Extensions::KHR_Maintenance9)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_QUERY_POOL_CREATE_RESET_BIT_KHR")]
        const RESET_KHR = 1;
    }
}

bitflags::bitflags! {
    /// [`VkQueryPipelineStatisticFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPipelineStatisticFlagBits.html)
    ///
    #[doc(alias = "VkQueryPipelineStatisticFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct QueryPipelineStatisticFlags: u32 {
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT")]
        const INPUT_ASSEMBLY_VERTICES = 1;
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT")]
        const INPUT_ASSEMBLY_PRIMITIVES = 2;
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT")]
        const VERTEX_SHADER_INVOCATIONS = 4;
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT")]
        const GEOMETRY_SHADER_INVOCATIONS = 8;
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT")]
        const GEOMETRY_SHADER_PRIMITIVES = 16;
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT")]
        const CLIPPING_INVOCATIONS = 32;
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT")]
        const CLIPPING_PRIMITIVES = 64;
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT")]
        const FRAGMENT_SHADER_INVOCATIONS = 128;
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT")]
        const TESSELLATION_CONTROL_SHADER_PATCHES = 256;
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT")]
        const TESSELLATION_EVALUATION_SHADER_INVOCATIONS = 512;
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT")]
        const COMPUTE_SHADER_INVOCATIONS = 1024;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MeshShader`](Extensions::EXT_MeshShader)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_TASK_SHADER_INVOCATIONS_BIT_EXT")]
        const TASK_SHADER_INVOCATIONS_EXT = 2048;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MeshShader`](Extensions::EXT_MeshShader)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_MESH_SHADER_INVOCATIONS_BIT_EXT")]
        const MESH_SHADER_INVOCATIONS_EXT = 4096;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`HUAWEI_ClusterCullingShader`](Extensions::HUAWEI_ClusterCullingShader)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_CLUSTER_CULLING_SHADER_INVOCATIONS_BIT_HUAWEI")]
        const CLUSTER_CULLING_SHADER_INVOCATIONS_HUAWEI = 8192;
    }
}

bitflags::bitflags! {
    /// [`VkQueryResultFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryResultFlagBits.html)
    ///
    #[doc(alias = "VkQueryResultFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct QueryResultFlags: u32 {
        #[doc(alias = "VK_QUERY_RESULT_64_BIT")]
        const _64 = 1;
        #[doc(alias = "VK_QUERY_RESULT_WAIT_BIT")]
        const WAIT = 2;
        #[doc(alias = "VK_QUERY_RESULT_WITH_AVAILABILITY_BIT")]
        const WITH_AVAILABILITY = 4;
        #[doc(alias = "VK_QUERY_RESULT_PARTIAL_BIT")]
        const PARTIAL = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extensions::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_QUERY_RESULT_WITH_STATUS_BIT_KHR")]
        const WITH_STATUS_KHR = 16;
    }
}

bitflags::bitflags! {
    /// [`VkBufferCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCreateFlagBits.html)
    ///
    #[doc(alias = "VkBufferCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct BufferCreateFlags: u32 {
        #[doc(alias = "VK_BUFFER_CREATE_SPARSE_BINDING_BIT")]
        const SPARSE_BINDING = 1;
        #[doc(alias = "VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT")]
        const SPARSE_RESIDENCY = 2;
        #[doc(alias = "VK_BUFFER_CREATE_SPARSE_ALIASED_BIT")]
        const SPARSE_ALIASED = 4;
        #[doc(alias = "VK_BUFFER_CREATE_PROTECTED_BIT")]
        const PROTECTED = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`KHR_BufferDeviceAddress`](Extensions::KHR_BufferDeviceAddress)
        /// - Extension [`EXT_BufferDeviceAddress`](Extensions::EXT_BufferDeviceAddress)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT")]
        const DEVICE_ADDRESS_CAPTURE_REPLAY = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorBuffer`](Extensions::EXT_DescriptorBuffer)
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT")]
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoMaintenance1`](Extensions::KHR_VideoMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_CREATE_VIDEO_PROFILE_INDEPENDENT_BIT_KHR")]
        const VIDEO_PROFILE_INDEPENDENT_KHR = 64;
    }
}
impl BufferCreateFlags {
    /// [`VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT.html)
    ///
    #[doc(alias = "VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT")]
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_EXT: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
    /// [`VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR.html)
    ///
    #[doc(alias = "VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR")]
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
}

bitflags::bitflags! {
    /// [`VkBufferUsageFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlagBits.html)
    ///
    #[doc(alias = "VkBufferUsageFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct BufferUsageFlags: u32 {
        #[doc(alias = "VK_BUFFER_USAGE_TRANSFER_SRC_BIT")]
        const TRANSFER_SRC = 1;
        #[doc(alias = "VK_BUFFER_USAGE_TRANSFER_DST_BIT")]
        const TRANSFER_DST = 2;
        #[doc(alias = "VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT")]
        const UNIFORM_TEXEL_BUFFER = 4;
        #[doc(alias = "VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT")]
        const STORAGE_TEXEL_BUFFER = 8;
        #[doc(alias = "VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT")]
        const UNIFORM_BUFFER = 16;
        #[doc(alias = "VK_BUFFER_USAGE_STORAGE_BUFFER_BIT")]
        const STORAGE_BUFFER = 32;
        #[doc(alias = "VK_BUFFER_USAGE_INDEX_BUFFER_BIT")]
        const INDEX_BUFFER = 64;
        #[doc(alias = "VK_BUFFER_USAGE_VERTEX_BUFFER_BIT")]
        const VERTEX_BUFFER = 128;
        #[doc(alias = "VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT")]
        const INDIRECT_BUFFER = 256;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`KHR_BufferDeviceAddress`](Extensions::KHR_BufferDeviceAddress)
        /// - Extension [`EXT_BufferDeviceAddress`](Extensions::EXT_BufferDeviceAddress)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT")]
        const SHADER_DEVICE_ADDRESS = 131072;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extensions::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_VIDEO_DECODE_SRC_BIT_KHR")]
        const VIDEO_DECODE_SRC_KHR = 8192;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extensions::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_VIDEO_DECODE_DST_BIT_KHR")]
        const VIDEO_DECODE_DST_KHR = 16384;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_TransformFeedback`](Extensions::EXT_TransformFeedback)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_BUFFER_BIT_EXT")]
        const TRANSFORM_FEEDBACK_BUFFER_EXT = 2048;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_TransformFeedback`](Extensions::EXT_TransformFeedback)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT")]
        const TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT = 4096;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ConditionalRendering`](Extensions::EXT_ConditionalRendering)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_CONDITIONAL_RENDERING_BIT_EXT")]
        const CONDITIONAL_RENDERING_EXT = 512;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMDX_ShaderEnqueue`](Extensions::AMDX_ShaderEnqueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_EXECUTION_GRAPH_SCRATCH_BIT_AMDX")]
        const EXECUTION_GRAPH_SCRATCH_AMDX = 33554432;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_DESCRIPTOR_HEAP_BIT_EXT")]
        const DESCRIPTOR_HEAP_EXT = 268435456;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_AccelerationStructure`](Extensions::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR")]
        const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR = 524288;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_AccelerationStructure`](Extensions::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_STORAGE_BIT_KHR")]
        const ACCELERATION_STRUCTURE_STORAGE_KHR = 1048576;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR")]
        const SHADER_BINDING_TABLE_KHR = 1024;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_VIDEO_ENCODE_DST_BIT_KHR")]
        const VIDEO_ENCODE_DST_KHR = 32768;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_VIDEO_ENCODE_SRC_BIT_KHR")]
        const VIDEO_ENCODE_SRC_KHR = 65536;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorBuffer`](Extensions::EXT_DescriptorBuffer)
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_SAMPLER_DESCRIPTOR_BUFFER_BIT_EXT")]
        const SAMPLER_DESCRIPTOR_BUFFER_EXT = 2097152;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorBuffer`](Extensions::EXT_DescriptorBuffer)
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_RESOURCE_DESCRIPTOR_BUFFER_BIT_EXT")]
        const RESOURCE_DESCRIPTOR_BUFFER_EXT = 4194304;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorBuffer`](Extensions::EXT_DescriptorBuffer)
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_BIT_EXT")]
        const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT = 67108864;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_OpacityMicromap`](Extensions::EXT_OpacityMicromap)
        /// - Extension [`KHR_OpacityMicromap`](Extensions::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_MICROMAP_BUILD_INPUT_READ_ONLY_BIT_EXT")]
        const MICROMAP_BUILD_INPUT_READ_ONLY_EXT = 8388608;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_OpacityMicromap`](Extensions::EXT_OpacityMicromap)
        /// - Extension [`KHR_OpacityMicromap`](Extensions::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_MICROMAP_STORAGE_BIT_EXT")]
        const MICROMAP_STORAGE_EXT = 16777216;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_TileMemoryHeap`](Extensions::QCOM_TileMemoryHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_TILE_MEMORY_BIT_QCOM")]
        const TILE_MEMORY_QCOM = 134217728;
    }
}
impl BufferUsageFlags {
    /// [`VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_EXT.html)
    ///
    #[doc(alias = "VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_EXT")]
    pub const SHADER_DEVICE_ADDRESS_EXT: Self = Self::SHADER_DEVICE_ADDRESS;
    /// [`VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_KHR.html)
    ///
    #[doc(alias = "VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_KHR")]
    pub const SHADER_DEVICE_ADDRESS_KHR: Self = Self::SHADER_DEVICE_ADDRESS;
    /// [`VK_BUFFER_USAGE_RAY_TRACING_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUFFER_USAGE_RAY_TRACING_BIT_NV.html)
    ///
    #[doc(alias = "VK_BUFFER_USAGE_RAY_TRACING_BIT_NV")]
    pub const RAY_TRACING_NV: Self = Self::SHADER_BINDING_TABLE_KHR;
}

bitflags::bitflags! {
    /// [`VkImageViewCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewCreateFlagBits.html)
    ///
    #[doc(alias = "VkImageViewCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ImageViewCreateFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_FragmentDensityMap`](Extensions::EXT_FragmentDensityMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT")]
        const FRAGMENT_DENSITY_MAP_DYNAMIC_EXT = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorBuffer`](Extensions::EXT_DescriptorBuffer)
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_VIEW_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT")]
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_FragmentDensityMap2`](Extensions::EXT_FragmentDensityMap2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT")]
        const FRAGMENT_DENSITY_MAP_DEFERRED_EXT = 2;
    }
}

bitflags::bitflags! {
    /// [`VkAccessFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlagBits.html)
    ///
    #[doc(alias = "VkAccessFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct AccessFlags: u32 {
        #[doc(alias = "VK_ACCESS_INDIRECT_COMMAND_READ_BIT")]
        const INDIRECT_COMMAND_READ = 1;
        #[doc(alias = "VK_ACCESS_INDEX_READ_BIT")]
        const INDEX_READ = 2;
        #[doc(alias = "VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT")]
        const VERTEX_ATTRIBUTE_READ = 4;
        #[doc(alias = "VK_ACCESS_UNIFORM_READ_BIT")]
        const UNIFORM_READ = 8;
        #[doc(alias = "VK_ACCESS_INPUT_ATTACHMENT_READ_BIT")]
        const INPUT_ATTACHMENT_READ = 16;
        #[doc(alias = "VK_ACCESS_SHADER_READ_BIT")]
        const SHADER_READ = 32;
        #[doc(alias = "VK_ACCESS_SHADER_WRITE_BIT")]
        const SHADER_WRITE = 64;
        #[doc(alias = "VK_ACCESS_COLOR_ATTACHMENT_READ_BIT")]
        const COLOR_ATTACHMENT_READ = 128;
        #[doc(alias = "VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT")]
        const COLOR_ATTACHMENT_WRITE = 256;
        #[doc(alias = "VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT")]
        const DEPTH_STENCIL_ATTACHMENT_READ = 512;
        #[doc(alias = "VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT")]
        const DEPTH_STENCIL_ATTACHMENT_WRITE = 1024;
        #[doc(alias = "VK_ACCESS_TRANSFER_READ_BIT")]
        const TRANSFER_READ = 2048;
        #[doc(alias = "VK_ACCESS_TRANSFER_WRITE_BIT")]
        const TRANSFER_WRITE = 4096;
        #[doc(alias = "VK_ACCESS_HOST_READ_BIT")]
        const HOST_READ = 8192;
        #[doc(alias = "VK_ACCESS_HOST_WRITE_BIT")]
        const HOST_WRITE = 16384;
        #[doc(alias = "VK_ACCESS_MEMORY_READ_BIT")]
        const MEMORY_READ = 32768;
        #[doc(alias = "VK_ACCESS_MEMORY_WRITE_BIT")]
        const MEMORY_WRITE = 65536;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_NONE")]
        const NONE = 0;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_TransformFeedback`](Extensions::EXT_TransformFeedback)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_TRANSFORM_FEEDBACK_WRITE_BIT_EXT")]
        const TRANSFORM_FEEDBACK_WRITE_EXT = 33554432;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_TransformFeedback`](Extensions::EXT_TransformFeedback)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT")]
        const TRANSFORM_FEEDBACK_COUNTER_READ_EXT = 67108864;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_TransformFeedback`](Extensions::EXT_TransformFeedback)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT")]
        const TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT = 134217728;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ConditionalRendering`](Extensions::EXT_ConditionalRendering)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_CONDITIONAL_RENDERING_READ_BIT_EXT")]
        const CONDITIONAL_RENDERING_READ_EXT = 1048576;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_BlendOperationAdvanced`](Extensions::EXT_BlendOperationAdvanced)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT")]
        const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT = 524288;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extensions::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR")]
        const ACCELERATION_STRUCTURE_READ_KHR = 2097152;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extensions::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR")]
        const ACCELERATION_STRUCTURE_WRITE_KHR = 4194304;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_FragmentDensityMap`](Extensions::EXT_FragmentDensityMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_FRAGMENT_DENSITY_MAP_READ_BIT_EXT")]
        const FRAGMENT_DENSITY_MAP_READ_EXT = 16777216;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_FragmentShadingRate`](Extensions::KHR_FragmentShadingRate)
        /// - Extension [`NV_ShadingRateImage`](Extensions::NV_ShadingRateImage)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR")]
        const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR = 8388608;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceGeneratedCommands`](Extensions::NV_DeviceGeneratedCommands)
        /// - Extension [`EXT_DeviceGeneratedCommands`](Extensions::EXT_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_COMMAND_PREPROCESS_READ_BIT_EXT")]
        const COMMAND_PREPROCESS_READ_EXT = 131072;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceGeneratedCommands`](Extensions::NV_DeviceGeneratedCommands)
        /// - Extension [`EXT_DeviceGeneratedCommands`](Extensions::EXT_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_COMMAND_PREPROCESS_WRITE_BIT_EXT")]
        const COMMAND_PREPROCESS_WRITE_EXT = 262144;
    }
}
impl AccessFlags {
    /// [`VK_ACCESS_NONE_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_NONE_KHR.html)
    ///
    #[doc(alias = "VK_ACCESS_NONE_KHR")]
    pub const NONE_KHR: Self = Self::NONE;
    /// [`VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_NV.html)
    ///
    #[doc(alias = "VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_NV")]
    pub const ACCELERATION_STRUCTURE_READ_NV: Self = Self::ACCELERATION_STRUCTURE_READ_KHR;
    /// [`VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_NV.html)
    ///
    #[doc(alias = "VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_NV")]
    pub const ACCELERATION_STRUCTURE_WRITE_NV: Self = Self::ACCELERATION_STRUCTURE_WRITE_KHR;
    /// [`VK_ACCESS_SHADING_RATE_IMAGE_READ_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_SHADING_RATE_IMAGE_READ_BIT_NV.html)
    ///
    #[doc(alias = "VK_ACCESS_SHADING_RATE_IMAGE_READ_BIT_NV")]
    pub const SHADING_RATE_IMAGE_READ_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR;
    /// [`VK_ACCESS_COMMAND_PREPROCESS_READ_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_COMMAND_PREPROCESS_READ_BIT_NV.html)
    ///
    #[doc(alias = "VK_ACCESS_COMMAND_PREPROCESS_READ_BIT_NV")]
    pub const COMMAND_PREPROCESS_READ_NV: Self = Self::COMMAND_PREPROCESS_READ_EXT;
    /// [`VK_ACCESS_COMMAND_PREPROCESS_WRITE_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_COMMAND_PREPROCESS_WRITE_BIT_NV.html)
    ///
    #[doc(alias = "VK_ACCESS_COMMAND_PREPROCESS_WRITE_BIT_NV")]
    pub const COMMAND_PREPROCESS_WRITE_NV: Self = Self::COMMAND_PREPROCESS_WRITE_EXT;
}

bitflags::bitflags! {
    /// [`VkDependencyFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDependencyFlagBits.html)
    ///
    #[doc(alias = "VkDependencyFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DependencyFlags: u32 {
        #[doc(alias = "VK_DEPENDENCY_BY_REGION_BIT")]
        const BY_REGION = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_DeviceGroup`](Extensions::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEPENDENCY_DEVICE_GROUP_BIT")]
        const DEVICE_GROUP = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_Multiview`](Extensions::KHR_Multiview)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEPENDENCY_VIEW_LOCAL_BIT")]
        const VIEW_LOCAL = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_AttachmentFeedbackLoopLayout`](Extensions::EXT_AttachmentFeedbackLoopLayout)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEPENDENCY_FEEDBACK_LOOP_BIT_EXT")]
        const FEEDBACK_LOOP_EXT = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance8`](Extensions::KHR_Maintenance8)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEPENDENCY_QUEUE_FAMILY_OWNERSHIP_TRANSFER_USE_ALL_STAGES_BIT_KHR")]
        const QUEUE_FAMILY_OWNERSHIP_TRANSFER_USE_ALL_STAGES_KHR = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance9`](Extensions::KHR_Maintenance9)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEPENDENCY_ASYMMETRIC_EVENT_BIT_KHR")]
        const ASYMMETRIC_EVENT_KHR = 64;
    }
}
impl DependencyFlags {
    /// [`VK_DEPENDENCY_DEVICE_GROUP_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_DEPENDENCY_DEVICE_GROUP_BIT_KHR.html)
    ///
    #[doc(alias = "VK_DEPENDENCY_DEVICE_GROUP_BIT_KHR")]
    pub const DEVICE_GROUP_KHR: Self = Self::DEVICE_GROUP;
    /// [`VK_DEPENDENCY_VIEW_LOCAL_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_DEPENDENCY_VIEW_LOCAL_BIT_KHR.html)
    ///
    #[doc(alias = "VK_DEPENDENCY_VIEW_LOCAL_BIT_KHR")]
    pub const VIEW_LOCAL_KHR: Self = Self::VIEW_LOCAL;
}

bitflags::bitflags! {
    /// [`VkCommandPoolCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolCreateFlagBits.html)
    ///
    #[doc(alias = "VkCommandPoolCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct CommandPoolCreateFlags: u32 {
        #[doc(alias = "VK_COMMAND_POOL_CREATE_TRANSIENT_BIT")]
        const TRANSIENT = 1;
        #[doc(alias = "VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT")]
        const RESET_COMMAND_BUFFER = 2;
        #[doc(alias = "VK_COMMAND_POOL_CREATE_PROTECTED_BIT")]
        const PROTECTED = 4;
    }
}

bitflags::bitflags! {
    /// [`VkCommandPoolResetFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolResetFlagBits.html)
    ///
    #[doc(alias = "VkCommandPoolResetFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct CommandPoolResetFlags: u32 {
        #[doc(alias = "VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT")]
        const RELEASE_RESOURCES = 1;
    }
}

bitflags::bitflags! {
    /// [`VkQueryControlFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryControlFlagBits.html)
    ///
    #[doc(alias = "VkQueryControlFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct QueryControlFlags: u32 {
        #[doc(alias = "VK_QUERY_CONTROL_PRECISE_BIT")]
        const PRECISE = 1;
    }
}

bitflags::bitflags! {
    /// [`VkCommandBufferUsageFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferUsageFlagBits.html)
    ///
    #[doc(alias = "VkCommandBufferUsageFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct CommandBufferUsageFlags: u32 {
        #[doc(alias = "VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT")]
        const ONE_TIME_SUBMIT = 1;
        #[doc(alias = "VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT")]
        const RENDER_PASS_CONTINUE = 2;
        #[doc(alias = "VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT")]
        const SIMULTANEOUS_USE = 4;
    }
}

bitflags::bitflags! {
    /// [`VkCommandBufferResetFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferResetFlagBits.html)
    ///
    #[doc(alias = "VkCommandBufferResetFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct CommandBufferResetFlags: u32 {
        #[doc(alias = "VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT")]
        const RELEASE_RESOURCES = 1;
    }
}

bitflags::bitflags! {
    /// [`VkEventCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkEventCreateFlagBits.html)
    ///
    #[doc(alias = "VkEventCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct EventCreateFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EVENT_CREATE_DEVICE_ONLY_BIT")]
        const DEVICE_ONLY = 1;
    }
}
impl EventCreateFlags {
    /// [`VK_EVENT_CREATE_DEVICE_ONLY_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EVENT_CREATE_DEVICE_ONLY_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EVENT_CREATE_DEVICE_ONLY_BIT_KHR")]
    pub const DEVICE_ONLY_KHR: Self = Self::DEVICE_ONLY;
}

bitflags::bitflags! {
    /// [`VkPipelineCacheCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCacheCreateFlagBits.html)
    ///
    #[doc(alias = "VkPipelineCacheCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PipelineCacheCreateFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_PipelineCreationCacheControl`](Extensions::EXT_PipelineCreationCacheControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT")]
        const EXTERNALLY_SYNCHRONIZED = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance8`](Extensions::KHR_Maintenance8)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CACHE_CREATE_INTERNALLY_SYNCHRONIZED_MERGE_BIT_KHR")]
        const INTERNALLY_SYNCHRONIZED_MERGE_KHR = 8;
    }
}
impl PipelineCacheCreateFlags {
    /// [`VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT_EXT.html)
    ///
    #[doc(alias = "VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT_EXT")]
    pub const EXTERNALLY_SYNCHRONIZED_EXT: Self = Self::EXTERNALLY_SYNCHRONIZED;
}

bitflags::bitflags! {
    /// [`VkPipelineCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlagBits.html)
    ///
    #[doc(alias = "VkPipelineCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PipelineCreateFlags: u32 {
        #[doc(alias = "VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT")]
        const DISABLE_OPTIMIZATION = 1;
        #[doc(alias = "VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT")]
        const ALLOW_DERIVATIVES = 2;
        #[doc(alias = "VK_PIPELINE_CREATE_DERIVATIVE_BIT")]
        const DERIVATIVE = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_DeviceGroup`](Extensions::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_DISPATCH_BASE_BIT")]
        const DISPATCH_BASE = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_DeviceGroup`](Extensions::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT")]
        const VIEW_INDEX_FROM_DEVICE_INDEX = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_PipelineCreationCacheControl`](Extensions::EXT_PipelineCreationCacheControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT")]
        const FAIL_ON_PIPELINE_COMPILE_REQUIRED = 256;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_PipelineCreationCacheControl`](Extensions::EXT_PipelineCreationCacheControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT")]
        const EARLY_RETURN_ON_FAILURE = 512;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`EXT_PipelineProtectedAccess`](Extensions::EXT_PipelineProtectedAccess)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_NO_PROTECTED_ACCESS_BIT")]
        const NO_PROTECTED_ACCESS = 134217728;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`EXT_PipelineProtectedAccess`](Extensions::EXT_PipelineProtectedAccess)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_PROTECTED_ACCESS_ONLY_BIT")]
        const PROTECTED_ACCESS_ONLY = 1073741824;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR")]
        const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR = 16384;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR")]
        const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR = 32768;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR")]
        const RAY_TRACING_NO_NULL_MISS_SHADERS_KHR = 65536;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR")]
        const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR = 131072;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR")]
        const RAY_TRACING_SKIP_TRIANGLES_KHR = 4096;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR")]
        const RAY_TRACING_SKIP_AABBS_KHR = 8192;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR")]
        const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR = 524288;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_DEFER_COMPILE_BIT_NV")]
        const DEFER_COMPILE_NV = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_FragmentDensityMap`](Extensions::EXT_FragmentDensityMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT")]
        const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT = 4194304;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_FragmentShadingRate`](Extensions::KHR_FragmentShadingRate)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
        const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = 2097152;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_PipelineExecutableProperties`](Extensions::KHR_PipelineExecutableProperties)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_CAPTURE_STATISTICS_BIT_KHR")]
        const CAPTURE_STATISTICS_KHR = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_PipelineExecutableProperties`](Extensions::KHR_PipelineExecutableProperties)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR")]
        const CAPTURE_INTERNAL_REPRESENTATIONS_KHR = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceGeneratedCommands`](Extensions::NV_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_INDIRECT_BINDABLE_BIT_NV")]
        const INDIRECT_BINDABLE_NV = 262144;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_PipelineLibrary`](Extensions::KHR_PipelineLibrary)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_LIBRARY_BIT_KHR")]
        const LIBRARY_KHR = 2048;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorBuffer`](Extensions::EXT_DescriptorBuffer)
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_DESCRIPTOR_BUFFER_BIT_EXT")]
        const DESCRIPTOR_BUFFER_EXT = 536870912;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_GraphicsPipelineLibrary`](Extensions::EXT_GraphicsPipelineLibrary)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RETAIN_LINK_TIME_OPTIMIZATION_INFO_BIT_EXT")]
        const RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT = 8388608;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_GraphicsPipelineLibrary`](Extensions::EXT_GraphicsPipelineLibrary)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_LINK_TIME_OPTIMIZATION_BIT_EXT")]
        const LINK_TIME_OPTIMIZATION_EXT = 1024;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracingMotionBlur`](Extensions::NV_RayTracingMotionBlur)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_ALLOW_MOTION_BIT_NV")]
        const RAY_TRACING_ALLOW_MOTION_NV = 1048576;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_AttachmentFeedbackLoopLayout`](Extensions::EXT_AttachmentFeedbackLoopLayout)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_COLOR_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT")]
        const COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT = 33554432;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_AttachmentFeedbackLoopLayout`](Extensions::EXT_AttachmentFeedbackLoopLayout)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT")]
        const DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT = 67108864;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DisplacementMicromap`](Extensions::NV_DisplacementMicromap)
        /// - Extension [`NV_ClusterAccelerationStructure`](Extensions::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_DISPLACEMENT_MICROMAP_BIT_NV")]
        const RAY_TRACING_DISPLACEMENT_MICROMAP_NV = 268435456;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_OpacityMicromap`](Extensions::KHR_OpacityMicromap)
        /// - Extension [`EXT_OpacityMicromap`](Extensions::EXT_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_OPACITY_MICROMAP_BIT_KHR")]
        const RAY_TRACING_OPACITY_MICROMAP_KHR = 16777216;
    }
}
impl PipelineCreateFlags {
    /// [`VK_PIPELINE_CREATE_DISPATCH_BASE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_CREATE_DISPATCH_BASE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_CREATE_DISPATCH_BASE_BIT_KHR")]
    pub const DISPATCH_BASE_KHR: Self = Self::DISPATCH_BASE;
    /// [`VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR")]
    pub const VIEW_INDEX_FROM_DEVICE_INDEX_KHR: Self = Self::VIEW_INDEX_FROM_DEVICE_INDEX;
    /// [`VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT_EXT.html)
    ///
    #[doc(alias = "VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT_EXT")]
    pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED_EXT: Self = Self::FAIL_ON_PIPELINE_COMPILE_REQUIRED;
    /// [`VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT_EXT.html)
    ///
    #[doc(alias = "VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT_EXT")]
    pub const EARLY_RETURN_ON_FAILURE_EXT: Self = Self::EARLY_RETURN_ON_FAILURE;
    /// [`VK_PIPELINE_CREATE_NO_PROTECTED_ACCESS_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_CREATE_NO_PROTECTED_ACCESS_BIT_EXT.html)
    ///
    #[doc(alias = "VK_PIPELINE_CREATE_NO_PROTECTED_ACCESS_BIT_EXT")]
    pub const NO_PROTECTED_ACCESS_EXT: Self = Self::NO_PROTECTED_ACCESS;
    /// [`VK_PIPELINE_CREATE_PROTECTED_ACCESS_ONLY_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_CREATE_PROTECTED_ACCESS_ONLY_BIT_EXT.html)
    ///
    #[doc(alias = "VK_PIPELINE_CREATE_PROTECTED_ACCESS_ONLY_BIT_EXT")]
    pub const PROTECTED_ACCESS_ONLY_EXT: Self = Self::PROTECTED_ACCESS_ONLY;
    /// [`VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT.html)
    ///
    #[doc(alias = "VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT")]
    pub const PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self =
        Self::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT;
    /// [`VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR.html)
    ///
    #[doc(
        alias = "VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR"
    )]
    pub const PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self =
        Self::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
    /// [`VK_PIPELINE_CREATE_RAY_TRACING_OPACITY_MICROMAP_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_CREATE_RAY_TRACING_OPACITY_MICROMAP_BIT_EXT.html)
    ///
    #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_OPACITY_MICROMAP_BIT_EXT")]
    pub const RAY_TRACING_OPACITY_MICROMAP_EXT: Self = Self::RAY_TRACING_OPACITY_MICROMAP_KHR;
}

bitflags::bitflags! {
    /// [`VkPipelineLayoutCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineLayoutCreateFlagBits.html)
    ///
    #[doc(alias = "VkPipelineLayoutCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PipelineLayoutCreateFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance11`](Extensions::KHR_Maintenance11)
        /// - Extension [`EXT_GraphicsPipelineLibrary`](Extensions::EXT_GraphicsPipelineLibrary)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_LAYOUT_CREATE_INDEPENDENT_SETS_BIT_EXT")]
        const INDEPENDENT_SETS_EXT = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance11`](Extensions::KHR_Maintenance11)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_LAYOUT_CREATE_NO_TASK_SHADER_BIT_KHR")]
        const NO_TASK_SHADER_KHR = 4;
    }
}

bitflags::bitflags! {
    /// [`VkPipelineShaderStageCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineShaderStageCreateFlagBits.html)
    ///
    #[doc(alias = "VkPipelineShaderStageCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PipelineShaderStageCreateFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_SubgroupSizeControl`](Extensions::EXT_SubgroupSizeControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT")]
        const ALLOW_VARYING_SUBGROUP_SIZE = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_SubgroupSizeControl`](Extensions::EXT_SubgroupSizeControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT")]
        const REQUIRE_FULL_SUBGROUPS = 2;
    }
}
impl PipelineShaderStageCreateFlags {
    /// [`VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT_EXT.html)
    ///
    #[doc(alias = "VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT_EXT")]
    pub const ALLOW_VARYING_SUBGROUP_SIZE_EXT: Self = Self::ALLOW_VARYING_SUBGROUP_SIZE;
    /// [`VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT_EXT.html)
    ///
    #[doc(alias = "VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT_EXT")]
    pub const REQUIRE_FULL_SUBGROUPS_EXT: Self = Self::REQUIRE_FULL_SUBGROUPS;
}

bitflags::bitflags! {
    /// [`VkSamplerCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerCreateFlagBits.html)
    ///
    #[doc(alias = "VkSamplerCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SamplerCreateFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_FragmentDensityMap`](Extensions::EXT_FragmentDensityMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT")]
        const SUBSAMPLED_EXT = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_FragmentDensityMap`](Extensions::EXT_FragmentDensityMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SAMPLER_CREATE_SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT")]
        const SUBSAMPLED_COARSE_RECONSTRUCTION_EXT = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorBuffer`](Extensions::EXT_DescriptorBuffer)
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SAMPLER_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT")]
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_NonSeamlessCubeMap`](Extensions::EXT_NonSeamlessCubeMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SAMPLER_CREATE_NON_SEAMLESS_CUBE_MAP_BIT_EXT")]
        const NON_SEAMLESS_CUBE_MAP_EXT = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_ImageProcessing`](Extensions::QCOM_ImageProcessing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SAMPLER_CREATE_IMAGE_PROCESSING_BIT_QCOM")]
        const IMAGE_PROCESSING_QCOM = 16;
    }
}

bitflags::bitflags! {
    /// [`VkDescriptorPoolCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPoolCreateFlagBits.html)
    ///
    #[doc(alias = "VkDescriptorPoolCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DescriptorPoolCreateFlags: u32 {
        #[doc(alias = "VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT")]
        const FREE_DESCRIPTOR_SET = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`EXT_DescriptorIndexing`](Extensions::EXT_DescriptorIndexing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT")]
        const UPDATE_AFTER_BIND = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_MutableDescriptorType`](Extensions::VALVE_MutableDescriptorType)
        /// - Extension [`EXT_MutableDescriptorType`](Extensions::EXT_MutableDescriptorType)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_EXT")]
        const HOST_ONLY_EXT = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DescriptorPoolOverallocation`](Extensions::NV_DescriptorPoolOverallocation)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_POOL_CREATE_ALLOW_OVERALLOCATION_SETS_BIT_NV")]
        const ALLOW_OVERALLOCATION_SETS_NV = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DescriptorPoolOverallocation`](Extensions::NV_DescriptorPoolOverallocation)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_POOL_CREATE_ALLOW_OVERALLOCATION_POOLS_BIT_NV")]
        const ALLOW_OVERALLOCATION_POOLS_NV = 16;
    }
}
impl DescriptorPoolCreateFlags {
    /// [`VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT_EXT.html)
    ///
    #[doc(alias = "VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT_EXT")]
    pub const UPDATE_AFTER_BIND_EXT: Self = Self::UPDATE_AFTER_BIND;
    /// [`VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE.html)
    ///
    #[doc(alias = "VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE")]
    pub const HOST_ONLY_VALVE: Self = Self::HOST_ONLY_EXT;
}

bitflags::bitflags! {
    /// [`VkDescriptorSetLayoutCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayoutCreateFlagBits.html)
    ///
    #[doc(alias = "VkDescriptorSetLayoutCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DescriptorSetLayoutCreateFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`EXT_DescriptorIndexing`](Extensions::EXT_DescriptorIndexing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT")]
        const UPDATE_AFTER_BIND_POOL = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_PushDescriptor`](Extensions::KHR_PushDescriptor)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT")]
        const PUSH_DESCRIPTOR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorBuffer`](Extensions::EXT_DescriptorBuffer)
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_DESCRIPTOR_BUFFER_BIT_EXT")]
        const DESCRIPTOR_BUFFER_EXT = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorBuffer`](Extensions::EXT_DescriptorBuffer)
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_EMBEDDED_IMMUTABLE_SAMPLERS_BIT_EXT")]
        const EMBEDDED_IMMUTABLE_SAMPLERS_EXT = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceGeneratedCommandsCompute`](Extensions::NV_DeviceGeneratedCommandsCompute)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_INDIRECT_BINDABLE_BIT_NV")]
        const INDIRECT_BINDABLE_NV = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_MutableDescriptorType`](Extensions::VALVE_MutableDescriptorType)
        /// - Extension [`EXT_MutableDescriptorType`](Extensions::EXT_MutableDescriptorType)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_EXT")]
        const HOST_ONLY_POOL_EXT = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_PerStageDescriptorSet`](Extensions::NV_PerStageDescriptorSet)
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_PER_STAGE_BIT_NV")]
        const PER_STAGE_NV = 64;
    }
}
impl DescriptorSetLayoutCreateFlags {
    /// [`VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT_EXT.html)
    ///
    #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT_EXT")]
    pub const UPDATE_AFTER_BIND_POOL_EXT: Self = Self::UPDATE_AFTER_BIND_POOL;
    /// [`VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR.html)
    ///
    #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR")]
    pub const PUSH_DESCRIPTOR_KHR: Self = Self::PUSH_DESCRIPTOR;
    /// [`VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_VALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_VALVE.html)
    ///
    #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_VALVE")]
    pub const HOST_ONLY_POOL_VALVE: Self = Self::HOST_ONLY_POOL_EXT;
}

bitflags::bitflags! {
    /// [`VkColorComponentFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkColorComponentFlagBits.html)
    ///
    #[doc(alias = "VkColorComponentFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ColorComponentFlags: u32 {
        #[doc(alias = "VK_COLOR_COMPONENT_R_BIT")]
        const R = 1;
        #[doc(alias = "VK_COLOR_COMPONENT_G_BIT")]
        const G = 2;
        #[doc(alias = "VK_COLOR_COMPONENT_B_BIT")]
        const B = 4;
        #[doc(alias = "VK_COLOR_COMPONENT_A_BIT")]
        const A = 8;
    }
}

bitflags::bitflags! {
    /// [`VkCullModeFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCullModeFlagBits.html)
    ///
    #[doc(alias = "VkCullModeFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct CullModeFlags: u32 {
        #[doc(alias = "VK_CULL_MODE_NONE")]
        const NONE = 0;
        #[doc(alias = "VK_CULL_MODE_FRONT_BIT")]
        const FRONT = 1;
        #[doc(alias = "VK_CULL_MODE_BACK_BIT")]
        const BACK = 2;
        #[doc(alias = "VK_CULL_MODE_FRONT_AND_BACK")]
        const FRONT_AND_BACK = 3;
    }
}

bitflags::bitflags! {
    /// [`VkPipelineColorBlendStateCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineColorBlendStateCreateFlagBits.html)
    ///
    #[doc(alias = "VkPipelineColorBlendStateCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PipelineColorBlendStateCreateFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_RasterizationOrderAttachmentAccess`](Extensions::ARM_RasterizationOrderAttachmentAccess)
        /// - Extension [`EXT_RasterizationOrderAttachmentAccess`](Extensions::EXT_RasterizationOrderAttachmentAccess)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_COLOR_BLEND_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_EXT")]
        const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXT = 1;
    }
}
impl PipelineColorBlendStateCreateFlags {
    /// [`VK_PIPELINE_COLOR_BLEND_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_ARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_COLOR_BLEND_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_ARM.html)
    ///
    #[doc(
        alias = "VK_PIPELINE_COLOR_BLEND_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_ARM"
    )]
    pub const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXT;
}

bitflags::bitflags! {
    /// [`VkPipelineDepthStencilStateCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineDepthStencilStateCreateFlagBits.html)
    ///
    #[doc(alias = "VkPipelineDepthStencilStateCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PipelineDepthStencilStateCreateFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_RasterizationOrderAttachmentAccess`](Extensions::ARM_RasterizationOrderAttachmentAccess)
        /// - Extension [`EXT_RasterizationOrderAttachmentAccess`](Extensions::EXT_RasterizationOrderAttachmentAccess)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT")]
        const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_RasterizationOrderAttachmentAccess`](Extensions::ARM_RasterizationOrderAttachmentAccess)
        /// - Extension [`EXT_RasterizationOrderAttachmentAccess`](Extensions::EXT_RasterizationOrderAttachmentAccess)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT")]
        const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT = 2;
    }
}
impl PipelineDepthStencilStateCreateFlags {
    /// [`VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM.html)
    ///
    #[doc(
        alias = "VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM"
    )]
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT;
    /// [`VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM.html)
    ///
    #[doc(
        alias = "VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM"
    )]
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT;
}

bitflags::bitflags! {
    /// [`VkAttachmentDescriptionFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentDescriptionFlagBits.html)
    ///
    #[doc(alias = "VkAttachmentDescriptionFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct AttachmentDescriptionFlags: u32 {
        #[doc(alias = "VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT")]
        const MAY_ALIAS = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance10`](Extensions::KHR_Maintenance10)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ATTACHMENT_DESCRIPTION_RESOLVE_SKIP_TRANSFER_FUNCTION_BIT_KHR")]
        const RESOLVE_SKIP_TRANSFER_FUNCTION_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance10`](Extensions::KHR_Maintenance10)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ATTACHMENT_DESCRIPTION_RESOLVE_ENABLE_TRANSFER_FUNCTION_BIT_KHR")]
        const RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR = 4;
    }
}

bitflags::bitflags! {
    /// [`VkFramebufferCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebufferCreateFlagBits.html)
    ///
    #[doc(alias = "VkFramebufferCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct FramebufferCreateFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`KHR_ImagelessFramebuffer`](Extensions::KHR_ImagelessFramebuffer)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT")]
        const IMAGELESS = 1;
    }
}
impl FramebufferCreateFlags {
    /// [`VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT_KHR")]
    pub const IMAGELESS_KHR: Self = Self::IMAGELESS;
}

bitflags::bitflags! {
    /// [`VkRenderPassCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassCreateFlagBits.html)
    ///
    #[doc(alias = "VkRenderPassCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct RenderPassCreateFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_RenderPassTransform`](Extensions::QCOM_RenderPassTransform)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDER_PASS_CREATE_TRANSFORM_BIT_QCOM")]
        const TRANSFORM_QCOM = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_FragmentDensityMapLayered`](Extensions::VALVE_FragmentDensityMapLayered)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDER_PASS_CREATE_PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE")]
        const PER_LAYER_FRAGMENT_DENSITY_VALVE = 4;
    }
}

bitflags::bitflags! {
    /// [`VkSubpassDescriptionFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassDescriptionFlagBits.html)
    ///
    #[doc(alias = "VkSubpassDescriptionFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SubpassDescriptionFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NVX_MultiviewPerViewAttributes`](Extensions::NVX_MultiviewPerViewAttributes)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX")]
        const PER_VIEW_ATTRIBUTES_NVX = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NVX_MultiviewPerViewAttributes`](Extensions::NVX_MultiviewPerViewAttributes)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_PER_VIEW_POSITION_X_ONLY_BIT_NVX")]
        const PER_VIEW_POSITION_X_ONLY_NVX = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_TileShading`](Extensions::QCOM_TileShading)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_TILE_SHADING_APRON_BIT_QCOM")]
        const TILE_SHADING_APRON_QCOM = 256;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_RasterizationOrderAttachmentAccess`](Extensions::ARM_RasterizationOrderAttachmentAccess)
        /// - Extension [`EXT_RasterizationOrderAttachmentAccess`](Extensions::EXT_RasterizationOrderAttachmentAccess)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_EXT")]
        const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_EXT = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_RasterizationOrderAttachmentAccess`](Extensions::ARM_RasterizationOrderAttachmentAccess)
        /// - Extension [`EXT_RasterizationOrderAttachmentAccess`](Extensions::EXT_RasterizationOrderAttachmentAccess)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT")]
        const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_RasterizationOrderAttachmentAccess`](Extensions::ARM_RasterizationOrderAttachmentAccess)
        /// - Extension [`EXT_RasterizationOrderAttachmentAccess`](Extensions::EXT_RasterizationOrderAttachmentAccess)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT")]
        const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_LegacyDithering`](Extensions::EXT_LegacyDithering)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_ENABLE_LEGACY_DITHERING_BIT_EXT")]
        const ENABLE_LEGACY_DITHERING_EXT = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_RenderPassShaderResolve`](Extensions::QCOM_RenderPassShaderResolve)
        /// - Extension [`EXT_CustomResolve`](Extensions::EXT_CustomResolve)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_FRAGMENT_REGION_BIT_EXT")]
        const FRAGMENT_REGION_EXT = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_RenderPassShaderResolve`](Extensions::QCOM_RenderPassShaderResolve)
        /// - Extension [`EXT_CustomResolve`](Extensions::EXT_CustomResolve)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_CUSTOM_RESOLVE_BIT_EXT")]
        const CUSTOM_RESOLVE_EXT = 8;
    }
}
impl SubpassDescriptionFlags {
    /// [`VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_ARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_ARM.html)
    ///
    #[doc(alias = "VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_ARM")]
    pub const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_EXT;
    /// [`VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM.html)
    ///
    #[doc(alias = "VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM")]
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT;
    /// [`VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM.html)
    ///
    #[doc(alias = "VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM")]
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT;
    /// [`VK_SUBPASS_DESCRIPTION_FRAGMENT_REGION_BIT_QCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SUBPASS_DESCRIPTION_FRAGMENT_REGION_BIT_QCOM.html)
    ///
    #[doc(alias = "VK_SUBPASS_DESCRIPTION_FRAGMENT_REGION_BIT_QCOM")]
    pub const FRAGMENT_REGION_QCOM: Self = Self::FRAGMENT_REGION_EXT;
    /// [`VK_SUBPASS_DESCRIPTION_SHADER_RESOLVE_BIT_QCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SUBPASS_DESCRIPTION_SHADER_RESOLVE_BIT_QCOM.html)
    ///
    #[doc(alias = "VK_SUBPASS_DESCRIPTION_SHADER_RESOLVE_BIT_QCOM")]
    pub const SHADER_RESOLVE_QCOM: Self = Self::CUSTOM_RESOLVE_EXT;
}

bitflags::bitflags! {
    /// [`VkStencilFaceFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkStencilFaceFlagBits.html)
    ///
    #[doc(alias = "VkStencilFaceFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct StencilFaceFlags: u32 {
        #[doc(alias = "VK_STENCIL_FACE_FRONT_BIT")]
        const FRONT = 1;
        #[doc(alias = "VK_STENCIL_FACE_BACK_BIT")]
        const BACK = 2;
        #[doc(alias = "VK_STENCIL_FACE_FRONT_AND_BACK")]
        const FRONT_AND_BACK = 3;
    }
}
impl StencilFaceFlags {
    /// [`VK_STENCIL_FRONT_AND_BACK`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_STENCIL_FRONT_AND_BACK.html)
    ///
    #[doc(alias = "VK_STENCIL_FRONT_AND_BACK")]
    pub const STENCIL_FRONT_AND_BACK: Self = Self::FRONT_AND_BACK;
}

bitflags::bitflags! {
    /// [`VkSubgroupFeatureFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubgroupFeatureFlagBits.html)
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkSubgroupFeatureFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SubgroupFeatureFlags: u32 {
        #[doc(alias = "VK_SUBGROUP_FEATURE_BASIC_BIT")]
        const BASIC = 1;
        #[doc(alias = "VK_SUBGROUP_FEATURE_VOTE_BIT")]
        const VOTE = 2;
        #[doc(alias = "VK_SUBGROUP_FEATURE_ARITHMETIC_BIT")]
        const ARITHMETIC = 4;
        #[doc(alias = "VK_SUBGROUP_FEATURE_BALLOT_BIT")]
        const BALLOT = 8;
        #[doc(alias = "VK_SUBGROUP_FEATURE_SHUFFLE_BIT")]
        const SHUFFLE = 16;
        #[doc(alias = "VK_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT")]
        const SHUFFLE_RELATIVE = 32;
        #[doc(alias = "VK_SUBGROUP_FEATURE_CLUSTERED_BIT")]
        const CLUSTERED = 64;
        #[doc(alias = "VK_SUBGROUP_FEATURE_QUAD_BIT")]
        const QUAD = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_ShaderSubgroupRotate`](Extensions::KHR_ShaderSubgroupRotate)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBGROUP_FEATURE_ROTATE_BIT")]
        const ROTATE = 512;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_ShaderSubgroupRotate`](Extensions::KHR_ShaderSubgroupRotate)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBGROUP_FEATURE_ROTATE_CLUSTERED_BIT")]
        const ROTATE_CLUSTERED = 1024;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ShaderSubgroupPartitioned`](Extensions::NV_ShaderSubgroupPartitioned)
        /// - Extension [`EXT_ShaderSubgroupPartitioned`](Extensions::EXT_ShaderSubgroupPartitioned)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBGROUP_FEATURE_PARTITIONED_BIT_EXT")]
        const PARTITIONED_EXT = 256;
    }
}
impl SubgroupFeatureFlags {
    /// [`VK_SUBGROUP_FEATURE_ROTATE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SUBGROUP_FEATURE_ROTATE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_SUBGROUP_FEATURE_ROTATE_BIT_KHR")]
    pub const ROTATE_KHR: Self = Self::ROTATE;
    /// [`VK_SUBGROUP_FEATURE_ROTATE_CLUSTERED_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SUBGROUP_FEATURE_ROTATE_CLUSTERED_BIT_KHR.html)
    ///
    #[doc(alias = "VK_SUBGROUP_FEATURE_ROTATE_CLUSTERED_BIT_KHR")]
    pub const ROTATE_CLUSTERED_KHR: Self = Self::ROTATE_CLUSTERED;
    /// [`VK_SUBGROUP_FEATURE_PARTITIONED_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SUBGROUP_FEATURE_PARTITIONED_BIT_NV.html)
    ///
    #[doc(alias = "VK_SUBGROUP_FEATURE_PARTITIONED_BIT_NV")]
    pub const PARTITIONED_NV: Self = Self::PARTITIONED_EXT;
}

bitflags::bitflags! {
    /// [`VkPeerMemoryFeatureFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPeerMemoryFeatureFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.1 with appropriate features
    /// - Extension [`KHR_DeviceGroup`](Extensions::KHR_DeviceGroup)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkPeerMemoryFeatureFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PeerMemoryFeatureFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_DeviceGroup`](Extensions::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT")]
        const COPY_SRC = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_DeviceGroup`](Extensions::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_DST_BIT")]
        const COPY_DST = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_DeviceGroup`](Extensions::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT")]
        const GENERIC_SRC = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_DeviceGroup`](Extensions::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT")]
        const GENERIC_DST = 8;
    }
}
/// [`VkPeerMemoryFeatureFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPeerMemoryFeatureFlagBitsKHR.html)
///
#[doc(alias = "VkPeerMemoryFeatureFlagBitsKHR")]
pub type PeerMemoryFeatureFlagsKHR = PeerMemoryFeatureFlags;
impl PeerMemoryFeatureFlags {
    /// [`VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT_KHR")]
    pub const COPY_SRC_KHR: Self = Self::COPY_SRC;
    /// [`VK_PEER_MEMORY_FEATURE_COPY_DST_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PEER_MEMORY_FEATURE_COPY_DST_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_DST_BIT_KHR")]
    pub const COPY_DST_KHR: Self = Self::COPY_DST;
    /// [`VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT_KHR")]
    pub const GENERIC_SRC_KHR: Self = Self::GENERIC_SRC;
    /// [`VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT_KHR")]
    pub const GENERIC_DST_KHR: Self = Self::GENERIC_DST;
}

bitflags::bitflags! {
    /// [`VkMemoryAllocateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryAllocateFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.1 with appropriate features
    /// - Extension [`KHR_DeviceGroup`](Extensions::KHR_DeviceGroup)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkMemoryAllocateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct MemoryAllocateFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_DeviceGroup`](Extensions::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT")]
        const DEVICE_MASK = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`KHR_BufferDeviceAddress`](Extensions::KHR_BufferDeviceAddress)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT")]
        const DEVICE_ADDRESS = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`KHR_BufferDeviceAddress`](Extensions::KHR_BufferDeviceAddress)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT")]
        const DEVICE_ADDRESS_CAPTURE_REPLAY = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ZeroInitializeDeviceMemory`](Extensions::EXT_ZeroInitializeDeviceMemory)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MEMORY_ALLOCATE_ZERO_INITIALIZE_BIT_EXT")]
        const ZERO_INITIALIZE_EXT = 8;
    }
}
/// [`VkMemoryAllocateFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryAllocateFlagBitsKHR.html)
///
#[doc(alias = "VkMemoryAllocateFlagBitsKHR")]
pub type MemoryAllocateFlagsKHR = MemoryAllocateFlags;
impl MemoryAllocateFlags {
    /// [`VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT_KHR.html)
    ///
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT_KHR")]
    pub const DEVICE_MASK_KHR: Self = Self::DEVICE_MASK;
    /// [`VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT_KHR.html)
    ///
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT_KHR")]
    pub const DEVICE_ADDRESS_KHR: Self = Self::DEVICE_ADDRESS;
    /// [`VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR.html)
    ///
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR")]
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
}

bitflags::bitflags! {
    /// [`VkExternalMemoryHandleTypeFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryHandleTypeFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.1 with appropriate features
    /// - Extension [`KHR_ExternalMemoryCapabilities`](Extensions::KHR_ExternalMemoryCapabilities)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkExternalMemoryHandleTypeFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ExternalMemoryHandleTypeFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extensions::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT")]
        const OPAQUE_FD = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extensions::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT")]
        const OPAQUE_WIN32 = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extensions::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT")]
        const OPAQUE_WIN32_KMT = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extensions::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT")]
        const D3D11_TEXTURE = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extensions::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT")]
        const D3D11_TEXTURE_KMT = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extensions::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT")]
        const D3D12_HEAP = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extensions::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT")]
        const D3D12_RESOURCE = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ExternalMemoryDmaBuf`](Extensions::EXT_ExternalMemoryDmaBuf)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT")]
        const DMA_BUF_EXT = 512;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ANDROID_ExternalMemoryAndroidHardwareBuffer`](Extensions::ANDROID_ExternalMemoryAndroidHardwareBuffer)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID")]
        const ANDROID_HARDWARE_BUFFER_ANDROID = 1024;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ExternalMemoryHost`](Extensions::EXT_ExternalMemoryHost)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT")]
        const HOST_ALLOCATION_EXT = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ExternalMemoryHost`](Extensions::EXT_ExternalMemoryHost)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT")]
        const HOST_MAPPED_FOREIGN_MEMORY_EXT = 256;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`FUCHSIA_ExternalMemory`](Extensions::FUCHSIA_ExternalMemory)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA")]
        const ZIRCON_VMO_FUCHSIA = 2048;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ExternalMemoryRdma`](Extensions::NV_ExternalMemoryRdma)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_RDMA_ADDRESS_BIT_NV")]
        const RDMA_ADDRESS_NV = 4096;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`OHOS_ExternalMemory`](Extensions::OHOS_ExternalMemory)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OH_NATIVE_BUFFER_BIT_OHOS")]
        const OH_NATIVE_BUFFER_OHOS = 32768;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QNX_ExternalMemoryScreenBuffer`](Extensions::QNX_ExternalMemoryScreenBuffer)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_SCREEN_BUFFER_BIT_QNX")]
        const SCREEN_BUFFER_QNX = 16384;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ExternalMemoryMetal`](Extensions::EXT_ExternalMemoryMetal)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_MTLBUFFER_BIT_EXT")]
        const MTLBUFFER_EXT = 65536;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ExternalMemoryMetal`](Extensions::EXT_ExternalMemoryMetal)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_MTLTEXTURE_BIT_EXT")]
        const MTLTEXTURE_EXT = 131072;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ExternalMemoryMetal`](Extensions::EXT_ExternalMemoryMetal)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_MTLHEAP_BIT_EXT")]
        const MTLHEAP_EXT = 262144;
    }
}
/// [`VkExternalMemoryHandleTypeFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryHandleTypeFlagBitsKHR.html)
///
#[doc(alias = "VkExternalMemoryHandleTypeFlagBitsKHR")]
pub type ExternalMemoryHandleTypeFlagsKHR = ExternalMemoryHandleTypeFlags;
impl ExternalMemoryHandleTypeFlags {
    /// [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR")]
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    /// [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR")]
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    /// [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR")]
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    /// [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR")]
    pub const D3D11_TEXTURE_KHR: Self = Self::D3D11_TEXTURE;
    /// [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR")]
    pub const D3D11_TEXTURE_KMT_KHR: Self = Self::D3D11_TEXTURE_KMT;
    /// [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR")]
    pub const D3D12_HEAP_KHR: Self = Self::D3D12_HEAP;
    /// [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR")]
    pub const D3D12_RESOURCE_KHR: Self = Self::D3D12_RESOURCE;
}

bitflags::bitflags! {
    /// [`VkExternalMemoryFeatureFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryFeatureFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.1 with appropriate features
    /// - Extension [`KHR_ExternalMemoryCapabilities`](Extensions::KHR_ExternalMemoryCapabilities)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkExternalMemoryFeatureFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ExternalMemoryFeatureFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extensions::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT")]
        const DEDICATED_ONLY = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extensions::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT")]
        const EXPORTABLE = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extensions::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT")]
        const IMPORTABLE = 4;
    }
}
/// [`VkExternalMemoryFeatureFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryFeatureFlagBitsKHR.html)
///
#[doc(alias = "VkExternalMemoryFeatureFlagBitsKHR")]
pub type ExternalMemoryFeatureFlagsKHR = ExternalMemoryFeatureFlags;
impl ExternalMemoryFeatureFlags {
    /// [`VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_KHR")]
    pub const DEDICATED_ONLY_KHR: Self = Self::DEDICATED_ONLY;
    /// [`VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_KHR")]
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    /// [`VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_KHR")]
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
}

bitflags::bitflags! {
    /// [`VkExternalFenceHandleTypeFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceHandleTypeFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.1 with appropriate features
    /// - Extension [`KHR_ExternalFenceCapabilities`](Extensions::KHR_ExternalFenceCapabilities)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkExternalFenceHandleTypeFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ExternalFenceHandleTypeFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalFenceCapabilities`](Extensions::KHR_ExternalFenceCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT")]
        const OPAQUE_FD = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalFenceCapabilities`](Extensions::KHR_ExternalFenceCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT")]
        const OPAQUE_WIN32 = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalFenceCapabilities`](Extensions::KHR_ExternalFenceCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT")]
        const OPAQUE_WIN32_KMT = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalFenceCapabilities`](Extensions::KHR_ExternalFenceCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT")]
        const SYNC_FD = 8;
    }
}
/// [`VkExternalFenceHandleTypeFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceHandleTypeFlagBitsKHR.html)
///
#[doc(alias = "VkExternalFenceHandleTypeFlagBitsKHR")]
pub type ExternalFenceHandleTypeFlagsKHR = ExternalFenceHandleTypeFlags;
impl ExternalFenceHandleTypeFlags {
    /// [`VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR")]
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    /// [`VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR")]
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    /// [`VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR")]
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    /// [`VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR")]
    pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
}

bitflags::bitflags! {
    /// [`VkExternalFenceFeatureFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceFeatureFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.1 with appropriate features
    /// - Extension [`KHR_ExternalFenceCapabilities`](Extensions::KHR_ExternalFenceCapabilities)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkExternalFenceFeatureFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ExternalFenceFeatureFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalFenceCapabilities`](Extensions::KHR_ExternalFenceCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT")]
        const EXPORTABLE = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalFenceCapabilities`](Extensions::KHR_ExternalFenceCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT")]
        const IMPORTABLE = 2;
    }
}
/// [`VkExternalFenceFeatureFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceFeatureFlagBitsKHR.html)
///
#[doc(alias = "VkExternalFenceFeatureFlagBitsKHR")]
pub type ExternalFenceFeatureFlagsKHR = ExternalFenceFeatureFlags;
impl ExternalFenceFeatureFlags {
    /// [`VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR")]
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    /// [`VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR")]
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
}

bitflags::bitflags! {
    /// [`VkFenceImportFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceImportFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.1 with appropriate features
    /// - Extension [`KHR_ExternalFence`](Extensions::KHR_ExternalFence)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkFenceImportFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct FenceImportFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalFence`](Extensions::KHR_ExternalFence)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FENCE_IMPORT_TEMPORARY_BIT")]
        const TEMPORARY = 1;
    }
}
/// [`VkFenceImportFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceImportFlagBitsKHR.html)
///
#[doc(alias = "VkFenceImportFlagBitsKHR")]
pub type FenceImportFlagsKHR = FenceImportFlags;
impl FenceImportFlags {
    /// [`VK_FENCE_IMPORT_TEMPORARY_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FENCE_IMPORT_TEMPORARY_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FENCE_IMPORT_TEMPORARY_BIT_KHR")]
    pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
}

bitflags::bitflags! {
    /// [`VkSemaphoreImportFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreImportFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.1 with appropriate features
    /// - Extension [`KHR_ExternalSemaphore`](Extensions::KHR_ExternalSemaphore)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkSemaphoreImportFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SemaphoreImportFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalSemaphore`](Extensions::KHR_ExternalSemaphore)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SEMAPHORE_IMPORT_TEMPORARY_BIT")]
        const TEMPORARY = 1;
    }
}
/// [`VkSemaphoreImportFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreImportFlagBitsKHR.html)
///
#[doc(alias = "VkSemaphoreImportFlagBitsKHR")]
pub type SemaphoreImportFlagsKHR = SemaphoreImportFlags;
impl SemaphoreImportFlags {
    /// [`VK_SEMAPHORE_IMPORT_TEMPORARY_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SEMAPHORE_IMPORT_TEMPORARY_BIT_KHR.html)
    ///
    #[doc(alias = "VK_SEMAPHORE_IMPORT_TEMPORARY_BIT_KHR")]
    pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
}

bitflags::bitflags! {
    /// [`VkExternalSemaphoreHandleTypeFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreHandleTypeFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.1 with appropriate features
    /// - Extension [`KHR_ExternalSemaphoreCapabilities`](Extensions::KHR_ExternalSemaphoreCapabilities)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkExternalSemaphoreHandleTypeFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ExternalSemaphoreHandleTypeFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalSemaphoreCapabilities`](Extensions::KHR_ExternalSemaphoreCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT")]
        const OPAQUE_FD = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalSemaphoreCapabilities`](Extensions::KHR_ExternalSemaphoreCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT")]
        const OPAQUE_WIN32 = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalSemaphoreCapabilities`](Extensions::KHR_ExternalSemaphoreCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT")]
        const OPAQUE_WIN32_KMT = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalSemaphoreCapabilities`](Extensions::KHR_ExternalSemaphoreCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT")]
        const D3D12_FENCE = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalSemaphoreCapabilities`](Extensions::KHR_ExternalSemaphoreCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT")]
        const SYNC_FD = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`FUCHSIA_ExternalSemaphore`](Extensions::FUCHSIA_ExternalSemaphore)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_ZIRCON_EVENT_BIT_FUCHSIA")]
        const ZIRCON_EVENT_FUCHSIA = 128;
    }
}
/// [`VkExternalSemaphoreHandleTypeFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreHandleTypeFlagBitsKHR.html)
///
#[doc(alias = "VkExternalSemaphoreHandleTypeFlagBitsKHR")]
pub type ExternalSemaphoreHandleTypeFlagsKHR = ExternalSemaphoreHandleTypeFlags;
impl ExternalSemaphoreHandleTypeFlags {
    /// [`VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR")]
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    /// [`VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR")]
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    /// [`VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR")]
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    /// [`VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT.html)
    ///
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT")]
    pub const D3D11_FENCE: Self = Self::D3D12_FENCE;
    /// [`VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR")]
    pub const D3D12_FENCE_KHR: Self = Self::D3D12_FENCE;
    /// [`VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT_KHR")]
    pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
}

bitflags::bitflags! {
    /// [`VkExternalSemaphoreFeatureFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreFeatureFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.1 with appropriate features
    /// - Extension [`KHR_ExternalSemaphoreCapabilities`](Extensions::KHR_ExternalSemaphoreCapabilities)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkExternalSemaphoreFeatureFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ExternalSemaphoreFeatureFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalSemaphoreCapabilities`](Extensions::KHR_ExternalSemaphoreCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT")]
        const EXPORTABLE = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalSemaphoreCapabilities`](Extensions::KHR_ExternalSemaphoreCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT")]
        const IMPORTABLE = 2;
    }
}
/// [`VkExternalSemaphoreFeatureFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreFeatureFlagBitsKHR.html)
///
#[doc(alias = "VkExternalSemaphoreFeatureFlagBitsKHR")]
pub type ExternalSemaphoreFeatureFlagsKHR = ExternalSemaphoreFeatureFlags;
impl ExternalSemaphoreFeatureFlags {
    /// [`VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHR")]
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    /// [`VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHR")]
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
}

bitflags::bitflags! {
    /// [`VkResolveModeFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkResolveModeFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.2 with appropriate features
    /// - Extension [`KHR_DepthStencilResolve`](Extensions::KHR_DepthStencilResolve)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkResolveModeFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ResolveModeFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`KHR_DepthStencilResolve`](Extensions::KHR_DepthStencilResolve)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RESOLVE_MODE_NONE")]
        const NONE = 0;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`KHR_DepthStencilResolve`](Extensions::KHR_DepthStencilResolve)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RESOLVE_MODE_SAMPLE_ZERO_BIT")]
        const SAMPLE_ZERO = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`KHR_DepthStencilResolve`](Extensions::KHR_DepthStencilResolve)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RESOLVE_MODE_AVERAGE_BIT")]
        const AVERAGE = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`KHR_DepthStencilResolve`](Extensions::KHR_DepthStencilResolve)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RESOLVE_MODE_MIN_BIT")]
        const MIN = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`KHR_DepthStencilResolve`](Extensions::KHR_DepthStencilResolve)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RESOLVE_MODE_MAX_BIT")]
        const MAX = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ANDROID_ExternalFormatResolve`](Extensions::ANDROID_ExternalFormatResolve)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RESOLVE_MODE_EXTERNAL_FORMAT_DOWNSAMPLE_BIT_ANDROID")]
        const EXTERNAL_FORMAT_DOWNSAMPLE_ANDROID = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_CustomResolve`](Extensions::EXT_CustomResolve)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RESOLVE_MODE_CUSTOM_BIT_EXT")]
        const CUSTOM_EXT = 32;
    }
}
/// [`VkResolveModeFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkResolveModeFlagBitsKHR.html)
///
#[doc(alias = "VkResolveModeFlagBitsKHR")]
pub type ResolveModeFlagsKHR = ResolveModeFlags;
impl ResolveModeFlags {
    /// [`VK_RESOLVE_MODE_NONE_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_RESOLVE_MODE_NONE_KHR.html)
    ///
    #[doc(alias = "VK_RESOLVE_MODE_NONE_KHR")]
    pub const NONE_KHR: Self = Self::NONE;
    /// [`VK_RESOLVE_MODE_SAMPLE_ZERO_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_RESOLVE_MODE_SAMPLE_ZERO_BIT_KHR.html)
    ///
    #[doc(alias = "VK_RESOLVE_MODE_SAMPLE_ZERO_BIT_KHR")]
    pub const SAMPLE_ZERO_KHR: Self = Self::SAMPLE_ZERO;
    /// [`VK_RESOLVE_MODE_AVERAGE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_RESOLVE_MODE_AVERAGE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_RESOLVE_MODE_AVERAGE_BIT_KHR")]
    pub const AVERAGE_KHR: Self = Self::AVERAGE;
    /// [`VK_RESOLVE_MODE_MIN_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_RESOLVE_MODE_MIN_BIT_KHR.html)
    ///
    #[doc(alias = "VK_RESOLVE_MODE_MIN_BIT_KHR")]
    pub const MIN_KHR: Self = Self::MIN;
    /// [`VK_RESOLVE_MODE_MAX_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_RESOLVE_MODE_MAX_BIT_KHR.html)
    ///
    #[doc(alias = "VK_RESOLVE_MODE_MAX_BIT_KHR")]
    pub const MAX_KHR: Self = Self::MAX;
}

bitflags::bitflags! {
    /// [`VkSemaphoreWaitFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreWaitFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.2 with appropriate features
    /// - Extension [`KHR_TimelineSemaphore`](Extensions::KHR_TimelineSemaphore)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkSemaphoreWaitFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SemaphoreWaitFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`KHR_TimelineSemaphore`](Extensions::KHR_TimelineSemaphore)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SEMAPHORE_WAIT_ANY_BIT")]
        const ANY = 1;
    }
}
/// [`VkSemaphoreWaitFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreWaitFlagBitsKHR.html)
///
#[doc(alias = "VkSemaphoreWaitFlagBitsKHR")]
pub type SemaphoreWaitFlagsKHR = SemaphoreWaitFlags;
impl SemaphoreWaitFlags {
    /// [`VK_SEMAPHORE_WAIT_ANY_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SEMAPHORE_WAIT_ANY_BIT_KHR.html)
    ///
    #[doc(alias = "VK_SEMAPHORE_WAIT_ANY_BIT_KHR")]
    pub const ANY_KHR: Self = Self::ANY;
}

bitflags::bitflags! {
    /// [`VkDescriptorBindingFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorBindingFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.2 with appropriate features
    /// - Extension [`EXT_DescriptorIndexing`](Extensions::EXT_DescriptorIndexing)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDescriptorBindingFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DescriptorBindingFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`EXT_DescriptorIndexing`](Extensions::EXT_DescriptorIndexing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT")]
        const UPDATE_AFTER_BIND = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`EXT_DescriptorIndexing`](Extensions::EXT_DescriptorIndexing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT")]
        const UPDATE_UNUSED_WHILE_PENDING = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`EXT_DescriptorIndexing`](Extensions::EXT_DescriptorIndexing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT")]
        const PARTIALLY_BOUND = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`EXT_DescriptorIndexing`](Extensions::EXT_DescriptorIndexing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT")]
        const VARIABLE_DESCRIPTOR_COUNT = 8;
    }
}
/// [`VkDescriptorBindingFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorBindingFlagBitsEXT.html)
///
#[doc(alias = "VkDescriptorBindingFlagBitsEXT")]
pub type DescriptorBindingFlagsEXT = DescriptorBindingFlags;
impl DescriptorBindingFlags {
    /// [`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT_EXT.html)
    ///
    #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT_EXT")]
    pub const UPDATE_AFTER_BIND_EXT: Self = Self::UPDATE_AFTER_BIND;
    /// [`VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT_EXT.html)
    ///
    #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT_EXT")]
    pub const UPDATE_UNUSED_WHILE_PENDING_EXT: Self = Self::UPDATE_UNUSED_WHILE_PENDING;
    /// [`VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT_EXT.html)
    ///
    #[doc(alias = "VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT_EXT")]
    pub const PARTIALLY_BOUND_EXT: Self = Self::PARTIALLY_BOUND;
    /// [`VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT_EXT.html)
    ///
    #[doc(alias = "VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT_EXT")]
    pub const VARIABLE_DESCRIPTOR_COUNT_EXT: Self = Self::VARIABLE_DESCRIPTOR_COUNT;
}

bitflags::bitflags! {
    /// [`VkToolPurposeFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkToolPurposeFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.3 with appropriate features
    /// - Extension [`EXT_ToolingInfo`](Extensions::EXT_ToolingInfo)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkToolPurposeFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ToolPurposeFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_ToolingInfo`](Extensions::EXT_ToolingInfo)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TOOL_PURPOSE_VALIDATION_BIT")]
        const VALIDATION = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_ToolingInfo`](Extensions::EXT_ToolingInfo)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TOOL_PURPOSE_PROFILING_BIT")]
        const PROFILING = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_ToolingInfo`](Extensions::EXT_ToolingInfo)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TOOL_PURPOSE_TRACING_BIT")]
        const TRACING = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_ToolingInfo`](Extensions::EXT_ToolingInfo)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT")]
        const ADDITIONAL_FEATURES = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_ToolingInfo`](Extensions::EXT_ToolingInfo)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT")]
        const MODIFYING_FEATURES = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_ToolingInfo`](Extensions::EXT_ToolingInfo)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TOOL_PURPOSE_DEBUG_REPORTING_BIT_EXT")]
        const DEBUG_REPORTING_EXT = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_ToolingInfo`](Extensions::EXT_ToolingInfo)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TOOL_PURPOSE_DEBUG_MARKERS_BIT_EXT")]
        const DEBUG_MARKERS_EXT = 64;
    }
}
/// [`VkToolPurposeFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkToolPurposeFlagBitsEXT.html)
///
#[doc(alias = "VkToolPurposeFlagBitsEXT")]
pub type ToolPurposeFlagsEXT = ToolPurposeFlags;
impl ToolPurposeFlags {
    /// [`VK_TOOL_PURPOSE_VALIDATION_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_TOOL_PURPOSE_VALIDATION_BIT_EXT.html)
    ///
    #[doc(alias = "VK_TOOL_PURPOSE_VALIDATION_BIT_EXT")]
    pub const VALIDATION_EXT: Self = Self::VALIDATION;
    /// [`VK_TOOL_PURPOSE_PROFILING_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_TOOL_PURPOSE_PROFILING_BIT_EXT.html)
    ///
    #[doc(alias = "VK_TOOL_PURPOSE_PROFILING_BIT_EXT")]
    pub const PROFILING_EXT: Self = Self::PROFILING;
    /// [`VK_TOOL_PURPOSE_TRACING_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_TOOL_PURPOSE_TRACING_BIT_EXT.html)
    ///
    #[doc(alias = "VK_TOOL_PURPOSE_TRACING_BIT_EXT")]
    pub const TRACING_EXT: Self = Self::TRACING;
    /// [`VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT_EXT.html)
    ///
    #[doc(alias = "VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT_EXT")]
    pub const ADDITIONAL_FEATURES_EXT: Self = Self::ADDITIONAL_FEATURES;
    /// [`VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT_EXT.html)
    ///
    #[doc(alias = "VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT_EXT")]
    pub const MODIFYING_FEATURES_EXT: Self = Self::MODIFYING_FEATURES;
}

bitflags::bitflags! {
    /// [`VkPrivateDataSlotCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlotCreateFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.3 with appropriate features
    /// - Extension [`EXT_PrivateData`](Extensions::EXT_PrivateData)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkPrivateDataSlotCreateFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PrivateDataSlotCreateFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_PrivateDataBaseHandle`](Extensions::NV_PrivateDataBaseHandle)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRIVATE_DATA_SLOT_CREATE_BASE_OBJECT_HANDLE_BIT_NV")]
        const BASE_OBJECT_HANDLE_NV = 1;
    }
}
/// [`VkPrivateDataSlotCreateFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlotCreateFlagBitsEXT.html)
///
#[doc(alias = "VkPrivateDataSlotCreateFlagBitsEXT")]
pub type PrivateDataSlotCreateFlagsEXT = PrivateDataSlotCreateFlags;

bitflags::bitflags! {
    /// [`VkPipelineStageFlagBits2`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineStageFlagBits2.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.3 with appropriate features
    /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkPipelineStageFlagBits2")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PipelineStageFlags2: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_NONE")]
        const NONE = 0;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT")]
        const TOP_OF_PIPE = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT")]
        const DRAW_INDIRECT = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT")]
        const VERTEX_INPUT = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT")]
        const VERTEX_SHADER = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT")]
        const TESSELLATION_CONTROL_SHADER = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT")]
        const TESSELLATION_EVALUATION_SHADER = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT")]
        const GEOMETRY_SHADER = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT")]
        const FRAGMENT_SHADER = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT")]
        const EARLY_FRAGMENT_TESTS = 256;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT")]
        const LATE_FRAGMENT_TESTS = 512;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT")]
        const COLOR_ATTACHMENT_OUTPUT = 1024;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT")]
        const COMPUTE_SHADER = 2048;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT")]
        const ALL_TRANSFER = 4096;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT")]
        const BOTTOM_OF_PIPE = 8192;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_HOST_BIT")]
        const HOST = 16384;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT")]
        const ALL_GRAPHICS = 32768;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT")]
        const ALL_COMMANDS = 65536;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_COPY_BIT")]
        const COPY = 4294967296;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_RESOLVE_BIT")]
        const RESOLVE = 8589934592;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_BLIT_BIT")]
        const BLIT = 17179869184;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_CLEAR_BIT")]
        const CLEAR = 34359738368;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT")]
        const INDEX_INPUT = 68719476736;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT")]
        const VERTEX_ATTRIBUTE_INPUT = 137438953472;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT")]
        const PRE_RASTERIZATION_SHADERS = 274877906944;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extensions::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR")]
        const VIDEO_DECODE_KHR = 67108864;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR")]
        const VIDEO_ENCODE_KHR = 134217728;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT")]
        const TRANSFORM_FEEDBACK_EXT = 16777216;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT")]
        const CONDITIONAL_RENDERING_EXT = 262144;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_EXT")]
        const COMMAND_PREPROCESS_EXT = 131072;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
        const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = 4194304;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR")]
        const ACCELERATION_STRUCTURE_BUILD_KHR = 33554432;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR")]
        const RAY_TRACING_SHADER_KHR = 2097152;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT")]
        const FRAGMENT_DENSITY_PROCESS_EXT = 8388608;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_EXT")]
        const TASK_SHADER_EXT = 524288;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_EXT")]
        const MESH_SHADER_EXT = 1048576;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`HUAWEI_SubpassShading`](Extensions::HUAWEI_SubpassShading)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_SUBPASS_SHADER_BIT_HUAWEI")]
        const SUBPASS_SHADER_HUAWEI = 549755813888;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`HUAWEI_InvocationMask`](Extensions::HUAWEI_InvocationMask)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI")]
        const INVOCATION_MASK_HUAWEI = 1099511627776;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_RayTracingMaintenance1`](Extensions::KHR_RayTracingMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_COPY_BIT_KHR")]
        const ACCELERATION_STRUCTURE_COPY_KHR = 268435456;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_OpacityMicromap`](Extensions::EXT_OpacityMicromap)
        /// - Extension [`KHR_OpacityMicromap`](Extensions::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_MICROMAP_BUILD_BIT_EXT")]
        const MICROMAP_BUILD_EXT = 1073741824;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`HUAWEI_ClusterCullingShader`](Extensions::HUAWEI_ClusterCullingShader)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_CLUSTER_CULLING_SHADER_BIT_HUAWEI")]
        const CLUSTER_CULLING_SHADER_HUAWEI = 2199023255552;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_OPTICAL_FLOW_BIT_NV")]
        const OPTICAL_FLOW_NV = 536870912;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_CooperativeVector`](Extensions::NV_CooperativeVector)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_CONVERT_COOPERATIVE_VECTOR_MATRIX_BIT_NV")]
        const CONVERT_COOPERATIVE_VECTOR_MATRIX_NV = 17592186044416;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraph`](Extensions::ARM_DataGraph)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_DATA_GRAPH_BIT_ARM")]
        const DATA_GRAPH_ARM = 4398046511104;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_CopyMemoryIndirect`](Extensions::KHR_CopyMemoryIndirect)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_COPY_INDIRECT_BIT_KHR")]
        const COPY_INDIRECT_KHR = 70368744177664;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MemoryDecompression`](Extensions::EXT_MemoryDecompression)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_MEMORY_DECOMPRESSION_BIT_EXT")]
        const MEMORY_DECOMPRESSION_EXT = 35184372088832;
    }
}
/// [`VkPipelineStageFlagBits2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineStageFlagBits2KHR.html)
///
#[doc(alias = "VkPipelineStageFlagBits2KHR")]
pub type PipelineStageFlags2KHR = PipelineStageFlags2;
impl PipelineStageFlags2 {
    /// [`VK_PIPELINE_STAGE_2_NONE_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_NONE_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_NONE_KHR")]
    pub const NONE_KHR: Self = Self::NONE;
    /// [`VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT_KHR")]
    pub const TOP_OF_PIPE_KHR: Self = Self::TOP_OF_PIPE;
    /// [`VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT_KHR")]
    pub const DRAW_INDIRECT_KHR: Self = Self::DRAW_INDIRECT;
    /// [`VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT_KHR")]
    pub const VERTEX_INPUT_KHR: Self = Self::VERTEX_INPUT;
    /// [`VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT_KHR")]
    pub const VERTEX_SHADER_KHR: Self = Self::VERTEX_SHADER;
    /// [`VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT_KHR")]
    pub const TESSELLATION_CONTROL_SHADER_KHR: Self = Self::TESSELLATION_CONTROL_SHADER;
    /// [`VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT_KHR")]
    pub const TESSELLATION_EVALUATION_SHADER_KHR: Self = Self::TESSELLATION_EVALUATION_SHADER;
    /// [`VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT_KHR")]
    pub const GEOMETRY_SHADER_KHR: Self = Self::GEOMETRY_SHADER;
    /// [`VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT_KHR")]
    pub const FRAGMENT_SHADER_KHR: Self = Self::FRAGMENT_SHADER;
    /// [`VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT_KHR")]
    pub const EARLY_FRAGMENT_TESTS_KHR: Self = Self::EARLY_FRAGMENT_TESTS;
    /// [`VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT_KHR")]
    pub const LATE_FRAGMENT_TESTS_KHR: Self = Self::LATE_FRAGMENT_TESTS;
    /// [`VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT_KHR")]
    pub const COLOR_ATTACHMENT_OUTPUT_KHR: Self = Self::COLOR_ATTACHMENT_OUTPUT;
    /// [`VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT_KHR")]
    pub const COMPUTE_SHADER_KHR: Self = Self::COMPUTE_SHADER;
    /// [`VK_PIPELINE_STAGE_2_TRANSFER_BIT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_TRANSFER_BIT.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_TRANSFER_BIT")]
    pub const TRANSFER: Self = Self::ALL_TRANSFER;
    /// [`VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT_KHR")]
    pub const ALL_TRANSFER_KHR: Self = Self::ALL_TRANSFER;
    /// [`VK_PIPELINE_STAGE_2_TRANSFER_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_TRANSFER_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_TRANSFER_BIT_KHR")]
    pub const TRANSFER_KHR: Self = Self::ALL_TRANSFER;
    /// [`VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT_KHR")]
    pub const BOTTOM_OF_PIPE_KHR: Self = Self::BOTTOM_OF_PIPE;
    /// [`VK_PIPELINE_STAGE_2_HOST_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_HOST_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_HOST_BIT_KHR")]
    pub const HOST_KHR: Self = Self::HOST;
    /// [`VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT_KHR")]
    pub const ALL_GRAPHICS_KHR: Self = Self::ALL_GRAPHICS;
    /// [`VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT_KHR")]
    pub const ALL_COMMANDS_KHR: Self = Self::ALL_COMMANDS;
    /// [`VK_PIPELINE_STAGE_2_COPY_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_COPY_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_COPY_BIT_KHR")]
    pub const COPY_KHR: Self = Self::COPY;
    /// [`VK_PIPELINE_STAGE_2_RESOLVE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_RESOLVE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_RESOLVE_BIT_KHR")]
    pub const RESOLVE_KHR: Self = Self::RESOLVE;
    /// [`VK_PIPELINE_STAGE_2_BLIT_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_BLIT_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_BLIT_BIT_KHR")]
    pub const BLIT_KHR: Self = Self::BLIT;
    /// [`VK_PIPELINE_STAGE_2_CLEAR_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_CLEAR_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_CLEAR_BIT_KHR")]
    pub const CLEAR_KHR: Self = Self::CLEAR;
    /// [`VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT_KHR")]
    pub const INDEX_INPUT_KHR: Self = Self::INDEX_INPUT;
    /// [`VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT_KHR")]
    pub const VERTEX_ATTRIBUTE_INPUT_KHR: Self = Self::VERTEX_ATTRIBUTE_INPUT;
    /// [`VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT_KHR")]
    pub const PRE_RASTERIZATION_SHADERS_KHR: Self = Self::PRE_RASTERIZATION_SHADERS;
    /// [`VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV")]
    pub const COMMAND_PREPROCESS_NV: Self = Self::COMMAND_PREPROCESS_EXT;
    /// [`VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV")]
    pub const SHADING_RATE_IMAGE_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
    /// [`VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_NV.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_NV")]
    pub const ACCELERATION_STRUCTURE_BUILD_NV: Self = Self::ACCELERATION_STRUCTURE_BUILD_KHR;
    /// [`VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_NV.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_NV")]
    pub const RAY_TRACING_SHADER_NV: Self = Self::RAY_TRACING_SHADER_KHR;
    /// [`VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV")]
    pub const TASK_SHADER_NV: Self = Self::TASK_SHADER_EXT;
    /// [`VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV")]
    pub const MESH_SHADER_NV: Self = Self::MESH_SHADER_EXT;
    /// [`VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI.html)
    ///
    #[doc(alias = "VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI")]
    pub const SUBPASS_SHADING_HUAWEI: Self = Self::SUBPASS_SHADER_HUAWEI;
}

bitflags::bitflags! {
    /// [`VkAccessFlagBits2`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlagBits2.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.3 with appropriate features
    /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkAccessFlagBits2")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct AccessFlags2: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_NONE")]
        const NONE = 0;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT")]
        const INDIRECT_COMMAND_READ = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_INDEX_READ_BIT")]
        const INDEX_READ = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT")]
        const VERTEX_ATTRIBUTE_READ = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_UNIFORM_READ_BIT")]
        const UNIFORM_READ = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT")]
        const INPUT_ATTACHMENT_READ = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_SHADER_READ_BIT")]
        const SHADER_READ = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_SHADER_WRITE_BIT")]
        const SHADER_WRITE = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT")]
        const COLOR_ATTACHMENT_READ = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT")]
        const COLOR_ATTACHMENT_WRITE = 256;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT")]
        const DEPTH_STENCIL_ATTACHMENT_READ = 512;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT")]
        const DEPTH_STENCIL_ATTACHMENT_WRITE = 1024;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_TRANSFER_READ_BIT")]
        const TRANSFER_READ = 2048;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_TRANSFER_WRITE_BIT")]
        const TRANSFER_WRITE = 4096;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_HOST_READ_BIT")]
        const HOST_READ = 8192;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_HOST_WRITE_BIT")]
        const HOST_WRITE = 16384;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_MEMORY_READ_BIT")]
        const MEMORY_READ = 32768;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_MEMORY_WRITE_BIT")]
        const MEMORY_WRITE = 65536;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_SHADER_SAMPLED_READ_BIT")]
        const SHADER_SAMPLED_READ = 4294967296;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_SHADER_STORAGE_READ_BIT")]
        const SHADER_STORAGE_READ = 8589934592;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT")]
        const SHADER_STORAGE_WRITE = 17179869184;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extensions::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_VIDEO_DECODE_READ_BIT_KHR")]
        const VIDEO_DECODE_READ_KHR = 34359738368;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extensions::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_VIDEO_DECODE_WRITE_BIT_KHR")]
        const VIDEO_DECODE_WRITE_KHR = 68719476736;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_SAMPLER_HEAP_READ_BIT_EXT")]
        const SAMPLER_HEAP_READ_EXT = 144115188075855872;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_RESOURCE_HEAP_READ_BIT_EXT")]
        const RESOURCE_HEAP_READ_EXT = 288230376151711744;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_VIDEO_ENCODE_READ_BIT_KHR")]
        const VIDEO_ENCODE_READ_KHR = 137438953472;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_VIDEO_ENCODE_WRITE_BIT_KHR")]
        const VIDEO_ENCODE_WRITE_KHR = 274877906944;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_TileShading`](Extensions::QCOM_TileShading)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_SHADER_TILE_ATTACHMENT_READ_BIT_QCOM")]
        const SHADER_TILE_ATTACHMENT_READ_QCOM = 2251799813685248;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_TileShading`](Extensions::QCOM_TileShading)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_SHADER_TILE_ATTACHMENT_WRITE_BIT_QCOM")]
        const SHADER_TILE_ATTACHMENT_WRITE_QCOM = 4503599627370496;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_TRANSFORM_FEEDBACK_WRITE_BIT_EXT")]
        const TRANSFORM_FEEDBACK_WRITE_EXT = 33554432;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT")]
        const TRANSFORM_FEEDBACK_COUNTER_READ_EXT = 67108864;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT")]
        const TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT = 134217728;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_CONDITIONAL_RENDERING_READ_BIT_EXT")]
        const CONDITIONAL_RENDERING_READ_EXT = 1048576;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_COMMAND_PREPROCESS_READ_BIT_EXT")]
        const COMMAND_PREPROCESS_READ_EXT = 131072;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_COMMAND_PREPROCESS_WRITE_BIT_EXT")]
        const COMMAND_PREPROCESS_WRITE_EXT = 262144;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR")]
        const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR = 8388608;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR")]
        const ACCELERATION_STRUCTURE_READ_KHR = 2097152;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_KHR")]
        const ACCELERATION_STRUCTURE_WRITE_KHR = 4194304;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_FRAGMENT_DENSITY_MAP_READ_BIT_EXT")]
        const FRAGMENT_DENSITY_MAP_READ_EXT = 16777216;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT")]
        const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT = 524288;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorBuffer`](Extensions::EXT_DescriptorBuffer)
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_DESCRIPTOR_BUFFER_READ_BIT_EXT")]
        const DESCRIPTOR_BUFFER_READ_EXT = 2199023255552;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`HUAWEI_InvocationMask`](Extensions::HUAWEI_InvocationMask)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_INVOCATION_MASK_READ_BIT_HUAWEI")]
        const INVOCATION_MASK_READ_HUAWEI = 549755813888;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_RayTracingMaintenance1`](Extensions::KHR_RayTracingMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_SHADER_BINDING_TABLE_READ_BIT_KHR")]
        const SHADER_BINDING_TABLE_READ_KHR = 1099511627776;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_OpacityMicromap`](Extensions::EXT_OpacityMicromap)
        /// - Extension [`KHR_OpacityMicromap`](Extensions::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_MICROMAP_READ_BIT_EXT")]
        const MICROMAP_READ_EXT = 17592186044416;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_OpacityMicromap`](Extensions::EXT_OpacityMicromap)
        /// - Extension [`KHR_OpacityMicromap`](Extensions::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_MICROMAP_WRITE_BIT_EXT")]
        const MICROMAP_WRITE_EXT = 35184372088832;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_OPTICAL_FLOW_READ_BIT_NV")]
        const OPTICAL_FLOW_READ_NV = 4398046511104;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_OPTICAL_FLOW_WRITE_BIT_NV")]
        const OPTICAL_FLOW_WRITE_NV = 8796093022208;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraph`](Extensions::ARM_DataGraph)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_DATA_GRAPH_READ_BIT_ARM")]
        const DATA_GRAPH_READ_ARM = 140737488355328;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraph`](Extensions::ARM_DataGraph)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_DATA_GRAPH_WRITE_BIT_ARM")]
        const DATA_GRAPH_WRITE_ARM = 281474976710656;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MemoryDecompression`](Extensions::EXT_MemoryDecompression)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_MEMORY_DECOMPRESSION_READ_BIT_EXT")]
        const MEMORY_DECOMPRESSION_READ_EXT = 36028797018963968;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MemoryDecompression`](Extensions::EXT_MemoryDecompression)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_MEMORY_DECOMPRESSION_WRITE_BIT_EXT")]
        const MEMORY_DECOMPRESSION_WRITE_EXT = 72057594037927936;
    }
}
/// [`VkAccessFlagBits2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlagBits2KHR.html)
///
#[doc(alias = "VkAccessFlagBits2KHR")]
pub type AccessFlags2KHR = AccessFlags2;
impl AccessFlags2 {
    /// [`VK_ACCESS_2_NONE_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_NONE_KHR.html)
    ///
    #[doc(alias = "VK_ACCESS_2_NONE_KHR")]
    pub const NONE_KHR: Self = Self::NONE;
    /// [`VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT_KHR.html)
    ///
    #[doc(alias = "VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT_KHR")]
    pub const INDIRECT_COMMAND_READ_KHR: Self = Self::INDIRECT_COMMAND_READ;
    /// [`VK_ACCESS_2_INDEX_READ_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_INDEX_READ_BIT_KHR.html)
    ///
    #[doc(alias = "VK_ACCESS_2_INDEX_READ_BIT_KHR")]
    pub const INDEX_READ_KHR: Self = Self::INDEX_READ;
    /// [`VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT_KHR.html)
    ///
    #[doc(alias = "VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT_KHR")]
    pub const VERTEX_ATTRIBUTE_READ_KHR: Self = Self::VERTEX_ATTRIBUTE_READ;
    /// [`VK_ACCESS_2_UNIFORM_READ_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_UNIFORM_READ_BIT_KHR.html)
    ///
    #[doc(alias = "VK_ACCESS_2_UNIFORM_READ_BIT_KHR")]
    pub const UNIFORM_READ_KHR: Self = Self::UNIFORM_READ;
    /// [`VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT_KHR.html)
    ///
    #[doc(alias = "VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT_KHR")]
    pub const INPUT_ATTACHMENT_READ_KHR: Self = Self::INPUT_ATTACHMENT_READ;
    /// [`VK_ACCESS_2_SHADER_READ_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_SHADER_READ_BIT_KHR.html)
    ///
    #[doc(alias = "VK_ACCESS_2_SHADER_READ_BIT_KHR")]
    pub const SHADER_READ_KHR: Self = Self::SHADER_READ;
    /// [`VK_ACCESS_2_SHADER_WRITE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_SHADER_WRITE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_ACCESS_2_SHADER_WRITE_BIT_KHR")]
    pub const SHADER_WRITE_KHR: Self = Self::SHADER_WRITE;
    /// [`VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT_KHR.html)
    ///
    #[doc(alias = "VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT_KHR")]
    pub const COLOR_ATTACHMENT_READ_KHR: Self = Self::COLOR_ATTACHMENT_READ;
    /// [`VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT_KHR")]
    pub const COLOR_ATTACHMENT_WRITE_KHR: Self = Self::COLOR_ATTACHMENT_WRITE;
    /// [`VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT_KHR.html)
    ///
    #[doc(alias = "VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT_KHR")]
    pub const DEPTH_STENCIL_ATTACHMENT_READ_KHR: Self = Self::DEPTH_STENCIL_ATTACHMENT_READ;
    /// [`VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT_KHR")]
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE_KHR: Self = Self::DEPTH_STENCIL_ATTACHMENT_WRITE;
    /// [`VK_ACCESS_2_TRANSFER_READ_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_TRANSFER_READ_BIT_KHR.html)
    ///
    #[doc(alias = "VK_ACCESS_2_TRANSFER_READ_BIT_KHR")]
    pub const TRANSFER_READ_KHR: Self = Self::TRANSFER_READ;
    /// [`VK_ACCESS_2_TRANSFER_WRITE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_TRANSFER_WRITE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_ACCESS_2_TRANSFER_WRITE_BIT_KHR")]
    pub const TRANSFER_WRITE_KHR: Self = Self::TRANSFER_WRITE;
    /// [`VK_ACCESS_2_HOST_READ_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_HOST_READ_BIT_KHR.html)
    ///
    #[doc(alias = "VK_ACCESS_2_HOST_READ_BIT_KHR")]
    pub const HOST_READ_KHR: Self = Self::HOST_READ;
    /// [`VK_ACCESS_2_HOST_WRITE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_HOST_WRITE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_ACCESS_2_HOST_WRITE_BIT_KHR")]
    pub const HOST_WRITE_KHR: Self = Self::HOST_WRITE;
    /// [`VK_ACCESS_2_MEMORY_READ_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_MEMORY_READ_BIT_KHR.html)
    ///
    #[doc(alias = "VK_ACCESS_2_MEMORY_READ_BIT_KHR")]
    pub const MEMORY_READ_KHR: Self = Self::MEMORY_READ;
    /// [`VK_ACCESS_2_MEMORY_WRITE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_MEMORY_WRITE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_ACCESS_2_MEMORY_WRITE_BIT_KHR")]
    pub const MEMORY_WRITE_KHR: Self = Self::MEMORY_WRITE;
    /// [`VK_ACCESS_2_SHADER_SAMPLED_READ_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_SHADER_SAMPLED_READ_BIT_KHR.html)
    ///
    #[doc(alias = "VK_ACCESS_2_SHADER_SAMPLED_READ_BIT_KHR")]
    pub const SHADER_SAMPLED_READ_KHR: Self = Self::SHADER_SAMPLED_READ;
    /// [`VK_ACCESS_2_SHADER_STORAGE_READ_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_SHADER_STORAGE_READ_BIT_KHR.html)
    ///
    #[doc(alias = "VK_ACCESS_2_SHADER_STORAGE_READ_BIT_KHR")]
    pub const SHADER_STORAGE_READ_KHR: Self = Self::SHADER_STORAGE_READ;
    /// [`VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT_KHR")]
    pub const SHADER_STORAGE_WRITE_KHR: Self = Self::SHADER_STORAGE_WRITE;
    /// [`VK_ACCESS_2_COMMAND_PREPROCESS_READ_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_COMMAND_PREPROCESS_READ_BIT_NV.html)
    ///
    #[doc(alias = "VK_ACCESS_2_COMMAND_PREPROCESS_READ_BIT_NV")]
    pub const COMMAND_PREPROCESS_READ_NV: Self = Self::COMMAND_PREPROCESS_READ_EXT;
    /// [`VK_ACCESS_2_COMMAND_PREPROCESS_WRITE_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_COMMAND_PREPROCESS_WRITE_BIT_NV.html)
    ///
    #[doc(alias = "VK_ACCESS_2_COMMAND_PREPROCESS_WRITE_BIT_NV")]
    pub const COMMAND_PREPROCESS_WRITE_NV: Self = Self::COMMAND_PREPROCESS_WRITE_EXT;
    /// [`VK_ACCESS_2_SHADING_RATE_IMAGE_READ_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_SHADING_RATE_IMAGE_READ_BIT_NV.html)
    ///
    #[doc(alias = "VK_ACCESS_2_SHADING_RATE_IMAGE_READ_BIT_NV")]
    pub const SHADING_RATE_IMAGE_READ_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR;
    /// [`VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_NV.html)
    ///
    #[doc(alias = "VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_NV")]
    pub const ACCELERATION_STRUCTURE_READ_NV: Self = Self::ACCELERATION_STRUCTURE_READ_KHR;
    /// [`VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_NV.html)
    ///
    #[doc(alias = "VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_NV")]
    pub const ACCELERATION_STRUCTURE_WRITE_NV: Self = Self::ACCELERATION_STRUCTURE_WRITE_KHR;
}

bitflags::bitflags! {
    /// [`VkSubmitFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubmitFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.3 with appropriate features
    /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkSubmitFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SubmitFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extensions::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBMIT_PROTECTED_BIT")]
        const PROTECTED = 1;
    }
}
/// [`VkSubmitFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubmitFlagBitsKHR.html)
///
#[doc(alias = "VkSubmitFlagBitsKHR")]
pub type SubmitFlagsKHR = SubmitFlags;
impl SubmitFlags {
    /// [`VK_SUBMIT_PROTECTED_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SUBMIT_PROTECTED_BIT_KHR.html)
    ///
    #[doc(alias = "VK_SUBMIT_PROTECTED_BIT_KHR")]
    pub const PROTECTED_KHR: Self = Self::PROTECTED;
}

bitflags::bitflags! {
    /// [`VkFormatFeatureFlagBits2`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlagBits2.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.3 with appropriate features
    /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkFormatFeatureFlagBits2")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct FormatFeatureFlags2: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT")]
        const SAMPLED_IMAGE = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT")]
        const STORAGE_IMAGE = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT")]
        const STORAGE_IMAGE_ATOMIC = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT")]
        const UNIFORM_TEXEL_BUFFER = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT")]
        const STORAGE_TEXEL_BUFFER = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT")]
        const STORAGE_TEXEL_BUFFER_ATOMIC = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT")]
        const VERTEX_BUFFER = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT")]
        const COLOR_ATTACHMENT = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT")]
        const COLOR_ATTACHMENT_BLEND = 256;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT")]
        const DEPTH_STENCIL_ATTACHMENT = 512;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_BLIT_SRC_BIT")]
        const BLIT_SRC = 1024;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_BLIT_DST_BIT")]
        const BLIT_DST = 2048;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT")]
        const SAMPLED_IMAGE_FILTER_LINEAR = 4096;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT")]
        const TRANSFER_SRC = 16384;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT")]
        const TRANSFER_DST = 32768;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT")]
        const SAMPLED_IMAGE_FILTER_MINMAX = 65536;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT")]
        const MIDPOINT_CHROMA_SAMPLES = 131072;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT")]
        const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER = 262144;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT")]
        const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER = 524288;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT")]
        const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT = 1048576;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT")]
        const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE = 2097152;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_DISJOINT_BIT")]
        const DISJOINT = 4194304;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT")]
        const COSITED_CHROMA_SAMPLES = 8388608;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT")]
        const STORAGE_READ_WITHOUT_FORMAT = 2147483648;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT")]
        const STORAGE_WRITE_WITHOUT_FORMAT = 4294967296;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT")]
        const SAMPLED_IMAGE_DEPTH_COMPARISON = 8589934592;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extensions::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT")]
        const SAMPLED_IMAGE_FILTER_CUBIC = 8192;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`EXT_HostImageCopy`](Extensions::EXT_HostImageCopy)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_HOST_IMAGE_TRANSFER_BIT")]
        const HOST_IMAGE_TRANSFER = 70368744177664;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extensions::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_DECODE_OUTPUT_BIT_KHR")]
        const VIDEO_DECODE_OUTPUT_KHR = 33554432;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extensions::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_DECODE_DPB_BIT_KHR")]
        const VIDEO_DECODE_DPB_KHR = 67108864;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_AccelerationStructure`](Extensions::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR")]
        const ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR = 536870912;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_FragmentDensityMap`](Extensions::EXT_FragmentDensityMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_FRAGMENT_DENSITY_MAP_BIT_EXT")]
        const FRAGMENT_DENSITY_MAP_EXT = 16777216;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_FragmentShadingRate`](Extensions::KHR_FragmentShadingRate)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
        const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = 1073741824;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_ENCODE_INPUT_BIT_KHR")]
        const VIDEO_ENCODE_INPUT_KHR = 134217728;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_ENCODE_DPB_BIT_KHR")]
        const VIDEO_ENCODE_DPB_KHR = 268435456;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_ImageProcessing3`](Extensions::QCOM_ImageProcessing3)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_BLOCK_MATCHING_SXD_BIT_QCOM")]
        const BLOCK_MATCHING_SXD_QCOM = 17592186044416;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracingLinearSweptSpheres`](Extensions::NV_RayTracingLinearSweptSpheres)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_ACCELERATION_STRUCTURE_RADIUS_BUFFER_BIT_NV")]
        const ACCELERATION_STRUCTURE_RADIUS_BUFFER_NV = 2251799813685248;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_LinearColorAttachment`](Extensions::NV_LinearColorAttachment)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV")]
        const LINEAR_COLOR_ATTACHMENT_NV = 274877906944;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_ImageProcessing`](Extensions::QCOM_ImageProcessing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_WEIGHT_IMAGE_BIT_QCOM")]
        const WEIGHT_IMAGE_QCOM = 17179869184;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_ImageProcessing`](Extensions::QCOM_ImageProcessing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_WEIGHT_SAMPLED_IMAGE_BIT_QCOM")]
        const WEIGHT_SAMPLED_IMAGE_QCOM = 34359738368;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_ImageProcessing`](Extensions::QCOM_ImageProcessing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_BLOCK_MATCHING_BIT_QCOM")]
        const BLOCK_MATCHING_QCOM = 68719476736;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_ImageProcessing`](Extensions::QCOM_ImageProcessing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_BOX_FILTER_SAMPLED_BIT_QCOM")]
        const BOX_FILTER_SAMPLED_QCOM = 137438953472;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_Tensors`](Extensions::ARM_Tensors)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_TENSOR_SHADER_BIT_ARM")]
        const TENSOR_SHADER_ARM = 549755813888;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_Tensors`](Extensions::ARM_Tensors)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_TENSOR_IMAGE_ALIASING_BIT_ARM")]
        const TENSOR_IMAGE_ALIASING_ARM = 8796093022208;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_OPTICAL_FLOW_IMAGE_BIT_NV")]
        const OPTICAL_FLOW_IMAGE_NV = 1099511627776;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_OPTICAL_FLOW_VECTOR_BIT_NV")]
        const OPTICAL_FLOW_VECTOR_NV = 2199023255552;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_OPTICAL_FLOW_COST_BIT_NV")]
        const OPTICAL_FLOW_COST_NV = 4398046511104;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraph`](Extensions::ARM_DataGraph)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_TENSOR_DATA_GRAPH_BIT_ARM")]
        const TENSOR_DATA_GRAPH_ARM = 281474976710656;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_CopyMemoryIndirect`](Extensions::KHR_CopyMemoryIndirect)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_COPY_IMAGE_INDIRECT_DST_BIT_KHR")]
        const COPY_IMAGE_INDIRECT_DST_KHR = 576460752303423488;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extensions::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR")]
        const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR = 562949953421312;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extensions::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR")]
        const VIDEO_ENCODE_EMPHASIS_MAP_KHR = 1125899906842624;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`IMG_FilterLinear2D`](Extensions::IMG_FilterLinear2D)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_2D_BIT_IMG")]
        const SAMPLED_IMAGE_FILTER_LINEAR_2D_IMG = 35184372088832;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance10`](Extensions::KHR_Maintenance10)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_DEPTH_COPY_ON_COMPUTE_QUEUE_BIT_KHR")]
        const DEPTH_COPY_ON_COMPUTE_QUEUE_KHR = 4503599627370496;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance10`](Extensions::KHR_Maintenance10)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_DEPTH_COPY_ON_TRANSFER_QUEUE_BIT_KHR")]
        const DEPTH_COPY_ON_TRANSFER_QUEUE_KHR = 9007199254740992;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance10`](Extensions::KHR_Maintenance10)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_STENCIL_COPY_ON_COMPUTE_QUEUE_BIT_KHR")]
        const STENCIL_COPY_ON_COMPUTE_QUEUE_KHR = 18014398509481984;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance10`](Extensions::KHR_Maintenance10)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_STENCIL_COPY_ON_TRANSFER_QUEUE_BIT_KHR")]
        const STENCIL_COPY_ON_TRANSFER_QUEUE_KHR = 36028797018963968;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_DATA_GRAPH_OPTICAL_FLOW_IMAGE_BIT_ARM")]
        const DATA_GRAPH_OPTICAL_FLOW_IMAGE_ARM = 72057594037927936;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_DATA_GRAPH_OPTICAL_FLOW_VECTOR_BIT_ARM")]
        const DATA_GRAPH_OPTICAL_FLOW_VECTOR_ARM = 144115188075855872;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_DATA_GRAPH_OPTICAL_FLOW_COST_BIT_ARM")]
        const DATA_GRAPH_OPTICAL_FLOW_COST_ARM = 288230376151711744;
    }
}
/// [`VkFormatFeatureFlagBits2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlagBits2KHR.html)
///
#[doc(alias = "VkFormatFeatureFlagBits2KHR")]
pub type FormatFeatureFlags2KHR = FormatFeatureFlags2;
impl FormatFeatureFlags2 {
    /// [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT_KHR")]
    pub const SAMPLED_IMAGE_KHR: Self = Self::SAMPLED_IMAGE;
    /// [`VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT_KHR")]
    pub const STORAGE_IMAGE_KHR: Self = Self::STORAGE_IMAGE;
    /// [`VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT_KHR")]
    pub const STORAGE_IMAGE_ATOMIC_KHR: Self = Self::STORAGE_IMAGE_ATOMIC;
    /// [`VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT_KHR")]
    pub const UNIFORM_TEXEL_BUFFER_KHR: Self = Self::UNIFORM_TEXEL_BUFFER;
    /// [`VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT_KHR")]
    pub const STORAGE_TEXEL_BUFFER_KHR: Self = Self::STORAGE_TEXEL_BUFFER;
    /// [`VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT_KHR")]
    pub const STORAGE_TEXEL_BUFFER_ATOMIC_KHR: Self = Self::STORAGE_TEXEL_BUFFER_ATOMIC;
    /// [`VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT_KHR")]
    pub const VERTEX_BUFFER_KHR: Self = Self::VERTEX_BUFFER;
    /// [`VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT_KHR")]
    pub const COLOR_ATTACHMENT_KHR: Self = Self::COLOR_ATTACHMENT;
    /// [`VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT_KHR")]
    pub const COLOR_ATTACHMENT_BLEND_KHR: Self = Self::COLOR_ATTACHMENT_BLEND;
    /// [`VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT_KHR")]
    pub const DEPTH_STENCIL_ATTACHMENT_KHR: Self = Self::DEPTH_STENCIL_ATTACHMENT;
    /// [`VK_FORMAT_FEATURE_2_BLIT_SRC_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_BLIT_SRC_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_BLIT_SRC_BIT_KHR")]
    pub const BLIT_SRC_KHR: Self = Self::BLIT_SRC;
    /// [`VK_FORMAT_FEATURE_2_BLIT_DST_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_BLIT_DST_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_BLIT_DST_BIT_KHR")]
    pub const BLIT_DST_KHR: Self = Self::BLIT_DST;
    /// [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT_KHR")]
    pub const SAMPLED_IMAGE_FILTER_LINEAR_KHR: Self = Self::SAMPLED_IMAGE_FILTER_LINEAR;
    /// [`VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT_KHR")]
    pub const TRANSFER_SRC_KHR: Self = Self::TRANSFER_SRC;
    /// [`VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT_KHR")]
    pub const TRANSFER_DST_KHR: Self = Self::TRANSFER_DST;
    /// [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT_KHR")]
    pub const SAMPLED_IMAGE_FILTER_MINMAX_KHR: Self = Self::SAMPLED_IMAGE_FILTER_MINMAX;
    /// [`VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT_KHR")]
    pub const MIDPOINT_CHROMA_SAMPLES_KHR: Self = Self::MIDPOINT_CHROMA_SAMPLES;
    /// [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR")]
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER;
    /// [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR.html)
    ///
    #[doc(
        alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR"
    )]
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER;
    /// [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR.html)
    ///
    #[doc(
        alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR"
    )]
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT;
    /// [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR.html)
    ///
    #[doc(
        alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR"
    )]
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE;
    /// [`VK_FORMAT_FEATURE_2_DISJOINT_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_DISJOINT_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_DISJOINT_BIT_KHR")]
    pub const DISJOINT_KHR: Self = Self::DISJOINT;
    /// [`VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT_KHR")]
    pub const COSITED_CHROMA_SAMPLES_KHR: Self = Self::COSITED_CHROMA_SAMPLES;
    /// [`VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT_KHR")]
    pub const STORAGE_READ_WITHOUT_FORMAT_KHR: Self = Self::STORAGE_READ_WITHOUT_FORMAT;
    /// [`VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT_KHR")]
    pub const STORAGE_WRITE_WITHOUT_FORMAT_KHR: Self = Self::STORAGE_WRITE_WITHOUT_FORMAT;
    /// [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT_KHR")]
    pub const SAMPLED_IMAGE_DEPTH_COMPARISON_KHR: Self = Self::SAMPLED_IMAGE_DEPTH_COMPARISON;
    /// [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT")]
    pub const SAMPLED_IMAGE_FILTER_CUBIC_EXT: Self = Self::SAMPLED_IMAGE_FILTER_CUBIC;
    /// [`VK_FORMAT_FEATURE_2_HOST_IMAGE_TRANSFER_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FORMAT_FEATURE_2_HOST_IMAGE_TRANSFER_BIT_EXT.html)
    ///
    #[doc(alias = "VK_FORMAT_FEATURE_2_HOST_IMAGE_TRANSFER_BIT_EXT")]
    pub const HOST_IMAGE_TRANSFER_EXT: Self = Self::HOST_IMAGE_TRANSFER;
}

bitflags::bitflags! {
    /// [`VkPipelineCreationFeedbackFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreationFeedbackFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.3 with appropriate features
    /// - Extension [`EXT_PipelineCreationFeedback`](Extensions::EXT_PipelineCreationFeedback)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkPipelineCreationFeedbackFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PipelineCreationFeedbackFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_PipelineCreationFeedback`](Extensions::EXT_PipelineCreationFeedback)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT")]
        const VALID = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_PipelineCreationFeedback`](Extensions::EXT_PipelineCreationFeedback)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT")]
        const APPLICATION_PIPELINE_CACHE_HIT = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_PipelineCreationFeedback`](Extensions::EXT_PipelineCreationFeedback)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT")]
        const BASE_PIPELINE_ACCELERATION = 4;
    }
}
/// [`VkPipelineCreationFeedbackFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreationFeedbackFlagBitsEXT.html)
///
#[doc(alias = "VkPipelineCreationFeedbackFlagBitsEXT")]
pub type PipelineCreationFeedbackFlagsEXT = PipelineCreationFeedbackFlags;
impl PipelineCreationFeedbackFlags {
    /// [`VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT_EXT.html)
    ///
    #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT_EXT")]
    pub const VALID_EXT: Self = Self::VALID;
    /// [`VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT_EXT.html)
    ///
    #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT_EXT")]
    pub const APPLICATION_PIPELINE_CACHE_HIT_EXT: Self = Self::APPLICATION_PIPELINE_CACHE_HIT;
    /// [`VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT_EXT.html)
    ///
    #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT_EXT")]
    pub const BASE_PIPELINE_ACCELERATION_EXT: Self = Self::BASE_PIPELINE_ACCELERATION;
}

bitflags::bitflags! {
    /// [`VkRenderingFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.3 with appropriate features
    /// - Extension [`KHR_DynamicRendering`](Extensions::KHR_DynamicRendering)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkRenderingFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct RenderingFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_DynamicRendering`](Extensions::KHR_DynamicRendering)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT")]
        const CONTENTS_SECONDARY_COMMAND_BUFFERS = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_DynamicRendering`](Extensions::KHR_DynamicRendering)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDERING_SUSPENDING_BIT")]
        const SUSPENDING = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_DynamicRendering`](Extensions::KHR_DynamicRendering)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDERING_RESUMING_BIT")]
        const RESUMING = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_LegacyDithering`](Extensions::EXT_LegacyDithering)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDERING_ENABLE_LEGACY_DITHERING_BIT_EXT")]
        const ENABLE_LEGACY_DITHERING_EXT = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance7`](Extensions::KHR_Maintenance7)
        /// - Extension [`EXT_NestedCommandBuffer`](Extensions::EXT_NestedCommandBuffer)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDERING_CONTENTS_INLINE_BIT_KHR")]
        const CONTENTS_INLINE_KHR = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_FragmentDensityMapLayered`](Extensions::VALVE_FragmentDensityMapLayered)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDERING_PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE")]
        const PER_LAYER_FRAGMENT_DENSITY_VALVE = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_CustomResolve`](Extensions::EXT_CustomResolve)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDERING_FRAGMENT_REGION_BIT_EXT")]
        const FRAGMENT_REGION_EXT = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_CustomResolve`](Extensions::EXT_CustomResolve)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDERING_CUSTOM_RESOLVE_BIT_EXT")]
        const CUSTOM_RESOLVE_EXT = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance10`](Extensions::KHR_Maintenance10)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDERING_LOCAL_READ_CONCURRENT_ACCESS_CONTROL_BIT_KHR")]
        const LOCAL_READ_CONCURRENT_ACCESS_CONTROL_KHR = 256;
    }
}
/// [`VkRenderingFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingFlagBitsKHR.html)
///
#[doc(alias = "VkRenderingFlagBitsKHR")]
pub type RenderingFlagsKHR = RenderingFlags;
impl RenderingFlags {
    /// [`VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT_KHR.html)
    ///
    #[doc(alias = "VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT_KHR")]
    pub const CONTENTS_SECONDARY_COMMAND_BUFFERS_KHR: Self =
        Self::CONTENTS_SECONDARY_COMMAND_BUFFERS;
    /// [`VK_RENDERING_SUSPENDING_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_RENDERING_SUSPENDING_BIT_KHR.html)
    ///
    #[doc(alias = "VK_RENDERING_SUSPENDING_BIT_KHR")]
    pub const SUSPENDING_KHR: Self = Self::SUSPENDING;
    /// [`VK_RENDERING_RESUMING_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_RENDERING_RESUMING_BIT_KHR.html)
    ///
    #[doc(alias = "VK_RENDERING_RESUMING_BIT_KHR")]
    pub const RESUMING_KHR: Self = Self::RESUMING;
    /// [`VK_RENDERING_CONTENTS_INLINE_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_RENDERING_CONTENTS_INLINE_BIT_EXT.html)
    ///
    #[doc(alias = "VK_RENDERING_CONTENTS_INLINE_BIT_EXT")]
    pub const CONTENTS_INLINE_EXT: Self = Self::CONTENTS_INLINE_KHR;
}

bitflags::bitflags! {
    /// [`VkMemoryUnmapFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryUnmapFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.4 with appropriate features
    /// - Extension [`KHR_MapMemory2`](Extensions::KHR_MapMemory2)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkMemoryUnmapFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct MemoryUnmapFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MapMemoryPlaced`](Extensions::EXT_MapMemoryPlaced)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MEMORY_UNMAP_RESERVE_BIT_EXT")]
        const RESERVE_EXT = 1;
    }
}
/// [`VkMemoryUnmapFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryUnmapFlagBitsKHR.html)
///
#[doc(alias = "VkMemoryUnmapFlagBitsKHR")]
pub type MemoryUnmapFlagsKHR = MemoryUnmapFlags;

bitflags::bitflags! {
    /// [`VkBufferUsageFlagBits2`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlagBits2.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.4 with appropriate features
    /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
    /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkBufferUsageFlagBits2")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct BufferUsageFlags2: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_TRANSFER_SRC_BIT")]
        const TRANSFER_SRC = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_TRANSFER_DST_BIT")]
        const TRANSFER_DST = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_UNIFORM_TEXEL_BUFFER_BIT")]
        const UNIFORM_TEXEL_BUFFER = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_STORAGE_TEXEL_BUFFER_BIT")]
        const STORAGE_TEXEL_BUFFER = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_UNIFORM_BUFFER_BIT")]
        const UNIFORM_BUFFER = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_STORAGE_BUFFER_BIT")]
        const STORAGE_BUFFER = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_INDEX_BUFFER_BIT")]
        const INDEX_BUFFER = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_VERTEX_BUFFER_BIT")]
        const VERTEX_BUFFER = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_INDIRECT_BUFFER_BIT")]
        const INDIRECT_BUFFER = 256;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_SHADER_DEVICE_ADDRESS_BIT")]
        const SHADER_DEVICE_ADDRESS = 131072;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMDX_ShaderEnqueue`](Extensions::AMDX_ShaderEnqueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_EXECUTION_GRAPH_SCRATCH_BIT_AMDX")]
        const EXECUTION_GRAPH_SCRATCH_AMDX = 33554432;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_DESCRIPTOR_HEAP_BIT_EXT")]
        const DESCRIPTOR_HEAP_EXT = 268435456;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        /// - Extension [`EXT_OpacityMicromap`](Extensions::EXT_OpacityMicromap)
        /// - Extension [`KHR_OpacityMicromap`](Extensions::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_MICROMAP_BUILD_INPUT_READ_ONLY_BIT_EXT")]
        const MICROMAP_BUILD_INPUT_READ_ONLY_EXT = 8388608;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        /// - Extension [`EXT_OpacityMicromap`](Extensions::EXT_OpacityMicromap)
        /// - Extension [`KHR_OpacityMicromap`](Extensions::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_MICROMAP_STORAGE_BIT_EXT")]
        const MICROMAP_STORAGE_EXT = 16777216;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_CONDITIONAL_RENDERING_BIT_EXT")]
        const CONDITIONAL_RENDERING_EXT = 512;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_SHADER_BINDING_TABLE_BIT_KHR")]
        const SHADER_BINDING_TABLE_KHR = 1024;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_TRANSFORM_FEEDBACK_BUFFER_BIT_EXT")]
        const TRANSFORM_FEEDBACK_BUFFER_EXT = 2048;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT")]
        const TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT = 4096;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_VIDEO_DECODE_SRC_BIT_KHR")]
        const VIDEO_DECODE_SRC_KHR = 8192;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_VIDEO_DECODE_DST_BIT_KHR")]
        const VIDEO_DECODE_DST_KHR = 16384;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_VIDEO_ENCODE_DST_BIT_KHR")]
        const VIDEO_ENCODE_DST_KHR = 32768;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_VIDEO_ENCODE_SRC_BIT_KHR")]
        const VIDEO_ENCODE_SRC_KHR = 65536;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR")]
        const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR = 524288;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_ACCELERATION_STRUCTURE_STORAGE_BIT_KHR")]
        const ACCELERATION_STRUCTURE_STORAGE_KHR = 1048576;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_SAMPLER_DESCRIPTOR_BUFFER_BIT_EXT")]
        const SAMPLER_DESCRIPTOR_BUFFER_EXT = 2097152;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_RESOURCE_DESCRIPTOR_BUFFER_BIT_EXT")]
        const RESOURCE_DESCRIPTOR_BUFFER_EXT = 4194304;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_BIT_EXT")]
        const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT = 67108864;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMDX_DenseGeometryFormat`](Extensions::AMDX_DenseGeometryFormat)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_COMPRESSED_DATA_DGF1_BIT_AMDX")]
        const COMPRESSED_DATA_DGF1_AMDX = 8589934592;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraph`](Extensions::ARM_DataGraph)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_DATA_GRAPH_FOREIGN_DESCRIPTOR_BIT_ARM")]
        const DATA_GRAPH_FOREIGN_DESCRIPTOR_ARM = 536870912;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_TileMemoryHeap`](Extensions::QCOM_TileMemoryHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_TILE_MEMORY_BIT_QCOM")]
        const TILE_MEMORY_QCOM = 134217728;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MemoryDecompression`](Extensions::EXT_MemoryDecompression)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_MEMORY_DECOMPRESSION_BIT_EXT")]
        const MEMORY_DECOMPRESSION_EXT = 4294967296;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DeviceGeneratedCommands`](Extensions::EXT_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_PREPROCESS_BUFFER_BIT_EXT")]
        const PREPROCESS_BUFFER_EXT = 2147483648;
    }
}
/// [`VkBufferUsageFlagBits2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlagBits2KHR.html)
///
#[doc(alias = "VkBufferUsageFlagBits2KHR")]
pub type BufferUsageFlags2KHR = BufferUsageFlags2;
impl BufferUsageFlags2 {
    /// [`VK_BUFFER_USAGE_2_TRANSFER_SRC_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUFFER_USAGE_2_TRANSFER_SRC_BIT_KHR.html)
    ///
    #[doc(alias = "VK_BUFFER_USAGE_2_TRANSFER_SRC_BIT_KHR")]
    pub const TRANSFER_SRC_KHR: Self = Self::TRANSFER_SRC;
    /// [`VK_BUFFER_USAGE_2_TRANSFER_DST_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUFFER_USAGE_2_TRANSFER_DST_BIT_KHR.html)
    ///
    #[doc(alias = "VK_BUFFER_USAGE_2_TRANSFER_DST_BIT_KHR")]
    pub const TRANSFER_DST_KHR: Self = Self::TRANSFER_DST;
    /// [`VK_BUFFER_USAGE_2_UNIFORM_TEXEL_BUFFER_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUFFER_USAGE_2_UNIFORM_TEXEL_BUFFER_BIT_KHR.html)
    ///
    #[doc(alias = "VK_BUFFER_USAGE_2_UNIFORM_TEXEL_BUFFER_BIT_KHR")]
    pub const UNIFORM_TEXEL_BUFFER_KHR: Self = Self::UNIFORM_TEXEL_BUFFER;
    /// [`VK_BUFFER_USAGE_2_STORAGE_TEXEL_BUFFER_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUFFER_USAGE_2_STORAGE_TEXEL_BUFFER_BIT_KHR.html)
    ///
    #[doc(alias = "VK_BUFFER_USAGE_2_STORAGE_TEXEL_BUFFER_BIT_KHR")]
    pub const STORAGE_TEXEL_BUFFER_KHR: Self = Self::STORAGE_TEXEL_BUFFER;
    /// [`VK_BUFFER_USAGE_2_UNIFORM_BUFFER_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUFFER_USAGE_2_UNIFORM_BUFFER_BIT_KHR.html)
    ///
    #[doc(alias = "VK_BUFFER_USAGE_2_UNIFORM_BUFFER_BIT_KHR")]
    pub const UNIFORM_BUFFER_KHR: Self = Self::UNIFORM_BUFFER;
    /// [`VK_BUFFER_USAGE_2_STORAGE_BUFFER_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUFFER_USAGE_2_STORAGE_BUFFER_BIT_KHR.html)
    ///
    #[doc(alias = "VK_BUFFER_USAGE_2_STORAGE_BUFFER_BIT_KHR")]
    pub const STORAGE_BUFFER_KHR: Self = Self::STORAGE_BUFFER;
    /// [`VK_BUFFER_USAGE_2_INDEX_BUFFER_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUFFER_USAGE_2_INDEX_BUFFER_BIT_KHR.html)
    ///
    #[doc(alias = "VK_BUFFER_USAGE_2_INDEX_BUFFER_BIT_KHR")]
    pub const INDEX_BUFFER_KHR: Self = Self::INDEX_BUFFER;
    /// [`VK_BUFFER_USAGE_2_VERTEX_BUFFER_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUFFER_USAGE_2_VERTEX_BUFFER_BIT_KHR.html)
    ///
    #[doc(alias = "VK_BUFFER_USAGE_2_VERTEX_BUFFER_BIT_KHR")]
    pub const VERTEX_BUFFER_KHR: Self = Self::VERTEX_BUFFER;
    /// [`VK_BUFFER_USAGE_2_INDIRECT_BUFFER_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUFFER_USAGE_2_INDIRECT_BUFFER_BIT_KHR.html)
    ///
    #[doc(alias = "VK_BUFFER_USAGE_2_INDIRECT_BUFFER_BIT_KHR")]
    pub const INDIRECT_BUFFER_KHR: Self = Self::INDIRECT_BUFFER;
    /// [`VK_BUFFER_USAGE_2_SHADER_DEVICE_ADDRESS_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUFFER_USAGE_2_SHADER_DEVICE_ADDRESS_BIT_KHR.html)
    ///
    #[doc(alias = "VK_BUFFER_USAGE_2_SHADER_DEVICE_ADDRESS_BIT_KHR")]
    pub const SHADER_DEVICE_ADDRESS_KHR: Self = Self::SHADER_DEVICE_ADDRESS;
    /// [`VK_BUFFER_USAGE_2_RAY_TRACING_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUFFER_USAGE_2_RAY_TRACING_BIT_NV.html)
    ///
    #[doc(alias = "VK_BUFFER_USAGE_2_RAY_TRACING_BIT_NV")]
    pub const RAY_TRACING_NV: Self = Self::SHADER_BINDING_TABLE_KHR;
}

bitflags::bitflags! {
    /// [`VkHostImageCopyFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkHostImageCopyFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.4 with appropriate features
    /// - Extension [`EXT_HostImageCopy`](Extensions::EXT_HostImageCopy)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkHostImageCopyFlagBits")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct HostImageCopyFlags: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`EXT_HostImageCopy`](Extensions::EXT_HostImageCopy)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_HOST_IMAGE_COPY_MEMCPY_BIT")]
        const MEMCPY = 1;
    }
}
/// [`VkHostImageCopyFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkHostImageCopyFlagBitsEXT.html)
///
#[doc(alias = "VkHostImageCopyFlagBitsEXT")]
pub type HostImageCopyFlagsEXT = HostImageCopyFlags;
impl HostImageCopyFlags {
    /// [`VK_HOST_IMAGE_COPY_MEMCPY_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_HOST_IMAGE_COPY_MEMCPY_BIT_EXT.html)
    ///
    #[doc(alias = "VK_HOST_IMAGE_COPY_MEMCPY_BIT_EXT")]
    pub const MEMCPY_EXT: Self = Self::MEMCPY;
}

bitflags::bitflags! {
    /// [`VkPipelineCreateFlagBits2`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlagBits2.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.4 with appropriate features
    /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
    /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkPipelineCreateFlagBits2")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PipelineCreateFlags2: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_DISABLE_OPTIMIZATION_BIT")]
        const DISABLE_OPTIMIZATION = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_ALLOW_DERIVATIVES_BIT")]
        const ALLOW_DERIVATIVES = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_DERIVATIVE_BIT")]
        const DERIVATIVE = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_VIEW_INDEX_FROM_DEVICE_INDEX_BIT")]
        const VIEW_INDEX_FROM_DEVICE_INDEX = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_DISPATCH_BASE_BIT")]
        const DISPATCH_BASE = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT")]
        const FAIL_ON_PIPELINE_COMPILE_REQUIRED = 256;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_EARLY_RETURN_ON_FAILURE_BIT")]
        const EARLY_RETURN_ON_FAILURE = 512;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_NO_PROTECTED_ACCESS_BIT")]
        const NO_PROTECTED_ACCESS = 134217728;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_PROTECTED_ACCESS_ONLY_BIT")]
        const PROTECTED_ACCESS_ONLY = 1073741824;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMDX_ShaderEnqueue`](Extensions::AMDX_ShaderEnqueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_EXECUTION_GRAPH_BIT_AMDX")]
        const EXECUTION_GRAPH_AMDX = 4294967296;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_DESCRIPTOR_HEAP_BIT_EXT")]
        const DESCRIPTOR_HEAP_EXT = 68719476736;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracingLinearSweptSpheres`](Extensions::NV_RayTracingLinearSweptSpheres)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_ALLOW_SPHERES_AND_LINEAR_SWEPT_SPHERES_BIT_NV")]
        const RAY_TRACING_ALLOW_SPHERES_AND_LINEAR_SWEPT_SPHERES_NV = 8589934592;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_LegacyDithering`](Extensions::EXT_LegacyDithering)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_ENABLE_LEGACY_DITHERING_BIT_EXT")]
        const ENABLE_LEGACY_DITHERING_EXT = 17179869184;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_DEFER_COMPILE_BIT_NV")]
        const DEFER_COMPILE_NV = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_CAPTURE_STATISTICS_BIT_KHR")]
        const CAPTURE_STATISTICS_KHR = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR")]
        const CAPTURE_INTERNAL_REPRESENTATIONS_KHR = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_LINK_TIME_OPTIMIZATION_BIT_EXT")]
        const LINK_TIME_OPTIMIZATION_EXT = 1024;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RETAIN_LINK_TIME_OPTIMIZATION_INFO_BIT_EXT")]
        const RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT = 8388608;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_LIBRARY_BIT_KHR")]
        const LIBRARY_KHR = 2048;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR")]
        const RAY_TRACING_SKIP_TRIANGLES_KHR = 4096;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_SKIP_AABBS_BIT_KHR")]
        const RAY_TRACING_SKIP_AABBS_KHR = 8192;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR")]
        const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR = 16384;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR")]
        const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR = 32768;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR")]
        const RAY_TRACING_NO_NULL_MISS_SHADERS_KHR = 65536;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR")]
        const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR = 131072;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR")]
        const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR = 524288;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_INDIRECT_BINDABLE_BIT_NV")]
        const INDIRECT_BINDABLE_NV = 262144;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_ALLOW_MOTION_BIT_NV")]
        const RAY_TRACING_ALLOW_MOTION_NV = 1048576;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
        const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = 2097152;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT")]
        const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT = 4194304;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_COLOR_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT")]
        const COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT = 33554432;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT")]
        const DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT = 67108864;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_DISPLACEMENT_MICROMAP_BIT_NV")]
        const RAY_TRACING_DISPLACEMENT_MICROMAP_NV = 268435456;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_DESCRIPTOR_BUFFER_BIT_EXT")]
        const DESCRIPTOR_BUFFER_EXT = 536870912;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        /// - Extension [`ARM_PipelineOpacityMicromap`](Extensions::ARM_PipelineOpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_DISALLOW_OPACITY_MICROMAP_BIT_ARM")]
        const DISALLOW_OPACITY_MICROMAP_ARM = 137438953472;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`ARM_ShaderInstrumentation`](Extensions::ARM_ShaderInstrumentation)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_INSTRUMENT_SHADERS_BIT_ARM")]
        const INSTRUMENT_SHADERS_ARM = 549755813888;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_PipelineBinary`](Extensions::KHR_PipelineBinary)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_CAPTURE_DATA_BIT_KHR")]
        const CAPTURE_DATA_KHR = 2147483648;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DeviceGeneratedCommands`](Extensions::EXT_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_INDIRECT_BINDABLE_BIT_EXT")]
        const INDIRECT_BINDABLE_EXT = 274877906944;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_FragmentDensityMapLayered`](Extensions::VALVE_FragmentDensityMapLayered)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE")]
        const PER_LAYER_FRAGMENT_DENSITY_VALVE = 1099511627776;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_OpacityMicromap`](Extensions::KHR_OpacityMicromap)
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        /// - Extension [`EXT_OpacityMicromap`](Extensions::EXT_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_OPACITY_MICROMAP_BIT_KHR")]
        const RAY_TRACING_OPACITY_MICROMAP_KHR = 16777216;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_OpacityMicromap`](Extensions::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_OPACITY_MICROMAP_DISALLOW_MIXED_SPECIAL_INDEX_BIT_KHR")]
        const OPACITY_MICROMAP_DISALLOW_MIXED_SPECIAL_INDEX_KHR = 2199023255552;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_Shader64BitIndexing`](Extensions::EXT_Shader64BitIndexing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_64_BIT_INDEXING_BIT_EXT")]
        const _64_INDEXING_EXT = 8796093022208;
    }
}
/// [`VkPipelineCreateFlagBits2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlagBits2KHR.html)
///
#[doc(alias = "VkPipelineCreateFlagBits2KHR")]
pub type PipelineCreateFlags2KHR = PipelineCreateFlags2;
impl PipelineCreateFlags2 {
    /// [`VK_PIPELINE_CREATE_2_DISABLE_OPTIMIZATION_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_CREATE_2_DISABLE_OPTIMIZATION_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_CREATE_2_DISABLE_OPTIMIZATION_BIT_KHR")]
    pub const DISABLE_OPTIMIZATION_KHR: Self = Self::DISABLE_OPTIMIZATION;
    /// [`VK_PIPELINE_CREATE_2_ALLOW_DERIVATIVES_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_CREATE_2_ALLOW_DERIVATIVES_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_CREATE_2_ALLOW_DERIVATIVES_BIT_KHR")]
    pub const ALLOW_DERIVATIVES_KHR: Self = Self::ALLOW_DERIVATIVES;
    /// [`VK_PIPELINE_CREATE_2_DERIVATIVE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_CREATE_2_DERIVATIVE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_CREATE_2_DERIVATIVE_BIT_KHR")]
    pub const DERIVATIVE_KHR: Self = Self::DERIVATIVE;
    /// [`VK_PIPELINE_CREATE_2_VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_CREATE_2_VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_CREATE_2_VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR")]
    pub const VIEW_INDEX_FROM_DEVICE_INDEX_KHR: Self = Self::VIEW_INDEX_FROM_DEVICE_INDEX;
    /// [`VK_PIPELINE_CREATE_2_DISPATCH_BASE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_CREATE_2_DISPATCH_BASE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_CREATE_2_DISPATCH_BASE_BIT_KHR")]
    pub const DISPATCH_BASE_KHR: Self = Self::DISPATCH_BASE;
    /// [`VK_PIPELINE_CREATE_2_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_CREATE_2_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_CREATE_2_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT_KHR")]
    pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED_KHR: Self = Self::FAIL_ON_PIPELINE_COMPILE_REQUIRED;
    /// [`VK_PIPELINE_CREATE_2_EARLY_RETURN_ON_FAILURE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_CREATE_2_EARLY_RETURN_ON_FAILURE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_CREATE_2_EARLY_RETURN_ON_FAILURE_BIT_KHR")]
    pub const EARLY_RETURN_ON_FAILURE_KHR: Self = Self::EARLY_RETURN_ON_FAILURE;
    /// [`VK_PIPELINE_CREATE_2_NO_PROTECTED_ACCESS_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_CREATE_2_NO_PROTECTED_ACCESS_BIT_EXT.html)
    ///
    #[doc(alias = "VK_PIPELINE_CREATE_2_NO_PROTECTED_ACCESS_BIT_EXT")]
    pub const NO_PROTECTED_ACCESS_EXT: Self = Self::NO_PROTECTED_ACCESS;
    /// [`VK_PIPELINE_CREATE_2_PROTECTED_ACCESS_ONLY_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_CREATE_2_PROTECTED_ACCESS_ONLY_BIT_EXT.html)
    ///
    #[doc(alias = "VK_PIPELINE_CREATE_2_PROTECTED_ACCESS_ONLY_BIT_EXT")]
    pub const PROTECTED_ACCESS_ONLY_EXT: Self = Self::PROTECTED_ACCESS_ONLY;
    /// [`VK_PIPELINE_CREATE_2_RAY_TRACING_SKIP_BUILT_IN_PRIMITIVES_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_CREATE_2_RAY_TRACING_SKIP_BUILT_IN_PRIMITIVES_BIT_KHR.html)
    ///
    #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_SKIP_BUILT_IN_PRIMITIVES_BIT_KHR")]
    pub const RAY_TRACING_SKIP_BUILT_IN_PRIMITIVES_KHR: Self = Self::RAY_TRACING_SKIP_TRIANGLES_KHR;
    /// [`VK_PIPELINE_CREATE_2_RAY_TRACING_OPACITY_MICROMAP_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_CREATE_2_RAY_TRACING_OPACITY_MICROMAP_BIT_EXT.html)
    ///
    #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_OPACITY_MICROMAP_BIT_EXT")]
    pub const RAY_TRACING_OPACITY_MICROMAP_EXT: Self = Self::RAY_TRACING_OPACITY_MICROMAP_KHR;
}

bitflags::bitflags! {
    /// [`VkSurfaceTransformFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceTransformFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_Surface`](Extensions::KHR_Surface)
    /// - Extension [`KHR_Display`](Extensions::KHR_Display)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkSurfaceTransformFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SurfaceTransformFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extensions::KHR_Surface)
        /// - Extension [`KHR_Display`](Extensions::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR")]
        const IDENTITY_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extensions::KHR_Surface)
        /// - Extension [`KHR_Display`](Extensions::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR")]
        const ROTATE_90_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extensions::KHR_Surface)
        /// - Extension [`KHR_Display`](Extensions::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR")]
        const ROTATE_180_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extensions::KHR_Surface)
        /// - Extension [`KHR_Display`](Extensions::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR")]
        const ROTATE_270_KHR = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extensions::KHR_Surface)
        /// - Extension [`KHR_Display`](Extensions::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR")]
        const HORIZONTAL_MIRROR_KHR = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extensions::KHR_Surface)
        /// - Extension [`KHR_Display`](Extensions::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR")]
        const HORIZONTAL_MIRROR_ROTATE_90_KHR = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extensions::KHR_Surface)
        /// - Extension [`KHR_Display`](Extensions::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR")]
        const HORIZONTAL_MIRROR_ROTATE_180_KHR = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extensions::KHR_Surface)
        /// - Extension [`KHR_Display`](Extensions::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR")]
        const HORIZONTAL_MIRROR_ROTATE_270_KHR = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extensions::KHR_Surface)
        /// - Extension [`KHR_Display`](Extensions::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR")]
        const INHERIT_KHR = 256;
    }
}

bitflags::bitflags! {
    /// [`VkCompositeAlphaFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCompositeAlphaFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_Surface`](Extensions::KHR_Surface)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkCompositeAlphaFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct CompositeAlphaFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extensions::KHR_Surface)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR")]
        const OPAQUE_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extensions::KHR_Surface)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR")]
        const PRE_MULTIPLIED_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extensions::KHR_Surface)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR")]
        const POST_MULTIPLIED_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extensions::KHR_Surface)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR")]
        const INHERIT_KHR = 8;
    }
}

bitflags::bitflags! {
    /// [`VkSwapchainCreateFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainCreateFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_Swapchain`](Extensions::KHR_Swapchain)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkSwapchainCreateFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SwapchainCreateFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_Swapchain`](Extensions::KHR_Swapchain)
        /// - Extension [`KHR_DeviceGroup`](Extensions::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR")]
        const SPLIT_INSTANCE_BIND_REGIONS_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Swapchain`](Extensions::KHR_Swapchain)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR")]
        const PROTECTED_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_SwapchainMutableFormat`](Extensions::KHR_SwapchainMutableFormat)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR")]
        const MUTABLE_FORMAT_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_PresentTiming`](Extensions::EXT_PresentTiming)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SWAPCHAIN_CREATE_PRESENT_TIMING_BIT_EXT")]
        const PRESENT_TIMING_EXT = 512;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_PresentId2`](Extensions::KHR_PresentId2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SWAPCHAIN_CREATE_PRESENT_ID_2_BIT_KHR")]
        const PRESENT_ID_2_KHR = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_PresentWait2`](Extensions::KHR_PresentWait2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SWAPCHAIN_CREATE_PRESENT_WAIT_2_BIT_KHR")]
        const PRESENT_WAIT_2_KHR = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_SwapchainMaintenance1`](Extensions::KHR_SwapchainMaintenance1)
        /// - Extension [`EXT_SwapchainMaintenance1`](Extensions::EXT_SwapchainMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SWAPCHAIN_CREATE_DEFERRED_MEMORY_ALLOCATION_BIT_KHR")]
        const DEFERRED_MEMORY_ALLOCATION_KHR = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MultisampledRenderToSwapchain`](Extensions::EXT_MultisampledRenderToSwapchain)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SWAPCHAIN_CREATE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT")]
        const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT = 256;
    }
}
impl SwapchainCreateFlagsKHR {
    /// [`VK_SWAPCHAIN_CREATE_DEFERRED_MEMORY_ALLOCATION_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SWAPCHAIN_CREATE_DEFERRED_MEMORY_ALLOCATION_BIT_EXT.html)
    ///
    #[doc(alias = "VK_SWAPCHAIN_CREATE_DEFERRED_MEMORY_ALLOCATION_BIT_EXT")]
    pub const DEFERRED_MEMORY_ALLOCATION_EXT: Self = Self::DEFERRED_MEMORY_ALLOCATION_KHR;
}

bitflags::bitflags! {
    /// [`VkDeviceGroupPresentModeFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupPresentModeFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.1 with appropriate features
    /// - Extension [`KHR_Swapchain`](Extensions::KHR_Swapchain)
    /// - Extension [`KHR_DeviceGroup`](Extensions::KHR_DeviceGroup)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDeviceGroupPresentModeFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DeviceGroupPresentModeFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_Swapchain`](Extensions::KHR_Swapchain)
        /// - Extension [`KHR_DeviceGroup`](Extensions::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR")]
        const LOCAL_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_Swapchain`](Extensions::KHR_Swapchain)
        /// - Extension [`KHR_DeviceGroup`](Extensions::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR")]
        const REMOTE_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_Swapchain`](Extensions::KHR_Swapchain)
        /// - Extension [`KHR_DeviceGroup`](Extensions::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR")]
        const SUM_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_Swapchain`](Extensions::KHR_Swapchain)
        /// - Extension [`KHR_DeviceGroup`](Extensions::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR")]
        const LOCAL_MULTI_DEVICE_KHR = 8;
    }
}

bitflags::bitflags! {
    /// [`VkDisplayPlaneAlphaFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayPlaneAlphaFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_Display`](Extensions::KHR_Display)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDisplayPlaneAlphaFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DisplayPlaneAlphaFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Display`](Extensions::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR")]
        const OPAQUE_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Display`](Extensions::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR")]
        const GLOBAL_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Display`](Extensions::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR")]
        const PER_PIXEL_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Display`](Extensions::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR")]
        const PER_PIXEL_PREMULTIPLIED_KHR = 8;
    }
}

bitflags::bitflags! {
    /// [`VkVideoCodecOperationFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoCodecOperationFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoQueue`](Extensions::KHR_VideoQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoCodecOperationFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoCodecOperationFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extensions::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CODEC_OPERATION_NONE_KHR")]
        const NONE_KHR = 0;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CODEC_OPERATION_ENCODE_H264_BIT_KHR")]
        const ENCODE_H264_KHR = 65536;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CODEC_OPERATION_ENCODE_H265_BIT_KHR")]
        const ENCODE_H265_KHR = 131072;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeH264`](Extensions::KHR_VideoDecodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CODEC_OPERATION_DECODE_H264_BIT_KHR")]
        const DECODE_H264_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeH265`](Extensions::KHR_VideoDecodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CODEC_OPERATION_DECODE_H265_BIT_KHR")]
        const DECODE_H265_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeAv1`](Extensions::KHR_VideoDecodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CODEC_OPERATION_DECODE_AV1_BIT_KHR")]
        const DECODE_AV1_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extensions::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CODEC_OPERATION_ENCODE_AV1_BIT_KHR")]
        const ENCODE_AV1_KHR = 262144;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeVp9`](Extensions::KHR_VideoDecodeVp9)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CODEC_OPERATION_DECODE_VP9_BIT_KHR")]
        const DECODE_VP9_KHR = 8;
    }
}

bitflags::bitflags! {
    /// [`VkVideoChromaSubsamplingFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoChromaSubsamplingFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoQueue`](Extensions::KHR_VideoQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoChromaSubsamplingFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoChromaSubsamplingFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extensions::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_INVALID_KHR")]
        const INVALID_KHR = 0;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extensions::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_MONOCHROME_BIT_KHR")]
        const MONOCHROME_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extensions::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_420_BIT_KHR")]
        const _420_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extensions::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_422_BIT_KHR")]
        const _422_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extensions::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_444_BIT_KHR")]
        const _444_KHR = 8;
    }
}

bitflags::bitflags! {
    /// [`VkVideoComponentBitDepthFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoComponentBitDepthFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoQueue`](Extensions::KHR_VideoQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoComponentBitDepthFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoComponentBitDepthFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extensions::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_COMPONENT_BIT_DEPTH_INVALID_KHR")]
        const INVALID_KHR = 0;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extensions::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_COMPONENT_BIT_DEPTH_8_BIT_KHR")]
        const _8_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extensions::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_COMPONENT_BIT_DEPTH_10_BIT_KHR")]
        const _10_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extensions::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_COMPONENT_BIT_DEPTH_12_BIT_KHR")]
        const _12_KHR = 16;
    }
}

bitflags::bitflags! {
    /// [`VkVideoCapabilityFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoCapabilityFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoQueue`](Extensions::KHR_VideoQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkVideoCapabilityFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoCapabilityFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extensions::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CAPABILITY_PROTECTED_CONTENT_BIT_KHR")]
        const PROTECTED_CONTENT_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extensions::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CAPABILITY_SEPARATE_REFERENCE_IMAGES_BIT_KHR")]
        const SEPARATE_REFERENCE_IMAGES_KHR = 2;
    }
}

bitflags::bitflags! {
    /// [`VkVideoSessionCreateFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionCreateFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoQueue`](Extensions::KHR_VideoQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoSessionCreateFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoSessionCreateFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extensions::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_SESSION_CREATE_PROTECTED_CONTENT_BIT_KHR")]
        const PROTECTED_CONTENT_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_SESSION_CREATE_ALLOW_ENCODE_PARAMETER_OPTIMIZATIONS_BIT_KHR")]
        const ALLOW_ENCODE_PARAMETER_OPTIMIZATIONS_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoMaintenance1`](Extensions::KHR_VideoMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_SESSION_CREATE_INLINE_QUERIES_BIT_KHR")]
        const INLINE_QUERIES_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extensions::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_SESSION_CREATE_ALLOW_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR")]
        const ALLOW_ENCODE_QUANTIZATION_DELTA_MAP_KHR = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extensions::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_SESSION_CREATE_ALLOW_ENCODE_EMPHASIS_MAP_BIT_KHR")]
        const ALLOW_ENCODE_EMPHASIS_MAP_KHR = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoMaintenance2`](Extensions::KHR_VideoMaintenance2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_SESSION_CREATE_INLINE_SESSION_PARAMETERS_BIT_KHR")]
        const INLINE_SESSION_PARAMETERS_KHR = 32;
    }
}

bitflags::bitflags! {
    /// [`VkVideoSessionParametersCreateFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionParametersCreateFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoQueue`](Extensions::KHR_VideoQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoSessionParametersCreateFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoSessionParametersCreateFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extensions::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_SESSION_PARAMETERS_CREATE_QUANTIZATION_MAP_COMPATIBLE_BIT_KHR")]
        const QUANTIZATION_MAP_COMPATIBLE_KHR = 1;
    }
}

bitflags::bitflags! {
    /// [`VkVideoCodingControlFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoCodingControlFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoQueue`](Extensions::KHR_VideoQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoCodingControlFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoCodingControlFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extensions::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CODING_CONTROL_RESET_BIT_KHR")]
        const RESET_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CODING_CONTROL_ENCODE_RATE_CONTROL_BIT_KHR")]
        const ENCODE_RATE_CONTROL_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CODING_CONTROL_ENCODE_QUALITY_LEVEL_BIT_KHR")]
        const ENCODE_QUALITY_LEVEL_KHR = 4;
    }
}

bitflags::bitflags! {
    /// [`VkVideoDecodeCapabilityFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeCapabilityFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoDecodeQueue`](Extensions::KHR_VideoDecodeQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkVideoDecodeCapabilityFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoDecodeCapabilityFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extensions::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_COINCIDE_BIT_KHR")]
        const DPB_AND_OUTPUT_COINCIDE_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extensions::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_DISTINCT_BIT_KHR")]
        const DPB_AND_OUTPUT_DISTINCT_KHR = 2;
    }
}

bitflags::bitflags! {
    /// [`VkVideoDecodeUsageFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeUsageFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoDecodeQueue`](Extensions::KHR_VideoDecodeQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoDecodeUsageFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoDecodeUsageFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extensions::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_DECODE_USAGE_DEFAULT_KHR")]
        const DEFAULT_KHR = 0;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extensions::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_DECODE_USAGE_TRANSCODING_BIT_KHR")]
        const TRANSCODING_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extensions::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_DECODE_USAGE_OFFLINE_BIT_KHR")]
        const OFFLINE_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extensions::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_DECODE_USAGE_STREAMING_BIT_KHR")]
        const STREAMING_KHR = 4;
    }
}

bitflags::bitflags! {
    /// [`VkVideoEncodeH264CapabilityFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264CapabilityFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkVideoEncodeH264CapabilityFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeH264CapabilityFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_HRD_COMPLIANCE_BIT_KHR")]
        const VIDEO_ENCODE_H264_CAPABILITY_HRD_COMPLIANCE_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_PREDICTION_WEIGHT_TABLE_GENERATED_BIT_KHR")]
        const VIDEO_ENCODE_H264_CAPABILITY_PREDICTION_WEIGHT_TABLE_GENERATED_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_ROW_UNALIGNED_SLICE_BIT_KHR")]
        const VIDEO_ENCODE_H264_CAPABILITY_ROW_UNALIGNED_SLICE_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_DIFFERENT_SLICE_TYPE_BIT_KHR")]
        const VIDEO_ENCODE_H264_CAPABILITY_DIFFERENT_SLICE_TYPE_KHR = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L0_LIST_BIT_KHR")]
        const VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L0_LIST_KHR = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L1_LIST_BIT_KHR")]
        const VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L1_LIST_KHR = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_PER_PICTURE_TYPE_MIN_MAX_QP_BIT_KHR")]
        const VIDEO_ENCODE_H264_CAPABILITY_PER_PICTURE_TYPE_MIN_MAX_QP_KHR = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_PER_SLICE_CONSTANT_QP_BIT_KHR")]
        const VIDEO_ENCODE_H264_CAPABILITY_PER_SLICE_CONSTANT_QP_KHR = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_GENERATE_PREFIX_NALU_BIT_KHR")]
        const VIDEO_ENCODE_H264_CAPABILITY_GENERATE_PREFIX_NALU_KHR = 256;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeIntraRefresh`](Extensions::KHR_VideoEncodeIntraRefresh)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_B_PICTURE_INTRA_REFRESH_BIT_KHR")]
        const VIDEO_ENCODE_H264_CAPABILITY_B_PICTURE_INTRA_REFRESH_KHR = 1024;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extensions::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_MB_QP_DIFF_WRAPAROUND_BIT_KHR")]
        const VIDEO_ENCODE_H264_CAPABILITY_MB_QP_DIFF_WRAPAROUND_KHR = 512;
    }
}

bitflags::bitflags! {
    /// [`VkVideoEncodeH264StdFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264StdFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkVideoEncodeH264StdFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeH264StdFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_SEPARATE_COLOR_PLANE_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_SEPARATE_COLOR_PLANE_FLAG_SET_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_SCALING_MATRIX_PRESENT_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_SCALING_MATRIX_PRESENT_FLAG_SET_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_CHROMA_QP_INDEX_OFFSET_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_CHROMA_QP_INDEX_OFFSET_KHR = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_SECOND_CHROMA_QP_INDEX_OFFSET_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_SECOND_CHROMA_QP_INDEX_OFFSET_KHR = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_PIC_INIT_QP_MINUS26_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_PIC_INIT_QP_MINUS26_KHR = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_WEIGHTED_PRED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_WEIGHTED_PRED_FLAG_SET_KHR = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_WEIGHTED_BIPRED_IDC_EXPLICIT_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_WEIGHTED_BIPRED_IDC_EXPLICIT_KHR = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_WEIGHTED_BIPRED_IDC_IMPLICIT_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_WEIGHTED_BIPRED_IDC_IMPLICIT_KHR = 256;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_TRANSFORM_8X8_MODE_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_TRANSFORM_8X8_MODE_FLAG_SET_KHR = 512;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_KHR = 1024;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_ENTROPY_CODING_MODE_FLAG_UNSET_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_ENTROPY_CODING_MODE_FLAG_UNSET_KHR = 2048;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_ENTROPY_CODING_MODE_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_ENTROPY_CODING_MODE_FLAG_SET_KHR = 4096;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_DIRECT_8X8_INFERENCE_FLAG_UNSET_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_DIRECT_8X8_INFERENCE_FLAG_UNSET_KHR = 8192;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_CONSTRAINED_INTRA_PRED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_CONSTRAINED_INTRA_PRED_FLAG_SET_KHR = 16384;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_DISABLED_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_DISABLED_KHR = 32768;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_ENABLED_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_ENABLED_KHR = 65536;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_PARTIAL_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_PARTIAL_KHR = 131072;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_SLICE_QP_DELTA_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_SLICE_QP_DELTA_KHR = 524288;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_DIFFERENT_SLICE_QP_DELTA_BIT_KHR")]
        const VIDEO_ENCODE_H264_STD_DIFFERENT_SLICE_QP_DELTA_KHR = 1048576;
    }
}

bitflags::bitflags! {
    /// [`VkVideoEncodeH264RateControlFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264RateControlFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodeH264RateControlFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeH264RateControlFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_RATE_CONTROL_ATTEMPT_HRD_COMPLIANCE_BIT_KHR")]
        const VIDEO_ENCODE_H264_RATE_CONTROL_ATTEMPT_HRD_COMPLIANCE_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_RATE_CONTROL_REGULAR_GOP_BIT_KHR")]
        const VIDEO_ENCODE_H264_RATE_CONTROL_REGULAR_GOP_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR")]
        const VIDEO_ENCODE_H264_RATE_CONTROL_REFERENCE_PATTERN_FLAT_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR")]
        const VIDEO_ENCODE_H264_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_KHR = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extensions::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_BIT_KHR")]
        const VIDEO_ENCODE_H264_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_KHR = 16;
    }
}

bitflags::bitflags! {
    /// [`VkVideoEncodeH265CapabilityFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265CapabilityFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkVideoEncodeH265CapabilityFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeH265CapabilityFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_HRD_COMPLIANCE_BIT_KHR")]
        const VIDEO_ENCODE_H265_CAPABILITY_HRD_COMPLIANCE_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_PREDICTION_WEIGHT_TABLE_GENERATED_BIT_KHR")]
        const VIDEO_ENCODE_H265_CAPABILITY_PREDICTION_WEIGHT_TABLE_GENERATED_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_ROW_UNALIGNED_SLICE_SEGMENT_BIT_KHR")]
        const VIDEO_ENCODE_H265_CAPABILITY_ROW_UNALIGNED_SLICE_SEGMENT_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_DIFFERENT_SLICE_SEGMENT_TYPE_BIT_KHR")]
        const VIDEO_ENCODE_H265_CAPABILITY_DIFFERENT_SLICE_SEGMENT_TYPE_KHR = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L0_LIST_BIT_KHR")]
        const VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L0_LIST_KHR = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L1_LIST_BIT_KHR")]
        const VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L1_LIST_KHR = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_PER_PICTURE_TYPE_MIN_MAX_QP_BIT_KHR")]
        const VIDEO_ENCODE_H265_CAPABILITY_PER_PICTURE_TYPE_MIN_MAX_QP_KHR = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_PER_SLICE_SEGMENT_CONSTANT_QP_BIT_KHR")]
        const VIDEO_ENCODE_H265_CAPABILITY_PER_SLICE_SEGMENT_CONSTANT_QP_KHR = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_TILES_PER_SLICE_SEGMENT_BIT_KHR")]
        const VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_TILES_PER_SLICE_SEGMENT_KHR = 256;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_SLICE_SEGMENTS_PER_TILE_BIT_KHR")]
        const VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_SLICE_SEGMENTS_PER_TILE_KHR = 512;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeIntraRefresh`](Extensions::KHR_VideoEncodeIntraRefresh)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_B_PICTURE_INTRA_REFRESH_BIT_KHR")]
        const VIDEO_ENCODE_H265_CAPABILITY_B_PICTURE_INTRA_REFRESH_KHR = 2048;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extensions::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_CU_QP_DIFF_WRAPAROUND_BIT_KHR")]
        const VIDEO_ENCODE_H265_CAPABILITY_CU_QP_DIFF_WRAPAROUND_KHR = 1024;
    }
}

bitflags::bitflags! {
    /// [`VkVideoEncodeH265StdFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265StdFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkVideoEncodeH265StdFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeH265StdFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_SEPARATE_COLOR_PLANE_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_SEPARATE_COLOR_PLANE_FLAG_SET_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_SCALING_LIST_DATA_PRESENT_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_SCALING_LIST_DATA_PRESENT_FLAG_SET_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_PCM_ENABLED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_PCM_ENABLED_FLAG_SET_KHR = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_SPS_TEMPORAL_MVP_ENABLED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_SPS_TEMPORAL_MVP_ENABLED_FLAG_SET_KHR = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_INIT_QP_MINUS26_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_INIT_QP_MINUS26_KHR = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_WEIGHTED_PRED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_WEIGHTED_PRED_FLAG_SET_KHR = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_WEIGHTED_BIPRED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_WEIGHTED_BIPRED_FLAG_SET_KHR = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_LOG2_PARALLEL_MERGE_LEVEL_MINUS2_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_LOG2_PARALLEL_MERGE_LEVEL_MINUS2_KHR = 256;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_SIGN_DATA_HIDING_ENABLED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_SIGN_DATA_HIDING_ENABLED_FLAG_SET_KHR = 512;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_TRANSFORM_SKIP_ENABLED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_TRANSFORM_SKIP_ENABLED_FLAG_SET_KHR = 1024;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_TRANSFORM_SKIP_ENABLED_FLAG_UNSET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_TRANSFORM_SKIP_ENABLED_FLAG_UNSET_KHR = 2048;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET_KHR = 4096;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_TRANSQUANT_BYPASS_ENABLED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_TRANSQUANT_BYPASS_ENABLED_FLAG_SET_KHR = 8192;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_CONSTRAINED_INTRA_PRED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_CONSTRAINED_INTRA_PRED_FLAG_SET_KHR = 16384;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_ENTROPY_CODING_SYNC_ENABLED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_ENTROPY_CODING_SYNC_ENABLED_FLAG_SET_KHR = 32768;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET_KHR = 65536;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET_KHR = 131072;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_DEPENDENT_SLICE_SEGMENT_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_DEPENDENT_SLICE_SEGMENT_FLAG_SET_KHR = 262144;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_SLICE_QP_DELTA_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_SLICE_QP_DELTA_KHR = 524288;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_DIFFERENT_SLICE_QP_DELTA_BIT_KHR")]
        const VIDEO_ENCODE_H265_STD_DIFFERENT_SLICE_QP_DELTA_KHR = 1048576;
    }
}

bitflags::bitflags! {
    /// [`VkVideoEncodeH265CtbSizeFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265CtbSizeFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkVideoEncodeH265CtbSizeFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeH265CtbSizeFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CTB_SIZE_16_BIT_KHR")]
        const VIDEO_ENCODE_H265_CTB_SIZE_16_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CTB_SIZE_32_BIT_KHR")]
        const VIDEO_ENCODE_H265_CTB_SIZE_32_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CTB_SIZE_64_BIT_KHR")]
        const VIDEO_ENCODE_H265_CTB_SIZE_64_KHR = 4;
    }
}

bitflags::bitflags! {
    /// [`VkVideoEncodeH265TransformBlockSizeFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265TransformBlockSizeFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkVideoEncodeH265TransformBlockSizeFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeH265TransformBlockSizeFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_4_BIT_KHR")]
        const VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_4_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_8_BIT_KHR")]
        const VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_8_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_16_BIT_KHR")]
        const VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_16_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_32_BIT_KHR")]
        const VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_32_KHR = 8;
    }
}

bitflags::bitflags! {
    /// [`VkVideoEncodeH265RateControlFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265RateControlFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodeH265RateControlFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeH265RateControlFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_RATE_CONTROL_ATTEMPT_HRD_COMPLIANCE_BIT_KHR")]
        const VIDEO_ENCODE_H265_RATE_CONTROL_ATTEMPT_HRD_COMPLIANCE_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_RATE_CONTROL_REGULAR_GOP_BIT_KHR")]
        const VIDEO_ENCODE_H265_RATE_CONTROL_REGULAR_GOP_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR")]
        const VIDEO_ENCODE_H265_RATE_CONTROL_REFERENCE_PATTERN_FLAT_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR")]
        const VIDEO_ENCODE_H265_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_KHR = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extensions::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_RATE_CONTROL_TEMPORAL_SUB_LAYER_PATTERN_DYADIC_BIT_KHR")]
        const VIDEO_ENCODE_H265_RATE_CONTROL_TEMPORAL_SUB_LAYER_PATTERN_DYADIC_KHR = 16;
    }
}

bitflags::bitflags! {
    /// [`VkVideoDecodeH264PictureLayoutFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeH264PictureLayoutFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoDecodeH264`](Extensions::KHR_VideoDecodeH264)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoDecodeH264PictureLayoutFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoDecodeH264PictureLayoutFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeH264`](Extensions::KHR_VideoDecodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_PROGRESSIVE_KHR")]
        const VIDEO_DECODE_H264_PICTURE_LAYOUT_PROGRESSIVE_KHR = 0;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeH264`](Extensions::KHR_VideoDecodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_INTERLEAVED_LINES_BIT_KHR")]
        const VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_INTERLEAVED_LINES_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeH264`](Extensions::KHR_VideoDecodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_SEPARATE_PLANES_BIT_KHR")]
        const VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_SEPARATE_PLANES_KHR = 2;
    }
}

bitflags::bitflags! {
    /// [`VkPerformanceCounterDescriptionFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterDescriptionFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_PerformanceQuery`](Extensions::KHR_PerformanceQuery)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkPerformanceCounterDescriptionFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PerformanceCounterDescriptionFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_PerformanceQuery`](Extensions::KHR_PerformanceQuery)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR")]
        const PERFORMANCE_IMPACTING_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_PerformanceQuery`](Extensions::KHR_PerformanceQuery)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR")]
        const CONCURRENTLY_IMPACTED_KHR = 2;
    }
}
impl PerformanceCounterDescriptionFlagsKHR {}

bitflags::bitflags! {
    /// [`VkAcquireProfilingLockFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAcquireProfilingLockFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_PerformanceQuery`](Extensions::KHR_PerformanceQuery)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkAcquireProfilingLockFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct AcquireProfilingLockFlagsKHR: u32 {
    }
}

bitflags::bitflags! {
    /// [`VkVideoEncodeFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodeFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeIntraRefresh`](Extensions::KHR_VideoEncodeIntraRefresh)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_INTRA_REFRESH_BIT_KHR")]
        const INTRA_REFRESH_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extensions::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_WITH_QUANTIZATION_DELTA_MAP_BIT_KHR")]
        const WITH_QUANTIZATION_DELTA_MAP_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extensions::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_WITH_EMPHASIS_MAP_BIT_KHR")]
        const WITH_EMPHASIS_MAP_KHR = 2;
    }
}

bitflags::bitflags! {
    /// [`VkVideoEncodeCapabilityFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeCapabilityFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkVideoEncodeCapabilityFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeCapabilityFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_CAPABILITY_PRECEDING_EXTERNALLY_ENCODED_BYTES_BIT_KHR")]
        const PRECEDING_EXTERNALLY_ENCODED_BYTES_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_CAPABILITY_INSUFFICIENT_BITSTREAM_BUFFER_RANGE_DETECTION_BIT_KHR")]
        const INSUFFICIENT_BITSTREAM_BUFFER_RANGE_DETECTION_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extensions::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_CAPABILITY_QUANTIZATION_DELTA_MAP_BIT_KHR")]
        const QUANTIZATION_DELTA_MAP_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extensions::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_CAPABILITY_EMPHASIS_MAP_BIT_KHR")]
        const EMPHASIS_MAP_KHR = 8;
    }
}

bitflags::bitflags! {
    /// [`VkVideoEncodeRateControlModeFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRateControlModeFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodeRateControlModeFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeRateControlModeFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RATE_CONTROL_MODE_DEFAULT_KHR")]
        const DEFAULT_KHR = 0;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RATE_CONTROL_MODE_DISABLED_BIT_KHR")]
        const DISABLED_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RATE_CONTROL_MODE_CBR_BIT_KHR")]
        const CBR_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RATE_CONTROL_MODE_VBR_BIT_KHR")]
        const VBR_KHR = 4;
    }
}

bitflags::bitflags! {
    /// [`VkVideoEncodeFeedbackFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeFeedbackFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodeFeedbackFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeFeedbackFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_BITSTREAM_BUFFER_OFFSET_BIT_KHR")]
        const BITSTREAM_BUFFER_OFFSET_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_BITSTREAM_BYTES_WRITTEN_BIT_KHR")]
        const BITSTREAM_BYTES_WRITTEN_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_BITSTREAM_HAS_OVERRIDES_BIT_KHR")]
        const BITSTREAM_HAS_OVERRIDES_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeFeedback2`](Extensions::KHR_VideoEncodeFeedback2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_AVERAGE_QUANTIZATION_BIT_KHR")]
        const AVERAGE_QUANTIZATION_KHR = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeFeedback2`](Extensions::KHR_VideoEncodeFeedback2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_MIN_QUANTIZATION_BIT_KHR")]
        const MIN_QUANTIZATION_KHR = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeFeedback2`](Extensions::KHR_VideoEncodeFeedback2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_MAX_QUANTIZATION_BIT_KHR")]
        const MAX_QUANTIZATION_KHR = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeFeedback2`](Extensions::KHR_VideoEncodeFeedback2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_INTRA_PIXELS_BIT_KHR")]
        const INTRA_PIXELS_KHR = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeFeedback2`](Extensions::KHR_VideoEncodeFeedback2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_INTER_PIXELS_BIT_KHR")]
        const INTER_PIXELS_KHR = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeFeedback2`](Extensions::KHR_VideoEncodeFeedback2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_SKIPPED_PIXELS_BIT_KHR")]
        const SKIPPED_PIXELS_KHR = 256;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeFeedback2`](Extensions::KHR_VideoEncodeFeedback2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_PICTURE_PARTITION_COUNT_BIT_KHR")]
        const PICTURE_PARTITION_COUNT_KHR = 512;
    }
}

bitflags::bitflags! {
    /// [`VkVideoEncodeUsageFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeUsageFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodeUsageFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeUsageFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_USAGE_DEFAULT_KHR")]
        const DEFAULT_KHR = 0;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_USAGE_TRANSCODING_BIT_KHR")]
        const TRANSCODING_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_USAGE_STREAMING_BIT_KHR")]
        const STREAMING_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_USAGE_RECORDING_BIT_KHR")]
        const RECORDING_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_USAGE_CONFERENCING_BIT_KHR")]
        const CONFERENCING_KHR = 8;
    }
}

bitflags::bitflags! {
    /// [`VkVideoEncodeContentFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeContentFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodeContentFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeContentFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_CONTENT_DEFAULT_KHR")]
        const DEFAULT_KHR = 0;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_CONTENT_CAMERA_BIT_KHR")]
        const CAMERA_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_CONTENT_DESKTOP_BIT_KHR")]
        const DESKTOP_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_CONTENT_RENDERED_BIT_KHR")]
        const RENDERED_KHR = 4;
    }
}

bitflags::bitflags! {
    /// [`VkAddressCommandFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAddressCommandFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_DeviceAddressCommands`](Extensions::KHR_DeviceAddressCommands)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkAddressCommandFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct AddressCommandFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_DeviceAddressCommands`](Extensions::KHR_DeviceAddressCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ADDRESS_COMMAND_PROTECTED_BIT_KHR")]
        const PROTECTED_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_DeviceAddressCommands`](Extensions::KHR_DeviceAddressCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ADDRESS_COMMAND_FULLY_BOUND_BIT_KHR")]
        const FULLY_BOUND_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_DeviceAddressCommands`](Extensions::KHR_DeviceAddressCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ADDRESS_COMMAND_STORAGE_BUFFER_USAGE_BIT_KHR")]
        const STORAGE_BUFFER_USAGE_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_DeviceAddressCommands`](Extensions::KHR_DeviceAddressCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ADDRESS_COMMAND_UNKNOWN_STORAGE_BUFFER_USAGE_BIT_KHR")]
        const UNKNOWN_STORAGE_BUFFER_USAGE_KHR = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_DeviceAddressCommands`](Extensions::KHR_DeviceAddressCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ADDRESS_COMMAND_TRANSFORM_FEEDBACK_BUFFER_USAGE_BIT_KHR")]
        const TRANSFORM_FEEDBACK_BUFFER_USAGE_KHR = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_DeviceAddressCommands`](Extensions::KHR_DeviceAddressCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ADDRESS_COMMAND_UNKNOWN_TRANSFORM_FEEDBACK_BUFFER_USAGE_BIT_KHR")]
        const UNKNOWN_TRANSFORM_FEEDBACK_BUFFER_USAGE_KHR = 32;
    }
}

bitflags::bitflags! {
    /// [`VkConditionalRenderingFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkConditionalRenderingFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_ConditionalRendering`](Extensions::EXT_ConditionalRendering)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkConditionalRenderingFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ConditionalRenderingFlagsEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ConditionalRendering`](Extensions::EXT_ConditionalRendering)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CONDITIONAL_RENDERING_INVERTED_BIT_EXT")]
        const INVERTED_EXT = 1;
    }
}

bitflags::bitflags! {
    /// [`VkAccelerationStructureCreateFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureCreateFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_AccelerationStructure`](Extensions::KHR_AccelerationStructure)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkAccelerationStructureCreateFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct AccelerationStructureCreateFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_AccelerationStructure`](Extensions::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR")]
        const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorBuffer`](Extensions::EXT_DescriptorBuffer)
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCELERATION_STRUCTURE_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT")]
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracingMotionBlur`](Extensions::NV_RayTracingMotionBlur)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV")]
        const MOTION_NV = 4;
    }
}

bitflags::bitflags! {
    /// [`VkPresentScalingFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentScalingFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_SurfaceMaintenance1`](Extensions::KHR_SurfaceMaintenance1)
    /// - Extension [`EXT_SurfaceMaintenance1`](Extensions::EXT_SurfaceMaintenance1)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkPresentScalingFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PresentScalingFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_SurfaceMaintenance1`](Extensions::KHR_SurfaceMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRESENT_SCALING_ONE_TO_ONE_BIT_KHR")]
        const ONE_TO_ONE_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_SurfaceMaintenance1`](Extensions::KHR_SurfaceMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRESENT_SCALING_ASPECT_RATIO_STRETCH_BIT_KHR")]
        const ASPECT_RATIO_STRETCH_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_SurfaceMaintenance1`](Extensions::KHR_SurfaceMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRESENT_SCALING_STRETCH_BIT_KHR")]
        const STRETCH_KHR = 4;
    }
}
/// [`VkPresentScalingFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentScalingFlagBitsEXT.html)
///
#[doc(alias = "VkPresentScalingFlagBitsEXT")]
pub type PresentScalingFlagsEXT = PresentScalingFlagsKHR;
impl PresentScalingFlagsKHR {
    /// [`VK_PRESENT_SCALING_ONE_TO_ONE_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PRESENT_SCALING_ONE_TO_ONE_BIT_EXT.html)
    ///
    #[doc(alias = "VK_PRESENT_SCALING_ONE_TO_ONE_BIT_EXT")]
    pub const ONE_TO_ONE_EXT: Self = Self::ONE_TO_ONE_KHR;
    /// [`VK_PRESENT_SCALING_ASPECT_RATIO_STRETCH_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PRESENT_SCALING_ASPECT_RATIO_STRETCH_BIT_EXT.html)
    ///
    #[doc(alias = "VK_PRESENT_SCALING_ASPECT_RATIO_STRETCH_BIT_EXT")]
    pub const ASPECT_RATIO_STRETCH_EXT: Self = Self::ASPECT_RATIO_STRETCH_KHR;
    /// [`VK_PRESENT_SCALING_STRETCH_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PRESENT_SCALING_STRETCH_BIT_EXT.html)
    ///
    #[doc(alias = "VK_PRESENT_SCALING_STRETCH_BIT_EXT")]
    pub const STRETCH_EXT: Self = Self::STRETCH_KHR;
}

bitflags::bitflags! {
    /// [`VkPresentGravityFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentGravityFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_SurfaceMaintenance1`](Extensions::KHR_SurfaceMaintenance1)
    /// - Extension [`EXT_SurfaceMaintenance1`](Extensions::EXT_SurfaceMaintenance1)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkPresentGravityFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PresentGravityFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_SurfaceMaintenance1`](Extensions::KHR_SurfaceMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRESENT_GRAVITY_MIN_BIT_KHR")]
        const MIN_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_SurfaceMaintenance1`](Extensions::KHR_SurfaceMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRESENT_GRAVITY_MAX_BIT_KHR")]
        const MAX_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_SurfaceMaintenance1`](Extensions::KHR_SurfaceMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRESENT_GRAVITY_CENTERED_BIT_KHR")]
        const CENTERED_KHR = 4;
    }
}
/// [`VkPresentGravityFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentGravityFlagBitsEXT.html)
///
#[doc(alias = "VkPresentGravityFlagBitsEXT")]
pub type PresentGravityFlagsEXT = PresentGravityFlagsKHR;
impl PresentGravityFlagsKHR {
    /// [`VK_PRESENT_GRAVITY_MIN_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PRESENT_GRAVITY_MIN_BIT_EXT.html)
    ///
    #[doc(alias = "VK_PRESENT_GRAVITY_MIN_BIT_EXT")]
    pub const MIN_EXT: Self = Self::MIN_KHR;
    /// [`VK_PRESENT_GRAVITY_MAX_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PRESENT_GRAVITY_MAX_BIT_EXT.html)
    ///
    #[doc(alias = "VK_PRESENT_GRAVITY_MAX_BIT_EXT")]
    pub const MAX_EXT: Self = Self::MAX_KHR;
    /// [`VK_PRESENT_GRAVITY_CENTERED_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PRESENT_GRAVITY_CENTERED_BIT_EXT.html)
    ///
    #[doc(alias = "VK_PRESENT_GRAVITY_CENTERED_BIT_EXT")]
    pub const CENTERED_EXT: Self = Self::CENTERED_KHR;
}

bitflags::bitflags! {
    /// [`VkVideoEncodeAV1CapabilityFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1CapabilityFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeAv1`](Extensions::KHR_VideoEncodeAv1)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkVideoEncodeAV1CapabilityFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeAV1CapabilityFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extensions::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_CAPABILITY_PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_BIT_KHR")]
        const VIDEO_ENCODE_AV1_CAPABILITY_PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extensions::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_CAPABILITY_GENERATE_OBU_EXTENSION_HEADER_BIT_KHR")]
        const VIDEO_ENCODE_AV1_CAPABILITY_GENERATE_OBU_EXTENSION_HEADER_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extensions::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_CAPABILITY_PRIMARY_REFERENCE_CDF_ONLY_BIT_KHR")]
        const VIDEO_ENCODE_AV1_CAPABILITY_PRIMARY_REFERENCE_CDF_ONLY_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extensions::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_CAPABILITY_FRAME_SIZE_OVERRIDE_BIT_KHR")]
        const VIDEO_ENCODE_AV1_CAPABILITY_FRAME_SIZE_OVERRIDE_KHR = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extensions::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_CAPABILITY_MOTION_VECTOR_SCALING_BIT_KHR")]
        const VIDEO_ENCODE_AV1_CAPABILITY_MOTION_VECTOR_SCALING_KHR = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeIntraRefresh`](Extensions::KHR_VideoEncodeIntraRefresh)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_CAPABILITY_COMPOUND_PREDICTION_INTRA_REFRESH_BIT_KHR")]
        const VIDEO_ENCODE_AV1_CAPABILITY_COMPOUND_PREDICTION_INTRA_REFRESH_KHR = 32;
    }
}

bitflags::bitflags! {
    /// [`VkVideoEncodeAV1StdFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1StdFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeAv1`](Extensions::KHR_VideoEncodeAv1)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkVideoEncodeAV1StdFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeAV1StdFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extensions::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_STD_UNIFORM_TILE_SPACING_FLAG_SET_BIT_KHR")]
        const VIDEO_ENCODE_AV1_STD_UNIFORM_TILE_SPACING_FLAG_SET_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extensions::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_STD_SKIP_MODE_PRESENT_UNSET_BIT_KHR")]
        const VIDEO_ENCODE_AV1_STD_SKIP_MODE_PRESENT_UNSET_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extensions::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_STD_PRIMARY_REF_FRAME_BIT_KHR")]
        const VIDEO_ENCODE_AV1_STD_PRIMARY_REF_FRAME_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extensions::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_STD_DELTA_Q_BIT_KHR")]
        const VIDEO_ENCODE_AV1_STD_DELTA_Q_KHR = 8;
    }
}

bitflags::bitflags! {
    /// [`VkVideoEncodeAV1SuperblockSizeFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1SuperblockSizeFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeAv1`](Extensions::KHR_VideoEncodeAv1)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkVideoEncodeAV1SuperblockSizeFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeAV1SuperblockSizeFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extensions::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_64_BIT_KHR")]
        const VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_64_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extensions::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_128_BIT_KHR")]
        const VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_128_KHR = 2;
    }
}

bitflags::bitflags! {
    /// [`VkVideoEncodeAV1RateControlFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1RateControlFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeAv1`](Extensions::KHR_VideoEncodeAv1)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodeAV1RateControlFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeAV1RateControlFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extensions::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_RATE_CONTROL_REGULAR_GOP_BIT_KHR")]
        const VIDEO_ENCODE_AV1_RATE_CONTROL_REGULAR_GOP_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extensions::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_BIT_KHR")]
        const VIDEO_ENCODE_AV1_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extensions::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR")]
        const VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_FLAT_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extensions::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR")]
        const VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_KHR = 8;
    }
}

bitflags::bitflags! {
    /// [`VkAddressCopyFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAddressCopyFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_CopyMemoryIndirect`](Extensions::KHR_CopyMemoryIndirect)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkAddressCopyFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct AddressCopyFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_CopyMemoryIndirect`](Extensions::KHR_CopyMemoryIndirect)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ADDRESS_COPY_DEVICE_LOCAL_BIT_KHR")]
        const DEVICE_LOCAL_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_CopyMemoryIndirect`](Extensions::KHR_CopyMemoryIndirect)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ADDRESS_COPY_SPARSE_BIT_KHR")]
        const SPARSE_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_CopyMemoryIndirect`](Extensions::KHR_CopyMemoryIndirect)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ADDRESS_COPY_PROTECTED_BIT_KHR")]
        const PROTECTED_KHR = 4;
    }
}

bitflags::bitflags! {
    /// [`VkVideoEncodeIntraRefreshModeFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeIntraRefreshModeFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeIntraRefresh`](Extensions::KHR_VideoEncodeIntraRefresh)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodeIntraRefreshModeFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeIntraRefreshModeFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeIntraRefresh`](Extensions::KHR_VideoEncodeIntraRefresh)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_NONE_KHR")]
        const NONE_KHR = 0;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeIntraRefresh`](Extensions::KHR_VideoEncodeIntraRefresh)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_PER_PICTURE_PARTITION_BIT_KHR")]
        const PER_PICTURE_PARTITION_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeIntraRefresh`](Extensions::KHR_VideoEncodeIntraRefresh)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_BLOCK_BASED_BIT_KHR")]
        const BLOCK_BASED_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeIntraRefresh`](Extensions::KHR_VideoEncodeIntraRefresh)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_BLOCK_ROW_BASED_BIT_KHR")]
        const BLOCK_ROW_BASED_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeIntraRefresh`](Extensions::KHR_VideoEncodeIntraRefresh)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_BLOCK_COLUMN_BASED_BIT_KHR")]
        const BLOCK_COLUMN_BASED_KHR = 8;
    }
}

bitflags::bitflags! {
    /// [`VkDeviceFaultFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceFaultFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_DeviceFault`](Extensions::KHR_DeviceFault)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkDeviceFaultFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DeviceFaultFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_DeviceFault`](Extensions::KHR_DeviceFault)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_FAULT_FLAG_DEVICE_LOST_KHR")]
        const FLAG_DEVICE_LOST_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_DeviceFault`](Extensions::KHR_DeviceFault)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_FAULT_FLAG_MEMORY_ADDRESS_KHR")]
        const FLAG_MEMORY_ADDRESS_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_DeviceFault`](Extensions::KHR_DeviceFault)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_FAULT_FLAG_INSTRUCTION_ADDRESS_KHR")]
        const FLAG_INSTRUCTION_ADDRESS_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_DeviceFault`](Extensions::KHR_DeviceFault)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_FAULT_FLAG_VENDOR_KHR")]
        const FLAG_VENDOR_KHR = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_DeviceFault`](Extensions::KHR_DeviceFault)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_FAULT_FLAG_WATCHDOG_TIMEOUT_KHR")]
        const FLAG_WATCHDOG_TIMEOUT_KHR = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_DeviceFault`](Extensions::KHR_DeviceFault)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_FAULT_FLAG_OVERFLOW_KHR")]
        const FLAG_OVERFLOW_KHR = 32;
    }
}

bitflags::bitflags! {
    /// [`VkAccessFlagBits3KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlagBits3KHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_Maintenance8`](Extensions::KHR_Maintenance8)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkAccessFlagBits3KHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct AccessFlags3KHR: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance8`](Extensions::KHR_Maintenance8)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_3_NONE_KHR")]
        const NONE_KHR = 0;
    }
}

bitflags::bitflags! {
    /// [`VkVideoEncodePerPartitionFeedbackFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodePerPartitionFeedbackFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeFeedback2`](Extensions::KHR_VideoEncodeFeedback2)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodePerPartitionFeedbackFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodePerPartitionFeedbackFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeFeedback2`](Extensions::KHR_VideoEncodeFeedback2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_STATUS_BIT_KHR")]
        const STATUS_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeFeedback2`](Extensions::KHR_VideoEncodeFeedback2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_BITSTREAM_BUFFER_OFFSET_BIT_KHR")]
        const BITSTREAM_BUFFER_OFFSET_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeFeedback2`](Extensions::KHR_VideoEncodeFeedback2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_BITSTREAM_BYTES_WRITTEN_BIT_KHR")]
        const BITSTREAM_BYTES_WRITTEN_KHR = 4;
    }
}

bitflags::bitflags! {
    /// [`VkRenderingAttachmentFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingAttachmentFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_Maintenance10`](Extensions::KHR_Maintenance10)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkRenderingAttachmentFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct RenderingAttachmentFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance10`](Extensions::KHR_Maintenance10)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDERING_ATTACHMENT_INPUT_ATTACHMENT_FEEDBACK_BIT_KHR")]
        const INPUT_ATTACHMENT_FEEDBACK_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance10`](Extensions::KHR_Maintenance10)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDERING_ATTACHMENT_RESOLVE_SKIP_TRANSFER_FUNCTION_BIT_KHR")]
        const RESOLVE_SKIP_TRANSFER_FUNCTION_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance10`](Extensions::KHR_Maintenance10)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDERING_ATTACHMENT_RESOLVE_ENABLE_TRANSFER_FUNCTION_BIT_KHR")]
        const RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR = 4;
    }
}

bitflags::bitflags! {
    /// [`VkResolveImageFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkResolveImageFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_Maintenance10`](Extensions::KHR_Maintenance10)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkResolveImageFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ResolveImageFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance10`](Extensions::KHR_Maintenance10)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RESOLVE_IMAGE_SKIP_TRANSFER_FUNCTION_BIT_KHR")]
        const SKIP_TRANSFER_FUNCTION_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance10`](Extensions::KHR_Maintenance10)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RESOLVE_IMAGE_ENABLE_TRANSFER_FUNCTION_BIT_KHR")]
        const ENABLE_TRANSFER_FUNCTION_KHR = 2;
    }
}

bitflags::bitflags! {
    /// [`VkFormatFeatureFlagBits4KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlagBits4KHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkFormatFeatureFlagBits4KHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct FormatFeatureFlags4KHR: u64 {
    }
}

bitflags::bitflags! {
    /// [`VkImageUsageFlagBits2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageUsageFlagBits2KHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkImageUsageFlagBits2KHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ImageUsageFlags2KHR: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_TRANSFER_SRC_BIT_KHR")]
        const TRANSFER_SRC_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_TRANSFER_DST_BIT_KHR")]
        const TRANSFER_DST_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_SAMPLED_BIT_KHR")]
        const SAMPLED_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_STORAGE_BIT_KHR")]
        const STORAGE_KHR = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_COLOR_ATTACHMENT_BIT_KHR")]
        const COLOR_ATTACHMENT_KHR = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_DEPTH_STENCIL_ATTACHMENT_BIT_KHR")]
        const DEPTH_STENCIL_ATTACHMENT_KHR = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_TRANSIENT_ATTACHMENT_BIT_KHR")]
        const TRANSIENT_ATTACHMENT_KHR = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_INPUT_ATTACHMENT_BIT_KHR")]
        const INPUT_ATTACHMENT_KHR = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
        const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = 256;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_FRAGMENT_DENSITY_MAP_BIT_EXT")]
        const FRAGMENT_DENSITY_MAP_EXT = 512;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_VIDEO_DECODE_DST_BIT_KHR")]
        const VIDEO_DECODE_DST_KHR = 1024;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_VIDEO_DECODE_SRC_BIT_KHR")]
        const VIDEO_DECODE_SRC_KHR = 2048;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_VIDEO_DECODE_DPB_BIT_KHR")]
        const VIDEO_DECODE_DPB_KHR = 4096;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_VIDEO_ENCODE_DST_BIT_KHR")]
        const VIDEO_ENCODE_DST_KHR = 8192;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_VIDEO_ENCODE_SRC_BIT_KHR")]
        const VIDEO_ENCODE_SRC_KHR = 16384;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_VIDEO_ENCODE_DPB_BIT_KHR")]
        const VIDEO_ENCODE_DPB_KHR = 32768;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_INVOCATION_MASK_BIT_HUAWEI")]
        const INVOCATION_MASK_HUAWEI = 262144;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT")]
        const ATTACHMENT_FEEDBACK_LOOP_EXT = 524288;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_SAMPLE_WEIGHT_BIT_QCOM")]
        const SAMPLE_WEIGHT_QCOM = 1048576;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_SAMPLE_BLOCK_MATCH_BIT_QCOM")]
        const SAMPLE_BLOCK_MATCH_QCOM = 2097152;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_HOST_TRANSFER_BIT_KHR")]
        const HOST_TRANSFER_KHR = 4194304;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_TENSOR_ALIASING_BIT_ARM")]
        const TENSOR_ALIASING_ARM = 8388608;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR")]
        const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR = 33554432;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR")]
        const VIDEO_ENCODE_EMPHASIS_MAP_KHR = 67108864;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_TILE_MEMORY_BIT_QCOM")]
        const TILE_MEMORY_QCOM = 134217728;
    }
}

bitflags::bitflags! {
    /// [`VkImageCreateFlagBits2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCreateFlagBits2KHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkImageCreateFlagBits2KHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ImageCreateFlags2KHR: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_SPARSE_BINDING_BIT_KHR")]
        const SPARSE_BINDING_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_SPARSE_RESIDENCY_BIT_KHR")]
        const SPARSE_RESIDENCY_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_SPARSE_ALIASED_BIT_KHR")]
        const SPARSE_ALIASED_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_MUTABLE_FORMAT_BIT_KHR")]
        const MUTABLE_FORMAT_KHR = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_CUBE_COMPATIBLE_BIT_KHR")]
        const CUBE_COMPATIBLE_KHR = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance11`](Extensions::KHR_Maintenance11)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_ALIAS_SINGLE_LAYER_DESCRIPTOR_BIT_KHR")]
        const ALIAS_SINGLE_LAYER_DESCRIPTOR_KHR = 4194304;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_2D_ARRAY_COMPATIBLE_BIT_KHR")]
        const _2D_ARRAY_COMPATIBLE_KHR = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR")]
        const SPLIT_INSTANCE_BIND_REGIONS_KHR = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR")]
        const BLOCK_TEXEL_VIEW_COMPATIBLE_KHR = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_EXTENDED_USAGE_BIT_KHR")]
        const EXTENDED_USAGE_KHR = 256;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_DISJOINT_BIT_KHR")]
        const DISJOINT_KHR = 512;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_ALIAS_BIT_KHR")]
        const ALIAS_KHR = 1024;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_PROTECTED_BIT_KHR")]
        const PROTECTED_KHR = 2048;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT")]
        const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT = 4096;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_CORNER_SAMPLED_BIT_NV")]
        const CORNER_SAMPLED_NV = 8192;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_SUBSAMPLED_BIT_EXT")]
        const SUBSAMPLED_EXT = 16384;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_FRAGMENT_DENSITY_MAP_OFFSET_BIT_EXT")]
        const FRAGMENT_DENSITY_MAP_OFFSET_EXT = 32768;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT")]
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT = 65536;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_2D_VIEW_COMPATIBLE_BIT_EXT")]
        const _2D_VIEW_COMPATIBLE_EXT = 131072;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT")]
        const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT = 262144;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extensions::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_VIDEO_PROFILE_INDEPENDENT_BIT_KHR")]
        const VIDEO_PROFILE_INDEPENDENT_KHR = 1048576;
    }
}

bitflags::bitflags! {
    /// [`VkDebugReportFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugReportFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_DebugReport`](Extensions::EXT_DebugReport)
    /// - Extension [`EXT_DebugUtils`](Extensions::EXT_DebugUtils)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDebugReportFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DebugReportFlagsEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DebugReport`](Extensions::EXT_DebugReport)
        /// - Extension [`EXT_DebugUtils`](Extensions::EXT_DebugUtils)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_REPORT_INFORMATION_BIT_EXT")]
        const INFORMATION_EXT = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DebugReport`](Extensions::EXT_DebugReport)
        /// - Extension [`EXT_DebugUtils`](Extensions::EXT_DebugUtils)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_REPORT_WARNING_BIT_EXT")]
        const WARNING_EXT = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DebugReport`](Extensions::EXT_DebugReport)
        /// - Extension [`EXT_DebugUtils`](Extensions::EXT_DebugUtils)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT")]
        const PERFORMANCE_WARNING_EXT = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DebugReport`](Extensions::EXT_DebugReport)
        /// - Extension [`EXT_DebugUtils`](Extensions::EXT_DebugUtils)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_REPORT_ERROR_BIT_EXT")]
        const ERROR_EXT = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DebugReport`](Extensions::EXT_DebugReport)
        /// - Extension [`EXT_DebugUtils`](Extensions::EXT_DebugUtils)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_REPORT_DEBUG_BIT_EXT")]
        const DEBUG_EXT = 16;
    }
}

bitflags::bitflags! {
    /// [`VkExternalMemoryHandleTypeFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryHandleTypeFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_ExternalMemoryCapabilities`](Extensions::NV_ExternalMemoryCapabilities)
    /// - Extension [`KHR_ExternalMemoryCapabilities`](Extensions::KHR_ExternalMemoryCapabilities)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkExternalMemoryHandleTypeFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ExternalMemoryHandleTypeFlagsNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ExternalMemoryCapabilities`](Extensions::NV_ExternalMemoryCapabilities)
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extensions::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV")]
        const OPAQUE_WIN32_NV = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ExternalMemoryCapabilities`](Extensions::NV_ExternalMemoryCapabilities)
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extensions::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV")]
        const OPAQUE_WIN32_KMT_NV = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ExternalMemoryCapabilities`](Extensions::NV_ExternalMemoryCapabilities)
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extensions::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV")]
        const D3D11_IMAGE_NV = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ExternalMemoryCapabilities`](Extensions::NV_ExternalMemoryCapabilities)
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extensions::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV")]
        const D3D11_IMAGE_KMT_NV = 8;
    }
}

bitflags::bitflags! {
    /// [`VkExternalMemoryFeatureFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryFeatureFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_ExternalMemoryCapabilities`](Extensions::NV_ExternalMemoryCapabilities)
    /// - Extension [`KHR_ExternalMemoryCapabilities`](Extensions::KHR_ExternalMemoryCapabilities)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkExternalMemoryFeatureFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ExternalMemoryFeatureFlagsNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ExternalMemoryCapabilities`](Extensions::NV_ExternalMemoryCapabilities)
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extensions::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV")]
        const DEDICATED_ONLY_NV = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ExternalMemoryCapabilities`](Extensions::NV_ExternalMemoryCapabilities)
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extensions::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV")]
        const EXPORTABLE_NV = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ExternalMemoryCapabilities`](Extensions::NV_ExternalMemoryCapabilities)
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extensions::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV")]
        const IMPORTABLE_NV = 4;
    }
}

bitflags::bitflags! {
    /// [`VkSurfaceCounterFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCounterFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_DisplaySurfaceCounter`](Extensions::EXT_DisplaySurfaceCounter)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkSurfaceCounterFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SurfaceCounterFlagsEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DisplaySurfaceCounter`](Extensions::EXT_DisplaySurfaceCounter)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SURFACE_COUNTER_VBLANK_BIT_EXT")]
        const VBLANK_EXT = 1;
    }
}
impl SurfaceCounterFlagsEXT {}

bitflags::bitflags! {
    /// [`VkDebugUtilsMessageSeverityFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessageSeverityFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_DebugUtils`](Extensions::EXT_DebugUtils)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDebugUtilsMessageSeverityFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DebugUtilsMessageSeverityFlagsEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DebugUtils`](Extensions::EXT_DebugUtils)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT")]
        const VERBOSE_EXT = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DebugUtils`](Extensions::EXT_DebugUtils)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT")]
        const INFO_EXT = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DebugUtils`](Extensions::EXT_DebugUtils)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT")]
        const WARNING_EXT = 256;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DebugUtils`](Extensions::EXT_DebugUtils)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT")]
        const ERROR_EXT = 4096;
    }
}

bitflags::bitflags! {
    /// [`VkDebugUtilsMessageTypeFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessageTypeFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_DebugUtils`](Extensions::EXT_DebugUtils)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDebugUtilsMessageTypeFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DebugUtilsMessageTypeFlagsEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DebugUtils`](Extensions::EXT_DebugUtils)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT")]
        const GENERAL_EXT = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DebugUtils`](Extensions::EXT_DebugUtils)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT")]
        const VALIDATION_EXT = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DebugUtils`](Extensions::EXT_DebugUtils)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT")]
        const PERFORMANCE_EXT = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DeviceAddressBindingReport`](Extensions::EXT_DeviceAddressBindingReport)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_TYPE_DEVICE_ADDRESS_BINDING_BIT_EXT")]
        const DEVICE_ADDRESS_BINDING_EXT = 8;
    }
}

bitflags::bitflags! {
    /// [`VkGpaSqShaderStageFlagBitsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaSqShaderStageFlagBitsAMD.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`AMD_GpaInterface`](Extensions::AMD_GpaInterface)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkGpaSqShaderStageFlagBitsAMD")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct GpaSqShaderStageFlagsAMD: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMD_GpaInterface`](Extensions::AMD_GpaInterface)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GPA_SQ_SHADER_STAGE_PS_BIT_AMD")]
        const PS_AMD = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMD_GpaInterface`](Extensions::AMD_GpaInterface)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GPA_SQ_SHADER_STAGE_VS_BIT_AMD")]
        const VS_AMD = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMD_GpaInterface`](Extensions::AMD_GpaInterface)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GPA_SQ_SHADER_STAGE_GS_BIT_AMD")]
        const GS_AMD = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMD_GpaInterface`](Extensions::AMD_GpaInterface)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GPA_SQ_SHADER_STAGE_ES_BIT_AMD")]
        const ES_AMD = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMD_GpaInterface`](Extensions::AMD_GpaInterface)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GPA_SQ_SHADER_STAGE_HS_BIT_AMD")]
        const HS_AMD = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMD_GpaInterface`](Extensions::AMD_GpaInterface)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GPA_SQ_SHADER_STAGE_LS_BIT_AMD")]
        const LS_AMD = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMD_GpaInterface`](Extensions::AMD_GpaInterface)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GPA_SQ_SHADER_STAGE_CS_BIT_AMD")]
        const CS_AMD = 64;
    }
}

bitflags::bitflags! {
    /// [`VkTensorViewCreateFlagBitsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorViewCreateFlagBitsARM.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
    /// - Extension [`ARM_Tensors`](Extensions::ARM_Tensors)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkTensorViewCreateFlagBitsARM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct TensorViewCreateFlagsARM: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_Tensors`](Extensions::ARM_Tensors)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TENSOR_VIEW_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_ARM")]
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM = 1;
    }
}

bitflags::bitflags! {
    /// [`VkSpirvResourceTypeFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSpirvResourceTypeFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkSpirvResourceTypeFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct SpirvResourceTypeFlagsEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_ALL_EXT")]
        const ALL_EXT = 2147483647;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_SAMPLER_BIT_EXT")]
        const SAMPLER_EXT = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_SAMPLED_IMAGE_BIT_EXT")]
        const SAMPLED_IMAGE_EXT = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_READ_ONLY_IMAGE_BIT_EXT")]
        const READ_ONLY_IMAGE_EXT = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_READ_WRITE_IMAGE_BIT_EXT")]
        const READ_WRITE_IMAGE_EXT = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_COMBINED_SAMPLED_IMAGE_BIT_EXT")]
        const COMBINED_SAMPLED_IMAGE_EXT = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_UNIFORM_BUFFER_BIT_EXT")]
        const UNIFORM_BUFFER_EXT = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_READ_ONLY_STORAGE_BUFFER_BIT_EXT")]
        const READ_ONLY_STORAGE_BUFFER_EXT = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_READ_WRITE_STORAGE_BUFFER_BIT_EXT")]
        const READ_WRITE_STORAGE_BUFFER_EXT = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_ACCELERATION_STRUCTURE_BIT_EXT")]
        const ACCELERATION_STRUCTURE_EXT = 256;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_TENSOR_BIT_ARM")]
        const TENSOR_ARM = 512;
    }
}

bitflags::bitflags! {
    /// [`VkGeometryFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
    /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
    /// - Extension [`KHR_AccelerationStructure`](Extensions::KHR_AccelerationStructure)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkGeometryFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct GeometryFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extensions::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GEOMETRY_OPAQUE_BIT_KHR")]
        const OPAQUE_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extensions::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR")]
        const NO_DUPLICATE_ANY_HIT_INVOCATION_KHR = 2;
    }
}
/// [`VkGeometryFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryFlagBitsNV.html)
///
#[doc(alias = "VkGeometryFlagBitsNV")]
pub type GeometryFlagsNV = GeometryFlagsKHR;
impl GeometryFlagsKHR {
    /// [`VK_GEOMETRY_OPAQUE_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GEOMETRY_OPAQUE_BIT_NV.html)
    ///
    #[doc(alias = "VK_GEOMETRY_OPAQUE_BIT_NV")]
    pub const OPAQUE_NV: Self = Self::OPAQUE_KHR;
    /// [`VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_NV.html)
    ///
    #[doc(alias = "VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_NV")]
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION_NV: Self = Self::NO_DUPLICATE_ANY_HIT_INVOCATION_KHR;
}

bitflags::bitflags! {
    /// [`VkGeometryInstanceFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryInstanceFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
    /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
    /// - Extension [`KHR_AccelerationStructure`](Extensions::KHR_AccelerationStructure)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkGeometryInstanceFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct GeometryInstanceFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extensions::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR")]
        const TRIANGLE_FACING_CULL_DISABLE_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extensions::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR")]
        const TRIANGLE_FLIP_FACING_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extensions::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR")]
        const FORCE_OPAQUE_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extensions::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR")]
        const FORCE_NO_OPAQUE_KHR = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_OpacityMicromap`](Extensions::KHR_OpacityMicromap)
        /// - Extension [`EXT_OpacityMicromap`](Extensions::EXT_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_OPACITY_MICROMAP_2_STATE_BIT_KHR")]
        const FORCE_OPACITY_MICROMAP_2_STATE_KHR = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_OpacityMicromap`](Extensions::KHR_OpacityMicromap)
        /// - Extension [`EXT_OpacityMicromap`](Extensions::EXT_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GEOMETRY_INSTANCE_DISABLE_OPACITY_MICROMAPS_BIT_KHR")]
        const DISABLE_OPACITY_MICROMAPS_KHR = 32;
    }
}
/// [`VkGeometryInstanceFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryInstanceFlagBitsNV.html)
///
#[doc(alias = "VkGeometryInstanceFlagBitsNV")]
pub type GeometryInstanceFlagsNV = GeometryInstanceFlagsKHR;
impl GeometryInstanceFlagsKHR {
    /// [`VK_GEOMETRY_INSTANCE_TRIANGLE_CULL_DISABLE_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GEOMETRY_INSTANCE_TRIANGLE_CULL_DISABLE_BIT_NV.html)
    ///
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_CULL_DISABLE_BIT_NV")]
    pub const TRIANGLE_CULL_DISABLE_NV: Self = Self::TRIANGLE_FACING_CULL_DISABLE_KHR;
    /// [`VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR")]
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE_KHR: Self = Self::TRIANGLE_FLIP_FACING_KHR;
    /// [`VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_NV.html)
    ///
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_NV")]
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE_NV: Self = Self::TRIANGLE_FLIP_FACING_KHR;
    /// [`VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_NV.html)
    ///
    #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_NV")]
    pub const FORCE_OPAQUE_NV: Self = Self::FORCE_OPAQUE_KHR;
    /// [`VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_NV.html)
    ///
    #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_NV")]
    pub const FORCE_NO_OPAQUE_NV: Self = Self::FORCE_NO_OPAQUE_KHR;
    /// [`VK_GEOMETRY_INSTANCE_FORCE_OPACITY_MICROMAP_2_STATE_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GEOMETRY_INSTANCE_FORCE_OPACITY_MICROMAP_2_STATE_BIT_EXT.html)
    ///
    #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_OPACITY_MICROMAP_2_STATE_BIT_EXT")]
    pub const FORCE_OPACITY_MICROMAP_2_STATE_EXT: Self = Self::FORCE_OPACITY_MICROMAP_2_STATE_KHR;
    /// [`VK_GEOMETRY_INSTANCE_DISABLE_OPACITY_MICROMAPS_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GEOMETRY_INSTANCE_DISABLE_OPACITY_MICROMAPS_BIT_EXT.html)
    ///
    #[doc(alias = "VK_GEOMETRY_INSTANCE_DISABLE_OPACITY_MICROMAPS_BIT_EXT")]
    pub const DISABLE_OPACITY_MICROMAPS_EXT: Self = Self::DISABLE_OPACITY_MICROMAPS_KHR;
}

bitflags::bitflags! {
    /// [`VkBuildAccelerationStructureFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildAccelerationStructureFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
    /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
    /// - Extension [`KHR_AccelerationStructure`](Extensions::KHR_AccelerationStructure)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkBuildAccelerationStructureFlagBitsKHR")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct BuildAccelerationStructureFlagsKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extensions::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR")]
        const ALLOW_UPDATE_KHR = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extensions::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR")]
        const ALLOW_COMPACTION_KHR = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extensions::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR")]
        const PREFER_FAST_TRACE_KHR = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extensions::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR")]
        const PREFER_FAST_BUILD_KHR = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extensions::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extensions::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extensions::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR")]
        const LOW_MEMORY_KHR = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracingMotionBlur`](Extensions::NV_RayTracingMotionBlur)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV")]
        const MOTION_NV = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_OpacityMicromap`](Extensions::EXT_OpacityMicromap)
        /// - Extension [`KHR_OpacityMicromap`](Extensions::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_OPACITY_MICROMAP_DATA_UPDATE_BIT_EXT")]
        const ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT = 256;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DisplacementMicromap`](Extensions::NV_DisplacementMicromap)
        /// - Extension [`NV_ClusterAccelerationStructure`](Extensions::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_DISPLACEMENT_MICROMAP_UPDATE_BIT_NV")]
        const ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV = 512;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_RayTracingPositionFetch`](Extensions::KHR_RayTracingPositionFetch)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_DATA_ACCESS_BIT_KHR")]
        const ALLOW_DATA_ACCESS_KHR = 2048;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extensions::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_CLUSTER_OPACITY_MICROMAPS_BIT_NV")]
        const ALLOW_CLUSTER_OPACITY_MICROMAPS_NV = 4096;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_OpacityMicromap`](Extensions::KHR_OpacityMicromap)
        /// - Extension [`EXT_OpacityMicromap`](Extensions::EXT_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_OPACITY_MICROMAP_UPDATE_BIT_KHR")]
        const ALLOW_OPACITY_MICROMAP_UPDATE_KHR = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_OpacityMicromap`](Extensions::KHR_OpacityMicromap)
        /// - Extension [`EXT_OpacityMicromap`](Extensions::EXT_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_DISABLE_OPACITY_MICROMAPS_BIT_KHR")]
        const ALLOW_DISABLE_OPACITY_MICROMAPS_KHR = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_OpacityMicromap`](Extensions::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_MICROMAP_LOSSY_BIT_KHR")]
        const MICROMAP_LOSSY_KHR = 1024;
    }
}
/// [`VkBuildAccelerationStructureFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildAccelerationStructureFlagBitsNV.html)
///
#[doc(alias = "VkBuildAccelerationStructureFlagBitsNV")]
pub type BuildAccelerationStructureFlagsNV = BuildAccelerationStructureFlagsKHR;
impl BuildAccelerationStructureFlagsKHR {
    /// [`VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_NV.html)
    ///
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_NV")]
    pub const ALLOW_UPDATE_NV: Self = Self::ALLOW_UPDATE_KHR;
    /// [`VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_NV.html)
    ///
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_NV")]
    pub const ALLOW_COMPACTION_NV: Self = Self::ALLOW_COMPACTION_KHR;
    /// [`VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_NV.html)
    ///
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_NV")]
    pub const PREFER_FAST_TRACE_NV: Self = Self::PREFER_FAST_TRACE_KHR;
    /// [`VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_NV.html)
    ///
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_NV")]
    pub const PREFER_FAST_BUILD_NV: Self = Self::PREFER_FAST_BUILD_KHR;
    /// [`VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_NV.html)
    ///
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_NV")]
    pub const LOW_MEMORY_NV: Self = Self::LOW_MEMORY_KHR;
    /// [`VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_OPACITY_MICROMAP_UPDATE_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_OPACITY_MICROMAP_UPDATE_BIT_EXT.html)
    ///
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_OPACITY_MICROMAP_UPDATE_BIT_EXT")]
    pub const ALLOW_OPACITY_MICROMAP_UPDATE_EXT: Self = Self::ALLOW_OPACITY_MICROMAP_UPDATE_KHR;
    /// [`VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_DISABLE_OPACITY_MICROMAPS_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_DISABLE_OPACITY_MICROMAPS_BIT_EXT.html)
    ///
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_DISABLE_OPACITY_MICROMAPS_BIT_EXT")]
    pub const ALLOW_DISABLE_OPACITY_MICROMAPS_EXT: Self = Self::ALLOW_DISABLE_OPACITY_MICROMAPS_KHR;
}

bitflags::bitflags! {
    /// [`VkPipelineCompilerControlFlagBitsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCompilerControlFlagBitsAMD.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`AMD_PipelineCompilerControl`](Extensions::AMD_PipelineCompilerControl)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkPipelineCompilerControlFlagBitsAMD")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PipelineCompilerControlFlagsAMD: u32 {
    }
}

bitflags::bitflags! {
    /// [`VkPresentStageFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentStageFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_PresentTiming`](Extensions::EXT_PresentTiming)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkPresentStageFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PresentStageFlagsEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_PresentTiming`](Extensions::EXT_PresentTiming)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRESENT_STAGE_QUEUE_OPERATIONS_END_BIT_EXT")]
        const QUEUE_OPERATIONS_END_EXT = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_PresentTiming`](Extensions::EXT_PresentTiming)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRESENT_STAGE_REQUEST_DEQUEUED_BIT_EXT")]
        const REQUEST_DEQUEUED_EXT = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_PresentTiming`](Extensions::EXT_PresentTiming)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRESENT_STAGE_IMAGE_FIRST_PIXEL_OUT_BIT_EXT")]
        const IMAGE_FIRST_PIXEL_OUT_EXT = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_PresentTiming`](Extensions::EXT_PresentTiming)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRESENT_STAGE_IMAGE_FIRST_PIXEL_VISIBLE_BIT_EXT")]
        const IMAGE_FIRST_PIXEL_VISIBLE_EXT = 8;
    }
}

bitflags::bitflags! {
    /// [`VkPastPresentationTimingFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPastPresentationTimingFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_PresentTiming`](Extensions::EXT_PresentTiming)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkPastPresentationTimingFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PastPresentationTimingFlagsEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_PresentTiming`](Extensions::EXT_PresentTiming)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PAST_PRESENTATION_TIMING_ALLOW_PARTIAL_RESULTS_BIT_EXT")]
        const ALLOW_PARTIAL_RESULTS_EXT = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_PresentTiming`](Extensions::EXT_PresentTiming)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PAST_PRESENTATION_TIMING_ALLOW_OUT_OF_ORDER_RESULTS_BIT_EXT")]
        const ALLOW_OUT_OF_ORDER_RESULTS_EXT = 2;
    }
}

bitflags::bitflags! {
    /// [`VkPresentTimingInfoFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentTimingInfoFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_PresentTiming`](Extensions::EXT_PresentTiming)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkPresentTimingInfoFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PresentTimingInfoFlagsEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_PresentTiming`](Extensions::EXT_PresentTiming)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRESENT_TIMING_INFO_PRESENT_AT_RELATIVE_TIME_BIT_EXT")]
        const PRESENT_AT_RELATIVE_TIME_EXT = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_PresentTiming`](Extensions::EXT_PresentTiming)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRESENT_TIMING_INFO_PRESENT_AT_NEAREST_REFRESH_CYCLE_BIT_EXT")]
        const PRESENT_AT_NEAREST_REFRESH_CYCLE_EXT = 2;
    }
}

bitflags::bitflags! {
    /// [`VkShaderCorePropertiesFlagBitsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderCorePropertiesFlagBitsAMD.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`AMD_ShaderCoreProperties2`](Extensions::AMD_ShaderCoreProperties2)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkShaderCorePropertiesFlagBitsAMD")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ShaderCorePropertiesFlagsAMD: u32 {
    }
}

bitflags::bitflags! {
    /// [`VkIndirectStateFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectStateFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_DeviceGeneratedCommands`](Extensions::NV_DeviceGeneratedCommands)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkIndirectStateFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct IndirectStateFlagsNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceGeneratedCommands`](Extensions::NV_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_INDIRECT_STATE_FLAG_FRONTFACE_BIT_NV")]
        const FLAG_FRONTFACE_NV = 1;
    }
}

bitflags::bitflags! {
    /// [`VkIndirectCommandsLayoutUsageFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutUsageFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_DeviceGeneratedCommands`](Extensions::NV_DeviceGeneratedCommands)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkIndirectCommandsLayoutUsageFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct IndirectCommandsLayoutUsageFlagsNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceGeneratedCommands`](Extensions::NV_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV")]
        const EXPLICIT_PREPROCESS_NV = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceGeneratedCommands`](Extensions::NV_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV")]
        const INDEXED_SEQUENCES_NV = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceGeneratedCommands`](Extensions::NV_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV")]
        const UNORDERED_SEQUENCES_NV = 4;
    }
}

bitflags::bitflags! {
    /// [`VkDeviceDiagnosticsConfigFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceDiagnosticsConfigFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_DeviceDiagnosticsConfig`](Extensions::NV_DeviceDiagnosticsConfig)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDeviceDiagnosticsConfigFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DeviceDiagnosticsConfigFlagsNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceDiagnosticsConfig`](Extensions::NV_DeviceDiagnosticsConfig)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_DEBUG_INFO_BIT_NV")]
        const ENABLE_SHADER_DEBUG_INFO_NV = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceDiagnosticsConfig`](Extensions::NV_DeviceDiagnosticsConfig)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_RESOURCE_TRACKING_BIT_NV")]
        const ENABLE_RESOURCE_TRACKING_NV = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceDiagnosticsConfig`](Extensions::NV_DeviceDiagnosticsConfig)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_AUTOMATIC_CHECKPOINTS_BIT_NV")]
        const ENABLE_AUTOMATIC_CHECKPOINTS_NV = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceDiagnosticsConfig`](Extensions::NV_DeviceDiagnosticsConfig)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_ERROR_REPORTING_BIT_NV")]
        const ENABLE_SHADER_ERROR_REPORTING_NV = 8;
    }
}

bitflags::bitflags! {
    /// [`VkTileShadingRenderPassFlagBitsQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkTileShadingRenderPassFlagBitsQCOM.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`QCOM_TileShading`](Extensions::QCOM_TileShading)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkTileShadingRenderPassFlagBitsQCOM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct TileShadingRenderPassFlagsQCOM: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_TileShading`](Extensions::QCOM_TileShading)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TILE_SHADING_RENDER_PASS_ENABLE_BIT_QCOM")]
        const ENABLE_QCOM = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_TileShading`](Extensions::QCOM_TileShading)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TILE_SHADING_RENDER_PASS_PER_TILE_EXECUTION_BIT_QCOM")]
        const PER_TILE_EXECUTION_QCOM = 2;
    }
}

bitflags::bitflags! {
    /// [`VkExportMetalObjectTypeFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExportMetalObjectTypeFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_MetalObjects`](Extensions::EXT_MetalObjects)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkExportMetalObjectTypeFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ExportMetalObjectTypeFlagsEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MetalObjects`](Extensions::EXT_MetalObjects)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXPORT_METAL_OBJECT_TYPE_METAL_DEVICE_BIT_EXT")]
        const METAL_DEVICE_EXT = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MetalObjects`](Extensions::EXT_MetalObjects)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXPORT_METAL_OBJECT_TYPE_METAL_COMMAND_QUEUE_BIT_EXT")]
        const METAL_COMMAND_QUEUE_EXT = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MetalObjects`](Extensions::EXT_MetalObjects)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXPORT_METAL_OBJECT_TYPE_METAL_BUFFER_BIT_EXT")]
        const METAL_BUFFER_EXT = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MetalObjects`](Extensions::EXT_MetalObjects)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXPORT_METAL_OBJECT_TYPE_METAL_TEXTURE_BIT_EXT")]
        const METAL_TEXTURE_EXT = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MetalObjects`](Extensions::EXT_MetalObjects)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXPORT_METAL_OBJECT_TYPE_METAL_IOSURFACE_BIT_EXT")]
        const METAL_IOSURFACE_EXT = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MetalObjects`](Extensions::EXT_MetalObjects)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXPORT_METAL_OBJECT_TYPE_METAL_SHARED_EVENT_BIT_EXT")]
        const METAL_SHARED_EVENT_EXT = 32;
    }
}

bitflags::bitflags! {
    /// [`VkGraphicsPipelineLibraryFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkGraphicsPipelineLibraryFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_GraphicsPipelineLibrary`](Extensions::EXT_GraphicsPipelineLibrary)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkGraphicsPipelineLibraryFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct GraphicsPipelineLibraryFlagsEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_GraphicsPipelineLibrary`](Extensions::EXT_GraphicsPipelineLibrary)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GRAPHICS_PIPELINE_LIBRARY_VERTEX_INPUT_INTERFACE_BIT_EXT")]
        const VERTEX_INPUT_INTERFACE_EXT = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_GraphicsPipelineLibrary`](Extensions::EXT_GraphicsPipelineLibrary)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GRAPHICS_PIPELINE_LIBRARY_PRE_RASTERIZATION_SHADERS_BIT_EXT")]
        const PRE_RASTERIZATION_SHADERS_EXT = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_GraphicsPipelineLibrary`](Extensions::EXT_GraphicsPipelineLibrary)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GRAPHICS_PIPELINE_LIBRARY_FRAGMENT_SHADER_BIT_EXT")]
        const FRAGMENT_SHADER_EXT = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_GraphicsPipelineLibrary`](Extensions::EXT_GraphicsPipelineLibrary)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GRAPHICS_PIPELINE_LIBRARY_FRAGMENT_OUTPUT_INTERFACE_BIT_EXT")]
        const FRAGMENT_OUTPUT_INTERFACE_EXT = 8;
    }
}

bitflags::bitflags! {
    /// [`VkImageCompressionFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCompressionFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkImageCompressionFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ImageCompressionFlagsEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_DEFAULT_EXT")]
        const DEFAULT_EXT = 0;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_DEFAULT_EXT")]
        const FIXED_RATE_DEFAULT_EXT = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_EXPLICIT_EXT")]
        const FIXED_RATE_EXPLICIT_EXT = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_DISABLED_EXT")]
        const DISABLED_EXT = 4;
    }
}

bitflags::bitflags! {
    /// [`VkImageCompressionFixedRateFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCompressionFixedRateFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkImageCompressionFixedRateFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ImageCompressionFixedRateFlagsEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_NONE_EXT")]
        const NONE_EXT = 0;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_1BPC_BIT_EXT")]
        const _1BPC_EXT = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_2BPC_BIT_EXT")]
        const _2BPC_EXT = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_3BPC_BIT_EXT")]
        const _3BPC_EXT = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_4BPC_BIT_EXT")]
        const _4BPC_EXT = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_5BPC_BIT_EXT")]
        const _5BPC_EXT = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_6BPC_BIT_EXT")]
        const _6BPC_EXT = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_7BPC_BIT_EXT")]
        const _7BPC_EXT = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_8BPC_BIT_EXT")]
        const _8BPC_EXT = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_9BPC_BIT_EXT")]
        const _9BPC_EXT = 256;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_10BPC_BIT_EXT")]
        const _10BPC_EXT = 512;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_11BPC_BIT_EXT")]
        const _11BPC_EXT = 1024;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_12BPC_BIT_EXT")]
        const _12BPC_EXT = 2048;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_13BPC_BIT_EXT")]
        const _13BPC_EXT = 4096;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_14BPC_BIT_EXT")]
        const _14BPC_EXT = 8192;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_15BPC_BIT_EXT")]
        const _15BPC_EXT = 16384;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_16BPC_BIT_EXT")]
        const _16BPC_EXT = 32768;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_17BPC_BIT_EXT")]
        const _17BPC_EXT = 65536;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_18BPC_BIT_EXT")]
        const _18BPC_EXT = 131072;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_19BPC_BIT_EXT")]
        const _19BPC_EXT = 262144;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_20BPC_BIT_EXT")]
        const _20BPC_EXT = 524288;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_21BPC_BIT_EXT")]
        const _21BPC_EXT = 1048576;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_22BPC_BIT_EXT")]
        const _22BPC_EXT = 2097152;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_23BPC_BIT_EXT")]
        const _23BPC_EXT = 4194304;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extensions::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_24BPC_BIT_EXT")]
        const _24BPC_EXT = 8388608;
    }
}

bitflags::bitflags! {
    /// [`VkDeviceAddressBindingFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceAddressBindingFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_DeviceAddressBindingReport`](Extensions::EXT_DeviceAddressBindingReport)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDeviceAddressBindingFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DeviceAddressBindingFlagsEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DeviceAddressBindingReport`](Extensions::EXT_DeviceAddressBindingReport)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_ADDRESS_BINDING_INTERNAL_OBJECT_BIT_EXT")]
        const INTERNAL_OBJECT_EXT = 1;
    }
}

bitflags::bitflags! {
    /// [`VkImageConstraintsInfoFlagBitsFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageConstraintsInfoFlagBitsFUCHSIA.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`FUCHSIA_BufferCollection`](Extensions::FUCHSIA_BufferCollection)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkImageConstraintsInfoFlagBitsFUCHSIA")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ImageConstraintsInfoFlagsFUCHSIA: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`FUCHSIA_BufferCollection`](Extensions::FUCHSIA_BufferCollection)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_RARELY_FUCHSIA")]
        const CPU_READ_RARELY_FUCHSIA = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`FUCHSIA_BufferCollection`](Extensions::FUCHSIA_BufferCollection)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_OFTEN_FUCHSIA")]
        const CPU_READ_OFTEN_FUCHSIA = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`FUCHSIA_BufferCollection`](Extensions::FUCHSIA_BufferCollection)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_RARELY_FUCHSIA")]
        const CPU_WRITE_RARELY_FUCHSIA = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`FUCHSIA_BufferCollection`](Extensions::FUCHSIA_BufferCollection)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_OFTEN_FUCHSIA")]
        const CPU_WRITE_OFTEN_FUCHSIA = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`FUCHSIA_BufferCollection`](Extensions::FUCHSIA_BufferCollection)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_PROTECTED_OPTIONAL_FUCHSIA")]
        const PROTECTED_OPTIONAL_FUCHSIA = 16;
    }
}

bitflags::bitflags! {
    /// [`VkFrameBoundaryFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFrameBoundaryFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_FrameBoundary`](Extensions::EXT_FrameBoundary)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkFrameBoundaryFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct FrameBoundaryFlagsEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_FrameBoundary`](Extensions::EXT_FrameBoundary)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FRAME_BOUNDARY_FRAME_END_BIT_EXT")]
        const FRAME_END_EXT = 1;
    }
}

bitflags::bitflags! {
    /// [`VkVideoEncodeRgbModelConversionFlagBitsVALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbModelConversionFlagBitsVALVE.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`VALVE_VideoEncodeRgbConversion`](Extensions::VALVE_VideoEncodeRgbConversion)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodeRgbModelConversionFlagBitsVALVE")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeRgbModelConversionFlagsVALVE: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_VideoEncodeRgbConversion`](Extensions::VALVE_VideoEncodeRgbConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_RGB_IDENTITY_BIT_VALVE")]
        const RGB_IDENTITY_VALVE = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_VideoEncodeRgbConversion`](Extensions::VALVE_VideoEncodeRgbConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_IDENTITY_BIT_VALVE")]
        const YCBCR_IDENTITY_VALVE = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_VideoEncodeRgbConversion`](Extensions::VALVE_VideoEncodeRgbConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_709_BIT_VALVE")]
        const YCBCR_709_VALVE = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_VideoEncodeRgbConversion`](Extensions::VALVE_VideoEncodeRgbConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_601_BIT_VALVE")]
        const YCBCR_601_VALVE = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_VideoEncodeRgbConversion`](Extensions::VALVE_VideoEncodeRgbConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_2020_BIT_VALVE")]
        const YCBCR_2020_VALVE = 16;
    }
}

bitflags::bitflags! {
    /// [`VkVideoEncodeRgbRangeCompressionFlagBitsVALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbRangeCompressionFlagBitsVALVE.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`VALVE_VideoEncodeRgbConversion`](Extensions::VALVE_VideoEncodeRgbConversion)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodeRgbRangeCompressionFlagBitsVALVE")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeRgbRangeCompressionFlagsVALVE: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_VideoEncodeRgbConversion`](Extensions::VALVE_VideoEncodeRgbConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_RANGE_COMPRESSION_FULL_RANGE_BIT_VALVE")]
        const FULL_RANGE_VALVE = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_VideoEncodeRgbConversion`](Extensions::VALVE_VideoEncodeRgbConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_RANGE_COMPRESSION_NARROW_RANGE_BIT_VALVE")]
        const NARROW_RANGE_VALVE = 2;
    }
}

bitflags::bitflags! {
    /// [`VkVideoEncodeRgbChromaOffsetFlagBitsVALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbChromaOffsetFlagBitsVALVE.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`VALVE_VideoEncodeRgbConversion`](Extensions::VALVE_VideoEncodeRgbConversion)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodeRgbChromaOffsetFlagBitsVALVE")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct VideoEncodeRgbChromaOffsetFlagsVALVE: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_VideoEncodeRgbConversion`](Extensions::VALVE_VideoEncodeRgbConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_CHROMA_OFFSET_COSITED_EVEN_BIT_VALVE")]
        const COSITED_EVEN_VALVE = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_VideoEncodeRgbConversion`](Extensions::VALVE_VideoEncodeRgbConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_CHROMA_OFFSET_MIDPOINT_BIT_VALVE")]
        const MIDPOINT_VALVE = 2;
    }
}

bitflags::bitflags! {
    /// [`VkBuildMicromapFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildMicromapFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_OpacityMicromap`](Extensions::EXT_OpacityMicromap)
    /// - Extension [`KHR_OpacityMicromap`](Extensions::KHR_OpacityMicromap)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkBuildMicromapFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct BuildMicromapFlagsEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_OpacityMicromap`](Extensions::EXT_OpacityMicromap)
        /// - Extension [`KHR_OpacityMicromap`](Extensions::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_MICROMAP_PREFER_FAST_TRACE_BIT_EXT")]
        const PREFER_FAST_TRACE_EXT = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_OpacityMicromap`](Extensions::EXT_OpacityMicromap)
        /// - Extension [`KHR_OpacityMicromap`](Extensions::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_MICROMAP_PREFER_FAST_BUILD_BIT_EXT")]
        const PREFER_FAST_BUILD_EXT = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_OpacityMicromap`](Extensions::EXT_OpacityMicromap)
        /// - Extension [`KHR_OpacityMicromap`](Extensions::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_MICROMAP_ALLOW_COMPACTION_BIT_EXT")]
        const ALLOW_COMPACTION_EXT = 4;
    }
}

bitflags::bitflags! {
    /// [`VkMicromapCreateFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapCreateFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_OpacityMicromap`](Extensions::EXT_OpacityMicromap)
    /// - Extension [`KHR_OpacityMicromap`](Extensions::KHR_OpacityMicromap)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkMicromapCreateFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct MicromapCreateFlagsEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_OpacityMicromap`](Extensions::EXT_OpacityMicromap)
        /// - Extension [`KHR_OpacityMicromap`](Extensions::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MICROMAP_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT")]
        const DEVICE_ADDRESS_CAPTURE_REPLAY_EXT = 1;
    }
}

bitflags::bitflags! {
    /// [`VkPhysicalDeviceSchedulingControlsFlagBitsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSchedulingControlsFlagBitsARM.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`ARM_SchedulingControls`](Extensions::ARM_SchedulingControls)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkPhysicalDeviceSchedulingControlsFlagBitsARM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PhysicalDeviceSchedulingControlsFlagsARM: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_SchedulingControls`](Extensions::ARM_SchedulingControls)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PHYSICAL_DEVICE_SCHEDULING_CONTROLS_SHADER_CORE_COUNT_ARM")]
        const SHADER_CORE_COUNT_ARM = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_SchedulingControls`](Extensions::ARM_SchedulingControls)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PHYSICAL_DEVICE_SCHEDULING_CONTROLS_DISPATCH_PARAMETERS_ARM")]
        const DISPATCH_PARAMETERS_ARM = 2;
    }
}

bitflags::bitflags! {
    /// [`VkMemoryDecompressionMethodFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryDecompressionMethodFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_MemoryDecompression`](Extensions::NV_MemoryDecompression)
    /// - Extension [`EXT_MemoryDecompression`](Extensions::EXT_MemoryDecompression)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkMemoryDecompressionMethodFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct MemoryDecompressionMethodFlagsEXT: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MemoryDecompression`](Extensions::EXT_MemoryDecompression)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MEMORY_DECOMPRESSION_METHOD_GDEFLATE_1_0_BIT_EXT")]
        const GDEFLATE_1_0_EXT = 1;
    }
}
/// [`VkMemoryDecompressionMethodFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryDecompressionMethodFlagBitsNV.html)
///
#[doc(alias = "VkMemoryDecompressionMethodFlagBitsNV")]
pub type MemoryDecompressionMethodFlagsNV = MemoryDecompressionMethodFlagsEXT;
impl MemoryDecompressionMethodFlagsEXT {
    /// [`VK_MEMORY_DECOMPRESSION_METHOD_GDEFLATE_1_0_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MEMORY_DECOMPRESSION_METHOD_GDEFLATE_1_0_BIT_NV.html)
    ///
    #[doc(alias = "VK_MEMORY_DECOMPRESSION_METHOD_GDEFLATE_1_0_BIT_NV")]
    pub const GDEFLATE_1_0_NV: Self = Self::GDEFLATE_1_0_EXT;
}

bitflags::bitflags! {
    /// [`VkTensorCreateFlagBitsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorCreateFlagBitsARM.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`ARM_Tensors`](Extensions::ARM_Tensors)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkTensorCreateFlagBitsARM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct TensorCreateFlagsARM: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_Tensors`](Extensions::ARM_Tensors)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TENSOR_CREATE_MUTABLE_FORMAT_BIT_ARM")]
        const MUTABLE_FORMAT_ARM = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_Tensors`](Extensions::ARM_Tensors)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TENSOR_CREATE_PROTECTED_BIT_ARM")]
        const PROTECTED_ARM = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TENSOR_CREATE_DESCRIPTOR_HEAP_CAPTURE_REPLAY_BIT_ARM")]
        const DESCRIPTOR_HEAP_CAPTURE_REPLAY_ARM = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_Tensors`](Extensions::ARM_Tensors)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TENSOR_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_ARM")]
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM = 4;
    }
}

bitflags::bitflags! {
    /// [`VkTensorUsageFlagBitsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorUsageFlagBitsARM.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`ARM_Tensors`](Extensions::ARM_Tensors)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkTensorUsageFlagBitsARM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct TensorUsageFlagsARM: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_Tensors`](Extensions::ARM_Tensors)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TENSOR_USAGE_SHADER_BIT_ARM")]
        const SHADER_ARM = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_Tensors`](Extensions::ARM_Tensors)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TENSOR_USAGE_TRANSFER_SRC_BIT_ARM")]
        const TRANSFER_SRC_ARM = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_Tensors`](Extensions::ARM_Tensors)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TENSOR_USAGE_TRANSFER_DST_BIT_ARM")]
        const TRANSFER_DST_ARM = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_Tensors`](Extensions::ARM_Tensors)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TENSOR_USAGE_IMAGE_ALIASING_BIT_ARM")]
        const IMAGE_ALIASING_ARM = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraph`](Extensions::ARM_DataGraph)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TENSOR_USAGE_DATA_GRAPH_BIT_ARM")]
        const DATA_GRAPH_ARM = 32;
    }
}

bitflags::bitflags! {
    /// [`VkOpticalFlowGridSizeFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowGridSizeFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkOpticalFlowGridSizeFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct OpticalFlowGridSizeFlagsNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_GRID_SIZE_UNKNOWN_NV")]
        const UNKNOWN_NV = 0;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_GRID_SIZE_1X1_BIT_NV")]
        const _1X1_NV = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_GRID_SIZE_2X2_BIT_NV")]
        const _2X2_NV = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_GRID_SIZE_4X4_BIT_NV")]
        const _4X4_NV = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_GRID_SIZE_8X8_BIT_NV")]
        const _8X8_NV = 8;
    }
}

bitflags::bitflags! {
    /// [`VkOpticalFlowUsageFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowUsageFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkOpticalFlowUsageFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct OpticalFlowUsageFlagsNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_USAGE_UNKNOWN_NV")]
        const UNKNOWN_NV = 0;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_USAGE_INPUT_BIT_NV")]
        const INPUT_NV = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_USAGE_OUTPUT_BIT_NV")]
        const OUTPUT_NV = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_USAGE_HINT_BIT_NV")]
        const HINT_NV = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_USAGE_COST_BIT_NV")]
        const COST_NV = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_USAGE_GLOBAL_FLOW_BIT_NV")]
        const GLOBAL_FLOW_NV = 16;
    }
}

bitflags::bitflags! {
    /// [`VkOpticalFlowSessionCreateFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowSessionCreateFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkOpticalFlowSessionCreateFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct OpticalFlowSessionCreateFlagsNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_HINT_BIT_NV")]
        const ENABLE_HINT_NV = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_COST_BIT_NV")]
        const ENABLE_COST_NV = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_GLOBAL_FLOW_BIT_NV")]
        const ENABLE_GLOBAL_FLOW_NV = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_SESSION_CREATE_ALLOW_REGIONS_BIT_NV")]
        const ALLOW_REGIONS_NV = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_SESSION_CREATE_BOTH_DIRECTIONS_BIT_NV")]
        const BOTH_DIRECTIONS_NV = 16;
    }
}

bitflags::bitflags! {
    /// [`VkOpticalFlowExecuteFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowExecuteFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkOpticalFlowExecuteFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct OpticalFlowExecuteFlagsNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extensions::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_EXECUTE_DISABLE_TEMPORAL_HINTS_BIT_NV")]
        const DISABLE_TEMPORAL_HINTS_NV = 1;
    }
}

bitflags::bitflags! {
    /// [`VkShaderCreateFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderCreateFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_ShaderObject`](Extensions::EXT_ShaderObject)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkShaderCreateFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ShaderCreateFlagsEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ShaderObject`](Extensions::EXT_ShaderObject)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_LINK_STAGE_BIT_EXT")]
        const LINK_STAGE_EXT = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extensions::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_DESCRIPTOR_HEAP_BIT_EXT")]
        const DESCRIPTOR_HEAP_EXT = 1024;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extensions::KHR_Maintenance5)
        /// - Extension [`ARM_ShaderInstrumentation`](Extensions::ARM_ShaderInstrumentation)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_INSTRUMENT_SHADER_BIT_ARM")]
        const INSTRUMENT_SHADER_ARM = 2048;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ShaderObject`](Extensions::EXT_ShaderObject)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT_EXT")]
        const ALLOW_VARYING_SUBGROUP_SIZE_EXT = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ShaderObject`](Extensions::EXT_ShaderObject)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_REQUIRE_FULL_SUBGROUPS_BIT_EXT")]
        const REQUIRE_FULL_SUBGROUPS_EXT = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ShaderObject`](Extensions::EXT_ShaderObject)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_NO_TASK_SHADER_BIT_EXT")]
        const NO_TASK_SHADER_EXT = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ShaderObject`](Extensions::EXT_ShaderObject)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_DISPATCH_BASE_BIT_EXT")]
        const DISPATCH_BASE_EXT = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ShaderObject`](Extensions::EXT_ShaderObject)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_EXT")]
        const FRAGMENT_SHADING_RATE_ATTACHMENT_EXT = 32;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ShaderObject`](Extensions::EXT_ShaderObject)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT")]
        const FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT = 64;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DeviceGeneratedCommands`](Extensions::EXT_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_INDIRECT_BINDABLE_BIT_EXT")]
        const INDIRECT_BINDABLE_EXT = 128;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_OpacityMicromap`](Extensions::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_OPACITY_MICROMAP_DISALLOW_MIXED_SPECIAL_INDEX_BIT_EXT")]
        const OPACITY_MICROMAP_DISALLOW_MIXED_SPECIAL_INDEX_EXT = 4096;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_Shader64BitIndexing`](Extensions::EXT_Shader64BitIndexing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_64_BIT_INDEXING_BIT_EXT")]
        const _64_INDEXING_EXT = 32768;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance11`](Extensions::KHR_Maintenance11)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_INDEPENDENT_SETS_BIT_KHR")]
        const INDEPENDENT_SETS_KHR = 262144;
    }
}

bitflags::bitflags! {
    /// [`VkDataGraphPipelineSessionCreateFlagBitsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionCreateFlagBitsARM.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`ARM_DataGraph`](Extensions::ARM_DataGraph)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDataGraphPipelineSessionCreateFlagBitsARM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DataGraphPipelineSessionCreateFlagsARM: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraph`](Extensions::ARM_DataGraph)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_PIPELINE_SESSION_CREATE_PROTECTED_BIT_ARM")]
        const PROTECTED_ARM = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_PIPELINE_SESSION_CREATE_OPTICAL_FLOW_CACHE_BIT_ARM")]
        const OPTICAL_FLOW_CACHE_ARM = 2;
    }
}

bitflags::bitflags! {
    /// [`VkDataGraphPipelineDispatchFlagBitsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineDispatchFlagBitsARM.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`ARM_DataGraph`](Extensions::ARM_DataGraph)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDataGraphPipelineDispatchFlagBitsARM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DataGraphPipelineDispatchFlagsARM: u64 {
    }
}

bitflags::bitflags! {
    /// [`VkDataGraphTOSAQualityFlagBitsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphTOSAQualityFlagBitsARM.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`ARM_DataGraphInstructionSetTosa`](Extensions::ARM_DataGraphInstructionSetTosa)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkDataGraphTOSAQualityFlagBitsARM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DataGraphTOSAQualityFlagsARM: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphInstructionSetTosa`](Extensions::ARM_DataGraphInstructionSetTosa)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_TOSA_QUALITY_ACCELERATED_ARM")]
        const ACCELERATED_ARM = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphInstructionSetTosa`](Extensions::ARM_DataGraphInstructionSetTosa)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_TOSA_QUALITY_CONFORMANT_ARM")]
        const CONFORMANT_ARM = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphInstructionSetTosa`](Extensions::ARM_DataGraphInstructionSetTosa)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_TOSA_QUALITY_EXPERIMENTAL_ARM")]
        const EXPERIMENTAL_ARM = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphInstructionSetTosa`](Extensions::ARM_DataGraphInstructionSetTosa)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_TOSA_QUALITY_DEPRECATED_ARM")]
        const DEPRECATED_ARM = 8;
    }
}

bitflags::bitflags! {
    /// [`VkClusterAccelerationStructureAddressResolutionFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureAddressResolutionFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_ClusterAccelerationStructure`](Extensions::NV_ClusterAccelerationStructure)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkClusterAccelerationStructureAddressResolutionFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ClusterAccelerationStructureAddressResolutionFlagsNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extensions::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_NONE_NV")]
        const NONE_NV = 0;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extensions::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_IMPLICIT_DATA_BIT_NV")]
        const INDIRECTED_DST_IMPLICIT_DATA_NV = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extensions::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SCRATCH_DATA_BIT_NV")]
        const INDIRECTED_SCRATCH_DATA_NV = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extensions::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_ADDRESS_ARRAY_BIT_NV")]
        const INDIRECTED_DST_ADDRESS_ARRAY_NV = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extensions::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_SIZES_ARRAY_BIT_NV")]
        const INDIRECTED_DST_SIZES_ARRAY_NV = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extensions::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SRC_INFOS_ARRAY_BIT_NV")]
        const INDIRECTED_SRC_INFOS_ARRAY_NV = 16;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extensions::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SRC_INFOS_COUNT_BIT_NV")]
        const INDIRECTED_SRC_INFOS_COUNT_NV = 32;
    }
}

bitflags::bitflags! {
    /// [`VkClusterAccelerationStructureClusterFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureClusterFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_ClusterAccelerationStructure`](Extensions::NV_ClusterAccelerationStructure)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkClusterAccelerationStructureClusterFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ClusterAccelerationStructureClusterFlagsNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extensions::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_CLUSTER_ALLOW_DISABLE_OPACITY_MICROMAPS_NV")]
        const ALLOW_DISABLE_OPACITY_MICROMAPS_NV = 1;
    }
}

bitflags::bitflags! {
    /// [`VkClusterAccelerationStructureGeometryFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureGeometryFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_ClusterAccelerationStructure`](Extensions::NV_ClusterAccelerationStructure)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkClusterAccelerationStructureGeometryFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ClusterAccelerationStructureGeometryFlagsNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extensions::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_CULL_DISABLE_BIT_NV")]
        const CULL_DISABLE_NV = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extensions::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_NO_DUPLICATE_ANYHIT_INVOCATION_BIT_NV")]
        const NO_DUPLICATE_ANYHIT_INVOCATION_NV = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extensions::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_OPAQUE_BIT_NV")]
        const OPAQUE_NV = 4;
    }
}

bitflags::bitflags! {
    /// [`VkClusterAccelerationStructureIndexFormatFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureIndexFormatFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_ClusterAccelerationStructure`](Extensions::NV_ClusterAccelerationStructure)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkClusterAccelerationStructureIndexFormatFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct ClusterAccelerationStructureIndexFormatFlagsNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extensions::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_8BIT_NV")]
        const _8BIT_NV = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extensions::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_16BIT_NV")]
        const _16BIT_NV = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extensions::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_32BIT_NV")]
        const _32BIT_NV = 4;
    }
}

bitflags::bitflags! {
    /// [`VkPartitionedAccelerationStructureInstanceFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPartitionedAccelerationStructureInstanceFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_PartitionedAccelerationStructure`](Extensions::NV_PartitionedAccelerationStructure)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkPartitionedAccelerationStructureInstanceFlagBitsNV")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct PartitionedAccelerationStructureInstanceFlagsNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_PartitionedAccelerationStructure`](Extensions::NV_PartitionedAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_TRIANGLE_FACING_CULL_DISABLE_BIT_NV")]
        const FLAG_TRIANGLE_FACING_CULL_DISABLE_NV = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_PartitionedAccelerationStructure`](Extensions::NV_PartitionedAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_TRIANGLE_FLIP_FACING_BIT_NV")]
        const FLAG_TRIANGLE_FLIP_FACING_NV = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_PartitionedAccelerationStructure`](Extensions::NV_PartitionedAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_FORCE_OPAQUE_BIT_NV")]
        const FLAG_FORCE_OPAQUE_NV = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_PartitionedAccelerationStructure`](Extensions::NV_PartitionedAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_FORCE_NO_OPAQUE_BIT_NV")]
        const FLAG_FORCE_NO_OPAQUE_NV = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_PartitionedAccelerationStructure`](Extensions::NV_PartitionedAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV")]
        const FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV = 16;
    }
}

bitflags::bitflags! {
    /// [`VkIndirectCommandsInputModeFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsInputModeFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_DeviceGeneratedCommands`](Extensions::EXT_DeviceGeneratedCommands)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkIndirectCommandsInputModeFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct IndirectCommandsInputModeFlagsEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DeviceGeneratedCommands`](Extensions::EXT_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_INDIRECT_COMMANDS_INPUT_MODE_VULKAN_INDEX_BUFFER_EXT")]
        const VULKAN_INDEX_BUFFER_EXT = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DeviceGeneratedCommands`](Extensions::EXT_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_INDIRECT_COMMANDS_INPUT_MODE_DXGI_INDEX_BUFFER_EXT")]
        const DXGI_INDEX_BUFFER_EXT = 2;
    }
}

bitflags::bitflags! {
    /// [`VkIndirectCommandsLayoutUsageFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutUsageFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_DeviceGeneratedCommands`](Extensions::EXT_DeviceGeneratedCommands)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkIndirectCommandsLayoutUsageFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct IndirectCommandsLayoutUsageFlagsEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DeviceGeneratedCommands`](Extensions::EXT_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_EXT")]
        const EXPLICIT_PREPROCESS_EXT = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DeviceGeneratedCommands`](Extensions::EXT_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_EXT")]
        const UNORDERED_SEQUENCES_EXT = 2;
    }
}

bitflags::bitflags! {
    /// [`VkDataGraphOpticalFlowGridSizeFlagBitsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowGridSizeFlagBitsARM.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDataGraphOpticalFlowGridSizeFlagBitsARM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DataGraphOpticalFlowGridSizeFlagsARM: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_UNKNOWN_ARM")]
        const UNKNOWN_ARM = 0;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_1X1_BIT_ARM")]
        const _1X1_ARM = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_2X2_BIT_ARM")]
        const _2X2_ARM = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_4X4_BIT_ARM")]
        const _4X4_ARM = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_8X8_BIT_ARM")]
        const _8X8_ARM = 8;
    }
}

bitflags::bitflags! {
    /// [`VkDataGraphOpticalFlowCreateFlagBitsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowCreateFlagBitsARM.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDataGraphOpticalFlowCreateFlagBitsARM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DataGraphOpticalFlowCreateFlagsARM: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_CREATE_ENABLE_HINT_BIT_ARM")]
        const ENABLE_HINT_ARM = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_CREATE_ENABLE_COST_BIT_ARM")]
        const ENABLE_COST_ARM = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_CREATE_RESERVED_30_BIT_ARM")]
        const RESERVED_30_ARM = 1073741824;
    }
}

bitflags::bitflags! {
    /// [`VkDataGraphOpticalFlowImageUsageFlagBitsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowImageUsageFlagBitsARM.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDataGraphOpticalFlowImageUsageFlagBitsARM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DataGraphOpticalFlowImageUsageFlagsARM: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_UNKNOWN_ARM")]
        const UNKNOWN_ARM = 0;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_INPUT_BIT_ARM")]
        const INPUT_ARM = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_OUTPUT_BIT_ARM")]
        const OUTPUT_ARM = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_HINT_BIT_ARM")]
        const HINT_ARM = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_COST_BIT_ARM")]
        const COST_ARM = 8;
    }
}

bitflags::bitflags! {
    /// [`VkDataGraphOpticalFlowExecuteFlagBitsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowExecuteFlagBitsARM.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDataGraphOpticalFlowExecuteFlagBitsARM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct DataGraphOpticalFlowExecuteFlagsARM: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_DISABLE_TEMPORAL_HINTS_BIT_ARM")]
        const DISABLE_TEMPORAL_HINTS_ARM = 1;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_INPUT_UNCHANGED_BIT_ARM")]
        const INPUT_UNCHANGED_ARM = 2;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_REFERENCE_UNCHANGED_BIT_ARM")]
        const REFERENCE_UNCHANGED_ARM = 4;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_INPUT_IS_PREVIOUS_REFERENCE_BIT_ARM")]
        const INPUT_IS_PREVIOUS_REFERENCE_ARM = 8;
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extensions::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_REFERENCE_IS_PREVIOUS_INPUT_BIT_ARM")]
        const REFERENCE_IS_PREVIOUS_INPUT_ARM = 16;
    }
}

bitflags::bitflags! {
    /// [`VkCooperativeMatrixFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCooperativeMatrixFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_CooperativeMatrixMaintenance1`](Extensions::EXT_CooperativeMatrixMaintenance1)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkCooperativeMatrixFlagBitsEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct CooperativeMatrixFlagsEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_CooperativeMatrixMaintenance1`](Extensions::EXT_CooperativeMatrixMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_COOPERATIVE_MATRIX_SATURATING_ACCUMULATION_BIT_EXT")]
        const SATURATING_ACCUMULATION_EXT = 1;
    }
}
