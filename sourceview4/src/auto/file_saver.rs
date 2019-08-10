// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buffer;
use CompressionType;
use Encoding;
use File;
use FileSaverFlags;
use NewlineType;
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
    pub struct FileSaver(Object<gtk_source_sys::GtkSourceFileSaver, gtk_source_sys::GtkSourceFileSaverClass, FileSaverClass>);

    match fn {
        get_type => || gtk_source_sys::gtk_source_file_saver_get_type(),
    }
}

impl FileSaver {
    /// Creates a new `FileSaver` object. The `buffer` will be saved to the
    /// `File`'s location.
    ///
    /// This constructor is suitable for a simple "save" operation, when the `file`
    /// already contains a non-`None` `File:location`.
    /// ## `buffer`
    /// the `Buffer` to save.
    /// ## `file`
    /// the `File`.
    ///
    /// # Returns
    ///
    /// a new `FileSaver` object.
    pub fn new<P: IsA<Buffer>, Q: IsA<File>>(buffer: &P, file: &Q) -> FileSaver {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_file_saver_new(buffer.as_ref().to_glib_none().0, file.as_ref().to_glib_none().0))
        }
    }

    /// Creates a new `FileSaver` object with a target location. When the
    /// file saving is finished successfully, `target_location` is set to the `file`'s
    /// `File:location` property. If an error occurs, the previous valid
    /// location is still available in `File`.
    ///
    /// This constructor is suitable for a "save as" operation, or for saving a new
    /// buffer for the first time.
    /// ## `buffer`
    /// the `Buffer` to save.
    /// ## `file`
    /// the `File`.
    /// ## `target_location`
    /// the `gio::File` where to save the buffer to.
    ///
    /// # Returns
    ///
    /// a new `FileSaver` object.
    pub fn new_with_target<P: IsA<Buffer>, Q: IsA<File>, R: IsA<gio::File>>(buffer: &P, file: &Q, target_location: &R) -> FileSaver {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_file_saver_new_with_target(buffer.as_ref().to_glib_none().0, file.as_ref().to_glib_none().0, target_location.as_ref().to_glib_none().0))
        }
    }
}

pub const NONE_FILE_SAVER: Option<&FileSaver> = None;

