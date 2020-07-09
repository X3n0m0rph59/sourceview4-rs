// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk_pixbuf;
use gio;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use gtk_source_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use GutterRenderer;

glib_wrapper! {
    pub struct GutterRendererPixbuf(Object<gtk_source_sys::GtkSourceGutterRendererPixbuf, gtk_source_sys::GtkSourceGutterRendererPixbufClass, GutterRendererPixbufClass>) @extends GutterRenderer;

    match fn {
        get_type => || gtk_source_sys::gtk_source_gutter_renderer_pixbuf_get_type(),
    }
}

impl GutterRendererPixbuf {
    pub fn new() -> GutterRendererPixbuf {
        assert_initialized_main_thread!();
        unsafe {
            GutterRenderer::from_glib_full(gtk_source_sys::gtk_source_gutter_renderer_pixbuf_new()).unsafe_cast()
        }
    }
}

impl Default for GutterRendererPixbuf {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_GUTTER_RENDERER_PIXBUF: Option<&GutterRendererPixbuf> = None;

pub trait GutterRendererPixbufExt: 'static {
    fn get_gicon(&self) -> Option<gio::Icon>;

    fn get_icon_name(&self) -> Option<GString>;

    fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn set_gicon<P: IsA<gio::Icon>>(&self, icon: Option<&P>);

    fn set_icon_name(&self, icon_name: Option<&str>);

    fn set_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>);

    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GutterRendererPixbuf>> GutterRendererPixbufExt for O {
    fn get_gicon(&self) -> Option<gio::Icon> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_gutter_renderer_pixbuf_get_gicon(self.as_ref().to_glib_none().0))
        }
    }

    fn get_icon_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_gutter_renderer_pixbuf_get_icon_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_gutter_renderer_pixbuf_get_pixbuf(self.as_ref().to_glib_none().0))
        }
    }

    fn set_gicon<P: IsA<gio::Icon>>(&self, icon: Option<&P>) {
        unsafe {
            gtk_source_sys::gtk_source_gutter_renderer_pixbuf_set_gicon(self.as_ref().to_glib_none().0, icon.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            gtk_source_sys::gtk_source_gutter_renderer_pixbuf_set_icon_name(self.as_ref().to_glib_none().0, icon_name.to_glib_none().0);
        }
    }

    fn set_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            gtk_source_sys::gtk_source_gutter_renderer_pixbuf_set_pixbuf(self.as_ref().to_glib_none().0, pixbuf.to_glib_none().0);
        }
    }

    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gicon_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceGutterRendererPixbuf, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<GutterRendererPixbuf>
        {
            let f: &F = &*(f as *const F);
            f(&GutterRendererPixbuf::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::gicon\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_gicon_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceGutterRendererPixbuf, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<GutterRendererPixbuf>
        {
            let f: &F = &*(f as *const F);
            f(&GutterRendererPixbuf::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_icon_name_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pixbuf_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceGutterRendererPixbuf, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<GutterRendererPixbuf>
        {
            let f: &F = &*(f as *const F);
            f(&GutterRendererPixbuf::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::pixbuf\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_pixbuf_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for GutterRendererPixbuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GutterRendererPixbuf")
    }
}
