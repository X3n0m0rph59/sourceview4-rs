// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

extern crate gdk_sys;
extern crate gio_sys;
extern crate glib_sys;
extern crate gobject_sys;
extern crate gtk_sys;
extern crate sourceview4_sys as gtk_source_sys;

#[macro_use]
extern crate glib;

#[macro_use]
extern crate bitflags;
extern crate cairo;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate gio;
extern crate gtk;
extern crate libc;
extern crate pango;

/// Asserts that this is the main thread and `gtk::init` has been called.
macro_rules! assert_initialized_main_thread {
    () => {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GtkSourceView may only be used from the main thread.");
            } else {
                panic!("Gtk has to be initialized before using GtkSourceView.");
            }
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

pub use glib::Error;

pub mod prelude;
pub use prelude::*;
mod auto;
pub use auto::*;
mod completion;
pub use completion::*;
mod completion_info;
pub use completion_info::*;
mod view;
pub use view::*;
mod gutter_renderer;
pub use gutter_renderer::*;
