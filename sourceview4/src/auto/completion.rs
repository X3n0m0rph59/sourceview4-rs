// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_source_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use CompletionContext;
use CompletionInfo;
use CompletionProvider;
use View;

glib_wrapper! {
    pub struct Completion(Object<gtk_source_sys::GtkSourceCompletion, gtk_source_sys::GtkSourceCompletionClass, CompletionClass>);

    match fn {
        get_type => || gtk_source_sys::gtk_source_completion_get_type(),
    }
}

pub const NONE_COMPLETION: Option<&Completion> = None;

pub trait CompletionExt: 'static {
    fn add_provider<P: IsA<CompletionProvider>>(&self, provider: &P) -> Result<(), glib::Error>;

    fn block_interactive(&self);

    fn get_info_window(&self) -> Option<CompletionInfo>;

    fn get_providers(&self) -> Vec<CompletionProvider>;

    fn get_view(&self) -> Option<View>;

    fn hide(&self);

    fn remove_provider<P: IsA<CompletionProvider>>(&self, provider: &P) -> Result<(), glib::Error>;

    fn start<P: IsA<CompletionContext>>(&self, providers: &[CompletionProvider], context: &P) -> bool;

    fn unblock_interactive(&self);

    fn get_property_accelerators(&self) -> u32;

    fn set_property_accelerators(&self, accelerators: u32);

    fn get_property_auto_complete_delay(&self) -> u32;

    fn set_property_auto_complete_delay(&self, auto_complete_delay: u32);

    fn get_property_proposal_page_size(&self) -> u32;

    fn set_property_proposal_page_size(&self, proposal_page_size: u32);

    fn get_property_provider_page_size(&self) -> u32;

    fn set_property_provider_page_size(&self, provider_page_size: u32);

    fn get_property_remember_info_visibility(&self) -> bool;

    fn set_property_remember_info_visibility(&self, remember_info_visibility: bool);

    fn get_property_select_on_show(&self) -> bool;

    fn set_property_select_on_show(&self, select_on_show: bool);

    fn get_property_show_headers(&self) -> bool;

    fn set_property_show_headers(&self, show_headers: bool);

    fn get_property_show_icons(&self) -> bool;

    fn set_property_show_icons(&self, show_icons: bool);

    fn connect_activate_proposal<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate_proposal(&self);

    fn connect_hide<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_hide(&self);

    //fn connect_move_cursor<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    //fn connect_move_page<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_populate_context<F: Fn(&Self, &CompletionContext) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_populate_context(&self, context: &CompletionContext);

    fn connect_show<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_show(&self);

    fn connect_property_accelerators_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_auto_complete_delay_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_proposal_page_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_provider_page_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_remember_info_visibility_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_select_on_show_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_headers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_icons_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Completion>> CompletionExt for O {
    fn add_provider<P: IsA<CompletionProvider>>(&self, provider: &P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_source_sys::gtk_source_completion_add_provider(self.as_ref().to_glib_none().0, provider.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn block_interactive(&self) {
        unsafe {
            gtk_source_sys::gtk_source_completion_block_interactive(self.as_ref().to_glib_none().0);
        }
    }

    fn get_info_window(&self) -> Option<CompletionInfo> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_completion_get_info_window(self.as_ref().to_glib_none().0))
        }
    }

    fn get_providers(&self) -> Vec<CompletionProvider> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(gtk_source_sys::gtk_source_completion_get_providers(self.as_ref().to_glib_none().0))
        }
    }

    fn get_view(&self) -> Option<View> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_completion_get_view(self.as_ref().to_glib_none().0))
        }
    }

    fn hide(&self) {
        unsafe {
            gtk_source_sys::gtk_source_completion_hide(self.as_ref().to_glib_none().0);
        }
    }

    fn remove_provider<P: IsA<CompletionProvider>>(&self, provider: &P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_source_sys::gtk_source_completion_remove_provider(self.as_ref().to_glib_none().0, provider.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn start<P: IsA<CompletionContext>>(&self, providers: &[CompletionProvider], context: &P) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_completion_start(self.as_ref().to_glib_none().0, providers.to_glib_none().0, context.as_ref().to_glib_none().0))
        }
    }

    fn unblock_interactive(&self) {
        unsafe {
            gtk_source_sys::gtk_source_completion_unblock_interactive(self.as_ref().to_glib_none().0);
        }
    }

    fn get_property_accelerators(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"accelerators\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `accelerators` getter").unwrap()
        }
    }

    fn set_property_accelerators(&self, accelerators: u32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"accelerators\0".as_ptr() as *const _, Value::from(&accelerators).to_glib_none().0);
        }
    }

    fn get_property_auto_complete_delay(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"auto-complete-delay\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `auto-complete-delay` getter").unwrap()
        }
    }

    fn set_property_auto_complete_delay(&self, auto_complete_delay: u32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"auto-complete-delay\0".as_ptr() as *const _, Value::from(&auto_complete_delay).to_glib_none().0);
        }
    }

    fn get_property_proposal_page_size(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"proposal-page-size\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `proposal-page-size` getter").unwrap()
        }
    }

    fn set_property_proposal_page_size(&self, proposal_page_size: u32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"proposal-page-size\0".as_ptr() as *const _, Value::from(&proposal_page_size).to_glib_none().0);
        }
    }

    fn get_property_provider_page_size(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"provider-page-size\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `provider-page-size` getter").unwrap()
        }
    }

    fn set_property_provider_page_size(&self, provider_page_size: u32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"provider-page-size\0".as_ptr() as *const _, Value::from(&provider_page_size).to_glib_none().0);
        }
    }

    fn get_property_remember_info_visibility(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"remember-info-visibility\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `remember-info-visibility` getter").unwrap()
        }
    }

    fn set_property_remember_info_visibility(&self, remember_info_visibility: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"remember-info-visibility\0".as_ptr() as *const _, Value::from(&remember_info_visibility).to_glib_none().0);
        }
    }

    fn get_property_select_on_show(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"select-on-show\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `select-on-show` getter").unwrap()
        }
    }

    fn set_property_select_on_show(&self, select_on_show: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"select-on-show\0".as_ptr() as *const _, Value::from(&select_on_show).to_glib_none().0);
        }
    }

    fn get_property_show_headers(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"show-headers\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `show-headers` getter").unwrap()
        }
    }

    fn set_property_show_headers(&self, show_headers: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"show-headers\0".as_ptr() as *const _, Value::from(&show_headers).to_glib_none().0);
        }
    }

    fn get_property_show_icons(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"show-icons\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `show-icons` getter").unwrap()
        }
    }

    fn set_property_show_icons(&self, show_icons: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"show-icons\0".as_ptr() as *const _, Value::from(&show_icons).to_glib_none().0);
        }
    }

    fn connect_activate_proposal<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_proposal_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletion, f: glib_sys::gpointer)
            where P: IsA<Completion>
        {
            let f: &F = &*(f as *const F);
            f(&Completion::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"activate-proposal\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(activate_proposal_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn emit_activate_proposal(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject).emit("activate-proposal", &[]).unwrap() };
    }

    fn connect_hide<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn hide_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletion, f: glib_sys::gpointer)
            where P: IsA<Completion>
        {
            let f: &F = &*(f as *const F);
            f(&Completion::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"hide\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(hide_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn emit_hide(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject).emit("hide", &[]).unwrap() };
    }

    //fn connect_move_cursor<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored step: Gtk.ScrollStep
    //}

    //fn connect_move_page<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored step: Gtk.ScrollStep
    //}

    fn connect_populate_context<F: Fn(&Self, &CompletionContext) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn populate_context_trampoline<P, F: Fn(&P, &CompletionContext) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletion, context: *mut gtk_source_sys::GtkSourceCompletionContext, f: glib_sys::gpointer)
            where P: IsA<Completion>
        {
            let f: &F = &*(f as *const F);
            f(&Completion::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(context))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"populate-context\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(populate_context_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn emit_populate_context(&self, context: &CompletionContext) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject).emit("populate-context", &[&context]).unwrap() };
    }

    fn connect_show<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn show_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletion, f: glib_sys::gpointer)
            where P: IsA<Completion>
        {
            let f: &F = &*(f as *const F);
            f(&Completion::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"show\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(show_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn emit_show(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject).emit("show", &[]).unwrap() };
    }

    fn connect_property_accelerators_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accelerators_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletion, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Completion>
        {
            let f: &F = &*(f as *const F);
            f(&Completion::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::accelerators\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_accelerators_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_auto_complete_delay_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_auto_complete_delay_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletion, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Completion>
        {
            let f: &F = &*(f as *const F);
            f(&Completion::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::auto-complete-delay\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_auto_complete_delay_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_proposal_page_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_proposal_page_size_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletion, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Completion>
        {
            let f: &F = &*(f as *const F);
            f(&Completion::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::proposal-page-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_proposal_page_size_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_provider_page_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_provider_page_size_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletion, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Completion>
        {
            let f: &F = &*(f as *const F);
            f(&Completion::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::provider-page-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_provider_page_size_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_remember_info_visibility_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_remember_info_visibility_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletion, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Completion>
        {
            let f: &F = &*(f as *const F);
            f(&Completion::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::remember-info-visibility\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_remember_info_visibility_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_select_on_show_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_select_on_show_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletion, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Completion>
        {
            let f: &F = &*(f as *const F);
            f(&Completion::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::select-on-show\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_select_on_show_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_show_headers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_headers_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletion, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Completion>
        {
            let f: &F = &*(f as *const F);
            f(&Completion::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-headers\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_show_headers_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_show_icons_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_icons_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletion, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Completion>
        {
            let f: &F = &*(f as *const F);
            f(&Completion::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-icons\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_show_icons_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Completion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Completion")
    }
}
