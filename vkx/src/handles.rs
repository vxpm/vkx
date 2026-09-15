// WARNING: AUTO GENERATED MODULE
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::manual::*;
use crate::platform::*;
use std::ffi::{c_char, c_int, c_uint, c_void};
/// `VkInstance`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkInstance.html>
///
/// # Handle type
/// Dispatchable
#[doc(alias = "VkInstance")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Instance(usize);

/// `VkPhysicalDevice`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevice.html>
///
/// # Handle type
/// Dispatchable
#[doc(alias = "VkPhysicalDevice")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PhysicalDevice(usize);

/// `VkDevice`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDevice.html>
///
/// # Handle type
/// Dispatchable
#[doc(alias = "VkDevice")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Device(usize);

/// `VkQueue`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueue.html>
///
/// # Handle type
/// Dispatchable
#[doc(alias = "VkQueue")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Queue(usize);

/// `VkSemaphore`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphore.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkSemaphore")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Semaphore(u64);

/// `VkCommandBuffer`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBuffer.html>
///
/// # Handle type
/// Dispatchable
#[doc(alias = "VkCommandBuffer")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CommandBuffer(usize);

/// `VkFence`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFence.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkFence")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Fence(u64);

/// `VkDeviceMemory`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemory.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkDeviceMemory")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DeviceMemory(u64);

/// `VkBuffer`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBuffer.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkBuffer")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Buffer(u64);

/// `VkImage`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImage.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkImage")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Image(u64);

/// `VkQueryPool`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPool.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkQueryPool")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct QueryPool(u64);

/// `VkImageView`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageView.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkImageView")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ImageView(u64);

/// `VkCommandPool`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPool.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkCommandPool")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CommandPool(u64);

/// `VkRenderPass`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPass.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkRenderPass")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct RenderPass(u64);

/// `VkFramebuffer`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebuffer.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkFramebuffer")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Framebuffer(u64);

/// `VkEvent`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkEvent.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkEvent")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Event(u64);

/// `VkBufferView`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferView.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkBufferView")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct BufferView(u64);

/// `VkShaderModule`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderModule.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkShaderModule")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ShaderModule(u64);

/// `VkPipelineCache`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCache.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkPipelineCache")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PipelineCache(u64);

/// `VkPipeline`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipeline.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkPipeline")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Pipeline(u64);

/// `VkPipelineLayout`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineLayout.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkPipelineLayout")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PipelineLayout(u64);

/// `VkDescriptorSetLayout`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayout.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkDescriptorSetLayout")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DescriptorSetLayout(u64);

/// `VkSampler`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSampler.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkSampler")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Sampler(u64);

/// `VkDescriptorSet`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSet.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkDescriptorSet")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DescriptorSet(u64);

/// `VkDescriptorPool`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPool.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkDescriptorPool")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DescriptorPool(u64);

/// `VkDescriptorUpdateTemplate`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplate.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkDescriptorUpdateTemplate")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DescriptorUpdateTemplate(u64);
pub type DescriptorUpdateTemplateKHR = DescriptorUpdateTemplate;

/// `VkSamplerYcbcrConversion`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversion.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkSamplerYcbcrConversion")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct SamplerYcbcrConversion(u64);
pub type SamplerYcbcrConversionKHR = SamplerYcbcrConversion;

/// `VkPrivateDataSlot`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlot.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkPrivateDataSlot")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PrivateDataSlot(u64);
pub type PrivateDataSlotEXT = PrivateDataSlot;

/// `VkSurfaceKHR`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceKHR.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkSurfaceKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct SurfaceKHR(u64);

/// `VkSwapchainKHR`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainKHR.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkSwapchainKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct SwapchainKHR(u64);

/// `VkDisplayKHR`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayKHR.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkDisplayKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DisplayKHR(u64);

/// `VkDisplayModeKHR`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayModeKHR.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkDisplayModeKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DisplayModeKHR(u64);

