// WARNING: AUTO GENERATED MODULE
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::manual::*;
use crate::platform::*;
use std::ffi::{c_char, c_int, c_uint, c_void};
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceCreateFlags.html>
#[doc(alias = "VkDeviceCreateFlags")]
pub type DeviceCreateFlags = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreCreateFlags.html>
#[doc(alias = "VkSemaphoreCreateFlags")]
pub type SemaphoreCreateFlags = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferViewCreateFlags.html>
#[doc(alias = "VkBufferViewCreateFlags")]
pub type BufferViewCreateFlags = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderModuleCreateFlags.html>
#[doc(alias = "VkShaderModuleCreateFlags")]
pub type ShaderModuleCreateFlags = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPoolResetFlags.html>
#[doc(alias = "VkDescriptorPoolResetFlags")]
pub type DescriptorPoolResetFlags = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineDynamicStateCreateFlags.html>
#[doc(alias = "VkPipelineDynamicStateCreateFlags")]
pub type PipelineDynamicStateCreateFlags = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineInputAssemblyStateCreateFlags.html>
#[doc(alias = "VkPipelineInputAssemblyStateCreateFlags")]
pub type PipelineInputAssemblyStateCreateFlags = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineMultisampleStateCreateFlags.html>
#[doc(alias = "VkPipelineMultisampleStateCreateFlags")]
pub type PipelineMultisampleStateCreateFlags = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationStateCreateFlags.html>
#[doc(alias = "VkPipelineRasterizationStateCreateFlags")]
pub type PipelineRasterizationStateCreateFlags = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineTessellationStateCreateFlags.html>
#[doc(alias = "VkPipelineTessellationStateCreateFlags")]
pub type PipelineTessellationStateCreateFlags = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineVertexInputStateCreateFlags.html>
#[doc(alias = "VkPipelineVertexInputStateCreateFlags")]
pub type PipelineVertexInputStateCreateFlags = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportStateCreateFlags.html>
#[doc(alias = "VkPipelineViewportStateCreateFlags")]
pub type PipelineViewportStateCreateFlags = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolTrimFlags.html>
#[doc(alias = "VkCommandPoolTrimFlags")]
pub type CommandPoolTrimFlags = u32;
pub type CommandPoolTrimFlagsKHR = CommandPoolTrimFlags;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplateCreateFlags.html>
#[doc(alias = "VkDescriptorUpdateTemplateCreateFlags")]
pub type DescriptorUpdateTemplateCreateFlags = u32;
pub type DescriptorUpdateTemplateCreateFlagsKHR = DescriptorUpdateTemplateCreateFlags;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayModeCreateFlagsKHR.html>
#[doc(alias = "VkDisplayModeCreateFlagsKHR")]
pub type DisplayModeCreateFlagsKHR = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplaySurfaceCreateFlagsKHR.html>
#[doc(alias = "VkDisplaySurfaceCreateFlagsKHR")]
pub type DisplaySurfaceCreateFlagsKHR = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkXlibSurfaceCreateFlagsKHR.html>
#[doc(alias = "VkXlibSurfaceCreateFlagsKHR")]
pub type XlibSurfaceCreateFlagsKHR = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkXcbSurfaceCreateFlagsKHR.html>
#[doc(alias = "VkXcbSurfaceCreateFlagsKHR")]
pub type XcbSurfaceCreateFlagsKHR = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkWaylandSurfaceCreateFlagsKHR.html>
#[doc(alias = "VkWaylandSurfaceCreateFlagsKHR")]
pub type WaylandSurfaceCreateFlagsKHR = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAndroidSurfaceCreateFlagsKHR.html>
#[doc(alias = "VkAndroidSurfaceCreateFlagsKHR")]
pub type AndroidSurfaceCreateFlagsKHR = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkWin32SurfaceCreateFlagsKHR.html>
#[doc(alias = "VkWin32SurfaceCreateFlagsKHR")]
pub type Win32SurfaceCreateFlagsKHR = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoBeginCodingFlagsKHR.html>
#[doc(alias = "VkVideoBeginCodingFlagsKHR")]
pub type VideoBeginCodingFlagsKHR = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEndCodingFlagsKHR.html>
#[doc(alias = "VkVideoEndCodingFlagsKHR")]
pub type VideoEndCodingFlagsKHR = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeFlagsKHR.html>
#[doc(alias = "VkVideoDecodeFlagsKHR")]
pub type VideoDecodeFlagsKHR = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRateControlFlagsKHR.html>
#[doc(alias = "VkVideoEncodeRateControlFlagsKHR")]
pub type VideoEncodeRateControlFlagsKHR = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationStateStreamCreateFlagsEXT.html>
#[doc(alias = "VkPipelineRasterizationStateStreamCreateFlagsEXT")]
pub type PipelineRasterizationStateStreamCreateFlagsEXT = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkStreamDescriptorSurfaceCreateFlagsGGP.html>
#[doc(alias = "VkStreamDescriptorSurfaceCreateFlagsGGP")]
pub type StreamDescriptorSurfaceCreateFlagsGGP = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkViSurfaceCreateFlagsNN.html>
#[doc(alias = "VkViSurfaceCreateFlagsNN")]
pub type ViSurfaceCreateFlagsNN = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportSwizzleStateCreateFlagsNV.html>
#[doc(alias = "VkPipelineViewportSwizzleStateCreateFlagsNV")]
pub type PipelineViewportSwizzleStateCreateFlagsNV = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineDiscardRectangleStateCreateFlagsEXT.html>
#[doc(alias = "VkPipelineDiscardRectangleStateCreateFlagsEXT")]
pub type PipelineDiscardRectangleStateCreateFlagsEXT = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationConservativeStateCreateFlagsEXT.html>
#[doc(alias = "VkPipelineRasterizationConservativeStateCreateFlagsEXT")]
pub type PipelineRasterizationConservativeStateCreateFlagsEXT = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationDepthClipStateCreateFlagsEXT.html>
#[doc(alias = "VkPipelineRasterizationDepthClipStateCreateFlagsEXT")]
pub type PipelineRasterizationDepthClipStateCreateFlagsEXT = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIOSSurfaceCreateFlagsMVK.html>
#[doc(alias = "VkIOSSurfaceCreateFlagsMVK")]
pub type IOSSurfaceCreateFlagsMVK = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMacOSSurfaceCreateFlagsMVK.html>
#[doc(alias = "VkMacOSSurfaceCreateFlagsMVK")]
pub type MacOSSurfaceCreateFlagsMVK = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessengerCallbackDataFlagsEXT.html>
#[doc(alias = "VkDebugUtilsMessengerCallbackDataFlagsEXT")]
pub type DebugUtilsMessengerCallbackDataFlagsEXT = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessengerCreateFlagsEXT.html>
#[doc(alias = "VkDebugUtilsMessengerCreateFlagsEXT")]
pub type DebugUtilsMessengerCreateFlagsEXT = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaPerfBlockPropertiesFlagsAMD.html>
#[doc(alias = "VkGpaPerfBlockPropertiesFlagsAMD")]
pub type GpaPerfBlockPropertiesFlagsAMD = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceGpaPropertiesFlagsAMD.html>
#[doc(alias = "VkPhysicalDeviceGpaPropertiesFlagsAMD")]
pub type PhysicalDeviceGpaPropertiesFlagsAMD = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCoverageToColorStateCreateFlagsNV.html>
#[doc(alias = "VkPipelineCoverageToColorStateCreateFlagsNV")]
pub type PipelineCoverageToColorStateCreateFlagsNV = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCoverageModulationStateCreateFlagsNV.html>
#[doc(alias = "VkPipelineCoverageModulationStateCreateFlagsNV")]
pub type PipelineCoverageModulationStateCreateFlagsNV = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkValidationCacheCreateFlagsEXT.html>
#[doc(alias = "VkValidationCacheCreateFlagsEXT")]
pub type ValidationCacheCreateFlagsEXT = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImagePipeSurfaceCreateFlagsFUCHSIA.html>
#[doc(alias = "VkImagePipeSurfaceCreateFlagsFUCHSIA")]
pub type ImagePipeSurfaceCreateFlagsFUCHSIA = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMetalSurfaceCreateFlagsEXT.html>
#[doc(alias = "VkMetalSurfaceCreateFlagsEXT")]
pub type MetalSurfaceCreateFlagsEXT = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCoverageReductionStateCreateFlagsNV.html>
#[doc(alias = "VkPipelineCoverageReductionStateCreateFlagsNV")]
pub type PipelineCoverageReductionStateCreateFlagsNV = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkHeadlessSurfaceCreateFlagsEXT.html>
#[doc(alias = "VkHeadlessSurfaceCreateFlagsEXT")]
pub type HeadlessSurfaceCreateFlagsEXT = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemoryReportFlagsEXT.html>
#[doc(alias = "VkDeviceMemoryReportFlagsEXT")]
pub type DeviceMemoryReportFlagsEXT = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMotionInfoFlagsNV.html>
#[doc(alias = "VkAccelerationStructureMotionInfoFlagsNV")]
pub type AccelerationStructureMotionInfoFlagsNV = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMotionInstanceFlagsNV.html>
#[doc(alias = "VkAccelerationStructureMotionInstanceFlagsNV")]
pub type AccelerationStructureMotionInstanceFlagsNV = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDirectFBSurfaceCreateFlagsEXT.html>
#[doc(alias = "VkDirectFBSurfaceCreateFlagsEXT")]
pub type DirectFBSurfaceCreateFlagsEXT = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageFormatConstraintsFlagsFUCHSIA.html>
#[doc(alias = "VkImageFormatConstraintsFlagsFUCHSIA")]
pub type ImageFormatConstraintsFlagsFUCHSIA = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkScreenSurfaceCreateFlagsQNX.html>
#[doc(alias = "VkScreenSurfaceCreateFlagsQNX")]
pub type ScreenSurfaceCreateFlagsQNX = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDirectDriverLoadingFlagsLUNARG.html>
#[doc(alias = "VkDirectDriverLoadingFlagsLUNARG")]
pub type DirectDriverLoadingFlagsLUNARG = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCreateFlagsOHOS.html>
#[doc(alias = "VkSurfaceCreateFlagsOHOS")]
pub type SurfaceCreateFlagsOHOS = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterDescriptionFlagsARM.html>
#[doc(alias = "VkPerformanceCounterDescriptionFlagsARM")]
pub type PerformanceCounterDescriptionFlagsARM = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderInstrumentationValuesFlagsARM.html>
#[doc(alias = "VkShaderInstrumentationValuesFlagsARM")]
pub type ShaderInstrumentationValuesFlagsARM = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkUbmSurfaceCreateFlagsSEC.html>
#[doc(alias = "VkUbmSurfaceCreateFlagsSEC")]
pub type UbmSurfaceCreateFlagsSEC = u32;
