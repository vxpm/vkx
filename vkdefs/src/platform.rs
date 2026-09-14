//! Platform specific types.
#![allow(nonstandard_style)]
#![allow(clippy::upper_case_acronyms)]

use std::ffi::{c_ulong, c_void};

pub type IDirectFB = c_void;
pub type IDirectFBSurface = c_void;

// Xlib
pub type Display = c_void;
pub type Window = c_ulong;

// Xcb
pub type xcb_connection_t = c_void;
pub type xcb_window_t = u32;

// Windows
pub type DWORD = c_ulong;
pub type LPCWSTR = *const u16;

pub type HANDLE = isize;
pub type HINSTANCE = isize;
pub type HWND = isize;
pub type HMONITOR = isize;

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
