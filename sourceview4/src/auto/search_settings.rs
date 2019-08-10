// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::GString;
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
    pub struct SearchSettings(Object<gtk_source_sys::GtkSourceSearchSettings, gtk_source_sys::GtkSourceSearchSettingsClass, SearchSettingsClass>);

    match fn {
        get_type => || gtk_source_sys::gtk_source_search_settings_get_type(),
    }
}

impl SearchSettings {
    pub fn new() -> SearchSettings {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_search_settings_new())
        }
    }
}

impl Default for SearchSettings {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SEARCH_SETTINGS: Option<&SearchSettings> = None;

pub trait SearchSettingsExt: 'static {
    fn get_at_word_boundaries(&self) -> bool;

    fn get_case_sensitive(&self) -> bool;

    fn get_regex_enabled(&self) -> bool;

    fn get_search_text(&self) -> Option<GString>;

    fn get_wrap_around(&self) -> bool;

    fn set_at_word_boundaries(&self, at_word_boundaries: bool);

    fn set_case_sensitive(&self, case_sensitive: bool);

    fn set_regex_enabled(&self, regex_enabled: bool);

    fn set_search_text(&self, search_text: Option<&str>);

    fn set_wrap_around(&self, wrap_around: bool);

    fn connect_property_at_word_boundaries_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_case_sensitive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_regex_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_search_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wrap_around_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SearchSettings>> SearchSettingsExt for O {
    fn get_at_word_boundaries(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_search_settings_get_at_word_boundaries(self.as_ref().to_glib_none().0))
        }
    }

    fn get_case_sensitive(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_search_settings_get_case_sensitive(self.as_ref().to_glib_none().0))
        }
    }

    fn get_regex_enabled(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_search_settings_get_regex_enabled(self.as_ref().to_glib_none().0))
        }
    }

    fn get_search_text(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_search_settings_get_search_text(self.as_ref().to_glib_none().0))
        }
    }

    fn get_wrap_around(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_search_settings_get_wrap_around(self.as_ref().to_glib_none().0))
        }
    }

    fn set_at_word_boundaries(&self, at_word_boundaries: bool) {
        unsafe {
            gtk_source_sys::gtk_source_search_settings_set_at_word_boundaries(self.as_ref().to_glib_none().0, at_word_boundaries.to_glib());
        }
    }

    fn set_case_sensitive(&self, case_sensitive: bool) {
        unsafe {
            gtk_source_sys::gtk_source_search_settings_set_case_sensitive(self.as_ref().to_glib_none().0, case_sensitive.to_glib());
        }
    }

    fn set_regex_enabled(&self, regex_enabled: bool) {
        unsafe {
            gtk_source_sys::gtk_source_search_settings_set_regex_enabled(self.as_ref().to_glib_none().0, regex_enabled.to_glib());
        }
    }

    fn set_search_text(&self, search_text: Option<&str>) {
        unsafe {
            gtk_source_sys::gtk_source_search_settings_set_search_text(self.as_ref().to_glib_none().0, search_text.to_glib_none().0);
        }
    }

    fn set_wrap_around(&self, wrap_around: bool) {
        unsafe {
            gtk_source_sys::gtk_source_search_settings_set_wrap_around(self.as_ref().to_glib_none().0, wrap_around.to_glib());
        }
    }

    fn connect_property_at_word_boundaries_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_at_word_boundaries_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceSearchSettings, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<SearchSettings>
        {
            let f: &F = &*(f as *const F);
            f(&SearchSettings::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::at-word-boundaries\0".as_ptr() as *const _,
                Some(transmute(notify_at_word_boundaries_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_case_sensitive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_case_sensitive_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceSearchSettings, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<SearchSettings>
        {
            let f: &F = &*(f as *const F);
            f(&SearchSettings::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::case-sensitive\0".as_ptr() as *const _,
                Some(transmute(notify_case_sensitive_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_regex_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_regex_enabled_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceSearchSettings, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<SearchSettings>
        {
            let f: &F = &*(f as *const F);
            f(&SearchSettings::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::regex-enabled\0".as_ptr() as *const _,
                Some(transmute(notify_regex_enabled_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_search_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_search_text_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceSearchSettings, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<SearchSettings>
        {
            let f: &F = &*(f as *const F);
            f(&SearchSettings::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::search-text\0".as_ptr() as *const _,
                Some(transmute(notify_search_text_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_wrap_around_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_wrap_around_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceSearchSettings, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<SearchSettings>
        {
            let f: &F = &*(f as *const F);
            f(&SearchSettings::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::wrap-around\0".as_ptr() as *const _,
                Some(transmute(notify_wrap_around_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for SearchSettings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SearchSettings")
    }
}
