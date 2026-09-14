/// Boolean like 32 bit integer wrapper.
///
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBool32.html>
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Bool32(u32);

impl Bool32 {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}

impl From<Bool32> for bool {
    fn from(value: Bool32) -> Self {
        value.0 != 0
    }
}

impl From<bool> for Bool32 {
    fn from(value: bool) -> Self {
        Self(value as u32)
    }
}

impl std::cmp::PartialEq<Self> for Bool32 {
    fn eq(&self, other: &Self) -> bool {
        bool::from(*self) == bool::from(*other)
    }
}

impl std::cmp::PartialEq<bool> for Bool32 {
    fn eq(&self, other: &bool) -> bool {
        bool::from(*self) == *other
    }
}

impl std::cmp::PartialEq<Bool32> for bool {
    fn eq(&self, other: &Bool32) -> bool {
        bool::from(*other) == *self
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceAddress.html>
pub type DeviceAddress = u64;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceAddress.html>
pub type DeviceSize = u64;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSampleMask.html>
pub type SampleMask = u32;
