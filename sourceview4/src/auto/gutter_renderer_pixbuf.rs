// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{GutterRenderer, GutterRendererAlignmentMode};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

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
        GutterRendererPixbufBuilder::new()
    }
}

impl Default for GutterRendererPixbuf {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`GutterRendererPixbuf`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct GutterRendererPixbufBuilder {
    builder: glib::object::ObjectBuilder<'static, GutterRendererPixbuf>,
}

impl GutterRendererPixbufBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn gicon(self, gicon: &impl IsA<gio::Icon>) -> Self {
        Self {
            builder: self.builder.property("gicon", gicon.clone().upcast()),
        }
    }

    pub fn icon_name(self, icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("icon-name", icon_name.into()),
        }
    }

    pub fn pixbuf(self, pixbuf: &gdk_pixbuf::Pixbuf) -> Self {
        Self {
            builder: self.builder.property("pixbuf", pixbuf.clone()),
        }
    }

    pub fn alignment_mode(self, alignment_mode: GutterRendererAlignmentMode) -> Self {
        Self {
            builder: self.builder.property("alignment-mode", alignment_mode),
        }
    }

    pub fn background_rgba(self, background_rgba: &gdk::RGBA) -> Self {
        Self {
            builder: self.builder.property("background-rgba", background_rgba),
        }
    }

    pub fn background_set(self, background_set: bool) -> Self {
        Self {
            builder: self.builder.property("background-set", background_set),
        }
    }

    pub fn size(self, size: i32) -> Self {
        Self {
            builder: self.builder.property("size", size),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn xalign(self, xalign: f32) -> Self {
        Self {
            builder: self.builder.property("xalign", xalign),
        }
    }

    pub fn xpad(self, xpad: i32) -> Self {
        Self {
            builder: self.builder.property("xpad", xpad),
        }
    }

    pub fn yalign(self, yalign: f32) -> Self {
        Self {
            builder: self.builder.property("yalign", yalign),
        }
    }

    pub fn ypad(self, ypad: i32) -> Self {
        Self {
            builder: self.builder.property("ypad", ypad),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`GutterRendererPixbuf`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> GutterRendererPixbuf {
        self.builder.build()
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
