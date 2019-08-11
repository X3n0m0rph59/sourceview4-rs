// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use CompletionActivation;
use CompletionProvider;
use gdk_pixbuf;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk;
use gtk_source_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct CompletionWords(Object<gtk_source_sys::GtkSourceCompletionWords, gtk_source_sys::GtkSourceCompletionWordsClass, CompletionWordsClass>) @implements CompletionProvider;

    match fn {
        get_type => || gtk_source_sys::gtk_source_completion_words_get_type(),
    }
}

impl CompletionWords {
    /// ## `name`
    /// The name for the provider, or `None`.
    /// ## `icon`
    /// A specific icon for the provider, or `None`.
    ///
    /// # Returns
    ///
    /// a new `CompletionWords` provider
    pub fn new(name: Option<&str>, icon: Option<&gdk_pixbuf::Pixbuf>) -> CompletionWords {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_completion_words_new(name.to_glib_none().0, icon.to_glib_none().0))
        }
    }
}

pub const NONE_COMPLETION_WORDS: Option<&CompletionWords> = None;

/// Trait containing all `CompletionWords` methods.
///
/// # Implementors
///
/// [`CompletionWords`](struct.CompletionWords.html)
pub trait CompletionWordsExt: 'static {
    /// Registers `buffer` in the `self` provider.
    /// ## `buffer`
    /// a `gtk::TextBuffer`
    fn register<P: IsA<gtk::TextBuffer>>(&self, buffer: &P);

    /// Unregisters `buffer` from the `self` provider.
    /// ## `buffer`
    /// a `gtk::TextBuffer`
    fn unregister<P: IsA<gtk::TextBuffer>>(&self, buffer: &P);

    /// The type of activation.
    fn set_property_activation(&self, activation: CompletionActivation);

    fn set_property_icon(&self, icon: Option<&gdk_pixbuf::Pixbuf>);

    fn set_property_interactive_delay(&self, interactive_delay: i32);

    fn get_property_minimum_word_size(&self) -> u32;

    fn set_property_minimum_word_size(&self, minimum_word_size: u32);

    fn set_property_name(&self, name: Option<&str>);

    fn set_property_priority(&self, priority: i32);

    fn get_property_proposals_batch_size(&self) -> u32;

    fn set_property_proposals_batch_size(&self, proposals_batch_size: u32);

    fn get_property_scan_batch_size(&self) -> u32;

    fn set_property_scan_batch_size(&self, scan_batch_size: u32);

    fn connect_property_activation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_interactive_delay_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_minimum_word_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_proposals_batch_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_scan_batch_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CompletionWords>> CompletionWordsExt for O {
    fn register<P: IsA<gtk::TextBuffer>>(&self, buffer: &P) {
        unsafe {
            gtk_source_sys::gtk_source_completion_words_register(self.as_ref().to_glib_none().0, buffer.as_ref().to_glib_none().0);
        }
    }

    fn unregister<P: IsA<gtk::TextBuffer>>(&self, buffer: &P) {
        unsafe {
            gtk_source_sys::gtk_source_completion_words_unregister(self.as_ref().to_glib_none().0, buffer.as_ref().to_glib_none().0);
        }
    }

    fn set_property_activation(&self, activation: CompletionActivation) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"activation\0".as_ptr() as *const _, Value::from(&activation).to_glib_none().0);
        }
    }

    fn set_property_icon(&self, icon: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"icon\0".as_ptr() as *const _, Value::from(icon).to_glib_none().0);
        }
    }

    fn set_property_interactive_delay(&self, interactive_delay: i32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"interactive-delay\0".as_ptr() as *const _, Value::from(&interactive_delay).to_glib_none().0);
        }
    }

    fn get_property_minimum_word_size(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"minimum-word-size\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_minimum_word_size(&self, minimum_word_size: u32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"minimum-word-size\0".as_ptr() as *const _, Value::from(&minimum_word_size).to_glib_none().0);
        }
    }

    fn set_property_name(&self, name: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"name\0".as_ptr() as *const _, Value::from(name).to_glib_none().0);
        }
    }

    fn set_property_priority(&self, priority: i32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"priority\0".as_ptr() as *const _, Value::from(&priority).to_glib_none().0);
        }
    }

    fn get_property_proposals_batch_size(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"proposals-batch-size\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_proposals_batch_size(&self, proposals_batch_size: u32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"proposals-batch-size\0".as_ptr() as *const _, Value::from(&proposals_batch_size).to_glib_none().0);
        }
    }

    fn get_property_scan_batch_size(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"scan-batch-size\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_scan_batch_size(&self, scan_batch_size: u32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"scan-batch-size\0".as_ptr() as *const _, Value::from(&scan_batch_size).to_glib_none().0);
        }
    }

    fn connect_property_activation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_activation_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletionWords, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<CompletionWords>
        {
            let f: &F = &*(f as *const F);
            f(&CompletionWords::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::activation\0".as_ptr() as *const _,
                Some(transmute(notify_activation_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletionWords, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<CompletionWords>
        {
            let f: &F = &*(f as *const F);
            f(&CompletionWords::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon\0".as_ptr() as *const _,
                Some(transmute(notify_icon_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_interactive_delay_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_interactive_delay_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletionWords, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<CompletionWords>
        {
            let f: &F = &*(f as *const F);
            f(&CompletionWords::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::interactive-delay\0".as_ptr() as *const _,
                Some(transmute(notify_interactive_delay_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_minimum_word_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_minimum_word_size_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletionWords, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<CompletionWords>
        {
            let f: &F = &*(f as *const F);
            f(&CompletionWords::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::minimum-word-size\0".as_ptr() as *const _,
                Some(transmute(notify_minimum_word_size_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletionWords, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<CompletionWords>
        {
            let f: &F = &*(f as *const F);
            f(&CompletionWords::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::name\0".as_ptr() as *const _,
                Some(transmute(notify_name_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_priority_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletionWords, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<CompletionWords>
        {
            let f: &F = &*(f as *const F);
            f(&CompletionWords::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::priority\0".as_ptr() as *const _,
                Some(transmute(notify_priority_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_proposals_batch_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_proposals_batch_size_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletionWords, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<CompletionWords>
        {
            let f: &F = &*(f as *const F);
            f(&CompletionWords::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::proposals-batch-size\0".as_ptr() as *const _,
                Some(transmute(notify_proposals_batch_size_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_scan_batch_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_scan_batch_size_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletionWords, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<CompletionWords>
        {
            let f: &F = &*(f as *const F);
            f(&CompletionWords::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::scan-batch-size\0".as_ptr() as *const _,
                Some(transmute(notify_scan_batch_size_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for CompletionWords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CompletionWords")
    }
}