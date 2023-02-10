// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkSourceUndoManager")]
    pub struct UndoManager(Interface<ffi::GtkSourceUndoManager, ffi::GtkSourceUndoManagerIface>);

    match fn {
        type_ => || ffi::gtk_source_undo_manager_get_type(),
    }
}

impl UndoManager {
    pub const NONE: Option<&'static UndoManager> = None;
}

pub trait UndoManagerExt: 'static {
    #[doc(alias = "gtk_source_undo_manager_begin_not_undoable_action")]
    fn begin_not_undoable_action(&self);

    #[doc(alias = "gtk_source_undo_manager_can_redo")]
    fn can_redo(&self) -> bool;

    #[doc(alias = "gtk_source_undo_manager_can_redo_changed")]
    fn can_redo_changed(&self);

    #[doc(alias = "gtk_source_undo_manager_can_undo")]
    fn can_undo(&self) -> bool;

    #[doc(alias = "gtk_source_undo_manager_can_undo_changed")]
    fn can_undo_changed(&self);

    #[doc(alias = "gtk_source_undo_manager_end_not_undoable_action")]
    fn end_not_undoable_action(&self);

    #[doc(alias = "gtk_source_undo_manager_redo")]
    fn redo(&self);

    #[doc(alias = "gtk_source_undo_manager_undo")]
    fn undo(&self);

    #[doc(alias = "can-redo-changed")]
    fn connect_can_redo_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_can_redo_changed(&self);

    #[doc(alias = "can-undo-changed")]
    fn connect_can_undo_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_can_undo_changed(&self);
}

impl<O: IsA<UndoManager>> UndoManagerExt for O {
    fn begin_not_undoable_action(&self) {
        unsafe {
            ffi::gtk_source_undo_manager_begin_not_undoable_action(self.as_ref().to_glib_none().0);
        }
    }

    fn can_redo(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_undo_manager_can_redo(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn can_redo_changed(&self) {
        unsafe {
            ffi::gtk_source_undo_manager_can_redo_changed(self.as_ref().to_glib_none().0);
        }
    }

    fn can_undo(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_undo_manager_can_undo(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn can_undo_changed(&self) {
        unsafe {
            ffi::gtk_source_undo_manager_can_undo_changed(self.as_ref().to_glib_none().0);
        }
    }

    fn end_not_undoable_action(&self) {
        unsafe {
            ffi::gtk_source_undo_manager_end_not_undoable_action(self.as_ref().to_glib_none().0);
        }
    }

    fn redo(&self) {
        unsafe {
            ffi::gtk_source_undo_manager_redo(self.as_ref().to_glib_none().0);
        }
    }

    fn undo(&self) {
        unsafe {
            ffi::gtk_source_undo_manager_undo(self.as_ref().to_glib_none().0);
        }
    }

    fn connect_can_redo_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn can_redo_changed_trampoline<
            P: IsA<UndoManager>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceUndoManager,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(UndoManager::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"can-redo-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    can_redo_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_can_redo_changed(&self) {
        self.emit_by_name::<()>("can-redo-changed", &[]);
    }

    fn connect_can_undo_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn can_undo_changed_trampoline<
            P: IsA<UndoManager>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceUndoManager,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(UndoManager::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"can-undo-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    can_undo_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_can_undo_changed(&self) {
        self.emit_by_name::<()>("can-undo-changed", &[]);
    }
}

impl fmt::Display for UndoManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("UndoManager")
    }
}
