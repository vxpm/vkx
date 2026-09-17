// WARNING: AUTO GENERATED MODULE
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::loader::*;
use crate::platform::*;
use std::ffi::{c_char, c_int, c_uint, c_void};

use crate::internal::*;

/// [`VkInstance`](https://docs.vulkan.org/refpages/latest/refpages/source/VkInstance.html)
///
/// # Handle type
/// Dispatchable
#[doc(alias = "VkInstance")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct InstanceHandle(usize);

/// [`VkPhysicalDevice`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevice.html)
///
/// # Handle type
/// Dispatchable
#[doc(alias = "VkPhysicalDevice")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PhysicalDeviceHandle(usize);

/// [`VkDevice`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDevice.html)
///
/// # Handle type
/// Dispatchable
#[doc(alias = "VkDevice")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DeviceHandle(usize);

/// [`VkQueue`](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueue.html)
///
/// # Handle type
/// Dispatchable
#[doc(alias = "VkQueue")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct QueueHandle(usize);

/// [`VkSemaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphore.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkSemaphore")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Semaphore(u64);

/// [`VkCommandBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBuffer.html)
///
/// # Handle type
/// Dispatchable
#[doc(alias = "VkCommandBuffer")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CommandBufferHandle(usize);

/// [`VkFence`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFence.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkFence")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Fence(u64);

/// [`VkDeviceMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemory.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkDeviceMemory")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DeviceMemory(u64);

/// [`VkBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBuffer.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkBuffer")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Buffer(u64);

/// [`VkImage`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImage.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkImage")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Image(u64);

/// [`VkQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPool.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkQueryPool")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct QueryPool(u64);

/// [`VkImageView`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageView.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkImageView")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ImageView(u64);

/// [`VkCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPool.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkCommandPool")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CommandPool(u64);

/// [`VkRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPass.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkRenderPass")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct RenderPass(u64);

/// [`VkFramebuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebuffer.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkFramebuffer")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Framebuffer(u64);

/// [`VkEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/VkEvent.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkEvent")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Event(u64);

/// [`VkBufferView`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferView.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkBufferView")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct BufferView(u64);

/// [`VkShaderModule`](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderModule.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkShaderModule")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ShaderModule(u64);

/// [`VkPipelineCache`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCache.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkPipelineCache")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PipelineCache(u64);

/// [`VkPipeline`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipeline.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkPipeline")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Pipeline(u64);

/// [`VkPipelineLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineLayout.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkPipelineLayout")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PipelineLayout(u64);

/// [`VkDescriptorSetLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayout.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkDescriptorSetLayout")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DescriptorSetLayout(u64);

/// [`VkSampler`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSampler.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkSampler")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Sampler(u64);

/// [`VkDescriptorSet`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSet.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkDescriptorSet")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DescriptorSet(u64);

/// [`VkDescriptorPool`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPool.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkDescriptorPool")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DescriptorPool(u64);

/// [`VkDescriptorUpdateTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplate.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkDescriptorUpdateTemplate")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DescriptorUpdateTemplate(u64);
/// [`VkDescriptorUpdateTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplateKHR.html)
///
#[doc(alias = "VkDescriptorUpdateTemplateKHR")]
pub type DescriptorUpdateTemplateKHR = DescriptorUpdateTemplate;

/// [`VkSamplerYcbcrConversion`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversion.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkSamplerYcbcrConversion")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct SamplerYcbcrConversion(u64);
/// [`VkSamplerYcbcrConversionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversionKHR.html)
///
#[doc(alias = "VkSamplerYcbcrConversionKHR")]
pub type SamplerYcbcrConversionKHR = SamplerYcbcrConversion;

/// [`VkPrivateDataSlot`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlot.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkPrivateDataSlot")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PrivateDataSlot(u64);
/// [`VkPrivateDataSlotEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlotEXT.html)
///
#[doc(alias = "VkPrivateDataSlotEXT")]
pub type PrivateDataSlotEXT = PrivateDataSlot;

