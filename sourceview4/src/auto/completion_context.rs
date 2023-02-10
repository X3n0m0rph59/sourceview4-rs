// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Completion, CompletionActivation, CompletionProposal, CompletionProvider};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkSourceCompletionContext")]
    pub struct CompletionContext(Object<ffi::GtkSourceCompletionContext, ffi::GtkSourceCompletionContextClass>);

    match fn {
        type_ => || ffi::gtk_source_completion_context_get_type(),
    }
}

impl CompletionContext {
    pub const NONE: Option<&'static CompletionContext> = None;

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`CompletionContext`] objects.
    ///
    /// This method returns an instance of [`CompletionContextBuilder`](crate::builders::CompletionContextBuilder) which can be used to create [`CompletionContext`] objects.
    pub fn builder() -> CompletionContextBuilder {
        CompletionContextBuilder::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`CompletionContext`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct CompletionContextBuilder {
    builder: glib::object::ObjectBuilder<'static, CompletionContext>,
}

impl CompletionContextBuilder {
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

    pub fn completion(self, completion: &impl IsA<Completion>) -> Self {
        Self {
            builder: self
                .builder
                .property("completion", completion.clone().upcast()),
        }
    }

    pub fn iter(self, iter: &gtk::TextIter) -> Self {
        Self {
            builder: self.builder.property("iter", iter),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`CompletionContext`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> CompletionContext {
        self.builder.build()
    }
}

pub trait CompletionContextExt: 'static {
    #[doc(alias = "gtk_source_completion_context_add_proposals")]
    fn add_proposals(
        &self,
        provider: &impl IsA<CompletionProvider>,
        proposals: &[CompletionProposal],
        finished: bool,
    );

    #[doc(alias = "gtk_source_completion_context_get_activation")]
    #[doc(alias = "get_activation")]
    fn activation(&self) -> CompletionActivation;

    #[doc(alias = "gtk_source_completion_context_get_iter")]
    #[doc(alias = "get_iter")]
    fn iter(&self) -> Option<gtk::TextIter>;

    fn set_activation(&self, activation: CompletionActivation);

    fn completion(&self) -> Option<Completion>;

    fn set_iter(&self, iter: Option<&gtk::TextIter>);

    #[doc(alias = "cancelled")]
    fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_cancelled(&self);

    #[doc(alias = "activation")]
    fn connect_activation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "iter")]
    fn connect_iter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CompletionContext>> CompletionContextExt for O {
    fn add_proposals(
        &self,
        provider: &impl IsA<CompletionProvider>,
        proposals: &[CompletionProposal],
        finished: bool,
    ) {
        unsafe {
            ffi::gtk_source_completion_context_add_proposals(
                self.as_ref().to_glib_none().0,
                provider.as_ref().to_glib_none().0,
                proposals.to_glib_none().0,
                finished.into_glib(),
            );
        }
    }

    fn activation(&self) -> CompletionActivation {
        unsafe {
            from_glib(ffi::gtk_source_completion_context_get_activation(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn iter(&self) -> Option<gtk::TextIter> {
        unsafe {
            let mut iter = gtk::TextIter::uninitialized();
            let ret = from_glib(ffi::gtk_source_completion_context_get_iter(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
            ));
            if ret {
                Some(iter)
            } else {
                None
            }
        }
    }

    fn set_activation(&self, activation: CompletionActivation) {
        glib::ObjectExt::set_property(self.as_ref(), "activation", &activation)
    }

    fn completion(&self) -> Option<Completion> {
        glib::ObjectExt::property(self.as_ref(), "completion")
    }

    fn set_iter(&self, iter: Option<&gtk::TextIter>) {
        glib::ObjectExt::set_property(self.as_ref(), "iter", &iter)
    }

    fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cancelled_trampoline<
            P: IsA<CompletionContext>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletionContext,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CompletionContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cancelled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cancelled_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_cancelled(&self) {
        self.emit_by_name::<()>("cancelled", &[]);
    }

    fn connect_activation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_activation_trampoline<
            P: IsA<CompletionContext>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletionContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CompletionContext::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_iter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_iter_trampoline<
            P: IsA<CompletionContext>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletionContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CompletionContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::iter\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_iter_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for CompletionContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CompletionContext")
    }
}
