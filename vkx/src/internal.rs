//! Some vulkan items implemented manually and utilities used in the crate.

use core::ffi::CStr;
use std::collections::HashSet;

/// Trait indicating a vulkan structure that can be extended.
///
/// # Safety
/// Types implementing this trait must be able to be treated as both a
/// [`BaseInStructure`](crate::BaseOutStructure) and a [`BaseOutStructure`](crate::BaseInStructure).
#[diagnostic::on_unimplemented(
    message = "Vulkan structure `{Self}` cannot be extended",
    note = "It does not have a `p_next` pointer"
)]
pub unsafe trait Extendable: Copy + Sized {
    const STRUCTURE_TYPE: crate::StructureType;

    #[inline(always)]
    fn with_next<T: Extends<Self>>(self, next: &mut T) -> Self {
        let mut new = self;
        new.push_next(next);
        new
    }

    #[inline(always)]
    fn push_next<T: Extends<Self>>(&mut self, next: &mut T) {
        let base_self = (self as *mut Self).cast::<crate::BaseOutStructure>();
        let base_next = (next as *mut T).cast::<crate::BaseOutStructure>();

        // SAFETY: trait contract
        unsafe {
            let self_old_next = std::ptr::replace(&raw mut (*base_self).p_next, base_next);
            let next_old_next = std::ptr::replace(&raw mut (*base_next).p_next, self_old_next);

            assert!(
                next_old_next.is_null(),
                "pushed a structure in a chain into another chain"
            );
        }
    }
}

/// Marker trait indicating a vulkan structure extends another.
#[diagnostic::on_unimplemented(
    message = "Vulkan structure `{Self}` does not extend structure `{T}`",
    note = "Documentation of `{T}` contains a list of all structures extending it"
)]
pub unsafe trait Extends<T>: Extendable {}

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

/// A version number.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Version(u32);

impl core::fmt::Debug for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Version")
            .field("variant", &self.variant())
            .field("major", &self.major())
            .field("minor", &self.minor())
            .field("patch", &self.patch())
            .finish()
    }
}

impl core::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.variant() == 0 {
            write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
        } else {
            write!(
                f,
                "{}.{}.{}.{}",
                self.variant(),
                self.major(),
                self.minor(),
                self.patch()
            )
        }
    }
}

impl Version {
    pub const V1_0: Self = Self::new(0, 1, 0, 0);
    pub const V1_1: Self = Self::new(0, 1, 1, 0);
    pub const V1_2: Self = Self::new(0, 1, 2, 0);
    pub const V1_3: Self = Self::new(0, 1, 3, 0);
    pub const V1_4: Self = Self::new(0, 1, 4, 0);

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAKE_API_VERSION.html>
    #[doc(alias = "VK_MAKE_API_VERSION")]
    #[inline(always)]
    pub const fn new(variant: u8, major: u8, minor: u16, patch: u16) -> Self {
        assert!((patch as u32) < (1 << 12));
        assert!((minor as u32) < (1 << 10));
        assert!((major as u32) < (1 << 7));
        assert!((variant as u32) < (1 << 3));
        let patch = patch as u32;
        let minor = (minor as u32) << 12;
        let major = (major as u32) << 22;
        let variant = (variant as u32) << 29;
        Self(patch | minor | major | variant)
    }

    /// Returns the packed version value.
    #[inline(always)]
    pub const fn get(self) -> u32 {
        self.0
    }

    /// Returns the variant version number.
    #[inline(always)]
    pub const fn variant(self) -> u8 {
        (self.0 >> 29) as u8
    }

    /// Returns the major version number.
    #[inline(always)]
    pub const fn major(self) -> u8 {
        (self.0 >> 22) as u8 & 0x7F
    }

    /// Returns the minor version number.
    #[inline(always)]
    pub const fn minor(self) -> u16 {
        ((self.0 >> 12) as u16) & 0x3FF
    }

    /// Returns the patch version number.
    #[inline(always)]
    pub const fn patch(self) -> u16 {
        (self.0 as u16) & 0xFFF
    }
}

impl From<u32> for Version {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Version> for u32 {
    fn from(value: Version) -> Self {
        value.get()
    }
}

impl crate::ResultCode {
    /// Like [`Self::split`], except it panics if the success code is anything other than
    /// [`SuccessCode::SUCCESS`](crate::SuccessCode::SUCCESS).
    #[track_caller]
    #[inline(always)]
    pub fn success(self) -> Result<(), crate::ErrorCode> {
        match self.split() {
            Ok(crate::SuccessCode::SUCCESS) => Ok(()),
            Ok(_) => panic!("called `ResultCode::success` on an OK but non SUCCESS result code"),
            Err(e) => Err(e),
        }
    }
}

impl std::fmt::Display for crate::ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for crate::ErrorCode {}

impl crate::Extension {
    pub fn to_ptrs(extensions: impl IntoIterator<Item = Self>) -> Vec<*const std::ffi::c_char> {
        extensions
            .into_iter()
            .map(|e| e.name().as_ptr())
            .collect::<Vec<_>>()
    }

    pub fn from_ext_properties(
        extensions: impl IntoIterator<Item = crate::ExtensionProperties>,
    ) -> HashSet<Self> {
        let mut result = HashSet::default();
        for ext in extensions {
            let name =
                CStr::from_bytes_until_nul(zerocopy::transmute_ref!(&ext.extension_name)).unwrap();

            let Some(variant) = Self::from_name(name) else {
                panic!("unknown extension '{name:?}'");
            };

            result.insert(variant);
        }

        result
    }
}

#[macro_export]
macro_rules! auto_count {
    (|$count:ident, $placeholder:ident| $recv:ident.$method:ident($($arg:expr),* $(,)?)) => {
        {
            let mut $count = 0u32;
            let mut $placeholder = std::ptr::null_mut();
            $recv.$method($($arg),*);

            let mut result = vec![Default::default(); $count as usize];
            let mut $placeholder = result.as_mut_ptr();
            $recv.$method($($arg),*);

            result
        }
    };
}
