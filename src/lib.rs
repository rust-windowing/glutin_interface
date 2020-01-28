//! These are the things glutin wants.

use std::os::raw;

use winit_types::dpi::PhysicalSize;
use winit_types::error::Error;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc(hidden)]
pub struct Seal;

/// Non exhaustive as new platforms may be added in the future.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[non_exhaustive]
pub enum RawDisplay {
    Wayland {
        /// A `*mut wl_proxy` of a `wl_display`.
        wl_display: Option<*mut raw::c_void>,

        #[doc(hidden)]
        #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
        _non_exhaustive_do_not_use: Seal,
    },

    Xlib {
        /// While with EGL is is possible to create a `EGLDisplay` without a
        /// `Display` (as it will create its own automatically via an
        /// implementation defined manner), glutin depends on having a `Display`
        /// for some of its functionality. Furthermore, a display is mandatory
        /// for use with GLX.
        display: *mut raw::c_void,
        /// Okay, so this one is a wierd one.
        ///
        /// > On the X11 platform, an EGLDisplay refers to a specific X11 screen
        /// > rather than an X11 display connection. This is the case because
        /// > separate X11 screens, even when belonging to the same X11 display
        /// > connection, may reside on different GPUs and/or be driven by
        /// > different drivers. Therefore, different X11 screens may have
        /// > different EGL capabilities.
        /// >
        /// > [...]  The value of attribute EGL_PLATFORM_X11_SCREEN_KHR
        /// > specifies the X11 screen to use. If the attribute is omitted from
        /// > <attrib_list>, then the display connection's default screen is
        /// > used.  Otherwise, the attribute's value must be a valid screen on
        /// > the display connection.
        /// >
        /// > -- EGL_KHR_platform_x11
        ///
        /// If you specify the screen, glutin promices to use that screen
        /// everywhere. If you don't, it's up to glutin to pick a screen of its
        /// choice, currently that's the display's default screen.
        ///
        /// Whatever screen is chosen, that will be the one glutin passes back
        /// to implementors of this interface when asking to build a Window
        /// and/or Pixmap.
        ///
        /// All implementors of this interface probably want to pass `None`.
        /// Previously the ability to control the screen was _NOT_ exposed, and
        /// I don't think anyone's ever requested it, but with the 0.23 refactor
        /// we might as well add it.
        screen: Option<raw::c_int>,

        #[doc(hidden)]
        #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
        _non_exhaustive_do_not_use: Seal,
    },

    Gbm {
        gbm_device: Option<*mut raw::c_void>,

        #[doc(hidden)]
        #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
        _non_exhaustive_do_not_use: Seal,
    },

    /// `EGL_DEFAULT_DISPLAY` works for windows on EGL, but I beleive so does
    /// a `HWND`'s `HDC`.
    ///
    /// A `HWND`'s `HDC` is needed for WGL, which is slightly unfortunate.
    ///
    /// While glutin may use a `HWND` of `Some(_)` internally, we expect that the
    /// library implementing this interface to always provide `None`. The task of
    /// deciding when to use to use a dummy should be left up to glutin.
    Windows {
        hwnd: Option<*mut raw::c_void>,

        #[doc(hidden)]
        #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
        _non_exhaustive_do_not_use: Seal,
    },

    /// `EGL_DEFAULT_DISPLAY` is mandatory for Android.
    Android {
        #[doc(hidden)]
        #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
        _non_exhaustive_do_not_use: Seal,
    },

    /// EGL_EXT_platform_device, a wierd NVIDIA-led egl platform.
    EglExtDevice {
        egl_device_ext: *mut raw::c_void,

        #[doc(hidden)]
        #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
        _non_exhaustive_do_not_use: Seal,
    },

    /// Nothing required for iOS.
    IOS {
        #[doc(hidden)]
        #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
        _non_exhaustive_do_not_use: Seal,
    },

    /// Nothing required for MacOS.
    MacOS {
        #[doc(hidden)]
        #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
        _non_exhaustive_do_not_use: Seal,
    },

