// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk_pixbuf;
use gio;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use gobject_sys;
use gtk_source_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct CompletionProposal(Interface<gtk_source_sys::GtkSourceCompletionProposal>);

    match fn {
        get_type => || gtk_source_sys::gtk_source_completion_proposal_get_type(),
    }
}

pub const NONE_COMPLETION_PROPOSAL: Option<&CompletionProposal> = None;

pub trait CompletionProposalExt: 'static {
    fn changed(&self);

    fn equal<P: IsA<CompletionProposal>>(&self, other: &P) -> bool;

    fn get_gicon(&self) -> Option<gio::Icon>;

    fn get_icon(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn get_icon_name(&self) -> Option<GString>;

    fn get_info(&self) -> Option<GString>;

    fn get_label(&self) -> Option<GString>;

    fn get_markup(&self) -> Option<GString>;

    fn get_text(&self) -> Option<GString>;

    fn hash(&self) -> u32;

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_changed(&self);
}

impl<O: IsA<CompletionProposal>> CompletionProposalExt for O {
    fn changed(&self) {
        unsafe {
            gtk_source_sys::gtk_source_completion_proposal_changed(self.as_ref().to_glib_none().0);
        }
    }

    fn equal<P: IsA<CompletionProposal>>(&self, other: &P) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_completion_proposal_equal(self.as_ref().to_glib_none().0, other.as_ref().to_glib_none().0))
        }
    }

    fn get_gicon(&self) -> Option<gio::Icon> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_completion_proposal_get_gicon(self.as_ref().to_glib_none().0))
        }
    }

    fn get_icon(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_completion_proposal_get_icon(self.as_ref().to_glib_none().0))
        }
    }

    fn get_icon_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_completion_proposal_get_icon_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_info(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_completion_proposal_get_info(self.as_ref().to_glib_none().0))
        }
    }

    fn get_label(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_completion_proposal_get_label(self.as_ref().to_glib_none().0))
        }
    }

    fn get_markup(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_completion_proposal_get_markup(self.as_ref().to_glib_none().0))
        }
    }

    fn get_text(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_completion_proposal_get_text(self.as_ref().to_glib_none().0))
        }
    }

    fn hash(&self) -> u32 {
        unsafe {
            gtk_source_sys::gtk_source_completion_proposal_hash(self.as_ref().to_glib_none().0)
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceCompletionProposal, f: glib_sys::gpointer)
            where P: IsA<CompletionProposal>
        {
            let f: &F = &*(f as *const F);
            f(&CompletionProposal::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(changed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn emit_changed(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject).emit("changed", &[]).unwrap() };
    }
}

impl fmt::Display for CompletionProposal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CompletionProposal")
    }
}
