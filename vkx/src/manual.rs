//! Manually implemented items.

/// Marker trait indicating a vulkan structure extends another.
#[diagnostic::on_unimplemented(
    message = "Vulkan structure `{Self}` does not extend structure `{T}`",
    note = "Documentation of `{T}` contains a list of all structures extending it"
)]
pub trait Extends<T> {}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBool32.html>
#[doc(alias = "VkBool32")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum Bool32 {
    #[default]
    False = 0,
    True = 1,
}

impl From<Bool32> for bool {
    #[inline(always)]
    fn from(value: Bool32) -> Self {
        value == Bool32::True
    }
}

impl From<bool> for Bool32 {
    #[inline(always)]
    fn from(value: bool) -> Self {
        if value { Self::True } else { Self::False }
    }
}

impl std::cmp::PartialEq<bool> for Bool32 {
    #[inline(always)]
    fn eq(&self, other: &bool) -> bool {
        bool::from(*self) == *other
    }
}

impl std::cmp::PartialEq<Bool32> for bool {
    #[inline(always)]
    fn eq(&self, other: &Bool32) -> bool {
        bool::from(*other) == *self
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceAddress.html>
#[doc(alias = "VkDeviceAddress")]
pub type DeviceAddress = u64;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceAddress.html>
#[doc(alias = "VkDeviceSize")]
pub type DeviceSize = u64;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSampleMask.html>
#[doc(alias = "VkSampleMask")]
pub type SampleMask = u32;
