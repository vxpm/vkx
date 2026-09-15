// WARNING: AUTO GENERATED MODULE
#![allow(nonstandard_style)]
#![allow(unused_imports)]

use crate::manual::*;
use crate::platform::*;
use std::ffi::{c_char, c_int, c_uint, c_void};
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkInstance.html>
#[doc(alias = "VkInstance")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Instance(usize);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevice.html>
#[doc(alias = "VkPhysicalDevice")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PhysicalDevice(usize);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDevice.html>
#[doc(alias = "VkDevice")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Device(usize);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueue.html>
#[doc(alias = "VkQueue")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Queue(usize);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphore.html>
#[doc(alias = "VkSemaphore")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Semaphore(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBuffer.html>
#[doc(alias = "VkCommandBuffer")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CommandBuffer(usize);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFence.html>
#[doc(alias = "VkFence")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Fence(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemory.html>
#[doc(alias = "VkDeviceMemory")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DeviceMemory(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBuffer.html>
#[doc(alias = "VkBuffer")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Buffer(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImage.html>
#[doc(alias = "VkImage")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Image(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPool.html>
#[doc(alias = "VkQueryPool")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct QueryPool(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageView.html>
#[doc(alias = "VkImageView")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ImageView(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPool.html>
#[doc(alias = "VkCommandPool")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CommandPool(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPass.html>
#[doc(alias = "VkRenderPass")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct RenderPass(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebuffer.html>
#[doc(alias = "VkFramebuffer")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Framebuffer(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkEvent.html>
#[doc(alias = "VkEvent")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Event(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferView.html>
#[doc(alias = "VkBufferView")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct BufferView(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderModule.html>
#[doc(alias = "VkShaderModule")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ShaderModule(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCache.html>
#[doc(alias = "VkPipelineCache")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PipelineCache(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipeline.html>
#[doc(alias = "VkPipeline")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Pipeline(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineLayout.html>
#[doc(alias = "VkPipelineLayout")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PipelineLayout(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayout.html>
#[doc(alias = "VkDescriptorSetLayout")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DescriptorSetLayout(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSampler.html>
#[doc(alias = "VkSampler")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Sampler(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSet.html>
#[doc(alias = "VkDescriptorSet")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DescriptorSet(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPool.html>
#[doc(alias = "VkDescriptorPool")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DescriptorPool(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplate.html>
#[doc(alias = "VkDescriptorUpdateTemplate")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DescriptorUpdateTemplate(u64);
pub type DescriptorUpdateTemplateKHR = DescriptorUpdateTemplate;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversion.html>
#[doc(alias = "VkSamplerYcbcrConversion")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct SamplerYcbcrConversion(u64);
pub type SamplerYcbcrConversionKHR = SamplerYcbcrConversion;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlot.html>
#[doc(alias = "VkPrivateDataSlot")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PrivateDataSlot(u64);
pub type PrivateDataSlotEXT = PrivateDataSlot;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceKHR.html>
#[doc(alias = "VkSurfaceKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct SurfaceKHR(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainKHR.html>
#[doc(alias = "VkSwapchainKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct SwapchainKHR(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayKHR.html>
#[doc(alias = "VkDisplayKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DisplayKHR(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayModeKHR.html>
#[doc(alias = "VkDisplayModeKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DisplayModeKHR(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionKHR.html>
#[doc(alias = "VkVideoSessionKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct VideoSessionKHR(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionParametersKHR.html>
#[doc(alias = "VkVideoSessionParametersKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct VideoSessionParametersKHR(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeferredOperationKHR.html>
#[doc(alias = "VkDeferredOperationKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DeferredOperationKHR(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureKHR.html>
#[doc(alias = "VkAccelerationStructureKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct AccelerationStructureKHR(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBinaryKHR.html>
#[doc(alias = "VkPipelineBinaryKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PipelineBinaryKHR(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugReportCallbackEXT.html>
#[doc(alias = "VkDebugReportCallbackEXT")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DebugReportCallbackEXT(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCuModuleNVX.html>
#[doc(alias = "VkCuModuleNVX")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CuModuleNVX(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCuFunctionNVX.html>
#[doc(alias = "VkCuFunctionNVX")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CuFunctionNVX(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessengerEXT.html>
#[doc(alias = "VkDebugUtilsMessengerEXT")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DebugUtilsMessengerEXT(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaSessionAMD.html>
#[doc(alias = "VkGpaSessionAMD")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct GpaSessionAMD(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorARM.html>
#[doc(alias = "VkTensorARM")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct TensorARM(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkValidationCacheEXT.html>
#[doc(alias = "VkValidationCacheEXT")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ValidationCacheEXT(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureNV.html>
#[doc(alias = "VkAccelerationStructureNV")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct AccelerationStructureNV(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceConfigurationINTEL.html>
#[doc(alias = "VkPerformanceConfigurationINTEL")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PerformanceConfigurationINTEL(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutNV.html>
#[doc(alias = "VkIndirectCommandsLayoutNV")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct IndirectCommandsLayoutNV(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCudaModuleNV.html>
#[doc(alias = "VkCudaModuleNV")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CudaModuleNV(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCudaFunctionNV.html>
#[doc(alias = "VkCudaFunctionNV")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CudaFunctionNV(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCollectionFUCHSIA.html>
#[doc(alias = "VkBufferCollectionFUCHSIA")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct BufferCollectionFUCHSIA(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapEXT.html>
#[doc(alias = "VkMicromapEXT")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct MicromapEXT(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorViewARM.html>
#[doc(alias = "VkTensorViewARM")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct TensorViewARM(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowSessionNV.html>
#[doc(alias = "VkOpticalFlowSessionNV")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct OpticalFlowSessionNV(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderEXT.html>
#[doc(alias = "VkShaderEXT")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ShaderEXT(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionARM.html>
#[doc(alias = "VkDataGraphPipelineSessionARM")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DataGraphPipelineSessionARM(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalComputeQueueNV.html>
#[doc(alias = "VkExternalComputeQueueNV")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ExternalComputeQueueNV(usize);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectExecutionSetEXT.html>
#[doc(alias = "VkIndirectExecutionSetEXT")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct IndirectExecutionSetEXT(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutEXT.html>
#[doc(alias = "VkIndirectCommandsLayoutEXT")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct IndirectCommandsLayoutEXT(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderInstrumentationARM.html>
#[doc(alias = "VkShaderInstrumentationARM")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ShaderInstrumentationARM(u64);
