//! Some vulkan items implemented manually and utilities used in the crate.

use core::ffi::CStr;
use std::collections::HashSet;

/// Trait for vulkan structures that can be extended.
///
/// # Safety
/// Types implementing this trait must be able to be treated as both a
/// [`BaseInStructure`](crate::BaseOutStructure) and a [`BaseOutStructure`](crate::BaseInStructure).
#[diagnostic::on_unimplemented(
    message = "Type `{Self}` is not a Vulkan structure that can be extended",
    note = "If `{Self}` is a Vulkan structure, then it does not have a `p_next` pointer"
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
///
/// # Safety
/// Types implementing this trait _must_ be able to extend `T` according to the vulkan spec.
#[diagnostic::on_unimplemented(
    message = "Type `{Self}` does not extend structure `{T}`",
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

impl crate::SuccessCode {
    /// Returns whether this [`SuccessCode`](crate::SuccessCode) is
    /// [`SUCCESS`](crate::SuccessCode::SUCCESS).
    #[inline(always)]
    pub fn is_success(self) -> bool {
        self == crate::SuccessCode::SUCCESS
    }
}

impl std::fmt::Display for crate::ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for crate::ErrorCode {}

impl From<Result<crate::SuccessCode, crate::ErrorCode>> for crate::ResultCode {
    fn from(value: Result<crate::SuccessCode, crate::ErrorCode>) -> Self {
        let raw = match value {
            Ok(x) => x as u32,
            Err(x) => x as u32,
        };

        unsafe { std::mem::transmute::<u32, Self>(raw) }
    }
}

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
            let name = chars_as_cstr(&ext.extension_name).unwrap();
            let Some(variant) = Self::from_name(name) else {
                panic!("unknown extension '{name:?}'");
            };

            result.insert(variant);
        }

        result
    }
}

#[inline(always)]
/// Helper function that creates a [`CStr`] from a slice of [`c_char`](std::ffi::c_char)s.
pub fn chars_as_cstr(slice: &[std::ffi::c_char]) -> Option<&CStr> {
    CStr::from_bytes_until_nul(zerocopy::transmute_ref!(slice)).ok()
}

#[macro_export]
macro_rules! auto_count {
    (|$count:ident, $placeholder:ident| $expr:expr) => {{
        let mut $count = 0u32;
        let mut $placeholder = None;
        _ = $expr;

        let mut out = vec![Default::default(); $count as usize];
        let mut $placeholder = Some(out.as_mut_ptr());
        let result = $expr;

        (out, result)
    }};
}

/// A flag-like type.
pub trait Flag: std::fmt::Debug + Copy + Sized + 'static {
    type Inner: Copy
        + PartialEq
        + Eq
        + Default
        + std::ops::Not<Output = Self::Inner>
        + std::ops::BitAnd<Output = Self::Inner>
        + std::ops::BitAndAssign
        + std::ops::BitOr<Output = Self::Inner>
        + std::ops::BitOrAssign
        + std::ops::BitXor<Output = Self::Inner>
        + std::ops::BitXorAssign;

    /// All the variants of this flag.
    const VARIANTS: &[Self];

    /// Turns this flag value into it's inner representation.
    fn to_inner(self) -> Self::Inner;
}

/// A set of [`Flag`]s.
///
/// `F` defines two things:
/// - The inner type of this set
/// - The known flag values
///
/// The inner value of the set _might_ contain flags that aren't defined in `F` (i.e. unknowns),
/// and no operations are going to implicitly discard those. If you wish to keep only known flags
/// of `F`, [`Self::truncated`] can do that.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FlagSet<F: Flag>(pub F::Inner);

impl<F: Flag> std::fmt::Debug for FlagSet<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_set();

        let mut count = 0;
        for variant in F::VARIANTS {
            let inner = variant.to_inner();
            if self.0 & inner == inner {
                count += 1;
                f.entry(&variant);
            }
        }

        if count == F::VARIANTS.len() {
            f.finish()
        } else {
            f.finish_non_exhaustive()
        }
    }
}

impl<F: Flag> FlagSet<F> {
    /// Returns an empty set.
    pub fn empty() -> Self {
        Self(Default::default())
    }

    /// Returns a full set. This returns a set with every possible flag set, not just known ones. If
    /// you need a set with known ones only, use [`Self::truncated`] afterwards.
    pub fn full() -> Self {
        !Self::empty()
    }