    /// `EGL_DEFAULT_DISPLAY` is mandatory for EGL_MESA_platform_surfaceless.
    ///
    /// Not to be confused with Mesa's surfaceless context extension.
    EglMesaSurfaceless {
        #[doc(hidden)]
        #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
        _non_exhaustive_do_not_use: Seal,
    },
}

pub trait NativeDisplay {
    fn raw_display(&self) -> RawDisplay;
}

#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
pub struct WaylandWindowParts {
    #[doc(hidden)]
    #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
    pub _non_exhaustive_do_not_use: Seal,
}

#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
pub struct X11WindowParts {
    pub x_visual_info: *const raw::c_void,
    pub screen: raw::c_int,

    #[doc(hidden)]
    #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
    pub _non_exhaustive_do_not_use: Seal,
}

#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
pub struct GbmWindowParts {
    pub color_format: u32,

    #[doc(hidden)]
    #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
    pub _non_exhaustive_do_not_use: Seal,
}

pub trait NativeWindowSource {
    type Window: NativeWindow;
    type WindowBuilder;

    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    fn build_wayland(
        &self,
        wb: Self::WindowBuilder,
        wwp: WaylandWindowParts,
    ) -> Result<Self::Window, Error>;

    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    fn build_x11(
        &self,
        wb: Self::WindowBuilder,
        xwp: X11WindowParts,
    ) -> Result<Self::Window, Error>;

    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    fn build_gbm(
        &self,
        wb: Self::WindowBuilder,
        gbmwp: GbmWindowParts,
    ) -> Result<Self::Window, Error>;
}

pub trait NativePixmapSource {
    type Pixmap: NativePixmap;
    type PixmapBuilder;

    // FIXME: other platforms
}

/// Wayland, Gbm, Android, EGLMesaSurfaceless, and EGLExtDevice do not support
/// pixmaps.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[non_exhaustive]
pub enum RawPixmap {
    Xlib {
        pixmap: *mut raw::c_void,

        #[doc(hidden)]
        #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
        _non_exhaustive_do_not_use: Seal,
    },

    Windows {
        hbitmap: *mut raw::c_void,

        #[doc(hidden)]
        #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
        _non_exhaustive_do_not_use: Seal,
    },
}

/// EGLExtDevice and EGLMesaSurfaceless do not support windows.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[non_exhaustive]
pub enum RawWindow {
    Windows {
        hwnd: *mut raw::c_void,

        #[doc(hidden)]
        #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
        _non_exhaustive_do_not_use: Seal,
    },

    Android {
        a_native_window: *mut raw::c_void,

        #[doc(hidden)]
        #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
        _non_exhaustive_do_not_use: Seal,
    },

    Gbm {
        gbm_surface: *mut raw::c_void,

        #[doc(hidden)]
        #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
        _non_exhaustive_do_not_use: Seal,
    },

    Xlib {
        window: raw::c_ulong,

        #[doc(hidden)]
        #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
        _non_exhaustive_do_not_use: Seal,
    },

    Wayland {
        /// A `*mut wl_proxy` of a `wl_surface`.
        wl_surface: *mut raw::c_void,

        #[doc(hidden)]
        #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
        _non_exhaustive_do_not_use: Seal,
    },

    MacOS {
        ns_view: *mut raw::c_void,
        ns_window: *mut raw::c_void,

        #[doc(hidden)]
        #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
        _non_exhaustive_do_not_use: Seal,
    },

    IOS {
        ns_view: *mut raw::c_void,
        ns_view_controller: *mut raw::c_void,
        ns_window: *mut raw::c_void,

        #[doc(hidden)]
        #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
        _non_exhaustive_do_not_use: Seal,
    },
}

pub trait NativeWindow {
    fn raw_window(&self) -> RawWindow;

    fn size(&self) -> PhysicalSize<u32>;
    fn scale_factor(&self) -> f64;
}

pub trait NativePixmap {
    fn raw_pixmap(&self) -> RawPixmap;
}
