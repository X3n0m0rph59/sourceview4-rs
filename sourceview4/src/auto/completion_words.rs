// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{CompletionActivation, CompletionProvider};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkSourceCompletionWords")]
    pub struct CompletionWords(Object<ffi::GtkSourceCompletionWords, ffi::GtkSourceCompletionWordsClass>) @implements CompletionProvider;

    match fn {
        type_ => || ffi::gtk_source_completion_words_get_type(),
    }
}

impl CompletionWords {
    pub const NONE: Option<&'static CompletionWords> = None;

    #[doc(alias = "gtk_source_completion_words_new")]
    pub fn new(name: Option<&str>, icon: Option<&gdk_pixbuf::Pixbuf>) -> CompletionWords {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_source_completion_words_new(
                name.to_glib_none().0,
                icon.to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`CompletionWords`] objects.
    ///
    /// This method returns an instance of [`CompletionWordsBuilder`](crate::builders::CompletionWordsBuilder) which can be used to create [`CompletionWords`] objects.
    pub fn builder() -> CompletionWordsBuilder {
        CompletionWordsBuilder::new()
    }
}

impl Default for CompletionWords {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`CompletionWords`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct CompletionWordsBuilder {
    builder: glib::object::ObjectBuilder<'static, CompletionWords>,
}

impl CompletionWordsBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn activation(self, activation: CompletionActivation) -> Self {
        Self {
            builder: self.builder.property("activation", activation),
        }
    }

    pub fn icon(self, icon: &gdk_pixbuf::Pixbuf) -> Self {
        Self {
            builder: self.builder.property("icon", icon.clone()),
        }
    }

    pub fn interactive_delay(self, interactive_delay: i32) -> Self {
        Self {
            builder: self
                .builder
                .property("interactive-delay", interactive_delay),
        }
    }

    pub fn minimum_word_size(self, minimum_word_size: u32) -> Self {
        Self {
            builder: self
                .builder
                .property("minimum-word-size", minimum_word_size),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn priority(self, priority: i32) -> Self {
        Self {
            builder: self.builder.property("priority", priority),
        }
    }

    pub fn proposals_batch_size(self, proposals_batch_size: u32) -> Self {
        Self {
            builder: self
                .builder
                .property("proposals-batch-size", proposals_batch_size),
        }
    }

    pub fn scan_batch_size(self, scan_batch_size: u32) -> Self {
        Self {
            builder: self.builder.property("scan-batch-size", scan_batch_size),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`CompletionWords`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> CompletionWords {
        self.builder.build()
    }
}

pub trait CompletionWordsExt: 'static {
    #[doc(alias = "gtk_source_completion_words_register")]
    fn register(&self, buffer: &impl IsA<gtk::TextBuffer>);

    #[doc(alias = "gtk_source_completion_words_unregister")]
    fn unregister(&self, buffer: &impl IsA<gtk::TextBuffer>);

    fn set_activation(&self, activation: CompletionActivation);

    fn set_icon(&self, icon: Option<&gdk_pixbuf::Pixbuf>);

    #[doc(alias = "interactive-delay")]
    fn set_interactive_delay(&self, interactive_delay: i32);

    #[doc(alias = "minimum-word-size")]
    fn minimum_word_size(&self) -> u32;

    #[doc(alias = "minimum-word-size")]
    fn set_minimum_word_size(&self, minimum_word_size: u32);

    fn set_name(&self, name: Option<&str>);

    fn set_priority(&self, priority: i32);

    #[doc(alias = "proposals-batch-size")]
    fn proposals_batch_size(&self) -> u32;

    #[doc(alias = "proposals-batch-size")]
    fn set_proposals_batch_size(&self, proposals_batch_size: u32);

    #[doc(alias = "scan-batch-size")]
    fn scan_batch_size(&self) -> u32;

    #[doc(alias = "scan-batch-size")]
    fn set_scan_batch_size(&self, scan_batch_size: u32);

    #[doc(alias = "activation")]
    fn connect_activation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "icon")]
    fn connect_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "interactive-delay")]
    fn connect_interactive_delay_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "minimum-word-size")]
    fn connect_minimum_word_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "name")]
    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "priority")]
    fn connect_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "proposals-batch-size")]
    fn connect_proposals_batch_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "scan-batch-size")]
    fn connect_scan_batch_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CompletionWords>> CompletionWordsExt for O {
    fn register(&self, buffer: &impl IsA<gtk::TextBuffer>) {
        unsafe {
            ffi::gtk_source_completion_words_register(
                self.as_ref().to_glib_none().0,
                buffer.as_ref().to_glib_none().0,
            );
        }
    }

    fn unregister(&self, buffer: &impl IsA<gtk::TextBuffer>) {
        unsafe {
            ffi::gtk_source_completion_words_unregister(
                self.as_ref().to_glib_none().0,
                buffer.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_activation(&self, activation: CompletionActivation) {
        glib::ObjectExt::set_property(self.as_ref(), "activation", &activation)
    }

    fn set_icon(&self, icon: Option<&gdk_pixbuf::Pixbuf>) {
        glib::ObjectExt::set_property(self.as_ref(), "icon", &icon)
    }

    fn set_interactive_delay(&self, interactive_delay: i32) {
        glib::ObjectExt::set_property(self.as_ref(), "interactive-delay", &interactive_delay)
    }

    fn minimum_word_size(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "minimum-word-size")
    }

    fn set_minimum_word_size(&self, minimum_word_size: u32) {
        glib::ObjectExt::set_property(self.as_ref(), "minimum-word-size", &minimum_word_size)
    }

    fn set_name(&self, name: Option<&str>) {
        glib::ObjectExt::set_property(self.as_ref(), "name", &name)
    }

    fn set_priority(&self, priority: i32) {
        glib::ObjectExt::set_property(self.as_ref(), "priority", &priority)
    }

    fn proposals_batch_size(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "proposals-batch-size")
    }

    fn set_proposals_batch_size(&self, proposals_batch_size: u32) {
        glib::ObjectExt::set_property(self.as_ref(), "proposals-batch-size", &proposals_batch_size)
    }

    fn scan_batch_size(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "scan-batch-size")
    }

    fn set_scan_batch_size(&self, scan_batch_size: u32) {
        glib::ObjectExt::set_property(self.as_ref(), "scan-batch-size", &scan_batch_size)
    }

    fn connect_activation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_activation_trampoline<
            P: IsA<CompletionWords>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletionWords,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CompletionWords::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::activation\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_activation_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_trampoline<
            P: IsA<CompletionWords>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletionWords,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CompletionWords::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_interactive_delay_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_interactive_delay_trampoline<
            P: IsA<CompletionWords>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletionWords,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CompletionWords::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::interactive-delay\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_interactive_delay_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_minimum_word_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_minimum_word_size_trampoline<
            P: IsA<CompletionWords>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletionWords,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CompletionWords::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::minimum-word-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_minimum_word_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<
            P: IsA<CompletionWords>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletionWords,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CompletionWords::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_priority_trampoline<
            P: IsA<CompletionWords>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletionWords,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CompletionWords::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::priority\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_priority_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_proposals_batch_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_proposals_batch_size_trampoline<
            P: IsA<CompletionWords>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletionWords,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CompletionWords::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::proposals-batch-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_proposals_batch_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_scan_batch_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_scan_batch_size_trampoline<
            P: IsA<CompletionWords>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletionWords,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CompletionWords::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::scan-batch-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_scan_batch_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for CompletionWords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CompletionWords")
    }
}
