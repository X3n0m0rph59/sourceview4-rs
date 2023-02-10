// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{GutterRendererAlignmentMode, GutterRendererState};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkSourceGutterRenderer")]
    pub struct GutterRenderer(Object<ffi::GtkSourceGutterRenderer, ffi::GtkSourceGutterRendererClass>);

    match fn {
        type_ => || ffi::gtk_source_gutter_renderer_get_type(),
    }
}

impl GutterRenderer {
    pub const NONE: Option<&'static GutterRenderer> = None;
}

pub trait GutterRendererExt: 'static {
    #[doc(alias = "gtk_source_gutter_renderer_activate")]
    fn activate(&self, iter: &mut gtk::TextIter, area: &mut gdk::Rectangle, event: &mut gdk::Event);

    #[doc(alias = "gtk_source_gutter_renderer_end")]
    fn end(&self);

    #[doc(alias = "gtk_source_gutter_renderer_get_alignment")]
    #[doc(alias = "get_alignment")]
    fn alignment(&self) -> (f32, f32);

    #[doc(alias = "gtk_source_gutter_renderer_get_alignment_mode")]
    #[doc(alias = "get_alignment_mode")]
    fn alignment_mode(&self) -> GutterRendererAlignmentMode;

    #[doc(alias = "gtk_source_gutter_renderer_get_background")]
    #[doc(alias = "get_background")]
    fn background(&self) -> Option<gdk::RGBA>;

    #[doc(alias = "gtk_source_gutter_renderer_get_padding")]
    #[doc(alias = "get_padding")]
    fn padding(&self) -> (i32, i32);

    #[doc(alias = "gtk_source_gutter_renderer_get_size")]
    #[doc(alias = "get_size")]
    fn size(&self) -> i32;

    #[doc(alias = "gtk_source_gutter_renderer_get_view")]
    #[doc(alias = "get_view")]
    fn view(&self) -> Option<gtk::TextView>;

    #[doc(alias = "gtk_source_gutter_renderer_get_visible")]
    #[doc(alias = "get_visible")]
    fn is_visible(&self) -> bool;

    #[doc(alias = "gtk_source_gutter_renderer_get_window_type")]
    #[doc(alias = "get_window_type")]
    fn window_type(&self) -> gtk::TextWindowType;

    #[doc(alias = "gtk_source_gutter_renderer_query_activatable")]
    fn query_activatable(
        &self,
        iter: &mut gtk::TextIter,
        area: &mut gdk::Rectangle,
        event: &mut gdk::Event,
    ) -> bool;

    #[doc(alias = "gtk_source_gutter_renderer_query_data")]
    fn query_data(
        &self,
        start: &mut gtk::TextIter,
        end: &mut gtk::TextIter,
        state: GutterRendererState,
    );

    #[doc(alias = "gtk_source_gutter_renderer_query_tooltip")]
    fn query_tooltip(
        &self,
        iter: &mut gtk::TextIter,
        area: &mut gdk::Rectangle,
        x: i32,
        y: i32,
        tooltip: &gtk::Tooltip,
    ) -> bool;

    #[doc(alias = "gtk_source_gutter_renderer_queue_draw")]
    fn queue_draw(&self);

    #[doc(alias = "gtk_source_gutter_renderer_set_alignment")]
    fn set_alignment(&self, xalign: f32, yalign: f32);

    #[doc(alias = "gtk_source_gutter_renderer_set_alignment_mode")]
    fn set_alignment_mode(&self, mode: GutterRendererAlignmentMode);

    #[doc(alias = "gtk_source_gutter_renderer_set_background")]
    fn set_background(&self, color: Option<&gdk::RGBA>);

    #[doc(alias = "gtk_source_gutter_renderer_set_padding")]
    fn set_padding(&self, xpad: i32, ypad: i32);

    #[doc(alias = "gtk_source_gutter_renderer_set_size")]
    fn set_size(&self, size: i32);

    #[doc(alias = "gtk_source_gutter_renderer_set_visible")]
    fn set_visible(&self, visible: bool);

    #[doc(alias = "background-rgba")]
    fn background_rgba(&self) -> Option<gdk::RGBA>;

    #[doc(alias = "background-rgba")]
    fn set_background_rgba(&self, background_rgba: Option<&gdk::RGBA>);

    #[doc(alias = "background-set")]
    fn is_background_set(&self) -> bool;

    #[doc(alias = "background-set")]
    fn set_background_set(&self, background_set: bool);

    fn xalign(&self) -> f32;

    fn set_xalign(&self, xalign: f32);

    fn xpad(&self) -> i32;

    fn set_xpad(&self, xpad: i32);

    fn yalign(&self) -> f32;

    fn set_yalign(&self, yalign: f32);

    fn ypad(&self) -> i32;

    fn set_ypad(&self, ypad: i32);

    #[doc(alias = "activate")]
    fn connect_activate<F: Fn(&Self, &gtk::TextIter, &gdk::Rectangle, &gdk::Event) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "query-activatable")]
    fn connect_query_activatable<
        F: Fn(&Self, &gtk::TextIter, &gdk::Rectangle, &gdk::Event) -> bool + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "query-data")]
    fn connect_query_data<
        F: Fn(&Self, &gtk::TextIter, &gtk::TextIter, GutterRendererState) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "query-tooltip")]
    fn connect_query_tooltip<
        F: Fn(&Self, &gtk::TextIter, &gdk::Rectangle, i32, i32, &gtk::Tooltip) -> bool + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "queue-draw")]
    fn connect_queue_draw<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "alignment-mode")]
    fn connect_alignment_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "background-rgba")]
    fn connect_background_rgba_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "background-set")]
    fn connect_background_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "size")]
    fn connect_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "view")]
    fn connect_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "visible")]
    fn connect_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "window-type")]
    fn connect_window_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "xalign")]
    fn connect_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "xpad")]
    fn connect_xpad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "yalign")]
    fn connect_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "ypad")]
    fn connect_ypad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GutterRenderer>> GutterRendererExt for O {
    fn activate(
        &self,
        iter: &mut gtk::TextIter,
        area: &mut gdk::Rectangle,
        event: &mut gdk::Event,
    ) {
        unsafe {
            ffi::gtk_source_gutter_renderer_activate(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
                area.to_glib_none_mut().0,
                event.to_glib_none_mut().0,
            );
        }
    }

    fn end(&self) {
        unsafe {
            ffi::gtk_source_gutter_renderer_end(self.as_ref().to_glib_none().0);
        }
    }

    fn alignment(&self) -> (f32, f32) {
        unsafe {
            let mut xalign = mem::MaybeUninit::uninit();
            let mut yalign = mem::MaybeUninit::uninit();
            ffi::gtk_source_gutter_renderer_get_alignment(
                self.as_ref().to_glib_none().0,
                xalign.as_mut_ptr(),
                yalign.as_mut_ptr(),
            );
            (xalign.assume_init(), yalign.assume_init())
        }
    }

    fn alignment_mode(&self) -> GutterRendererAlignmentMode {
        unsafe {
            from_glib(ffi::gtk_source_gutter_renderer_get_alignment_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn background(&self) -> Option<gdk::RGBA> {
        unsafe {
            let mut color = gdk::RGBA::uninitialized();
            let ret = from_glib(ffi::gtk_source_gutter_renderer_get_background(
                self.as_ref().to_glib_none().0,
                color.to_glib_none_mut().0,
            ));
            if ret {
                Some(color)
            } else {
                None
            }
        }
    }

    fn padding(&self) -> (i32, i32) {
        unsafe {
            let mut xpad = mem::MaybeUninit::uninit();
            let mut ypad = mem::MaybeUninit::uninit();
            ffi::gtk_source_gutter_renderer_get_padding(
                self.as_ref().to_glib_none().0,
                xpad.as_mut_ptr(),
                ypad.as_mut_ptr(),
            );
            (xpad.assume_init(), ypad.assume_init())
        }
    }

    fn size(&self) -> i32 {
        unsafe { ffi::gtk_source_gutter_renderer_get_size(self.as_ref().to_glib_none().0) }
    }

    fn view(&self) -> Option<gtk::TextView> {
        unsafe {
            from_glib_none(ffi::gtk_source_gutter_renderer_get_view(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_gutter_renderer_get_visible(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn window_type(&self) -> gtk::TextWindowType {
        unsafe {
            from_glib(ffi::gtk_source_gutter_renderer_get_window_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn query_activatable(
        &self,
        iter: &mut gtk::TextIter,
        area: &mut gdk::Rectangle,
        event: &mut gdk::Event,
    ) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_gutter_renderer_query_activatable(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
                area.to_glib_none_mut().0,
                event.to_glib_none_mut().0,
            ))
        }
    }

    fn query_data(
        &self,
        start: &mut gtk::TextIter,
        end: &mut gtk::TextIter,
        state: GutterRendererState,
    ) {
        unsafe {
            ffi::gtk_source_gutter_renderer_query_data(
                self.as_ref().to_glib_none().0,
                start.to_glib_none_mut().0,
                end.to_glib_none_mut().0,
                state.into_glib(),
            );
        }
    }

    fn query_tooltip(
        &self,
        iter: &mut gtk::TextIter,
        area: &mut gdk::Rectangle,
        x: i32,
        y: i32,
        tooltip: &gtk::Tooltip,
    ) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_gutter_renderer_query_tooltip(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
                area.to_glib_none_mut().0,
                x,
                y,
                tooltip.to_glib_none().0,
            ))
        }
    }

    fn queue_draw(&self) {
        unsafe {
            ffi::gtk_source_gutter_renderer_queue_draw(self.as_ref().to_glib_none().0);
        }
    }

    fn set_alignment(&self, xalign: f32, yalign: f32) {
        unsafe {
            ffi::gtk_source_gutter_renderer_set_alignment(
                self.as_ref().to_glib_none().0,
                xalign,
                yalign,
            );
        }
    }

    fn set_alignment_mode(&self, mode: GutterRendererAlignmentMode) {
        unsafe {
            ffi::gtk_source_gutter_renderer_set_alignment_mode(
                self.as_ref().to_glib_none().0,
                mode.into_glib(),
            );
        }
    }

    fn set_background(&self, color: Option<&gdk::RGBA>) {
        unsafe {
            ffi::gtk_source_gutter_renderer_set_background(
                self.as_ref().to_glib_none().0,
                color.to_glib_none().0,
            );
        }
    }

    fn set_padding(&self, xpad: i32, ypad: i32) {
        unsafe {
            ffi::gtk_source_gutter_renderer_set_padding(self.as_ref().to_glib_none().0, xpad, ypad);
        }
    }

    fn set_size(&self, size: i32) {
        unsafe {
            ffi::gtk_source_gutter_renderer_set_size(self.as_ref().to_glib_none().0, size);
        }
    }

    fn set_visible(&self, visible: bool) {
        unsafe {
            ffi::gtk_source_gutter_renderer_set_visible(
                self.as_ref().to_glib_none().0,
                visible.into_glib(),
            );
        }
    }

    fn background_rgba(&self) -> Option<gdk::RGBA> {
        glib::ObjectExt::property(self.as_ref(), "background-rgba")
    }

    fn set_background_rgba(&self, background_rgba: Option<&gdk::RGBA>) {
        glib::ObjectExt::set_property(self.as_ref(), "background-rgba", &background_rgba)
    }

    fn is_background_set(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "background-set")
    }

    fn set_background_set(&self, background_set: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "background-set", &background_set)
    }

    fn xalign(&self) -> f32 {
        glib::ObjectExt::property(self.as_ref(), "xalign")
    }

    fn set_xalign(&self, xalign: f32) {
        glib::ObjectExt::set_property(self.as_ref(), "xalign", &xalign)
    }

    fn xpad(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "xpad")
    }

    fn set_xpad(&self, xpad: i32) {
        glib::ObjectExt::set_property(self.as_ref(), "xpad", &xpad)
    }

    fn yalign(&self) -> f32 {
        glib::ObjectExt::property(self.as_ref(), "yalign")
    }

    fn set_yalign(&self, yalign: f32) {
        glib::ObjectExt::set_property(self.as_ref(), "yalign", &yalign)
    }

    fn ypad(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "ypad")
    }

    fn set_ypad(&self, ypad: i32) {
        glib::ObjectExt::set_property(self.as_ref(), "ypad", &ypad)
    }

    fn connect_activate<F: Fn(&Self, &gtk::TextIter, &gdk::Rectangle, &gdk::Event) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<
            P: IsA<GutterRenderer>,
            F: Fn(&P, &gtk::TextIter, &gdk::Rectangle, &gdk::Event) + 'static,
        >(
            this: *mut ffi::GtkSourceGutterRenderer,
            iter: *mut gtk::ffi::GtkTextIter,
            area: *mut gdk::ffi::GdkRectangle,
            event: *mut gdk::ffi::GdkEvent,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                GutterRenderer::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(iter),
                &from_glib_borrow(area),
                &from_glib_none(event),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    activate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_query_activatable<
        F: Fn(&Self, &gtk::TextIter, &gdk::Rectangle, &gdk::Event) -> bool + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn query_activatable_trampoline<
            P: IsA<GutterRenderer>,
            F: Fn(&P, &gtk::TextIter, &gdk::Rectangle, &gdk::Event) -> bool + 'static,
        >(
            this: *mut ffi::GtkSourceGutterRenderer,
            iter: *mut gtk::ffi::GtkTextIter,
            area: *mut gdk::ffi::GdkRectangle,
            event: *mut gdk::ffi::GdkEvent,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                GutterRenderer::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(iter),
                &from_glib_borrow(area),
                &from_glib_none(event),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"query-activatable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    query_activatable_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_query_data<
        F: Fn(&Self, &gtk::TextIter, &gtk::TextIter, GutterRendererState) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn query_data_trampoline<
            P: IsA<GutterRenderer>,
            F: Fn(&P, &gtk::TextIter, &gtk::TextIter, GutterRendererState) + 'static,
        >(
            this: *mut ffi::GtkSourceGutterRenderer,
            start: *mut gtk::ffi::GtkTextIter,
            end: *mut gtk::ffi::GtkTextIter,
            state: ffi::GtkSourceGutterRendererState,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                GutterRenderer::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(start),
                &from_glib_borrow(end),
                from_glib(state),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"query-data\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    query_data_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_query_tooltip<
        F: Fn(&Self, &gtk::TextIter, &gdk::Rectangle, i32, i32, &gtk::Tooltip) -> bool + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn query_tooltip_trampoline<
            P: IsA<GutterRenderer>,
            F: Fn(&P, &gtk::TextIter, &gdk::Rectangle, i32, i32, &gtk::Tooltip) -> bool + 'static,
        >(
            this: *mut ffi::GtkSourceGutterRenderer,
            iter: *mut gtk::ffi::GtkTextIter,
            area: *mut gdk::ffi::GdkRectangle,
            x: libc::c_int,
            y: libc::c_int,
            tooltip: *mut gtk::ffi::GtkTooltip,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                GutterRenderer::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(iter),
                &from_glib_borrow(area),
                x,
                y,
                &from_glib_borrow(tooltip),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"query-tooltip\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    query_tooltip_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_queue_draw<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn queue_draw_trampoline<P: IsA<GutterRenderer>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceGutterRenderer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GutterRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"queue-draw\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    queue_draw_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_alignment_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_alignment_mode_trampoline<
            P: IsA<GutterRenderer>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceGutterRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GutterRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::alignment-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_alignment_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_background_rgba_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_background_rgba_trampoline<
            P: IsA<GutterRenderer>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceGutterRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GutterRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::background-rgba\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_background_rgba_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_background_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_background_set_trampoline<
            P: IsA<GutterRenderer>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceGutterRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GutterRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::background-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_background_set_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_size_trampoline<P: IsA<GutterRenderer>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceGutterRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GutterRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_view_trampoline<P: IsA<GutterRenderer>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceGutterRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GutterRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::view\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_view_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_trampoline<
            P: IsA<GutterRenderer>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceGutterRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GutterRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::visible\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_visible_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_window_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_window_type_trampoline<
            P: IsA<GutterRenderer>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceGutterRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GutterRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::window-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_window_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_xalign_trampoline<
            P: IsA<GutterRenderer>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceGutterRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GutterRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::xalign\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_xalign_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_xpad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_xpad_trampoline<P: IsA<GutterRenderer>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceGutterRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GutterRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::xpad\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_xpad_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_yalign_trampoline<
            P: IsA<GutterRenderer>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceGutterRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GutterRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::yalign\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_yalign_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_ypad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ypad_trampoline<P: IsA<GutterRenderer>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceGutterRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GutterRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ypad\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ypad_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for GutterRenderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GutterRenderer")
    }
}
