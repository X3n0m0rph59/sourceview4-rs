// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use CompressionType;
use Encoding;
use NewlineType;
use gio;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_source_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct File(Object<gtk_source_sys::GtkSourceFile, gtk_source_sys::GtkSourceFileClass, FileClass>);

    match fn {
        get_type => || gtk_source_sys::gtk_source_file_get_type(),
    }
}

impl File {
    ///
    /// # Returns
    ///
    /// a new `File` object.
    pub fn new() -> File {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_file_new())
        }
    }
}

impl Default for File {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_FILE: Option<&File> = None;

/// Trait containing all `File` methods.
///
/// # Implementors
///
/// [`File`](struct.File.html)
pub trait FileExt: 'static {
    /// Checks synchronously the file on disk, to know whether the file is externally
    /// modified, or has been deleted, and whether the file is read-only.
    ///
    /// `File` doesn't create a `gio::FileMonitor` to track those properties, so
    /// this function needs to be called instead. Creating lots of `gio::FileMonitor`'s
    /// would take lots of resources.
    ///
    /// Since this function is synchronous, it is advised to call it only on local
    /// files. See `FileExt::is_local`.
    fn check_file_on_disk(&self);

    ///
    /// # Returns
    ///
    /// the compression type.
    fn get_compression_type(&self) -> CompressionType;

    /// The encoding is initially `None`. After a successful file loading or saving
    /// operation, the encoding is non-`None`.
    ///
    /// # Returns
    ///
    /// the character encoding.
    fn get_encoding(&self) -> Option<Encoding>;

    ///
    /// # Returns
    ///
    /// the `gio::File`.
    fn get_location(&self) -> Option<gio::File>;

    ///
    /// # Returns
    ///
    /// the newline type.
    fn get_newline_type(&self) -> NewlineType;

    /// Returns whether the file has been deleted. If the
    /// `File:location` is `None`, returns `false`.
    ///
    /// To have an up-to-date value, you must first call
    /// `FileExt::check_file_on_disk`.
    ///
    /// # Returns
    ///
    /// whether the file has been deleted.
    fn is_deleted(&self) -> bool;

    /// Returns whether the file is externally modified. If the
    /// `File:location` is `None`, returns `false`.
    ///
    /// To have an up-to-date value, you must first call
    /// `FileExt::check_file_on_disk`.
    ///
    /// # Returns
    ///
    /// whether the file is externally modified.
    fn is_externally_modified(&self) -> bool;

    /// Returns whether the file is local. If the `File:location` is `None`,
    /// returns `false`.
    ///
    /// # Returns
    ///
    /// whether the file is local.
    fn is_local(&self) -> bool;

    /// Returns whether the file is read-only. If the
    /// `File:location` is `None`, returns `false`.
    ///
    /// To have an up-to-date value, you must first call
    /// `FileExt::check_file_on_disk`.
    ///
    /// # Returns
    ///
    /// whether the file is read-only.
    fn is_readonly(&self) -> bool;

    /// Sets the location.
    /// ## `location`
    /// the new `gio::File`, or `None`.
    fn set_location<P: IsA<gio::File>>(&self, location: Option<&P>);

    //fn set_mount_operation_factory(&self, callback: /*Unimplemented*/Fn(&File, /*Unimplemented*/Option<Fundamental: Pointer>) -> gio::MountOperation, user_data: /*Unimplemented*/Option<Fundamental: Pointer>);

    /// Whether the file is read-only or not. The value of this property is
    /// not updated automatically (there is no file monitors).
    fn get_property_read_only(&self) -> bool;

    fn connect_property_compression_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_encoding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_newline_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_read_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<File>> FileExt for O {
    fn check_file_on_disk(&self) {
        unsafe {
            gtk_source_sys::gtk_source_file_check_file_on_disk(self.as_ref().to_glib_none().0);
        }
    }

    fn get_compression_type(&self) -> CompressionType {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_file_get_compression_type(self.as_ref().to_glib_none().0))
        }
    }

    fn get_encoding(&self) -> Option<Encoding> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_file_get_encoding(self.as_ref().to_glib_none().0))
        }
    }

    fn get_location(&self) -> Option<gio::File> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_file_get_location(self.as_ref().to_glib_none().0))
        }
    }

    fn get_newline_type(&self) -> NewlineType {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_file_get_newline_type(self.as_ref().to_glib_none().0))
        }
    }

    fn is_deleted(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_file_is_deleted(self.as_ref().to_glib_none().0))
        }
    }

    fn is_externally_modified(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_file_is_externally_modified(self.as_ref().to_glib_none().0))
        }
    }

    fn is_local(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_file_is_local(self.as_ref().to_glib_none().0))
        }
    }

    fn is_readonly(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_file_is_readonly(self.as_ref().to_glib_none().0))
        }
    }

    fn set_location<P: IsA<gio::File>>(&self, location: Option<&P>) {
        unsafe {
            gtk_source_sys::gtk_source_file_set_location(self.as_ref().to_glib_none().0, location.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    //fn set_mount_operation_factory(&self, callback: /*Unimplemented*/Fn(&File, /*Unimplemented*/Option<Fundamental: Pointer>) -> gio::MountOperation, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call gtk_source_sys:gtk_source_file_set_mount_operation_factory() }
    //}

    fn get_property_read_only(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"read-only\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn connect_property_compression_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_compression_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceFile, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<File>
        {
            let f: &F = &*(f as *const F);
            f(&File::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::compression-type\0".as_ptr() as *const _,
                Some(transmute(notify_compression_type_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_encoding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_encoding_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceFile, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<File>
        {
            let f: &F = &*(f as *const F);
            f(&File::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::encoding\0".as_ptr() as *const _,
                Some(transmute(notify_encoding_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_location_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceFile, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<File>
        {
            let f: &F = &*(f as *const F);
            f(&File::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::location\0".as_ptr() as *const _,
                Some(transmute(notify_location_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_newline_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_newline_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceFile, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<File>
        {
            let f: &F = &*(f as *const F);
            f(&File::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::newline-type\0".as_ptr() as *const _,
                Some(transmute(notify_newline_type_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_read_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_read_only_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceFile, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<File>
        {
            let f: &F = &*(f as *const F);
            f(&File::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::read-only\0".as_ptr() as *const _,
                Some(transmute(notify_read_only_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "File")
    }
}