/// Trait containing all `FileSaver` methods.
///
/// # Implementors
///
/// [`FileSaver`](struct.FileSaver.html)
pub trait FileSaverExt: 'static {
    ///
    /// # Returns
    ///
    /// the `Buffer` to save.
    fn get_buffer(&self) -> Option<Buffer>;

    ///
    /// # Returns
    ///
    /// the compression type.
    fn get_compression_type(&self) -> CompressionType;

    ///
    /// # Returns
    ///
    /// the encoding.
    fn get_encoding(&self) -> Option<Encoding>;

    ///
    /// # Returns
    ///
    /// the `File`.
    fn get_file(&self) -> Option<File>;

    ///
    /// # Returns
    ///
    /// the flags.
    fn get_flags(&self) -> FileSaverFlags;

    ///
    /// # Returns
    ///
    /// the `gio::File` where to save the buffer to.
    fn get_location(&self) -> Option<gio::File>;

    ///
    /// # Returns
    ///
    /// the newline type.
    fn get_newline_type(&self) -> NewlineType;

    //fn save_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), Error>) + Send + 'static, R: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: glib::Priority, cancellable: Option<&P>, progress_callback: Q, progress_callback_notify: Fn() + 'static, callback: R);

    //#[cfg(feature = "futures")]
    //fn save_async_future<Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: glib::Priority, progress_callback: Q, progress_callback_notify: Fn() + 'static) -> Box_<dyn future::Future<Output = Result<(), Error>> + std::marker::Unpin>;

    /// Sets the compression type. By default the compression type is taken from the
    /// `File`.
    /// ## `compression_type`
    /// the new compression type.
    fn set_compression_type(&self, compression_type: CompressionType);

    /// Sets the encoding. If `encoding` is `None`, the UTF-8 encoding will be set.
    /// By default the encoding is taken from the `File`.
    /// ## `encoding`
    /// the new encoding, or `None` for UTF-8.
    fn set_encoding(&self, encoding: Option<&Encoding>);

    /// ## `flags`
    /// the new flags.
    fn set_flags(&self, flags: FileSaverFlags);

    /// Sets the newline type. By default the newline type is taken from the
    /// `File`.
    /// ## `newline_type`
    /// the new newline type.
    fn set_newline_type(&self, newline_type: NewlineType);

    fn connect_property_compression_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_encoding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_newline_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FileSaver>> FileSaverExt for O {
    fn get_buffer(&self) -> Option<Buffer> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_file_saver_get_buffer(self.as_ref().to_glib_none().0))
        }
    }

    fn get_compression_type(&self) -> CompressionType {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_file_saver_get_compression_type(self.as_ref().to_glib_none().0))
        }
    }

    fn get_encoding(&self) -> Option<Encoding> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_file_saver_get_encoding(self.as_ref().to_glib_none().0))
        }
    }

    fn get_file(&self) -> Option<File> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_file_saver_get_file(self.as_ref().to_glib_none().0))
        }
    }

    fn get_flags(&self) -> FileSaverFlags {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_file_saver_get_flags(self.as_ref().to_glib_none().0))
        }
    }

    fn get_location(&self) -> Option<gio::File> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_file_saver_get_location(self.as_ref().to_glib_none().0))
        }
    }

    fn get_newline_type(&self) -> NewlineType {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_file_saver_get_newline_type(self.as_ref().to_glib_none().0))
        }
    }

    //fn save_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), Error>) + Send + 'static, R: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: glib::Priority, cancellable: Option<&P>, progress_callback: Q, progress_callback_notify: Fn() + 'static, callback: R) {
    //    unsafe { TODO: call gtk_source_sys:gtk_source_file_saver_save_async() }
    //}

    //#[cfg(feature = "futures")]
    //fn save_async_future<Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: glib::Priority, progress_callback: Q, progress_callback_notify: Fn() + 'static) -> Box_<dyn future::Future<Output = Result<(), Error>> + std::marker::Unpin> {
        //use gio::GioFuture;
        //use fragile::Fragile;

        //let progress_callback = progress_callback.map(ToOwned::to_owned);
        //let progress_callback_notify = progress_callback_notify.map(ToOwned::to_owned);
        //GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = Fragile::new(send);
        //    obj.save_async(
        //        io_priority,
        //        Some(&cancellable),
        //        progress_callback.as_ref().map(::std::borrow::Borrow::borrow),
        //        progress_callback_notify.as_ref().map(::std::borrow::Borrow::borrow),
        //        move |res| {
        //            let _ = send.into_inner().send(res);
        //        },
        //    );

        //    cancellable
        //})
    //}

    fn set_compression_type(&self, compression_type: CompressionType) {
        unsafe {
            gtk_source_sys::gtk_source_file_saver_set_compression_type(self.as_ref().to_glib_none().0, compression_type.to_glib());
        }
    }

    fn set_encoding(&self, encoding: Option<&Encoding>) {
        unsafe {
            gtk_source_sys::gtk_source_file_saver_set_encoding(self.as_ref().to_glib_none().0, encoding.to_glib_none().0);
        }
    }

    fn set_flags(&self, flags: FileSaverFlags) {
        unsafe {
            gtk_source_sys::gtk_source_file_saver_set_flags(self.as_ref().to_glib_none().0, flags.to_glib());
        }
    }

    fn set_newline_type(&self, newline_type: NewlineType) {
        unsafe {
            gtk_source_sys::gtk_source_file_saver_set_newline_type(self.as_ref().to_glib_none().0, newline_type.to_glib());
        }
    }

    fn connect_property_compression_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_compression_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceFileSaver, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<FileSaver>
        {
            let f: &F = &*(f as *const F);
            f(&FileSaver::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::compression-type\0".as_ptr() as *const _,
                Some(transmute(notify_compression_type_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_encoding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_encoding_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceFileSaver, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<FileSaver>
        {
            let f: &F = &*(f as *const F);
            f(&FileSaver::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::encoding\0".as_ptr() as *const _,
                Some(transmute(notify_encoding_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_flags_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceFileSaver, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<FileSaver>
        {
            let f: &F = &*(f as *const F);
            f(&FileSaver::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::flags\0".as_ptr() as *const _,
                Some(transmute(notify_flags_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_newline_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_newline_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceFileSaver, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<FileSaver>
        {
            let f: &F = &*(f as *const F);
            f(&FileSaver::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::newline-type\0".as_ptr() as *const _,
                Some(transmute(notify_newline_type_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for FileSaver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FileSaver")
    }
}
