// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::GutterRenderer;
use crate::GutterRendererAlignmentMode;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkSourceGutterRendererPixbuf")]
    pub struct GutterRendererPixbuf(Object<ffi::GtkSourceGutterRendererPixbuf, ffi::GtkSourceGutterRendererPixbufClass>) @extends GutterRenderer;

    match fn {
        type_ => || ffi::gtk_source_gutter_renderer_pixbuf_get_type(),
    }
}

impl GutterRendererPixbuf {
    pub const NONE: Option<&'static GutterRendererPixbuf> = None;

    #[doc(alias = "gtk_source_gutter_renderer_pixbuf_new")]
    pub fn new() -> GutterRendererPixbuf {
        assert_initialized_main_thread!();
        unsafe {
            GutterRenderer::from_glib_full(ffi::gtk_source_gutter_renderer_pixbuf_new())
                .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`GutterRendererPixbuf`] objects.
    ///
    /// This method returns an instance of [`GutterRendererPixbufBuilder`](crate::builders::GutterRendererPixbufBuilder) which can be used to create [`GutterRendererPixbuf`] objects.
    pub fn builder() -> GutterRendererPixbufBuilder {
        GutterRendererPixbufBuilder::default()
    }
}

impl Default for GutterRendererPixbuf {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`GutterRendererPixbuf`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct GutterRendererPixbufBuilder {
    gicon: Option<gio::Icon>,
    icon_name: Option<String>,
    pixbuf: Option<gdk_pixbuf::Pixbuf>,
    alignment_mode: Option<GutterRendererAlignmentMode>,
    background_rgba: Option<gdk::RGBA>,
    background_set: Option<bool>,
    size: Option<i32>,
    visible: Option<bool>,
    xalign: Option<f32>,
    xpad: Option<i32>,
    yalign: Option<f32>,
    ypad: Option<i32>,
}

impl GutterRendererPixbufBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`GutterRendererPixbufBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`GutterRendererPixbuf`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> GutterRendererPixbuf {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref gicon) = self.gicon {
            properties.push(("gicon", gicon));
        }
        if let Some(ref icon_name) = self.icon_name {
            properties.push(("icon-name", icon_name));
        }
        if let Some(ref pixbuf) = self.pixbuf {
            properties.push(("pixbuf", pixbuf));
        }
        if let Some(ref alignment_mode) = self.alignment_mode {
            properties.push(("alignment-mode", alignment_mode));
        }
        if let Some(ref background_rgba) = self.background_rgba {
            properties.push(("background-rgba", background_rgba));
        }
        if let Some(ref background_set) = self.background_set {
            properties.push(("background-set", background_set));
        }
        if let Some(ref size) = self.size {
            properties.push(("size", size));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref xalign) = self.xalign {
            properties.push(("xalign", xalign));
        }
        if let Some(ref xpad) = self.xpad {
            properties.push(("xpad", xpad));
        }
        if let Some(ref yalign) = self.yalign {
            properties.push(("yalign", yalign));
        }
        if let Some(ref ypad) = self.ypad {
            properties.push(("ypad", ypad));
        }
        glib::Object::new::<GutterRendererPixbuf>(&properties)
    }

    pub fn gicon(mut self, gicon: &impl IsA<gio::Icon>) -> Self {
        self.gicon = Some(gicon.clone().upcast());
        self
    }

    pub fn icon_name(mut self, icon_name: &str) -> Self {
        self.icon_name = Some(icon_name.to_string());
        self
    }

    pub fn pixbuf(mut self, pixbuf: &gdk_pixbuf::Pixbuf) -> Self {
        self.pixbuf = Some(pixbuf.clone());
        self
    }

    pub fn alignment_mode(mut self, alignment_mode: GutterRendererAlignmentMode) -> Self {
        self.alignment_mode = Some(alignment_mode);
        self
    }

    pub fn background_rgba(mut self, background_rgba: &gdk::RGBA) -> Self {
        self.background_rgba = Some(background_rgba.clone());
        self
    }

    pub fn background_set(mut self, background_set: bool) -> Self {
        self.background_set = Some(background_set);
        self
    }

    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn xalign(mut self, xalign: f32) -> Self {
        self.xalign = Some(xalign);
        self
    }

    pub fn xpad(mut self, xpad: i32) -> Self {
        self.xpad = Some(xpad);
        self
    }

    pub fn yalign(mut self, yalign: f32) -> Self {
        self.yalign = Some(yalign);
        self
    }

    pub fn ypad(mut self, ypad: i32) -> Self {
        self.ypad = Some(ypad);
        self
    }
}

pub trait GutterRendererPixbufExt: 'static {
    #[doc(alias = "gtk_source_gutter_renderer_pixbuf_get_gicon")]
    #[doc(alias = "get_gicon")]
    fn gicon(&self) -> Option<gio::Icon>;

    #[doc(alias = "gtk_source_gutter_renderer_pixbuf_get_icon_name")]
    #[doc(alias = "get_icon_name")]
    fn icon_name(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_source_gutter_renderer_pixbuf_get_pixbuf")]
    #[doc(alias = "get_pixbuf")]
    fn pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf>;

    #[doc(alias = "gtk_source_gutter_renderer_pixbuf_set_gicon")]
    fn set_gicon(&self, icon: Option<&impl IsA<gio::Icon>>);

    #[doc(alias = "gtk_source_gutter_renderer_pixbuf_set_icon_name")]
    fn set_icon_name(&self, icon_name: Option<&str>);

    #[doc(alias = "gtk_source_gutter_renderer_pixbuf_set_pixbuf")]
    fn set_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>);

    #[doc(alias = "gicon")]
    fn connect_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "icon-name")]
    fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "pixbuf")]
    fn connect_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GutterRendererPixbuf>> GutterRendererPixbufExt for O {
    fn gicon(&self) -> Option<gio::Icon> {
        unsafe {
            from_glib_none(ffi::gtk_source_gutter_renderer_pixbuf_get_gicon(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn icon_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_source_gutter_renderer_pixbuf_get_icon_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(ffi::gtk_source_gutter_renderer_pixbuf_get_pixbuf(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_gicon(&self, icon: Option<&impl IsA<gio::Icon>>) {
        unsafe {
            ffi::gtk_source_gutter_renderer_pixbuf_set_gicon(
                self.as_ref().to_glib_none().0,
                icon.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            ffi::gtk_source_gutter_renderer_pixbuf_set_icon_name(
                self.as_ref().to_glib_none().0,
                icon_name.to_glib_none().0,
            );
        }
    }

    fn set_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            ffi::gtk_source_gutter_renderer_pixbuf_set_pixbuf(
                self.as_ref().to_glib_none().0,
                pixbuf.to_glib_none().0,
            );
        }
    }

    fn connect_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gicon_trampoline<
            P: IsA<GutterRendererPixbuf>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceGutterRendererPixbuf,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GutterRendererPixbuf::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::gicon\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_gicon_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<
            P: IsA<GutterRendererPixbuf>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceGutterRendererPixbuf,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GutterRendererPixbuf::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pixbuf_trampoline<
            P: IsA<GutterRendererPixbuf>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceGutterRendererPixbuf,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GutterRendererPixbuf::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pixbuf\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pixbuf_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for GutterRendererPixbuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GutterRendererPixbuf")
    }
}
