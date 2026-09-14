// WARNING: AUTO GENERATED MODULE
#![allow(nonstandard_style)]
#![allow(unused_imports)]

use crate::manual::*;
use crate::platform::*;
use std::ffi::{c_char, c_int, c_uint, c_void};
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkInstance.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Instance(usize);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevice.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PhysicalDevice(usize);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDevice.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Device(usize);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueue.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Queue(usize);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphore.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Semaphore(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBuffer.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CommandBuffer(usize);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFence.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Fence(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemory.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DeviceMemory(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBuffer.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Buffer(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImage.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Image(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPool.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct QueryPool(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageView.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ImageView(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPool.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CommandPool(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPass.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct RenderPass(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebuffer.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Framebuffer(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkEvent.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Event(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferView.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BufferView(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderModule.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ShaderModule(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCache.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PipelineCache(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipeline.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Pipeline(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineLayout.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PipelineLayout(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayout.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DescriptorSetLayout(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSampler.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Sampler(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSet.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DescriptorSet(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPool.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DescriptorPool(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplate.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DescriptorUpdateTemplate(u64);
pub type DescriptorUpdateTemplateKHR = DescriptorUpdateTemplate;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversion.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SamplerYcbcrConversion(u64);
pub type SamplerYcbcrConversionKHR = SamplerYcbcrConversion;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlot.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PrivateDataSlot(u64);
pub type PrivateDataSlotEXT = PrivateDataSlot;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceKHR.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SurfaceKHR(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainKHR.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SwapchainKHR(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayKHR.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DisplayKHR(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayModeKHR.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DisplayModeKHR(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionKHR.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct VideoSessionKHR(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionParametersKHR.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct VideoSessionParametersKHR(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeferredOperationKHR.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DeferredOperationKHR(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureKHR.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AccelerationStructureKHR(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBinaryKHR.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PipelineBinaryKHR(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugReportCallbackEXT.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DebugReportCallbackEXT(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCuModuleNVX.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CuModuleNVX(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCuFunctionNVX.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CuFunctionNVX(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessengerEXT.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DebugUtilsMessengerEXT(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaSessionAMD.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct GpaSessionAMD(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorARM.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TensorARM(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkValidationCacheEXT.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ValidationCacheEXT(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureNV.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AccelerationStructureNV(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceConfigurationINTEL.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PerformanceConfigurationINTEL(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutNV.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct IndirectCommandsLayoutNV(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCudaModuleNV.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CudaModuleNV(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCudaFunctionNV.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CudaFunctionNV(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCollectionFUCHSIA.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BufferCollectionFUCHSIA(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapEXT.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MicromapEXT(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorViewARM.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TensorViewARM(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowSessionNV.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OpticalFlowSessionNV(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderEXT.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ShaderEXT(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionARM.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DataGraphPipelineSessionARM(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalComputeQueueNV.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ExternalComputeQueueNV(usize);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectExecutionSetEXT.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct IndirectExecutionSetEXT(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutEXT.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct IndirectCommandsLayoutEXT(u64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderInstrumentationARM.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ShaderInstrumentationARM(u64);
