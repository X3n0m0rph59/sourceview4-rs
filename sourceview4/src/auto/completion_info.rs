// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use gtk;
use gtk_source_sys;
use std::fmt;

glib_wrapper! {
    pub struct CompletionInfo(Object<gtk_source_sys::GtkSourceCompletionInfo, gtk_source_sys::GtkSourceCompletionInfoClass, CompletionInfoClass>) @extends gtk::Window, gtk::Bin, gtk::Container, gtk::Widget;

    match fn {
        get_type => || gtk_source_sys::gtk_source_completion_info_get_type(),
    }
}

impl CompletionInfo {
    pub fn new() -> CompletionInfo {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_completion_info_new())
        }
    }
}

impl Default for CompletionInfo {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_COMPLETION_INFO: Option<&CompletionInfo> = None;

impl fmt::Display for CompletionInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CompletionInfo")
    }
}
