// WARNING: AUTO GENERATED MODULE
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::loader::*;
use crate::platform::*;
use std::ffi::{c_char, c_int, c_uint, c_void};

use crate::enums::*;

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
/// - Extension [`KHR_Display`](Extensions::KHR_Display)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkDisplayModeCreateFlagsKHR")]
pub type DisplayModeCreateFlagsKHR = u32;

/// [`VkDisplaySurfaceCreateFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplaySurfaceCreateFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_Display`](Extensions::KHR_Display)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkDisplaySurfaceCreateFlagsKHR")]
pub type DisplaySurfaceCreateFlagsKHR = u32;

/// [`VkXlibSurfaceCreateFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkXlibSurfaceCreateFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_XlibSurface`](Extensions::KHR_XlibSurface)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkXlibSurfaceCreateFlagsKHR")]
pub type XlibSurfaceCreateFlagsKHR = u32;

/// [`VkXcbSurfaceCreateFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkXcbSurfaceCreateFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_XcbSurface`](Extensions::KHR_XcbSurface)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkXcbSurfaceCreateFlagsKHR")]
pub type XcbSurfaceCreateFlagsKHR = u32;

/// [`VkWaylandSurfaceCreateFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkWaylandSurfaceCreateFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_WaylandSurface`](Extensions::KHR_WaylandSurface)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkWaylandSurfaceCreateFlagsKHR")]
pub type WaylandSurfaceCreateFlagsKHR = u32;

/// [`VkAndroidSurfaceCreateFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAndroidSurfaceCreateFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_AndroidSurface`](Extensions::KHR_AndroidSurface)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkAndroidSurfaceCreateFlagsKHR")]
pub type AndroidSurfaceCreateFlagsKHR = u32;

/// [`VkWin32SurfaceCreateFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkWin32SurfaceCreateFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_Win32Surface`](Extensions::KHR_Win32Surface)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkWin32SurfaceCreateFlagsKHR")]
pub type Win32SurfaceCreateFlagsKHR = u32;

/// [`VkVideoBeginCodingFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoBeginCodingFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoQueue`](Extensions::KHR_VideoQueue)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkVideoBeginCodingFlagsKHR")]
pub type VideoBeginCodingFlagsKHR = u32;

/// [`VkVideoEndCodingFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEndCodingFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoQueue`](Extensions::KHR_VideoQueue)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkVideoEndCodingFlagsKHR")]
pub type VideoEndCodingFlagsKHR = u32;

/// [`VkVideoDecodeFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoDecodeQueue`](Extensions::KHR_VideoDecodeQueue)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkVideoDecodeFlagsKHR")]
pub type VideoDecodeFlagsKHR = u32;

/// [`VkVideoEncodeRateControlFlagsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRateControlFlagsKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoEncodeQueue`](Extensions::KHR_VideoEncodeQueue)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkVideoEncodeRateControlFlagsKHR")]
pub type VideoEncodeRateControlFlagsKHR = u32;

/// [`VkPipelineRasterizationStateStreamCreateFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationStateStreamCreateFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_TransformFeedback`](Extensions::EXT_TransformFeedback)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkPipelineRasterizationStateStreamCreateFlagsEXT")]
pub type PipelineRasterizationStateStreamCreateFlagsEXT = u32;

/// [`VkStreamDescriptorSurfaceCreateFlagsGGP`](https://docs.vulkan.org/refpages/latest/refpages/source/VkStreamDescriptorSurfaceCreateFlagsGGP.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`GGP_StreamDescriptorSurface`](Extensions::GGP_StreamDescriptorSurface)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkStreamDescriptorSurfaceCreateFlagsGGP")]
pub type StreamDescriptorSurfaceCreateFlagsGGP = u32;

/// [`VkViSurfaceCreateFlagsNN`](https://docs.vulkan.org/refpages/latest/refpages/source/VkViSurfaceCreateFlagsNN.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NN_ViSurface`](Extensions::NN_ViSurface)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkViSurfaceCreateFlagsNN")]
pub type ViSurfaceCreateFlagsNN = u32;

/// [`VkPipelineViewportSwizzleStateCreateFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportSwizzleStateCreateFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_ViewportSwizzle`](Extensions::NV_ViewportSwizzle)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkPipelineViewportSwizzleStateCreateFlagsNV")]
pub type PipelineViewportSwizzleStateCreateFlagsNV = u32;

