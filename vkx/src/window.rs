//! Support for creating surfaces on a window through [`raw_window_handle`].

use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};

use crate::Extension;

/// Window error.
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Handle(#[from] raw_window_handle::HandleError),
    #[error("vulkan error {0:?}")]
    Vulkan(#[from] crate::ErrorCode),
    #[error("platform not supported")]
    NotSupported,
    #[error("handle was in an invalid state")]
    InvalidHandle,
}

/// Creates a [`surface handle`](crate::SurfaceKHR) for the given `window` and `display` using
/// `instance`.
///
/// # Safety
/// The required extensions for this `display` must have been enabled in `instance`. To get a list
/// of them, use [`get_required_extensions`].
pub unsafe fn create_surface(
    instance: &crate::Instance,
    display: impl HasDisplayHandle,
    window: impl HasWindowHandle,
) -> Result<crate::SurfaceKHR, Error> {
    let display = display.display_handle()?;
    let window = window.window_handle()?;

    let mut surface = Default::default();
    match (display.as_raw(), window.as_raw()) {
        (RawDisplayHandle::Wayland(display), RawWindowHandle::Wayland(window)) => {
            let info = crate::WaylandSurfaceCreateInfoKHR {
                display: display.display.as_ptr(),
                surface: window.surface.as_ptr(),
                ..Default::default()
            };

            unsafe {
                instance
                    .create_wayland_surface_khr(&info, std::ptr::null(), &mut surface)
                    .success()?
            }
        }

        (RawDisplayHandle::Xlib(display), RawWindowHandle::Xlib(window)) => {
            let info = crate::XlibSurfaceCreateInfoKHR {
                dpy: display.display.ok_or(Error::InvalidHandle)?.as_ptr(),
                window: window.window,
                ..Default::default()
            };

            unsafe {
                instance
                    .create_xlib_surface_khr(&info, std::ptr::null(), &mut surface)
                    .success()?
            }
        }

        (RawDisplayHandle::Xcb(display), RawWindowHandle::Xcb(window)) => {
            let info = crate::XcbSurfaceCreateInfoKHR {
                connection: display.connection.ok_or(Error::InvalidHandle)?.as_ptr(),
                window: window.window.get(),
                ..Default::default()
            };

            unsafe {
                instance
                    .create_xcb_surface_khr(&info, std::ptr::null(), &mut surface)
                    .success()?
            }
        }

        (RawDisplayHandle::Android(_), RawWindowHandle::AndroidNdk(window)) => {
            let info = crate::AndroidSurfaceCreateInfoKHR {
                window: window.a_native_window.as_ptr(),
                ..Default::default()
            };

            unsafe {
                instance
                    .create_android_surface_khr(&info, std::ptr::null(), &mut surface)
                    .success()?
            }
        }

        (RawDisplayHandle::Windows(_), RawWindowHandle::Win32(window)) => {
            let info = crate::Win32SurfaceCreateInfoKHR {
                hwnd: window.hwnd.get(),
                hinstance: window.hinstance.ok_or(Error::InvalidHandle)?.get(),
                ..Default::default()
            };

            unsafe {
                instance
                    .create_win_32_surface_khr(&info, std::ptr::null(), &mut surface)
                    .success()?
            }
        }

        #[cfg(target_os = "macos")]
        (RawDisplayHandle::AppKit(_), RawWindowHandle::AppKit(window)) => {
            let layer = unsafe { raw_window_metal::Layer::from_ns_view(window.ns_view) };
            let create_info = crate::MetalSurfaceCreateInfoEXT {
                p_layer: layer.into_raw().as_ptr().cast(),
                ..Default::default()
            };

            unsafe {
                instance
                    .create_metal_surface_ext(&info, std::ptr::null(), &mut surface)
                    .success()?
            }
        }

        #[cfg(target_os = "ios")]
        (RawDisplayHandle::UiKit(_), RawWindowHandle::UiKit(window)) => {
            let layer = unsafe { raw_window_metal::Layer::from_ui_view(window.ui_view) };
            let create_info = crate::MetalSurfaceCreateInfoEXT {
                p_layer: layer.into_raw().as_ptr().cast(),
                ..Default::default()
            };

            unsafe {
                instance
                    .create_metal_surface_ext(&info, std::ptr::null(), &mut surface)
                    .success()?
            }
        }

        (RawDisplayHandle::Ohos(_), RawWindowHandle::OhosNdk(window)) => {
            let info = crate::SurfaceCreateInfoOHOS {
                window: window.native_window.as_ptr().cast(),
                ..Default::default()
            };

            unsafe {
                instance
                    .create_surface_ohos(&info, std::ptr::null(), &mut surface)
                    .success()?
            }
        }

        _ => return Err(Error::NotSupported),
    }

    Ok(surface)
}

/// Get the required instance extensions for creating a surface using `display`.
pub fn get_required_extensions(
    display: impl HasDisplayHandle,
) -> Result<&'static [Extension], Error> {
    let handle = display.display_handle()?;
    let extensions = match handle.as_raw() {
        RawDisplayHandle::Windows(_) => &[Extension::KHR_Surface, Extension::KHR_Win32Surface],
        RawDisplayHandle::Wayland(_) => &[Extension::KHR_Surface, Extension::KHR_WaylandSurface],
        RawDisplayHandle::Xlib(_) => &[Extension::KHR_Surface, Extension::KHR_XlibSurface],
        RawDisplayHandle::Xcb(_) => &[Extension::KHR_Surface, Extension::KHR_XcbSurface],
        RawDisplayHandle::Android(_) => &[Extension::KHR_Surface, Extension::KHR_AndroidSurface],
        RawDisplayHandle::Ohos(_) => &[Extension::KHR_Surface, Extension::OHOS_Surface],
        RawDisplayHandle::AppKit(_) | RawDisplayHandle::UiKit(_) => {
            &[Extension::KHR_Surface, Extension::EXT_MetalSurface]
        }
        _ => return Err(Error::NotSupported),
    };

    Ok(extensions)
}
