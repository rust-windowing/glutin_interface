use std::os::raw;

use winit_types::dpi::PhysicalSize;
use winit_types::error::Error;

/// `None`, if the type is wrapped in an option, means use default display.
///
/// Non exhaustive as new platforms may be added in the future.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[non_exhaustive]
pub enum RawDisplay {
    Wayland {
        /// A `*mut wl_proxy` of a `wl_display`.
        wl_display: Option<*mut raw::c_void>,
    },

    Xlib {
        display: Option<*mut raw::c_void>,
    },

    /// If using Xlib/XCB, please provide Xlib handles instead.
    XCB {
        xcb_connection_t: Option<*mut raw::c_void>,
    },

    Gbm {
        gbm_device: Option<*mut raw::c_void>,
    },

    /// `EGL_DEFAULT_DISPLAY` works for windows on EGL, but I beleive so does
    /// a `HWND`'s `HDC`.
    ///
    /// A `HWND`'s `HDC` is needed for WGL, which is slightly unfortunate. For
    /// that reason we expect a dummy HWND, like winit's `thread_msg_target`,
    /// that we can use for stuff like searching for the appropriate pixel
    /// config.
    ///
    /// While glutin may use a `HWND` of `None` internally, we expect that the
    /// library implementing this interface to never provide `None`. The task of
    /// deciding when to use `None` should be left up to glutin.
    Windows {
        hwnd: Option<*mut raw::c_void>,
    },

    /// `EGL_DEFAULT_DISPLAY` is mandatory for Android.
    Android,

    Device {
        egl_device_ext: *mut raw::c_void,
    },

    /// Don't specify any display type. Useful on windows.
    /// `None` means `EGL_DEFAULT_DISPLAY`.
    Other(Option<*mut raw::c_void>),

    /// Nothing required for iOS.
    IOS,

    /// Nothing required for iOS.
    MacOS,

    /// `EGL_DEFAULT_DISPLAY` is mandatory for EGL_MESA_platform_surfaceless.
    ///
    /// Not to be confused with Mesa's surfaceless context extension.
    Surfaceless,
}

pub trait NativeDisplay {
    fn display(&self) -> RawDisplay;
}

pub trait NativeWindowSource {
    type Window: NativeWindow;

    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    fn build_wayland(self) -> Result<Self::Window, Error>;

    // FIXME: other platforms
}

pub trait NativePixmapSource {
    type Pixmap: NativePixmap;

    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    fn build_wayland(self) -> Result<Self::Pixmap, Error>;

    // FIXME: other platforms
}

/// Wayland, Gbm, Android, Surfaceless, and Device do not support pixmaps.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[non_exhaustive]
pub enum RawPixmap {
    Xlib {
        pixmap: *mut raw::c_void,
    },

    XCB {
        xcb_pixmap_t: u32,
    },

    Windows {
        hbitmap: *mut raw::c_void,
    },

    /// Don't specify the type.
    Other(*mut raw::c_void),
    // FIXME: Macos/ios?
}

/// Device and surfaceless do not support windows.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[non_exhaustive]
pub enum RawWindow {
    Windows {
        hwnd: *mut raw::c_void,
    },

    Android {
        a_native_window: *mut raw::c_void,
    },

    Gbm {
        gbm_surface: *mut raw::c_void,
    },

    Xlib {
        window: raw::c_ulong,
    },

    XCB {
        xcb_window_t: u32,
    },

    Wayland {
        /// A `*mut wl_proxy` of a `wl_surface`.
        wl_surface: *mut raw::c_void,
    },

    MacOS {
        ns_view: *mut raw::c_void,
        ns_window: *mut raw::c_void,
    },

    IOS {
        ns_view: *mut raw::c_void,
        ns_view_controller: *mut raw::c_void,
        ns_window: *mut raw::c_void,
    },

    /// Don't specify the type.
    Other(*mut raw::c_void),
}

pub trait NativeWindow {
    fn raw_window(&self) -> RawWindow;

    fn size(&self) -> PhysicalSize;
    fn hidpi_factor(&self) -> f64;
}

pub trait NativePixmap {
    fn raw_pixmap(&self) -> RawPixmap;
}
