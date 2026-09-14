 // WARNING: AUTO GENERATED MODULE
#![allow(nonstandard_style)]
#![allow(unused_imports)]

use std::ffi::{c_void, c_int, c_uint, c_char};
use crate::inner::*;
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceCreateFlags.html>
pub type DeviceCreateFlags = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreCreateFlags.html>
pub type SemaphoreCreateFlags = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferViewCreateFlags.html>
pub type BufferViewCreateFlags = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderModuleCreateFlags.html>
pub type ShaderModuleCreateFlags = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPoolResetFlags.html>
pub type DescriptorPoolResetFlags = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineDynamicStateCreateFlags.html>
pub type PipelineDynamicStateCreateFlags = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineInputAssemblyStateCreateFlags.html>
pub type PipelineInputAssemblyStateCreateFlags = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineMultisampleStateCreateFlags.html>
pub type PipelineMultisampleStateCreateFlags = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationStateCreateFlags.html>
pub type PipelineRasterizationStateCreateFlags = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineTessellationStateCreateFlags.html>
pub type PipelineTessellationStateCreateFlags = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineVertexInputStateCreateFlags.html>
pub type PipelineVertexInputStateCreateFlags = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportStateCreateFlags.html>
pub type PipelineViewportStateCreateFlags = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolTrimFlags.html>
pub type CommandPoolTrimFlags = u32;
pub type CommandPoolTrimFlagsKHR = CommandPoolTrimFlags;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplateCreateFlags.html>
pub type DescriptorUpdateTemplateCreateFlags = u32;
pub type DescriptorUpdateTemplateCreateFlagsKHR = DescriptorUpdateTemplateCreateFlags;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayModeCreateFlagsKHR.html>
pub type DisplayModeCreateFlagsKHR = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplaySurfaceCreateFlagsKHR.html>
pub type DisplaySurfaceCreateFlagsKHR = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkXlibSurfaceCreateFlagsKHR.html>
pub type XlibSurfaceCreateFlagsKHR = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkXcbSurfaceCreateFlagsKHR.html>
pub type XcbSurfaceCreateFlagsKHR = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkWaylandSurfaceCreateFlagsKHR.html>
pub type WaylandSurfaceCreateFlagsKHR = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAndroidSurfaceCreateFlagsKHR.html>
pub type AndroidSurfaceCreateFlagsKHR = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkWin32SurfaceCreateFlagsKHR.html>
pub type Win32SurfaceCreateFlagsKHR = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoBeginCodingFlagsKHR.html>
pub type VideoBeginCodingFlagsKHR = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEndCodingFlagsKHR.html>
pub type VideoEndCodingFlagsKHR = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeFlagsKHR.html>
pub type VideoDecodeFlagsKHR = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRateControlFlagsKHR.html>
pub type VideoEncodeRateControlFlagsKHR = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationStateStreamCreateFlagsEXT.html>
pub type PipelineRasterizationStateStreamCreateFlagsEXT = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkStreamDescriptorSurfaceCreateFlagsGGP.html>
pub type StreamDescriptorSurfaceCreateFlagsGGP = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkViSurfaceCreateFlagsNN.html>
pub type ViSurfaceCreateFlagsNN = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportSwizzleStateCreateFlagsNV.html>
pub type PipelineViewportSwizzleStateCreateFlagsNV = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineDiscardRectangleStateCreateFlagsEXT.html>
pub type PipelineDiscardRectangleStateCreateFlagsEXT = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationConservativeStateCreateFlagsEXT.html>
pub type PipelineRasterizationConservativeStateCreateFlagsEXT = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationDepthClipStateCreateFlagsEXT.html>
pub type PipelineRasterizationDepthClipStateCreateFlagsEXT = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIOSSurfaceCreateFlagsMVK.html>
pub type IOSSurfaceCreateFlagsMVK = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMacOSSurfaceCreateFlagsMVK.html>
pub type MacOSSurfaceCreateFlagsMVK = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessengerCallbackDataFlagsEXT.html>
pub type DebugUtilsMessengerCallbackDataFlagsEXT = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessengerCreateFlagsEXT.html>
pub type DebugUtilsMessengerCreateFlagsEXT = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaPerfBlockPropertiesFlagsAMD.html>
pub type GpaPerfBlockPropertiesFlagsAMD = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceGpaPropertiesFlagsAMD.html>
pub type PhysicalDeviceGpaPropertiesFlagsAMD = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCoverageToColorStateCreateFlagsNV.html>
pub type PipelineCoverageToColorStateCreateFlagsNV = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCoverageModulationStateCreateFlagsNV.html>
pub type PipelineCoverageModulationStateCreateFlagsNV = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkValidationCacheCreateFlagsEXT.html>
pub type ValidationCacheCreateFlagsEXT = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImagePipeSurfaceCreateFlagsFUCHSIA.html>
pub type ImagePipeSurfaceCreateFlagsFUCHSIA = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMetalSurfaceCreateFlagsEXT.html>
pub type MetalSurfaceCreateFlagsEXT = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCoverageReductionStateCreateFlagsNV.html>
pub type PipelineCoverageReductionStateCreateFlagsNV = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkHeadlessSurfaceCreateFlagsEXT.html>
pub type HeadlessSurfaceCreateFlagsEXT = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemoryReportFlagsEXT.html>
pub type DeviceMemoryReportFlagsEXT = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMotionInfoFlagsNV.html>
pub type AccelerationStructureMotionInfoFlagsNV = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMotionInstanceFlagsNV.html>
pub type AccelerationStructureMotionInstanceFlagsNV = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDirectFBSurfaceCreateFlagsEXT.html>
pub type DirectFBSurfaceCreateFlagsEXT = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageFormatConstraintsFlagsFUCHSIA.html>
pub type ImageFormatConstraintsFlagsFUCHSIA = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkScreenSurfaceCreateFlagsQNX.html>
pub type ScreenSurfaceCreateFlagsQNX = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDirectDriverLoadingFlagsLUNARG.html>
pub type DirectDriverLoadingFlagsLUNARG = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCreateFlagsOHOS.html>
pub type SurfaceCreateFlagsOHOS = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterDescriptionFlagsARM.html>
pub type PerformanceCounterDescriptionFlagsARM = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderInstrumentationValuesFlagsARM.html>
pub type ShaderInstrumentationValuesFlagsARM = u32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkUbmSurfaceCreateFlagsSEC.html>
pub type UbmSurfaceCreateFlagsSEC = u32;
