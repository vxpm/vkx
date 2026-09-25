// WARNING: AUTO GENERATED MODULE
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(clippy::all)]

use crate::loader::*;
use crate::platform::*;
use std::ffi::{c_char, c_int, c_uint, c_void};

use crate::FlagSet;
use crate::enums::*;

crate::__vkx_internal_flags! {
    /// [`VkFormatFeatureFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlagBits.html)
    ///
    #[doc(alias = "VkFormatFeatureFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum FormatFeatureFlag: u32 {
        #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT")]
        #[default]
        SAMPLED_IMAGE = 1,
        #[doc(alias = "VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT")]
        STORAGE_IMAGE = 2,
        #[doc(alias = "VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT")]
        STORAGE_IMAGE_ATOMIC = 4,
        #[doc(alias = "VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT")]
        UNIFORM_TEXEL_BUFFER = 8,
        #[doc(alias = "VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT")]
        STORAGE_TEXEL_BUFFER = 16,
        #[doc(alias = "VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT")]
        STORAGE_TEXEL_BUFFER_ATOMIC = 32,
        #[doc(alias = "VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT")]
        VERTEX_BUFFER = 64,
        #[doc(alias = "VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT")]
        COLOR_ATTACHMENT = 128,
        #[doc(alias = "VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT")]
        COLOR_ATTACHMENT_BLEND = 256,
        #[doc(alias = "VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT")]
        DEPTH_STENCIL_ATTACHMENT = 512,
        #[doc(alias = "VK_FORMAT_FEATURE_BLIT_SRC_BIT")]
        BLIT_SRC = 1024,
        #[doc(alias = "VK_FORMAT_FEATURE_BLIT_DST_BIT")]
        BLIT_DST = 2048,
        #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT")]
        SAMPLED_IMAGE_FILTER_LINEAR = 4096,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_Maintenance1`](Extension::KHR_Maintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_TRANSFER_SRC_BIT")]
        TRANSFER_SRC = 16384,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_Maintenance1`](Extension::KHR_Maintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_TRANSFER_DST_BIT")]
        TRANSFER_DST = 32768,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_SamplerYcbcrConversion`](Extension::KHR_SamplerYcbcrConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT")]
        MIDPOINT_CHROMA_SAMPLES = 131072,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_SamplerYcbcrConversion`](Extension::KHR_SamplerYcbcrConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT")]
        SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER = 262144,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_SamplerYcbcrConversion`](Extension::KHR_SamplerYcbcrConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT")]
        SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER = 524288,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_SamplerYcbcrConversion`](Extension::KHR_SamplerYcbcrConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT")]
        SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT = 1048576,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_SamplerYcbcrConversion`](Extension::KHR_SamplerYcbcrConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT")]
        SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE = 2097152,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_SamplerYcbcrConversion`](Extension::KHR_SamplerYcbcrConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_DISJOINT_BIT")]
        DISJOINT = 4194304,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_SamplerYcbcrConversion`](Extension::KHR_SamplerYcbcrConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT")]
        COSITED_CHROMA_SAMPLES = 8388608,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`EXT_SamplerFilterMinmax`](Extension::EXT_SamplerFilterMinmax)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT")]
        SAMPLED_IMAGE_FILTER_MINMAX = 65536,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extension::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_VIDEO_DECODE_OUTPUT_BIT_KHR")]
        VIDEO_DECODE_OUTPUT_KHR = 33554432,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extension::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_VIDEO_DECODE_DPB_BIT_KHR")]
        VIDEO_DECODE_DPB_KHR = 67108864,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR")]
        ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR = 536870912,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`IMG_FilterCubic`](Extension::IMG_FilterCubic)
        /// - Extension [`EXT_FilterCubic`](Extension::EXT_FilterCubic)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT")]
        SAMPLED_IMAGE_FILTER_CUBIC_EXT = 8192,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_FragmentDensityMap`](Extension::EXT_FragmentDensityMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_FRAGMENT_DENSITY_MAP_BIT_EXT")]
        FRAGMENT_DENSITY_MAP_EXT = 16777216,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_FragmentShadingRate`](Extension::KHR_FragmentShadingRate)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
        FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = 1073741824,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_VIDEO_ENCODE_INPUT_BIT_KHR")]
        VIDEO_ENCODE_INPUT_KHR = 134217728,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_VIDEO_ENCODE_DPB_BIT_KHR")]
        VIDEO_ENCODE_DPB_KHR = 268435456,
    }
}
impl FormatFeatureFlag {
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
/// [`VkFormatFeatureFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlags.html)
///
#[doc(alias = "VkFormatFeatureFlags")]
pub type FormatFeatureFlags = FlagSet<FormatFeatureFlag>;

crate::__vkx_internal_flags! {
    /// [`VkImageCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCreateFlagBits.html)
    ///
    #[doc(alias = "VkImageCreateFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ImageCreateFlag: u32 {
        #[doc(alias = "VK_IMAGE_CREATE_SPARSE_BINDING_BIT")]
        #[default]
        SPARSE_BINDING = 1,
        #[doc(alias = "VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT")]
        SPARSE_RESIDENCY = 2,
        #[doc(alias = "VK_IMAGE_CREATE_SPARSE_ALIASED_BIT")]
        SPARSE_ALIASED = 4,
        #[doc(alias = "VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT")]
        MUTABLE_FORMAT = 8,
        #[doc(alias = "VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT")]
        CUBE_COMPATIBLE = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_BindMemory2`](Extension::KHR_BindMemory2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_ALIAS_BIT")]
        ALIAS = 1024,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_DeviceGroup`](Extension::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT")]
        SPLIT_INSTANCE_BIND_REGIONS = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_Maintenance1`](Extension::KHR_Maintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT")]
        _2D_ARRAY_COMPATIBLE = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_Maintenance2`](Extension::KHR_Maintenance2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT")]
        BLOCK_TEXEL_VIEW_COMPATIBLE = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_Maintenance2`](Extension::KHR_Maintenance2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_EXTENDED_USAGE_BIT")]
        EXTENDED_USAGE = 256,
        #[doc(alias = "VK_IMAGE_CREATE_PROTECTED_BIT")]
        PROTECTED = 2048,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_SamplerYcbcrConversion`](Extension::KHR_SamplerYcbcrConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_DISJOINT_BIT")]
        DISJOINT = 512,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_CornerSampledImage`](Extension::NV_CornerSampledImage)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV")]
        CORNER_SAMPLED_NV = 8192,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        /// - Extension [`EXT_DescriptorBuffer`](Extension::EXT_DescriptorBuffer)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_DESCRIPTOR_HEAP_CAPTURE_REPLAY_BIT_EXT")]
        DESCRIPTOR_HEAP_CAPTURE_REPLAY_EXT = 65536,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_SampleLocations`](Extension::EXT_SampleLocations)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT")]
        SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT = 4096,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_FragmentDensityMap`](Extension::EXT_FragmentDensityMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT")]
        SUBSAMPLED_EXT = 16384,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MultisampledRenderToSingleSampled`](Extension::EXT_MultisampledRenderToSingleSampled)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT")]
        MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT = 262144,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_Image2DViewOf3D`](Extension::EXT_Image2DViewOf3D)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2D_VIEW_COMPATIBLE_BIT_EXT")]
        _2D_VIEW_COMPATIBLE_EXT = 131072,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoMaintenance1`](Extension::KHR_VideoMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_VIDEO_PROFILE_INDEPENDENT_BIT_KHR")]
        VIDEO_PROFILE_INDEPENDENT_KHR = 1048576,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_FragmentDensityMapOffset`](Extension::QCOM_FragmentDensityMapOffset)
        /// - Extension [`EXT_FragmentDensityMapOffset`](Extension::EXT_FragmentDensityMapOffset)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_EXT")]
        FRAGMENT_DENSITY_MAP_OFFSET_EXT = 32768,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance11`](Extension::KHR_Maintenance11)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_ALIAS_SINGLE_LAYER_DESCRIPTOR_BIT_KHR")]
        ALIAS_SINGLE_LAYER_DESCRIPTOR_KHR = 4194304,
    }
}
impl ImageCreateFlag {
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
/// [`VkImageCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCreateFlags.html)
///
#[doc(alias = "VkImageCreateFlags")]
pub type ImageCreateFlags = FlagSet<ImageCreateFlag>;

crate::__vkx_internal_flags! {
    /// [`VkSampleCountFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSampleCountFlagBits.html)
    ///
    #[doc(alias = "VkSampleCountFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum SampleCountFlag: u32 {
        #[doc(alias = "VK_SAMPLE_COUNT_1_BIT")]
        #[default]
        _1 = 1,
        #[doc(alias = "VK_SAMPLE_COUNT_2_BIT")]
        _2 = 2,
        #[doc(alias = "VK_SAMPLE_COUNT_4_BIT")]
        _4 = 4,
        #[doc(alias = "VK_SAMPLE_COUNT_8_BIT")]
        _8 = 8,
        #[doc(alias = "VK_SAMPLE_COUNT_16_BIT")]
        _16 = 16,
        #[doc(alias = "VK_SAMPLE_COUNT_32_BIT")]
        _32 = 32,
        #[doc(alias = "VK_SAMPLE_COUNT_64_BIT")]
        _64 = 64,
    }
}
/// [`VkSampleCountFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSampleCountFlags.html)
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkSampleCountFlags")]
pub type SampleCountFlags = FlagSet<SampleCountFlag>;

crate::__vkx_internal_flags! {
    /// [`VkImageUsageFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageUsageFlagBits.html)
    ///
    #[doc(alias = "VkImageUsageFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ImageUsageFlag: u32 {
        #[doc(alias = "VK_IMAGE_USAGE_TRANSFER_SRC_BIT")]
        #[default]
        TRANSFER_SRC = 1,
        #[doc(alias = "VK_IMAGE_USAGE_TRANSFER_DST_BIT")]
        TRANSFER_DST = 2,
        #[doc(alias = "VK_IMAGE_USAGE_SAMPLED_BIT")]
        SAMPLED = 4,
        #[doc(alias = "VK_IMAGE_USAGE_STORAGE_BIT")]
        STORAGE = 8,
        #[doc(alias = "VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT")]
        COLOR_ATTACHMENT = 16,
        #[doc(alias = "VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT")]
        DEPTH_STENCIL_ATTACHMENT = 32,
        #[doc(alias = "VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT")]
        TRANSIENT_ATTACHMENT = 64,
        #[doc(alias = "VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT")]
        INPUT_ATTACHMENT = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`EXT_HostImageCopy`](Extension::EXT_HostImageCopy)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_HOST_TRANSFER_BIT")]
        HOST_TRANSFER = 4194304,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extension::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR")]
        VIDEO_DECODE_DST_KHR = 1024,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extension::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_VIDEO_DECODE_SRC_BIT_KHR")]
        VIDEO_DECODE_SRC_KHR = 2048,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extension::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR")]
        VIDEO_DECODE_DPB_KHR = 4096,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_FragmentDensityMap`](Extension::EXT_FragmentDensityMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_FRAGMENT_DENSITY_MAP_BIT_EXT")]
        FRAGMENT_DENSITY_MAP_EXT = 512,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_FragmentShadingRate`](Extension::KHR_FragmentShadingRate)
        /// - Extension [`NV_ShadingRateImage`](Extension::NV_ShadingRateImage)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
        FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = 256,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_VIDEO_ENCODE_DST_BIT_KHR")]
        VIDEO_ENCODE_DST_KHR = 8192,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_VIDEO_ENCODE_SRC_BIT_KHR")]
        VIDEO_ENCODE_SRC_KHR = 16384,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_VIDEO_ENCODE_DPB_BIT_KHR")]
        VIDEO_ENCODE_DPB_KHR = 32768,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_AttachmentFeedbackLoopLayout`](Extension::EXT_AttachmentFeedbackLoopLayout)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT")]
        ATTACHMENT_FEEDBACK_LOOP_EXT = 524288,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`HUAWEI_InvocationMask`](Extension::HUAWEI_InvocationMask)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_INVOCATION_MASK_BIT_HUAWEI")]
        INVOCATION_MASK_HUAWEI = 262144,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_ImageProcessing`](Extension::QCOM_ImageProcessing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_SAMPLE_WEIGHT_BIT_QCOM")]
        SAMPLE_WEIGHT_QCOM = 1048576,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_ImageProcessing`](Extension::QCOM_ImageProcessing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_SAMPLE_BLOCK_MATCH_BIT_QCOM")]
        SAMPLE_BLOCK_MATCH_QCOM = 2097152,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_Tensors`](Extension::ARM_Tensors)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_TENSOR_ALIASING_BIT_ARM")]
        TENSOR_ALIASING_ARM = 8388608,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_TileMemoryHeap`](Extension::QCOM_TileMemoryHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_TILE_MEMORY_BIT_QCOM")]
        TILE_MEMORY_QCOM = 134217728,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extension::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR")]
        VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR = 33554432,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extension::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR")]
        VIDEO_ENCODE_EMPHASIS_MAP_KHR = 67108864,
    }
}
impl ImageUsageFlag {
    /// [`VK_IMAGE_USAGE_HOST_TRANSFER_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMAGE_USAGE_HOST_TRANSFER_BIT_EXT.html)
    ///
    #[doc(alias = "VK_IMAGE_USAGE_HOST_TRANSFER_BIT_EXT")]
    pub const HOST_TRANSFER_EXT: Self = Self::HOST_TRANSFER;
    /// [`VK_IMAGE_USAGE_SHADING_RATE_IMAGE_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMAGE_USAGE_SHADING_RATE_IMAGE_BIT_NV.html)
    ///
    #[doc(alias = "VK_IMAGE_USAGE_SHADING_RATE_IMAGE_BIT_NV")]
    pub const SHADING_RATE_IMAGE_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
}
/// [`VkImageUsageFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageUsageFlags.html)
///
#[doc(alias = "VkImageUsageFlags")]
pub type ImageUsageFlags = FlagSet<ImageUsageFlag>;

crate::__vkx_internal_flags! {
    /// [`VkInstanceCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkInstanceCreateFlagBits.html)
    ///
    #[doc(alias = "VkInstanceCreateFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum InstanceCreateFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_PortabilityEnumeration`](Extension::KHR_PortabilityEnumeration)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR")]
        #[default]
        ENUMERATE_PORTABILITY_KHR = 1,
    }
}
/// [`VkInstanceCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkInstanceCreateFlags.html)
///
#[doc(alias = "VkInstanceCreateFlags")]
pub type InstanceCreateFlags = FlagSet<InstanceCreateFlag>;

crate::__vkx_internal_flags! {
    /// [`VkMemoryHeapFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryHeapFlagBits.html)
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkMemoryHeapFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum MemoryHeapFlag: u32 {
        #[doc(alias = "VK_MEMORY_HEAP_DEVICE_LOCAL_BIT")]
        #[default]
        DEVICE_LOCAL = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_DeviceGroupCreation`](Extension::KHR_DeviceGroupCreation)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MEMORY_HEAP_MULTI_INSTANCE_BIT")]
        MULTI_INSTANCE = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_TileMemoryHeap`](Extension::QCOM_TileMemoryHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MEMORY_HEAP_TILE_MEMORY_BIT_QCOM")]
        TILE_MEMORY_QCOM = 8,
    }
}
impl MemoryHeapFlag {
    /// [`VK_MEMORY_HEAP_MULTI_INSTANCE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MEMORY_HEAP_MULTI_INSTANCE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_MEMORY_HEAP_MULTI_INSTANCE_BIT_KHR")]
    pub const MULTI_INSTANCE_KHR: Self = Self::MULTI_INSTANCE;
}
/// [`VkMemoryHeapFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryHeapFlags.html)
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkMemoryHeapFlags")]
pub type MemoryHeapFlags = FlagSet<MemoryHeapFlag>;

crate::__vkx_internal_flags! {
    /// [`VkMemoryPropertyFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryPropertyFlagBits.html)
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkMemoryPropertyFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum MemoryPropertyFlag: u32 {
        #[doc(alias = "VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT")]
        #[default]
        DEVICE_LOCAL = 1,
        #[doc(alias = "VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT")]
        HOST_VISIBLE = 2,
        #[doc(alias = "VK_MEMORY_PROPERTY_HOST_COHERENT_BIT")]
        HOST_COHERENT = 4,
        #[doc(alias = "VK_MEMORY_PROPERTY_HOST_CACHED_BIT")]
        HOST_CACHED = 8,
        #[doc(alias = "VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT")]
        LAZILY_ALLOCATED = 16,
        #[doc(alias = "VK_MEMORY_PROPERTY_PROTECTED_BIT")]
        PROTECTED = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMD_DeviceCoherentMemory`](Extension::AMD_DeviceCoherentMemory)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD")]
        DEVICE_COHERENT_AMD = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMD_DeviceCoherentMemory`](Extension::AMD_DeviceCoherentMemory)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MEMORY_PROPERTY_DEVICE_UNCACHED_BIT_AMD")]
        DEVICE_UNCACHED_AMD = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ExternalMemoryRdma`](Extension::NV_ExternalMemoryRdma)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MEMORY_PROPERTY_RDMA_CAPABLE_BIT_NV")]
        RDMA_CAPABLE_NV = 256,
    }
}
/// [`VkMemoryPropertyFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryPropertyFlags.html)
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkMemoryPropertyFlags")]
pub type MemoryPropertyFlags = FlagSet<MemoryPropertyFlag>;

crate::__vkx_internal_flags! {
    /// [`VkQueueFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFlagBits.html)
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkQueueFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum QueueFlag: u32 {
        #[doc(alias = "VK_QUEUE_GRAPHICS_BIT")]
        #[default]
        GRAPHICS = 1,
        #[doc(alias = "VK_QUEUE_COMPUTE_BIT")]
        COMPUTE = 2,
        #[doc(alias = "VK_QUEUE_TRANSFER_BIT")]
        TRANSFER = 4,
        #[doc(alias = "VK_QUEUE_SPARSE_BINDING_BIT")]
        SPARSE_BINDING = 8,
        #[doc(alias = "VK_QUEUE_PROTECTED_BIT")]
        PROTECTED = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extension::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_QUEUE_VIDEO_DECODE_BIT_KHR")]
        VIDEO_DECODE_KHR = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_QUEUE_VIDEO_ENCODE_BIT_KHR")]
        VIDEO_ENCODE_KHR = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_QUEUE_OPTICAL_FLOW_BIT_NV")]
        OPTICAL_FLOW_NV = 256,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraph`](Extension::ARM_DataGraph)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_QUEUE_DATA_GRAPH_BIT_ARM")]
        DATA_GRAPH_ARM = 1024,
    }
}
/// [`VkQueueFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFlags.html)
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkQueueFlags")]
pub type QueueFlags = FlagSet<QueueFlag>;

crate::__vkx_internal_flags! {
    /// [`VkShaderStageFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderStageFlagBits.html)
    ///
    #[doc(alias = "VkShaderStageFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ShaderStageFlag: u32 {
        #[doc(alias = "VK_SHADER_STAGE_VERTEX_BIT")]
        #[default]
        VERTEX = 1,
        #[doc(alias = "VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT")]
        TESSELLATION_CONTROL = 2,
        #[doc(alias = "VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT")]
        TESSELLATION_EVALUATION = 4,
        #[doc(alias = "VK_SHADER_STAGE_GEOMETRY_BIT")]
        GEOMETRY = 8,
        #[doc(alias = "VK_SHADER_STAGE_FRAGMENT_BIT")]
        FRAGMENT = 16,
        #[doc(alias = "VK_SHADER_STAGE_COMPUTE_BIT")]
        COMPUTE = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_STAGE_RAYGEN_BIT_KHR")]
        RAYGEN_KHR = 256,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_STAGE_ANY_HIT_BIT_KHR")]
        ANY_HIT_KHR = 512,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR")]
        CLOSEST_HIT_KHR = 1024,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_STAGE_MISS_BIT_KHR")]
        MISS_KHR = 2048,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_STAGE_INTERSECTION_BIT_KHR")]
        INTERSECTION_KHR = 4096,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_STAGE_CALLABLE_BIT_KHR")]
        CALLABLE_KHR = 8192,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_MeshShader`](Extension::NV_MeshShader)
        /// - Extension [`EXT_MeshShader`](Extension::EXT_MeshShader)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_STAGE_TASK_BIT_EXT")]
        TASK_EXT = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_MeshShader`](Extension::NV_MeshShader)
        /// - Extension [`EXT_MeshShader`](Extension::EXT_MeshShader)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_STAGE_MESH_BIT_EXT")]
        MESH_EXT = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`HUAWEI_SubpassShading`](Extension::HUAWEI_SubpassShading)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_STAGE_SUBPASS_SHADING_BIT_HUAWEI")]
        SUBPASS_SHADING_HUAWEI = 16384,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`HUAWEI_ClusterCullingShader`](Extension::HUAWEI_ClusterCullingShader)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_STAGE_CLUSTER_CULLING_BIT_HUAWEI")]
        CLUSTER_CULLING_HUAWEI = 524288,
    }
}
impl ShaderStageFlag {
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
    #[doc(alias = "VK_SHADER_STAGE_ALL_GRAPHICS")]
    pub const ALL_GRAPHICS: FlagSet<Self> = FlagSet(31);
    #[doc(alias = "VK_SHADER_STAGE_ALL")]
    pub const ALL: FlagSet<Self> = FlagSet(2147483647);
}
/// [`VkShaderStageFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderStageFlags.html)
///
#[doc(alias = "VkShaderStageFlags")]
pub type ShaderStageFlags = FlagSet<ShaderStageFlag>;

crate::__vkx_internal_flags! {
    /// [`VkDeviceQueueCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceQueueCreateFlagBits.html)
    ///
    #[doc(alias = "VkDeviceQueueCreateFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum DeviceQueueCreateFlag: u32 {
        #[doc(alias = "VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT")]
        #[default]
        PROTECTED = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_InternallySynchronizedQueues`](Extension::KHR_InternallySynchronizedQueues)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_QUEUE_CREATE_INTERNALLY_SYNCHRONIZED_BIT_KHR")]
        INTERNALLY_SYNCHRONIZED_KHR = 4,
    }
}
/// [`VkDeviceQueueCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceQueueCreateFlags.html)
///
#[doc(alias = "VkDeviceQueueCreateFlags")]
pub type DeviceQueueCreateFlags = FlagSet<DeviceQueueCreateFlag>;

crate::__vkx_internal_flags! {
    /// [`VkPipelineStageFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineStageFlagBits.html)
    ///
    #[doc(alias = "VkPipelineStageFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum PipelineStageFlag: u32 {
        #[doc(alias = "VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT")]
        #[default]
        TOP_OF_PIPE = 1,
        #[doc(alias = "VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT")]
        DRAW_INDIRECT = 2,
        #[doc(alias = "VK_PIPELINE_STAGE_VERTEX_INPUT_BIT")]
        VERTEX_INPUT = 4,
        #[doc(alias = "VK_PIPELINE_STAGE_VERTEX_SHADER_BIT")]
        VERTEX_SHADER = 8,
        #[doc(alias = "VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT")]
        TESSELLATION_CONTROL_SHADER = 16,
        #[doc(alias = "VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT")]
        TESSELLATION_EVALUATION_SHADER = 32,
        #[doc(alias = "VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT")]
        GEOMETRY_SHADER = 64,
        #[doc(alias = "VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT")]
        FRAGMENT_SHADER = 128,
        #[doc(alias = "VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT")]
        EARLY_FRAGMENT_TESTS = 256,
        #[doc(alias = "VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT")]
        LATE_FRAGMENT_TESTS = 512,
        #[doc(alias = "VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT")]
        COLOR_ATTACHMENT_OUTPUT = 1024,
        #[doc(alias = "VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT")]
        COMPUTE_SHADER = 2048,
        #[doc(alias = "VK_PIPELINE_STAGE_TRANSFER_BIT")]
        TRANSFER = 4096,
        #[doc(alias = "VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT")]
        BOTTOM_OF_PIPE = 8192,
        #[doc(alias = "VK_PIPELINE_STAGE_HOST_BIT")]
        HOST = 16384,
        #[doc(alias = "VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT")]
        ALL_GRAPHICS = 32768,
        #[doc(alias = "VK_PIPELINE_STAGE_ALL_COMMANDS_BIT")]
        ALL_COMMANDS = 65536,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_NONE")]
        NONE = 0,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_TransformFeedback`](Extension::EXT_TransformFeedback)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT")]
        TRANSFORM_FEEDBACK_EXT = 16777216,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ConditionalRendering`](Extension::EXT_ConditionalRendering)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_CONDITIONAL_RENDERING_BIT_EXT")]
        CONDITIONAL_RENDERING_EXT = 262144,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR")]
        ACCELERATION_STRUCTURE_BUILD_KHR = 33554432,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_KHR")]
        RAY_TRACING_SHADER_KHR = 2097152,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_FragmentDensityMap`](Extension::EXT_FragmentDensityMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT")]
        FRAGMENT_DENSITY_PROCESS_EXT = 8388608,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_FragmentShadingRate`](Extension::KHR_FragmentShadingRate)
        /// - Extension [`NV_ShadingRateImage`](Extension::NV_ShadingRateImage)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
        FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = 4194304,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_MeshShader`](Extension::NV_MeshShader)
        /// - Extension [`EXT_MeshShader`](Extension::EXT_MeshShader)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_TASK_SHADER_BIT_EXT")]
        TASK_SHADER_EXT = 524288,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_MeshShader`](Extension::NV_MeshShader)
        /// - Extension [`EXT_MeshShader`](Extension::EXT_MeshShader)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_MESH_SHADER_BIT_EXT")]
        MESH_SHADER_EXT = 1048576,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceGeneratedCommands`](Extension::NV_DeviceGeneratedCommands)
        /// - Extension [`EXT_DeviceGeneratedCommands`](Extension::EXT_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_COMMAND_PREPROCESS_BIT_EXT")]
        COMMAND_PREPROCESS_EXT = 131072,
    }
}
impl PipelineStageFlag {
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
/// [`VkPipelineStageFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineStageFlags.html)
///
#[doc(alias = "VkPipelineStageFlags")]
pub type PipelineStageFlags = FlagSet<PipelineStageFlag>;

crate::__vkx_internal_flags! {
    /// [`VkMemoryMapFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryMapFlagBits.html)
    ///
    #[doc(alias = "VkMemoryMapFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum MemoryMapFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MapMemoryPlaced`](Extension::EXT_MapMemoryPlaced)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MEMORY_MAP_PLACED_BIT_EXT")]
        #[default]
        PLACED_EXT = 1,
    }
}
/// [`VkMemoryMapFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryMapFlags.html)
///
#[doc(alias = "VkMemoryMapFlags")]
pub type MemoryMapFlags = FlagSet<MemoryMapFlag>;

crate::__vkx_internal_flags! {
    /// [`VkImageAspectFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageAspectFlagBits.html)
    ///
    #[doc(alias = "VkImageAspectFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ImageAspectFlag: u32 {
        #[doc(alias = "VK_IMAGE_ASPECT_COLOR_BIT")]
        #[default]
        COLOR = 1,
        #[doc(alias = "VK_IMAGE_ASPECT_DEPTH_BIT")]
        DEPTH = 2,
        #[doc(alias = "VK_IMAGE_ASPECT_STENCIL_BIT")]
        STENCIL = 4,
        #[doc(alias = "VK_IMAGE_ASPECT_METADATA_BIT")]
        METADATA = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_SamplerYcbcrConversion`](Extension::KHR_SamplerYcbcrConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_ASPECT_PLANE_0_BIT")]
        PLANE_0 = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_SamplerYcbcrConversion`](Extension::KHR_SamplerYcbcrConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_ASPECT_PLANE_1_BIT")]
        PLANE_1 = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_SamplerYcbcrConversion`](Extension::KHR_SamplerYcbcrConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_ASPECT_PLANE_2_BIT")]
        PLANE_2 = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Maintenance4`](Extension::KHR_Maintenance4)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_ASPECT_NONE")]
        NONE = 0,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageDrmFormatModifier`](Extension::EXT_ImageDrmFormatModifier)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_ASPECT_MEMORY_PLANE_0_BIT_EXT")]
        MEMORY_PLANE_0_EXT = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageDrmFormatModifier`](Extension::EXT_ImageDrmFormatModifier)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_ASPECT_MEMORY_PLANE_1_BIT_EXT")]
        MEMORY_PLANE_1_EXT = 256,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageDrmFormatModifier`](Extension::EXT_ImageDrmFormatModifier)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_ASPECT_MEMORY_PLANE_2_BIT_EXT")]
        MEMORY_PLANE_2_EXT = 512,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageDrmFormatModifier`](Extension::EXT_ImageDrmFormatModifier)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_ASPECT_MEMORY_PLANE_3_BIT_EXT")]
        MEMORY_PLANE_3_EXT = 1024,
    }
}
impl ImageAspectFlag {
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
/// [`VkImageAspectFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageAspectFlags.html)
///
#[doc(alias = "VkImageAspectFlags")]
pub type ImageAspectFlags = FlagSet<ImageAspectFlag>;

crate::__vkx_internal_flags! {
    /// [`VkSparseImageFormatFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageFormatFlagBits.html)
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkSparseImageFormatFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum SparseImageFormatFlag: u32 {
        #[doc(alias = "VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT")]
        #[default]
        SINGLE_MIPTAIL = 1,
        #[doc(alias = "VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT")]
        ALIGNED_MIP_SIZE = 2,
        #[doc(alias = "VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT")]
        NONSTANDARD_BLOCK_SIZE = 4,
    }
}
/// [`VkSparseImageFormatFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageFormatFlags.html)
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkSparseImageFormatFlags")]
pub type SparseImageFormatFlags = FlagSet<SparseImageFormatFlag>;

crate::__vkx_internal_flags! {
    /// [`VkSparseMemoryBindFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseMemoryBindFlagBits.html)
    ///
    #[doc(alias = "VkSparseMemoryBindFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum SparseMemoryBindFlag: u32 {
        #[doc(alias = "VK_SPARSE_MEMORY_BIND_METADATA_BIT")]
        #[default]
        METADATA = 1,
    }
}
/// [`VkSparseMemoryBindFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseMemoryBindFlags.html)
///
#[doc(alias = "VkSparseMemoryBindFlags")]
pub type SparseMemoryBindFlags = FlagSet<SparseMemoryBindFlag>;

crate::__vkx_internal_flags! {
    /// [`VkFenceCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceCreateFlagBits.html)
    ///
    #[doc(alias = "VkFenceCreateFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum FenceCreateFlag: u32 {
        #[doc(alias = "VK_FENCE_CREATE_SIGNALED_BIT")]
        #[default]
        SIGNALED = 1,
    }
}
/// [`VkFenceCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceCreateFlags.html)
///
#[doc(alias = "VkFenceCreateFlags")]
pub type FenceCreateFlags = FlagSet<FenceCreateFlag>;

crate::__vkx_internal_flags! {
    /// [`VkQueryPoolCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPoolCreateFlagBits.html)
    ///
    #[doc(alias = "VkQueryPoolCreateFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum QueryPoolCreateFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance9`](Extension::KHR_Maintenance9)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_QUERY_POOL_CREATE_RESET_BIT_KHR")]
        #[default]
        RESET_KHR = 1,
    }
}
/// [`VkQueryPoolCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPoolCreateFlags.html)
///
#[doc(alias = "VkQueryPoolCreateFlags")]
pub type QueryPoolCreateFlags = FlagSet<QueryPoolCreateFlag>;

crate::__vkx_internal_flags! {
    /// [`VkQueryPipelineStatisticFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPipelineStatisticFlagBits.html)
    ///
    #[doc(alias = "VkQueryPipelineStatisticFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum QueryPipelineStatisticFlag: u32 {
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT")]
        #[default]
        INPUT_ASSEMBLY_VERTICES = 1,
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT")]
        INPUT_ASSEMBLY_PRIMITIVES = 2,
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT")]
        VERTEX_SHADER_INVOCATIONS = 4,
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT")]
        GEOMETRY_SHADER_INVOCATIONS = 8,
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT")]
        GEOMETRY_SHADER_PRIMITIVES = 16,
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT")]
        CLIPPING_INVOCATIONS = 32,
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT")]
        CLIPPING_PRIMITIVES = 64,
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT")]
        FRAGMENT_SHADER_INVOCATIONS = 128,
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT")]
        TESSELLATION_CONTROL_SHADER_PATCHES = 256,
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT")]
        TESSELLATION_EVALUATION_SHADER_INVOCATIONS = 512,
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT")]
        COMPUTE_SHADER_INVOCATIONS = 1024,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MeshShader`](Extension::EXT_MeshShader)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_TASK_SHADER_INVOCATIONS_BIT_EXT")]
        TASK_SHADER_INVOCATIONS_EXT = 2048,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MeshShader`](Extension::EXT_MeshShader)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_MESH_SHADER_INVOCATIONS_BIT_EXT")]
        MESH_SHADER_INVOCATIONS_EXT = 4096,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`HUAWEI_ClusterCullingShader`](Extension::HUAWEI_ClusterCullingShader)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_QUERY_PIPELINE_STATISTIC_CLUSTER_CULLING_SHADER_INVOCATIONS_BIT_HUAWEI")]
        CLUSTER_CULLING_SHADER_INVOCATIONS_HUAWEI = 8192,
    }
}
/// [`VkQueryPipelineStatisticFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPipelineStatisticFlags.html)
///
#[doc(alias = "VkQueryPipelineStatisticFlags")]
pub type QueryPipelineStatisticFlags = FlagSet<QueryPipelineStatisticFlag>;

crate::__vkx_internal_flags! {
    /// [`VkQueryResultFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryResultFlagBits.html)
    ///
    #[doc(alias = "VkQueryResultFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum QueryResultFlag: u32 {
        #[doc(alias = "VK_QUERY_RESULT_64_BIT")]
        #[default]
        _64 = 1,
        #[doc(alias = "VK_QUERY_RESULT_WAIT_BIT")]
        WAIT = 2,
        #[doc(alias = "VK_QUERY_RESULT_WITH_AVAILABILITY_BIT")]
        WITH_AVAILABILITY = 4,
        #[doc(alias = "VK_QUERY_RESULT_PARTIAL_BIT")]
        PARTIAL = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_QUERY_RESULT_WITH_STATUS_BIT_KHR")]
        WITH_STATUS_KHR = 16,
    }
}
/// [`VkQueryResultFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryResultFlags.html)
///
#[doc(alias = "VkQueryResultFlags")]
pub type QueryResultFlags = FlagSet<QueryResultFlag>;

crate::__vkx_internal_flags! {
    /// [`VkBufferCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCreateFlagBits.html)
    ///
    #[doc(alias = "VkBufferCreateFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum BufferCreateFlag: u32 {
        #[doc(alias = "VK_BUFFER_CREATE_SPARSE_BINDING_BIT")]
        #[default]
        SPARSE_BINDING = 1,
        #[doc(alias = "VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT")]
        SPARSE_RESIDENCY = 2,
        #[doc(alias = "VK_BUFFER_CREATE_SPARSE_ALIASED_BIT")]
        SPARSE_ALIASED = 4,
        #[doc(alias = "VK_BUFFER_CREATE_PROTECTED_BIT")]
        PROTECTED = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`KHR_BufferDeviceAddress`](Extension::KHR_BufferDeviceAddress)
        /// - Extension [`EXT_BufferDeviceAddress`](Extension::EXT_BufferDeviceAddress)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT")]
        DEVICE_ADDRESS_CAPTURE_REPLAY = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorBuffer`](Extension::EXT_DescriptorBuffer)
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT")]
        DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoMaintenance1`](Extension::KHR_VideoMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_CREATE_VIDEO_PROFILE_INDEPENDENT_BIT_KHR")]
        VIDEO_PROFILE_INDEPENDENT_KHR = 64,
    }
}
impl BufferCreateFlag {
    /// [`VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT.html)
    ///
    #[doc(alias = "VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT")]
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_EXT: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
    /// [`VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR.html)
    ///
    #[doc(alias = "VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR")]
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
}
/// [`VkBufferCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCreateFlags.html)
///
#[doc(alias = "VkBufferCreateFlags")]
pub type BufferCreateFlags = FlagSet<BufferCreateFlag>;