/// `VkVideoSessionKHR`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionKHR.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkVideoSessionKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct VideoSessionKHR(u64);

/// `VkVideoSessionParametersKHR`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionParametersKHR.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkVideoSessionParametersKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct VideoSessionParametersKHR(u64);

/// `VkDeferredOperationKHR`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeferredOperationKHR.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkDeferredOperationKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DeferredOperationKHR(u64);

/// `VkAccelerationStructureKHR`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureKHR.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkAccelerationStructureKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct AccelerationStructureKHR(u64);

/// `VkPipelineBinaryKHR`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBinaryKHR.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkPipelineBinaryKHR")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PipelineBinaryKHR(u64);

/// `VkDebugReportCallbackEXT`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugReportCallbackEXT.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkDebugReportCallbackEXT")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DebugReportCallbackEXT(u64);

/// `VkCuModuleNVX`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCuModuleNVX.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkCuModuleNVX")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CuModuleNVX(u64);

/// `VkCuFunctionNVX`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCuFunctionNVX.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkCuFunctionNVX")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CuFunctionNVX(u64);

/// `VkDebugUtilsMessengerEXT`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessengerEXT.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkDebugUtilsMessengerEXT")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DebugUtilsMessengerEXT(u64);

/// `VkGpaSessionAMD`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaSessionAMD.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkGpaSessionAMD")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct GpaSessionAMD(u64);

/// `VkTensorARM`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorARM.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkTensorARM")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct TensorARM(u64);

/// `VkValidationCacheEXT`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkValidationCacheEXT.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkValidationCacheEXT")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ValidationCacheEXT(u64);

/// `VkAccelerationStructureNV`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureNV.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkAccelerationStructureNV")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct AccelerationStructureNV(u64);

/// `VkPerformanceConfigurationINTEL`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceConfigurationINTEL.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkPerformanceConfigurationINTEL")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PerformanceConfigurationINTEL(u64);

/// `VkIndirectCommandsLayoutNV`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutNV.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkIndirectCommandsLayoutNV")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct IndirectCommandsLayoutNV(u64);

/// `VkCudaModuleNV`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCudaModuleNV.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkCudaModuleNV")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CudaModuleNV(u64);

/// `VkCudaFunctionNV`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCudaFunctionNV.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkCudaFunctionNV")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CudaFunctionNV(u64);

/// `VkBufferCollectionFUCHSIA`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCollectionFUCHSIA.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkBufferCollectionFUCHSIA")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct BufferCollectionFUCHSIA(u64);

/// `VkMicromapEXT`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapEXT.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkMicromapEXT")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct MicromapEXT(u64);

/// `VkTensorViewARM`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorViewARM.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkTensorViewARM")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct TensorViewARM(u64);

/// `VkOpticalFlowSessionNV`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowSessionNV.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkOpticalFlowSessionNV")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct OpticalFlowSessionNV(u64);

/// `VkShaderEXT`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderEXT.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkShaderEXT")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ShaderEXT(u64);

/// `VkDataGraphPipelineSessionARM`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionARM.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkDataGraphPipelineSessionARM")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DataGraphPipelineSessionARM(u64);

/// `VkExternalComputeQueueNV`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalComputeQueueNV.html>
///
/// # Handle type
/// Dispatchable
#[doc(alias = "VkExternalComputeQueueNV")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ExternalComputeQueueNV(usize);

/// `VkIndirectExecutionSetEXT`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectExecutionSetEXT.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkIndirectExecutionSetEXT")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct IndirectExecutionSetEXT(u64);

/// `VkIndirectCommandsLayoutEXT`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutEXT.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkIndirectCommandsLayoutEXT")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct IndirectCommandsLayoutEXT(u64);

/// `VkShaderInstrumentationARM`
///
/// # Vulkan documentation
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderInstrumentationARM.html>
///
/// # Handle type
/// Non-dispatchable
#[doc(alias = "VkShaderInstrumentationARM")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ShaderInstrumentationARM(u64);
