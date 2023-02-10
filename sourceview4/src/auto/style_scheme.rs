// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Style;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkSourceStyleScheme")]
    pub struct StyleScheme(Object<ffi::GtkSourceStyleScheme, ffi::GtkSourceStyleSchemeClass>);

    match fn {
        type_ => || ffi::gtk_source_style_scheme_get_type(),
    }
}

impl StyleScheme {
    pub const NONE: Option<&'static StyleScheme> = None;

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`StyleScheme`] objects.
    ///
    /// This method returns an instance of [`StyleSchemeBuilder`](crate::builders::StyleSchemeBuilder) which can be used to create [`StyleScheme`] objects.
    pub fn builder() -> StyleSchemeBuilder {
        StyleSchemeBuilder::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`StyleScheme`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct StyleSchemeBuilder {
    builder: glib::object::ObjectBuilder<'static, StyleScheme>,
}

impl StyleSchemeBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn id(self, id: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("id", id.into()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`StyleScheme`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> StyleScheme {
        self.builder.build()
    }
}

pub trait StyleSchemeExt: 'static {
    #[doc(alias = "gtk_source_style_scheme_get_authors")]
    #[doc(alias = "get_authors")]
    fn authors(&self) -> Vec<glib::GString>;

    #[doc(alias = "gtk_source_style_scheme_get_description")]
    #[doc(alias = "get_description")]
    fn description(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_source_style_scheme_get_filename")]
    #[doc(alias = "get_filename")]
    fn filename(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_source_style_scheme_get_id")]
    #[doc(alias = "get_id")]
    fn id(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_source_style_scheme_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_source_style_scheme_get_style")]
    #[doc(alias = "get_style")]
    fn style(&self, style_id: &str) -> Option<Style>;

    #[doc(alias = "description")]
    fn connect_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "filename")]
    fn connect_filename_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "name")]
    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<StyleScheme>> StyleSchemeExt for O {
    fn authors(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_source_style_scheme_get_authors(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn description(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_get_description(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn filename(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_get_filename(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_get_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_get_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn style(&self, style_id: &str) -> Option<Style> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_get_style(
                self.as_ref().to_glib_none().0,
                style_id.to_glib_none().0,
            ))
        }
    }

    fn connect_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_description_trampoline<
            P: IsA<StyleScheme>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceStyleScheme,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(StyleScheme::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::description\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_description_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_filename_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_filename_trampoline<
            P: IsA<StyleScheme>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceStyleScheme,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(StyleScheme::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::filename\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_filename_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P: IsA<StyleScheme>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceStyleScheme,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(StyleScheme::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for StyleScheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("StyleScheme")
    }
}