crate::__vkx_internal_flags! {
    /// [`VkBufferUsageFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlagBits.html)
    ///
    #[doc(alias = "VkBufferUsageFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum BufferUsageFlag: u32 {
        #[doc(alias = "VK_BUFFER_USAGE_TRANSFER_SRC_BIT")]
        #[default]
        TRANSFER_SRC = 1,
        #[doc(alias = "VK_BUFFER_USAGE_TRANSFER_DST_BIT")]
        TRANSFER_DST = 2,
        #[doc(alias = "VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT")]
        UNIFORM_TEXEL_BUFFER = 4,
        #[doc(alias = "VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT")]
        STORAGE_TEXEL_BUFFER = 8,
        #[doc(alias = "VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT")]
        UNIFORM_BUFFER = 16,
        #[doc(alias = "VK_BUFFER_USAGE_STORAGE_BUFFER_BIT")]
        STORAGE_BUFFER = 32,
        #[doc(alias = "VK_BUFFER_USAGE_INDEX_BUFFER_BIT")]
        INDEX_BUFFER = 64,
        #[doc(alias = "VK_BUFFER_USAGE_VERTEX_BUFFER_BIT")]
        VERTEX_BUFFER = 128,
        #[doc(alias = "VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT")]
        INDIRECT_BUFFER = 256,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`KHR_BufferDeviceAddress`](Extension::KHR_BufferDeviceAddress)
        /// - Extension [`EXT_BufferDeviceAddress`](Extension::EXT_BufferDeviceAddress)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT")]
        SHADER_DEVICE_ADDRESS = 131072,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extension::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_VIDEO_DECODE_SRC_BIT_KHR")]
        VIDEO_DECODE_SRC_KHR = 8192,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extension::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_VIDEO_DECODE_DST_BIT_KHR")]
        VIDEO_DECODE_DST_KHR = 16384,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_TransformFeedback`](Extension::EXT_TransformFeedback)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_BUFFER_BIT_EXT")]
        TRANSFORM_FEEDBACK_BUFFER_EXT = 2048,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_TransformFeedback`](Extension::EXT_TransformFeedback)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT")]
        TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT = 4096,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ConditionalRendering`](Extension::EXT_ConditionalRendering)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_CONDITIONAL_RENDERING_BIT_EXT")]
        CONDITIONAL_RENDERING_EXT = 512,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMDX_ShaderEnqueue`](Extension::AMDX_ShaderEnqueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_EXECUTION_GRAPH_SCRATCH_BIT_AMDX")]
        EXECUTION_GRAPH_SCRATCH_AMDX = 33554432,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_DESCRIPTOR_HEAP_BIT_EXT")]
        DESCRIPTOR_HEAP_EXT = 268435456,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR")]
        ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR = 524288,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_STORAGE_BIT_KHR")]
        ACCELERATION_STRUCTURE_STORAGE_KHR = 1048576,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR")]
        SHADER_BINDING_TABLE_KHR = 1024,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_VIDEO_ENCODE_DST_BIT_KHR")]
        VIDEO_ENCODE_DST_KHR = 32768,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_VIDEO_ENCODE_SRC_BIT_KHR")]
        VIDEO_ENCODE_SRC_KHR = 65536,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorBuffer`](Extension::EXT_DescriptorBuffer)
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_SAMPLER_DESCRIPTOR_BUFFER_BIT_EXT")]
        SAMPLER_DESCRIPTOR_BUFFER_EXT = 2097152,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorBuffer`](Extension::EXT_DescriptorBuffer)
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_RESOURCE_DESCRIPTOR_BUFFER_BIT_EXT")]
        RESOURCE_DESCRIPTOR_BUFFER_EXT = 4194304,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorBuffer`](Extension::EXT_DescriptorBuffer)
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_BIT_EXT")]
        PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT = 67108864,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_OpacityMicromap`](Extension::EXT_OpacityMicromap)
        /// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_MICROMAP_BUILD_INPUT_READ_ONLY_BIT_EXT")]
        MICROMAP_BUILD_INPUT_READ_ONLY_EXT = 8388608,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_OpacityMicromap`](Extension::EXT_OpacityMicromap)
        /// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_MICROMAP_STORAGE_BIT_EXT")]
        MICROMAP_STORAGE_EXT = 16777216,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_TileMemoryHeap`](Extension::QCOM_TileMemoryHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_TILE_MEMORY_BIT_QCOM")]
        TILE_MEMORY_QCOM = 134217728,
    }
}
impl BufferUsageFlag {
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
/// [`VkBufferUsageFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlags.html)
///
#[doc(alias = "VkBufferUsageFlags")]
pub type BufferUsageFlags = FlagSet<BufferUsageFlag>;

crate::__vkx_internal_flags! {
    /// [`VkImageViewCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewCreateFlagBits.html)
    ///
    #[doc(alias = "VkImageViewCreateFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ImageViewCreateFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_FragmentDensityMap`](Extension::EXT_FragmentDensityMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT")]
        #[default]
        FRAGMENT_DENSITY_MAP_DYNAMIC_EXT = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorBuffer`](Extension::EXT_DescriptorBuffer)
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_VIEW_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT")]
        DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_FragmentDensityMap2`](Extension::EXT_FragmentDensityMap2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT")]
        FRAGMENT_DENSITY_MAP_DEFERRED_EXT = 2,
    }
}
/// [`VkImageViewCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewCreateFlags.html)
///
#[doc(alias = "VkImageViewCreateFlags")]
pub type ImageViewCreateFlags = FlagSet<ImageViewCreateFlag>;

crate::__vkx_internal_flags! {
    /// [`VkAccessFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlagBits.html)
    ///
    #[doc(alias = "VkAccessFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum AccessFlag: u32 {
        #[doc(alias = "VK_ACCESS_INDIRECT_COMMAND_READ_BIT")]
        #[default]
        INDIRECT_COMMAND_READ = 1,
        #[doc(alias = "VK_ACCESS_INDEX_READ_BIT")]
        INDEX_READ = 2,
        #[doc(alias = "VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT")]
        VERTEX_ATTRIBUTE_READ = 4,
        #[doc(alias = "VK_ACCESS_UNIFORM_READ_BIT")]
        UNIFORM_READ = 8,
        #[doc(alias = "VK_ACCESS_INPUT_ATTACHMENT_READ_BIT")]
        INPUT_ATTACHMENT_READ = 16,
        #[doc(alias = "VK_ACCESS_SHADER_READ_BIT")]
        SHADER_READ = 32,
        #[doc(alias = "VK_ACCESS_SHADER_WRITE_BIT")]
        SHADER_WRITE = 64,
        #[doc(alias = "VK_ACCESS_COLOR_ATTACHMENT_READ_BIT")]
        COLOR_ATTACHMENT_READ = 128,
        #[doc(alias = "VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT")]
        COLOR_ATTACHMENT_WRITE = 256,
        #[doc(alias = "VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT")]
        DEPTH_STENCIL_ATTACHMENT_READ = 512,
        #[doc(alias = "VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT")]
        DEPTH_STENCIL_ATTACHMENT_WRITE = 1024,
        #[doc(alias = "VK_ACCESS_TRANSFER_READ_BIT")]
        TRANSFER_READ = 2048,
        #[doc(alias = "VK_ACCESS_TRANSFER_WRITE_BIT")]
        TRANSFER_WRITE = 4096,
        #[doc(alias = "VK_ACCESS_HOST_READ_BIT")]
        HOST_READ = 8192,
        #[doc(alias = "VK_ACCESS_HOST_WRITE_BIT")]
        HOST_WRITE = 16384,
        #[doc(alias = "VK_ACCESS_MEMORY_READ_BIT")]
        MEMORY_READ = 32768,
        #[doc(alias = "VK_ACCESS_MEMORY_WRITE_BIT")]
        MEMORY_WRITE = 65536,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_NONE")]
        NONE = 0,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_TransformFeedback`](Extension::EXT_TransformFeedback)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_TRANSFORM_FEEDBACK_WRITE_BIT_EXT")]
        TRANSFORM_FEEDBACK_WRITE_EXT = 33554432,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_TransformFeedback`](Extension::EXT_TransformFeedback)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT")]
        TRANSFORM_FEEDBACK_COUNTER_READ_EXT = 67108864,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_TransformFeedback`](Extension::EXT_TransformFeedback)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT")]
        TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT = 134217728,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ConditionalRendering`](Extension::EXT_ConditionalRendering)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_CONDITIONAL_RENDERING_READ_BIT_EXT")]
        CONDITIONAL_RENDERING_READ_EXT = 1048576,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_BlendOperationAdvanced`](Extension::EXT_BlendOperationAdvanced)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT")]
        COLOR_ATTACHMENT_READ_NONCOHERENT_EXT = 524288,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR")]
        ACCELERATION_STRUCTURE_READ_KHR = 2097152,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR")]
        ACCELERATION_STRUCTURE_WRITE_KHR = 4194304,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_FragmentDensityMap`](Extension::EXT_FragmentDensityMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_FRAGMENT_DENSITY_MAP_READ_BIT_EXT")]
        FRAGMENT_DENSITY_MAP_READ_EXT = 16777216,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_FragmentShadingRate`](Extension::KHR_FragmentShadingRate)
        /// - Extension [`NV_ShadingRateImage`](Extension::NV_ShadingRateImage)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR")]
        FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR = 8388608,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceGeneratedCommands`](Extension::NV_DeviceGeneratedCommands)
        /// - Extension [`EXT_DeviceGeneratedCommands`](Extension::EXT_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_COMMAND_PREPROCESS_READ_BIT_EXT")]
        COMMAND_PREPROCESS_READ_EXT = 131072,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceGeneratedCommands`](Extension::NV_DeviceGeneratedCommands)
        /// - Extension [`EXT_DeviceGeneratedCommands`](Extension::EXT_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_COMMAND_PREPROCESS_WRITE_BIT_EXT")]
        COMMAND_PREPROCESS_WRITE_EXT = 262144,
    }
}
impl AccessFlag {
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
/// [`VkAccessFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlags.html)
///
#[doc(alias = "VkAccessFlags")]
pub type AccessFlags = FlagSet<AccessFlag>;

crate::__vkx_internal_flags! {
    /// [`VkDependencyFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDependencyFlagBits.html)
    ///
    #[doc(alias = "VkDependencyFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum DependencyFlag: u32 {
        #[doc(alias = "VK_DEPENDENCY_BY_REGION_BIT")]
        #[default]
        BY_REGION = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_DeviceGroup`](Extension::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEPENDENCY_DEVICE_GROUP_BIT")]
        DEVICE_GROUP = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_Multiview`](Extension::KHR_Multiview)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEPENDENCY_VIEW_LOCAL_BIT")]
        VIEW_LOCAL = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_AttachmentFeedbackLoopLayout`](Extension::EXT_AttachmentFeedbackLoopLayout)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEPENDENCY_FEEDBACK_LOOP_BIT_EXT")]
        FEEDBACK_LOOP_EXT = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance8`](Extension::KHR_Maintenance8)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEPENDENCY_QUEUE_FAMILY_OWNERSHIP_TRANSFER_USE_ALL_STAGES_BIT_KHR")]
        QUEUE_FAMILY_OWNERSHIP_TRANSFER_USE_ALL_STAGES_KHR = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance9`](Extension::KHR_Maintenance9)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEPENDENCY_ASYMMETRIC_EVENT_BIT_KHR")]
        ASYMMETRIC_EVENT_KHR = 64,
    }
}
impl DependencyFlag {
    /// [`VK_DEPENDENCY_DEVICE_GROUP_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_DEPENDENCY_DEVICE_GROUP_BIT_KHR.html)
    ///
    #[doc(alias = "VK_DEPENDENCY_DEVICE_GROUP_BIT_KHR")]
    pub const DEVICE_GROUP_KHR: Self = Self::DEVICE_GROUP;
    /// [`VK_DEPENDENCY_VIEW_LOCAL_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_DEPENDENCY_VIEW_LOCAL_BIT_KHR.html)
    ///
    #[doc(alias = "VK_DEPENDENCY_VIEW_LOCAL_BIT_KHR")]
    pub const VIEW_LOCAL_KHR: Self = Self::VIEW_LOCAL;
}
/// [`VkDependencyFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDependencyFlags.html)
///
#[doc(alias = "VkDependencyFlags")]
pub type DependencyFlags = FlagSet<DependencyFlag>;

crate::__vkx_internal_flags! {
    /// [`VkCommandPoolCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolCreateFlagBits.html)
    ///
    #[doc(alias = "VkCommandPoolCreateFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum CommandPoolCreateFlag: u32 {
        #[doc(alias = "VK_COMMAND_POOL_CREATE_TRANSIENT_BIT")]
        #[default]
        TRANSIENT = 1,
        #[doc(alias = "VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT")]
        RESET_COMMAND_BUFFER = 2,
        #[doc(alias = "VK_COMMAND_POOL_CREATE_PROTECTED_BIT")]
        PROTECTED = 4,
    }
}
/// [`VkCommandPoolCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolCreateFlags.html)
///
#[doc(alias = "VkCommandPoolCreateFlags")]
pub type CommandPoolCreateFlags = FlagSet<CommandPoolCreateFlag>;

crate::__vkx_internal_flags! {
    /// [`VkCommandPoolResetFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolResetFlagBits.html)
    ///
    #[doc(alias = "VkCommandPoolResetFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum CommandPoolResetFlag: u32 {
        #[doc(alias = "VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT")]
        #[default]
        RELEASE_RESOURCES = 1,
    }
}
/// [`VkCommandPoolResetFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolResetFlags.html)
///
#[doc(alias = "VkCommandPoolResetFlags")]
pub type CommandPoolResetFlags = FlagSet<CommandPoolResetFlag>;

crate::__vkx_internal_flags! {
    /// [`VkQueryControlFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryControlFlagBits.html)
    ///
    #[doc(alias = "VkQueryControlFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum QueryControlFlag: u32 {
        #[doc(alias = "VK_QUERY_CONTROL_PRECISE_BIT")]
        #[default]
        PRECISE = 1,
    }
}
/// [`VkQueryControlFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryControlFlags.html)
///
#[doc(alias = "VkQueryControlFlags")]
pub type QueryControlFlags = FlagSet<QueryControlFlag>;

crate::__vkx_internal_flags! {
    /// [`VkCommandBufferUsageFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferUsageFlagBits.html)
    ///
    #[doc(alias = "VkCommandBufferUsageFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum CommandBufferUsageFlag: u32 {
        #[doc(alias = "VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT")]
        #[default]
        ONE_TIME_SUBMIT = 1,
        #[doc(alias = "VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT")]
        RENDER_PASS_CONTINUE = 2,
        #[doc(alias = "VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT")]
        SIMULTANEOUS_USE = 4,
    }
}
/// [`VkCommandBufferUsageFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferUsageFlags.html)
///
#[doc(alias = "VkCommandBufferUsageFlags")]
pub type CommandBufferUsageFlags = FlagSet<CommandBufferUsageFlag>;

crate::__vkx_internal_flags! {
    /// [`VkCommandBufferResetFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferResetFlagBits.html)
    ///
    #[doc(alias = "VkCommandBufferResetFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum CommandBufferResetFlag: u32 {
        #[doc(alias = "VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT")]
        #[default]
        RELEASE_RESOURCES = 1,
    }
}
/// [`VkCommandBufferResetFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferResetFlags.html)
///
#[doc(alias = "VkCommandBufferResetFlags")]
pub type CommandBufferResetFlags = FlagSet<CommandBufferResetFlag>;

crate::__vkx_internal_flags! {
    /// [`VkEventCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkEventCreateFlagBits.html)
    ///
    #[doc(alias = "VkEventCreateFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum EventCreateFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EVENT_CREATE_DEVICE_ONLY_BIT")]
        #[default]
        DEVICE_ONLY = 1,
    }
}
impl EventCreateFlag {
    /// [`VK_EVENT_CREATE_DEVICE_ONLY_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EVENT_CREATE_DEVICE_ONLY_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EVENT_CREATE_DEVICE_ONLY_BIT_KHR")]
    pub const DEVICE_ONLY_KHR: Self = Self::DEVICE_ONLY;
}
/// [`VkEventCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkEventCreateFlags.html)
///
#[doc(alias = "VkEventCreateFlags")]
pub type EventCreateFlags = FlagSet<EventCreateFlag>;

crate::__vkx_internal_flags! {
    /// [`VkPipelineCacheCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCacheCreateFlagBits.html)
    ///
    #[doc(alias = "VkPipelineCacheCreateFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum PipelineCacheCreateFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_PipelineCreationCacheControl`](Extension::EXT_PipelineCreationCacheControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT")]
        #[default]
        EXTERNALLY_SYNCHRONIZED = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance8`](Extension::KHR_Maintenance8)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CACHE_CREATE_INTERNALLY_SYNCHRONIZED_MERGE_BIT_KHR")]
        INTERNALLY_SYNCHRONIZED_MERGE_KHR = 8,
    }
}
impl PipelineCacheCreateFlag {
    /// [`VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT_EXT.html)
    ///
    #[doc(alias = "VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT_EXT")]
    pub const EXTERNALLY_SYNCHRONIZED_EXT: Self = Self::EXTERNALLY_SYNCHRONIZED;
}
/// [`VkPipelineCacheCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCacheCreateFlags.html)
///
#[doc(alias = "VkPipelineCacheCreateFlags")]
pub type PipelineCacheCreateFlags = FlagSet<PipelineCacheCreateFlag>;

crate::__vkx_internal_flags! {
    /// [`VkPipelineCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlagBits.html)
    ///
    #[doc(alias = "VkPipelineCreateFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum PipelineCreateFlag: u32 {
        #[doc(alias = "VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT")]
        #[default]
        DISABLE_OPTIMIZATION = 1,
        #[doc(alias = "VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT")]
        ALLOW_DERIVATIVES = 2,
        #[doc(alias = "VK_PIPELINE_CREATE_DERIVATIVE_BIT")]
        DERIVATIVE = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_DeviceGroup`](Extension::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_DISPATCH_BASE_BIT")]
        DISPATCH_BASE = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_DeviceGroup`](Extension::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT")]
        VIEW_INDEX_FROM_DEVICE_INDEX = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_PipelineCreationCacheControl`](Extension::EXT_PipelineCreationCacheControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT")]
        FAIL_ON_PIPELINE_COMPILE_REQUIRED = 256,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_PipelineCreationCacheControl`](Extension::EXT_PipelineCreationCacheControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT")]
        EARLY_RETURN_ON_FAILURE = 512,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`EXT_PipelineProtectedAccess`](Extension::EXT_PipelineProtectedAccess)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_NO_PROTECTED_ACCESS_BIT")]
        NO_PROTECTED_ACCESS = 134217728,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`EXT_PipelineProtectedAccess`](Extension::EXT_PipelineProtectedAccess)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_PROTECTED_ACCESS_ONLY_BIT")]
        PROTECTED_ACCESS_ONLY = 1073741824,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR")]
        RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR = 16384,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR")]
        RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR = 32768,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR")]
        RAY_TRACING_NO_NULL_MISS_SHADERS_KHR = 65536,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR")]
        RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR = 131072,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR")]
        RAY_TRACING_SKIP_TRIANGLES_KHR = 4096,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR")]
        RAY_TRACING_SKIP_AABBS_KHR = 8192,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR")]
        RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR = 524288,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_DEFER_COMPILE_BIT_NV")]
        DEFER_COMPILE_NV = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_FragmentDensityMap`](Extension::EXT_FragmentDensityMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT")]
        RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT = 4194304,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_FragmentShadingRate`](Extension::KHR_FragmentShadingRate)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
        RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = 2097152,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_PipelineExecutableProperties`](Extension::KHR_PipelineExecutableProperties)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_CAPTURE_STATISTICS_BIT_KHR")]
        CAPTURE_STATISTICS_KHR = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_PipelineExecutableProperties`](Extension::KHR_PipelineExecutableProperties)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR")]
        CAPTURE_INTERNAL_REPRESENTATIONS_KHR = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceGeneratedCommands`](Extension::NV_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_INDIRECT_BINDABLE_BIT_NV")]
        INDIRECT_BINDABLE_NV = 262144,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_PipelineLibrary`](Extension::KHR_PipelineLibrary)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_LIBRARY_BIT_KHR")]
        LIBRARY_KHR = 2048,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorBuffer`](Extension::EXT_DescriptorBuffer)
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_DESCRIPTOR_BUFFER_BIT_EXT")]
        DESCRIPTOR_BUFFER_EXT = 536870912,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_GraphicsPipelineLibrary`](Extension::EXT_GraphicsPipelineLibrary)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RETAIN_LINK_TIME_OPTIMIZATION_INFO_BIT_EXT")]
        RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT = 8388608,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_GraphicsPipelineLibrary`](Extension::EXT_GraphicsPipelineLibrary)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_LINK_TIME_OPTIMIZATION_BIT_EXT")]
        LINK_TIME_OPTIMIZATION_EXT = 1024,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracingMotionBlur`](Extension::NV_RayTracingMotionBlur)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_ALLOW_MOTION_BIT_NV")]
        RAY_TRACING_ALLOW_MOTION_NV = 1048576,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_AttachmentFeedbackLoopLayout`](Extension::EXT_AttachmentFeedbackLoopLayout)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_COLOR_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT")]
        COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT = 33554432,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_AttachmentFeedbackLoopLayout`](Extension::EXT_AttachmentFeedbackLoopLayout)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT")]
        DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT = 67108864,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DisplacementMicromap`](Extension::NV_DisplacementMicromap)
        /// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_DISPLACEMENT_MICROMAP_BIT_NV")]
        RAY_TRACING_DISPLACEMENT_MICROMAP_NV = 268435456,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
        /// - Extension [`EXT_OpacityMicromap`](Extension::EXT_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_RAY_TRACING_OPACITY_MICROMAP_BIT_KHR")]
        RAY_TRACING_OPACITY_MICROMAP_KHR = 16777216,
    }
}
impl PipelineCreateFlag {
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
/// [`VkPipelineCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlags.html)
///
#[doc(alias = "VkPipelineCreateFlags")]
pub type PipelineCreateFlags = FlagSet<PipelineCreateFlag>;

crate::__vkx_internal_flags! {
    /// [`VkPipelineLayoutCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineLayoutCreateFlagBits.html)
    ///
    #[doc(alias = "VkPipelineLayoutCreateFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum PipelineLayoutCreateFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance11`](Extension::KHR_Maintenance11)
        /// - Extension [`EXT_GraphicsPipelineLibrary`](Extension::EXT_GraphicsPipelineLibrary)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_LAYOUT_CREATE_INDEPENDENT_SETS_BIT_EXT")]
        #[default]
        INDEPENDENT_SETS_EXT = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance11`](Extension::KHR_Maintenance11)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_LAYOUT_CREATE_NO_TASK_SHADER_BIT_KHR")]
        NO_TASK_SHADER_KHR = 4,
    }
}
/// [`VkPipelineLayoutCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineLayoutCreateFlags.html)
///
#[doc(alias = "VkPipelineLayoutCreateFlags")]
pub type PipelineLayoutCreateFlags = FlagSet<PipelineLayoutCreateFlag>;

crate::__vkx_internal_flags! {
    /// [`VkPipelineShaderStageCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineShaderStageCreateFlagBits.html)
    ///
    #[doc(alias = "VkPipelineShaderStageCreateFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum PipelineShaderStageCreateFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_SubgroupSizeControl`](Extension::EXT_SubgroupSizeControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT")]
        #[default]
        ALLOW_VARYING_SUBGROUP_SIZE = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_SubgroupSizeControl`](Extension::EXT_SubgroupSizeControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT")]
        REQUIRE_FULL_SUBGROUPS = 2,
    }
}
impl PipelineShaderStageCreateFlag {
    /// [`VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT_EXT.html)
    ///
    #[doc(alias = "VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT_EXT")]
    pub const ALLOW_VARYING_SUBGROUP_SIZE_EXT: Self = Self::ALLOW_VARYING_SUBGROUP_SIZE;
    /// [`VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT_EXT.html)
    ///
    #[doc(alias = "VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT_EXT")]
    pub const REQUIRE_FULL_SUBGROUPS_EXT: Self = Self::REQUIRE_FULL_SUBGROUPS;
}
/// [`VkPipelineShaderStageCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineShaderStageCreateFlags.html)
///
#[doc(alias = "VkPipelineShaderStageCreateFlags")]
pub type PipelineShaderStageCreateFlags = FlagSet<PipelineShaderStageCreateFlag>;

crate::__vkx_internal_flags! {
    /// [`VkSamplerCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerCreateFlagBits.html)
    ///
    #[doc(alias = "VkSamplerCreateFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum SamplerCreateFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_FragmentDensityMap`](Extension::EXT_FragmentDensityMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT")]
        #[default]
        SUBSAMPLED_EXT = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_FragmentDensityMap`](Extension::EXT_FragmentDensityMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SAMPLER_CREATE_SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT")]
        SUBSAMPLED_COARSE_RECONSTRUCTION_EXT = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorBuffer`](Extension::EXT_DescriptorBuffer)
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SAMPLER_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT")]
        DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_NonSeamlessCubeMap`](Extension::EXT_NonSeamlessCubeMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SAMPLER_CREATE_NON_SEAMLESS_CUBE_MAP_BIT_EXT")]
        NON_SEAMLESS_CUBE_MAP_EXT = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_ImageProcessing`](Extension::QCOM_ImageProcessing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SAMPLER_CREATE_IMAGE_PROCESSING_BIT_QCOM")]
        IMAGE_PROCESSING_QCOM = 16,
    }
}
/// [`VkSamplerCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerCreateFlags.html)
///
#[doc(alias = "VkSamplerCreateFlags")]
pub type SamplerCreateFlags = FlagSet<SamplerCreateFlag>;

crate::__vkx_internal_flags! {
    /// [`VkDescriptorPoolCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPoolCreateFlagBits.html)
    ///
    #[doc(alias = "VkDescriptorPoolCreateFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum DescriptorPoolCreateFlag: u32 {
        #[doc(alias = "VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT")]
        #[default]
        FREE_DESCRIPTOR_SET = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`EXT_DescriptorIndexing`](Extension::EXT_DescriptorIndexing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT")]
        UPDATE_AFTER_BIND = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_MutableDescriptorType`](Extension::VALVE_MutableDescriptorType)
        /// - Extension [`EXT_MutableDescriptorType`](Extension::EXT_MutableDescriptorType)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_EXT")]
        HOST_ONLY_EXT = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DescriptorPoolOverallocation`](Extension::NV_DescriptorPoolOverallocation)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_POOL_CREATE_ALLOW_OVERALLOCATION_SETS_BIT_NV")]
        ALLOW_OVERALLOCATION_SETS_NV = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DescriptorPoolOverallocation`](Extension::NV_DescriptorPoolOverallocation)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_POOL_CREATE_ALLOW_OVERALLOCATION_POOLS_BIT_NV")]
        ALLOW_OVERALLOCATION_POOLS_NV = 16,
    }
}
impl DescriptorPoolCreateFlag {
    /// [`VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT_EXT.html)
    ///
    #[doc(alias = "VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT_EXT")]
    pub const UPDATE_AFTER_BIND_EXT: Self = Self::UPDATE_AFTER_BIND;
    /// [`VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE.html)
    ///
    #[doc(alias = "VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE")]
    pub const HOST_ONLY_VALVE: Self = Self::HOST_ONLY_EXT;
}
/// [`VkDescriptorPoolCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPoolCreateFlags.html)
///
#[doc(alias = "VkDescriptorPoolCreateFlags")]
pub type DescriptorPoolCreateFlags = FlagSet<DescriptorPoolCreateFlag>;

crate::__vkx_internal_flags! {
    /// [`VkDescriptorSetLayoutCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayoutCreateFlagBits.html)
    ///
    #[doc(alias = "VkDescriptorSetLayoutCreateFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum DescriptorSetLayoutCreateFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`EXT_DescriptorIndexing`](Extension::EXT_DescriptorIndexing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT")]
        #[default]
        UPDATE_AFTER_BIND_POOL = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_PushDescriptor`](Extension::KHR_PushDescriptor)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT")]
        PUSH_DESCRIPTOR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorBuffer`](Extension::EXT_DescriptorBuffer)
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_DESCRIPTOR_BUFFER_BIT_EXT")]
        DESCRIPTOR_BUFFER_EXT = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorBuffer`](Extension::EXT_DescriptorBuffer)
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_EMBEDDED_IMMUTABLE_SAMPLERS_BIT_EXT")]
        EMBEDDED_IMMUTABLE_SAMPLERS_EXT = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceGeneratedCommandsCompute`](Extension::NV_DeviceGeneratedCommandsCompute)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_INDIRECT_BINDABLE_BIT_NV")]
        INDIRECT_BINDABLE_NV = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_MutableDescriptorType`](Extension::VALVE_MutableDescriptorType)
        /// - Extension [`EXT_MutableDescriptorType`](Extension::EXT_MutableDescriptorType)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_EXT")]
        HOST_ONLY_POOL_EXT = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_PerStageDescriptorSet`](Extension::NV_PerStageDescriptorSet)
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_SET_LAYOUT_CREATE_PER_STAGE_BIT_NV")]
        PER_STAGE_NV = 64,
    }
}
impl DescriptorSetLayoutCreateFlag {
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
/// [`VkDescriptorSetLayoutCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayoutCreateFlags.html)
///
#[doc(alias = "VkDescriptorSetLayoutCreateFlags")]
pub type DescriptorSetLayoutCreateFlags = FlagSet<DescriptorSetLayoutCreateFlag>;

crate::__vkx_internal_flags! {
    /// [`VkColorComponentFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkColorComponentFlagBits.html)
    ///
    #[doc(alias = "VkColorComponentFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ColorComponentFlag: u32 {
        #[doc(alias = "VK_COLOR_COMPONENT_R_BIT")]
        #[default]
        R = 1,
        #[doc(alias = "VK_COLOR_COMPONENT_G_BIT")]
        G = 2,
        #[doc(alias = "VK_COLOR_COMPONENT_B_BIT")]
        B = 4,
        #[doc(alias = "VK_COLOR_COMPONENT_A_BIT")]
        A = 8,
    }
}
/// [`VkColorComponentFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkColorComponentFlags.html)
///
#[doc(alias = "VkColorComponentFlags")]
pub type ColorComponentFlags = FlagSet<ColorComponentFlag>;

crate::__vkx_internal_flags! {
    /// [`VkCullModeFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCullModeFlagBits.html)
    ///
    #[doc(alias = "VkCullModeFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum CullModeFlag: u32 {
        #[doc(alias = "VK_CULL_MODE_NONE")]
        #[default]
        NONE = 0,
        #[doc(alias = "VK_CULL_MODE_FRONT_BIT")]
        FRONT = 1,
        #[doc(alias = "VK_CULL_MODE_BACK_BIT")]
        BACK = 2,
    }
}
impl CullModeFlag {
    #[doc(alias = "VK_CULL_MODE_FRONT_AND_BACK")]
    pub const FRONT_AND_BACK: FlagSet<Self> = FlagSet(3);
}
/// [`VkCullModeFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCullModeFlags.html)
///
#[doc(alias = "VkCullModeFlags")]
pub type CullModeFlags = FlagSet<CullModeFlag>;

crate::__vkx_internal_flags! {
    /// [`VkPipelineColorBlendStateCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineColorBlendStateCreateFlagBits.html)
    ///
    #[doc(alias = "VkPipelineColorBlendStateCreateFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum PipelineColorBlendStateCreateFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_RasterizationOrderAttachmentAccess`](Extension::ARM_RasterizationOrderAttachmentAccess)
        /// - Extension [`EXT_RasterizationOrderAttachmentAccess`](Extension::EXT_RasterizationOrderAttachmentAccess)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_COLOR_BLEND_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_EXT")]
        #[default]
        RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXT = 1,
    }
}
impl PipelineColorBlendStateCreateFlag {
    /// [`VK_PIPELINE_COLOR_BLEND_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_ARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PIPELINE_COLOR_BLEND_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_ARM.html)
    ///
    #[doc(
        alias = "VK_PIPELINE_COLOR_BLEND_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_ARM"
    )]
    pub const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXT;
}
/// [`VkPipelineColorBlendStateCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineColorBlendStateCreateFlags.html)
///
#[doc(alias = "VkPipelineColorBlendStateCreateFlags")]
pub type PipelineColorBlendStateCreateFlags = FlagSet<PipelineColorBlendStateCreateFlag>;

crate::__vkx_internal_flags! {
    /// [`VkPipelineDepthStencilStateCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineDepthStencilStateCreateFlagBits.html)
    ///
    #[doc(alias = "VkPipelineDepthStencilStateCreateFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum PipelineDepthStencilStateCreateFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_RasterizationOrderAttachmentAccess`](Extension::ARM_RasterizationOrderAttachmentAccess)
        /// - Extension [`EXT_RasterizationOrderAttachmentAccess`](Extension::EXT_RasterizationOrderAttachmentAccess)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT")]
        #[default]
        RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_RasterizationOrderAttachmentAccess`](Extension::ARM_RasterizationOrderAttachmentAccess)
        /// - Extension [`EXT_RasterizationOrderAttachmentAccess`](Extension::EXT_RasterizationOrderAttachmentAccess)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT")]
        RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT = 2,
    }
}
impl PipelineDepthStencilStateCreateFlag {
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
/// [`VkPipelineDepthStencilStateCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineDepthStencilStateCreateFlags.html)
///
#[doc(alias = "VkPipelineDepthStencilStateCreateFlags")]
pub type PipelineDepthStencilStateCreateFlags = FlagSet<PipelineDepthStencilStateCreateFlag>;

crate::__vkx_internal_flags! {
    /// [`VkAttachmentDescriptionFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentDescriptionFlagBits.html)
    ///
    #[doc(alias = "VkAttachmentDescriptionFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum AttachmentDescriptionFlag: u32 {
        #[doc(alias = "VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT")]
        #[default]
        MAY_ALIAS = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance10`](Extension::KHR_Maintenance10)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ATTACHMENT_DESCRIPTION_RESOLVE_SKIP_TRANSFER_FUNCTION_BIT_KHR")]
        RESOLVE_SKIP_TRANSFER_FUNCTION_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance10`](Extension::KHR_Maintenance10)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ATTACHMENT_DESCRIPTION_RESOLVE_ENABLE_TRANSFER_FUNCTION_BIT_KHR")]
        RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR = 4,
    }
}
/// [`VkAttachmentDescriptionFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentDescriptionFlags.html)
///
#[doc(alias = "VkAttachmentDescriptionFlags")]
pub type AttachmentDescriptionFlags = FlagSet<AttachmentDescriptionFlag>;

crate::__vkx_internal_flags! {
    /// [`VkFramebufferCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebufferCreateFlagBits.html)
    ///
    #[doc(alias = "VkFramebufferCreateFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum FramebufferCreateFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`KHR_ImagelessFramebuffer`](Extension::KHR_ImagelessFramebuffer)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT")]
        #[default]
        IMAGELESS = 1,
    }
}
impl FramebufferCreateFlag {
    /// [`VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT_KHR")]
    pub const IMAGELESS_KHR: Self = Self::IMAGELESS;
}
/// [`VkFramebufferCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebufferCreateFlags.html)
///
#[doc(alias = "VkFramebufferCreateFlags")]
pub type FramebufferCreateFlags = FlagSet<FramebufferCreateFlag>;

crate::__vkx_internal_flags! {
    /// [`VkRenderPassCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassCreateFlagBits.html)
    ///
    #[doc(alias = "VkRenderPassCreateFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum RenderPassCreateFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_RenderPassTransform`](Extension::QCOM_RenderPassTransform)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDER_PASS_CREATE_TRANSFORM_BIT_QCOM")]
        #[default]
        TRANSFORM_QCOM = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_FragmentDensityMapLayered`](Extension::VALVE_FragmentDensityMapLayered)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDER_PASS_CREATE_PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE")]
        PER_LAYER_FRAGMENT_DENSITY_VALVE = 4,
    }
}
/// [`VkRenderPassCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassCreateFlags.html)
///
#[doc(alias = "VkRenderPassCreateFlags")]
pub type RenderPassCreateFlags = FlagSet<RenderPassCreateFlag>;

crate::__vkx_internal_flags! {
    /// [`VkSubpassDescriptionFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassDescriptionFlagBits.html)
    ///
    #[doc(alias = "VkSubpassDescriptionFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum SubpassDescriptionFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NVX_MultiviewPerViewAttributes`](Extension::NVX_MultiviewPerViewAttributes)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX")]
        #[default]
        PER_VIEW_ATTRIBUTES_NVX = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NVX_MultiviewPerViewAttributes`](Extension::NVX_MultiviewPerViewAttributes)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_PER_VIEW_POSITION_X_ONLY_BIT_NVX")]
        PER_VIEW_POSITION_X_ONLY_NVX = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_TileShading`](Extension::QCOM_TileShading)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_TILE_SHADING_APRON_BIT_QCOM")]
        TILE_SHADING_APRON_QCOM = 256,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_RasterizationOrderAttachmentAccess`](Extension::ARM_RasterizationOrderAttachmentAccess)
        /// - Extension [`EXT_RasterizationOrderAttachmentAccess`](Extension::EXT_RasterizationOrderAttachmentAccess)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_EXT")]
        RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_EXT = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_RasterizationOrderAttachmentAccess`](Extension::ARM_RasterizationOrderAttachmentAccess)
        /// - Extension [`EXT_RasterizationOrderAttachmentAccess`](Extension::EXT_RasterizationOrderAttachmentAccess)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT")]
        RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_RasterizationOrderAttachmentAccess`](Extension::ARM_RasterizationOrderAttachmentAccess)
        /// - Extension [`EXT_RasterizationOrderAttachmentAccess`](Extension::EXT_RasterizationOrderAttachmentAccess)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT")]
        RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_LegacyDithering`](Extension::EXT_LegacyDithering)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_ENABLE_LEGACY_DITHERING_BIT_EXT")]
        ENABLE_LEGACY_DITHERING_EXT = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_RenderPassShaderResolve`](Extension::QCOM_RenderPassShaderResolve)
        /// - Extension [`EXT_CustomResolve`](Extension::EXT_CustomResolve)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_FRAGMENT_REGION_BIT_EXT")]
        FRAGMENT_REGION_EXT = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_RenderPassShaderResolve`](Extension::QCOM_RenderPassShaderResolve)
        /// - Extension [`EXT_CustomResolve`](Extension::EXT_CustomResolve)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBPASS_DESCRIPTION_CUSTOM_RESOLVE_BIT_EXT")]
        CUSTOM_RESOLVE_EXT = 8,
    }
}
impl SubpassDescriptionFlag {
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
/// [`VkSubpassDescriptionFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassDescriptionFlags.html)
///
#[doc(alias = "VkSubpassDescriptionFlags")]
pub type SubpassDescriptionFlags = FlagSet<SubpassDescriptionFlag>;

crate::__vkx_internal_flags! {
    /// [`VkStencilFaceFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkStencilFaceFlagBits.html)
    ///
    #[doc(alias = "VkStencilFaceFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum StencilFaceFlag: u32 {
        #[doc(alias = "VK_STENCIL_FACE_FRONT_BIT")]
        #[default]
        FRONT = 1,
        #[doc(alias = "VK_STENCIL_FACE_BACK_BIT")]
        BACK = 2,
    }
}
impl StencilFaceFlag {
    #[doc(alias = "VK_STENCIL_FACE_FRONT_AND_BACK")]
    pub const FRONT_AND_BACK: FlagSet<Self> = FlagSet(3);
}
/// [`VkStencilFaceFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkStencilFaceFlags.html)
///
#[doc(alias = "VkStencilFaceFlags")]
pub type StencilFaceFlags = FlagSet<StencilFaceFlag>;

