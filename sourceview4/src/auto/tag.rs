// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkSourceTag")]
    pub struct Tag(Object<ffi::GtkSourceTag, ffi::GtkSourceTagClass>) @extends gtk::TextTag;

    match fn {
        type_ => || ffi::gtk_source_tag_get_type(),
    }
}

impl Tag {
    pub const NONE: Option<&'static Tag> = None;

    #[doc(alias = "gtk_source_tag_new")]
    pub fn new(name: Option<&str>) -> Tag {
        assert_initialized_main_thread!();
        unsafe {
            gtk::TextTag::from_glib_full(ffi::gtk_source_tag_new(name.to_glib_none().0))
                .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Tag`] objects.
    ///
    /// This method returns an instance of [`TagBuilder`](crate::builders::TagBuilder) which can be used to create [`Tag`] objects.
    pub fn builder() -> TagBuilder {
        TagBuilder::default()
    }
}

impl Default for Tag {
    fn default() -> Self {
        glib::object::Object::new::<Self>(&[])
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Tag`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct TagBuilder {
    draw_spaces: Option<bool>,
    draw_spaces_set: Option<bool>,
    accumulative_margin: Option<bool>,
    background: Option<String>,
    background_full_height: Option<bool>,
    background_full_height_set: Option<bool>,
    background_rgba: Option<gdk::RGBA>,
    background_set: Option<bool>,
    direction: Option<gtk::TextDirection>,
    editable: Option<bool>,
    editable_set: Option<bool>,
    fallback: Option<bool>,
    fallback_set: Option<bool>,
    family: Option<String>,
    family_set: Option<bool>,
    font: Option<String>,
    font_desc: Option<pango::FontDescription>,
    font_features: Option<String>,
    font_features_set: Option<bool>,
    foreground: Option<String>,
    foreground_rgba: Option<gdk::RGBA>,
    foreground_set: Option<bool>,
    indent: Option<i32>,
    indent_set: Option<bool>,
    invisible: Option<bool>,
    invisible_set: Option<bool>,
    justification: Option<gtk::Justification>,
    justification_set: Option<bool>,
    language: Option<String>,
    language_set: Option<bool>,
    left_margin: Option<i32>,
    left_margin_set: Option<bool>,
    letter_spacing: Option<i32>,
    letter_spacing_set: Option<bool>,
    name: Option<String>,
    paragraph_background: Option<String>,
    paragraph_background_rgba: Option<gdk::RGBA>,
    paragraph_background_set: Option<bool>,
    pixels_above_lines: Option<i32>,
    pixels_above_lines_set: Option<bool>,
    pixels_below_lines: Option<i32>,
    pixels_below_lines_set: Option<bool>,
    pixels_inside_wrap: Option<i32>,
    pixels_inside_wrap_set: Option<bool>,
    right_margin: Option<i32>,
    right_margin_set: Option<bool>,
    rise: Option<i32>,
    rise_set: Option<bool>,
    scale: Option<f64>,
    scale_set: Option<bool>,
    size: Option<i32>,
    size_points: Option<f64>,
    size_set: Option<bool>,
    stretch: Option<pango::Stretch>,
    stretch_set: Option<bool>,
    strikethrough: Option<bool>,
    strikethrough_rgba: Option<gdk::RGBA>,
    strikethrough_rgba_set: Option<bool>,
    strikethrough_set: Option<bool>,
    style: Option<pango::Style>,
    style_set: Option<bool>,
    tabs: Option<pango::TabArray>,
    tabs_set: Option<bool>,
    underline: Option<pango::Underline>,
    underline_rgba: Option<gdk::RGBA>,
    underline_rgba_set: Option<bool>,
    underline_set: Option<bool>,
    variant: Option<pango::Variant>,
    variant_set: Option<bool>,
    weight: Option<i32>,
    weight_set: Option<bool>,
    wrap_mode: Option<gtk::WrapMode>,
    wrap_mode_set: Option<bool>,
}

impl TagBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`TagBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Tag`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Tag {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref draw_spaces) = self.draw_spaces {
            properties.push(("draw-spaces", draw_spaces));
        }
        if let Some(ref draw_spaces_set) = self.draw_spaces_set {
            properties.push(("draw-spaces-set", draw_spaces_set));
        }
        if let Some(ref accumulative_margin) = self.accumulative_margin {
            properties.push(("accumulative-margin", accumulative_margin));
        }
        if let Some(ref background) = self.background {
            properties.push(("background", background));
        }
        if let Some(ref background_full_height) = self.background_full_height {
            properties.push(("background-full-height", background_full_height));
        }
        if let Some(ref background_full_height_set) = self.background_full_height_set {
            properties.push(("background-full-height-set", background_full_height_set));
        }
        if let Some(ref background_rgba) = self.background_rgba {
            properties.push(("background-rgba", background_rgba));
        }
        if let Some(ref background_set) = self.background_set {
            properties.push(("background-set", background_set));
        }
        if let Some(ref direction) = self.direction {
            properties.push(("direction", direction));
        }
        if let Some(ref editable) = self.editable {
            properties.push(("editable", editable));
        }
        if let Some(ref editable_set) = self.editable_set {
            properties.push(("editable-set", editable_set));
        }
        if let Some(ref fallback) = self.fallback {
            properties.push(("fallback", fallback));
        }
        if let Some(ref fallback_set) = self.fallback_set {
            properties.push(("fallback-set", fallback_set));
        }
        if let Some(ref family) = self.family {
            properties.push(("family", family));
        }
        if let Some(ref family_set) = self.family_set {
            properties.push(("family-set", family_set));
        }
        if let Some(ref font) = self.font {
            properties.push(("font", font));
        }
        if let Some(ref font_desc) = self.font_desc {
            properties.push(("font-desc", font_desc));
        }
        if let Some(ref font_features) = self.font_features {
            properties.push(("font-features", font_features));
        }
        if let Some(ref font_features_set) = self.font_features_set {
            properties.push(("font-features-set", font_features_set));
        }
        if let Some(ref foreground) = self.foreground {
            properties.push(("foreground", foreground));
        }
        if let Some(ref foreground_rgba) = self.foreground_rgba {
            properties.push(("foreground-rgba", foreground_rgba));
        }
        if let Some(ref foreground_set) = self.foreground_set {
            properties.push(("foreground-set", foreground_set));
        }
        if let Some(ref indent) = self.indent {
            properties.push(("indent", indent));
        }
        if let Some(ref indent_set) = self.indent_set {
            properties.push(("indent-set", indent_set));
        }
        if let Some(ref invisible) = self.invisible {
            properties.push(("invisible", invisible));
        }
        if let Some(ref invisible_set) = self.invisible_set {
            properties.push(("invisible-set", invisible_set));
        }
        if let Some(ref justification) = self.justification {
            properties.push(("justification", justification));
        }
        if let Some(ref justification_set) = self.justification_set {
            properties.push(("justification-set", justification_set));
        }
        if let Some(ref language) = self.language {
            properties.push(("language", language));
        }
        if let Some(ref language_set) = self.language_set {
            properties.push(("language-set", language_set));
        }
        if let Some(ref left_margin) = self.left_margin {
            properties.push(("left-margin", left_margin));
        }
        if let Some(ref left_margin_set) = self.left_margin_set {
            properties.push(("left-margin-set", left_margin_set));
        }
        if let Some(ref letter_spacing) = self.letter_spacing {
            properties.push(("letter-spacing", letter_spacing));
        }
        if let Some(ref letter_spacing_set) = self.letter_spacing_set {
            properties.push(("letter-spacing-set", letter_spacing_set));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref paragraph_background) = self.paragraph_background {
            properties.push(("paragraph-background", paragraph_background));
        }
        if let Some(ref paragraph_background_rgba) = self.paragraph_background_rgba {
            properties.push(("paragraph-background-rgba", paragraph_background_rgba));
        }
        if let Some(ref paragraph_background_set) = self.paragraph_background_set {
            properties.push(("paragraph-background-set", paragraph_background_set));
        }
        if let Some(ref pixels_above_lines) = self.pixels_above_lines {
            properties.push(("pixels-above-lines", pixels_above_lines));
        }
        if let Some(ref pixels_above_lines_set) = self.pixels_above_lines_set {
            properties.push(("pixels-above-lines-set", pixels_above_lines_set));
        }
        if let Some(ref pixels_below_lines) = self.pixels_below_lines {
            properties.push(("pixels-below-lines", pixels_below_lines));
        }
        if let Some(ref pixels_below_lines_set) = self.pixels_below_lines_set {
            properties.push(("pixels-below-lines-set", pixels_below_lines_set));
        }
        if let Some(ref pixels_inside_wrap) = self.pixels_inside_wrap {
            properties.push(("pixels-inside-wrap", pixels_inside_wrap));
        }
        if let Some(ref pixels_inside_wrap_set) = self.pixels_inside_wrap_set {
            properties.push(("pixels-inside-wrap-set", pixels_inside_wrap_set));
        }
        if let Some(ref right_margin) = self.right_margin {
            properties.push(("right-margin", right_margin));
        }
        if let Some(ref right_margin_set) = self.right_margin_set {
            properties.push(("right-margin-set", right_margin_set));
        }
        if let Some(ref rise) = self.rise {
            properties.push(("rise", rise));
        }
        if let Some(ref rise_set) = self.rise_set {
            properties.push(("rise-set", rise_set));
        }
        if let Some(ref scale) = self.scale {
            properties.push(("scale", scale));
        }
        if let Some(ref scale_set) = self.scale_set {
            properties.push(("scale-set", scale_set));
        }
        if let Some(ref size) = self.size {
            properties.push(("size", size));
        }
        if let Some(ref size_points) = self.size_points {
            properties.push(("size-points", size_points));
        }
        if let Some(ref size_set) = self.size_set {
            properties.push(("size-set", size_set));
        }
        if let Some(ref stretch) = self.stretch {
            properties.push(("stretch", stretch));
        }
        if let Some(ref stretch_set) = self.stretch_set {
            properties.push(("stretch-set", stretch_set));
        }
        if let Some(ref strikethrough) = self.strikethrough {
            properties.push(("strikethrough", strikethrough));
        }
        if let Some(ref strikethrough_rgba) = self.strikethrough_rgba {
            properties.push(("strikethrough-rgba", strikethrough_rgba));
        }
        if let Some(ref strikethrough_rgba_set) = self.strikethrough_rgba_set {
            properties.push(("strikethrough-rgba-set", strikethrough_rgba_set));
        }
        if let Some(ref strikethrough_set) = self.strikethrough_set {
            properties.push(("strikethrough-set", strikethrough_set));
        }
        if let Some(ref style) = self.style {
            properties.push(("style", style));
        }
        if let Some(ref style_set) = self.style_set {
            properties.push(("style-set", style_set));
        }
        if let Some(ref tabs) = self.tabs {
            properties.push(("tabs", tabs));
        }
        if let Some(ref tabs_set) = self.tabs_set {
            properties.push(("tabs-set", tabs_set));
        }
        if let Some(ref underline) = self.underline {
            properties.push(("underline", underline));
        }
        if let Some(ref underline_rgba) = self.underline_rgba {
            properties.push(("underline-rgba", underline_rgba));
        }
        if let Some(ref underline_rgba_set) = self.underline_rgba_set {
            properties.push(("underline-rgba-set", underline_rgba_set));
        }
        if let Some(ref underline_set) = self.underline_set {
            properties.push(("underline-set", underline_set));
        }
        if let Some(ref variant) = self.variant {
            properties.push(("variant", variant));
        }
        if let Some(ref variant_set) = self.variant_set {
            properties.push(("variant-set", variant_set));
        }
        if let Some(ref weight) = self.weight {
            properties.push(("weight", weight));
        }
        if let Some(ref weight_set) = self.weight_set {
            properties.push(("weight-set", weight_set));
        }
        if let Some(ref wrap_mode) = self.wrap_mode {
            properties.push(("wrap-mode", wrap_mode));
        }
        if let Some(ref wrap_mode_set) = self.wrap_mode_set {
            properties.push(("wrap-mode-set", wrap_mode_set));
        }
        glib::Object::new::<Tag>(&properties)
    }

    pub fn draw_spaces(mut self, draw_spaces: bool) -> Self {
        self.draw_spaces = Some(draw_spaces);
        self
    }

    pub fn draw_spaces_set(mut self, draw_spaces_set: bool) -> Self {
        self.draw_spaces_set = Some(draw_spaces_set);
        self
    }

    pub fn accumulative_margin(mut self, accumulative_margin: bool) -> Self {
        self.accumulative_margin = Some(accumulative_margin);
        self
    }

    pub fn background(mut self, background: &str) -> Self {
        self.background = Some(background.to_string());
        self
    }

    pub fn background_full_height(mut self, background_full_height: bool) -> Self {
        self.background_full_height = Some(background_full_height);
        self
    }

    pub fn background_full_height_set(mut self, background_full_height_set: bool) -> Self {
        self.background_full_height_set = Some(background_full_height_set);
        self
    }

    pub fn background_rgba(mut self, background_rgba: &gdk::RGBA) -> Self {
        self.background_rgba = Some(background_rgba.clone());
        self
    }

    pub fn background_set(mut self, background_set: bool) -> Self {
        self.background_set = Some(background_set);
        self
    }

    pub fn direction(mut self, direction: gtk::TextDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn editable(mut self, editable: bool) -> Self {
        self.editable = Some(editable);
        self
    }

    pub fn editable_set(mut self, editable_set: bool) -> Self {
        self.editable_set = Some(editable_set);
        self
    }

    pub fn fallback(mut self, fallback: bool) -> Self {
        self.fallback = Some(fallback);
        self
    }

    pub fn fallback_set(mut self, fallback_set: bool) -> Self {
        self.fallback_set = Some(fallback_set);
        self
    }

    pub fn family(mut self, family: &str) -> Self {
        self.family = Some(family.to_string());
        self
    }

    pub fn family_set(mut self, family_set: bool) -> Self {
        self.family_set = Some(family_set);
        self
    }

    pub fn font(mut self, font: &str) -> Self {
        self.font = Some(font.to_string());
        self
    }

    pub fn font_desc(mut self, font_desc: &pango::FontDescription) -> Self {
        self.font_desc = Some(font_desc.clone());
        self
    }

    pub fn font_features(mut self, font_features: &str) -> Self {
        self.font_features = Some(font_features.to_string());
        self
    }

    pub fn font_features_set(mut self, font_features_set: bool) -> Self {
        self.font_features_set = Some(font_features_set);
        self
    }

    pub fn foreground(mut self, foreground: &str) -> Self {
        self.foreground = Some(foreground.to_string());
        self
    }

    pub fn foreground_rgba(mut self, foreground_rgba: &gdk::RGBA) -> Self {
        self.foreground_rgba = Some(foreground_rgba.clone());
        self
    }

    pub fn foreground_set(mut self, foreground_set: bool) -> Self {
        self.foreground_set = Some(foreground_set);
        self
    }

    pub fn indent(mut self, indent: i32) -> Self {
        self.indent = Some(indent);
        self
    }

    pub fn indent_set(mut self, indent_set: bool) -> Self {
        self.indent_set = Some(indent_set);
        self
    }

    pub fn invisible(mut self, invisible: bool) -> Self {
        self.invisible = Some(invisible);
        self
    }

    pub fn invisible_set(mut self, invisible_set: bool) -> Self {
        self.invisible_set = Some(invisible_set);
        self
    }

    pub fn justification(mut self, justification: gtk::Justification) -> Self {
        self.justification = Some(justification);
        self
    }

    pub fn justification_set(mut self, justification_set: bool) -> Self {
        self.justification_set = Some(justification_set);
        self
    }

    pub fn language(mut self, language: &str) -> Self {
        self.language = Some(language.to_string());
        self
    }

    pub fn language_set(mut self, language_set: bool) -> Self {
        self.language_set = Some(language_set);
        self
    }

    pub fn left_margin(mut self, left_margin: i32) -> Self {
        self.left_margin = Some(left_margin);
        self
    }

    pub fn left_margin_set(mut self, left_margin_set: bool) -> Self {
        self.left_margin_set = Some(left_margin_set);
        self
    }

    pub fn letter_spacing(mut self, letter_spacing: i32) -> Self {
        self.letter_spacing = Some(letter_spacing);
        self
    }

    pub fn letter_spacing_set(mut self, letter_spacing_set: bool) -> Self {
        self.letter_spacing_set = Some(letter_spacing_set);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn paragraph_background(mut self, paragraph_background: &str) -> Self {
        self.paragraph_background = Some(paragraph_background.to_string());
        self
    }

    pub fn paragraph_background_rgba(mut self, paragraph_background_rgba: &gdk::RGBA) -> Self {
        self.paragraph_background_rgba = Some(paragraph_background_rgba.clone());
        self
    }

    pub fn paragraph_background_set(mut self, paragraph_background_set: bool) -> Self {
        self.paragraph_background_set = Some(paragraph_background_set);
        self
    }

    pub fn pixels_above_lines(mut self, pixels_above_lines: i32) -> Self {
        self.pixels_above_lines = Some(pixels_above_lines);
        self
    }

    pub fn pixels_above_lines_set(mut self, pixels_above_lines_set: bool) -> Self {
        self.pixels_above_lines_set = Some(pixels_above_lines_set);
        self
    }

    pub fn pixels_below_lines(mut self, pixels_below_lines: i32) -> Self {
        self.pixels_below_lines = Some(pixels_below_lines);
        self
    }

    pub fn pixels_below_lines_set(mut self, pixels_below_lines_set: bool) -> Self {
        self.pixels_below_lines_set = Some(pixels_below_lines_set);
        self
    }

    pub fn pixels_inside_wrap(mut self, pixels_inside_wrap: i32) -> Self {
        self.pixels_inside_wrap = Some(pixels_inside_wrap);
        self
    }

    pub fn pixels_inside_wrap_set(mut self, pixels_inside_wrap_set: bool) -> Self {
        self.pixels_inside_wrap_set = Some(pixels_inside_wrap_set);
        self
    }

    pub fn right_margin(mut self, right_margin: i32) -> Self {
        self.right_margin = Some(right_margin);
        self
    }

    pub fn right_margin_set(mut self, right_margin_set: bool) -> Self {
        self.right_margin_set = Some(right_margin_set);
        self
    }

    pub fn rise(mut self, rise: i32) -> Self {
        self.rise = Some(rise);
        self
    }

    pub fn rise_set(mut self, rise_set: bool) -> Self {
        self.rise_set = Some(rise_set);
        self
    }

    pub fn scale(mut self, scale: f64) -> Self {
        self.scale = Some(scale);
        self
    }

    pub fn scale_set(mut self, scale_set: bool) -> Self {
        self.scale_set = Some(scale_set);
        self
    }

    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn size_points(mut self, size_points: f64) -> Self {
        self.size_points = Some(size_points);
        self
    }

    pub fn size_set(mut self, size_set: bool) -> Self {
        self.size_set = Some(size_set);
        self
    }

    pub fn stretch(mut self, stretch: pango::Stretch) -> Self {
        self.stretch = Some(stretch);
        self
    }

    pub fn stretch_set(mut self, stretch_set: bool) -> Self {
        self.stretch_set = Some(stretch_set);
        self
    }

    pub fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = Some(strikethrough);
        self
    }

    pub fn strikethrough_rgba(mut self, strikethrough_rgba: &gdk::RGBA) -> Self {
        self.strikethrough_rgba = Some(strikethrough_rgba.clone());
        self
    }

    pub fn strikethrough_rgba_set(mut self, strikethrough_rgba_set: bool) -> Self {
        self.strikethrough_rgba_set = Some(strikethrough_rgba_set);
        self
    }

    pub fn strikethrough_set(mut self, strikethrough_set: bool) -> Self {
        self.strikethrough_set = Some(strikethrough_set);
        self
    }

    pub fn style(mut self, style: pango::Style) -> Self {
        self.style = Some(style);
        self
    }

    pub fn style_set(mut self, style_set: bool) -> Self {
        self.style_set = Some(style_set);
        self
    }

    pub fn tabs(mut self, tabs: &pango::TabArray) -> Self {
        self.tabs = Some(tabs.clone());
        self
    }

    pub fn tabs_set(mut self, tabs_set: bool) -> Self {
        self.tabs_set = Some(tabs_set);
        self
    }

    pub fn underline(mut self, underline: pango::Underline) -> Self {
        self.underline = Some(underline);
        self
    }

    pub fn underline_rgba(mut self, underline_rgba: &gdk::RGBA) -> Self {
        self.underline_rgba = Some(underline_rgba.clone());
        self
    }

    pub fn underline_rgba_set(mut self, underline_rgba_set: bool) -> Self {
        self.underline_rgba_set = Some(underline_rgba_set);
        self
    }

    pub fn underline_set(mut self, underline_set: bool) -> Self {
        self.underline_set = Some(underline_set);
        self
    }

    pub fn variant(mut self, variant: pango::Variant) -> Self {
        self.variant = Some(variant);
        self
    }

    pub fn variant_set(mut self, variant_set: bool) -> Self {
        self.variant_set = Some(variant_set);
        self
    }

    pub fn weight(mut self, weight: i32) -> Self {
        self.weight = Some(weight);
        self
    }

    pub fn weight_set(mut self, weight_set: bool) -> Self {
        self.weight_set = Some(weight_set);
        self
    }

    pub fn wrap_mode(mut self, wrap_mode: gtk::WrapMode) -> Self {
        self.wrap_mode = Some(wrap_mode);
        self
    }

    pub fn wrap_mode_set(mut self, wrap_mode_set: bool) -> Self {
        self.wrap_mode_set = Some(wrap_mode_set);
        self
    }
}

pub trait TagExt: 'static {
    #[doc(alias = "draw-spaces")]
    fn draws_spaces(&self) -> bool;

    #[doc(alias = "draw-spaces")]
    fn set_draw_spaces(&self, draw_spaces: bool);

    #[doc(alias = "draw-spaces-set")]
    fn draws_spaces_set(&self) -> bool;

    #[doc(alias = "draw-spaces-set")]
    fn set_draw_spaces_set(&self, draw_spaces_set: bool);

    #[doc(alias = "draw-spaces")]
    fn connect_draw_spaces_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "draw-spaces-set")]
    fn connect_draw_spaces_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Tag>> TagExt for O {
    fn draws_spaces(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "draw-spaces")
    }

    fn set_draw_spaces(&self, draw_spaces: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "draw-spaces", &draw_spaces)
    }

    fn draws_spaces_set(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "draw-spaces-set")
    }

    fn set_draw_spaces_set(&self, draw_spaces_set: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "draw-spaces-set", &draw_spaces_set)
    }

    fn connect_draw_spaces_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_draw_spaces_trampoline<P: IsA<Tag>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceTag,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Tag::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::draw-spaces\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_draw_spaces_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_draw_spaces_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_draw_spaces_set_trampoline<P: IsA<Tag>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceTag,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Tag::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::draw-spaces-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_draw_spaces_set_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Tag")
    }
}
