// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::translate::*;
use gtk;
use gtk_source_sys;
use std::fmt;

glib_wrapper! {
    pub struct StyleSchemeChooserWidget(Object<gtk_source_sys::GtkSourceStyleSchemeChooserWidget, gtk_source_sys::GtkSourceStyleSchemeChooserWidgetClass, StyleSchemeChooserWidgetClass>) @extends gtk::Bin, gtk::Container, gtk::Widget;

    match fn {
        get_type => || gtk_source_sys::gtk_source_style_scheme_chooser_widget_get_type(),
    }
}

impl StyleSchemeChooserWidget {
    /// Creates a new `StyleSchemeChooserWidget`.
    ///
    /// # Returns
    ///
    /// a new `StyleSchemeChooserWidget`.
    pub fn new() -> StyleSchemeChooserWidget {
        assert_initialized_main_thread!();
        unsafe {
            gtk::Widget::from_glib_none(gtk_source_sys::gtk_source_style_scheme_chooser_widget_new()).unsafe_cast()
        }
    }
}

impl Default for StyleSchemeChooserWidget {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_STYLE_SCHEME_CHOOSER_WIDGET: Option<&StyleSchemeChooserWidget> = None;

impl fmt::Display for StyleSchemeChooserWidget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StyleSchemeChooserWidget")
    }
}