crate::__vkx_internal_flags! {
    /// [`VkSubgroupFeatureFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubgroupFeatureFlagBits.html)
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkSubgroupFeatureFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum SubgroupFeatureFlag: u32 {
        #[doc(alias = "VK_SUBGROUP_FEATURE_BASIC_BIT")]
        #[default]
        BASIC = 1,
        #[doc(alias = "VK_SUBGROUP_FEATURE_VOTE_BIT")]
        VOTE = 2,
        #[doc(alias = "VK_SUBGROUP_FEATURE_ARITHMETIC_BIT")]
        ARITHMETIC = 4,
        #[doc(alias = "VK_SUBGROUP_FEATURE_BALLOT_BIT")]
        BALLOT = 8,
        #[doc(alias = "VK_SUBGROUP_FEATURE_SHUFFLE_BIT")]
        SHUFFLE = 16,
        #[doc(alias = "VK_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT")]
        SHUFFLE_RELATIVE = 32,
        #[doc(alias = "VK_SUBGROUP_FEATURE_CLUSTERED_BIT")]
        CLUSTERED = 64,
        #[doc(alias = "VK_SUBGROUP_FEATURE_QUAD_BIT")]
        QUAD = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_ShaderSubgroupRotate`](Extension::KHR_ShaderSubgroupRotate)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBGROUP_FEATURE_ROTATE_BIT")]
        ROTATE = 512,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_ShaderSubgroupRotate`](Extension::KHR_ShaderSubgroupRotate)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBGROUP_FEATURE_ROTATE_CLUSTERED_BIT")]
        ROTATE_CLUSTERED = 1024,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ShaderSubgroupPartitioned`](Extension::NV_ShaderSubgroupPartitioned)
        /// - Extension [`EXT_ShaderSubgroupPartitioned`](Extension::EXT_ShaderSubgroupPartitioned)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBGROUP_FEATURE_PARTITIONED_BIT_EXT")]
        PARTITIONED_EXT = 256,
    }
}
impl SubgroupFeatureFlag {
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
/// [`VkSubgroupFeatureFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubgroupFeatureFlags.html)
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkSubgroupFeatureFlags")]
pub type SubgroupFeatureFlags = FlagSet<SubgroupFeatureFlag>;

crate::__vkx_internal_flags! {
    /// [`VkPeerMemoryFeatureFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPeerMemoryFeatureFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.1 with appropriate features
    /// - Extension [`KHR_DeviceGroup`](Extension::KHR_DeviceGroup)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkPeerMemoryFeatureFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum PeerMemoryFeatureFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_DeviceGroup`](Extension::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT")]
        #[default]
        COPY_SRC = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_DeviceGroup`](Extension::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_DST_BIT")]
        COPY_DST = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_DeviceGroup`](Extension::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT")]
        GENERIC_SRC = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_DeviceGroup`](Extension::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT")]
        GENERIC_DST = 8,
    }
}
/// [`VkPeerMemoryFeatureFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPeerMemoryFeatureFlagBitsKHR.html)
///
#[doc(alias = "VkPeerMemoryFeatureFlagBitsKHR")]
pub type PeerMemoryFeatureFlagKHR = PeerMemoryFeatureFlag;
impl PeerMemoryFeatureFlag {
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
/// [`VkPeerMemoryFeatureFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPeerMemoryFeatureFlags.html)
///
#[doc(alias = "VkPeerMemoryFeatureFlags")]
pub type PeerMemoryFeatureFlags = FlagSet<PeerMemoryFeatureFlag>;
/// [`VkPeerMemoryFeatureFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPeerMemoryFeatureFlagsKHR.html)
///
#[doc(alias = "VkPeerMemoryFeatureFlagsKHR")]
pub type PeerMemoryFeatureFlagsKHR = PeerMemoryFeatureFlags;

crate::__vkx_internal_flags! {
    /// [`VkMemoryAllocateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryAllocateFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.1 with appropriate features
    /// - Extension [`KHR_DeviceGroup`](Extension::KHR_DeviceGroup)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkMemoryAllocateFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum MemoryAllocateFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_DeviceGroup`](Extension::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT")]
        #[default]
        DEVICE_MASK = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`KHR_BufferDeviceAddress`](Extension::KHR_BufferDeviceAddress)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT")]
        DEVICE_ADDRESS = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`KHR_BufferDeviceAddress`](Extension::KHR_BufferDeviceAddress)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT")]
        DEVICE_ADDRESS_CAPTURE_REPLAY = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ZeroInitializeDeviceMemory`](Extension::EXT_ZeroInitializeDeviceMemory)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MEMORY_ALLOCATE_ZERO_INITIALIZE_BIT_EXT")]
        ZERO_INITIALIZE_EXT = 8,
    }
}
/// [`VkMemoryAllocateFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryAllocateFlagBitsKHR.html)
///
#[doc(alias = "VkMemoryAllocateFlagBitsKHR")]
pub type MemoryAllocateFlagKHR = MemoryAllocateFlag;
impl MemoryAllocateFlag {
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
/// [`VkMemoryAllocateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryAllocateFlags.html)
///
#[doc(alias = "VkMemoryAllocateFlags")]
pub type MemoryAllocateFlags = FlagSet<MemoryAllocateFlag>;
/// [`VkMemoryAllocateFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryAllocateFlagsKHR.html)
///
#[doc(alias = "VkMemoryAllocateFlagsKHR")]
pub type MemoryAllocateFlagsKHR = MemoryAllocateFlags;

crate::__vkx_internal_flags! {
    /// [`VkExternalMemoryHandleTypeFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryHandleTypeFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.1 with appropriate features
    /// - Extension [`KHR_ExternalMemoryCapabilities`](Extension::KHR_ExternalMemoryCapabilities)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkExternalMemoryHandleTypeFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ExternalMemoryHandleTypeFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extension::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT")]
        #[default]
        OPAQUE_FD = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extension::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT")]
        OPAQUE_WIN32 = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extension::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT")]
        OPAQUE_WIN32_KMT = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extension::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT")]
        D3D11_TEXTURE = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extension::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT")]
        D3D11_TEXTURE_KMT = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extension::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT")]
        D3D12_HEAP = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extension::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT")]
        D3D12_RESOURCE = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ExternalMemoryDmaBuf`](Extension::EXT_ExternalMemoryDmaBuf)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT")]
        DMA_BUF_EXT = 512,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ANDROID_ExternalMemoryAndroidHardwareBuffer`](Extension::ANDROID_ExternalMemoryAndroidHardwareBuffer)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID")]
        ANDROID_HARDWARE_BUFFER_ANDROID = 1024,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ExternalMemoryHost`](Extension::EXT_ExternalMemoryHost)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT")]
        HOST_ALLOCATION_EXT = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ExternalMemoryHost`](Extension::EXT_ExternalMemoryHost)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT")]
        HOST_MAPPED_FOREIGN_MEMORY_EXT = 256,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`FUCHSIA_ExternalMemory`](Extension::FUCHSIA_ExternalMemory)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA")]
        ZIRCON_VMO_FUCHSIA = 2048,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ExternalMemoryRdma`](Extension::NV_ExternalMemoryRdma)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_RDMA_ADDRESS_BIT_NV")]
        RDMA_ADDRESS_NV = 4096,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`OHOS_ExternalMemory`](Extension::OHOS_ExternalMemory)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OH_NATIVE_BUFFER_BIT_OHOS")]
        OH_NATIVE_BUFFER_OHOS = 32768,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QNX_ExternalMemoryScreenBuffer`](Extension::QNX_ExternalMemoryScreenBuffer)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_SCREEN_BUFFER_BIT_QNX")]
        SCREEN_BUFFER_QNX = 16384,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ExternalMemoryMetal`](Extension::EXT_ExternalMemoryMetal)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_MTLBUFFER_BIT_EXT")]
        MTLBUFFER_EXT = 65536,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ExternalMemoryMetal`](Extension::EXT_ExternalMemoryMetal)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_MTLTEXTURE_BIT_EXT")]
        MTLTEXTURE_EXT = 131072,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ExternalMemoryMetal`](Extension::EXT_ExternalMemoryMetal)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_MTLHEAP_BIT_EXT")]
        MTLHEAP_EXT = 262144,
    }
}
/// [`VkExternalMemoryHandleTypeFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryHandleTypeFlagBitsKHR.html)
///
#[doc(alias = "VkExternalMemoryHandleTypeFlagBitsKHR")]
pub type ExternalMemoryHandleTypeFlagKHR = ExternalMemoryHandleTypeFlag;
impl ExternalMemoryHandleTypeFlag {
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
/// [`VkExternalMemoryHandleTypeFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryHandleTypeFlags.html)
///
#[doc(alias = "VkExternalMemoryHandleTypeFlags")]
pub type ExternalMemoryHandleTypeFlags = FlagSet<ExternalMemoryHandleTypeFlag>;
/// [`VkExternalMemoryHandleTypeFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryHandleTypeFlagsKHR.html)
///
#[doc(alias = "VkExternalMemoryHandleTypeFlagsKHR")]
pub type ExternalMemoryHandleTypeFlagsKHR = ExternalMemoryHandleTypeFlags;

crate::__vkx_internal_flags! {
    /// [`VkExternalMemoryFeatureFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryFeatureFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.1 with appropriate features
    /// - Extension [`KHR_ExternalMemoryCapabilities`](Extension::KHR_ExternalMemoryCapabilities)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkExternalMemoryFeatureFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ExternalMemoryFeatureFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extension::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT")]
        #[default]
        DEDICATED_ONLY = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extension::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT")]
        EXPORTABLE = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extension::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT")]
        IMPORTABLE = 4,
    }
}
/// [`VkExternalMemoryFeatureFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryFeatureFlagBitsKHR.html)
///
#[doc(alias = "VkExternalMemoryFeatureFlagBitsKHR")]
pub type ExternalMemoryFeatureFlagKHR = ExternalMemoryFeatureFlag;
impl ExternalMemoryFeatureFlag {
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
/// [`VkExternalMemoryFeatureFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryFeatureFlags.html)
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkExternalMemoryFeatureFlags")]
pub type ExternalMemoryFeatureFlags = FlagSet<ExternalMemoryFeatureFlag>;
/// [`VkExternalMemoryFeatureFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryFeatureFlagsKHR.html)
///
#[doc(alias = "VkExternalMemoryFeatureFlagsKHR")]
pub type ExternalMemoryFeatureFlagsKHR = ExternalMemoryFeatureFlags;

crate::__vkx_internal_flags! {
    /// [`VkExternalFenceHandleTypeFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceHandleTypeFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.1 with appropriate features
    /// - Extension [`KHR_ExternalFenceCapabilities`](Extension::KHR_ExternalFenceCapabilities)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkExternalFenceHandleTypeFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ExternalFenceHandleTypeFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalFenceCapabilities`](Extension::KHR_ExternalFenceCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT")]
        #[default]
        OPAQUE_FD = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalFenceCapabilities`](Extension::KHR_ExternalFenceCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT")]
        OPAQUE_WIN32 = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalFenceCapabilities`](Extension::KHR_ExternalFenceCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT")]
        OPAQUE_WIN32_KMT = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalFenceCapabilities`](Extension::KHR_ExternalFenceCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT")]
        SYNC_FD = 8,
    }
}
/// [`VkExternalFenceHandleTypeFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceHandleTypeFlagBitsKHR.html)
///
#[doc(alias = "VkExternalFenceHandleTypeFlagBitsKHR")]
pub type ExternalFenceHandleTypeFlagKHR = ExternalFenceHandleTypeFlag;
impl ExternalFenceHandleTypeFlag {
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
/// [`VkExternalFenceHandleTypeFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceHandleTypeFlags.html)
///
#[doc(alias = "VkExternalFenceHandleTypeFlags")]
pub type ExternalFenceHandleTypeFlags = FlagSet<ExternalFenceHandleTypeFlag>;
/// [`VkExternalFenceHandleTypeFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceHandleTypeFlagsKHR.html)
///
#[doc(alias = "VkExternalFenceHandleTypeFlagsKHR")]
pub type ExternalFenceHandleTypeFlagsKHR = ExternalFenceHandleTypeFlags;

crate::__vkx_internal_flags! {
    /// [`VkExternalFenceFeatureFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceFeatureFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.1 with appropriate features
    /// - Extension [`KHR_ExternalFenceCapabilities`](Extension::KHR_ExternalFenceCapabilities)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkExternalFenceFeatureFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ExternalFenceFeatureFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalFenceCapabilities`](Extension::KHR_ExternalFenceCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT")]
        #[default]
        EXPORTABLE = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalFenceCapabilities`](Extension::KHR_ExternalFenceCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT")]
        IMPORTABLE = 2,
    }
}
/// [`VkExternalFenceFeatureFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceFeatureFlagBitsKHR.html)
///
#[doc(alias = "VkExternalFenceFeatureFlagBitsKHR")]
pub type ExternalFenceFeatureFlagKHR = ExternalFenceFeatureFlag;
impl ExternalFenceFeatureFlag {
    /// [`VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR")]
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    /// [`VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR")]
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
}
/// [`VkExternalFenceFeatureFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceFeatureFlags.html)
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkExternalFenceFeatureFlags")]
pub type ExternalFenceFeatureFlags = FlagSet<ExternalFenceFeatureFlag>;
/// [`VkExternalFenceFeatureFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceFeatureFlagsKHR.html)
///
#[doc(alias = "VkExternalFenceFeatureFlagsKHR")]
pub type ExternalFenceFeatureFlagsKHR = ExternalFenceFeatureFlags;

crate::__vkx_internal_flags! {
    /// [`VkFenceImportFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceImportFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.1 with appropriate features
    /// - Extension [`KHR_ExternalFence`](Extension::KHR_ExternalFence)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkFenceImportFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum FenceImportFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalFence`](Extension::KHR_ExternalFence)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FENCE_IMPORT_TEMPORARY_BIT")]
        #[default]
        TEMPORARY = 1,
    }
}
/// [`VkFenceImportFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceImportFlagBitsKHR.html)
///
#[doc(alias = "VkFenceImportFlagBitsKHR")]
pub type FenceImportFlagKHR = FenceImportFlag;
impl FenceImportFlag {
    /// [`VK_FENCE_IMPORT_TEMPORARY_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FENCE_IMPORT_TEMPORARY_BIT_KHR.html)
    ///
    #[doc(alias = "VK_FENCE_IMPORT_TEMPORARY_BIT_KHR")]
    pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
}
/// [`VkFenceImportFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceImportFlags.html)
///
#[doc(alias = "VkFenceImportFlags")]
pub type FenceImportFlags = FlagSet<FenceImportFlag>;
/// [`VkFenceImportFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceImportFlagsKHR.html)
///
#[doc(alias = "VkFenceImportFlagsKHR")]
pub type FenceImportFlagsKHR = FenceImportFlags;

crate::__vkx_internal_flags! {
    /// [`VkSemaphoreImportFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreImportFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.1 with appropriate features
    /// - Extension [`KHR_ExternalSemaphore`](Extension::KHR_ExternalSemaphore)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkSemaphoreImportFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum SemaphoreImportFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalSemaphore`](Extension::KHR_ExternalSemaphore)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SEMAPHORE_IMPORT_TEMPORARY_BIT")]
        #[default]
        TEMPORARY = 1,
    }
}
/// [`VkSemaphoreImportFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreImportFlagBitsKHR.html)
///
#[doc(alias = "VkSemaphoreImportFlagBitsKHR")]
pub type SemaphoreImportFlagKHR = SemaphoreImportFlag;
impl SemaphoreImportFlag {
    /// [`VK_SEMAPHORE_IMPORT_TEMPORARY_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SEMAPHORE_IMPORT_TEMPORARY_BIT_KHR.html)
    ///
    #[doc(alias = "VK_SEMAPHORE_IMPORT_TEMPORARY_BIT_KHR")]
    pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
}
/// [`VkSemaphoreImportFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreImportFlags.html)
///
#[doc(alias = "VkSemaphoreImportFlags")]
pub type SemaphoreImportFlags = FlagSet<SemaphoreImportFlag>;
/// [`VkSemaphoreImportFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreImportFlagsKHR.html)
///
#[doc(alias = "VkSemaphoreImportFlagsKHR")]
pub type SemaphoreImportFlagsKHR = SemaphoreImportFlags;

crate::__vkx_internal_flags! {
    /// [`VkExternalSemaphoreHandleTypeFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreHandleTypeFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.1 with appropriate features
    /// - Extension [`KHR_ExternalSemaphoreCapabilities`](Extension::KHR_ExternalSemaphoreCapabilities)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkExternalSemaphoreHandleTypeFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ExternalSemaphoreHandleTypeFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalSemaphoreCapabilities`](Extension::KHR_ExternalSemaphoreCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT")]
        #[default]
        OPAQUE_FD = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalSemaphoreCapabilities`](Extension::KHR_ExternalSemaphoreCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT")]
        OPAQUE_WIN32 = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalSemaphoreCapabilities`](Extension::KHR_ExternalSemaphoreCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT")]
        OPAQUE_WIN32_KMT = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalSemaphoreCapabilities`](Extension::KHR_ExternalSemaphoreCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT")]
        D3D12_FENCE = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalSemaphoreCapabilities`](Extension::KHR_ExternalSemaphoreCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT")]
        SYNC_FD = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`FUCHSIA_ExternalSemaphore`](Extension::FUCHSIA_ExternalSemaphore)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_ZIRCON_EVENT_BIT_FUCHSIA")]
        ZIRCON_EVENT_FUCHSIA = 128,
    }
}
/// [`VkExternalSemaphoreHandleTypeFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreHandleTypeFlagBitsKHR.html)
///
#[doc(alias = "VkExternalSemaphoreHandleTypeFlagBitsKHR")]
pub type ExternalSemaphoreHandleTypeFlagKHR = ExternalSemaphoreHandleTypeFlag;
impl ExternalSemaphoreHandleTypeFlag {
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
/// [`VkExternalSemaphoreHandleTypeFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreHandleTypeFlags.html)
///
#[doc(alias = "VkExternalSemaphoreHandleTypeFlags")]
pub type ExternalSemaphoreHandleTypeFlags = FlagSet<ExternalSemaphoreHandleTypeFlag>;
/// [`VkExternalSemaphoreHandleTypeFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreHandleTypeFlagsKHR.html)
///
#[doc(alias = "VkExternalSemaphoreHandleTypeFlagsKHR")]
pub type ExternalSemaphoreHandleTypeFlagsKHR = ExternalSemaphoreHandleTypeFlags;

crate::__vkx_internal_flags! {
    /// [`VkExternalSemaphoreFeatureFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreFeatureFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.1 with appropriate features
    /// - Extension [`KHR_ExternalSemaphoreCapabilities`](Extension::KHR_ExternalSemaphoreCapabilities)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkExternalSemaphoreFeatureFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ExternalSemaphoreFeatureFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalSemaphoreCapabilities`](Extension::KHR_ExternalSemaphoreCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT")]
        #[default]
        EXPORTABLE = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_ExternalSemaphoreCapabilities`](Extension::KHR_ExternalSemaphoreCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT")]
        IMPORTABLE = 2,
    }
}
/// [`VkExternalSemaphoreFeatureFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreFeatureFlagBitsKHR.html)
///
#[doc(alias = "VkExternalSemaphoreFeatureFlagBitsKHR")]
pub type ExternalSemaphoreFeatureFlagKHR = ExternalSemaphoreFeatureFlag;
impl ExternalSemaphoreFeatureFlag {
    /// [`VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHR")]
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    /// [`VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHR.html)
    ///
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHR")]
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
}
/// [`VkExternalSemaphoreFeatureFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreFeatureFlags.html)
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkExternalSemaphoreFeatureFlags")]
pub type ExternalSemaphoreFeatureFlags = FlagSet<ExternalSemaphoreFeatureFlag>;
/// [`VkExternalSemaphoreFeatureFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreFeatureFlagsKHR.html)
///
#[doc(alias = "VkExternalSemaphoreFeatureFlagsKHR")]
pub type ExternalSemaphoreFeatureFlagsKHR = ExternalSemaphoreFeatureFlags;

crate::__vkx_internal_flags! {
    /// [`VkResolveModeFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkResolveModeFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.2 with appropriate features
    /// - Extension [`KHR_DepthStencilResolve`](Extension::KHR_DepthStencilResolve)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkResolveModeFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ResolveModeFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`KHR_DepthStencilResolve`](Extension::KHR_DepthStencilResolve)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RESOLVE_MODE_NONE")]
        #[default]
        NONE = 0,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`KHR_DepthStencilResolve`](Extension::KHR_DepthStencilResolve)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RESOLVE_MODE_SAMPLE_ZERO_BIT")]
        SAMPLE_ZERO = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`KHR_DepthStencilResolve`](Extension::KHR_DepthStencilResolve)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RESOLVE_MODE_AVERAGE_BIT")]
        AVERAGE = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`KHR_DepthStencilResolve`](Extension::KHR_DepthStencilResolve)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RESOLVE_MODE_MIN_BIT")]
        MIN = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`KHR_DepthStencilResolve`](Extension::KHR_DepthStencilResolve)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RESOLVE_MODE_MAX_BIT")]
        MAX = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ANDROID_ExternalFormatResolve`](Extension::ANDROID_ExternalFormatResolve)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RESOLVE_MODE_EXTERNAL_FORMAT_DOWNSAMPLE_BIT_ANDROID")]
        EXTERNAL_FORMAT_DOWNSAMPLE_ANDROID = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_CustomResolve`](Extension::EXT_CustomResolve)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RESOLVE_MODE_CUSTOM_BIT_EXT")]
        CUSTOM_EXT = 32,
    }
}
/// [`VkResolveModeFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkResolveModeFlagBitsKHR.html)
///
#[doc(alias = "VkResolveModeFlagBitsKHR")]
pub type ResolveModeFlagKHR = ResolveModeFlag;
impl ResolveModeFlag {
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
/// [`VkResolveModeFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkResolveModeFlags.html)
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkResolveModeFlags")]
pub type ResolveModeFlags = FlagSet<ResolveModeFlag>;
/// [`VkResolveModeFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkResolveModeFlagsKHR.html)
///
#[doc(alias = "VkResolveModeFlagsKHR")]
pub type ResolveModeFlagsKHR = ResolveModeFlags;

crate::__vkx_internal_flags! {
    /// [`VkSemaphoreWaitFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreWaitFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.2 with appropriate features
    /// - Extension [`KHR_TimelineSemaphore`](Extension::KHR_TimelineSemaphore)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkSemaphoreWaitFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum SemaphoreWaitFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`KHR_TimelineSemaphore`](Extension::KHR_TimelineSemaphore)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SEMAPHORE_WAIT_ANY_BIT")]
        #[default]
        ANY = 1,
    }
}
/// [`VkSemaphoreWaitFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreWaitFlagBitsKHR.html)
///
#[doc(alias = "VkSemaphoreWaitFlagBitsKHR")]
pub type SemaphoreWaitFlagKHR = SemaphoreWaitFlag;
impl SemaphoreWaitFlag {
    /// [`VK_SEMAPHORE_WAIT_ANY_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SEMAPHORE_WAIT_ANY_BIT_KHR.html)
    ///
    #[doc(alias = "VK_SEMAPHORE_WAIT_ANY_BIT_KHR")]
    pub const ANY_KHR: Self = Self::ANY;
}
/// [`VkSemaphoreWaitFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreWaitFlags.html)
///
#[doc(alias = "VkSemaphoreWaitFlags")]
pub type SemaphoreWaitFlags = FlagSet<SemaphoreWaitFlag>;
/// [`VkSemaphoreWaitFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreWaitFlagsKHR.html)
///
#[doc(alias = "VkSemaphoreWaitFlagsKHR")]
pub type SemaphoreWaitFlagsKHR = SemaphoreWaitFlags;

crate::__vkx_internal_flags! {
    /// [`VkDescriptorBindingFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorBindingFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.2 with appropriate features
    /// - Extension [`EXT_DescriptorIndexing`](Extension::EXT_DescriptorIndexing)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDescriptorBindingFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum DescriptorBindingFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`EXT_DescriptorIndexing`](Extension::EXT_DescriptorIndexing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT")]
        #[default]
        UPDATE_AFTER_BIND = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`EXT_DescriptorIndexing`](Extension::EXT_DescriptorIndexing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT")]
        UPDATE_UNUSED_WHILE_PENDING = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`EXT_DescriptorIndexing`](Extension::EXT_DescriptorIndexing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT")]
        PARTIALLY_BOUND = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.2 with appropriate features
        /// - Extension [`EXT_DescriptorIndexing`](Extension::EXT_DescriptorIndexing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT")]
        VARIABLE_DESCRIPTOR_COUNT = 8,
    }
}
/// [`VkDescriptorBindingFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorBindingFlagBitsEXT.html)
///
#[doc(alias = "VkDescriptorBindingFlagBitsEXT")]
pub type DescriptorBindingFlagEXT = DescriptorBindingFlag;
impl DescriptorBindingFlag {
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
/// [`VkDescriptorBindingFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorBindingFlags.html)
///
#[doc(alias = "VkDescriptorBindingFlags")]
pub type DescriptorBindingFlags = FlagSet<DescriptorBindingFlag>;
/// [`VkDescriptorBindingFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorBindingFlagsEXT.html)
///
#[doc(alias = "VkDescriptorBindingFlagsEXT")]
pub type DescriptorBindingFlagsEXT = DescriptorBindingFlags;

crate::__vkx_internal_flags! {
    /// [`VkToolPurposeFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkToolPurposeFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.3 with appropriate features
    /// - Extension [`EXT_ToolingInfo`](Extension::EXT_ToolingInfo)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkToolPurposeFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ToolPurposeFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_ToolingInfo`](Extension::EXT_ToolingInfo)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TOOL_PURPOSE_VALIDATION_BIT")]
        #[default]
        VALIDATION = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_ToolingInfo`](Extension::EXT_ToolingInfo)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TOOL_PURPOSE_PROFILING_BIT")]
        PROFILING = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_ToolingInfo`](Extension::EXT_ToolingInfo)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TOOL_PURPOSE_TRACING_BIT")]
        TRACING = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_ToolingInfo`](Extension::EXT_ToolingInfo)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT")]
        ADDITIONAL_FEATURES = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_ToolingInfo`](Extension::EXT_ToolingInfo)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT")]
        MODIFYING_FEATURES = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_ToolingInfo`](Extension::EXT_ToolingInfo)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TOOL_PURPOSE_DEBUG_REPORTING_BIT_EXT")]
        DEBUG_REPORTING_EXT = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_ToolingInfo`](Extension::EXT_ToolingInfo)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TOOL_PURPOSE_DEBUG_MARKERS_BIT_EXT")]
        DEBUG_MARKERS_EXT = 64,
    }
}
/// [`VkToolPurposeFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkToolPurposeFlagBitsEXT.html)
///
#[doc(alias = "VkToolPurposeFlagBitsEXT")]
pub type ToolPurposeFlagEXT = ToolPurposeFlag;
impl ToolPurposeFlag {
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
/// [`VkToolPurposeFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkToolPurposeFlags.html)
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkToolPurposeFlags")]
pub type ToolPurposeFlags = FlagSet<ToolPurposeFlag>;
/// [`VkToolPurposeFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkToolPurposeFlagsEXT.html)
///
#[doc(alias = "VkToolPurposeFlagsEXT")]
pub type ToolPurposeFlagsEXT = ToolPurposeFlags;

crate::__vkx_internal_flags! {
    /// [`VkPrivateDataSlotCreateFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlotCreateFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.3 with appropriate features
    /// - Extension [`EXT_PrivateData`](Extension::EXT_PrivateData)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkPrivateDataSlotCreateFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum PrivateDataSlotCreateFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_PrivateDataBaseHandle`](Extension::NV_PrivateDataBaseHandle)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRIVATE_DATA_SLOT_CREATE_BASE_OBJECT_HANDLE_BIT_NV")]
        #[default]
        BASE_OBJECT_HANDLE_NV = 1,
    }
}
/// [`VkPrivateDataSlotCreateFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlotCreateFlagBitsEXT.html)
///
#[doc(alias = "VkPrivateDataSlotCreateFlagBitsEXT")]
pub type PrivateDataSlotCreateFlagEXT = PrivateDataSlotCreateFlag;
/// [`VkPrivateDataSlotCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlotCreateFlags.html)
///
#[doc(alias = "VkPrivateDataSlotCreateFlags")]
pub type PrivateDataSlotCreateFlags = FlagSet<PrivateDataSlotCreateFlag>;
/// [`VkPrivateDataSlotCreateFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlotCreateFlagsEXT.html)
///
#[doc(alias = "VkPrivateDataSlotCreateFlagsEXT")]
pub type PrivateDataSlotCreateFlagsEXT = PrivateDataSlotCreateFlags;

