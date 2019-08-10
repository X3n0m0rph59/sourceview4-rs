// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use CompletionProposal;
use gdk_pixbuf;
use gio;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gtk_source_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct CompletionItem(Object<gtk_source_sys::GtkSourceCompletionItem, gtk_source_sys::GtkSourceCompletionItemClass, CompletionItemClass>) @implements CompletionProposal;

    match fn {
        get_type => || gtk_source_sys::gtk_source_completion_item_get_type(),
    }
}

impl CompletionItem {
    pub fn new() -> CompletionItem {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_completion_item_new())
        }
    }
}

impl Default for CompletionItem {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_COMPLETION_ITEM: Option<&CompletionItem> = None;

pub trait CompletionItemExt: 'static {
    fn set_gicon<P: IsA<gio::Icon>>(&self, gicon: Option<&P>);

    fn set_icon(&self, icon: Option<&gdk_pixbuf::Pixbuf>);

    fn set_icon_name(&self, icon_name: Option<&str>);

    fn set_info(&self, info: Option<&str>);

    fn set_label(&self, label: Option<&str>);

    fn set_markup(&self, markup: Option<&str>);

    fn set_text(&self, text: Option<&str>);

    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_info_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CompletionItem>> CompletionItemExt for O {
    fn set_gicon<P: IsA<gio::Icon>>(&self, gicon: Option<&P>) {
        unsafe {
            gtk_source_sys::gtk_source_completion_item_set_gicon(self.as_ref().to_glib_none().0, gicon.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_icon(&self, icon: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            gtk_source_sys::gtk_source_completion_item_set_icon(self.as_ref().to_glib_none().0, icon.to_glib_none().0);
        }
    }

    fn set_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            gtk_source_sys::gtk_source_completion_item_set_icon_name(self.as_ref().to_glib_none().0, icon_name.to_glib_none().0);
        }
    }

    fn set_info(&self, info: Option<&str>) {
        unsafe {
            gtk_source_sys::gtk_source_completion_item_set_info(self.as_ref().to_glib_none().0, info.to_glib_none().0);
        }
    }

    fn set_label(&self, label: Option<&str>) {
        unsafe {
            gtk_source_sys::gtk_source_completion_item_set_label(self.as_ref().to_glib_none().0, label.to_glib_none().0);
        }
    }

    fn set_markup(&self, markup: Option<&str>) {
        unsafe {
            gtk_source_sys::gtk_source_completion_item_set_markup(self.as_ref().to_glib_none().0, markup.to_glib_none().0);
        }
    }

    fn set_text(&self, text: Option<&str>) {
        unsafe {
            gtk_source_sys::gtk_source_completion_item_set_text(self.as_ref().to_glib_none().0, text.to_glib_none().0);
        }
    }

    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gicon_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletionItem, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<CompletionItem>
        {
            let f: &F = &*(f as *const F);
            f(&CompletionItem::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::gicon\0".as_ptr() as *const _,
                Some(transmute(notify_gicon_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletionItem, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<CompletionItem>
        {
            let f: &F = &*(f as *const F);
            f(&CompletionItem::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon\0".as_ptr() as *const _,
                Some(transmute(notify_icon_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletionItem, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<CompletionItem>
        {
            let f: &F = &*(f as *const F);
            f(&CompletionItem::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon-name\0".as_ptr() as *const _,
                Some(transmute(notify_icon_name_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_info_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_info_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletionItem, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<CompletionItem>
        {
            let f: &F = &*(f as *const F);
            f(&CompletionItem::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::info\0".as_ptr() as *const _,
                Some(transmute(notify_info_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_label_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletionItem, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<CompletionItem>
        {
            let f: &F = &*(f as *const F);
            f(&CompletionItem::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::label\0".as_ptr() as *const _,
                Some(transmute(notify_label_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_markup_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletionItem, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<CompletionItem>
        {
            let f: &F = &*(f as *const F);
            f(&CompletionItem::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::markup\0".as_ptr() as *const _,
                Some(transmute(notify_markup_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletionItem, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<CompletionItem>
        {
            let f: &F = &*(f as *const F);
            f(&CompletionItem::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::text\0".as_ptr() as *const _,
                Some(transmute(notify_text_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for CompletionItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CompletionItem")
    }
}