/// [`VkPipelineDiscardRectangleStateCreateFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineDiscardRectangleStateCreateFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DiscardRectangles`](Extensions::EXT_DiscardRectangles)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkPipelineDiscardRectangleStateCreateFlagsEXT")]
pub type PipelineDiscardRectangleStateCreateFlagsEXT = u32;

/// [`VkPipelineRasterizationConservativeStateCreateFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationConservativeStateCreateFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_ConservativeRasterization`](Extensions::EXT_ConservativeRasterization)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkPipelineRasterizationConservativeStateCreateFlagsEXT")]
pub type PipelineRasterizationConservativeStateCreateFlagsEXT = u32;

/// [`VkPipelineRasterizationDepthClipStateCreateFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationDepthClipStateCreateFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DepthClipEnable`](Extensions::EXT_DepthClipEnable)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkPipelineRasterizationDepthClipStateCreateFlagsEXT")]
pub type PipelineRasterizationDepthClipStateCreateFlagsEXT = u32;

/// [`VkIOSSurfaceCreateFlagsMVK`](https://docs.vulkan.org/refpages/latest/refpages/source/VkIOSSurfaceCreateFlagsMVK.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`MVK_IosSurface`](Extensions::MVK_IosSurface)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkIOSSurfaceCreateFlagsMVK")]
pub type IOSSurfaceCreateFlagsMVK = u32;

/// [`VkMacOSSurfaceCreateFlagsMVK`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMacOSSurfaceCreateFlagsMVK.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`MVK_MacosSurface`](Extensions::MVK_MacosSurface)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkMacOSSurfaceCreateFlagsMVK")]
pub type MacOSSurfaceCreateFlagsMVK = u32;

/// [`VkDebugUtilsMessengerCallbackDataFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessengerCallbackDataFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DebugUtils`](Extensions::EXT_DebugUtils)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkDebugUtilsMessengerCallbackDataFlagsEXT")]
pub type DebugUtilsMessengerCallbackDataFlagsEXT = u32;

/// [`VkDebugUtilsMessengerCreateFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessengerCreateFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DebugUtils`](Extensions::EXT_DebugUtils)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkDebugUtilsMessengerCreateFlagsEXT")]
pub type DebugUtilsMessengerCreateFlagsEXT = u32;

/// [`VkGpaPerfBlockPropertiesFlagsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaPerfBlockPropertiesFlagsAMD.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`AMD_GpaInterface`](Extensions::AMD_GpaInterface)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkGpaPerfBlockPropertiesFlagsAMD")]
pub type GpaPerfBlockPropertiesFlagsAMD = u32;

/// [`VkPhysicalDeviceGpaPropertiesFlagsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceGpaPropertiesFlagsAMD.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`AMD_GpaInterface`](Extensions::AMD_GpaInterface)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkPhysicalDeviceGpaPropertiesFlagsAMD")]
pub type PhysicalDeviceGpaPropertiesFlagsAMD = u32;

/// [`VkPipelineCoverageToColorStateCreateFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCoverageToColorStateCreateFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_FragmentCoverageToColor`](Extensions::NV_FragmentCoverageToColor)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkPipelineCoverageToColorStateCreateFlagsNV")]
pub type PipelineCoverageToColorStateCreateFlagsNV = u32;

/// [`VkPipelineCoverageModulationStateCreateFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCoverageModulationStateCreateFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_FramebufferMixedSamples`](Extensions::NV_FramebufferMixedSamples)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkPipelineCoverageModulationStateCreateFlagsNV")]
pub type PipelineCoverageModulationStateCreateFlagsNV = u32;

/// [`VkValidationCacheCreateFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkValidationCacheCreateFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_ValidationCache`](Extensions::EXT_ValidationCache)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkValidationCacheCreateFlagsEXT")]
pub type ValidationCacheCreateFlagsEXT = u32;

/// [`VkImagePipeSurfaceCreateFlagsFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImagePipeSurfaceCreateFlagsFUCHSIA.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`FUCHSIA_ImagepipeSurface`](Extensions::FUCHSIA_ImagepipeSurface)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkImagePipeSurfaceCreateFlagsFUCHSIA")]
pub type ImagePipeSurfaceCreateFlagsFUCHSIA = u32;

/// [`VkMetalSurfaceCreateFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMetalSurfaceCreateFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_MetalSurface`](Extensions::EXT_MetalSurface)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkMetalSurfaceCreateFlagsEXT")]
pub type MetalSurfaceCreateFlagsEXT = u32;

