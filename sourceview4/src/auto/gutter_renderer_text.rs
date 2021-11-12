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
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkSourceGutterRendererText")]
    pub struct GutterRendererText(Object<ffi::GtkSourceGutterRendererText, ffi::GtkSourceGutterRendererTextClass>) @extends GutterRenderer;

    match fn {
        type_ => || ffi::gtk_source_gutter_renderer_text_get_type(),
    }
}

impl GutterRendererText {
    #[doc(alias = "gtk_source_gutter_renderer_text_new")]
    pub fn new() -> GutterRendererText {
        assert_initialized_main_thread!();
        unsafe {
            GutterRenderer::from_glib_full(ffi::gtk_source_gutter_renderer_text_new()).unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`GutterRendererText`] objects.
    ///
    /// This method returns an instance of [`GutterRendererTextBuilder`] which can be used to create [`GutterRendererText`] objects.
    pub fn builder() -> GutterRendererTextBuilder {
        GutterRendererTextBuilder::default()
    }
}

impl Default for GutterRendererText {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`GutterRendererText`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct GutterRendererTextBuilder {
    markup: Option<String>,
    text: Option<String>,
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

impl GutterRendererTextBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`GutterRendererTextBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`GutterRendererText`].
    pub fn build(self) -> GutterRendererText {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref markup) = self.markup {
            properties.push(("markup", markup));
        }
        if let Some(ref text) = self.text {
            properties.push(("text", text));
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
        glib::Object::new::<GutterRendererText>(&properties)
            .expect("Failed to create an instance of GutterRendererText")
    }

    pub fn markup(mut self, markup: &str) -> Self {
        self.markup = Some(markup.to_string());
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
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

impl GutterRendererText {
    pub const NONE: Option<&'static GutterRendererText> = None;
}

pub trait GutterRendererTextExt: 'static {
    #[doc(alias = "gtk_source_gutter_renderer_text_measure")]
    fn measure(&self, text: &str) -> (i32, i32);

    #[doc(alias = "gtk_source_gutter_renderer_text_measure_markup")]
    fn measure_markup(&self, markup: &str) -> (i32, i32);

    #[doc(alias = "gtk_source_gutter_renderer_text_set_markup")]
    fn set_markup(&self, markup: &str);

    #[doc(alias = "gtk_source_gutter_renderer_text_set_text")]
    fn set_text(&self, text: &str);

    fn markup(&self) -> Option<glib::GString>;

    fn text(&self) -> Option<glib::GString>;

    #[doc(alias = "markup")]
    fn connect_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "text")]
    fn connect_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GutterRendererText>> GutterRendererTextExt for O {
    fn measure(&self, text: &str) -> (i32, i32) {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            ffi::gtk_source_gutter_renderer_text_measure(
                self.as_ref().to_glib_none().0,
                text.to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            let width = width.assume_init();
            let height = height.assume_init();
            (width, height)
        }
    }

    fn measure_markup(&self, markup: &str) -> (i32, i32) {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            ffi::gtk_source_gutter_renderer_text_measure_markup(
                self.as_ref().to_glib_none().0,
                markup.to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            let width = width.assume_init();
            let height = height.assume_init();
            (width, height)
        }
    }

    fn set_markup(&self, markup: &str) {
        let length = markup.len() as i32;
        unsafe {
            ffi::gtk_source_gutter_renderer_text_set_markup(
                self.as_ref().to_glib_none().0,
                markup.to_glib_none().0,
                length,
            );
        }
    }

    fn set_text(&self, text: &str) {
        let length = text.len() as i32;
        unsafe {
            ffi::gtk_source_gutter_renderer_text_set_text(
                self.as_ref().to_glib_none().0,
                text.to_glib_none().0,
                length,
            );
        }
    }

    fn markup(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "markup")
    }

    fn text(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "text")
    }

    fn connect_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_markup_trampoline<
            P: IsA<GutterRendererText>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceGutterRendererText,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GutterRendererText::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::markup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_markup_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_trampoline<
            P: IsA<GutterRendererText>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceGutterRendererText,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GutterRendererText::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for GutterRendererText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GutterRendererText")
    }
}
