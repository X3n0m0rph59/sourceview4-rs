// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkSourceStyle")]
    pub struct Style(Object<ffi::GtkSourceStyle, ffi::GtkSourceStyleClass>);

    match fn {
        type_ => || ffi::gtk_source_style_get_type(),
    }
}

impl Style {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Style`] objects.
    ///
    /// This method returns an instance of [`StyleBuilder`] which can be used to create [`Style`] objects.
    pub fn builder() -> StyleBuilder {
        StyleBuilder::default()
    }

    #[doc(alias = "gtk_source_style_apply")]
    pub fn apply(&self, tag: &impl IsA<gtk::TextTag>) {
        unsafe {
            ffi::gtk_source_style_apply(self.to_glib_none().0, tag.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_source_style_copy")]
    pub fn copy(&self) -> Option<Style> {
        unsafe { from_glib_full(ffi::gtk_source_style_copy(self.to_glib_none().0)) }
    }

    pub fn background(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self, "background")
    }

    #[doc(alias = "background-set")]
    pub fn is_background_set(&self) -> bool {
        glib::ObjectExt::property(self, "background-set")
    }

    pub fn is_bold(&self) -> bool {
        glib::ObjectExt::property(self, "bold")
    }

    #[doc(alias = "bold-set")]
    pub fn is_bold_set(&self) -> bool {
        glib::ObjectExt::property(self, "bold-set")
    }

    pub fn foreground(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self, "foreground")
    }

    #[doc(alias = "foreground-set")]
    pub fn is_foreground_set(&self) -> bool {
        glib::ObjectExt::property(self, "foreground-set")
    }

    pub fn is_italic(&self) -> bool {
        glib::ObjectExt::property(self, "italic")
    }

    #[doc(alias = "italic-set")]
    pub fn is_italic_set(&self) -> bool {
        glib::ObjectExt::property(self, "italic-set")
    }

    #[doc(alias = "line-background")]
    pub fn line_background(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self, "line-background")
    }

    #[doc(alias = "line-background-set")]
    pub fn is_line_background_set(&self) -> bool {
        glib::ObjectExt::property(self, "line-background-set")
    }

    #[doc(alias = "pango-underline")]
    pub fn pango_underline(&self) -> pango::Underline {
        glib::ObjectExt::property(self, "pango-underline")
    }

    pub fn scale(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self, "scale")
    }

    #[doc(alias = "scale-set")]
    pub fn is_scale_set(&self) -> bool {
        glib::ObjectExt::property(self, "scale-set")
    }

    pub fn is_strikethrough(&self) -> bool {
        glib::ObjectExt::property(self, "strikethrough")
    }

    #[doc(alias = "strikethrough-set")]
    pub fn is_strikethrough_set(&self) -> bool {
        glib::ObjectExt::property(self, "strikethrough-set")
    }

    #[doc(alias = "underline-color")]
    pub fn underline_color(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self, "underline-color")
    }

    #[doc(alias = "underline-color-set")]
    pub fn is_underline_color_set(&self) -> bool {
        glib::ObjectExt::property(self, "underline-color-set")
    }

    #[doc(alias = "underline-set")]
    pub fn is_underline_set(&self) -> bool {
        glib::ObjectExt::property(self, "underline-set")
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Style`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct StyleBuilder {
    background: Option<String>,
    background_set: Option<bool>,
    bold: Option<bool>,
    bold_set: Option<bool>,
    foreground: Option<String>,
    foreground_set: Option<bool>,
    italic: Option<bool>,
    italic_set: Option<bool>,
    line_background: Option<String>,
    line_background_set: Option<bool>,
    pango_underline: Option<pango::Underline>,
    scale: Option<String>,
    scale_set: Option<bool>,
    strikethrough: Option<bool>,
    strikethrough_set: Option<bool>,
    underline_color: Option<String>,
    underline_color_set: Option<bool>,
    underline_set: Option<bool>,
}

impl StyleBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`StyleBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Style`].
    pub fn build(self) -> Style {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref background) = self.background {
            properties.push(("background", background));
        }
        if let Some(ref background_set) = self.background_set {
            properties.push(("background-set", background_set));
        }
        if let Some(ref bold) = self.bold {
            properties.push(("bold", bold));
        }
        if let Some(ref bold_set) = self.bold_set {
            properties.push(("bold-set", bold_set));
        }
        if let Some(ref foreground) = self.foreground {
            properties.push(("foreground", foreground));
        }
        if let Some(ref foreground_set) = self.foreground_set {
            properties.push(("foreground-set", foreground_set));
        }
        if let Some(ref italic) = self.italic {
            properties.push(("italic", italic));
        }
        if let Some(ref italic_set) = self.italic_set {
            properties.push(("italic-set", italic_set));
        }
        if let Some(ref line_background) = self.line_background {
            properties.push(("line-background", line_background));
        }
        if let Some(ref line_background_set) = self.line_background_set {
            properties.push(("line-background-set", line_background_set));
        }
        if let Some(ref pango_underline) = self.pango_underline {
            properties.push(("pango-underline", pango_underline));
        }
        if let Some(ref scale) = self.scale {
            properties.push(("scale", scale));
        }
        if let Some(ref scale_set) = self.scale_set {
            properties.push(("scale-set", scale_set));
        }
        if let Some(ref strikethrough) = self.strikethrough {
            properties.push(("strikethrough", strikethrough));
        }
        if let Some(ref strikethrough_set) = self.strikethrough_set {
            properties.push(("strikethrough-set", strikethrough_set));
        }
        if let Some(ref underline_color) = self.underline_color {
            properties.push(("underline-color", underline_color));
        }
        if let Some(ref underline_color_set) = self.underline_color_set {
            properties.push(("underline-color-set", underline_color_set));
        }
        if let Some(ref underline_set) = self.underline_set {
            properties.push(("underline-set", underline_set));
        }
        glib::Object::new::<Style>(&properties).expect("Failed to create an instance of Style")
    }

    pub fn background(mut self, background: &str) -> Self {
        self.background = Some(background.to_string());
        self
    }

    pub fn background_set(mut self, background_set: bool) -> Self {
        self.background_set = Some(background_set);
        self
    }

    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = Some(bold);
        self
    }

    pub fn bold_set(mut self, bold_set: bool) -> Self {
        self.bold_set = Some(bold_set);
        self
    }

    pub fn foreground(mut self, foreground: &str) -> Self {
        self.foreground = Some(foreground.to_string());
        self
    }

    pub fn foreground_set(mut self, foreground_set: bool) -> Self {
        self.foreground_set = Some(foreground_set);
        self
    }

    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = Some(italic);
        self
    }

    pub fn italic_set(mut self, italic_set: bool) -> Self {
        self.italic_set = Some(italic_set);
        self
    }

    pub fn line_background(mut self, line_background: &str) -> Self {
        self.line_background = Some(line_background.to_string());
        self
    }

    pub fn line_background_set(mut self, line_background_set: bool) -> Self {
        self.line_background_set = Some(line_background_set);
        self
    }

    pub fn pango_underline(mut self, pango_underline: pango::Underline) -> Self {
        self.pango_underline = Some(pango_underline);
        self
    }

    pub fn scale(mut self, scale: &str) -> Self {
        self.scale = Some(scale.to_string());
        self
    }

    pub fn scale_set(mut self, scale_set: bool) -> Self {
        self.scale_set = Some(scale_set);
        self
    }

    pub fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = Some(strikethrough);
        self
    }

    pub fn strikethrough_set(mut self, strikethrough_set: bool) -> Self {
        self.strikethrough_set = Some(strikethrough_set);
        self
    }

    pub fn underline_color(mut self, underline_color: &str) -> Self {
        self.underline_color = Some(underline_color.to_string());
        self
    }

    pub fn underline_color_set(mut self, underline_color_set: bool) -> Self {
        self.underline_color_set = Some(underline_color_set);
        self
    }

    pub fn underline_set(mut self, underline_set: bool) -> Self {
        self.underline_set = Some(underline_set);
        self
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Style")
    }
}