    /// Returns whether this set is a superset of `other`, i.e. it contains every flag in `other`
    /// and possibly more.
    pub fn is_superset(self, other: impl Into<Self>) -> bool {
        let other = other.into();
        self.0 & other.0 == other.0
    }

    /// Returns whether this set is a subset of `other`, i.e. `other` contains every flag in it
    /// and possibly more.
    pub fn is_subset(self, other: impl Into<Self>) -> bool {
        other.into().is_superset(self)
    }

    /// Returns whether this set contains `other`. This is an alias of [`Self::is_superset`].
    pub fn contains(self, other: impl Into<Self>) -> bool {
        self.is_superset(other)
    }

    /// Truncates this set, keeping only known flags of `F`.
    #[must_use]
    pub fn truncated(self) -> Self {
        let mut result = self.0;
        for variant in F::VARIANTS {
            let inner = variant.to_inner();
            result &= inner;
        }

        Self(result)
    }
}

impl<F: Flag> Default for FlagSet<F> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<F: Flag> From<F> for FlagSet<F> {
    #[inline(always)]
    fn from(value: F) -> Self {
        Self(value.to_inner())
    }
}

impl<F: Flag> std::ops::Not for FlagSet<F> {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl<F: Flag, T: Into<FlagSet<F>>> std::ops::BitAnd<T> for FlagSet<F> {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: T) -> Self::Output {
        Self(self.0 & rhs.into().0)
    }
}

impl<F: Flag, T: Into<FlagSet<F>>> std::ops::BitAndAssign<T> for FlagSet<F> {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: T) {
        self.0 &= rhs.into().0;
    }
}

impl<F: Flag, T: Into<FlagSet<F>>> std::ops::BitOr<T> for FlagSet<F> {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: T) -> Self::Output {
        Self(self.0 | rhs.into().0)
    }
}

impl<F: Flag, T: Into<FlagSet<F>>> std::ops::BitOrAssign<T> for FlagSet<F> {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: T) {
        self.0 |= rhs.into().0;
    }
}

impl<F: Flag, T: Into<FlagSet<F>>> std::ops::BitXor<T> for FlagSet<F> {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: T) -> Self::Output {
        Self(self.0 ^ rhs.into().0)
    }
}

impl<F: Flag, T: Into<FlagSet<F>>> std::ops::BitXorAssign<T> for FlagSet<F> {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: T) {
        self.0 ^= rhs.into().0;
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! __vkx_internal_flags {
    (
        $(#[$ty_meta:meta])*
        $vis:vis enum $name:ident: $inner:ty {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident = $value:expr
            ),* $(,)?
        }
    ) => {
        $(#[$ty_meta])*
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        #[repr($inner)]
        $vis enum $name {
            $(
                $(#[$variant_meta])*
                $variant = $value
            ),*
        }

        const _: () = {
            $(
                let value: $inner = $value;
                assert!(
                    value == 0 || value.count_ones() == 1,
                    "flag variants must only have one bit set"
                );
            )*
        };

        impl $crate::Flag for $name {
            type Inner = $inner;

            const VARIANTS: &[Self] = {
                &[$($name::$variant),*]
            };

            fn to_inner(self) -> Self::Inner {
                self as $inner
            }
        }

        impl ::std::ops::Not for $name {
            type Output = $crate::FlagSet<$name>;

            #[inline(always)]
            fn not(self) -> Self::Output {
                !$crate::FlagSet::from(self)
            }
        }

        impl<T: Into<$crate::FlagSet<$name>>> ::std::ops::BitAnd<T> for $name {
            type Output = $crate::FlagSet<$name>;

            #[inline(always)]
            fn bitand(self, rhs: T) -> Self::Output {
                $crate::FlagSet::from(self) & rhs
            }
        }

        impl<T: Into<$crate::FlagSet<$name>>> ::std::ops::BitOr<T> for $name {
            type Output = $crate::FlagSet<$name>;

            #[inline(always)]
            fn bitor(self, rhs: T) -> Self::Output {
                $crate::FlagSet::from(self) | rhs
            }
        }

        impl<T: Into<$crate::FlagSet<$name>>> std::ops::BitXor<T> for $name {
            type Output = $crate::FlagSet<$name>;

            #[inline(always)]
            fn bitxor(self, rhs: T) -> Self::Output {
                $crate::FlagSet::from(self) ^ rhs
            }
        }
    };
}