crate::__vkx_internal_flags! {
    /// [`VkPipelineStageFlagBits2`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineStageFlagBits2.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.3 with appropriate features
    /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkPipelineStageFlagBits2")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum PipelineStageFlag2: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_NONE")]
        #[default]
        NONE = 0,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT")]
        TOP_OF_PIPE = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT")]
        DRAW_INDIRECT = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT")]
        VERTEX_INPUT = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT")]
        VERTEX_SHADER = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT")]
        TESSELLATION_CONTROL_SHADER = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT")]
        TESSELLATION_EVALUATION_SHADER = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT")]
        GEOMETRY_SHADER = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT")]
        FRAGMENT_SHADER = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT")]
        EARLY_FRAGMENT_TESTS = 256,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT")]
        LATE_FRAGMENT_TESTS = 512,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT")]
        COLOR_ATTACHMENT_OUTPUT = 1024,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT")]
        COMPUTE_SHADER = 2048,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT")]
        ALL_TRANSFER = 4096,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT")]
        BOTTOM_OF_PIPE = 8192,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_HOST_BIT")]
        HOST = 16384,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT")]
        ALL_GRAPHICS = 32768,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT")]
        ALL_COMMANDS = 65536,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_COPY_BIT")]
        COPY = 4294967296,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_RESOLVE_BIT")]
        RESOLVE = 8589934592,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_BLIT_BIT")]
        BLIT = 17179869184,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_CLEAR_BIT")]
        CLEAR = 34359738368,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT")]
        INDEX_INPUT = 68719476736,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT")]
        VERTEX_ATTRIBUTE_INPUT = 137438953472,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT")]
        PRE_RASTERIZATION_SHADERS = 274877906944,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extension::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR")]
        VIDEO_DECODE_KHR = 67108864,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR")]
        VIDEO_ENCODE_KHR = 134217728,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT")]
        TRANSFORM_FEEDBACK_EXT = 16777216,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT")]
        CONDITIONAL_RENDERING_EXT = 262144,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_EXT")]
        COMMAND_PREPROCESS_EXT = 131072,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
        FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = 4194304,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR")]
        ACCELERATION_STRUCTURE_BUILD_KHR = 33554432,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR")]
        RAY_TRACING_SHADER_KHR = 2097152,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT")]
        FRAGMENT_DENSITY_PROCESS_EXT = 8388608,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_EXT")]
        TASK_SHADER_EXT = 524288,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_EXT")]
        MESH_SHADER_EXT = 1048576,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`HUAWEI_SubpassShading`](Extension::HUAWEI_SubpassShading)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_SUBPASS_SHADER_BIT_HUAWEI")]
        SUBPASS_SHADER_HUAWEI = 549755813888,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`HUAWEI_InvocationMask`](Extension::HUAWEI_InvocationMask)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI")]
        INVOCATION_MASK_HUAWEI = 1099511627776,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_RayTracingMaintenance1`](Extension::KHR_RayTracingMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_COPY_BIT_KHR")]
        ACCELERATION_STRUCTURE_COPY_KHR = 268435456,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_OpacityMicromap`](Extension::EXT_OpacityMicromap)
        /// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_MICROMAP_BUILD_BIT_EXT")]
        MICROMAP_BUILD_EXT = 1073741824,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`HUAWEI_ClusterCullingShader`](Extension::HUAWEI_ClusterCullingShader)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_CLUSTER_CULLING_SHADER_BIT_HUAWEI")]
        CLUSTER_CULLING_SHADER_HUAWEI = 2199023255552,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_OPTICAL_FLOW_BIT_NV")]
        OPTICAL_FLOW_NV = 536870912,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_CooperativeVector`](Extension::NV_CooperativeVector)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_CONVERT_COOPERATIVE_VECTOR_MATRIX_BIT_NV")]
        CONVERT_COOPERATIVE_VECTOR_MATRIX_NV = 17592186044416,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraph`](Extension::ARM_DataGraph)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_DATA_GRAPH_BIT_ARM")]
        DATA_GRAPH_ARM = 4398046511104,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_CopyMemoryIndirect`](Extension::KHR_CopyMemoryIndirect)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_COPY_INDIRECT_BIT_KHR")]
        COPY_INDIRECT_KHR = 70368744177664,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MemoryDecompression`](Extension::EXT_MemoryDecompression)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_STAGE_2_MEMORY_DECOMPRESSION_BIT_EXT")]
        MEMORY_DECOMPRESSION_EXT = 35184372088832,
    }
}
/// [`VkPipelineStageFlagBits2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineStageFlagBits2KHR.html)
///
#[doc(alias = "VkPipelineStageFlagBits2KHR")]
pub type PipelineStageFlag2KHR = PipelineStageFlag2;
impl PipelineStageFlag2 {
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
/// [`VkPipelineStageFlags2`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineStageFlags2.html)
///
#[doc(alias = "VkPipelineStageFlags2")]
pub type PipelineStageFlags2 = FlagSet<PipelineStageFlag2>;
/// [`VkPipelineStageFlags2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineStageFlags2KHR.html)
///
#[doc(alias = "VkPipelineStageFlags2KHR")]
pub type PipelineStageFlags2KHR = PipelineStageFlags2;

crate::__vkx_internal_flags! {
    /// [`VkAccessFlagBits2`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlagBits2.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.3 with appropriate features
    /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkAccessFlagBits2")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum AccessFlag2: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_NONE")]
        #[default]
        NONE = 0,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT")]
        INDIRECT_COMMAND_READ = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_INDEX_READ_BIT")]
        INDEX_READ = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT")]
        VERTEX_ATTRIBUTE_READ = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_UNIFORM_READ_BIT")]
        UNIFORM_READ = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT")]
        INPUT_ATTACHMENT_READ = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_SHADER_READ_BIT")]
        SHADER_READ = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_SHADER_WRITE_BIT")]
        SHADER_WRITE = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT")]
        COLOR_ATTACHMENT_READ = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT")]
        COLOR_ATTACHMENT_WRITE = 256,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT")]
        DEPTH_STENCIL_ATTACHMENT_READ = 512,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT")]
        DEPTH_STENCIL_ATTACHMENT_WRITE = 1024,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_TRANSFER_READ_BIT")]
        TRANSFER_READ = 2048,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_TRANSFER_WRITE_BIT")]
        TRANSFER_WRITE = 4096,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_HOST_READ_BIT")]
        HOST_READ = 8192,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_HOST_WRITE_BIT")]
        HOST_WRITE = 16384,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_MEMORY_READ_BIT")]
        MEMORY_READ = 32768,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_MEMORY_WRITE_BIT")]
        MEMORY_WRITE = 65536,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_SHADER_SAMPLED_READ_BIT")]
        SHADER_SAMPLED_READ = 4294967296,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_SHADER_STORAGE_READ_BIT")]
        SHADER_STORAGE_READ = 8589934592,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT")]
        SHADER_STORAGE_WRITE = 17179869184,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extension::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_VIDEO_DECODE_READ_BIT_KHR")]
        VIDEO_DECODE_READ_KHR = 34359738368,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extension::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_VIDEO_DECODE_WRITE_BIT_KHR")]
        VIDEO_DECODE_WRITE_KHR = 68719476736,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_SAMPLER_HEAP_READ_BIT_EXT")]
        SAMPLER_HEAP_READ_EXT = 144115188075855872,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_RESOURCE_HEAP_READ_BIT_EXT")]
        RESOURCE_HEAP_READ_EXT = 288230376151711744,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_VIDEO_ENCODE_READ_BIT_KHR")]
        VIDEO_ENCODE_READ_KHR = 137438953472,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_VIDEO_ENCODE_WRITE_BIT_KHR")]
        VIDEO_ENCODE_WRITE_KHR = 274877906944,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_TileShading`](Extension::QCOM_TileShading)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_SHADER_TILE_ATTACHMENT_READ_BIT_QCOM")]
        SHADER_TILE_ATTACHMENT_READ_QCOM = 2251799813685248,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_TileShading`](Extension::QCOM_TileShading)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_SHADER_TILE_ATTACHMENT_WRITE_BIT_QCOM")]
        SHADER_TILE_ATTACHMENT_WRITE_QCOM = 4503599627370496,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_TRANSFORM_FEEDBACK_WRITE_BIT_EXT")]
        TRANSFORM_FEEDBACK_WRITE_EXT = 33554432,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT")]
        TRANSFORM_FEEDBACK_COUNTER_READ_EXT = 67108864,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT")]
        TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT = 134217728,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_CONDITIONAL_RENDERING_READ_BIT_EXT")]
        CONDITIONAL_RENDERING_READ_EXT = 1048576,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_COMMAND_PREPROCESS_READ_BIT_EXT")]
        COMMAND_PREPROCESS_READ_EXT = 131072,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_COMMAND_PREPROCESS_WRITE_BIT_EXT")]
        COMMAND_PREPROCESS_WRITE_EXT = 262144,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR")]
        FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR = 8388608,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR")]
        ACCELERATION_STRUCTURE_READ_KHR = 2097152,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_KHR")]
        ACCELERATION_STRUCTURE_WRITE_KHR = 4194304,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_FRAGMENT_DENSITY_MAP_READ_BIT_EXT")]
        FRAGMENT_DENSITY_MAP_READ_EXT = 16777216,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT")]
        COLOR_ATTACHMENT_READ_NONCOHERENT_EXT = 524288,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorBuffer`](Extension::EXT_DescriptorBuffer)
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_DESCRIPTOR_BUFFER_READ_BIT_EXT")]
        DESCRIPTOR_BUFFER_READ_EXT = 2199023255552,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`HUAWEI_InvocationMask`](Extension::HUAWEI_InvocationMask)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_INVOCATION_MASK_READ_BIT_HUAWEI")]
        INVOCATION_MASK_READ_HUAWEI = 549755813888,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_RayTracingMaintenance1`](Extension::KHR_RayTracingMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_SHADER_BINDING_TABLE_READ_BIT_KHR")]
        SHADER_BINDING_TABLE_READ_KHR = 1099511627776,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_OpacityMicromap`](Extension::EXT_OpacityMicromap)
        /// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_MICROMAP_READ_BIT_EXT")]
        MICROMAP_READ_EXT = 17592186044416,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_OpacityMicromap`](Extension::EXT_OpacityMicromap)
        /// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_MICROMAP_WRITE_BIT_EXT")]
        MICROMAP_WRITE_EXT = 35184372088832,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_OPTICAL_FLOW_READ_BIT_NV")]
        OPTICAL_FLOW_READ_NV = 4398046511104,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_OPTICAL_FLOW_WRITE_BIT_NV")]
        OPTICAL_FLOW_WRITE_NV = 8796093022208,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraph`](Extension::ARM_DataGraph)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_DATA_GRAPH_READ_BIT_ARM")]
        DATA_GRAPH_READ_ARM = 140737488355328,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraph`](Extension::ARM_DataGraph)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_DATA_GRAPH_WRITE_BIT_ARM")]
        DATA_GRAPH_WRITE_ARM = 281474976710656,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MemoryDecompression`](Extension::EXT_MemoryDecompression)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_MEMORY_DECOMPRESSION_READ_BIT_EXT")]
        MEMORY_DECOMPRESSION_READ_EXT = 36028797018963968,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MemoryDecompression`](Extension::EXT_MemoryDecompression)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_2_MEMORY_DECOMPRESSION_WRITE_BIT_EXT")]
        MEMORY_DECOMPRESSION_WRITE_EXT = 72057594037927936,
    }
}
/// [`VkAccessFlagBits2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlagBits2KHR.html)
///
#[doc(alias = "VkAccessFlagBits2KHR")]
pub type AccessFlag2KHR = AccessFlag2;
impl AccessFlag2 {
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
/// [`VkAccessFlags2`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlags2.html)
///
#[doc(alias = "VkAccessFlags2")]
pub type AccessFlags2 = FlagSet<AccessFlag2>;
/// [`VkAccessFlags2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlags2KHR.html)
///
#[doc(alias = "VkAccessFlags2KHR")]
pub type AccessFlags2KHR = AccessFlags2;

crate::__vkx_internal_flags! {
    /// [`VkSubmitFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubmitFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.3 with appropriate features
    /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkSubmitFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum SubmitFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_Synchronization2`](Extension::KHR_Synchronization2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SUBMIT_PROTECTED_BIT")]
        #[default]
        PROTECTED = 1,
    }
}
/// [`VkSubmitFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubmitFlagBitsKHR.html)
///
#[doc(alias = "VkSubmitFlagBitsKHR")]
pub type SubmitFlagKHR = SubmitFlag;
impl SubmitFlag {
    /// [`VK_SUBMIT_PROTECTED_BIT_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SUBMIT_PROTECTED_BIT_KHR.html)
    ///
    #[doc(alias = "VK_SUBMIT_PROTECTED_BIT_KHR")]
    pub const PROTECTED_KHR: Self = Self::PROTECTED;
}
/// [`VkSubmitFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubmitFlags.html)
///
#[doc(alias = "VkSubmitFlags")]
pub type SubmitFlags = FlagSet<SubmitFlag>;
/// [`VkSubmitFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubmitFlagsKHR.html)
///
#[doc(alias = "VkSubmitFlagsKHR")]
pub type SubmitFlagsKHR = SubmitFlags;

crate::__vkx_internal_flags! {
    /// [`VkFormatFeatureFlagBits2`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlagBits2.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.3 with appropriate features
    /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkFormatFeatureFlagBits2")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum FormatFeatureFlag2: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT")]
        #[default]
        SAMPLED_IMAGE = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT")]
        STORAGE_IMAGE = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT")]
        STORAGE_IMAGE_ATOMIC = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT")]
        UNIFORM_TEXEL_BUFFER = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT")]
        STORAGE_TEXEL_BUFFER = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT")]
        STORAGE_TEXEL_BUFFER_ATOMIC = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT")]
        VERTEX_BUFFER = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT")]
        COLOR_ATTACHMENT = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT")]
        COLOR_ATTACHMENT_BLEND = 256,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT")]
        DEPTH_STENCIL_ATTACHMENT = 512,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_BLIT_SRC_BIT")]
        BLIT_SRC = 1024,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_BLIT_DST_BIT")]
        BLIT_DST = 2048,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT")]
        SAMPLED_IMAGE_FILTER_LINEAR = 4096,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT")]
        TRANSFER_SRC = 16384,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT")]
        TRANSFER_DST = 32768,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT")]
        SAMPLED_IMAGE_FILTER_MINMAX = 65536,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT")]
        MIDPOINT_CHROMA_SAMPLES = 131072,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT")]
        SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER = 262144,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT")]
        SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER = 524288,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT")]
        SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT = 1048576,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT")]
        SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE = 2097152,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_DISJOINT_BIT")]
        DISJOINT = 4194304,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT")]
        COSITED_CHROMA_SAMPLES = 8388608,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT")]
        STORAGE_READ_WITHOUT_FORMAT = 2147483648,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT")]
        STORAGE_WRITE_WITHOUT_FORMAT = 4294967296,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT")]
        SAMPLED_IMAGE_DEPTH_COMPARISON = 8589934592,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_FormatFeatureFlags2`](Extension::KHR_FormatFeatureFlags2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT")]
        SAMPLED_IMAGE_FILTER_CUBIC = 8192,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`EXT_HostImageCopy`](Extension::EXT_HostImageCopy)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_HOST_IMAGE_TRANSFER_BIT")]
        HOST_IMAGE_TRANSFER = 70368744177664,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extension::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_DECODE_OUTPUT_BIT_KHR")]
        VIDEO_DECODE_OUTPUT_KHR = 33554432,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extension::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_DECODE_DPB_BIT_KHR")]
        VIDEO_DECODE_DPB_KHR = 67108864,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR")]
        ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR = 536870912,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_FragmentDensityMap`](Extension::EXT_FragmentDensityMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_FRAGMENT_DENSITY_MAP_BIT_EXT")]
        FRAGMENT_DENSITY_MAP_EXT = 16777216,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_FragmentShadingRate`](Extension::KHR_FragmentShadingRate)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
        FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = 1073741824,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_ENCODE_INPUT_BIT_KHR")]
        VIDEO_ENCODE_INPUT_KHR = 134217728,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_ENCODE_DPB_BIT_KHR")]
        VIDEO_ENCODE_DPB_KHR = 268435456,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_ImageProcessing3`](Extension::QCOM_ImageProcessing3)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_BLOCK_MATCHING_SXD_BIT_QCOM")]
        BLOCK_MATCHING_SXD_QCOM = 17592186044416,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracingLinearSweptSpheres`](Extension::NV_RayTracingLinearSweptSpheres)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_ACCELERATION_STRUCTURE_RADIUS_BUFFER_BIT_NV")]
        ACCELERATION_STRUCTURE_RADIUS_BUFFER_NV = 2251799813685248,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_LinearColorAttachment`](Extension::NV_LinearColorAttachment)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV")]
        LINEAR_COLOR_ATTACHMENT_NV = 274877906944,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_ImageProcessing`](Extension::QCOM_ImageProcessing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_WEIGHT_IMAGE_BIT_QCOM")]
        WEIGHT_IMAGE_QCOM = 17179869184,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_ImageProcessing`](Extension::QCOM_ImageProcessing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_WEIGHT_SAMPLED_IMAGE_BIT_QCOM")]
        WEIGHT_SAMPLED_IMAGE_QCOM = 34359738368,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_ImageProcessing`](Extension::QCOM_ImageProcessing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_BLOCK_MATCHING_BIT_QCOM")]
        BLOCK_MATCHING_QCOM = 68719476736,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_ImageProcessing`](Extension::QCOM_ImageProcessing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_BOX_FILTER_SAMPLED_BIT_QCOM")]
        BOX_FILTER_SAMPLED_QCOM = 137438953472,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_Tensors`](Extension::ARM_Tensors)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_TENSOR_SHADER_BIT_ARM")]
        TENSOR_SHADER_ARM = 549755813888,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_Tensors`](Extension::ARM_Tensors)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_TENSOR_IMAGE_ALIASING_BIT_ARM")]
        TENSOR_IMAGE_ALIASING_ARM = 8796093022208,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_OPTICAL_FLOW_IMAGE_BIT_NV")]
        OPTICAL_FLOW_IMAGE_NV = 1099511627776,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_OPTICAL_FLOW_VECTOR_BIT_NV")]
        OPTICAL_FLOW_VECTOR_NV = 2199023255552,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_OPTICAL_FLOW_COST_BIT_NV")]
        OPTICAL_FLOW_COST_NV = 4398046511104,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraph`](Extension::ARM_DataGraph)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_TENSOR_DATA_GRAPH_BIT_ARM")]
        TENSOR_DATA_GRAPH_ARM = 281474976710656,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_CopyMemoryIndirect`](Extension::KHR_CopyMemoryIndirect)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_COPY_IMAGE_INDIRECT_DST_BIT_KHR")]
        COPY_IMAGE_INDIRECT_DST_KHR = 576460752303423488,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extension::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR")]
        VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR = 562949953421312,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extension::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR")]
        VIDEO_ENCODE_EMPHASIS_MAP_KHR = 1125899906842624,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`IMG_FilterLinear2D`](Extension::IMG_FilterLinear2D)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_2D_BIT_IMG")]
        SAMPLED_IMAGE_FILTER_LINEAR_2D_IMG = 35184372088832,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance10`](Extension::KHR_Maintenance10)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_DEPTH_COPY_ON_COMPUTE_QUEUE_BIT_KHR")]
        DEPTH_COPY_ON_COMPUTE_QUEUE_KHR = 4503599627370496,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance10`](Extension::KHR_Maintenance10)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_DEPTH_COPY_ON_TRANSFER_QUEUE_BIT_KHR")]
        DEPTH_COPY_ON_TRANSFER_QUEUE_KHR = 9007199254740992,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance10`](Extension::KHR_Maintenance10)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_STENCIL_COPY_ON_COMPUTE_QUEUE_BIT_KHR")]
        STENCIL_COPY_ON_COMPUTE_QUEUE_KHR = 18014398509481984,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance10`](Extension::KHR_Maintenance10)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_STENCIL_COPY_ON_TRANSFER_QUEUE_BIT_KHR")]
        STENCIL_COPY_ON_TRANSFER_QUEUE_KHR = 36028797018963968,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_DATA_GRAPH_OPTICAL_FLOW_IMAGE_BIT_ARM")]
        DATA_GRAPH_OPTICAL_FLOW_IMAGE_ARM = 72057594037927936,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_DATA_GRAPH_OPTICAL_FLOW_VECTOR_BIT_ARM")]
        DATA_GRAPH_OPTICAL_FLOW_VECTOR_ARM = 144115188075855872,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FORMAT_FEATURE_2_DATA_GRAPH_OPTICAL_FLOW_COST_BIT_ARM")]
        DATA_GRAPH_OPTICAL_FLOW_COST_ARM = 288230376151711744,
    }
}
/// [`VkFormatFeatureFlagBits2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlagBits2KHR.html)
///
#[doc(alias = "VkFormatFeatureFlagBits2KHR")]
pub type FormatFeatureFlag2KHR = FormatFeatureFlag2;
impl FormatFeatureFlag2 {
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
/// [`VkFormatFeatureFlags2`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlags2.html)
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkFormatFeatureFlags2")]
pub type FormatFeatureFlags2 = FlagSet<FormatFeatureFlag2>;
/// [`VkFormatFeatureFlags2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlags2KHR.html)
///
#[doc(alias = "VkFormatFeatureFlags2KHR")]
pub type FormatFeatureFlags2KHR = FormatFeatureFlags2;

crate::__vkx_internal_flags! {
    /// [`VkPipelineCreationFeedbackFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreationFeedbackFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.3 with appropriate features
    /// - Extension [`EXT_PipelineCreationFeedback`](Extension::EXT_PipelineCreationFeedback)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkPipelineCreationFeedbackFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum PipelineCreationFeedbackFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_PipelineCreationFeedback`](Extension::EXT_PipelineCreationFeedback)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT")]
        #[default]
        VALID = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_PipelineCreationFeedback`](Extension::EXT_PipelineCreationFeedback)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT")]
        APPLICATION_PIPELINE_CACHE_HIT = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`EXT_PipelineCreationFeedback`](Extension::EXT_PipelineCreationFeedback)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT")]
        BASE_PIPELINE_ACCELERATION = 4,
    }
}
/// [`VkPipelineCreationFeedbackFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreationFeedbackFlagBitsEXT.html)
///
#[doc(alias = "VkPipelineCreationFeedbackFlagBitsEXT")]
pub type PipelineCreationFeedbackFlagEXT = PipelineCreationFeedbackFlag;
impl PipelineCreationFeedbackFlag {
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
/// [`VkPipelineCreationFeedbackFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreationFeedbackFlags.html)
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkPipelineCreationFeedbackFlags")]
pub type PipelineCreationFeedbackFlags = FlagSet<PipelineCreationFeedbackFlag>;
/// [`VkPipelineCreationFeedbackFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreationFeedbackFlagsEXT.html)
///
#[doc(alias = "VkPipelineCreationFeedbackFlagsEXT")]
pub type PipelineCreationFeedbackFlagsEXT = PipelineCreationFeedbackFlags;

crate::__vkx_internal_flags! {
    /// [`VkRenderingFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.3 with appropriate features
    /// - Extension [`KHR_DynamicRendering`](Extension::KHR_DynamicRendering)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkRenderingFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum RenderingFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_DynamicRendering`](Extension::KHR_DynamicRendering)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT")]
        #[default]
        CONTENTS_SECONDARY_COMMAND_BUFFERS = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_DynamicRendering`](Extension::KHR_DynamicRendering)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDERING_SUSPENDING_BIT")]
        SUSPENDING = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.3 with appropriate features
        /// - Extension [`KHR_DynamicRendering`](Extension::KHR_DynamicRendering)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDERING_RESUMING_BIT")]
        RESUMING = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_LegacyDithering`](Extension::EXT_LegacyDithering)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDERING_ENABLE_LEGACY_DITHERING_BIT_EXT")]
        ENABLE_LEGACY_DITHERING_EXT = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance7`](Extension::KHR_Maintenance7)
        /// - Extension [`EXT_NestedCommandBuffer`](Extension::EXT_NestedCommandBuffer)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDERING_CONTENTS_INLINE_BIT_KHR")]
        CONTENTS_INLINE_KHR = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_FragmentDensityMapLayered`](Extension::VALVE_FragmentDensityMapLayered)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDERING_PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE")]
        PER_LAYER_FRAGMENT_DENSITY_VALVE = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_CustomResolve`](Extension::EXT_CustomResolve)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDERING_FRAGMENT_REGION_BIT_EXT")]
        FRAGMENT_REGION_EXT = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_CustomResolve`](Extension::EXT_CustomResolve)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDERING_CUSTOM_RESOLVE_BIT_EXT")]
        CUSTOM_RESOLVE_EXT = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance10`](Extension::KHR_Maintenance10)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDERING_LOCAL_READ_CONCURRENT_ACCESS_CONTROL_BIT_KHR")]
        LOCAL_READ_CONCURRENT_ACCESS_CONTROL_KHR = 256,
    }
}
/// [`VkRenderingFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingFlagBitsKHR.html)
///
#[doc(alias = "VkRenderingFlagBitsKHR")]
pub type RenderingFlagKHR = RenderingFlag;
impl RenderingFlag {
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
/// [`VkRenderingFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingFlags.html)
///
#[doc(alias = "VkRenderingFlags")]
pub type RenderingFlags = FlagSet<RenderingFlag>;
/// [`VkRenderingFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingFlagsKHR.html)
///
#[doc(alias = "VkRenderingFlagsKHR")]
pub type RenderingFlagsKHR = RenderingFlags;

crate::__vkx_internal_flags! {
    /// [`VkMemoryUnmapFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryUnmapFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.4 with appropriate features
    /// - Extension [`KHR_MapMemory2`](Extension::KHR_MapMemory2)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkMemoryUnmapFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum MemoryUnmapFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MapMemoryPlaced`](Extension::EXT_MapMemoryPlaced)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MEMORY_UNMAP_RESERVE_BIT_EXT")]
        #[default]
        RESERVE_EXT = 1,
    }
}
/// [`VkMemoryUnmapFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryUnmapFlagBitsKHR.html)
///
#[doc(alias = "VkMemoryUnmapFlagBitsKHR")]
pub type MemoryUnmapFlagKHR = MemoryUnmapFlag;
/// [`VkMemoryUnmapFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryUnmapFlags.html)
///
#[doc(alias = "VkMemoryUnmapFlags")]
pub type MemoryUnmapFlags = FlagSet<MemoryUnmapFlag>;
/// [`VkMemoryUnmapFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryUnmapFlagsKHR.html)
///
#[doc(alias = "VkMemoryUnmapFlagsKHR")]
pub type MemoryUnmapFlagsKHR = MemoryUnmapFlags;

crate::__vkx_internal_flags! {
    /// [`VkBufferUsageFlagBits2`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlagBits2.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.4 with appropriate features
    /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
    /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkBufferUsageFlagBits2")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum BufferUsageFlag2: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_TRANSFER_SRC_BIT")]
        #[default]
        TRANSFER_SRC = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_TRANSFER_DST_BIT")]
        TRANSFER_DST = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_UNIFORM_TEXEL_BUFFER_BIT")]
        UNIFORM_TEXEL_BUFFER = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_STORAGE_TEXEL_BUFFER_BIT")]
        STORAGE_TEXEL_BUFFER = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_UNIFORM_BUFFER_BIT")]
        UNIFORM_BUFFER = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_STORAGE_BUFFER_BIT")]
        STORAGE_BUFFER = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_INDEX_BUFFER_BIT")]
        INDEX_BUFFER = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_VERTEX_BUFFER_BIT")]
        VERTEX_BUFFER = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_INDIRECT_BUFFER_BIT")]
        INDIRECT_BUFFER = 256,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_SHADER_DEVICE_ADDRESS_BIT")]
        SHADER_DEVICE_ADDRESS = 131072,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMDX_ShaderEnqueue`](Extension::AMDX_ShaderEnqueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_EXECUTION_GRAPH_SCRATCH_BIT_AMDX")]
        EXECUTION_GRAPH_SCRATCH_AMDX = 33554432,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_DESCRIPTOR_HEAP_BIT_EXT")]
        DESCRIPTOR_HEAP_EXT = 268435456,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        /// - Extension [`EXT_OpacityMicromap`](Extension::EXT_OpacityMicromap)
        /// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_MICROMAP_BUILD_INPUT_READ_ONLY_BIT_EXT")]
        MICROMAP_BUILD_INPUT_READ_ONLY_EXT = 8388608,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        /// - Extension [`EXT_OpacityMicromap`](Extension::EXT_OpacityMicromap)
        /// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_MICROMAP_STORAGE_BIT_EXT")]
        MICROMAP_STORAGE_EXT = 16777216,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_CONDITIONAL_RENDERING_BIT_EXT")]
        CONDITIONAL_RENDERING_EXT = 512,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_SHADER_BINDING_TABLE_BIT_KHR")]
        SHADER_BINDING_TABLE_KHR = 1024,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_TRANSFORM_FEEDBACK_BUFFER_BIT_EXT")]
        TRANSFORM_FEEDBACK_BUFFER_EXT = 2048,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT")]
        TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT = 4096,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_VIDEO_DECODE_SRC_BIT_KHR")]
        VIDEO_DECODE_SRC_KHR = 8192,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_VIDEO_DECODE_DST_BIT_KHR")]
        VIDEO_DECODE_DST_KHR = 16384,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_VIDEO_ENCODE_DST_BIT_KHR")]
        VIDEO_ENCODE_DST_KHR = 32768,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_VIDEO_ENCODE_SRC_BIT_KHR")]
        VIDEO_ENCODE_SRC_KHR = 65536,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR")]
        ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR = 524288,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_ACCELERATION_STRUCTURE_STORAGE_BIT_KHR")]
        ACCELERATION_STRUCTURE_STORAGE_KHR = 1048576,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_SAMPLER_DESCRIPTOR_BUFFER_BIT_EXT")]
        SAMPLER_DESCRIPTOR_BUFFER_EXT = 2097152,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_RESOURCE_DESCRIPTOR_BUFFER_BIT_EXT")]
        RESOURCE_DESCRIPTOR_BUFFER_EXT = 4194304,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_BIT_EXT")]
        PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT = 67108864,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMDX_DenseGeometryFormat`](Extension::AMDX_DenseGeometryFormat)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_COMPRESSED_DATA_DGF1_BIT_AMDX")]
        COMPRESSED_DATA_DGF1_AMDX = 8589934592,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraph`](Extension::ARM_DataGraph)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_DATA_GRAPH_FOREIGN_DESCRIPTOR_BIT_ARM")]
        DATA_GRAPH_FOREIGN_DESCRIPTOR_ARM = 536870912,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_TileMemoryHeap`](Extension::QCOM_TileMemoryHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_TILE_MEMORY_BIT_QCOM")]
        TILE_MEMORY_QCOM = 134217728,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MemoryDecompression`](Extension::EXT_MemoryDecompression)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_MEMORY_DECOMPRESSION_BIT_EXT")]
        MEMORY_DECOMPRESSION_EXT = 4294967296,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DeviceGeneratedCommands`](Extension::EXT_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUFFER_USAGE_2_PREPROCESS_BUFFER_BIT_EXT")]
        PREPROCESS_BUFFER_EXT = 2147483648,
    }
}
/// [`VkBufferUsageFlagBits2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlagBits2KHR.html)
///
#[doc(alias = "VkBufferUsageFlagBits2KHR")]
pub type BufferUsageFlag2KHR = BufferUsageFlag2;
impl BufferUsageFlag2 {
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
/// [`VkBufferUsageFlags2`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlags2.html)
///
#[doc(alias = "VkBufferUsageFlags2")]
pub type BufferUsageFlags2 = FlagSet<BufferUsageFlag2>;
/// [`VkBufferUsageFlags2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlags2KHR.html)
///
#[doc(alias = "VkBufferUsageFlags2KHR")]
pub type BufferUsageFlags2KHR = BufferUsageFlags2;

crate::__vkx_internal_flags! {
    /// [`VkHostImageCopyFlagBits`](https://docs.vulkan.org/refpages/latest/refpages/source/VkHostImageCopyFlagBits.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.4 with appropriate features
    /// - Extension [`EXT_HostImageCopy`](Extension::EXT_HostImageCopy)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkHostImageCopyFlagBits")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum HostImageCopyFlag: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`EXT_HostImageCopy`](Extension::EXT_HostImageCopy)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_HOST_IMAGE_COPY_MEMCPY_BIT")]
        #[default]
        MEMCPY = 1,
    }
}
/// [`VkHostImageCopyFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkHostImageCopyFlagBitsEXT.html)
///
#[doc(alias = "VkHostImageCopyFlagBitsEXT")]
pub type HostImageCopyFlagEXT = HostImageCopyFlag;
impl HostImageCopyFlag {
    /// [`VK_HOST_IMAGE_COPY_MEMCPY_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_HOST_IMAGE_COPY_MEMCPY_BIT_EXT.html)
    ///
    #[doc(alias = "VK_HOST_IMAGE_COPY_MEMCPY_BIT_EXT")]
    pub const MEMCPY_EXT: Self = Self::MEMCPY;
}
/// [`VkHostImageCopyFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkHostImageCopyFlags.html)
///
#[doc(alias = "VkHostImageCopyFlags")]
pub type HostImageCopyFlags = FlagSet<HostImageCopyFlag>;
/// [`VkHostImageCopyFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkHostImageCopyFlagsEXT.html)
///
#[doc(alias = "VkHostImageCopyFlagsEXT")]
pub type HostImageCopyFlagsEXT = HostImageCopyFlags;

