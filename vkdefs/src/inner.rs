/// Boolean like 32 bit integer wrapper.
///
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBool32.html>
#[derive(Debug, Clone, Copy)]
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

/// A device buffer address.
///
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceAddress.html>
pub type DeviceAddress = u64;

/// Device memory size and offsets.
///
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceAddress.html>
pub type DeviceSize = u64;

/// Platform specific types.
mod platform {
    #![allow(nonstandard_style)]
    use std::ffi::*;

    pub type IDirectFB = c_void;
    pub type IDirectFBSurface = c_void;

    // Xlib
    pub type Display = c_void;
    pub type Window = c_ulong;

    // Xcb
    pub type xcb_connection_t = c_void;
    pub type xcb_window_t = u32;
    pub type xcb_visualid_t = u32;

    // Windows
    pub type DWORD = c_ulong;
    pub type LPCWSTR = *const u16;

    pub type HANDLE = isize;
    pub type HINSTANCE = HANDLE;
    pub type HWND = HANDLE;
    pub type HMONITOR = HANDLE;

    pub type SECURITY_ATTRIBUTES = c_void;

    // Wayland
    pub type wl_display = c_void;
    pub type wl_surface = c_void;

    // Fuchsia
    pub type zx_handle_t = u32;

    // QNX
    pub type _screen_buffer = c_void;
    pub type _screen_context = c_void;
    pub type _screen_window = c_void;

    // GPG
    pub type GgpStreamDescriptor = u32;
    pub type GgpFrameToken = u64;

    // UBM
    pub type ubm_device = c_void;
    pub type ubm_surface = c_void;

    // Open Harmony
    pub type OHNativeWindow = c_void;
    pub type OH_NativeBuffer = c_void;

    // Metal
    pub type CAMetalLayer = c_void;

    pub type MTLBuffer_id = *mut c_void;
    pub type MTLCommandQueue_id = *mut c_void;
    pub type MTLDevice_id = *mut c_void;
    pub type MTLSharedEvent_id = *mut c_void;
    pub type MTLTexture_id = *mut c_void;

    pub type IOSurfaceRef = *mut c_void;

    // Android
    pub type AHardwareBuffer = c_void;
    pub type ANativeWindow = c_void;
}

pub use platform::*;
