// WARNING: AUTO GENERATED MODULE
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::loader::*;
use crate::platform::*;
use std::ffi::{c_char, c_int, c_uint, c_void};

use crate::enums::*;
use crate::internal::*;

/// [`VkInstance`](https://docs.vulkan.org/refpages/latest/refpages/source/VkInstance.html)
///
/// # About
/// Dispatchable handle.
#[doc(alias = "VkInstance")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct InstanceHandle(usize);
impl InstanceHandle {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for InstanceHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InstanceHandle({:016X})", self.0)
    }
}

/// [`VkPhysicalDevice`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevice.html)
///
/// # About
/// Dispatchable handle.
/// Child of [`Instance`].
#[doc(alias = "VkPhysicalDevice")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PhysicalDeviceHandle(usize);
impl PhysicalDeviceHandle {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for PhysicalDeviceHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PhysicalDeviceHandle({:016X})", self.0)
    }
}

/// [`VkDevice`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDevice.html)
///
/// # About
/// Dispatchable handle.
/// Child of [`PhysicalDevice`].
#[doc(alias = "VkDevice")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DeviceHandle(usize);
impl DeviceHandle {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for DeviceHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeviceHandle({:016X})", self.0)
    }
}

/// [`VkQueue`](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueue.html)
///
/// # About
/// Dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkQueue")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct QueueHandle(usize);
impl QueueHandle {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for QueueHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QueueHandle({:016X})", self.0)
    }
}

/// [`VkSemaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphore.html)
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkSemaphore")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Semaphore(u64);
impl Semaphore {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for Semaphore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Semaphore({:016X})", self.0)
    }
}

/// [`VkCommandBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBuffer.html)
///
/// # About
/// Dispatchable handle.
/// Child of [`CommandPool`].
#[doc(alias = "VkCommandBuffer")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CommandBufferHandle(usize);
impl CommandBufferHandle {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for CommandBufferHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CommandBufferHandle({:016X})", self.0)
    }
}

/// [`VkFence`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFence.html)
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkFence")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Fence(u64);
impl Fence {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for Fence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fence({:016X})", self.0)
    }
}

/// [`VkDeviceMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemory.html)
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkDeviceMemory")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DeviceMemory(u64);
impl DeviceMemory {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for DeviceMemory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeviceMemory({:016X})", self.0)
    }
}

/// [`VkBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBuffer.html)
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkBuffer")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Buffer(u64);
impl Buffer {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Buffer({:016X})", self.0)
    }
}

/// [`VkImage`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImage.html)
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkImage")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Image(u64);
impl Image {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Image({:016X})", self.0)
    }
}

/// [`VkQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPool.html)
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkQueryPool")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct QueryPool(u64);
impl QueryPool {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for QueryPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QueryPool({:016X})", self.0)
    }
}

/// [`VkImageView`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageView.html)
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkImageView")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ImageView(u64);
impl ImageView {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for ImageView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ImageView({:016X})", self.0)
    }
}

/// [`VkCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPool.html)
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkCommandPool")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CommandPool(u64);
impl CommandPool {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for CommandPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CommandPool({:016X})", self.0)
    }
}

/// [`VkRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPass.html)
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkRenderPass")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct RenderPass(u64);
impl RenderPass {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for RenderPass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RenderPass({:016X})", self.0)
    }
}

/// [`VkFramebuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebuffer.html)
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkFramebuffer")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Framebuffer(u64);
impl Framebuffer {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for Framebuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Framebuffer({:016X})", self.0)
    }
}

/// [`VkEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/VkEvent.html)
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkEvent")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Event(u64);
impl Event {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Event({:016X})", self.0)
    }
}

/// [`VkBufferView`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferView.html)
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkBufferView")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct BufferView(u64);
impl BufferView {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for BufferView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BufferView({:016X})", self.0)
    }
}

/// [`VkShaderModule`](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderModule.html)
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkShaderModule")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ShaderModule(u64);
impl ShaderModule {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for ShaderModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ShaderModule({:016X})", self.0)
    }
}

/// [`VkPipelineCache`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCache.html)
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkPipelineCache")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PipelineCache(u64);
impl PipelineCache {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for PipelineCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PipelineCache({:016X})", self.0)
    }
}

/// [`VkPipeline`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipeline.html)
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkPipeline")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Pipeline(u64);
impl Pipeline {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for Pipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pipeline({:016X})", self.0)
    }
}