crate::__vkx_internal_flags! {
    /// [`VkPipelineCreateFlagBits2`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlagBits2.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.4 with appropriate features
    /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
    /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkPipelineCreateFlagBits2")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum PipelineCreateFlag2: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_DISABLE_OPTIMIZATION_BIT")]
        #[default]
        DISABLE_OPTIMIZATION = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_ALLOW_DERIVATIVES_BIT")]
        ALLOW_DERIVATIVES = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_DERIVATIVE_BIT")]
        DERIVATIVE = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_VIEW_INDEX_FROM_DEVICE_INDEX_BIT")]
        VIEW_INDEX_FROM_DEVICE_INDEX = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_DISPATCH_BASE_BIT")]
        DISPATCH_BASE = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT")]
        FAIL_ON_PIPELINE_COMPILE_REQUIRED = 256,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_EARLY_RETURN_ON_FAILURE_BIT")]
        EARLY_RETURN_ON_FAILURE = 512,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_NO_PROTECTED_ACCESS_BIT")]
        NO_PROTECTED_ACCESS = 134217728,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_PROTECTED_ACCESS_ONLY_BIT")]
        PROTECTED_ACCESS_ONLY = 1073741824,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMDX_ShaderEnqueue`](Extension::AMDX_ShaderEnqueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_EXECUTION_GRAPH_BIT_AMDX")]
        EXECUTION_GRAPH_AMDX = 4294967296,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_DESCRIPTOR_HEAP_BIT_EXT")]
        DESCRIPTOR_HEAP_EXT = 68719476736,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracingLinearSweptSpheres`](Extension::NV_RayTracingLinearSweptSpheres)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_ALLOW_SPHERES_AND_LINEAR_SWEPT_SPHERES_BIT_NV")]
        RAY_TRACING_ALLOW_SPHERES_AND_LINEAR_SWEPT_SPHERES_NV = 8589934592,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_LegacyDithering`](Extension::EXT_LegacyDithering)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_ENABLE_LEGACY_DITHERING_BIT_EXT")]
        ENABLE_LEGACY_DITHERING_EXT = 17179869184,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_DEFER_COMPILE_BIT_NV")]
        DEFER_COMPILE_NV = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_CAPTURE_STATISTICS_BIT_KHR")]
        CAPTURE_STATISTICS_KHR = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR")]
        CAPTURE_INTERNAL_REPRESENTATIONS_KHR = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_LINK_TIME_OPTIMIZATION_BIT_EXT")]
        LINK_TIME_OPTIMIZATION_EXT = 1024,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RETAIN_LINK_TIME_OPTIMIZATION_INFO_BIT_EXT")]
        RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT = 8388608,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_LIBRARY_BIT_KHR")]
        LIBRARY_KHR = 2048,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR")]
        RAY_TRACING_SKIP_TRIANGLES_KHR = 4096,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_SKIP_AABBS_BIT_KHR")]
        RAY_TRACING_SKIP_AABBS_KHR = 8192,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR")]
        RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR = 16384,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR")]
        RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR = 32768,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR")]
        RAY_TRACING_NO_NULL_MISS_SHADERS_KHR = 65536,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR")]
        RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR = 131072,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR")]
        RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR = 524288,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_INDIRECT_BINDABLE_BIT_NV")]
        INDIRECT_BINDABLE_NV = 262144,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_ALLOW_MOTION_BIT_NV")]
        RAY_TRACING_ALLOW_MOTION_NV = 1048576,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
        RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = 2097152,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT")]
        RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT = 4194304,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_COLOR_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT")]
        COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT = 33554432,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT")]
        DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT = 67108864,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_DISPLACEMENT_MICROMAP_BIT_NV")]
        RAY_TRACING_DISPLACEMENT_MICROMAP_NV = 268435456,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_DESCRIPTOR_BUFFER_BIT_EXT")]
        DESCRIPTOR_BUFFER_EXT = 536870912,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        /// - Extension [`ARM_PipelineOpacityMicromap`](Extension::ARM_PipelineOpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_DISALLOW_OPACITY_MICROMAP_BIT_ARM")]
        DISALLOW_OPACITY_MICROMAP_ARM = 137438953472,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`ARM_ShaderInstrumentation`](Extension::ARM_ShaderInstrumentation)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_INSTRUMENT_SHADERS_BIT_ARM")]
        INSTRUMENT_SHADERS_ARM = 549755813888,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_PipelineBinary`](Extension::KHR_PipelineBinary)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_CAPTURE_DATA_BIT_KHR")]
        CAPTURE_DATA_KHR = 2147483648,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DeviceGeneratedCommands`](Extension::EXT_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_INDIRECT_BINDABLE_BIT_EXT")]
        INDIRECT_BINDABLE_EXT = 274877906944,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_FragmentDensityMapLayered`](Extension::VALVE_FragmentDensityMapLayered)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE")]
        PER_LAYER_FRAGMENT_DENSITY_VALVE = 1099511627776,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        /// - Extension [`EXT_OpacityMicromap`](Extension::EXT_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_RAY_TRACING_OPACITY_MICROMAP_BIT_KHR")]
        RAY_TRACING_OPACITY_MICROMAP_KHR = 16777216,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_OPACITY_MICROMAP_DISALLOW_MIXED_SPECIAL_INDEX_BIT_KHR")]
        OPACITY_MICROMAP_DISALLOW_MIXED_SPECIAL_INDEX_KHR = 2199023255552,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_Shader64BitIndexing`](Extension::EXT_Shader64BitIndexing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PIPELINE_CREATE_2_64_BIT_INDEXING_BIT_EXT")]
        _64_INDEXING_EXT = 8796093022208,
    }
}
/// [`VkPipelineCreateFlagBits2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlagBits2KHR.html)
///
#[doc(alias = "VkPipelineCreateFlagBits2KHR")]
pub type PipelineCreateFlag2KHR = PipelineCreateFlag2;
impl PipelineCreateFlag2 {
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
/// [`VkPipelineCreateFlags2`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlags2.html)
///
#[doc(alias = "VkPipelineCreateFlags2")]
pub type PipelineCreateFlags2 = FlagSet<PipelineCreateFlag2>;
/// [`VkPipelineCreateFlags2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlags2KHR.html)
///
#[doc(alias = "VkPipelineCreateFlags2KHR")]
pub type PipelineCreateFlags2KHR = PipelineCreateFlags2;

crate::__vkx_internal_flags! {
    /// [`VkSurfaceTransformFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceTransformFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_Surface`](Extension::KHR_Surface)
    /// - Extension [`KHR_Display`](Extension::KHR_Display)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkSurfaceTransformFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum SurfaceTransformFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extension::KHR_Surface)
        /// - Extension [`KHR_Display`](Extension::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR")]
        #[default]
        IDENTITY_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extension::KHR_Surface)
        /// - Extension [`KHR_Display`](Extension::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR")]
        ROTATE_90_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extension::KHR_Surface)
        /// - Extension [`KHR_Display`](Extension::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR")]
        ROTATE_180_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extension::KHR_Surface)
        /// - Extension [`KHR_Display`](Extension::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR")]
        ROTATE_270_KHR = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extension::KHR_Surface)
        /// - Extension [`KHR_Display`](Extension::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR")]
        HORIZONTAL_MIRROR_KHR = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extension::KHR_Surface)
        /// - Extension [`KHR_Display`](Extension::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR")]
        HORIZONTAL_MIRROR_ROTATE_90_KHR = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extension::KHR_Surface)
        /// - Extension [`KHR_Display`](Extension::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR")]
        HORIZONTAL_MIRROR_ROTATE_180_KHR = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extension::KHR_Surface)
        /// - Extension [`KHR_Display`](Extension::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR")]
        HORIZONTAL_MIRROR_ROTATE_270_KHR = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extension::KHR_Surface)
        /// - Extension [`KHR_Display`](Extension::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR")]
        INHERIT_KHR = 256,
    }
}
/// [`VkSurfaceTransformFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceTransformFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_Surface`](Extension::KHR_Surface)
/// - Extension [`KHR_Display`](Extension::KHR_Display)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkSurfaceTransformFlagsKHR")]
pub type SurfaceTransformFlagsKHR = FlagSet<SurfaceTransformFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkCompositeAlphaFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCompositeAlphaFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_Surface`](Extension::KHR_Surface)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkCompositeAlphaFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum CompositeAlphaFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extension::KHR_Surface)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR")]
        #[default]
        OPAQUE_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extension::KHR_Surface)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR")]
        PRE_MULTIPLIED_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extension::KHR_Surface)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR")]
        POST_MULTIPLIED_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Surface`](Extension::KHR_Surface)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR")]
        INHERIT_KHR = 8,
    }
}
/// [`VkCompositeAlphaFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCompositeAlphaFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_Surface`](Extension::KHR_Surface)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkCompositeAlphaFlagsKHR")]
pub type CompositeAlphaFlagsKHR = FlagSet<CompositeAlphaFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkSwapchainCreateFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainCreateFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_Swapchain`](Extension::KHR_Swapchain)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkSwapchainCreateFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum SwapchainCreateFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_Swapchain`](Extension::KHR_Swapchain)
        /// - Extension [`KHR_DeviceGroup`](Extension::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR")]
        #[default]
        SPLIT_INSTANCE_BIND_REGIONS_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Swapchain`](Extension::KHR_Swapchain)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR")]
        PROTECTED_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_SwapchainMutableFormat`](Extension::KHR_SwapchainMutableFormat)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR")]
        MUTABLE_FORMAT_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_PresentTiming`](Extension::EXT_PresentTiming)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SWAPCHAIN_CREATE_PRESENT_TIMING_BIT_EXT")]
        PRESENT_TIMING_EXT = 512,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_PresentId2`](Extension::KHR_PresentId2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SWAPCHAIN_CREATE_PRESENT_ID_2_BIT_KHR")]
        PRESENT_ID_2_KHR = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_PresentWait2`](Extension::KHR_PresentWait2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SWAPCHAIN_CREATE_PRESENT_WAIT_2_BIT_KHR")]
        PRESENT_WAIT_2_KHR = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_SwapchainMaintenance1`](Extension::KHR_SwapchainMaintenance1)
        /// - Extension [`EXT_SwapchainMaintenance1`](Extension::EXT_SwapchainMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SWAPCHAIN_CREATE_DEFERRED_MEMORY_ALLOCATION_BIT_KHR")]
        DEFERRED_MEMORY_ALLOCATION_KHR = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MultisampledRenderToSwapchain`](Extension::EXT_MultisampledRenderToSwapchain)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SWAPCHAIN_CREATE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT")]
        MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT = 256,
    }
}
impl SwapchainCreateFlagKHR {
    /// [`VK_SWAPCHAIN_CREATE_DEFERRED_MEMORY_ALLOCATION_BIT_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SWAPCHAIN_CREATE_DEFERRED_MEMORY_ALLOCATION_BIT_EXT.html)
    ///
    #[doc(alias = "VK_SWAPCHAIN_CREATE_DEFERRED_MEMORY_ALLOCATION_BIT_EXT")]
    pub const DEFERRED_MEMORY_ALLOCATION_EXT: Self = Self::DEFERRED_MEMORY_ALLOCATION_KHR;
}
/// [`VkSwapchainCreateFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainCreateFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_Swapchain`](Extension::KHR_Swapchain)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkSwapchainCreateFlagsKHR")]
pub type SwapchainCreateFlagsKHR = FlagSet<SwapchainCreateFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkDeviceGroupPresentModeFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupPresentModeFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Version 1.1 with appropriate features
    /// - Extension [`KHR_Swapchain`](Extension::KHR_Swapchain)
    /// - Extension [`KHR_DeviceGroup`](Extension::KHR_DeviceGroup)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDeviceGroupPresentModeFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum DeviceGroupPresentModeFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_Swapchain`](Extension::KHR_Swapchain)
        /// - Extension [`KHR_DeviceGroup`](Extension::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR")]
        #[default]
        LOCAL_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_Swapchain`](Extension::KHR_Swapchain)
        /// - Extension [`KHR_DeviceGroup`](Extension::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR")]
        REMOTE_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_Swapchain`](Extension::KHR_Swapchain)
        /// - Extension [`KHR_DeviceGroup`](Extension::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR")]
        SUM_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.1 with appropriate features
        /// - Extension [`KHR_Swapchain`](Extension::KHR_Swapchain)
        /// - Extension [`KHR_DeviceGroup`](Extension::KHR_DeviceGroup)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR")]
        LOCAL_MULTI_DEVICE_KHR = 8,
    }
}
/// [`VkDeviceGroupPresentModeFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupPresentModeFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Version 1.1 with appropriate features
/// - Extension [`KHR_Swapchain`](Extension::KHR_Swapchain)
/// - Extension [`KHR_DeviceGroup`](Extension::KHR_DeviceGroup)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkDeviceGroupPresentModeFlagsKHR")]
pub type DeviceGroupPresentModeFlagsKHR = FlagSet<DeviceGroupPresentModeFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkDisplayPlaneAlphaFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayPlaneAlphaFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_Display`](Extension::KHR_Display)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDisplayPlaneAlphaFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum DisplayPlaneAlphaFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Display`](Extension::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR")]
        #[default]
        OPAQUE_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Display`](Extension::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR")]
        GLOBAL_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Display`](Extension::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR")]
        PER_PIXEL_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Display`](Extension::KHR_Display)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR")]
        PER_PIXEL_PREMULTIPLIED_KHR = 8,
    }
}
/// [`VkDisplayPlaneAlphaFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayPlaneAlphaFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_Display`](Extension::KHR_Display)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkDisplayPlaneAlphaFlagsKHR")]
pub type DisplayPlaneAlphaFlagsKHR = FlagSet<DisplayPlaneAlphaFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoCodecOperationFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoCodecOperationFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoCodecOperationFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoCodecOperationFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CODEC_OPERATION_NONE_KHR")]
        #[default]
        NONE_KHR = 0,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CODEC_OPERATION_ENCODE_H264_BIT_KHR")]
        ENCODE_H264_KHR = 65536,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CODEC_OPERATION_ENCODE_H265_BIT_KHR")]
        ENCODE_H265_KHR = 131072,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeH264`](Extension::KHR_VideoDecodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CODEC_OPERATION_DECODE_H264_BIT_KHR")]
        DECODE_H264_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeH265`](Extension::KHR_VideoDecodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CODEC_OPERATION_DECODE_H265_BIT_KHR")]
        DECODE_H265_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeAv1`](Extension::KHR_VideoDecodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CODEC_OPERATION_DECODE_AV1_BIT_KHR")]
        DECODE_AV1_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extension::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CODEC_OPERATION_ENCODE_AV1_BIT_KHR")]
        ENCODE_AV1_KHR = 262144,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeVp9`](Extension::KHR_VideoDecodeVp9)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CODEC_OPERATION_DECODE_VP9_BIT_KHR")]
        DECODE_VP9_KHR = 8,
    }
}
/// [`VkVideoCodecOperationFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoCodecOperationFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkVideoCodecOperationFlagsKHR")]
pub type VideoCodecOperationFlagsKHR = FlagSet<VideoCodecOperationFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoChromaSubsamplingFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoChromaSubsamplingFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoChromaSubsamplingFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoChromaSubsamplingFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_INVALID_KHR")]
        #[default]
        INVALID_KHR = 0,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_MONOCHROME_BIT_KHR")]
        MONOCHROME_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_420_BIT_KHR")]
        _420_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_422_BIT_KHR")]
        _422_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_444_BIT_KHR")]
        _444_KHR = 8,
    }
}
/// [`VkVideoChromaSubsamplingFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoChromaSubsamplingFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkVideoChromaSubsamplingFlagsKHR")]
pub type VideoChromaSubsamplingFlagsKHR = FlagSet<VideoChromaSubsamplingFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoComponentBitDepthFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoComponentBitDepthFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoComponentBitDepthFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoComponentBitDepthFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_COMPONENT_BIT_DEPTH_INVALID_KHR")]
        #[default]
        INVALID_KHR = 0,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_COMPONENT_BIT_DEPTH_8_BIT_KHR")]
        _8_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_COMPONENT_BIT_DEPTH_10_BIT_KHR")]
        _10_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_COMPONENT_BIT_DEPTH_12_BIT_KHR")]
        _12_KHR = 16,
    }
}
/// [`VkVideoComponentBitDepthFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoComponentBitDepthFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkVideoComponentBitDepthFlagsKHR")]
pub type VideoComponentBitDepthFlagsKHR = FlagSet<VideoComponentBitDepthFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoCapabilityFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoCapabilityFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkVideoCapabilityFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoCapabilityFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CAPABILITY_PROTECTED_CONTENT_BIT_KHR")]
        #[default]
        PROTECTED_CONTENT_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CAPABILITY_SEPARATE_REFERENCE_IMAGES_BIT_KHR")]
        SEPARATE_REFERENCE_IMAGES_KHR = 2,
    }
}
/// [`VkVideoCapabilityFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoCapabilityFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkVideoCapabilityFlagsKHR")]
pub type VideoCapabilityFlagsKHR = FlagSet<VideoCapabilityFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoSessionCreateFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionCreateFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoSessionCreateFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoSessionCreateFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_SESSION_CREATE_PROTECTED_CONTENT_BIT_KHR")]
        #[default]
        PROTECTED_CONTENT_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_SESSION_CREATE_ALLOW_ENCODE_PARAMETER_OPTIMIZATIONS_BIT_KHR")]
        ALLOW_ENCODE_PARAMETER_OPTIMIZATIONS_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoMaintenance1`](Extension::KHR_VideoMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_SESSION_CREATE_INLINE_QUERIES_BIT_KHR")]
        INLINE_QUERIES_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extension::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_SESSION_CREATE_ALLOW_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR")]
        ALLOW_ENCODE_QUANTIZATION_DELTA_MAP_KHR = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extension::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_SESSION_CREATE_ALLOW_ENCODE_EMPHASIS_MAP_BIT_KHR")]
        ALLOW_ENCODE_EMPHASIS_MAP_KHR = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoMaintenance2`](Extension::KHR_VideoMaintenance2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_SESSION_CREATE_INLINE_SESSION_PARAMETERS_BIT_KHR")]
        INLINE_SESSION_PARAMETERS_KHR = 32,
    }
}
/// [`VkVideoSessionCreateFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionCreateFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkVideoSessionCreateFlagsKHR")]
pub type VideoSessionCreateFlagsKHR = FlagSet<VideoSessionCreateFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoSessionParametersCreateFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionParametersCreateFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoSessionParametersCreateFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoSessionParametersCreateFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extension::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_SESSION_PARAMETERS_CREATE_QUANTIZATION_MAP_COMPATIBLE_BIT_KHR")]
        #[default]
        QUANTIZATION_MAP_COMPATIBLE_KHR = 1,
    }
}
/// [`VkVideoSessionParametersCreateFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionParametersCreateFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkVideoSessionParametersCreateFlagsKHR")]
pub type VideoSessionParametersCreateFlagsKHR = FlagSet<VideoSessionParametersCreateFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoCodingControlFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoCodingControlFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoCodingControlFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoCodingControlFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CODING_CONTROL_RESET_BIT_KHR")]
        #[default]
        RESET_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CODING_CONTROL_ENCODE_RATE_CONTROL_BIT_KHR")]
        ENCODE_RATE_CONTROL_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_CODING_CONTROL_ENCODE_QUALITY_LEVEL_BIT_KHR")]
        ENCODE_QUALITY_LEVEL_KHR = 4,
    }
}
/// [`VkVideoCodingControlFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoCodingControlFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkVideoCodingControlFlagsKHR")]
pub type VideoCodingControlFlagsKHR = FlagSet<VideoCodingControlFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoDecodeCapabilityFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeCapabilityFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoDecodeQueue`](Extension::KHR_VideoDecodeQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkVideoDecodeCapabilityFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoDecodeCapabilityFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extension::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_COINCIDE_BIT_KHR")]
        #[default]
        DPB_AND_OUTPUT_COINCIDE_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extension::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_DISTINCT_BIT_KHR")]
        DPB_AND_OUTPUT_DISTINCT_KHR = 2,
    }
}
/// [`VkVideoDecodeCapabilityFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeCapabilityFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoDecodeQueue`](Extension::KHR_VideoDecodeQueue)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkVideoDecodeCapabilityFlagsKHR")]
pub type VideoDecodeCapabilityFlagsKHR = FlagSet<VideoDecodeCapabilityFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoDecodeUsageFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeUsageFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoDecodeQueue`](Extension::KHR_VideoDecodeQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoDecodeUsageFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoDecodeUsageFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extension::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_DECODE_USAGE_DEFAULT_KHR")]
        #[default]
        DEFAULT_KHR = 0,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extension::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_DECODE_USAGE_TRANSCODING_BIT_KHR")]
        TRANSCODING_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extension::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_DECODE_USAGE_OFFLINE_BIT_KHR")]
        OFFLINE_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeQueue`](Extension::KHR_VideoDecodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_DECODE_USAGE_STREAMING_BIT_KHR")]
        STREAMING_KHR = 4,
    }
}
/// [`VkVideoDecodeUsageFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeUsageFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoDecodeQueue`](Extension::KHR_VideoDecodeQueue)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkVideoDecodeUsageFlagsKHR")]
pub type VideoDecodeUsageFlagsKHR = FlagSet<VideoDecodeUsageFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoEncodeH264CapabilityFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264CapabilityFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkVideoEncodeH264CapabilityFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoEncodeH264CapabilityFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_HRD_COMPLIANCE_BIT_KHR")]
        #[default]
        VIDEO_ENCODE_H264_CAPABILITY_HRD_COMPLIANCE_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_PREDICTION_WEIGHT_TABLE_GENERATED_BIT_KHR")]
        VIDEO_ENCODE_H264_CAPABILITY_PREDICTION_WEIGHT_TABLE_GENERATED_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_ROW_UNALIGNED_SLICE_BIT_KHR")]
        VIDEO_ENCODE_H264_CAPABILITY_ROW_UNALIGNED_SLICE_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_DIFFERENT_SLICE_TYPE_BIT_KHR")]
        VIDEO_ENCODE_H264_CAPABILITY_DIFFERENT_SLICE_TYPE_KHR = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L0_LIST_BIT_KHR")]
        VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L0_LIST_KHR = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L1_LIST_BIT_KHR")]
        VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L1_LIST_KHR = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_PER_PICTURE_TYPE_MIN_MAX_QP_BIT_KHR")]
        VIDEO_ENCODE_H264_CAPABILITY_PER_PICTURE_TYPE_MIN_MAX_QP_KHR = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_PER_SLICE_CONSTANT_QP_BIT_KHR")]
        VIDEO_ENCODE_H264_CAPABILITY_PER_SLICE_CONSTANT_QP_KHR = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_GENERATE_PREFIX_NALU_BIT_KHR")]
        VIDEO_ENCODE_H264_CAPABILITY_GENERATE_PREFIX_NALU_KHR = 256,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeIntraRefresh`](Extension::KHR_VideoEncodeIntraRefresh)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_B_PICTURE_INTRA_REFRESH_BIT_KHR")]
        VIDEO_ENCODE_H264_CAPABILITY_B_PICTURE_INTRA_REFRESH_KHR = 1024,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extension::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_CAPABILITY_MB_QP_DIFF_WRAPAROUND_BIT_KHR")]
        VIDEO_ENCODE_H264_CAPABILITY_MB_QP_DIFF_WRAPAROUND_KHR = 512,
    }
}
/// [`VkVideoEncodeH264CapabilityFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264CapabilityFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkVideoEncodeH264CapabilityFlagsKHR")]
pub type VideoEncodeH264CapabilityFlagsKHR = FlagSet<VideoEncodeH264CapabilityFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoEncodeH264StdFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264StdFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkVideoEncodeH264StdFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoEncodeH264StdFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_SEPARATE_COLOR_PLANE_FLAG_SET_BIT_KHR")]
        #[default]
        VIDEO_ENCODE_H264_STD_SEPARATE_COLOR_PLANE_FLAG_SET_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_BIT_KHR")]
        VIDEO_ENCODE_H264_STD_QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_SCALING_MATRIX_PRESENT_FLAG_SET_BIT_KHR")]
        VIDEO_ENCODE_H264_STD_SCALING_MATRIX_PRESENT_FLAG_SET_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_CHROMA_QP_INDEX_OFFSET_BIT_KHR")]
        VIDEO_ENCODE_H264_STD_CHROMA_QP_INDEX_OFFSET_KHR = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_SECOND_CHROMA_QP_INDEX_OFFSET_BIT_KHR")]
        VIDEO_ENCODE_H264_STD_SECOND_CHROMA_QP_INDEX_OFFSET_KHR = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_PIC_INIT_QP_MINUS26_BIT_KHR")]
        VIDEO_ENCODE_H264_STD_PIC_INIT_QP_MINUS26_KHR = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_WEIGHTED_PRED_FLAG_SET_BIT_KHR")]
        VIDEO_ENCODE_H264_STD_WEIGHTED_PRED_FLAG_SET_KHR = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_WEIGHTED_BIPRED_IDC_EXPLICIT_BIT_KHR")]
        VIDEO_ENCODE_H264_STD_WEIGHTED_BIPRED_IDC_EXPLICIT_KHR = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_WEIGHTED_BIPRED_IDC_IMPLICIT_BIT_KHR")]
        VIDEO_ENCODE_H264_STD_WEIGHTED_BIPRED_IDC_IMPLICIT_KHR = 256,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_TRANSFORM_8X8_MODE_FLAG_SET_BIT_KHR")]
        VIDEO_ENCODE_H264_STD_TRANSFORM_8X8_MODE_FLAG_SET_KHR = 512,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_BIT_KHR")]
        VIDEO_ENCODE_H264_STD_DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_KHR = 1024,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_ENTROPY_CODING_MODE_FLAG_UNSET_BIT_KHR")]
        VIDEO_ENCODE_H264_STD_ENTROPY_CODING_MODE_FLAG_UNSET_KHR = 2048,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_ENTROPY_CODING_MODE_FLAG_SET_BIT_KHR")]
        VIDEO_ENCODE_H264_STD_ENTROPY_CODING_MODE_FLAG_SET_KHR = 4096,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_DIRECT_8X8_INFERENCE_FLAG_UNSET_BIT_KHR")]
        VIDEO_ENCODE_H264_STD_DIRECT_8X8_INFERENCE_FLAG_UNSET_KHR = 8192,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_CONSTRAINED_INTRA_PRED_FLAG_SET_BIT_KHR")]
        VIDEO_ENCODE_H264_STD_CONSTRAINED_INTRA_PRED_FLAG_SET_KHR = 16384,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_DISABLED_BIT_KHR")]
        VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_DISABLED_KHR = 32768,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_ENABLED_BIT_KHR")]
        VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_ENABLED_KHR = 65536,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_PARTIAL_BIT_KHR")]
        VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_PARTIAL_KHR = 131072,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_SLICE_QP_DELTA_BIT_KHR")]
        VIDEO_ENCODE_H264_STD_SLICE_QP_DELTA_KHR = 524288,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_STD_DIFFERENT_SLICE_QP_DELTA_BIT_KHR")]
        VIDEO_ENCODE_H264_STD_DIFFERENT_SLICE_QP_DELTA_KHR = 1048576,
    }
}
/// [`VkVideoEncodeH264StdFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264StdFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkVideoEncodeH264StdFlagsKHR")]
pub type VideoEncodeH264StdFlagsKHR = FlagSet<VideoEncodeH264StdFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoEncodeH264RateControlFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264RateControlFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodeH264RateControlFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoEncodeH264RateControlFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_RATE_CONTROL_ATTEMPT_HRD_COMPLIANCE_BIT_KHR")]
        #[default]
        VIDEO_ENCODE_H264_RATE_CONTROL_ATTEMPT_HRD_COMPLIANCE_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_RATE_CONTROL_REGULAR_GOP_BIT_KHR")]
        VIDEO_ENCODE_H264_RATE_CONTROL_REGULAR_GOP_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR")]
        VIDEO_ENCODE_H264_RATE_CONTROL_REFERENCE_PATTERN_FLAT_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR")]
        VIDEO_ENCODE_H264_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_KHR = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H264_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_BIT_KHR")]
        VIDEO_ENCODE_H264_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_KHR = 16,
    }
}
/// [`VkVideoEncodeH264RateControlFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264RateControlFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoEncodeH264`](Extension::KHR_VideoEncodeH264)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkVideoEncodeH264RateControlFlagsKHR")]
pub type VideoEncodeH264RateControlFlagsKHR = FlagSet<VideoEncodeH264RateControlFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoEncodeH265CapabilityFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265CapabilityFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkVideoEncodeH265CapabilityFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoEncodeH265CapabilityFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_HRD_COMPLIANCE_BIT_KHR")]
        #[default]
        VIDEO_ENCODE_H265_CAPABILITY_HRD_COMPLIANCE_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_PREDICTION_WEIGHT_TABLE_GENERATED_BIT_KHR")]
        VIDEO_ENCODE_H265_CAPABILITY_PREDICTION_WEIGHT_TABLE_GENERATED_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_ROW_UNALIGNED_SLICE_SEGMENT_BIT_KHR")]
        VIDEO_ENCODE_H265_CAPABILITY_ROW_UNALIGNED_SLICE_SEGMENT_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_DIFFERENT_SLICE_SEGMENT_TYPE_BIT_KHR")]
        VIDEO_ENCODE_H265_CAPABILITY_DIFFERENT_SLICE_SEGMENT_TYPE_KHR = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L0_LIST_BIT_KHR")]
        VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L0_LIST_KHR = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L1_LIST_BIT_KHR")]
        VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L1_LIST_KHR = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_PER_PICTURE_TYPE_MIN_MAX_QP_BIT_KHR")]
        VIDEO_ENCODE_H265_CAPABILITY_PER_PICTURE_TYPE_MIN_MAX_QP_KHR = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_PER_SLICE_SEGMENT_CONSTANT_QP_BIT_KHR")]
        VIDEO_ENCODE_H265_CAPABILITY_PER_SLICE_SEGMENT_CONSTANT_QP_KHR = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_TILES_PER_SLICE_SEGMENT_BIT_KHR")]
        VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_TILES_PER_SLICE_SEGMENT_KHR = 256,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_SLICE_SEGMENTS_PER_TILE_BIT_KHR")]
        VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_SLICE_SEGMENTS_PER_TILE_KHR = 512,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeIntraRefresh`](Extension::KHR_VideoEncodeIntraRefresh)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_B_PICTURE_INTRA_REFRESH_BIT_KHR")]
        VIDEO_ENCODE_H265_CAPABILITY_B_PICTURE_INTRA_REFRESH_KHR = 2048,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extension::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CAPABILITY_CU_QP_DIFF_WRAPAROUND_BIT_KHR")]
        VIDEO_ENCODE_H265_CAPABILITY_CU_QP_DIFF_WRAPAROUND_KHR = 1024,
    }
}
/// [`VkVideoEncodeH265CapabilityFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265CapabilityFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkVideoEncodeH265CapabilityFlagsKHR")]
pub type VideoEncodeH265CapabilityFlagsKHR = FlagSet<VideoEncodeH265CapabilityFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoEncodeH265StdFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265StdFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkVideoEncodeH265StdFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoEncodeH265StdFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_SEPARATE_COLOR_PLANE_FLAG_SET_BIT_KHR")]
        #[default]
        VIDEO_ENCODE_H265_STD_SEPARATE_COLOR_PLANE_FLAG_SET_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET_BIT_KHR")]
        VIDEO_ENCODE_H265_STD_SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_SCALING_LIST_DATA_PRESENT_FLAG_SET_BIT_KHR")]
        VIDEO_ENCODE_H265_STD_SCALING_LIST_DATA_PRESENT_FLAG_SET_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_PCM_ENABLED_FLAG_SET_BIT_KHR")]
        VIDEO_ENCODE_H265_STD_PCM_ENABLED_FLAG_SET_KHR = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_SPS_TEMPORAL_MVP_ENABLED_FLAG_SET_BIT_KHR")]
        VIDEO_ENCODE_H265_STD_SPS_TEMPORAL_MVP_ENABLED_FLAG_SET_KHR = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_INIT_QP_MINUS26_BIT_KHR")]
        VIDEO_ENCODE_H265_STD_INIT_QP_MINUS26_KHR = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_WEIGHTED_PRED_FLAG_SET_BIT_KHR")]
        VIDEO_ENCODE_H265_STD_WEIGHTED_PRED_FLAG_SET_KHR = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_WEIGHTED_BIPRED_FLAG_SET_BIT_KHR")]
        VIDEO_ENCODE_H265_STD_WEIGHTED_BIPRED_FLAG_SET_KHR = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_LOG2_PARALLEL_MERGE_LEVEL_MINUS2_BIT_KHR")]
        VIDEO_ENCODE_H265_STD_LOG2_PARALLEL_MERGE_LEVEL_MINUS2_KHR = 256,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_SIGN_DATA_HIDING_ENABLED_FLAG_SET_BIT_KHR")]
        VIDEO_ENCODE_H265_STD_SIGN_DATA_HIDING_ENABLED_FLAG_SET_KHR = 512,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_TRANSFORM_SKIP_ENABLED_FLAG_SET_BIT_KHR")]
        VIDEO_ENCODE_H265_STD_TRANSFORM_SKIP_ENABLED_FLAG_SET_KHR = 1024,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_TRANSFORM_SKIP_ENABLED_FLAG_UNSET_BIT_KHR")]
        VIDEO_ENCODE_H265_STD_TRANSFORM_SKIP_ENABLED_FLAG_UNSET_KHR = 2048,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET_BIT_KHR")]
        VIDEO_ENCODE_H265_STD_PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET_KHR = 4096,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_TRANSQUANT_BYPASS_ENABLED_FLAG_SET_BIT_KHR")]
        VIDEO_ENCODE_H265_STD_TRANSQUANT_BYPASS_ENABLED_FLAG_SET_KHR = 8192,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_CONSTRAINED_INTRA_PRED_FLAG_SET_BIT_KHR")]
        VIDEO_ENCODE_H265_STD_CONSTRAINED_INTRA_PRED_FLAG_SET_KHR = 16384,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_ENTROPY_CODING_SYNC_ENABLED_FLAG_SET_BIT_KHR")]
        VIDEO_ENCODE_H265_STD_ENTROPY_CODING_SYNC_ENABLED_FLAG_SET_KHR = 32768,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET_BIT_KHR")]
        VIDEO_ENCODE_H265_STD_DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET_KHR = 65536,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET_BIT_KHR")]
        VIDEO_ENCODE_H265_STD_DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET_KHR = 131072,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_DEPENDENT_SLICE_SEGMENT_FLAG_SET_BIT_KHR")]
        VIDEO_ENCODE_H265_STD_DEPENDENT_SLICE_SEGMENT_FLAG_SET_KHR = 262144,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_SLICE_QP_DELTA_BIT_KHR")]
        VIDEO_ENCODE_H265_STD_SLICE_QP_DELTA_KHR = 524288,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_STD_DIFFERENT_SLICE_QP_DELTA_BIT_KHR")]
        VIDEO_ENCODE_H265_STD_DIFFERENT_SLICE_QP_DELTA_KHR = 1048576,
    }
}
/// [`VkVideoEncodeH265StdFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265StdFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkVideoEncodeH265StdFlagsKHR")]
pub type VideoEncodeH265StdFlagsKHR = FlagSet<VideoEncodeH265StdFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoEncodeH265CtbSizeFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265CtbSizeFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkVideoEncodeH265CtbSizeFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoEncodeH265CtbSizeFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CTB_SIZE_16_BIT_KHR")]
        #[default]
        VIDEO_ENCODE_H265_CTB_SIZE_16_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CTB_SIZE_32_BIT_KHR")]
        VIDEO_ENCODE_H265_CTB_SIZE_32_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_CTB_SIZE_64_BIT_KHR")]
        VIDEO_ENCODE_H265_CTB_SIZE_64_KHR = 4,
    }
}
/// [`VkVideoEncodeH265CtbSizeFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265CtbSizeFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkVideoEncodeH265CtbSizeFlagsKHR")]
pub type VideoEncodeH265CtbSizeFlagsKHR = FlagSet<VideoEncodeH265CtbSizeFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoEncodeH265TransformBlockSizeFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265TransformBlockSizeFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkVideoEncodeH265TransformBlockSizeFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoEncodeH265TransformBlockSizeFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_4_BIT_KHR")]
        #[default]
        VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_4_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_8_BIT_KHR")]
        VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_8_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_16_BIT_KHR")]
        VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_16_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_32_BIT_KHR")]
        VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_32_KHR = 8,
    }
}
/// [`VkVideoEncodeH265TransformBlockSizeFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265TransformBlockSizeFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkVideoEncodeH265TransformBlockSizeFlagsKHR")]
pub type VideoEncodeH265TransformBlockSizeFlagsKHR =
    FlagSet<VideoEncodeH265TransformBlockSizeFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoEncodeH265RateControlFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265RateControlFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodeH265RateControlFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoEncodeH265RateControlFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_RATE_CONTROL_ATTEMPT_HRD_COMPLIANCE_BIT_KHR")]
        #[default]
        VIDEO_ENCODE_H265_RATE_CONTROL_ATTEMPT_HRD_COMPLIANCE_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_RATE_CONTROL_REGULAR_GOP_BIT_KHR")]
        VIDEO_ENCODE_H265_RATE_CONTROL_REGULAR_GOP_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR")]
        VIDEO_ENCODE_H265_RATE_CONTROL_REFERENCE_PATTERN_FLAT_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR")]
        VIDEO_ENCODE_H265_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_KHR = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_H265_RATE_CONTROL_TEMPORAL_SUB_LAYER_PATTERN_DYADIC_BIT_KHR")]
        VIDEO_ENCODE_H265_RATE_CONTROL_TEMPORAL_SUB_LAYER_PATTERN_DYADIC_KHR = 16,
    }
}
/// [`VkVideoEncodeH265RateControlFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265RateControlFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoEncodeH265`](Extension::KHR_VideoEncodeH265)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkVideoEncodeH265RateControlFlagsKHR")]
pub type VideoEncodeH265RateControlFlagsKHR = FlagSet<VideoEncodeH265RateControlFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoDecodeH264PictureLayoutFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeH264PictureLayoutFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoDecodeH264`](Extension::KHR_VideoDecodeH264)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoDecodeH264PictureLayoutFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoDecodeH264PictureLayoutFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeH264`](Extension::KHR_VideoDecodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_PROGRESSIVE_KHR")]
        #[default]
        VIDEO_DECODE_H264_PICTURE_LAYOUT_PROGRESSIVE_KHR = 0,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeH264`](Extension::KHR_VideoDecodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_INTERLEAVED_LINES_BIT_KHR")]
        VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_INTERLEAVED_LINES_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoDecodeH264`](Extension::KHR_VideoDecodeH264)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_SEPARATE_PLANES_BIT_KHR")]
        VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_SEPARATE_PLANES_KHR = 2,
    }
}
/// [`VkVideoDecodeH264PictureLayoutFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeH264PictureLayoutFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoDecodeH264`](Extension::KHR_VideoDecodeH264)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkVideoDecodeH264PictureLayoutFlagsKHR")]
pub type VideoDecodeH264PictureLayoutFlagsKHR = FlagSet<VideoDecodeH264PictureLayoutFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkPerformanceCounterDescriptionFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterDescriptionFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_PerformanceQuery`](Extension::KHR_PerformanceQuery)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkPerformanceCounterDescriptionFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum PerformanceCounterDescriptionFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_PerformanceQuery`](Extension::KHR_PerformanceQuery)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR")]
        #[default]
        PERFORMANCE_IMPACTING_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_PerformanceQuery`](Extension::KHR_PerformanceQuery)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR")]
        CONCURRENTLY_IMPACTED_KHR = 2,
    }
}
impl PerformanceCounterDescriptionFlagKHR {}
/// [`VkPerformanceCounterDescriptionFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterDescriptionFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_PerformanceQuery`](Extension::KHR_PerformanceQuery)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkPerformanceCounterDescriptionFlagsKHR")]
pub type PerformanceCounterDescriptionFlagsKHR = FlagSet<PerformanceCounterDescriptionFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkAcquireProfilingLockFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAcquireProfilingLockFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_PerformanceQuery`](Extension::KHR_PerformanceQuery)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkAcquireProfilingLockFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum AcquireProfilingLockFlagKHR: u32 {
        #[default]
        #[doc(hidden)]
        __PLACEHOLDER = 0,
    }
}
/// [`VkAcquireProfilingLockFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAcquireProfilingLockFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_PerformanceQuery`](Extension::KHR_PerformanceQuery)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkAcquireProfilingLockFlagsKHR")]
pub type AcquireProfilingLockFlagsKHR = FlagSet<AcquireProfilingLockFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoEncodeFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodeFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoEncodeFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeIntraRefresh`](Extension::KHR_VideoEncodeIntraRefresh)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_INTRA_REFRESH_BIT_KHR")]
        #[default]
        INTRA_REFRESH_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extension::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_WITH_QUANTIZATION_DELTA_MAP_BIT_KHR")]
        WITH_QUANTIZATION_DELTA_MAP_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extension::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_WITH_EMPHASIS_MAP_BIT_KHR")]
        WITH_EMPHASIS_MAP_KHR = 2,
    }
}
/// [`VkVideoEncodeFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkVideoEncodeFlagsKHR")]
pub type VideoEncodeFlagsKHR = FlagSet<VideoEncodeFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoEncodeCapabilityFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeCapabilityFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkVideoEncodeCapabilityFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoEncodeCapabilityFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_CAPABILITY_PRECEDING_EXTERNALLY_ENCODED_BYTES_BIT_KHR")]
        #[default]
        PRECEDING_EXTERNALLY_ENCODED_BYTES_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_CAPABILITY_INSUFFICIENT_BITSTREAM_BUFFER_RANGE_DETECTION_BIT_KHR")]
        INSUFFICIENT_BITSTREAM_BUFFER_RANGE_DETECTION_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extension::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_CAPABILITY_QUANTIZATION_DELTA_MAP_BIT_KHR")]
        QUANTIZATION_DELTA_MAP_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQuantizationMap`](Extension::KHR_VideoEncodeQuantizationMap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_CAPABILITY_EMPHASIS_MAP_BIT_KHR")]
        EMPHASIS_MAP_KHR = 8,
    }
}
/// [`VkVideoEncodeCapabilityFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeCapabilityFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkVideoEncodeCapabilityFlagsKHR")]
pub type VideoEncodeCapabilityFlagsKHR = FlagSet<VideoEncodeCapabilityFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoEncodeRateControlModeFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRateControlModeFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodeRateControlModeFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoEncodeRateControlModeFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RATE_CONTROL_MODE_DEFAULT_KHR")]
        #[default]
        DEFAULT_KHR = 0,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RATE_CONTROL_MODE_DISABLED_BIT_KHR")]
        DISABLED_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RATE_CONTROL_MODE_CBR_BIT_KHR")]
        CBR_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RATE_CONTROL_MODE_VBR_BIT_KHR")]
        VBR_KHR = 4,
    }
}
/// [`VkVideoEncodeRateControlModeFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRateControlModeFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkVideoEncodeRateControlModeFlagsKHR")]
pub type VideoEncodeRateControlModeFlagsKHR = FlagSet<VideoEncodeRateControlModeFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoEncodeFeedbackFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeFeedbackFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodeFeedbackFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoEncodeFeedbackFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_BITSTREAM_BUFFER_OFFSET_BIT_KHR")]
        #[default]
        BITSTREAM_BUFFER_OFFSET_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_BITSTREAM_BYTES_WRITTEN_BIT_KHR")]
        BITSTREAM_BYTES_WRITTEN_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_BITSTREAM_HAS_OVERRIDES_BIT_KHR")]
        BITSTREAM_HAS_OVERRIDES_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeFeedback2`](Extension::KHR_VideoEncodeFeedback2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_AVERAGE_QUANTIZATION_BIT_KHR")]
        AVERAGE_QUANTIZATION_KHR = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeFeedback2`](Extension::KHR_VideoEncodeFeedback2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_MIN_QUANTIZATION_BIT_KHR")]
        MIN_QUANTIZATION_KHR = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeFeedback2`](Extension::KHR_VideoEncodeFeedback2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_MAX_QUANTIZATION_BIT_KHR")]
        MAX_QUANTIZATION_KHR = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeFeedback2`](Extension::KHR_VideoEncodeFeedback2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_INTRA_PIXELS_BIT_KHR")]
        INTRA_PIXELS_KHR = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeFeedback2`](Extension::KHR_VideoEncodeFeedback2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_INTER_PIXELS_BIT_KHR")]
        INTER_PIXELS_KHR = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeFeedback2`](Extension::KHR_VideoEncodeFeedback2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_SKIPPED_PIXELS_BIT_KHR")]
        SKIPPED_PIXELS_KHR = 256,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeFeedback2`](Extension::KHR_VideoEncodeFeedback2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_FEEDBACK_PICTURE_PARTITION_COUNT_BIT_KHR")]
        PICTURE_PARTITION_COUNT_KHR = 512,
    }
}
/// [`VkVideoEncodeFeedbackFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeFeedbackFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkVideoEncodeFeedbackFlagsKHR")]
pub type VideoEncodeFeedbackFlagsKHR = FlagSet<VideoEncodeFeedbackFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoEncodeUsageFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeUsageFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodeUsageFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoEncodeUsageFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_USAGE_DEFAULT_KHR")]
        #[default]
        DEFAULT_KHR = 0,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_USAGE_TRANSCODING_BIT_KHR")]
        TRANSCODING_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_USAGE_STREAMING_BIT_KHR")]
        STREAMING_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_USAGE_RECORDING_BIT_KHR")]
        RECORDING_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_USAGE_CONFERENCING_BIT_KHR")]
        CONFERENCING_KHR = 8,
    }
}
/// [`VkVideoEncodeUsageFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeUsageFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkVideoEncodeUsageFlagsKHR")]
pub type VideoEncodeUsageFlagsKHR = FlagSet<VideoEncodeUsageFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoEncodeContentFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeContentFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodeContentFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoEncodeContentFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_CONTENT_DEFAULT_KHR")]
        #[default]
        DEFAULT_KHR = 0,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_CONTENT_CAMERA_BIT_KHR")]
        CAMERA_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_CONTENT_DESKTOP_BIT_KHR")]
        DESKTOP_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_CONTENT_RENDERED_BIT_KHR")]
        RENDERED_KHR = 4,
    }
}
/// [`VkVideoEncodeContentFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeContentFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkVideoEncodeContentFlagsKHR")]
pub type VideoEncodeContentFlagsKHR = FlagSet<VideoEncodeContentFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkAddressCommandFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAddressCommandFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_DeviceAddressCommands`](Extension::KHR_DeviceAddressCommands)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkAddressCommandFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum AddressCommandFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_DeviceAddressCommands`](Extension::KHR_DeviceAddressCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ADDRESS_COMMAND_PROTECTED_BIT_KHR")]
        #[default]
        PROTECTED_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_DeviceAddressCommands`](Extension::KHR_DeviceAddressCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ADDRESS_COMMAND_FULLY_BOUND_BIT_KHR")]
        FULLY_BOUND_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_DeviceAddressCommands`](Extension::KHR_DeviceAddressCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ADDRESS_COMMAND_STORAGE_BUFFER_USAGE_BIT_KHR")]
        STORAGE_BUFFER_USAGE_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_DeviceAddressCommands`](Extension::KHR_DeviceAddressCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ADDRESS_COMMAND_UNKNOWN_STORAGE_BUFFER_USAGE_BIT_KHR")]
        UNKNOWN_STORAGE_BUFFER_USAGE_KHR = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_DeviceAddressCommands`](Extension::KHR_DeviceAddressCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ADDRESS_COMMAND_TRANSFORM_FEEDBACK_BUFFER_USAGE_BIT_KHR")]
        TRANSFORM_FEEDBACK_BUFFER_USAGE_KHR = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_DeviceAddressCommands`](Extension::KHR_DeviceAddressCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ADDRESS_COMMAND_UNKNOWN_TRANSFORM_FEEDBACK_BUFFER_USAGE_BIT_KHR")]
        UNKNOWN_TRANSFORM_FEEDBACK_BUFFER_USAGE_KHR = 32,
    }
}
/// [`VkAddressCommandFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAddressCommandFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_DeviceAddressCommands`](Extension::KHR_DeviceAddressCommands)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkAddressCommandFlagsKHR")]
pub type AddressCommandFlagsKHR = FlagSet<AddressCommandFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkConditionalRenderingFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkConditionalRenderingFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_ConditionalRendering`](Extension::EXT_ConditionalRendering)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkConditionalRenderingFlagBitsEXT")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ConditionalRenderingFlagEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ConditionalRendering`](Extension::EXT_ConditionalRendering)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CONDITIONAL_RENDERING_INVERTED_BIT_EXT")]
        #[default]
        INVERTED_EXT = 1,
    }
}
/// [`VkConditionalRenderingFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkConditionalRenderingFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_DeviceAddressCommands`](Extension::KHR_DeviceAddressCommands)
/// - Extension [`EXT_ConditionalRendering`](Extension::EXT_ConditionalRendering)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkConditionalRenderingFlagsEXT")]
pub type ConditionalRenderingFlagsEXT = FlagSet<ConditionalRenderingFlagEXT>;

