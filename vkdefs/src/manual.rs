/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBool32.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Bool32 {
    False = 0,
    True = 1,
}

impl From<Bool32> for bool {
    fn from(value: Bool32) -> Self {
        value == Bool32::True
    }
}

impl From<bool> for Bool32 {
    fn from(value: bool) -> Self {
        if value { Self::True } else { Self::False }
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