/// [`VkSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceKHR.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkSurfaceKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct SurfaceKHR(u64);

/// [`VkSwapchainKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainKHR.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkSwapchainKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct SwapchainKHR(u64);

/// [`VkDisplayKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayKHR.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkDisplayKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DisplayKHR(u64);

/// [`VkDisplayModeKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayModeKHR.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkDisplayModeKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DisplayModeKHR(u64);

/// [`VkVideoSessionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionKHR.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkVideoSessionKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct VideoSessionKHR(u64);

/// [`VkVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionParametersKHR.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkVideoSessionParametersKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct VideoSessionParametersKHR(u64);

/// [`VkDeferredOperationKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeferredOperationKHR.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkDeferredOperationKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DeferredOperationKHR(u64);

/// [`VkAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureKHR.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkAccelerationStructureKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct AccelerationStructureKHR(u64);

/// [`VkPipelineBinaryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBinaryKHR.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkPipelineBinaryKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PipelineBinaryKHR(u64);

/// [`VkDebugReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugReportCallbackEXT.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkDebugReportCallbackEXT")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DebugReportCallbackEXT(u64);

/// [`VkCuModuleNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCuModuleNVX.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkCuModuleNVX")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CuModuleNVX(u64);

/// [`VkCuFunctionNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCuFunctionNVX.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkCuFunctionNVX")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CuFunctionNVX(u64);

/// [`VkDebugUtilsMessengerEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessengerEXT.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkDebugUtilsMessengerEXT")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DebugUtilsMessengerEXT(u64);

/// [`VkGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaSessionAMD.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkGpaSessionAMD")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct GpaSessionAMD(u64);

/// [`VkTensorARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorARM.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkTensorARM")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct TensorARM(u64);

/// [`VkValidationCacheEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkValidationCacheEXT.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkValidationCacheEXT")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ValidationCacheEXT(u64);

/// [`VkAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureNV.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkAccelerationStructureNV")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct AccelerationStructureNV(u64);

/// [`VkPerformanceConfigurationINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceConfigurationINTEL.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkPerformanceConfigurationINTEL")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PerformanceConfigurationINTEL(u64);

/// [`VkIndirectCommandsLayoutNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutNV.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkIndirectCommandsLayoutNV")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct IndirectCommandsLayoutNV(u64);

/// [`VkCudaModuleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCudaModuleNV.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkCudaModuleNV")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CudaModuleNV(u64);

/// [`VkCudaFunctionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCudaFunctionNV.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkCudaFunctionNV")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CudaFunctionNV(u64);

/// [`VkBufferCollectionFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCollectionFUCHSIA.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkBufferCollectionFUCHSIA")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct BufferCollectionFUCHSIA(u64);

/// [`VkMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapEXT.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkMicromapEXT")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct MicromapEXT(u64);

/// [`VkTensorViewARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorViewARM.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkTensorViewARM")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct TensorViewARM(u64);

/// [`VkOpticalFlowSessionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowSessionNV.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkOpticalFlowSessionNV")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct OpticalFlowSessionNV(u64);

/// [`VkShaderEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderEXT.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkShaderEXT")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ShaderEXT(u64);

/// [`VkDataGraphPipelineSessionARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionARM.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkDataGraphPipelineSessionARM")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DataGraphPipelineSessionARM(u64);

/// [`VkExternalComputeQueueNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalComputeQueueNV.html)
///
/// # Handle type
/// Dispatchable
#[doc(alias = "VkExternalComputeQueueNV")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ExternalComputeQueueNV(u64);

/// [`VkIndirectExecutionSetEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectExecutionSetEXT.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkIndirectExecutionSetEXT")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct IndirectExecutionSetEXT(u64);

/// [`VkIndirectCommandsLayoutEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutEXT.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkIndirectCommandsLayoutEXT")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct IndirectCommandsLayoutEXT(u64);

/// [`VkShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderInstrumentationARM.html)
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkShaderInstrumentationARM")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ShaderInstrumentationARM(u64);