crate::__vkx_internal_flags! {
    /// [`VkAccelerationStructureCreateFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureCreateFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkAccelerationStructureCreateFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum AccelerationStructureCreateFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR")]
        #[default]
        DEVICE_ADDRESS_CAPTURE_REPLAY_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorBuffer`](Extension::EXT_DescriptorBuffer)
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCELERATION_STRUCTURE_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT")]
        DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracingMotionBlur`](Extension::NV_RayTracingMotionBlur)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV")]
        MOTION_NV = 4,
    }
}
/// [`VkAccelerationStructureCreateFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureCreateFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_DeviceAddressCommands`](Extension::KHR_DeviceAddressCommands)
/// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkAccelerationStructureCreateFlagsKHR")]
pub type AccelerationStructureCreateFlagsKHR = FlagSet<AccelerationStructureCreateFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkPresentScalingFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentScalingFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_SurfaceMaintenance1`](Extension::KHR_SurfaceMaintenance1)
    /// - Extension [`EXT_SurfaceMaintenance1`](Extension::EXT_SurfaceMaintenance1)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkPresentScalingFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum PresentScalingFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_SurfaceMaintenance1`](Extension::KHR_SurfaceMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRESENT_SCALING_ONE_TO_ONE_BIT_KHR")]
        #[default]
        ONE_TO_ONE_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_SurfaceMaintenance1`](Extension::KHR_SurfaceMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRESENT_SCALING_ASPECT_RATIO_STRETCH_BIT_KHR")]
        ASPECT_RATIO_STRETCH_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_SurfaceMaintenance1`](Extension::KHR_SurfaceMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRESENT_SCALING_STRETCH_BIT_KHR")]
        STRETCH_KHR = 4,
    }
}
/// [`VkPresentScalingFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentScalingFlagBitsEXT.html)
///
#[doc(alias = "VkPresentScalingFlagBitsEXT")]
pub type PresentScalingFlagEXT = PresentScalingFlagKHR;
impl PresentScalingFlagKHR {
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
/// [`VkPresentScalingFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentScalingFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_SurfaceMaintenance1`](Extension::KHR_SurfaceMaintenance1)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkPresentScalingFlagsKHR")]
pub type PresentScalingFlagsKHR = FlagSet<PresentScalingFlagKHR>;
/// [`VkPresentScalingFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentScalingFlagsEXT.html)
///
#[doc(alias = "VkPresentScalingFlagsEXT")]
pub type PresentScalingFlagsEXT = PresentScalingFlagsKHR;

crate::__vkx_internal_flags! {
    /// [`VkPresentGravityFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentGravityFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_SurfaceMaintenance1`](Extension::KHR_SurfaceMaintenance1)
    /// - Extension [`EXT_SurfaceMaintenance1`](Extension::EXT_SurfaceMaintenance1)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkPresentGravityFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum PresentGravityFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_SurfaceMaintenance1`](Extension::KHR_SurfaceMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRESENT_GRAVITY_MIN_BIT_KHR")]
        #[default]
        MIN_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_SurfaceMaintenance1`](Extension::KHR_SurfaceMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRESENT_GRAVITY_MAX_BIT_KHR")]
        MAX_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_SurfaceMaintenance1`](Extension::KHR_SurfaceMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRESENT_GRAVITY_CENTERED_BIT_KHR")]
        CENTERED_KHR = 4,
    }
}
/// [`VkPresentGravityFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentGravityFlagBitsEXT.html)
///
#[doc(alias = "VkPresentGravityFlagBitsEXT")]
pub type PresentGravityFlagEXT = PresentGravityFlagKHR;
impl PresentGravityFlagKHR {
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
/// [`VkPresentGravityFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentGravityFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_SurfaceMaintenance1`](Extension::KHR_SurfaceMaintenance1)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkPresentGravityFlagsKHR")]
pub type PresentGravityFlagsKHR = FlagSet<PresentGravityFlagKHR>;
/// [`VkPresentGravityFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentGravityFlagsEXT.html)
///
#[doc(alias = "VkPresentGravityFlagsEXT")]
pub type PresentGravityFlagsEXT = PresentGravityFlagsKHR;

crate::__vkx_internal_flags! {
    /// [`VkVideoEncodeAV1CapabilityFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1CapabilityFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeAv1`](Extension::KHR_VideoEncodeAv1)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkVideoEncodeAV1CapabilityFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoEncodeAV1CapabilityFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extension::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_CAPABILITY_PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_BIT_KHR")]
        #[default]
        VIDEO_ENCODE_AV1_CAPABILITY_PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extension::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_CAPABILITY_GENERATE_OBU_EXTENSION_HEADER_BIT_KHR")]
        VIDEO_ENCODE_AV1_CAPABILITY_GENERATE_OBU_EXTENSION_HEADER_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extension::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_CAPABILITY_PRIMARY_REFERENCE_CDF_ONLY_BIT_KHR")]
        VIDEO_ENCODE_AV1_CAPABILITY_PRIMARY_REFERENCE_CDF_ONLY_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extension::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_CAPABILITY_FRAME_SIZE_OVERRIDE_BIT_KHR")]
        VIDEO_ENCODE_AV1_CAPABILITY_FRAME_SIZE_OVERRIDE_KHR = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extension::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_CAPABILITY_MOTION_VECTOR_SCALING_BIT_KHR")]
        VIDEO_ENCODE_AV1_CAPABILITY_MOTION_VECTOR_SCALING_KHR = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeIntraRefresh`](Extension::KHR_VideoEncodeIntraRefresh)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_CAPABILITY_COMPOUND_PREDICTION_INTRA_REFRESH_BIT_KHR")]
        VIDEO_ENCODE_AV1_CAPABILITY_COMPOUND_PREDICTION_INTRA_REFRESH_KHR = 32,
    }
}
/// [`VkVideoEncodeAV1CapabilityFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1CapabilityFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoEncodeAv1`](Extension::KHR_VideoEncodeAv1)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkVideoEncodeAV1CapabilityFlagsKHR")]
pub type VideoEncodeAV1CapabilityFlagsKHR = FlagSet<VideoEncodeAV1CapabilityFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoEncodeAV1StdFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1StdFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeAv1`](Extension::KHR_VideoEncodeAv1)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkVideoEncodeAV1StdFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoEncodeAV1StdFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extension::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_STD_UNIFORM_TILE_SPACING_FLAG_SET_BIT_KHR")]
        #[default]
        VIDEO_ENCODE_AV1_STD_UNIFORM_TILE_SPACING_FLAG_SET_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extension::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_STD_SKIP_MODE_PRESENT_UNSET_BIT_KHR")]
        VIDEO_ENCODE_AV1_STD_SKIP_MODE_PRESENT_UNSET_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extension::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_STD_PRIMARY_REF_FRAME_BIT_KHR")]
        VIDEO_ENCODE_AV1_STD_PRIMARY_REF_FRAME_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extension::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_STD_DELTA_Q_BIT_KHR")]
        VIDEO_ENCODE_AV1_STD_DELTA_Q_KHR = 8,
    }
}
/// [`VkVideoEncodeAV1StdFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1StdFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoEncodeAv1`](Extension::KHR_VideoEncodeAv1)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkVideoEncodeAV1StdFlagsKHR")]
pub type VideoEncodeAV1StdFlagsKHR = FlagSet<VideoEncodeAV1StdFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoEncodeAV1SuperblockSizeFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1SuperblockSizeFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeAv1`](Extension::KHR_VideoEncodeAv1)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkVideoEncodeAV1SuperblockSizeFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoEncodeAV1SuperblockSizeFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extension::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_64_BIT_KHR")]
        #[default]
        VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_64_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extension::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_128_BIT_KHR")]
        VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_128_KHR = 2,
    }
}
/// [`VkVideoEncodeAV1SuperblockSizeFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1SuperblockSizeFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoEncodeAv1`](Extension::KHR_VideoEncodeAv1)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkVideoEncodeAV1SuperblockSizeFlagsKHR")]
pub type VideoEncodeAV1SuperblockSizeFlagsKHR = FlagSet<VideoEncodeAV1SuperblockSizeFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoEncodeAV1RateControlFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1RateControlFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeAv1`](Extension::KHR_VideoEncodeAv1)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodeAV1RateControlFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoEncodeAV1RateControlFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extension::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_RATE_CONTROL_REGULAR_GOP_BIT_KHR")]
        #[default]
        VIDEO_ENCODE_AV1_RATE_CONTROL_REGULAR_GOP_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extension::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_BIT_KHR")]
        VIDEO_ENCODE_AV1_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extension::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR")]
        VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_FLAT_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeAv1`](Extension::KHR_VideoEncodeAv1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR")]
        VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_KHR = 8,
    }
}
/// [`VkVideoEncodeAV1RateControlFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1RateControlFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoEncodeAv1`](Extension::KHR_VideoEncodeAv1)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkVideoEncodeAV1RateControlFlagsKHR")]
pub type VideoEncodeAV1RateControlFlagsKHR = FlagSet<VideoEncodeAV1RateControlFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkAddressCopyFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAddressCopyFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_CopyMemoryIndirect`](Extension::KHR_CopyMemoryIndirect)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkAddressCopyFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum AddressCopyFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_CopyMemoryIndirect`](Extension::KHR_CopyMemoryIndirect)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ADDRESS_COPY_DEVICE_LOCAL_BIT_KHR")]
        #[default]
        DEVICE_LOCAL_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_CopyMemoryIndirect`](Extension::KHR_CopyMemoryIndirect)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ADDRESS_COPY_SPARSE_BIT_KHR")]
        SPARSE_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_CopyMemoryIndirect`](Extension::KHR_CopyMemoryIndirect)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ADDRESS_COPY_PROTECTED_BIT_KHR")]
        PROTECTED_KHR = 4,
    }
}
/// [`VkAddressCopyFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAddressCopyFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_CopyMemoryIndirect`](Extension::KHR_CopyMemoryIndirect)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkAddressCopyFlagsKHR")]
pub type AddressCopyFlagsKHR = FlagSet<AddressCopyFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoEncodeIntraRefreshModeFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeIntraRefreshModeFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeIntraRefresh`](Extension::KHR_VideoEncodeIntraRefresh)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodeIntraRefreshModeFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoEncodeIntraRefreshModeFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeIntraRefresh`](Extension::KHR_VideoEncodeIntraRefresh)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_NONE_KHR")]
        #[default]
        NONE_KHR = 0,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeIntraRefresh`](Extension::KHR_VideoEncodeIntraRefresh)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_PER_PICTURE_PARTITION_BIT_KHR")]
        PER_PICTURE_PARTITION_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeIntraRefresh`](Extension::KHR_VideoEncodeIntraRefresh)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_BLOCK_BASED_BIT_KHR")]
        BLOCK_BASED_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeIntraRefresh`](Extension::KHR_VideoEncodeIntraRefresh)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_BLOCK_ROW_BASED_BIT_KHR")]
        BLOCK_ROW_BASED_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeIntraRefresh`](Extension::KHR_VideoEncodeIntraRefresh)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_BLOCK_COLUMN_BASED_BIT_KHR")]
        BLOCK_COLUMN_BASED_KHR = 8,
    }
}
/// [`VkVideoEncodeIntraRefreshModeFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeIntraRefreshModeFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoEncodeIntraRefresh`](Extension::KHR_VideoEncodeIntraRefresh)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkVideoEncodeIntraRefreshModeFlagsKHR")]
pub type VideoEncodeIntraRefreshModeFlagsKHR = FlagSet<VideoEncodeIntraRefreshModeFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkDeviceFaultFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceFaultFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_DeviceFault`](Extension::KHR_DeviceFault)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkDeviceFaultFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum DeviceFaultFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_DeviceFault`](Extension::KHR_DeviceFault)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_FAULT_FLAG_DEVICE_LOST_KHR")]
        #[default]
        FLAG_DEVICE_LOST_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_DeviceFault`](Extension::KHR_DeviceFault)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_FAULT_FLAG_MEMORY_ADDRESS_KHR")]
        FLAG_MEMORY_ADDRESS_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_DeviceFault`](Extension::KHR_DeviceFault)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_FAULT_FLAG_INSTRUCTION_ADDRESS_KHR")]
        FLAG_INSTRUCTION_ADDRESS_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_DeviceFault`](Extension::KHR_DeviceFault)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_FAULT_FLAG_VENDOR_KHR")]
        FLAG_VENDOR_KHR = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_DeviceFault`](Extension::KHR_DeviceFault)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_FAULT_FLAG_WATCHDOG_TIMEOUT_KHR")]
        FLAG_WATCHDOG_TIMEOUT_KHR = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_DeviceFault`](Extension::KHR_DeviceFault)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_FAULT_FLAG_OVERFLOW_KHR")]
        FLAG_OVERFLOW_KHR = 32,
    }
}
/// [`VkDeviceFaultFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceFaultFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_DeviceFault`](Extension::KHR_DeviceFault)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkDeviceFaultFlagsKHR")]
pub type DeviceFaultFlagsKHR = FlagSet<DeviceFaultFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkAccessFlagBits3KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlagBits3KHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_Maintenance8`](Extension::KHR_Maintenance8)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkAccessFlagBits3KHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum AccessFlag3KHR: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance8`](Extension::KHR_Maintenance8)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_ACCESS_3_NONE_KHR")]
        #[default]
        NONE_KHR = 0,
    }
}
/// [`VkAccessFlags3KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlags3KHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_Maintenance8`](Extension::KHR_Maintenance8)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkAccessFlags3KHR")]
pub type AccessFlags3KHR = FlagSet<AccessFlag3KHR>;

crate::__vkx_internal_flags! {
    /// [`VkVideoEncodePerPartitionFeedbackFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodePerPartitionFeedbackFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_VideoEncodeFeedback2`](Extension::KHR_VideoEncodeFeedback2)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodePerPartitionFeedbackFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoEncodePerPartitionFeedbackFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeFeedback2`](Extension::KHR_VideoEncodeFeedback2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_STATUS_BIT_KHR")]
        #[default]
        STATUS_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeFeedback2`](Extension::KHR_VideoEncodeFeedback2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_BITSTREAM_BUFFER_OFFSET_BIT_KHR")]
        BITSTREAM_BUFFER_OFFSET_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_VideoEncodeFeedback2`](Extension::KHR_VideoEncodeFeedback2)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_BITSTREAM_BYTES_WRITTEN_BIT_KHR")]
        BITSTREAM_BYTES_WRITTEN_KHR = 4,
    }
}
/// [`VkVideoEncodePerPartitionFeedbackFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodePerPartitionFeedbackFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoEncodeFeedback2`](Extension::KHR_VideoEncodeFeedback2)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkVideoEncodePerPartitionFeedbackFlagsKHR")]
pub type VideoEncodePerPartitionFeedbackFlagsKHR = FlagSet<VideoEncodePerPartitionFeedbackFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkRenderingAttachmentFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingAttachmentFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_Maintenance10`](Extension::KHR_Maintenance10)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkRenderingAttachmentFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum RenderingAttachmentFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance10`](Extension::KHR_Maintenance10)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDERING_ATTACHMENT_INPUT_ATTACHMENT_FEEDBACK_BIT_KHR")]
        #[default]
        INPUT_ATTACHMENT_FEEDBACK_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance10`](Extension::KHR_Maintenance10)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDERING_ATTACHMENT_RESOLVE_SKIP_TRANSFER_FUNCTION_BIT_KHR")]
        RESOLVE_SKIP_TRANSFER_FUNCTION_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance10`](Extension::KHR_Maintenance10)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RENDERING_ATTACHMENT_RESOLVE_ENABLE_TRANSFER_FUNCTION_BIT_KHR")]
        RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR = 4,
    }
}
/// [`VkRenderingAttachmentFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingAttachmentFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_Maintenance10`](Extension::KHR_Maintenance10)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkRenderingAttachmentFlagsKHR")]
pub type RenderingAttachmentFlagsKHR = FlagSet<RenderingAttachmentFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkResolveImageFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkResolveImageFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_Maintenance10`](Extension::KHR_Maintenance10)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkResolveImageFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ResolveImageFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance10`](Extension::KHR_Maintenance10)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RESOLVE_IMAGE_SKIP_TRANSFER_FUNCTION_BIT_KHR")]
        #[default]
        SKIP_TRANSFER_FUNCTION_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance10`](Extension::KHR_Maintenance10)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_RESOLVE_IMAGE_ENABLE_TRANSFER_FUNCTION_BIT_KHR")]
        ENABLE_TRANSFER_FUNCTION_KHR = 2,
    }
}
/// [`VkResolveImageFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkResolveImageFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_Maintenance10`](Extension::KHR_Maintenance10)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkResolveImageFlagsKHR")]
pub type ResolveImageFlagsKHR = FlagSet<ResolveImageFlagKHR>;

crate::__vkx_internal_flags! {
    /// [`VkFormatFeatureFlagBits4KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlagBits4KHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkFormatFeatureFlagBits4KHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum FormatFeatureFlag4KHR: u64 {
        #[default]
        #[doc(hidden)]
        __PLACEHOLDER = 0,
    }
}
/// [`VkFormatFeatureFlags4KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlags4KHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkFormatFeatureFlags4KHR")]
pub type FormatFeatureFlags4KHR = FlagSet<FormatFeatureFlag4KHR>;

crate::__vkx_internal_flags! {
    /// [`VkImageUsageFlagBits2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageUsageFlagBits2KHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkImageUsageFlagBits2KHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ImageUsageFlag2KHR: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_TRANSFER_SRC_BIT_KHR")]
        #[default]
        TRANSFER_SRC_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_TRANSFER_DST_BIT_KHR")]
        TRANSFER_DST_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_SAMPLED_BIT_KHR")]
        SAMPLED_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_STORAGE_BIT_KHR")]
        STORAGE_KHR = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_COLOR_ATTACHMENT_BIT_KHR")]
        COLOR_ATTACHMENT_KHR = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_DEPTH_STENCIL_ATTACHMENT_BIT_KHR")]
        DEPTH_STENCIL_ATTACHMENT_KHR = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_TRANSIENT_ATTACHMENT_BIT_KHR")]
        TRANSIENT_ATTACHMENT_KHR = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_INPUT_ATTACHMENT_BIT_KHR")]
        INPUT_ATTACHMENT_KHR = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
        FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = 256,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_FRAGMENT_DENSITY_MAP_BIT_EXT")]
        FRAGMENT_DENSITY_MAP_EXT = 512,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_VIDEO_DECODE_DST_BIT_KHR")]
        VIDEO_DECODE_DST_KHR = 1024,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_VIDEO_DECODE_SRC_BIT_KHR")]
        VIDEO_DECODE_SRC_KHR = 2048,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_VIDEO_DECODE_DPB_BIT_KHR")]
        VIDEO_DECODE_DPB_KHR = 4096,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_VIDEO_ENCODE_DST_BIT_KHR")]
        VIDEO_ENCODE_DST_KHR = 8192,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_VIDEO_ENCODE_SRC_BIT_KHR")]
        VIDEO_ENCODE_SRC_KHR = 16384,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_VIDEO_ENCODE_DPB_BIT_KHR")]
        VIDEO_ENCODE_DPB_KHR = 32768,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_INVOCATION_MASK_BIT_HUAWEI")]
        INVOCATION_MASK_HUAWEI = 262144,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT")]
        ATTACHMENT_FEEDBACK_LOOP_EXT = 524288,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_SAMPLE_WEIGHT_BIT_QCOM")]
        SAMPLE_WEIGHT_QCOM = 1048576,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_SAMPLE_BLOCK_MATCH_BIT_QCOM")]
        SAMPLE_BLOCK_MATCH_QCOM = 2097152,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_HOST_TRANSFER_BIT_KHR")]
        HOST_TRANSFER_KHR = 4194304,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_TENSOR_ALIASING_BIT_ARM")]
        TENSOR_ALIASING_ARM = 8388608,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR")]
        VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR = 33554432,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR")]
        VIDEO_ENCODE_EMPHASIS_MAP_KHR = 67108864,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_USAGE_2_TILE_MEMORY_BIT_QCOM")]
        TILE_MEMORY_QCOM = 134217728,
    }
}
/// [`VkImageUsageFlags2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageUsageFlags2KHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkImageUsageFlags2KHR")]
pub type ImageUsageFlags2KHR = FlagSet<ImageUsageFlag2KHR>;

crate::__vkx_internal_flags! {
    /// [`VkImageCreateFlagBits2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCreateFlagBits2KHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkImageCreateFlagBits2KHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ImageCreateFlag2KHR: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_SPARSE_BINDING_BIT_KHR")]
        #[default]
        SPARSE_BINDING_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_SPARSE_RESIDENCY_BIT_KHR")]
        SPARSE_RESIDENCY_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_SPARSE_ALIASED_BIT_KHR")]
        SPARSE_ALIASED_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_MUTABLE_FORMAT_BIT_KHR")]
        MUTABLE_FORMAT_KHR = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_CUBE_COMPATIBLE_BIT_KHR")]
        CUBE_COMPATIBLE_KHR = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance11`](Extension::KHR_Maintenance11)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_ALIAS_SINGLE_LAYER_DESCRIPTOR_BIT_KHR")]
        ALIAS_SINGLE_LAYER_DESCRIPTOR_KHR = 4194304,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_2D_ARRAY_COMPATIBLE_BIT_KHR")]
        _2D_ARRAY_COMPATIBLE_KHR = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR")]
        SPLIT_INSTANCE_BIND_REGIONS_KHR = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR")]
        BLOCK_TEXEL_VIEW_COMPATIBLE_KHR = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_EXTENDED_USAGE_BIT_KHR")]
        EXTENDED_USAGE_KHR = 256,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_DISJOINT_BIT_KHR")]
        DISJOINT_KHR = 512,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_ALIAS_BIT_KHR")]
        ALIAS_KHR = 1024,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_PROTECTED_BIT_KHR")]
        PROTECTED_KHR = 2048,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT")]
        SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT = 4096,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_CORNER_SAMPLED_BIT_NV")]
        CORNER_SAMPLED_NV = 8192,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_SUBSAMPLED_BIT_EXT")]
        SUBSAMPLED_EXT = 16384,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_FRAGMENT_DENSITY_MAP_OFFSET_BIT_EXT")]
        FRAGMENT_DENSITY_MAP_OFFSET_EXT = 32768,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT")]
        DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT = 65536,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_2D_VIEW_COMPATIBLE_BIT_EXT")]
        _2D_VIEW_COMPATIBLE_EXT = 131072,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT")]
        MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT = 262144,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CREATE_2_VIDEO_PROFILE_INDEPENDENT_BIT_KHR")]
        VIDEO_PROFILE_INDEPENDENT_KHR = 1048576,
    }
}
/// [`VkImageCreateFlags2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCreateFlags2KHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_ExtendedFlags`](Extension::KHR_ExtendedFlags)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkImageCreateFlags2KHR")]
pub type ImageCreateFlags2KHR = FlagSet<ImageCreateFlag2KHR>;

crate::__vkx_internal_flags! {
    /// [`VkDebugReportFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugReportFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_DebugReport`](Extension::EXT_DebugReport)
    /// - Extension [`EXT_DebugUtils`](Extension::EXT_DebugUtils)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDebugReportFlagBitsEXT")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum DebugReportFlagEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DebugReport`](Extension::EXT_DebugReport)
        /// - Extension [`EXT_DebugUtils`](Extension::EXT_DebugUtils)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_REPORT_INFORMATION_BIT_EXT")]
        #[default]
        INFORMATION_EXT = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DebugReport`](Extension::EXT_DebugReport)
        /// - Extension [`EXT_DebugUtils`](Extension::EXT_DebugUtils)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_REPORT_WARNING_BIT_EXT")]
        WARNING_EXT = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DebugReport`](Extension::EXT_DebugReport)
        /// - Extension [`EXT_DebugUtils`](Extension::EXT_DebugUtils)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT")]
        PERFORMANCE_WARNING_EXT = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DebugReport`](Extension::EXT_DebugReport)
        /// - Extension [`EXT_DebugUtils`](Extension::EXT_DebugUtils)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_REPORT_ERROR_BIT_EXT")]
        ERROR_EXT = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DebugReport`](Extension::EXT_DebugReport)
        /// - Extension [`EXT_DebugUtils`](Extension::EXT_DebugUtils)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_REPORT_DEBUG_BIT_EXT")]
        DEBUG_EXT = 16,
    }
}
/// [`VkDebugReportFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugReportFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DebugReport`](Extension::EXT_DebugReport)
/// - Extension [`EXT_DebugUtils`](Extension::EXT_DebugUtils)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkDebugReportFlagsEXT")]
pub type DebugReportFlagsEXT = FlagSet<DebugReportFlagEXT>;

crate::__vkx_internal_flags! {
    /// [`VkExternalMemoryHandleTypeFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryHandleTypeFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_ExternalMemoryCapabilities`](Extension::NV_ExternalMemoryCapabilities)
    /// - Extension [`KHR_ExternalMemoryCapabilities`](Extension::KHR_ExternalMemoryCapabilities)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkExternalMemoryHandleTypeFlagBitsNV")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ExternalMemoryHandleTypeFlagNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ExternalMemoryCapabilities`](Extension::NV_ExternalMemoryCapabilities)
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extension::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV")]
        #[default]
        OPAQUE_WIN32_NV = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ExternalMemoryCapabilities`](Extension::NV_ExternalMemoryCapabilities)
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extension::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV")]
        OPAQUE_WIN32_KMT_NV = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ExternalMemoryCapabilities`](Extension::NV_ExternalMemoryCapabilities)
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extension::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV")]
        D3D11_IMAGE_NV = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ExternalMemoryCapabilities`](Extension::NV_ExternalMemoryCapabilities)
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extension::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV")]
        D3D11_IMAGE_KMT_NV = 8,
    }
}
/// [`VkExternalMemoryHandleTypeFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryHandleTypeFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_ExternalMemoryCapabilities`](Extension::NV_ExternalMemoryCapabilities)
/// - Extension [`KHR_ExternalMemoryCapabilities`](Extension::KHR_ExternalMemoryCapabilities)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkExternalMemoryHandleTypeFlagsNV")]
pub type ExternalMemoryHandleTypeFlagsNV = FlagSet<ExternalMemoryHandleTypeFlagNV>;

crate::__vkx_internal_flags! {
    /// [`VkExternalMemoryFeatureFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryFeatureFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_ExternalMemoryCapabilities`](Extension::NV_ExternalMemoryCapabilities)
    /// - Extension [`KHR_ExternalMemoryCapabilities`](Extension::KHR_ExternalMemoryCapabilities)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkExternalMemoryFeatureFlagBitsNV")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ExternalMemoryFeatureFlagNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ExternalMemoryCapabilities`](Extension::NV_ExternalMemoryCapabilities)
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extension::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV")]
        #[default]
        DEDICATED_ONLY_NV = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ExternalMemoryCapabilities`](Extension::NV_ExternalMemoryCapabilities)
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extension::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV")]
        EXPORTABLE_NV = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ExternalMemoryCapabilities`](Extension::NV_ExternalMemoryCapabilities)
        /// - Extension [`KHR_ExternalMemoryCapabilities`](Extension::KHR_ExternalMemoryCapabilities)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV")]
        IMPORTABLE_NV = 4,
    }
}
/// [`VkExternalMemoryFeatureFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryFeatureFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_ExternalMemoryCapabilities`](Extension::NV_ExternalMemoryCapabilities)
/// - Extension [`KHR_ExternalMemoryCapabilities`](Extension::KHR_ExternalMemoryCapabilities)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkExternalMemoryFeatureFlagsNV")]
pub type ExternalMemoryFeatureFlagsNV = FlagSet<ExternalMemoryFeatureFlagNV>;

crate::__vkx_internal_flags! {
    /// [`VkSurfaceCounterFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCounterFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_DisplaySurfaceCounter`](Extension::EXT_DisplaySurfaceCounter)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkSurfaceCounterFlagBitsEXT")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum SurfaceCounterFlagEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DisplaySurfaceCounter`](Extension::EXT_DisplaySurfaceCounter)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SURFACE_COUNTER_VBLANK_BIT_EXT")]
        #[default]
        VBLANK_EXT = 1,
    }
}
impl SurfaceCounterFlagEXT {}
/// [`VkSurfaceCounterFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCounterFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DisplaySurfaceCounter`](Extension::EXT_DisplaySurfaceCounter)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkSurfaceCounterFlagsEXT")]
pub type SurfaceCounterFlagsEXT = FlagSet<SurfaceCounterFlagEXT>;

crate::__vkx_internal_flags! {
    /// [`VkDebugUtilsMessageSeverityFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessageSeverityFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_DebugUtils`](Extension::EXT_DebugUtils)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDebugUtilsMessageSeverityFlagBitsEXT")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum DebugUtilsMessageSeverityFlagEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DebugUtils`](Extension::EXT_DebugUtils)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT")]
        #[default]
        VERBOSE_EXT = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DebugUtils`](Extension::EXT_DebugUtils)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT")]
        INFO_EXT = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DebugUtils`](Extension::EXT_DebugUtils)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT")]
        WARNING_EXT = 256,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DebugUtils`](Extension::EXT_DebugUtils)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT")]
        ERROR_EXT = 4096,
    }
}
/// [`VkDebugUtilsMessageSeverityFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessageSeverityFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DebugUtils`](Extension::EXT_DebugUtils)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkDebugUtilsMessageSeverityFlagsEXT")]
pub type DebugUtilsMessageSeverityFlagsEXT = FlagSet<DebugUtilsMessageSeverityFlagEXT>;

crate::__vkx_internal_flags! {
    /// [`VkDebugUtilsMessageTypeFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessageTypeFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_DebugUtils`](Extension::EXT_DebugUtils)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDebugUtilsMessageTypeFlagBitsEXT")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum DebugUtilsMessageTypeFlagEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DebugUtils`](Extension::EXT_DebugUtils)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT")]
        #[default]
        GENERAL_EXT = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DebugUtils`](Extension::EXT_DebugUtils)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT")]
        VALIDATION_EXT = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DebugUtils`](Extension::EXT_DebugUtils)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT")]
        PERFORMANCE_EXT = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DeviceAddressBindingReport`](Extension::EXT_DeviceAddressBindingReport)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_TYPE_DEVICE_ADDRESS_BINDING_BIT_EXT")]
        DEVICE_ADDRESS_BINDING_EXT = 8,
    }
}
/// [`VkDebugUtilsMessageTypeFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessageTypeFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DebugUtils`](Extension::EXT_DebugUtils)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkDebugUtilsMessageTypeFlagsEXT")]
pub type DebugUtilsMessageTypeFlagsEXT = FlagSet<DebugUtilsMessageTypeFlagEXT>;

crate::__vkx_internal_flags! {
    /// [`VkGpaSqShaderStageFlagBitsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaSqShaderStageFlagBitsAMD.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`AMD_GpaInterface`](Extension::AMD_GpaInterface)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkGpaSqShaderStageFlagBitsAMD")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum GpaSqShaderStageFlagAMD: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMD_GpaInterface`](Extension::AMD_GpaInterface)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GPA_SQ_SHADER_STAGE_PS_BIT_AMD")]
        #[default]
        PS_AMD = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMD_GpaInterface`](Extension::AMD_GpaInterface)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GPA_SQ_SHADER_STAGE_VS_BIT_AMD")]
        VS_AMD = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMD_GpaInterface`](Extension::AMD_GpaInterface)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GPA_SQ_SHADER_STAGE_GS_BIT_AMD")]
        GS_AMD = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMD_GpaInterface`](Extension::AMD_GpaInterface)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GPA_SQ_SHADER_STAGE_ES_BIT_AMD")]
        ES_AMD = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMD_GpaInterface`](Extension::AMD_GpaInterface)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GPA_SQ_SHADER_STAGE_HS_BIT_AMD")]
        HS_AMD = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMD_GpaInterface`](Extension::AMD_GpaInterface)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GPA_SQ_SHADER_STAGE_LS_BIT_AMD")]
        LS_AMD = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`AMD_GpaInterface`](Extension::AMD_GpaInterface)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GPA_SQ_SHADER_STAGE_CS_BIT_AMD")]
        CS_AMD = 64,
    }
}
/// [`VkGpaSqShaderStageFlagsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaSqShaderStageFlagsAMD.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`AMD_GpaInterface`](Extension::AMD_GpaInterface)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkGpaSqShaderStageFlagsAMD")]
pub type GpaSqShaderStageFlagsAMD = FlagSet<GpaSqShaderStageFlagAMD>;

crate::__vkx_internal_flags! {
    /// [`VkTensorViewCreateFlagBitsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorViewCreateFlagBitsARM.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
    /// - Extension [`ARM_Tensors`](Extension::ARM_Tensors)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkTensorViewCreateFlagBitsARM")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum TensorViewCreateFlagARM: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_Tensors`](Extension::ARM_Tensors)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TENSOR_VIEW_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_ARM")]
        #[default]
        DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM = 1,
    }
}
/// [`VkTensorViewCreateFlagsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorViewCreateFlagsARM.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
/// - Extension [`ARM_Tensors`](Extension::ARM_Tensors)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkTensorViewCreateFlagsARM")]
pub type TensorViewCreateFlagsARM = FlagSet<TensorViewCreateFlagARM>;

crate::__vkx_internal_flags! {
    /// [`VkSpirvResourceTypeFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSpirvResourceTypeFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkSpirvResourceTypeFlagBitsEXT")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum SpirvResourceTypeFlagEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_SAMPLER_BIT_EXT")]
        #[default]
        SAMPLER_EXT = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_SAMPLED_IMAGE_BIT_EXT")]
        SAMPLED_IMAGE_EXT = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_READ_ONLY_IMAGE_BIT_EXT")]
        READ_ONLY_IMAGE_EXT = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_READ_WRITE_IMAGE_BIT_EXT")]
        READ_WRITE_IMAGE_EXT = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_COMBINED_SAMPLED_IMAGE_BIT_EXT")]
        COMBINED_SAMPLED_IMAGE_EXT = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_UNIFORM_BUFFER_BIT_EXT")]
        UNIFORM_BUFFER_EXT = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_READ_ONLY_STORAGE_BUFFER_BIT_EXT")]
        READ_ONLY_STORAGE_BUFFER_EXT = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_READ_WRITE_STORAGE_BUFFER_BIT_EXT")]
        READ_WRITE_STORAGE_BUFFER_EXT = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_ACCELERATION_STRUCTURE_BIT_EXT")]
        ACCELERATION_STRUCTURE_EXT = 256,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_TENSOR_BIT_ARM")]
        TENSOR_ARM = 512,
    }
}
impl SpirvResourceTypeFlagEXT {
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VK_SPIRV_RESOURCE_TYPE_ALL_EXT")]
    pub const ALL_EXT: FlagSet<Self> = FlagSet(2147483647);
}
/// [`VkSpirvResourceTypeFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSpirvResourceTypeFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkSpirvResourceTypeFlagsEXT")]
pub type SpirvResourceTypeFlagsEXT = FlagSet<SpirvResourceTypeFlagEXT>;

crate::__vkx_internal_flags! {
    /// [`VkGeometryFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
    /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
    /// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkGeometryFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum GeometryFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GEOMETRY_OPAQUE_BIT_KHR")]
        #[default]
        OPAQUE_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR")]
        NO_DUPLICATE_ANY_HIT_INVOCATION_KHR = 2,
    }
}
/// [`VkGeometryFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryFlagBitsNV.html)
///
#[doc(alias = "VkGeometryFlagBitsNV")]
pub type GeometryFlagNV = GeometryFlagKHR;
impl GeometryFlagKHR {
    /// [`VK_GEOMETRY_OPAQUE_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GEOMETRY_OPAQUE_BIT_NV.html)
    ///
    #[doc(alias = "VK_GEOMETRY_OPAQUE_BIT_NV")]
    pub const OPAQUE_NV: Self = Self::OPAQUE_KHR;
    /// [`VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_NV.html)
    ///
    #[doc(alias = "VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_NV")]
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION_NV: Self = Self::NO_DUPLICATE_ANY_HIT_INVOCATION_KHR;
}
/// [`VkGeometryFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
/// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
/// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkGeometryFlagsKHR")]
pub type GeometryFlagsKHR = FlagSet<GeometryFlagKHR>;
/// [`VkGeometryFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryFlagsNV.html)
///
#[doc(alias = "VkGeometryFlagsNV")]
pub type GeometryFlagsNV = GeometryFlagsKHR;

