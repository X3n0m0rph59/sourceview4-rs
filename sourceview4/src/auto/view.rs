// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use gdk_sys;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk;
use gtk_source_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use BackgroundPatternType;
use Buffer;
use ChangeCaseType;
use Completion;
use Gutter;
use MarkAttributes;
use SmartHomeEndType;
use SpaceDrawer;

glib_wrapper! {
    pub struct View(Object<gtk_source_sys::GtkSourceView, gtk_source_sys::GtkSourceViewClass, ViewClass>) @extends gtk::TextView, gtk::Container, gtk::Widget;

    match fn {
        get_type => || gtk_source_sys::gtk_source_view_get_type(),
    }
}

impl View {
    pub fn new() -> View {
        assert_initialized_main_thread!();
        unsafe {
            gtk::Widget::from_glib_none(gtk_source_sys::gtk_source_view_new()).unsafe_cast()
        }
    }

    pub fn new_with_buffer<P: IsA<Buffer>>(buffer: &P) -> View {
        skip_assert_initialized!();
        unsafe {
            gtk::Widget::from_glib_none(gtk_source_sys::gtk_source_view_new_with_buffer(buffer.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }
}

impl Default for View {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_VIEW: Option<&View> = None;

pub trait ViewExt: 'static {
    fn get_auto_indent(&self) -> bool;

    fn get_background_pattern(&self) -> BackgroundPatternType;

    fn get_completion(&self) -> Option<Completion>;

    fn get_gutter(&self, window_type: gtk::TextWindowType) -> Option<Gutter>;

    fn get_highlight_current_line(&self) -> bool;

    fn get_indent_on_tab(&self) -> bool;

    fn get_indent_width(&self) -> i32;

    fn get_insert_spaces_instead_of_tabs(&self) -> bool;

    fn get_right_margin_position(&self) -> u32;

    fn get_show_line_marks(&self) -> bool;

    fn get_show_line_numbers(&self) -> bool;

    fn get_show_right_margin(&self) -> bool;

    fn get_smart_backspace(&self) -> bool;

    fn get_smart_home_end(&self) -> SmartHomeEndType;

    fn get_space_drawer(&self) -> Option<SpaceDrawer>;

    fn get_tab_width(&self) -> u32;

    fn get_visual_column(&self, iter: &gtk::TextIter) -> u32;

    fn indent_lines(&self, start: &mut gtk::TextIter, end: &mut gtk::TextIter);

    fn set_auto_indent(&self, enable: bool);

    fn set_background_pattern(&self, background_pattern: BackgroundPatternType);

    fn set_highlight_current_line(&self, highlight: bool);

    fn set_indent_on_tab(&self, enable: bool);

    fn set_indent_width(&self, width: i32);

    fn set_insert_spaces_instead_of_tabs(&self, enable: bool);

    fn set_mark_attributes<P: IsA<MarkAttributes>>(&self, category: &str, attributes: &P, priority: i32);

    fn set_right_margin_position(&self, pos: u32);

    fn set_show_line_marks(&self, show: bool);

    fn set_show_line_numbers(&self, show: bool);

    fn set_show_right_margin(&self, show: bool);

    fn set_smart_backspace(&self, smart_backspace: bool);

    fn set_smart_home_end(&self, smart_home_end: SmartHomeEndType);

    fn set_tab_width(&self, width: u32);

    fn unindent_lines(&self, start: &mut gtk::TextIter, end: &mut gtk::TextIter);

    fn connect_change_case<F: Fn(&Self, ChangeCaseType) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_change_case(&self, case_type: ChangeCaseType);

    fn connect_change_number<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_change_number(&self, count: i32);

    fn connect_join_lines<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_join_lines(&self);

    fn connect_line_mark_activated<F: Fn(&Self, &gtk::TextIter, &gdk::Event) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_move_lines<F: Fn(&Self, bool) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_move_lines(&self, down: bool);

    fn connect_move_to_matching_bracket<F: Fn(&Self, bool) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_move_to_matching_bracket(&self, extend_selection: bool);

    fn connect_move_words<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_move_words(&self, count: i32);

    fn connect_redo<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_redo(&self);

    fn connect_show_completion<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_show_completion(&self);

    fn connect_smart_home_end<F: Fn(&Self, &gtk::TextIter, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_undo<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_undo(&self);

    fn connect_property_auto_indent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_background_pattern_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_completion_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_highlight_current_line_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_indent_on_tab_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_indent_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_insert_spaces_instead_of_tabs_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_right_margin_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_line_marks_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_line_numbers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_right_margin_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_smart_backspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_smart_home_end_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_space_drawer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tab_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<View>> ViewExt for O {
    fn get_auto_indent(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_view_get_auto_indent(self.as_ref().to_glib_none().0))
        }
    }

    fn get_background_pattern(&self) -> BackgroundPatternType {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_view_get_background_pattern(self.as_ref().to_glib_none().0))
        }
    }

    fn get_completion(&self) -> Option<Completion> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_view_get_completion(self.as_ref().to_glib_none().0))
        }
    }

    fn get_gutter(&self, window_type: gtk::TextWindowType) -> Option<Gutter> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_view_get_gutter(self.as_ref().to_glib_none().0, window_type.to_glib()))
        }
    }

    fn get_highlight_current_line(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_view_get_highlight_current_line(self.as_ref().to_glib_none().0))
        }
    }

    fn get_indent_on_tab(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_view_get_indent_on_tab(self.as_ref().to_glib_none().0))
        }
    }

    fn get_indent_width(&self) -> i32 {
        unsafe {
            gtk_source_sys::gtk_source_view_get_indent_width(self.as_ref().to_glib_none().0)
        }
    }

    fn get_insert_spaces_instead_of_tabs(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_view_get_insert_spaces_instead_of_tabs(self.as_ref().to_glib_none().0))
        }
    }

    fn get_right_margin_position(&self) -> u32 {
        unsafe {
            gtk_source_sys::gtk_source_view_get_right_margin_position(self.as_ref().to_glib_none().0)
        }
    }

    fn get_show_line_marks(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_view_get_show_line_marks(self.as_ref().to_glib_none().0))
        }
    }

    fn get_show_line_numbers(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_view_get_show_line_numbers(self.as_ref().to_glib_none().0))
        }
    }

    fn get_show_right_margin(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_view_get_show_right_margin(self.as_ref().to_glib_none().0))
        }
    }

    fn get_smart_backspace(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_view_get_smart_backspace(self.as_ref().to_glib_none().0))
        }
    }

    fn get_smart_home_end(&self) -> SmartHomeEndType {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_view_get_smart_home_end(self.as_ref().to_glib_none().0))
        }
    }

    fn get_space_drawer(&self) -> Option<SpaceDrawer> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_view_get_space_drawer(self.as_ref().to_glib_none().0))
        }
    }

    fn get_tab_width(&self) -> u32 {
        unsafe {
            gtk_source_sys::gtk_source_view_get_tab_width(self.as_ref().to_glib_none().0)
        }
    }

    fn get_visual_column(&self, iter: &gtk::TextIter) -> u32 {
        unsafe {
            gtk_source_sys::gtk_source_view_get_visual_column(self.as_ref().to_glib_none().0, iter.to_glib_none().0)
        }
    }

    fn indent_lines(&self, start: &mut gtk::TextIter, end: &mut gtk::TextIter) {
        unsafe {
            gtk_source_sys::gtk_source_view_indent_lines(self.as_ref().to_glib_none().0, start.to_glib_none_mut().0, end.to_glib_none_mut().0);
        }
    }

    fn set_auto_indent(&self, enable: bool) {
        unsafe {
            gtk_source_sys::gtk_source_view_set_auto_indent(self.as_ref().to_glib_none().0, enable.to_glib());
        }
    }

    fn set_background_pattern(&self, background_pattern: BackgroundPatternType) {
        unsafe {
            gtk_source_sys::gtk_source_view_set_background_pattern(self.as_ref().to_glib_none().0, background_pattern.to_glib());
        }
    }

    fn set_highlight_current_line(&self, highlight: bool) {
        unsafe {
            gtk_source_sys::gtk_source_view_set_highlight_current_line(self.as_ref().to_glib_none().0, highlight.to_glib());
        }
    }

    fn set_indent_on_tab(&self, enable: bool) {
        unsafe {
            gtk_source_sys::gtk_source_view_set_indent_on_tab(self.as_ref().to_glib_none().0, enable.to_glib());
        }
    }

    fn set_indent_width(&self, width: i32) {
        unsafe {
            gtk_source_sys::gtk_source_view_set_indent_width(self.as_ref().to_glib_none().0, width);
        }
    }

    fn set_insert_spaces_instead_of_tabs(&self, enable: bool) {
        unsafe {
            gtk_source_sys::gtk_source_view_set_insert_spaces_instead_of_tabs(self.as_ref().to_glib_none().0, enable.to_glib());
        }
    }

    fn set_mark_attributes<P: IsA<MarkAttributes>>(&self, category: &str, attributes: &P, priority: i32) {
        unsafe {
            gtk_source_sys::gtk_source_view_set_mark_attributes(self.as_ref().to_glib_none().0, category.to_glib_none().0, attributes.as_ref().to_glib_none().0, priority);
        }
    }

    fn set_right_margin_position(&self, pos: u32) {
        unsafe {
            gtk_source_sys::gtk_source_view_set_right_margin_position(self.as_ref().to_glib_none().0, pos);
        }
    }

    fn set_show_line_marks(&self, show: bool) {
        unsafe {
            gtk_source_sys::gtk_source_view_set_show_line_marks(self.as_ref().to_glib_none().0, show.to_glib());
        }
    }

    fn set_show_line_numbers(&self, show: bool) {
        unsafe {
            gtk_source_sys::gtk_source_view_set_show_line_numbers(self.as_ref().to_glib_none().0, show.to_glib());
        }
    }

    fn set_show_right_margin(&self, show: bool) {
        unsafe {
            gtk_source_sys::gtk_source_view_set_show_right_margin(self.as_ref().to_glib_none().0, show.to_glib());
        }
    }

    fn set_smart_backspace(&self, smart_backspace: bool) {
        unsafe {
            gtk_source_sys::gtk_source_view_set_smart_backspace(self.as_ref().to_glib_none().0, smart_backspace.to_glib());
        }
    }

    fn set_smart_home_end(&self, smart_home_end: SmartHomeEndType) {
        unsafe {
            gtk_source_sys::gtk_source_view_set_smart_home_end(self.as_ref().to_glib_none().0, smart_home_end.to_glib());
        }
    }

    fn set_tab_width(&self, width: u32) {
        unsafe {
            gtk_source_sys::gtk_source_view_set_tab_width(self.as_ref().to_glib_none().0, width);
        }
    }

    fn unindent_lines(&self, start: &mut gtk::TextIter, end: &mut gtk::TextIter) {
        unsafe {
            gtk_source_sys::gtk_source_view_unindent_lines(self.as_ref().to_glib_none().0, start.to_glib_none_mut().0, end.to_glib_none_mut().0);
        }
    }

    fn connect_change_case<F: Fn(&Self, ChangeCaseType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn change_case_trampoline<P, F: Fn(&P, ChangeCaseType) + 'static>(this: *mut gtk_source_sys::GtkSourceView, case_type: gtk_source_sys::GtkSourceChangeCaseType, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast(), from_glib(case_type))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"change-case\0".as_ptr() as *const _,
                Some(transmute(change_case_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_change_case(&self, case_type: ChangeCaseType) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("change-case", &[&case_type]).unwrap() };
    }

    fn connect_change_number<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn change_number_trampoline<P, F: Fn(&P, i32) + 'static>(this: *mut gtk_source_sys::GtkSourceView, count: libc::c_int, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast(), count)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"change-number\0".as_ptr() as *const _,
                Some(transmute(change_number_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_change_number(&self, count: i32) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("change-number", &[&count]).unwrap() };
    }

    fn connect_join_lines<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn join_lines_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceView, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"join-lines\0".as_ptr() as *const _,
                Some(transmute(join_lines_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_join_lines(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("join-lines", &[]).unwrap() };
    }

    fn connect_line_mark_activated<F: Fn(&Self, &gtk::TextIter, &gdk::Event) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn line_mark_activated_trampoline<P, F: Fn(&P, &gtk::TextIter, &gdk::Event) + 'static>(this: *mut gtk_source_sys::GtkSourceView, iter: *mut gtk_sys::GtkTextIter, event: *mut gdk_sys::GdkEvent, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(iter), &from_glib_none(event))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"line-mark-activated\0".as_ptr() as *const _,
                Some(transmute(line_mark_activated_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_move_lines<F: Fn(&Self, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn move_lines_trampoline<P, F: Fn(&P, bool) + 'static>(this: *mut gtk_source_sys::GtkSourceView, down: glib_sys::gboolean, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast(), from_glib(down))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"move-lines\0".as_ptr() as *const _,
                Some(transmute(move_lines_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_move_lines(&self, down: bool) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("move-lines", &[&down]).unwrap() };
    }

    fn connect_move_to_matching_bracket<F: Fn(&Self, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn move_to_matching_bracket_trampoline<P, F: Fn(&P, bool) + 'static>(this: *mut gtk_source_sys::GtkSourceView, extend_selection: glib_sys::gboolean, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast(), from_glib(extend_selection))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"move-to-matching-bracket\0".as_ptr() as *const _,
                Some(transmute(move_to_matching_bracket_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_move_to_matching_bracket(&self, extend_selection: bool) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("move-to-matching-bracket", &[&extend_selection]).unwrap() };
    }

    fn connect_move_words<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn move_words_trampoline<P, F: Fn(&P, i32) + 'static>(this: *mut gtk_source_sys::GtkSourceView, count: libc::c_int, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast(), count)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"move-words\0".as_ptr() as *const _,
                Some(transmute(move_words_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_move_words(&self, count: i32) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("move-words", &[&count]).unwrap() };
    }

    fn connect_redo<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn redo_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceView, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"redo\0".as_ptr() as *const _,
                Some(transmute(redo_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_redo(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("redo", &[]).unwrap() };
    }

    fn connect_show_completion<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn show_completion_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceView, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"show-completion\0".as_ptr() as *const _,
                Some(transmute(show_completion_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_show_completion(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("show-completion", &[]).unwrap() };
    }

    fn connect_smart_home_end<F: Fn(&Self, &gtk::TextIter, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn smart_home_end_trampoline<P, F: Fn(&P, &gtk::TextIter, i32) + 'static>(this: *mut gtk_source_sys::GtkSourceView, iter: *mut gtk_sys::GtkTextIter, count: libc::c_int, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(iter), count)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"smart-home-end\0".as_ptr() as *const _,
                Some(transmute(smart_home_end_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_undo<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn undo_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceView, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"undo\0".as_ptr() as *const _,
                Some(transmute(undo_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_undo(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("undo", &[]).unwrap() };
    }

    fn connect_property_auto_indent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_auto_indent_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::auto-indent\0".as_ptr() as *const _,
                Some(transmute(notify_auto_indent_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_background_pattern_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_background_pattern_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::background-pattern\0".as_ptr() as *const _,
                Some(transmute(notify_background_pattern_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_completion_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_completion_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::completion\0".as_ptr() as *const _,
                Some(transmute(notify_completion_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_highlight_current_line_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_highlight_current_line_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::highlight-current-line\0".as_ptr() as *const _,
                Some(transmute(notify_highlight_current_line_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_indent_on_tab_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_indent_on_tab_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::indent-on-tab\0".as_ptr() as *const _,
                Some(transmute(notify_indent_on_tab_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_indent_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_indent_width_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::indent-width\0".as_ptr() as *const _,
                Some(transmute(notify_indent_width_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_insert_spaces_instead_of_tabs_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_insert_spaces_instead_of_tabs_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::insert-spaces-instead-of-tabs\0".as_ptr() as *const _,
                Some(transmute(notify_insert_spaces_instead_of_tabs_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_right_margin_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_right_margin_position_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::right-margin-position\0".as_ptr() as *const _,
                Some(transmute(notify_right_margin_position_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_line_marks_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_line_marks_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-line-marks\0".as_ptr() as *const _,
                Some(transmute(notify_show_line_marks_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_line_numbers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_line_numbers_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-line-numbers\0".as_ptr() as *const _,
                Some(transmute(notify_show_line_numbers_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_right_margin_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_right_margin_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-right-margin\0".as_ptr() as *const _,
                Some(transmute(notify_show_right_margin_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_smart_backspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_smart_backspace_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::smart-backspace\0".as_ptr() as *const _,
                Some(transmute(notify_smart_backspace_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_smart_home_end_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_smart_home_end_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::smart-home-end\0".as_ptr() as *const _,
                Some(transmute(notify_smart_home_end_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_space_drawer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_space_drawer_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::space-drawer\0".as_ptr() as *const _,
                Some(transmute(notify_space_drawer_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_tab_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tab_width_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<View>
        {
            let f: &F = &*(f as *const F);
            f(&View::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::tab-width\0".as_ptr() as *const _,
                Some(transmute(notify_tab_width_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for View {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "View")
    }
}