/// [`VkPipelineLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineLayout.html)
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkPipelineLayout")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PipelineLayout(u64);
impl PipelineLayout {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for PipelineLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PipelineLayout({:016X})", self.0)
    }
}

/// [`VkDescriptorSetLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayout.html)
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkDescriptorSetLayout")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DescriptorSetLayout(u64);
impl DescriptorSetLayout {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for DescriptorSetLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DescriptorSetLayout({:016X})", self.0)
    }
}

/// [`VkSampler`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSampler.html)
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkSampler")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Sampler(u64);
impl Sampler {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for Sampler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sampler({:016X})", self.0)
    }
}

/// [`VkDescriptorSet`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSet.html)
///
/// # About
/// Non-dispatchable handle.
/// Child of [`DescriptorPool`].
#[doc(alias = "VkDescriptorSet")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DescriptorSet(u64);
impl DescriptorSet {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for DescriptorSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DescriptorSet({:016X})", self.0)
    }
}

/// [`VkDescriptorPool`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPool.html)
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkDescriptorPool")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DescriptorPool(u64);
impl DescriptorPool {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for DescriptorPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DescriptorPool({:016X})", self.0)
    }
}

/// [`VkDescriptorUpdateTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplate.html)
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkDescriptorUpdateTemplate")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DescriptorUpdateTemplate(u64);
impl DescriptorUpdateTemplate {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for DescriptorUpdateTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DescriptorUpdateTemplate({:016X})", self.0)
    }
}
/// [`VkDescriptorUpdateTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplateKHR.html)
///
#[doc(alias = "VkDescriptorUpdateTemplateKHR")]
pub type DescriptorUpdateTemplateKHR = DescriptorUpdateTemplate;

/// [`VkSamplerYcbcrConversion`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversion.html)
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkSamplerYcbcrConversion")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct SamplerYcbcrConversion(u64);
impl SamplerYcbcrConversion {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for SamplerYcbcrConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SamplerYcbcrConversion({:016X})", self.0)
    }
}
/// [`VkSamplerYcbcrConversionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversionKHR.html)
///
#[doc(alias = "VkSamplerYcbcrConversionKHR")]
pub type SamplerYcbcrConversionKHR = SamplerYcbcrConversion;

/// [`VkPrivateDataSlot`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlot.html)
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkPrivateDataSlot")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PrivateDataSlot(u64);
impl PrivateDataSlot {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for PrivateDataSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PrivateDataSlot({:016X})", self.0)
    }
}
/// [`VkPrivateDataSlotEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlotEXT.html)
///
#[doc(alias = "VkPrivateDataSlotEXT")]
pub type PrivateDataSlotEXT = PrivateDataSlot;

/// [`VkSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_Surface`](Extension::KHR_Surface)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Instance`].
#[doc(alias = "VkSurfaceKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct SurfaceKHR(u64);
impl SurfaceKHR {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for SurfaceKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SurfaceKHR({:016X})", self.0)
    }
}

/// [`VkSwapchainKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_Swapchain`](Extension::KHR_Swapchain)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkSwapchainKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct SwapchainKHR(u64);
impl SwapchainKHR {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for SwapchainKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SwapchainKHR({:016X})", self.0)
    }
}

/// [`VkDisplayKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_Display`](Extension::KHR_Display)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`PhysicalDevice`].
#[doc(alias = "VkDisplayKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DisplayKHR(u64);
impl DisplayKHR {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for DisplayKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DisplayKHR({:016X})", self.0)
    }
}

/// [`VkDisplayModeKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayModeKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_Display`](Extension::KHR_Display)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`DisplayKHR`].
#[doc(alias = "VkDisplayModeKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DisplayModeKHR(u64);
impl DisplayModeKHR {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for DisplayModeKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DisplayModeKHR({:016X})", self.0)
    }
}

/// [`VkVideoSessionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkVideoSessionKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct VideoSessionKHR(u64);
impl VideoSessionKHR {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for VideoSessionKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VideoSessionKHR({:016X})", self.0)
    }
}

/// [`VkVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionParametersKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_VideoQueue`](Extension::KHR_VideoQueue)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkVideoSessionParametersKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct VideoSessionParametersKHR(u64);
impl VideoSessionParametersKHR {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for VideoSessionParametersKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VideoSessionParametersKHR({:016X})", self.0)
    }
}

/// [`VkDeferredOperationKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeferredOperationKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_DeferredHostOperations`](Extension::KHR_DeferredHostOperations)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkDeferredOperationKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DeferredOperationKHR(u64);
impl DeferredOperationKHR {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for DeferredOperationKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeferredOperationKHR({:016X})", self.0)
    }
}