crate::__vkx_internal_flags! {
    /// [`VkGeometryInstanceFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryInstanceFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
    /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
    /// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkGeometryInstanceFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum GeometryInstanceFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR")]
        #[default]
        TRIANGLE_FACING_CULL_DISABLE_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR")]
        TRIANGLE_FLIP_FACING_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR")]
        FORCE_OPAQUE_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR")]
        FORCE_NO_OPAQUE_KHR = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
        /// - Extension [`EXT_OpacityMicromap`](Extension::EXT_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_OPACITY_MICROMAP_2_STATE_BIT_KHR")]
        FORCE_OPACITY_MICROMAP_2_STATE_KHR = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
        /// - Extension [`EXT_OpacityMicromap`](Extension::EXT_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GEOMETRY_INSTANCE_DISABLE_OPACITY_MICROMAPS_BIT_KHR")]
        DISABLE_OPACITY_MICROMAPS_KHR = 32,
    }
}
/// [`VkGeometryInstanceFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryInstanceFlagBitsNV.html)
///
#[doc(alias = "VkGeometryInstanceFlagBitsNV")]
pub type GeometryInstanceFlagNV = GeometryInstanceFlagKHR;
impl GeometryInstanceFlagKHR {
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
/// [`VkGeometryInstanceFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryInstanceFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
/// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
/// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkGeometryInstanceFlagsKHR")]
pub type GeometryInstanceFlagsKHR = FlagSet<GeometryInstanceFlagKHR>;
/// [`VkGeometryInstanceFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryInstanceFlagsNV.html)
///
#[doc(alias = "VkGeometryInstanceFlagsNV")]
pub type GeometryInstanceFlagsNV = GeometryInstanceFlagsKHR;

crate::__vkx_internal_flags! {
    /// [`VkBuildAccelerationStructureFlagBitsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildAccelerationStructureFlagBitsKHR.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
    /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
    /// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkBuildAccelerationStructureFlagBitsKHR")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum BuildAccelerationStructureFlagKHR: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR")]
        #[default]
        ALLOW_UPDATE_KHR = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR")]
        ALLOW_COMPACTION_KHR = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR")]
        PREFER_FAST_TRACE_KHR = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR")]
        PREFER_FAST_BUILD_KHR = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
        /// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
        /// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR")]
        LOW_MEMORY_KHR = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_RayTracingMotionBlur`](Extension::NV_RayTracingMotionBlur)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV")]
        MOTION_NV = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_OpacityMicromap`](Extension::EXT_OpacityMicromap)
        /// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_OPACITY_MICROMAP_DATA_UPDATE_BIT_EXT")]
        ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT = 256,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DisplacementMicromap`](Extension::NV_DisplacementMicromap)
        /// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_DISPLACEMENT_MICROMAP_UPDATE_BIT_NV")]
        ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV = 512,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_RayTracingPositionFetch`](Extension::KHR_RayTracingPositionFetch)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_DATA_ACCESS_BIT_KHR")]
        ALLOW_DATA_ACCESS_KHR = 2048,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_CLUSTER_OPACITY_MICROMAPS_BIT_NV")]
        ALLOW_CLUSTER_OPACITY_MICROMAPS_NV = 4096,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
        /// - Extension [`EXT_OpacityMicromap`](Extension::EXT_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_OPACITY_MICROMAP_UPDATE_BIT_KHR")]
        ALLOW_OPACITY_MICROMAP_UPDATE_KHR = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
        /// - Extension [`EXT_OpacityMicromap`](Extension::EXT_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_DISABLE_OPACITY_MICROMAPS_BIT_KHR")]
        ALLOW_DISABLE_OPACITY_MICROMAPS_KHR = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_MICROMAP_LOSSY_BIT_KHR")]
        MICROMAP_LOSSY_KHR = 1024,
    }
}
/// [`VkBuildAccelerationStructureFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildAccelerationStructureFlagBitsNV.html)
///
#[doc(alias = "VkBuildAccelerationStructureFlagBitsNV")]
pub type BuildAccelerationStructureFlagNV = BuildAccelerationStructureFlagKHR;
impl BuildAccelerationStructureFlagKHR {
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
/// [`VkBuildAccelerationStructureFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildAccelerationStructureFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
/// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
/// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkBuildAccelerationStructureFlagsKHR")]
pub type BuildAccelerationStructureFlagsKHR = FlagSet<BuildAccelerationStructureFlagKHR>;
/// [`VkBuildAccelerationStructureFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildAccelerationStructureFlagsNV.html)
///
#[doc(alias = "VkBuildAccelerationStructureFlagsNV")]
pub type BuildAccelerationStructureFlagsNV = BuildAccelerationStructureFlagsKHR;

crate::__vkx_internal_flags! {
    /// [`VkPipelineCompilerControlFlagBitsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCompilerControlFlagBitsAMD.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`AMD_PipelineCompilerControl`](Extension::AMD_PipelineCompilerControl)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkPipelineCompilerControlFlagBitsAMD")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum PipelineCompilerControlFlagAMD: u32 {
        #[default]
        #[doc(hidden)]
        __PLACEHOLDER = 0,
    }
}
/// [`VkPipelineCompilerControlFlagsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCompilerControlFlagsAMD.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`AMD_PipelineCompilerControl`](Extension::AMD_PipelineCompilerControl)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkPipelineCompilerControlFlagsAMD")]
pub type PipelineCompilerControlFlagsAMD = FlagSet<PipelineCompilerControlFlagAMD>;

crate::__vkx_internal_flags! {
    /// [`VkPresentStageFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentStageFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_PresentTiming`](Extension::EXT_PresentTiming)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkPresentStageFlagBitsEXT")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum PresentStageFlagEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_PresentTiming`](Extension::EXT_PresentTiming)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRESENT_STAGE_QUEUE_OPERATIONS_END_BIT_EXT")]
        #[default]
        QUEUE_OPERATIONS_END_EXT = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_PresentTiming`](Extension::EXT_PresentTiming)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRESENT_STAGE_REQUEST_DEQUEUED_BIT_EXT")]
        REQUEST_DEQUEUED_EXT = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_PresentTiming`](Extension::EXT_PresentTiming)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRESENT_STAGE_IMAGE_FIRST_PIXEL_OUT_BIT_EXT")]
        IMAGE_FIRST_PIXEL_OUT_EXT = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_PresentTiming`](Extension::EXT_PresentTiming)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRESENT_STAGE_IMAGE_FIRST_PIXEL_VISIBLE_BIT_EXT")]
        IMAGE_FIRST_PIXEL_VISIBLE_EXT = 8,
    }
}
/// [`VkPresentStageFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentStageFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_PresentTiming`](Extension::EXT_PresentTiming)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkPresentStageFlagsEXT")]
pub type PresentStageFlagsEXT = FlagSet<PresentStageFlagEXT>;

crate::__vkx_internal_flags! {
    /// [`VkPastPresentationTimingFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPastPresentationTimingFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_PresentTiming`](Extension::EXT_PresentTiming)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkPastPresentationTimingFlagBitsEXT")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum PastPresentationTimingFlagEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_PresentTiming`](Extension::EXT_PresentTiming)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PAST_PRESENTATION_TIMING_ALLOW_PARTIAL_RESULTS_BIT_EXT")]
        #[default]
        ALLOW_PARTIAL_RESULTS_EXT = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_PresentTiming`](Extension::EXT_PresentTiming)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PAST_PRESENTATION_TIMING_ALLOW_OUT_OF_ORDER_RESULTS_BIT_EXT")]
        ALLOW_OUT_OF_ORDER_RESULTS_EXT = 2,
    }
}
/// [`VkPastPresentationTimingFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPastPresentationTimingFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_PresentTiming`](Extension::EXT_PresentTiming)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkPastPresentationTimingFlagsEXT")]
pub type PastPresentationTimingFlagsEXT = FlagSet<PastPresentationTimingFlagEXT>;

crate::__vkx_internal_flags! {
    /// [`VkPresentTimingInfoFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentTimingInfoFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_PresentTiming`](Extension::EXT_PresentTiming)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkPresentTimingInfoFlagBitsEXT")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum PresentTimingInfoFlagEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_PresentTiming`](Extension::EXT_PresentTiming)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRESENT_TIMING_INFO_PRESENT_AT_RELATIVE_TIME_BIT_EXT")]
        #[default]
        PRESENT_AT_RELATIVE_TIME_EXT = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_PresentTiming`](Extension::EXT_PresentTiming)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PRESENT_TIMING_INFO_PRESENT_AT_NEAREST_REFRESH_CYCLE_BIT_EXT")]
        PRESENT_AT_NEAREST_REFRESH_CYCLE_EXT = 2,
    }
}
/// [`VkPresentTimingInfoFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentTimingInfoFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_PresentTiming`](Extension::EXT_PresentTiming)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkPresentTimingInfoFlagsEXT")]
pub type PresentTimingInfoFlagsEXT = FlagSet<PresentTimingInfoFlagEXT>;

crate::__vkx_internal_flags! {
    /// [`VkShaderCorePropertiesFlagBitsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderCorePropertiesFlagBitsAMD.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`AMD_ShaderCoreProperties2`](Extension::AMD_ShaderCoreProperties2)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkShaderCorePropertiesFlagBitsAMD")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ShaderCorePropertiesFlagAMD: u32 {
        #[default]
        #[doc(hidden)]
        __PLACEHOLDER = 0,
    }
}
/// [`VkShaderCorePropertiesFlagsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderCorePropertiesFlagsAMD.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`AMD_ShaderCoreProperties2`](Extension::AMD_ShaderCoreProperties2)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkShaderCorePropertiesFlagsAMD")]
pub type ShaderCorePropertiesFlagsAMD = FlagSet<ShaderCorePropertiesFlagAMD>;

crate::__vkx_internal_flags! {
    /// [`VkIndirectStateFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectStateFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_DeviceGeneratedCommands`](Extension::NV_DeviceGeneratedCommands)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkIndirectStateFlagBitsNV")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum IndirectStateFlagNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceGeneratedCommands`](Extension::NV_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_INDIRECT_STATE_FLAG_FRONTFACE_BIT_NV")]
        #[default]
        FLAG_FRONTFACE_NV = 1,
    }
}
/// [`VkIndirectStateFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectStateFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_DeviceGeneratedCommands`](Extension::NV_DeviceGeneratedCommands)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkIndirectStateFlagsNV")]
pub type IndirectStateFlagsNV = FlagSet<IndirectStateFlagNV>;

crate::__vkx_internal_flags! {
    /// [`VkIndirectCommandsLayoutUsageFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutUsageFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_DeviceGeneratedCommands`](Extension::NV_DeviceGeneratedCommands)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkIndirectCommandsLayoutUsageFlagBitsNV")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum IndirectCommandsLayoutUsageFlagNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceGeneratedCommands`](Extension::NV_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV")]
        #[default]
        EXPLICIT_PREPROCESS_NV = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceGeneratedCommands`](Extension::NV_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV")]
        INDEXED_SEQUENCES_NV = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceGeneratedCommands`](Extension::NV_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV")]
        UNORDERED_SEQUENCES_NV = 4,
    }
}
/// [`VkIndirectCommandsLayoutUsageFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutUsageFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_DeviceGeneratedCommands`](Extension::NV_DeviceGeneratedCommands)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkIndirectCommandsLayoutUsageFlagsNV")]
pub type IndirectCommandsLayoutUsageFlagsNV = FlagSet<IndirectCommandsLayoutUsageFlagNV>;

crate::__vkx_internal_flags! {
    /// [`VkDeviceDiagnosticsConfigFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceDiagnosticsConfigFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_DeviceDiagnosticsConfig`](Extension::NV_DeviceDiagnosticsConfig)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDeviceDiagnosticsConfigFlagBitsNV")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum DeviceDiagnosticsConfigFlagNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceDiagnosticsConfig`](Extension::NV_DeviceDiagnosticsConfig)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_DEBUG_INFO_BIT_NV")]
        #[default]
        ENABLE_SHADER_DEBUG_INFO_NV = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceDiagnosticsConfig`](Extension::NV_DeviceDiagnosticsConfig)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_RESOURCE_TRACKING_BIT_NV")]
        ENABLE_RESOURCE_TRACKING_NV = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceDiagnosticsConfig`](Extension::NV_DeviceDiagnosticsConfig)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_AUTOMATIC_CHECKPOINTS_BIT_NV")]
        ENABLE_AUTOMATIC_CHECKPOINTS_NV = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_DeviceDiagnosticsConfig`](Extension::NV_DeviceDiagnosticsConfig)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_ERROR_REPORTING_BIT_NV")]
        ENABLE_SHADER_ERROR_REPORTING_NV = 8,
    }
}
/// [`VkDeviceDiagnosticsConfigFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceDiagnosticsConfigFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_DeviceDiagnosticsConfig`](Extension::NV_DeviceDiagnosticsConfig)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkDeviceDiagnosticsConfigFlagsNV")]
pub type DeviceDiagnosticsConfigFlagsNV = FlagSet<DeviceDiagnosticsConfigFlagNV>;

crate::__vkx_internal_flags! {
    /// [`VkTileShadingRenderPassFlagBitsQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkTileShadingRenderPassFlagBitsQCOM.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`QCOM_TileShading`](Extension::QCOM_TileShading)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkTileShadingRenderPassFlagBitsQCOM")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum TileShadingRenderPassFlagQCOM: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_TileShading`](Extension::QCOM_TileShading)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TILE_SHADING_RENDER_PASS_ENABLE_BIT_QCOM")]
        #[default]
        ENABLE_QCOM = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`QCOM_TileShading`](Extension::QCOM_TileShading)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TILE_SHADING_RENDER_PASS_PER_TILE_EXECUTION_BIT_QCOM")]
        PER_TILE_EXECUTION_QCOM = 2,
    }
}
/// [`VkTileShadingRenderPassFlagsQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkTileShadingRenderPassFlagsQCOM.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`QCOM_TileShading`](Extension::QCOM_TileShading)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkTileShadingRenderPassFlagsQCOM")]
pub type TileShadingRenderPassFlagsQCOM = FlagSet<TileShadingRenderPassFlagQCOM>;

crate::__vkx_internal_flags! {
    /// [`VkExportMetalObjectTypeFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExportMetalObjectTypeFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_MetalObjects`](Extension::EXT_MetalObjects)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkExportMetalObjectTypeFlagBitsEXT")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ExportMetalObjectTypeFlagEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MetalObjects`](Extension::EXT_MetalObjects)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXPORT_METAL_OBJECT_TYPE_METAL_DEVICE_BIT_EXT")]
        #[default]
        METAL_DEVICE_EXT = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MetalObjects`](Extension::EXT_MetalObjects)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXPORT_METAL_OBJECT_TYPE_METAL_COMMAND_QUEUE_BIT_EXT")]
        METAL_COMMAND_QUEUE_EXT = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MetalObjects`](Extension::EXT_MetalObjects)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXPORT_METAL_OBJECT_TYPE_METAL_BUFFER_BIT_EXT")]
        METAL_BUFFER_EXT = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MetalObjects`](Extension::EXT_MetalObjects)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXPORT_METAL_OBJECT_TYPE_METAL_TEXTURE_BIT_EXT")]
        METAL_TEXTURE_EXT = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MetalObjects`](Extension::EXT_MetalObjects)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXPORT_METAL_OBJECT_TYPE_METAL_IOSURFACE_BIT_EXT")]
        METAL_IOSURFACE_EXT = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MetalObjects`](Extension::EXT_MetalObjects)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_EXPORT_METAL_OBJECT_TYPE_METAL_SHARED_EVENT_BIT_EXT")]
        METAL_SHARED_EVENT_EXT = 32,
    }
}
/// [`VkExportMetalObjectTypeFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExportMetalObjectTypeFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_MetalObjects`](Extension::EXT_MetalObjects)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkExportMetalObjectTypeFlagsEXT")]
pub type ExportMetalObjectTypeFlagsEXT = FlagSet<ExportMetalObjectTypeFlagEXT>;

crate::__vkx_internal_flags! {
    /// [`VkGraphicsPipelineLibraryFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkGraphicsPipelineLibraryFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_GraphicsPipelineLibrary`](Extension::EXT_GraphicsPipelineLibrary)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkGraphicsPipelineLibraryFlagBitsEXT")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum GraphicsPipelineLibraryFlagEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_GraphicsPipelineLibrary`](Extension::EXT_GraphicsPipelineLibrary)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GRAPHICS_PIPELINE_LIBRARY_VERTEX_INPUT_INTERFACE_BIT_EXT")]
        #[default]
        VERTEX_INPUT_INTERFACE_EXT = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_GraphicsPipelineLibrary`](Extension::EXT_GraphicsPipelineLibrary)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GRAPHICS_PIPELINE_LIBRARY_PRE_RASTERIZATION_SHADERS_BIT_EXT")]
        PRE_RASTERIZATION_SHADERS_EXT = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_GraphicsPipelineLibrary`](Extension::EXT_GraphicsPipelineLibrary)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GRAPHICS_PIPELINE_LIBRARY_FRAGMENT_SHADER_BIT_EXT")]
        FRAGMENT_SHADER_EXT = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_GraphicsPipelineLibrary`](Extension::EXT_GraphicsPipelineLibrary)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_GRAPHICS_PIPELINE_LIBRARY_FRAGMENT_OUTPUT_INTERFACE_BIT_EXT")]
        FRAGMENT_OUTPUT_INTERFACE_EXT = 8,
    }
}
/// [`VkGraphicsPipelineLibraryFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkGraphicsPipelineLibraryFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_GraphicsPipelineLibrary`](Extension::EXT_GraphicsPipelineLibrary)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkGraphicsPipelineLibraryFlagsEXT")]
pub type GraphicsPipelineLibraryFlagsEXT = FlagSet<GraphicsPipelineLibraryFlagEXT>;

crate::__vkx_internal_flags! {
    /// [`VkImageCompressionFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCompressionFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkImageCompressionFlagBitsEXT")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ImageCompressionFlagEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_DEFAULT_EXT")]
        #[default]
        DEFAULT_EXT = 0,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_DEFAULT_EXT")]
        FIXED_RATE_DEFAULT_EXT = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_EXPLICIT_EXT")]
        FIXED_RATE_EXPLICIT_EXT = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_DISABLED_EXT")]
        DISABLED_EXT = 4,
    }
}
/// [`VkImageCompressionFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCompressionFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkImageCompressionFlagsEXT")]
pub type ImageCompressionFlagsEXT = FlagSet<ImageCompressionFlagEXT>;

crate::__vkx_internal_flags! {
    /// [`VkImageCompressionFixedRateFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCompressionFixedRateFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkImageCompressionFixedRateFlagBitsEXT")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ImageCompressionFixedRateFlagEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_NONE_EXT")]
        #[default]
        NONE_EXT = 0,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_1BPC_BIT_EXT")]
        _1BPC_EXT = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_2BPC_BIT_EXT")]
        _2BPC_EXT = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_3BPC_BIT_EXT")]
        _3BPC_EXT = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_4BPC_BIT_EXT")]
        _4BPC_EXT = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_5BPC_BIT_EXT")]
        _5BPC_EXT = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_6BPC_BIT_EXT")]
        _6BPC_EXT = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_7BPC_BIT_EXT")]
        _7BPC_EXT = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_8BPC_BIT_EXT")]
        _8BPC_EXT = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_9BPC_BIT_EXT")]
        _9BPC_EXT = 256,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_10BPC_BIT_EXT")]
        _10BPC_EXT = 512,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_11BPC_BIT_EXT")]
        _11BPC_EXT = 1024,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_12BPC_BIT_EXT")]
        _12BPC_EXT = 2048,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_13BPC_BIT_EXT")]
        _13BPC_EXT = 4096,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_14BPC_BIT_EXT")]
        _14BPC_EXT = 8192,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_15BPC_BIT_EXT")]
        _15BPC_EXT = 16384,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_16BPC_BIT_EXT")]
        _16BPC_EXT = 32768,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_17BPC_BIT_EXT")]
        _17BPC_EXT = 65536,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_18BPC_BIT_EXT")]
        _18BPC_EXT = 131072,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_19BPC_BIT_EXT")]
        _19BPC_EXT = 262144,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_20BPC_BIT_EXT")]
        _20BPC_EXT = 524288,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_21BPC_BIT_EXT")]
        _21BPC_EXT = 1048576,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_22BPC_BIT_EXT")]
        _22BPC_EXT = 2097152,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_23BPC_BIT_EXT")]
        _23BPC_EXT = 4194304,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_COMPRESSION_FIXED_RATE_24BPC_BIT_EXT")]
        _24BPC_EXT = 8388608,
    }
}
/// [`VkImageCompressionFixedRateFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCompressionFixedRateFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_ImageCompressionControl`](Extension::EXT_ImageCompressionControl)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkImageCompressionFixedRateFlagsEXT")]
pub type ImageCompressionFixedRateFlagsEXT = FlagSet<ImageCompressionFixedRateFlagEXT>;

crate::__vkx_internal_flags! {
    /// [`VkDeviceAddressBindingFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceAddressBindingFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_DeviceAddressBindingReport`](Extension::EXT_DeviceAddressBindingReport)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDeviceAddressBindingFlagBitsEXT")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum DeviceAddressBindingFlagEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DeviceAddressBindingReport`](Extension::EXT_DeviceAddressBindingReport)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DEVICE_ADDRESS_BINDING_INTERNAL_OBJECT_BIT_EXT")]
        #[default]
        INTERNAL_OBJECT_EXT = 1,
    }
}
/// [`VkDeviceAddressBindingFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceAddressBindingFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DeviceAddressBindingReport`](Extension::EXT_DeviceAddressBindingReport)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkDeviceAddressBindingFlagsEXT")]
pub type DeviceAddressBindingFlagsEXT = FlagSet<DeviceAddressBindingFlagEXT>;

crate::__vkx_internal_flags! {
    /// [`VkImageConstraintsInfoFlagBitsFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageConstraintsInfoFlagBitsFUCHSIA.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`FUCHSIA_BufferCollection`](Extension::FUCHSIA_BufferCollection)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkImageConstraintsInfoFlagBitsFUCHSIA")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ImageConstraintsInfoFlagFUCHSIA: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`FUCHSIA_BufferCollection`](Extension::FUCHSIA_BufferCollection)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_RARELY_FUCHSIA")]
        #[default]
        CPU_READ_RARELY_FUCHSIA = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`FUCHSIA_BufferCollection`](Extension::FUCHSIA_BufferCollection)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_OFTEN_FUCHSIA")]
        CPU_READ_OFTEN_FUCHSIA = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`FUCHSIA_BufferCollection`](Extension::FUCHSIA_BufferCollection)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_RARELY_FUCHSIA")]
        CPU_WRITE_RARELY_FUCHSIA = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`FUCHSIA_BufferCollection`](Extension::FUCHSIA_BufferCollection)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_OFTEN_FUCHSIA")]
        CPU_WRITE_OFTEN_FUCHSIA = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`FUCHSIA_BufferCollection`](Extension::FUCHSIA_BufferCollection)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_PROTECTED_OPTIONAL_FUCHSIA")]
        PROTECTED_OPTIONAL_FUCHSIA = 16,
    }
}
/// [`VkImageConstraintsInfoFlagsFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageConstraintsInfoFlagsFUCHSIA.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`FUCHSIA_BufferCollection`](Extension::FUCHSIA_BufferCollection)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkImageConstraintsInfoFlagsFUCHSIA")]
pub type ImageConstraintsInfoFlagsFUCHSIA = FlagSet<ImageConstraintsInfoFlagFUCHSIA>;

crate::__vkx_internal_flags! {
    /// [`VkFrameBoundaryFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFrameBoundaryFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_FrameBoundary`](Extension::EXT_FrameBoundary)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkFrameBoundaryFlagBitsEXT")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum FrameBoundaryFlagEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_FrameBoundary`](Extension::EXT_FrameBoundary)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_FRAME_BOUNDARY_FRAME_END_BIT_EXT")]
        #[default]
        FRAME_END_EXT = 1,
    }
}
/// [`VkFrameBoundaryFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFrameBoundaryFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_FrameBoundary`](Extension::EXT_FrameBoundary)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkFrameBoundaryFlagsEXT")]
pub type FrameBoundaryFlagsEXT = FlagSet<FrameBoundaryFlagEXT>;

crate::__vkx_internal_flags! {
    /// [`VkVideoEncodeRgbModelConversionFlagBitsVALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbModelConversionFlagBitsVALVE.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`VALVE_VideoEncodeRgbConversion`](Extension::VALVE_VideoEncodeRgbConversion)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodeRgbModelConversionFlagBitsVALVE")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoEncodeRgbModelConversionFlagVALVE: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_VideoEncodeRgbConversion`](Extension::VALVE_VideoEncodeRgbConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_RGB_IDENTITY_BIT_VALVE")]
        #[default]
        RGB_IDENTITY_VALVE = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_VideoEncodeRgbConversion`](Extension::VALVE_VideoEncodeRgbConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_IDENTITY_BIT_VALVE")]
        YCBCR_IDENTITY_VALVE = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_VideoEncodeRgbConversion`](Extension::VALVE_VideoEncodeRgbConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_709_BIT_VALVE")]
        YCBCR_709_VALVE = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_VideoEncodeRgbConversion`](Extension::VALVE_VideoEncodeRgbConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_601_BIT_VALVE")]
        YCBCR_601_VALVE = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_VideoEncodeRgbConversion`](Extension::VALVE_VideoEncodeRgbConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_2020_BIT_VALVE")]
        YCBCR_2020_VALVE = 16,
    }
}
/// [`VkVideoEncodeRgbModelConversionFlagsVALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbModelConversionFlagsVALVE.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`VALVE_VideoEncodeRgbConversion`](Extension::VALVE_VideoEncodeRgbConversion)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkVideoEncodeRgbModelConversionFlagsVALVE")]
pub type VideoEncodeRgbModelConversionFlagsVALVE = FlagSet<VideoEncodeRgbModelConversionFlagVALVE>;

crate::__vkx_internal_flags! {
    /// [`VkVideoEncodeRgbRangeCompressionFlagBitsVALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbRangeCompressionFlagBitsVALVE.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`VALVE_VideoEncodeRgbConversion`](Extension::VALVE_VideoEncodeRgbConversion)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodeRgbRangeCompressionFlagBitsVALVE")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoEncodeRgbRangeCompressionFlagVALVE: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_VideoEncodeRgbConversion`](Extension::VALVE_VideoEncodeRgbConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_RANGE_COMPRESSION_FULL_RANGE_BIT_VALVE")]
        #[default]
        FULL_RANGE_VALVE = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_VideoEncodeRgbConversion`](Extension::VALVE_VideoEncodeRgbConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_RANGE_COMPRESSION_NARROW_RANGE_BIT_VALVE")]
        NARROW_RANGE_VALVE = 2,
    }
}
/// [`VkVideoEncodeRgbRangeCompressionFlagsVALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbRangeCompressionFlagsVALVE.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`VALVE_VideoEncodeRgbConversion`](Extension::VALVE_VideoEncodeRgbConversion)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkVideoEncodeRgbRangeCompressionFlagsVALVE")]
pub type VideoEncodeRgbRangeCompressionFlagsVALVE =
    FlagSet<VideoEncodeRgbRangeCompressionFlagVALVE>;

crate::__vkx_internal_flags! {
    /// [`VkVideoEncodeRgbChromaOffsetFlagBitsVALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbChromaOffsetFlagBitsVALVE.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`VALVE_VideoEncodeRgbConversion`](Extension::VALVE_VideoEncodeRgbConversion)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkVideoEncodeRgbChromaOffsetFlagBitsVALVE")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum VideoEncodeRgbChromaOffsetFlagVALVE: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_VideoEncodeRgbConversion`](Extension::VALVE_VideoEncodeRgbConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_CHROMA_OFFSET_COSITED_EVEN_BIT_VALVE")]
        #[default]
        COSITED_EVEN_VALVE = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`VALVE_VideoEncodeRgbConversion`](Extension::VALVE_VideoEncodeRgbConversion)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_VIDEO_ENCODE_RGB_CHROMA_OFFSET_MIDPOINT_BIT_VALVE")]
        MIDPOINT_VALVE = 2,
    }
}
/// [`VkVideoEncodeRgbChromaOffsetFlagsVALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbChromaOffsetFlagsVALVE.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`VALVE_VideoEncodeRgbConversion`](Extension::VALVE_VideoEncodeRgbConversion)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkVideoEncodeRgbChromaOffsetFlagsVALVE")]
pub type VideoEncodeRgbChromaOffsetFlagsVALVE = FlagSet<VideoEncodeRgbChromaOffsetFlagVALVE>;

crate::__vkx_internal_flags! {
    /// [`VkBuildMicromapFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildMicromapFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_OpacityMicromap`](Extension::EXT_OpacityMicromap)
    /// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkBuildMicromapFlagBitsEXT")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum BuildMicromapFlagEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_OpacityMicromap`](Extension::EXT_OpacityMicromap)
        /// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_MICROMAP_PREFER_FAST_TRACE_BIT_EXT")]
        #[default]
        PREFER_FAST_TRACE_EXT = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_OpacityMicromap`](Extension::EXT_OpacityMicromap)
        /// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_MICROMAP_PREFER_FAST_BUILD_BIT_EXT")]
        PREFER_FAST_BUILD_EXT = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_OpacityMicromap`](Extension::EXT_OpacityMicromap)
        /// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_BUILD_MICROMAP_ALLOW_COMPACTION_BIT_EXT")]
        ALLOW_COMPACTION_EXT = 4,
    }
}
/// [`VkBuildMicromapFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildMicromapFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_OpacityMicromap`](Extension::EXT_OpacityMicromap)
/// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkBuildMicromapFlagsEXT")]
pub type BuildMicromapFlagsEXT = FlagSet<BuildMicromapFlagEXT>;

crate::__vkx_internal_flags! {
    /// [`VkMicromapCreateFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapCreateFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_OpacityMicromap`](Extension::EXT_OpacityMicromap)
    /// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkMicromapCreateFlagBitsEXT")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum MicromapCreateFlagEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_OpacityMicromap`](Extension::EXT_OpacityMicromap)
        /// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MICROMAP_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT")]
        #[default]
        DEVICE_ADDRESS_CAPTURE_REPLAY_EXT = 1,
    }
}
/// [`VkMicromapCreateFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapCreateFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_OpacityMicromap`](Extension::EXT_OpacityMicromap)
/// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkMicromapCreateFlagsEXT")]
pub type MicromapCreateFlagsEXT = FlagSet<MicromapCreateFlagEXT>;

crate::__vkx_internal_flags! {
    /// [`VkPhysicalDeviceSchedulingControlsFlagBitsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSchedulingControlsFlagBitsARM.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`ARM_SchedulingControls`](Extension::ARM_SchedulingControls)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkPhysicalDeviceSchedulingControlsFlagBitsARM")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum PhysicalDeviceSchedulingControlsFlagARM: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_SchedulingControls`](Extension::ARM_SchedulingControls)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PHYSICAL_DEVICE_SCHEDULING_CONTROLS_SHADER_CORE_COUNT_ARM")]
        #[default]
        SHADER_CORE_COUNT_ARM = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_SchedulingControls`](Extension::ARM_SchedulingControls)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PHYSICAL_DEVICE_SCHEDULING_CONTROLS_DISPATCH_PARAMETERS_ARM")]
        DISPATCH_PARAMETERS_ARM = 2,
    }
}
/// [`VkPhysicalDeviceSchedulingControlsFlagsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSchedulingControlsFlagsARM.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`ARM_SchedulingControls`](Extension::ARM_SchedulingControls)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkPhysicalDeviceSchedulingControlsFlagsARM")]
pub type PhysicalDeviceSchedulingControlsFlagsARM =
    FlagSet<PhysicalDeviceSchedulingControlsFlagARM>;

crate::__vkx_internal_flags! {
    /// [`VkMemoryDecompressionMethodFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryDecompressionMethodFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_MemoryDecompression`](Extension::NV_MemoryDecompression)
    /// - Extension [`EXT_MemoryDecompression`](Extension::EXT_MemoryDecompression)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkMemoryDecompressionMethodFlagBitsEXT")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum MemoryDecompressionMethodFlagEXT: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_MemoryDecompression`](Extension::EXT_MemoryDecompression)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_MEMORY_DECOMPRESSION_METHOD_GDEFLATE_1_0_BIT_EXT")]
        #[default]
        GDEFLATE_1_0_EXT = 1,
    }
}
/// [`VkMemoryDecompressionMethodFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryDecompressionMethodFlagBitsNV.html)
///
#[doc(alias = "VkMemoryDecompressionMethodFlagBitsNV")]
pub type MemoryDecompressionMethodFlagNV = MemoryDecompressionMethodFlagEXT;
impl MemoryDecompressionMethodFlagEXT {
    /// [`VK_MEMORY_DECOMPRESSION_METHOD_GDEFLATE_1_0_BIT_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MEMORY_DECOMPRESSION_METHOD_GDEFLATE_1_0_BIT_NV.html)
    ///
    #[doc(alias = "VK_MEMORY_DECOMPRESSION_METHOD_GDEFLATE_1_0_BIT_NV")]
    pub const GDEFLATE_1_0_NV: Self = Self::GDEFLATE_1_0_EXT;
}
/// [`VkMemoryDecompressionMethodFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryDecompressionMethodFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_MemoryDecompression`](Extension::NV_MemoryDecompression)
/// - Extension [`EXT_MemoryDecompression`](Extension::EXT_MemoryDecompression)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkMemoryDecompressionMethodFlagsEXT")]
pub type MemoryDecompressionMethodFlagsEXT = FlagSet<MemoryDecompressionMethodFlagEXT>;
/// [`VkMemoryDecompressionMethodFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryDecompressionMethodFlagsNV.html)
///
#[doc(alias = "VkMemoryDecompressionMethodFlagsNV")]
pub type MemoryDecompressionMethodFlagsNV = MemoryDecompressionMethodFlagsEXT;

crate::__vkx_internal_flags! {
    /// [`VkTensorCreateFlagBitsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorCreateFlagBitsARM.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`ARM_Tensors`](Extension::ARM_Tensors)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkTensorCreateFlagBitsARM")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum TensorCreateFlagARM: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_Tensors`](Extension::ARM_Tensors)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TENSOR_CREATE_MUTABLE_FORMAT_BIT_ARM")]
        #[default]
        MUTABLE_FORMAT_ARM = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_Tensors`](Extension::ARM_Tensors)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TENSOR_CREATE_PROTECTED_BIT_ARM")]
        PROTECTED_ARM = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TENSOR_CREATE_DESCRIPTOR_HEAP_CAPTURE_REPLAY_BIT_ARM")]
        DESCRIPTOR_HEAP_CAPTURE_REPLAY_ARM = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_Tensors`](Extension::ARM_Tensors)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TENSOR_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_ARM")]
        DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM = 4,
    }
}
/// [`VkTensorCreateFlagsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorCreateFlagsARM.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`ARM_Tensors`](Extension::ARM_Tensors)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkTensorCreateFlagsARM")]
pub type TensorCreateFlagsARM = FlagSet<TensorCreateFlagARM>;

crate::__vkx_internal_flags! {
    /// [`VkTensorUsageFlagBitsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorUsageFlagBitsARM.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`ARM_Tensors`](Extension::ARM_Tensors)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkTensorUsageFlagBitsARM")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum TensorUsageFlagARM: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_Tensors`](Extension::ARM_Tensors)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TENSOR_USAGE_SHADER_BIT_ARM")]
        #[default]
        SHADER_ARM = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_Tensors`](Extension::ARM_Tensors)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TENSOR_USAGE_TRANSFER_SRC_BIT_ARM")]
        TRANSFER_SRC_ARM = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_Tensors`](Extension::ARM_Tensors)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TENSOR_USAGE_TRANSFER_DST_BIT_ARM")]
        TRANSFER_DST_ARM = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_Tensors`](Extension::ARM_Tensors)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TENSOR_USAGE_IMAGE_ALIASING_BIT_ARM")]
        IMAGE_ALIASING_ARM = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraph`](Extension::ARM_DataGraph)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_TENSOR_USAGE_DATA_GRAPH_BIT_ARM")]
        DATA_GRAPH_ARM = 32,
    }
}
/// [`VkTensorUsageFlagsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorUsageFlagsARM.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`ARM_Tensors`](Extension::ARM_Tensors)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkTensorUsageFlagsARM")]
pub type TensorUsageFlagsARM = FlagSet<TensorUsageFlagARM>;

crate::__vkx_internal_flags! {
    /// [`VkOpticalFlowGridSizeFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowGridSizeFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkOpticalFlowGridSizeFlagBitsNV")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum OpticalFlowGridSizeFlagNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_GRID_SIZE_UNKNOWN_NV")]
        #[default]
        UNKNOWN_NV = 0,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_GRID_SIZE_1X1_BIT_NV")]
        _1X1_NV = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_GRID_SIZE_2X2_BIT_NV")]
        _2X2_NV = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_GRID_SIZE_4X4_BIT_NV")]
        _4X4_NV = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_GRID_SIZE_8X8_BIT_NV")]
        _8X8_NV = 8,
    }
}
/// [`VkOpticalFlowGridSizeFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowGridSizeFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkOpticalFlowGridSizeFlagsNV")]
pub type OpticalFlowGridSizeFlagsNV = FlagSet<OpticalFlowGridSizeFlagNV>;

crate::__vkx_internal_flags! {
    /// [`VkOpticalFlowUsageFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowUsageFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkOpticalFlowUsageFlagBitsNV")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum OpticalFlowUsageFlagNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_USAGE_UNKNOWN_NV")]
        #[default]
        UNKNOWN_NV = 0,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_USAGE_INPUT_BIT_NV")]
        INPUT_NV = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_USAGE_OUTPUT_BIT_NV")]
        OUTPUT_NV = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_USAGE_HINT_BIT_NV")]
        HINT_NV = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_USAGE_COST_BIT_NV")]
        COST_NV = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_USAGE_GLOBAL_FLOW_BIT_NV")]
        GLOBAL_FLOW_NV = 16,
    }
}
/// [`VkOpticalFlowUsageFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowUsageFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkOpticalFlowUsageFlagsNV")]
pub type OpticalFlowUsageFlagsNV = FlagSet<OpticalFlowUsageFlagNV>;