/// [`VkPipelineCoverageReductionStateCreateFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCoverageReductionStateCreateFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_CoverageReductionMode`](Extensions::NV_CoverageReductionMode)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkPipelineCoverageReductionStateCreateFlagsNV")]
pub type PipelineCoverageReductionStateCreateFlagsNV = u32;

/// [`VkHeadlessSurfaceCreateFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkHeadlessSurfaceCreateFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_HeadlessSurface`](Extensions::EXT_HeadlessSurface)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkHeadlessSurfaceCreateFlagsEXT")]
pub type HeadlessSurfaceCreateFlagsEXT = u32;

/// [`VkDeviceMemoryReportFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemoryReportFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DeviceMemoryReport`](Extensions::EXT_DeviceMemoryReport)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkDeviceMemoryReportFlagsEXT")]
pub type DeviceMemoryReportFlagsEXT = u32;

/// [`VkAccelerationStructureMotionInfoFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMotionInfoFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_RayTracingMotionBlur`](Extensions::NV_RayTracingMotionBlur)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkAccelerationStructureMotionInfoFlagsNV")]
pub type AccelerationStructureMotionInfoFlagsNV = u32;

/// [`VkAccelerationStructureMotionInstanceFlagsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMotionInstanceFlagsNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_RayTracingMotionBlur`](Extensions::NV_RayTracingMotionBlur)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkAccelerationStructureMotionInstanceFlagsNV")]
pub type AccelerationStructureMotionInstanceFlagsNV = u32;

/// [`VkDirectFBSurfaceCreateFlagsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDirectFBSurfaceCreateFlagsEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DirectfbSurface`](Extensions::EXT_DirectfbSurface)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkDirectFBSurfaceCreateFlagsEXT")]
pub type DirectFBSurfaceCreateFlagsEXT = u32;

/// [`VkImageFormatConstraintsFlagsFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageFormatConstraintsFlagsFUCHSIA.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`FUCHSIA_BufferCollection`](Extensions::FUCHSIA_BufferCollection)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkImageFormatConstraintsFlagsFUCHSIA")]
pub type ImageFormatConstraintsFlagsFUCHSIA = u32;

/// [`VkScreenSurfaceCreateFlagsQNX`](https://docs.vulkan.org/refpages/latest/refpages/source/VkScreenSurfaceCreateFlagsQNX.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`QNX_ScreenSurface`](Extensions::QNX_ScreenSurface)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkScreenSurfaceCreateFlagsQNX")]
pub type ScreenSurfaceCreateFlagsQNX = u32;

/// [`VkDirectDriverLoadingFlagsLUNARG`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDirectDriverLoadingFlagsLUNARG.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`LUNARG_DirectDriverLoading`](Extensions::LUNARG_DirectDriverLoading)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkDirectDriverLoadingFlagsLUNARG")]
pub type DirectDriverLoadingFlagsLUNARG = u32;

/// [`VkSurfaceCreateFlagsOHOS`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCreateFlagsOHOS.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`OHOS_Surface`](Extensions::OHOS_Surface)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkSurfaceCreateFlagsOHOS")]
pub type SurfaceCreateFlagsOHOS = u32;

/// [`VkPerformanceCounterDescriptionFlagsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterDescriptionFlagsARM.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`ARM_PerformanceCountersByRegion`](Extensions::ARM_PerformanceCountersByRegion)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
/// # Returned only
/// This type is only returned by Vulkan, never constructed by the API user.
#[doc(alias = "VkPerformanceCounterDescriptionFlagsARM")]
pub type PerformanceCounterDescriptionFlagsARM = u32;

/// [`VkShaderInstrumentationValuesFlagsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderInstrumentationValuesFlagsARM.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`ARM_ShaderInstrumentation`](Extensions::ARM_ShaderInstrumentation)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkShaderInstrumentationValuesFlagsARM")]
pub type ShaderInstrumentationValuesFlagsARM = u32;

/// [`VkUbmSurfaceCreateFlagsSEC`](https://docs.vulkan.org/refpages/latest/refpages/source/VkUbmSurfaceCreateFlagsSEC.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`SEC_UbmSurface`](Extensions::SEC_UbmSurface)
///
/// Note this is not an exhaustive requirement list. For more information check vulkan documentation.
///
#[doc(alias = "VkUbmSurfaceCreateFlagsSEC")]
pub type UbmSurfaceCreateFlagsSEC = u32;