/// [`VkAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_DeviceAddressCommands`](Extension::KHR_DeviceAddressCommands)
/// - Extension [`KHR_AccelerationStructure`](Extension::KHR_AccelerationStructure)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkAccelerationStructureKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct AccelerationStructureKHR(u64);
impl AccelerationStructureKHR {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for AccelerationStructureKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AccelerationStructureKHR({:016X})", self.0)
    }
}

/// [`VkPipelineBinaryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBinaryKHR.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`KHR_PipelineBinary`](Extension::KHR_PipelineBinary)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkPipelineBinaryKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PipelineBinaryKHR(u64);
impl PipelineBinaryKHR {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for PipelineBinaryKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PipelineBinaryKHR({:016X})", self.0)
    }
}

/// [`VkDebugReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugReportCallbackEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DebugReport`](Extension::EXT_DebugReport)
/// - Extension [`EXT_DebugUtils`](Extension::EXT_DebugUtils)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Instance`].
#[doc(alias = "VkDebugReportCallbackEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DebugReportCallbackEXT(u64);
impl DebugReportCallbackEXT {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for DebugReportCallbackEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DebugReportCallbackEXT({:016X})", self.0)
    }
}

/// [`VkCuModuleNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCuModuleNVX.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NVX_BinaryImport`](Extension::NVX_BinaryImport)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkCuModuleNVX")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CuModuleNVX(u64);
impl CuModuleNVX {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for CuModuleNVX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CuModuleNVX({:016X})", self.0)
    }
}

/// [`VkCuFunctionNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCuFunctionNVX.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NVX_BinaryImport`](Extension::NVX_BinaryImport)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkCuFunctionNVX")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CuFunctionNVX(u64);
impl CuFunctionNVX {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for CuFunctionNVX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CuFunctionNVX({:016X})", self.0)
    }
}

/// [`VkDebugUtilsMessengerEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessengerEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DebugUtils`](Extension::EXT_DebugUtils)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Instance`].
#[doc(alias = "VkDebugUtilsMessengerEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DebugUtilsMessengerEXT(u64);
impl DebugUtilsMessengerEXT {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for DebugUtilsMessengerEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DebugUtilsMessengerEXT({:016X})", self.0)
    }
}

/// [`VkGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaSessionAMD.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`AMD_GpaInterface`](Extension::AMD_GpaInterface)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkGpaSessionAMD")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct GpaSessionAMD(u64);
impl GpaSessionAMD {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for GpaSessionAMD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GpaSessionAMD({:016X})", self.0)
    }
}

/// [`VkTensorARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorARM.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DescriptorHeap`](Extension::EXT_DescriptorHeap)
/// - Extension [`ARM_Tensors`](Extension::ARM_Tensors)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkTensorARM")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct TensorARM(u64);
impl TensorARM {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for TensorARM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TensorARM({:016X})", self.0)
    }
}

/// [`VkValidationCacheEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkValidationCacheEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_ValidationCache`](Extension::EXT_ValidationCache)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkValidationCacheEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ValidationCacheEXT(u64);
impl ValidationCacheEXT {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for ValidationCacheEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValidationCacheEXT({:016X})", self.0)
    }
}

/// [`VkAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_RayTracing`](Extension::NV_RayTracing)
/// - Extension [`KHR_RayTracingPipeline`](Extension::KHR_RayTracingPipeline)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkAccelerationStructureNV")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct AccelerationStructureNV(u64);
impl AccelerationStructureNV {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for AccelerationStructureNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AccelerationStructureNV({:016X})", self.0)
    }
}

/// [`VkPerformanceConfigurationINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceConfigurationINTEL.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`INTEL_PerformanceQuery`](Extension::INTEL_PerformanceQuery)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkPerformanceConfigurationINTEL")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PerformanceConfigurationINTEL(u64);
impl PerformanceConfigurationINTEL {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for PerformanceConfigurationINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PerformanceConfigurationINTEL({:016X})", self.0)
    }
}

/// [`VkIndirectCommandsLayoutNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_DeviceGeneratedCommands`](Extension::NV_DeviceGeneratedCommands)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkIndirectCommandsLayoutNV")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct IndirectCommandsLayoutNV(u64);
impl IndirectCommandsLayoutNV {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for IndirectCommandsLayoutNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IndirectCommandsLayoutNV({:016X})", self.0)
    }
}