crate::__vkx_internal_flags! {
    /// [`VkOpticalFlowSessionCreateFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowSessionCreateFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkOpticalFlowSessionCreateFlagBitsNV")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum OpticalFlowSessionCreateFlagNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_HINT_BIT_NV")]
        #[default]
        ENABLE_HINT_NV = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_COST_BIT_NV")]
        ENABLE_COST_NV = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_GLOBAL_FLOW_BIT_NV")]
        ENABLE_GLOBAL_FLOW_NV = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_SESSION_CREATE_ALLOW_REGIONS_BIT_NV")]
        ALLOW_REGIONS_NV = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_SESSION_CREATE_BOTH_DIRECTIONS_BIT_NV")]
        BOTH_DIRECTIONS_NV = 16,
    }
}
/// [`VkOpticalFlowSessionCreateFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowSessionCreateFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkOpticalFlowSessionCreateFlagsNV")]
pub type OpticalFlowSessionCreateFlagsNV = FlagSet<OpticalFlowSessionCreateFlagNV>;

crate::__vkx_internal_flags! {
    /// [`VkOpticalFlowExecuteFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowExecuteFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkOpticalFlowExecuteFlagBitsNV")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum OpticalFlowExecuteFlagNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_OPTICAL_FLOW_EXECUTE_DISABLE_TEMPORAL_HINTS_BIT_NV")]
        #[default]
        DISABLE_TEMPORAL_HINTS_NV = 1,
    }
}
/// [`VkOpticalFlowExecuteFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowExecuteFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkOpticalFlowExecuteFlagsNV")]
pub type OpticalFlowExecuteFlagsNV = FlagSet<OpticalFlowExecuteFlagNV>;

crate::__vkx_internal_flags! {
    /// [`VkShaderCreateFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderCreateFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_ShaderObject`](Extension::EXT_ShaderObject)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkShaderCreateFlagBitsEXT")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ShaderCreateFlagEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ShaderObject`](Extension::EXT_ShaderObject)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_LINK_STAGE_BIT_EXT")]
        #[default]
        LINK_STAGE_EXT = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_DESCRIPTOR_HEAP_BIT_EXT")]
        DESCRIPTOR_HEAP_EXT = 1024,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Version 1.4 with appropriate features
        /// - Extension [`KHR_Maintenance5`](Extension::KHR_Maintenance5)
        /// - Extension [`ARM_ShaderInstrumentation`](Extension::ARM_ShaderInstrumentation)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_INSTRUMENT_SHADER_BIT_ARM")]
        INSTRUMENT_SHADER_ARM = 2048,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ShaderObject`](Extension::EXT_ShaderObject)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT_EXT")]
        ALLOW_VARYING_SUBGROUP_SIZE_EXT = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ShaderObject`](Extension::EXT_ShaderObject)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_REQUIRE_FULL_SUBGROUPS_BIT_EXT")]
        REQUIRE_FULL_SUBGROUPS_EXT = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ShaderObject`](Extension::EXT_ShaderObject)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_NO_TASK_SHADER_BIT_EXT")]
        NO_TASK_SHADER_EXT = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ShaderObject`](Extension::EXT_ShaderObject)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_DISPATCH_BASE_BIT_EXT")]
        DISPATCH_BASE_EXT = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ShaderObject`](Extension::EXT_ShaderObject)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_EXT")]
        FRAGMENT_SHADING_RATE_ATTACHMENT_EXT = 32,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_ShaderObject`](Extension::EXT_ShaderObject)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT")]
        FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT = 64,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DeviceGeneratedCommands`](Extension::EXT_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_INDIRECT_BINDABLE_BIT_EXT")]
        INDIRECT_BINDABLE_EXT = 128,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_OPACITY_MICROMAP_DISALLOW_MIXED_SPECIAL_INDEX_BIT_EXT")]
        OPACITY_MICROMAP_DISALLOW_MIXED_SPECIAL_INDEX_EXT = 4096,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_Shader64BitIndexing`](Extension::EXT_Shader64BitIndexing)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_64_BIT_INDEXING_BIT_EXT")]
        _64_INDEXING_EXT = 32768,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`KHR_Maintenance11`](Extension::KHR_Maintenance11)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_SHADER_CREATE_INDEPENDENT_SETS_BIT_KHR")]
        INDEPENDENT_SETS_KHR = 262144,
    }
}
/// [`VkShaderCreateFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderCreateFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_ShaderObject`](Extension::EXT_ShaderObject)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkShaderCreateFlagsEXT")]
pub type ShaderCreateFlagsEXT = FlagSet<ShaderCreateFlagEXT>;

crate::__vkx_internal_flags! {
    /// [`VkDataGraphPipelineSessionCreateFlagBitsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionCreateFlagBitsARM.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`ARM_DataGraph`](Extension::ARM_DataGraph)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDataGraphPipelineSessionCreateFlagBitsARM")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum DataGraphPipelineSessionCreateFlagARM: u64 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraph`](Extension::ARM_DataGraph)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_PIPELINE_SESSION_CREATE_PROTECTED_BIT_ARM")]
        #[default]
        PROTECTED_ARM = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_PIPELINE_SESSION_CREATE_OPTICAL_FLOW_CACHE_BIT_ARM")]
        OPTICAL_FLOW_CACHE_ARM = 2,
    }
}
/// [`VkDataGraphPipelineSessionCreateFlagsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionCreateFlagsARM.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`ARM_DataGraph`](Extension::ARM_DataGraph)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkDataGraphPipelineSessionCreateFlagsARM")]
pub type DataGraphPipelineSessionCreateFlagsARM = FlagSet<DataGraphPipelineSessionCreateFlagARM>;

crate::__vkx_internal_flags! {
    /// [`VkDataGraphPipelineDispatchFlagBitsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineDispatchFlagBitsARM.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`ARM_DataGraph`](Extension::ARM_DataGraph)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDataGraphPipelineDispatchFlagBitsARM")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum DataGraphPipelineDispatchFlagARM: u64 {
        #[default]
        #[doc(hidden)]
        __PLACEHOLDER = 0,
    }
}
/// [`VkDataGraphPipelineDispatchFlagsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineDispatchFlagsARM.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`ARM_DataGraph`](Extension::ARM_DataGraph)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkDataGraphPipelineDispatchFlagsARM")]
pub type DataGraphPipelineDispatchFlagsARM = FlagSet<DataGraphPipelineDispatchFlagARM>;

crate::__vkx_internal_flags! {
    /// [`VkDataGraphTOSAQualityFlagBitsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphTOSAQualityFlagBitsARM.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`ARM_DataGraphInstructionSetTosa`](Extension::ARM_DataGraphInstructionSetTosa)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkDataGraphTOSAQualityFlagBitsARM")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum DataGraphTOSAQualityFlagARM: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphInstructionSetTosa`](Extension::ARM_DataGraphInstructionSetTosa)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_TOSA_QUALITY_ACCELERATED_ARM")]
        #[default]
        ACCELERATED_ARM = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphInstructionSetTosa`](Extension::ARM_DataGraphInstructionSetTosa)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_TOSA_QUALITY_CONFORMANT_ARM")]
        CONFORMANT_ARM = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphInstructionSetTosa`](Extension::ARM_DataGraphInstructionSetTosa)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_TOSA_QUALITY_EXPERIMENTAL_ARM")]
        EXPERIMENTAL_ARM = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphInstructionSetTosa`](Extension::ARM_DataGraphInstructionSetTosa)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_TOSA_QUALITY_DEPRECATED_ARM")]
        DEPRECATED_ARM = 8,
    }
}
/// [`VkDataGraphTOSAQualityFlagsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphTOSAQualityFlagsARM.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`ARM_DataGraphInstructionSetTosa`](Extension::ARM_DataGraphInstructionSetTosa)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkDataGraphTOSAQualityFlagsARM")]
pub type DataGraphTOSAQualityFlagsARM = FlagSet<DataGraphTOSAQualityFlagARM>;

crate::__vkx_internal_flags! {
    /// [`VkClusterAccelerationStructureAddressResolutionFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureAddressResolutionFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkClusterAccelerationStructureAddressResolutionFlagBitsNV")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ClusterAccelerationStructureAddressResolutionFlagNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_NONE_NV")]
        #[default]
        NONE_NV = 0,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_IMPLICIT_DATA_BIT_NV")]
        INDIRECTED_DST_IMPLICIT_DATA_NV = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SCRATCH_DATA_BIT_NV")]
        INDIRECTED_SCRATCH_DATA_NV = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_ADDRESS_ARRAY_BIT_NV")]
        INDIRECTED_DST_ADDRESS_ARRAY_NV = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_SIZES_ARRAY_BIT_NV")]
        INDIRECTED_DST_SIZES_ARRAY_NV = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SRC_INFOS_ARRAY_BIT_NV")]
        INDIRECTED_SRC_INFOS_ARRAY_NV = 16,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SRC_INFOS_COUNT_BIT_NV")]
        INDIRECTED_SRC_INFOS_COUNT_NV = 32,
    }
}
/// [`VkClusterAccelerationStructureAddressResolutionFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureAddressResolutionFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkClusterAccelerationStructureAddressResolutionFlagsNV")]
pub type ClusterAccelerationStructureAddressResolutionFlagsNV =
    FlagSet<ClusterAccelerationStructureAddressResolutionFlagNV>;

crate::__vkx_internal_flags! {
    /// [`VkClusterAccelerationStructureClusterFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureClusterFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkClusterAccelerationStructureClusterFlagBitsNV")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ClusterAccelerationStructureClusterFlagNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_CLUSTER_ALLOW_DISABLE_OPACITY_MICROMAPS_NV")]
        #[default]
        ALLOW_DISABLE_OPACITY_MICROMAPS_NV = 1,
    }
}
/// [`VkClusterAccelerationStructureClusterFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureClusterFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkClusterAccelerationStructureClusterFlagsNV")]
pub type ClusterAccelerationStructureClusterFlagsNV =
    FlagSet<ClusterAccelerationStructureClusterFlagNV>;

crate::__vkx_internal_flags! {
    /// [`VkClusterAccelerationStructureGeometryFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureGeometryFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkClusterAccelerationStructureGeometryFlagBitsNV")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ClusterAccelerationStructureGeometryFlagNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_CULL_DISABLE_BIT_NV")]
        #[default]
        CULL_DISABLE_NV = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_NO_DUPLICATE_ANYHIT_INVOCATION_BIT_NV")]
        NO_DUPLICATE_ANYHIT_INVOCATION_NV = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_OPAQUE_BIT_NV")]
        OPAQUE_NV = 4,
    }
}
/// [`VkClusterAccelerationStructureGeometryFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureGeometryFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkClusterAccelerationStructureGeometryFlagsNV")]
pub type ClusterAccelerationStructureGeometryFlagsNV =
    FlagSet<ClusterAccelerationStructureGeometryFlagNV>;

crate::__vkx_internal_flags! {
    /// [`VkClusterAccelerationStructureIndexFormatFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureIndexFormatFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    /// # Returned only
    /// This type is only returned by Vulkan, never constructed by the API user.
    #[doc(alias = "VkClusterAccelerationStructureIndexFormatFlagBitsNV")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum ClusterAccelerationStructureIndexFormatFlagNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_8BIT_NV")]
        #[default]
        _8BIT_NV = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_16BIT_NV")]
        _16BIT_NV = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_32BIT_NV")]
        _32BIT_NV = 4,
    }
}
/// [`VkClusterAccelerationStructureIndexFormatFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureIndexFormatFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_ClusterAccelerationStructure`](Extension::NV_ClusterAccelerationStructure)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkClusterAccelerationStructureIndexFormatFlagsNV")]
pub type ClusterAccelerationStructureIndexFormatFlagsNV =
    FlagSet<ClusterAccelerationStructureIndexFormatFlagNV>;

crate::__vkx_internal_flags! {
    /// [`VkPartitionedAccelerationStructureInstanceFlagBitsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPartitionedAccelerationStructureInstanceFlagBitsNV.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`NV_PartitionedAccelerationStructure`](Extension::NV_PartitionedAccelerationStructure)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkPartitionedAccelerationStructureInstanceFlagBitsNV")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum PartitionedAccelerationStructureInstanceFlagNV: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_PartitionedAccelerationStructure`](Extension::NV_PartitionedAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_TRIANGLE_FACING_CULL_DISABLE_BIT_NV")]
        #[default]
        FLAG_TRIANGLE_FACING_CULL_DISABLE_NV = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_PartitionedAccelerationStructure`](Extension::NV_PartitionedAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_TRIANGLE_FLIP_FACING_BIT_NV")]
        FLAG_TRIANGLE_FLIP_FACING_NV = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_PartitionedAccelerationStructure`](Extension::NV_PartitionedAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_FORCE_OPAQUE_BIT_NV")]
        FLAG_FORCE_OPAQUE_NV = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_PartitionedAccelerationStructure`](Extension::NV_PartitionedAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_FORCE_NO_OPAQUE_BIT_NV")]
        FLAG_FORCE_NO_OPAQUE_NV = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`NV_PartitionedAccelerationStructure`](Extension::NV_PartitionedAccelerationStructure)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV")]
        FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV = 16,
    }
}
/// [`VkPartitionedAccelerationStructureInstanceFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPartitionedAccelerationStructureInstanceFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_PartitionedAccelerationStructure`](Extension::NV_PartitionedAccelerationStructure)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkPartitionedAccelerationStructureInstanceFlagsNV")]
pub type PartitionedAccelerationStructureInstanceFlagsNV =
    FlagSet<PartitionedAccelerationStructureInstanceFlagNV>;

crate::__vkx_internal_flags! {
    /// [`VkIndirectCommandsInputModeFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsInputModeFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_DeviceGeneratedCommands`](Extension::EXT_DeviceGeneratedCommands)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkIndirectCommandsInputModeFlagBitsEXT")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum IndirectCommandsInputModeFlagEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DeviceGeneratedCommands`](Extension::EXT_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_INDIRECT_COMMANDS_INPUT_MODE_VULKAN_INDEX_BUFFER_EXT")]
        #[default]
        VULKAN_INDEX_BUFFER_EXT = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DeviceGeneratedCommands`](Extension::EXT_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_INDIRECT_COMMANDS_INPUT_MODE_DXGI_INDEX_BUFFER_EXT")]
        DXGI_INDEX_BUFFER_EXT = 2,
    }
}
/// [`VkIndirectCommandsInputModeFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsInputModeFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DeviceGeneratedCommands`](Extension::EXT_DeviceGeneratedCommands)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkIndirectCommandsInputModeFlagsEXT")]
pub type IndirectCommandsInputModeFlagsEXT = FlagSet<IndirectCommandsInputModeFlagEXT>;

crate::__vkx_internal_flags! {
    /// [`VkIndirectCommandsLayoutUsageFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutUsageFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_DeviceGeneratedCommands`](Extension::EXT_DeviceGeneratedCommands)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkIndirectCommandsLayoutUsageFlagBitsEXT")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum IndirectCommandsLayoutUsageFlagEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DeviceGeneratedCommands`](Extension::EXT_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_EXT")]
        #[default]
        EXPLICIT_PREPROCESS_EXT = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_DeviceGeneratedCommands`](Extension::EXT_DeviceGeneratedCommands)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_EXT")]
        UNORDERED_SEQUENCES_EXT = 2,
    }
}
/// [`VkIndirectCommandsLayoutUsageFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutUsageFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DeviceGeneratedCommands`](Extension::EXT_DeviceGeneratedCommands)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkIndirectCommandsLayoutUsageFlagsEXT")]
pub type IndirectCommandsLayoutUsageFlagsEXT = FlagSet<IndirectCommandsLayoutUsageFlagEXT>;

crate::__vkx_internal_flags! {
    /// [`VkDataGraphOpticalFlowGridSizeFlagBitsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowGridSizeFlagBitsARM.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDataGraphOpticalFlowGridSizeFlagBitsARM")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum DataGraphOpticalFlowGridSizeFlagARM: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_UNKNOWN_ARM")]
        #[default]
        UNKNOWN_ARM = 0,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_1X1_BIT_ARM")]
        _1X1_ARM = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_2X2_BIT_ARM")]
        _2X2_ARM = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_4X4_BIT_ARM")]
        _4X4_ARM = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_8X8_BIT_ARM")]
        _8X8_ARM = 8,
    }
}
/// [`VkDataGraphOpticalFlowGridSizeFlagsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowGridSizeFlagsARM.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkDataGraphOpticalFlowGridSizeFlagsARM")]
pub type DataGraphOpticalFlowGridSizeFlagsARM = FlagSet<DataGraphOpticalFlowGridSizeFlagARM>;

crate::__vkx_internal_flags! {
    /// [`VkDataGraphOpticalFlowCreateFlagBitsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowCreateFlagBitsARM.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDataGraphOpticalFlowCreateFlagBitsARM")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum DataGraphOpticalFlowCreateFlagARM: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_CREATE_ENABLE_HINT_BIT_ARM")]
        #[default]
        ENABLE_HINT_ARM = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_CREATE_ENABLE_COST_BIT_ARM")]
        ENABLE_COST_ARM = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_CREATE_RESERVED_30_BIT_ARM")]
        RESERVED_30_ARM = 1073741824,
    }
}
/// [`VkDataGraphOpticalFlowCreateFlagsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowCreateFlagsARM.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkDataGraphOpticalFlowCreateFlagsARM")]
pub type DataGraphOpticalFlowCreateFlagsARM = FlagSet<DataGraphOpticalFlowCreateFlagARM>;

crate::__vkx_internal_flags! {
    /// [`VkDataGraphOpticalFlowImageUsageFlagBitsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowImageUsageFlagBitsARM.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDataGraphOpticalFlowImageUsageFlagBitsARM")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum DataGraphOpticalFlowImageUsageFlagARM: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_UNKNOWN_ARM")]
        #[default]
        UNKNOWN_ARM = 0,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_INPUT_BIT_ARM")]
        INPUT_ARM = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_OUTPUT_BIT_ARM")]
        OUTPUT_ARM = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_HINT_BIT_ARM")]
        HINT_ARM = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_COST_BIT_ARM")]
        COST_ARM = 8,
    }
}
/// [`VkDataGraphOpticalFlowImageUsageFlagsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowImageUsageFlagsARM.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkDataGraphOpticalFlowImageUsageFlagsARM")]
pub type DataGraphOpticalFlowImageUsageFlagsARM = FlagSet<DataGraphOpticalFlowImageUsageFlagARM>;

crate::__vkx_internal_flags! {
    /// [`VkDataGraphOpticalFlowExecuteFlagBitsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowExecuteFlagBitsARM.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkDataGraphOpticalFlowExecuteFlagBitsARM")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum DataGraphOpticalFlowExecuteFlagARM: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_DISABLE_TEMPORAL_HINTS_BIT_ARM")]
        #[default]
        DISABLE_TEMPORAL_HINTS_ARM = 1,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_INPUT_UNCHANGED_BIT_ARM")]
        INPUT_UNCHANGED_ARM = 2,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_REFERENCE_UNCHANGED_BIT_ARM")]
        REFERENCE_UNCHANGED_ARM = 4,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_INPUT_IS_PREVIOUS_REFERENCE_BIT_ARM")]
        INPUT_IS_PREVIOUS_REFERENCE_ARM = 8,
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_REFERENCE_IS_PREVIOUS_INPUT_BIT_ARM")]
        REFERENCE_IS_PREVIOUS_INPUT_ARM = 16,
    }
}
/// [`VkDataGraphOpticalFlowExecuteFlagsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowExecuteFlagsARM.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`ARM_DataGraphOpticalFlow`](Extension::ARM_DataGraphOpticalFlow)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkDataGraphOpticalFlowExecuteFlagsARM")]
pub type DataGraphOpticalFlowExecuteFlagsARM = FlagSet<DataGraphOpticalFlowExecuteFlagARM>;

crate::__vkx_internal_flags! {
    /// [`VkCooperativeMatrixFlagBitsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCooperativeMatrixFlagBitsEXT.html)
    ///
    /// # Requirements
    /// This requires _at least_ one of the following:
    /// - Extension [`EXT_CooperativeMatrixMaintenance1`](Extension::EXT_CooperativeMatrixMaintenance1)
    ///
    /// Note this list might not be exhaustive. For more information check vulkan documentation.
    ///
    #[doc(alias = "VkCooperativeMatrixFlagBitsEXT")]
    #[derive(Default)]
    #[non_exhaustive]
    pub enum CooperativeMatrixFlagEXT: u32 {
        /// # Requirements
        /// This requires _at least_ one of the following:
        /// - Extension [`EXT_CooperativeMatrixMaintenance1`](Extension::EXT_CooperativeMatrixMaintenance1)
        ///
        /// Note this list might not be exhaustive. For more information check vulkan documentation.
        ///
        #[doc(alias = "VK_COOPERATIVE_MATRIX_SATURATING_ACCUMULATION_BIT_EXT")]
        #[default]
        SATURATING_ACCUMULATION_EXT = 1,
    }
}
/// [`VkCooperativeMatrixFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCooperativeMatrixFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_CooperativeMatrixMaintenance1`](Extension::EXT_CooperativeMatrixMaintenance1)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkCooperativeMatrixFlagsEXT")]
pub type CooperativeMatrixFlagsEXT = FlagSet<CooperativeMatrixFlagEXT>;

/// [`VkDeviceCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceCreateFlags.html)
///
#[doc(alias = "VkDeviceCreateFlags")]
pub type DeviceCreateFlags = u32;

/// [`VkSemaphoreCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreCreateFlags.html)
///
#[doc(alias = "VkSemaphoreCreateFlags")]
pub type SemaphoreCreateFlags = u32;

/// [`VkBufferViewCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferViewCreateFlags.html)
///
#[doc(alias = "VkBufferViewCreateFlags")]
pub type BufferViewCreateFlags = u32;

/// [`VkShaderModuleCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderModuleCreateFlags.html)
///
#[doc(alias = "VkShaderModuleCreateFlags")]
pub type ShaderModuleCreateFlags = u32;

/// [`VkDescriptorPoolResetFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPoolResetFlags.html)
///
#[doc(alias = "VkDescriptorPoolResetFlags")]
pub type DescriptorPoolResetFlags = u32;

/// [`VkPipelineDynamicStateCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineDynamicStateCreateFlags.html)
///
#[doc(alias = "VkPipelineDynamicStateCreateFlags")]
pub type PipelineDynamicStateCreateFlags = u32;

/// [`VkPipelineInputAssemblyStateCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineInputAssemblyStateCreateFlags.html)
///
#[doc(alias = "VkPipelineInputAssemblyStateCreateFlags")]
pub type PipelineInputAssemblyStateCreateFlags = u32;

/// [`VkPipelineMultisampleStateCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineMultisampleStateCreateFlags.html)
///
#[doc(alias = "VkPipelineMultisampleStateCreateFlags")]
pub type PipelineMultisampleStateCreateFlags = u32;

/// [`VkPipelineRasterizationStateCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationStateCreateFlags.html)
///
#[doc(alias = "VkPipelineRasterizationStateCreateFlags")]
pub type PipelineRasterizationStateCreateFlags = u32;

/// [`VkPipelineTessellationStateCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineTessellationStateCreateFlags.html)
///
#[doc(alias = "VkPipelineTessellationStateCreateFlags")]
pub type PipelineTessellationStateCreateFlags = u32;

/// [`VkPipelineVertexInputStateCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineVertexInputStateCreateFlags.html)
///
#[doc(alias = "VkPipelineVertexInputStateCreateFlags")]
pub type PipelineVertexInputStateCreateFlags = u32;

/// [`VkPipelineViewportStateCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportStateCreateFlags.html)
///
#[doc(alias = "VkPipelineViewportStateCreateFlags")]
pub type PipelineViewportStateCreateFlags = u32;

/// [`VkCommandPoolTrimFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolTrimFlags.html)
///
#[doc(alias = "VkCommandPoolTrimFlags")]
pub type CommandPoolTrimFlags = u32;
/// [`VkCommandPoolTrimFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolTrimFlagsKHR.html)
///
#[doc(alias = "VkCommandPoolTrimFlagsKHR")]
pub type CommandPoolTrimFlagsKHR = CommandPoolTrimFlags;

/// [`VkDescriptorUpdateTemplateCreateFlags`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplateCreateFlags.html)
///
#[doc(alias = "VkDescriptorUpdateTemplateCreateFlags")]
pub type DescriptorUpdateTemplateCreateFlags = u32;
/// [`VkDescriptorUpdateTemplateCreateFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplateCreateFlagsKHR.html)
///
#[doc(alias = "VkDescriptorUpdateTemplateCreateFlagsKHR")]
pub type DescriptorUpdateTemplateCreateFlagsKHR = DescriptorUpdateTemplateCreateFlags;

/// [`VkDisplayModeCreateFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayModeCreateFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_Display`](Extension::KHR_Display)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkDisplayModeCreateFlagsKHR")]
pub type DisplayModeCreateFlagsKHR = u32;

/// [`VkDisplaySurfaceCreateFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplaySurfaceCreateFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_Display`](Extension::KHR_Display)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkDisplaySurfaceCreateFlagsKHR")]
pub type DisplaySurfaceCreateFlagsKHR = u32;

/// [`VkXlibSurfaceCreateFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkXlibSurfaceCreateFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_XlibSurface`](Extension::KHR_XlibSurface)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkXlibSurfaceCreateFlagsKHR")]
pub type XlibSurfaceCreateFlagsKHR = u32;

/// [`VkXcbSurfaceCreateFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkXcbSurfaceCreateFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_XcbSurface`](Extension::KHR_XcbSurface)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkXcbSurfaceCreateFlagsKHR")]
pub type XcbSurfaceCreateFlagsKHR = u32;

/// [`VkWaylandSurfaceCreateFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkWaylandSurfaceCreateFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_WaylandSurface`](Extension::KHR_WaylandSurface)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkWaylandSurfaceCreateFlagsKHR")]
pub type WaylandSurfaceCreateFlagsKHR = u32;

/// [`VkAndroidSurfaceCreateFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAndroidSurfaceCreateFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_AndroidSurface`](Extension::KHR_AndroidSurface)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkAndroidSurfaceCreateFlagsKHR")]
pub type AndroidSurfaceCreateFlagsKHR = u32;

/// [`VkWin32SurfaceCreateFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkWin32SurfaceCreateFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_Win32Surface`](Extension::KHR_Win32Surface)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkWin32SurfaceCreateFlagsKHR")]
pub type Win32SurfaceCreateFlagsKHR = u32;

/// [`VkVideoBeginCodingFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoBeginCodingFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkVideoBeginCodingFlagsKHR")]
pub type VideoBeginCodingFlagsKHR = u32;

/// [`VkVideoEndCodingFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEndCodingFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkVideoEndCodingFlagsKHR")]
pub type VideoEndCodingFlagsKHR = u32;

/// [`VkVideoDecodeFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoDecodeQueue`](Extension::KHR_VideoDecodeQueue)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkVideoDecodeFlagsKHR")]
pub type VideoDecodeFlagsKHR = u32;

/// [`VkVideoEncodeRateControlFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRateControlFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoEncodeQueue`](Extension::KHR_VideoEncodeQueue)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkVideoEncodeRateControlFlagsKHR")]
pub type VideoEncodeRateControlFlagsKHR = u32;

/// [`VkPipelineRasterizationStateStreamCreateFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationStateStreamCreateFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_TransformFeedback`](Extension::EXT_TransformFeedback)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkPipelineRasterizationStateStreamCreateFlagsEXT")]
pub type PipelineRasterizationStateStreamCreateFlagsEXT = u32;

/// [`VkStreamDescriptorSurfaceCreateFlagsGGP`](https://docs.vulkan.org/refpages/latest/refpages/source/VkStreamDescriptorSurfaceCreateFlagsGGP.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`GGP_StreamDescriptorSurface`](Extension::GGP_StreamDescriptorSurface)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkStreamDescriptorSurfaceCreateFlagsGGP")]
pub type StreamDescriptorSurfaceCreateFlagsGGP = u32;

/// [`VkViSurfaceCreateFlagsNN`](https://docs.vulkan.org/refpages/latest/refpages/source/VkViSurfaceCreateFlagsNN.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NN_ViSurface`](Extension::NN_ViSurface)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkViSurfaceCreateFlagsNN")]
pub type ViSurfaceCreateFlagsNN = u32;

/// [`VkPipelineViewportSwizzleStateCreateFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportSwizzleStateCreateFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_ViewportSwizzle`](Extension::NV_ViewportSwizzle)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkPipelineViewportSwizzleStateCreateFlagsNV")]
pub type PipelineViewportSwizzleStateCreateFlagsNV = u32;

/// [`VkPipelineDiscardRectangleStateCreateFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineDiscardRectangleStateCreateFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DiscardRectangles`](Extension::EXT_DiscardRectangles)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkPipelineDiscardRectangleStateCreateFlagsEXT")]
pub type PipelineDiscardRectangleStateCreateFlagsEXT = u32;

/// [`VkPipelineRasterizationConservativeStateCreateFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationConservativeStateCreateFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_ConservativeRasterization`](Extension::EXT_ConservativeRasterization)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkPipelineRasterizationConservativeStateCreateFlagsEXT")]
pub type PipelineRasterizationConservativeStateCreateFlagsEXT = u32;

/// [`VkPipelineRasterizationDepthClipStateCreateFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationDepthClipStateCreateFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DepthClipEnable`](Extension::EXT_DepthClipEnable)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkPipelineRasterizationDepthClipStateCreateFlagsEXT")]
pub type PipelineRasterizationDepthClipStateCreateFlagsEXT = u32;

/// [`VkIOSSurfaceCreateFlagsMVK`](https://docs.vulkan.org/refpages/latest/refpages/source/VkIOSSurfaceCreateFlagsMVK.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`MVK_IosSurface`](Extension::MVK_IosSurface)
/// - Extension [`EXT_MetalSurface`](Extension::EXT_MetalSurface)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkIOSSurfaceCreateFlagsMVK")]
pub type IOSSurfaceCreateFlagsMVK = u32;

/// [`VkMacOSSurfaceCreateFlagsMVK`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMacOSSurfaceCreateFlagsMVK.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`MVK_MacosSurface`](Extension::MVK_MacosSurface)
/// - Extension [`EXT_MetalSurface`](Extension::EXT_MetalSurface)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkMacOSSurfaceCreateFlagsMVK")]
pub type MacOSSurfaceCreateFlagsMVK = u32;

/// [`VkDebugUtilsMessengerCallbackDataFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessengerCallbackDataFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DebugUtils`](Extension::EXT_DebugUtils)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkDebugUtilsMessengerCallbackDataFlagsEXT")]
pub type DebugUtilsMessengerCallbackDataFlagsEXT = u32;

/// [`VkDebugUtilsMessengerCreateFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessengerCreateFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DebugUtils`](Extension::EXT_DebugUtils)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkDebugUtilsMessengerCreateFlagsEXT")]
pub type DebugUtilsMessengerCreateFlagsEXT = u32;

/// [`VkGpaPerfBlockPropertiesFlagsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaPerfBlockPropertiesFlagsAMD.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`AMD_GpaInterface`](Extension::AMD_GpaInterface)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkGpaPerfBlockPropertiesFlagsAMD")]
pub type GpaPerfBlockPropertiesFlagsAMD = u32;

/// [`VkPhysicalDeviceGpaPropertiesFlagsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceGpaPropertiesFlagsAMD.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`AMD_GpaInterface`](Extension::AMD_GpaInterface)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkPhysicalDeviceGpaPropertiesFlagsAMD")]
pub type PhysicalDeviceGpaPropertiesFlagsAMD = u32;

/// [`VkPipelineCoverageToColorStateCreateFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCoverageToColorStateCreateFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_FragmentCoverageToColor`](Extension::NV_FragmentCoverageToColor)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkPipelineCoverageToColorStateCreateFlagsNV")]
pub type PipelineCoverageToColorStateCreateFlagsNV = u32;

/// [`VkPipelineCoverageModulationStateCreateFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCoverageModulationStateCreateFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_FramebufferMixedSamples`](Extension::NV_FramebufferMixedSamples)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkPipelineCoverageModulationStateCreateFlagsNV")]
pub type PipelineCoverageModulationStateCreateFlagsNV = u32;

/// [`VkValidationCacheCreateFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkValidationCacheCreateFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_ValidationCache`](Extension::EXT_ValidationCache)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkValidationCacheCreateFlagsEXT")]
pub type ValidationCacheCreateFlagsEXT = u32;

/// [`VkImagePipeSurfaceCreateFlagsFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImagePipeSurfaceCreateFlagsFUCHSIA.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`FUCHSIA_ImagepipeSurface`](Extension::FUCHSIA_ImagepipeSurface)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkImagePipeSurfaceCreateFlagsFUCHSIA")]
pub type ImagePipeSurfaceCreateFlagsFUCHSIA = u32;

/// [`VkMetalSurfaceCreateFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMetalSurfaceCreateFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_MetalSurface`](Extension::EXT_MetalSurface)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkMetalSurfaceCreateFlagsEXT")]
pub type MetalSurfaceCreateFlagsEXT = u32;

/// [`VkPipelineCoverageReductionStateCreateFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCoverageReductionStateCreateFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_CoverageReductionMode`](Extension::NV_CoverageReductionMode)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkPipelineCoverageReductionStateCreateFlagsNV")]
pub type PipelineCoverageReductionStateCreateFlagsNV = u32;

/// [`VkHeadlessSurfaceCreateFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkHeadlessSurfaceCreateFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_HeadlessSurface`](Extension::EXT_HeadlessSurface)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkHeadlessSurfaceCreateFlagsEXT")]
pub type HeadlessSurfaceCreateFlagsEXT = u32;

/// [`VkDeviceMemoryReportFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemoryReportFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DeviceMemoryReport`](Extension::EXT_DeviceMemoryReport)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkDeviceMemoryReportFlagsEXT")]
pub type DeviceMemoryReportFlagsEXT = u32;

/// [`VkAccelerationStructureMotionInfoFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMotionInfoFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_RayTracingMotionBlur`](Extension::NV_RayTracingMotionBlur)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkAccelerationStructureMotionInfoFlagsNV")]
pub type AccelerationStructureMotionInfoFlagsNV = u32;

/// [`VkAccelerationStructureMotionInstanceFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMotionInstanceFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_RayTracingMotionBlur`](Extension::NV_RayTracingMotionBlur)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkAccelerationStructureMotionInstanceFlagsNV")]
pub type AccelerationStructureMotionInstanceFlagsNV = u32;

/// [`VkDirectFBSurfaceCreateFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDirectFBSurfaceCreateFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DirectfbSurface`](Extension::EXT_DirectfbSurface)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkDirectFBSurfaceCreateFlagsEXT")]
pub type DirectFBSurfaceCreateFlagsEXT = u32;

/// [`VkImageFormatConstraintsFlagsFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageFormatConstraintsFlagsFUCHSIA.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`FUCHSIA_BufferCollection`](Extension::FUCHSIA_BufferCollection)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkImageFormatConstraintsFlagsFUCHSIA")]
pub type ImageFormatConstraintsFlagsFUCHSIA = u32;

/// [`VkScreenSurfaceCreateFlagsQNX`](https://docs.vulkan.org/refpages/latest/refpages/source/VkScreenSurfaceCreateFlagsQNX.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`QNX_ScreenSurface`](Extension::QNX_ScreenSurface)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkScreenSurfaceCreateFlagsQNX")]
pub type ScreenSurfaceCreateFlagsQNX = u32;

/// [`VkDirectDriverLoadingFlagsLUNARG`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDirectDriverLoadingFlagsLUNARG.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`LUNARG_DirectDriverLoading`](Extension::LUNARG_DirectDriverLoading)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkDirectDriverLoadingFlagsLUNARG")]
pub type DirectDriverLoadingFlagsLUNARG = u32;

/// [`VkSurfaceCreateFlagsOHOS`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCreateFlagsOHOS.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`OHOS_Surface`](Extension::OHOS_Surface)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkSurfaceCreateFlagsOHOS")]
pub type SurfaceCreateFlagsOHOS = u32;

/// [`VkPerformanceCounterDescriptionFlagsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterDescriptionFlagsARM.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`ARM_PerformanceCountersByRegion`](Extension::ARM_PerformanceCountersByRegion)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkPerformanceCounterDescriptionFlagsARM")]
pub type PerformanceCounterDescriptionFlagsARM = u32;

/// [`VkShaderInstrumentationValuesFlagsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderInstrumentationValuesFlagsARM.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`ARM_ShaderInstrumentation`](Extension::ARM_ShaderInstrumentation)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkShaderInstrumentationValuesFlagsARM")]
pub type ShaderInstrumentationValuesFlagsARM = u32;

/// [`VkUbmSurfaceCreateFlagsSEC`](https://docs.vulkan.org/refpages/latest/refpages/source/VkUbmSurfaceCreateFlagsSEC.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`SEC_UbmSurface`](Extension::SEC_UbmSurface)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
#[doc(alias = "VkUbmSurfaceCreateFlagsSEC")]
pub type UbmSurfaceCreateFlagsSEC = u32;
