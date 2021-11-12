// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::GutterRenderer;
use crate::View;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkSourceGutter")]
    pub struct Gutter(Object<ffi::GtkSourceGutter, ffi::GtkSourceGutterClass>);

    match fn {
        type_ => || ffi::gtk_source_gutter_get_type(),
    }
}

impl Gutter {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Gutter`] objects.
    ///
    /// This method returns an instance of [`GutterBuilder`] which can be used to create [`Gutter`] objects.
    pub fn builder() -> GutterBuilder {
        GutterBuilder::default()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Gutter`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct GutterBuilder {
    view: Option<View>,
    window_type: Option<gtk::TextWindowType>,
}

impl GutterBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`GutterBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Gutter`].
    pub fn build(self) -> Gutter {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref view) = self.view {
            properties.push(("view", view));
        }
        if let Some(ref window_type) = self.window_type {
            properties.push(("window-type", window_type));
        }
        glib::Object::new::<Gutter>(&properties).expect("Failed to create an instance of Gutter")
    }

    pub fn view(mut self, view: &impl IsA<View>) -> Self {
        self.view = Some(view.clone().upcast());
        self
    }

    pub fn window_type(mut self, window_type: gtk::TextWindowType) -> Self {
        self.window_type = Some(window_type);
        self
    }
}

impl Gutter {
    pub const NONE: Option<&'static Gutter> = None;
}

pub trait GutterExt: 'static {
    #[doc(alias = "gtk_source_gutter_get_renderer_at_pos")]
    #[doc(alias = "get_renderer_at_pos")]
    fn renderer_at_pos(&self, x: i32, y: i32) -> Option<GutterRenderer>;

    #[doc(alias = "gtk_source_gutter_get_view")]
    #[doc(alias = "get_view")]
    fn view(&self) -> Option<View>;

    #[doc(alias = "gtk_source_gutter_get_window_type")]
    #[doc(alias = "get_window_type")]
    fn window_type(&self) -> gtk::TextWindowType;

    #[doc(alias = "gtk_source_gutter_insert")]
    fn insert(&self, renderer: &impl IsA<GutterRenderer>, position: i32) -> bool;

    #[doc(alias = "gtk_source_gutter_queue_draw")]
    fn queue_draw(&self);

    #[doc(alias = "gtk_source_gutter_remove")]
    fn remove(&self, renderer: &impl IsA<GutterRenderer>);

    #[doc(alias = "gtk_source_gutter_reorder")]
    fn reorder(&self, renderer: &impl IsA<GutterRenderer>, position: i32);
}

impl<O: IsA<Gutter>> GutterExt for O {
    fn renderer_at_pos(&self, x: i32, y: i32) -> Option<GutterRenderer> {
        unsafe {
            from_glib_none(ffi::gtk_source_gutter_get_renderer_at_pos(
                self.as_ref().to_glib_none().0,
                x,
                y,
            ))
        }
    }

    fn view(&self) -> Option<View> {
        unsafe {
            from_glib_none(ffi::gtk_source_gutter_get_view(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn window_type(&self) -> gtk::TextWindowType {
        unsafe {
            from_glib(ffi::gtk_source_gutter_get_window_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn insert(&self, renderer: &impl IsA<GutterRenderer>, position: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_gutter_insert(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                position,
            ))
        }
    }

    fn queue_draw(&self) {
        unsafe {
            ffi::gtk_source_gutter_queue_draw(self.as_ref().to_glib_none().0);
        }
    }

    fn remove(&self, renderer: &impl IsA<GutterRenderer>) {
        unsafe {
            ffi::gtk_source_gutter_remove(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
            );
        }
    }

    fn reorder(&self, renderer: &impl IsA<GutterRenderer>, position: i32) {
        unsafe {
            ffi::gtk_source_gutter_reorder(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                position,
            );
        }
    }
}

impl fmt::Display for Gutter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Gutter")
    }
}