/// [`VkCudaModuleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCudaModuleNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_CudaKernelLaunch`](Extension::NV_CudaKernelLaunch)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkCudaModuleNV")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CudaModuleNV(u64);
impl CudaModuleNV {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for CudaModuleNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CudaModuleNV({:016X})", self.0)
    }
}

/// [`VkCudaFunctionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCudaFunctionNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_CudaKernelLaunch`](Extension::NV_CudaKernelLaunch)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkCudaFunctionNV")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CudaFunctionNV(u64);
impl CudaFunctionNV {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for CudaFunctionNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CudaFunctionNV({:016X})", self.0)
    }
}

/// [`VkBufferCollectionFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCollectionFUCHSIA.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`FUCHSIA_BufferCollection`](Extension::FUCHSIA_BufferCollection)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkBufferCollectionFUCHSIA")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct BufferCollectionFUCHSIA(u64);
impl BufferCollectionFUCHSIA {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for BufferCollectionFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BufferCollectionFUCHSIA({:016X})", self.0)
    }
}

/// [`VkMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_OpacityMicromap`](Extension::EXT_OpacityMicromap)
/// - Extension [`KHR_OpacityMicromap`](Extension::KHR_OpacityMicromap)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkMicromapEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct MicromapEXT(u64);
impl MicromapEXT {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for MicromapEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MicromapEXT({:016X})", self.0)
    }
}

/// [`VkTensorViewARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorViewARM.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`ARM_Tensors`](Extension::ARM_Tensors)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkTensorViewARM")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct TensorViewARM(u64);
impl TensorViewARM {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for TensorViewARM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TensorViewARM({:016X})", self.0)
    }
}

/// [`VkOpticalFlowSessionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowSessionNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_OpticalFlow`](Extension::NV_OpticalFlow)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkOpticalFlowSessionNV")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct OpticalFlowSessionNV(u64);
impl OpticalFlowSessionNV {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for OpticalFlowSessionNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OpticalFlowSessionNV({:016X})", self.0)
    }
}

/// [`VkShaderEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_ShaderObject`](Extension::EXT_ShaderObject)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkShaderEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ShaderEXT(u64);
impl ShaderEXT {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for ShaderEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ShaderEXT({:016X})", self.0)
    }
}

/// [`VkDataGraphPipelineSessionARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionARM.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`ARM_DataGraph`](Extension::ARM_DataGraph)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkDataGraphPipelineSessionARM")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DataGraphPipelineSessionARM(u64);
impl DataGraphPipelineSessionARM {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for DataGraphPipelineSessionARM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DataGraphPipelineSessionARM({:016X})", self.0)
    }
}

/// [`VkExternalComputeQueueNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalComputeQueueNV.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`NV_ExternalComputeQueue`](Extension::NV_ExternalComputeQueue)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkExternalComputeQueueNV")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ExternalComputeQueueNV(u64);
impl ExternalComputeQueueNV {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for ExternalComputeQueueNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExternalComputeQueueNV({:016X})", self.0)
    }
}

/// [`VkIndirectExecutionSetEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectExecutionSetEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DeviceGeneratedCommands`](Extension::EXT_DeviceGeneratedCommands)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkIndirectExecutionSetEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct IndirectExecutionSetEXT(u64);
impl IndirectExecutionSetEXT {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for IndirectExecutionSetEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IndirectExecutionSetEXT({:016X})", self.0)
    }
}

/// [`VkIndirectCommandsLayoutEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutEXT.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`EXT_DeviceGeneratedCommands`](Extension::EXT_DeviceGeneratedCommands)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkIndirectCommandsLayoutEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct IndirectCommandsLayoutEXT(u64);
impl IndirectCommandsLayoutEXT {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for IndirectCommandsLayoutEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IndirectCommandsLayoutEXT({:016X})", self.0)
    }
}

/// [`VkShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderInstrumentationARM.html)
///
/// # Requirements
/// This requires _at least_ one of the following:
/// - Extension [`ARM_ShaderInstrumentation`](Extension::ARM_ShaderInstrumentation)
///
/// Note this list might not be exhaustive. For more information check vulkan documentation.
///
/// # About
/// Non-dispatchable handle.
/// Child of [`Device`].
#[doc(alias = "VkShaderInstrumentationARM")]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ShaderInstrumentationARM(u64);
impl ShaderInstrumentationARM {
    /// Returns a null handle. This is an alias for [`Self::default`].
    #[inline(always)]
    pub fn null() -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for ShaderInstrumentationARM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ShaderInstrumentationARM({:016X})", self.0)
    }
}
