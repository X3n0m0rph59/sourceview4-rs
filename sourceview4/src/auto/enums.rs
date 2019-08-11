// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::StaticType;
use glib::Type;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use gobject_sys;
use gtk_source_sys;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum BackgroundPatternType {
    None,
    Grid,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for BackgroundPatternType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BackgroundPatternType::{}", match *self {
            BackgroundPatternType::None => "None",
            BackgroundPatternType::Grid => "Grid",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl ToGlib for BackgroundPatternType {
    type GlibType = gtk_source_sys::GtkSourceBackgroundPatternType;

    fn to_glib(&self) -> gtk_source_sys::GtkSourceBackgroundPatternType {
        match *self {
            BackgroundPatternType::None => gtk_source_sys::GTK_SOURCE_BACKGROUND_PATTERN_TYPE_NONE,
            BackgroundPatternType::Grid => gtk_source_sys::GTK_SOURCE_BACKGROUND_PATTERN_TYPE_GRID,
            BackgroundPatternType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<gtk_source_sys::GtkSourceBackgroundPatternType> for BackgroundPatternType {
    fn from_glib(value: gtk_source_sys::GtkSourceBackgroundPatternType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => BackgroundPatternType::None,
            1 => BackgroundPatternType::Grid,
            value => BackgroundPatternType::__Unknown(value),
        }
    }
}

impl StaticType for BackgroundPatternType {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_source_sys::gtk_source_background_pattern_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for BackgroundPatternType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for BackgroundPatternType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for BackgroundPatternType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum BracketMatchType {
    None,
    OutOfRange,
    NotFound,
    Found,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for BracketMatchType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BracketMatchType::{}", match *self {
            BracketMatchType::None => "None",
            BracketMatchType::OutOfRange => "OutOfRange",
            BracketMatchType::NotFound => "NotFound",
            BracketMatchType::Found => "Found",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl ToGlib for BracketMatchType {
    type GlibType = gtk_source_sys::GtkSourceBracketMatchType;

    fn to_glib(&self) -> gtk_source_sys::GtkSourceBracketMatchType {
        match *self {
            BracketMatchType::None => gtk_source_sys::GTK_SOURCE_BRACKET_MATCH_NONE,
            BracketMatchType::OutOfRange => gtk_source_sys::GTK_SOURCE_BRACKET_MATCH_OUT_OF_RANGE,
            BracketMatchType::NotFound => gtk_source_sys::GTK_SOURCE_BRACKET_MATCH_NOT_FOUND,
            BracketMatchType::Found => gtk_source_sys::GTK_SOURCE_BRACKET_MATCH_FOUND,
            BracketMatchType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<gtk_source_sys::GtkSourceBracketMatchType> for BracketMatchType {
    fn from_glib(value: gtk_source_sys::GtkSourceBracketMatchType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => BracketMatchType::None,
            1 => BracketMatchType::OutOfRange,
            2 => BracketMatchType::NotFound,
            3 => BracketMatchType::Found,
            value => BracketMatchType::__Unknown(value),
        }
    }
}

impl StaticType for BracketMatchType {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_source_sys::gtk_source_bracket_match_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for BracketMatchType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for BracketMatchType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for BracketMatchType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum ChangeCaseType {
    Lower,
    Upper,
    Toggle,
    Title,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ChangeCaseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ChangeCaseType::{}", match *self {
            ChangeCaseType::Lower => "Lower",
            ChangeCaseType::Upper => "Upper",
            ChangeCaseType::Toggle => "Toggle",
            ChangeCaseType::Title => "Title",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl ToGlib for ChangeCaseType {
    type GlibType = gtk_source_sys::GtkSourceChangeCaseType;

    fn to_glib(&self) -> gtk_source_sys::GtkSourceChangeCaseType {
        match *self {
            ChangeCaseType::Lower => gtk_source_sys::GTK_SOURCE_CHANGE_CASE_LOWER,
            ChangeCaseType::Upper => gtk_source_sys::GTK_SOURCE_CHANGE_CASE_UPPER,
            ChangeCaseType::Toggle => gtk_source_sys::GTK_SOURCE_CHANGE_CASE_TOGGLE,
            ChangeCaseType::Title => gtk_source_sys::GTK_SOURCE_CHANGE_CASE_TITLE,
            ChangeCaseType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<gtk_source_sys::GtkSourceChangeCaseType> for ChangeCaseType {
    fn from_glib(value: gtk_source_sys::GtkSourceChangeCaseType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => ChangeCaseType::Lower,
            1 => ChangeCaseType::Upper,
            2 => ChangeCaseType::Toggle,
            3 => ChangeCaseType::Title,
            value => ChangeCaseType::__Unknown(value),
        }
    }
}

impl StaticType for ChangeCaseType {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_source_sys::gtk_source_change_case_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ChangeCaseType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ChangeCaseType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for ChangeCaseType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum CompressionType {
    None,
    Gzip,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for CompressionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CompressionType::{}", match *self {
            CompressionType::None => "None",
            CompressionType::Gzip => "Gzip",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl ToGlib for CompressionType {
    type GlibType = gtk_source_sys::GtkSourceCompressionType;

    fn to_glib(&self) -> gtk_source_sys::GtkSourceCompressionType {
        match *self {
            CompressionType::None => gtk_source_sys::GTK_SOURCE_COMPRESSION_TYPE_NONE,
            CompressionType::Gzip => gtk_source_sys::GTK_SOURCE_COMPRESSION_TYPE_GZIP,
            CompressionType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<gtk_source_sys::GtkSourceCompressionType> for CompressionType {
    fn from_glib(value: gtk_source_sys::GtkSourceCompressionType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => CompressionType::None,
            1 => CompressionType::Gzip,
            value => CompressionType::__Unknown(value),
        }
    }
}

impl StaticType for CompressionType {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_source_sys::gtk_source_compression_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for CompressionType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for CompressionType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for CompressionType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// The alignment mode of the renderer, when a cell spans multiple lines (due to
/// text wrapping).
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum GutterRendererAlignmentMode {
    Cell,
    First,
    Last,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for GutterRendererAlignmentMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GutterRendererAlignmentMode::{}", match *self {
            GutterRendererAlignmentMode::Cell => "Cell",
            GutterRendererAlignmentMode::First => "First",
            GutterRendererAlignmentMode::Last => "Last",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl ToGlib for GutterRendererAlignmentMode {
    type GlibType = gtk_source_sys::GtkSourceGutterRendererAlignmentMode;

    fn to_glib(&self) -> gtk_source_sys::GtkSourceGutterRendererAlignmentMode {
        match *self {
            GutterRendererAlignmentMode::Cell => gtk_source_sys::GTK_SOURCE_GUTTER_RENDERER_ALIGNMENT_MODE_CELL,
            GutterRendererAlignmentMode::First => gtk_source_sys::GTK_SOURCE_GUTTER_RENDERER_ALIGNMENT_MODE_FIRST,
            GutterRendererAlignmentMode::Last => gtk_source_sys::GTK_SOURCE_GUTTER_RENDERER_ALIGNMENT_MODE_LAST,
            GutterRendererAlignmentMode::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<gtk_source_sys::GtkSourceGutterRendererAlignmentMode> for GutterRendererAlignmentMode {
    fn from_glib(value: gtk_source_sys::GtkSourceGutterRendererAlignmentMode) -> Self {
        skip_assert_initialized!();
        match value {
            0 => GutterRendererAlignmentMode::Cell,
            1 => GutterRendererAlignmentMode::First,
            2 => GutterRendererAlignmentMode::Last,
            value => GutterRendererAlignmentMode::__Unknown(value),
        }
    }
}

impl StaticType for GutterRendererAlignmentMode {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_source_sys::gtk_source_gutter_renderer_alignment_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for GutterRendererAlignmentMode {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for GutterRendererAlignmentMode {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for GutterRendererAlignmentMode {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum NewlineType {
    Lf,
    Cr,
    CrLf,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for NewlineType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NewlineType::{}", match *self {
            NewlineType::Lf => "Lf",
            NewlineType::Cr => "Cr",
            NewlineType::CrLf => "CrLf",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl ToGlib for NewlineType {
    type GlibType = gtk_source_sys::GtkSourceNewlineType;

    fn to_glib(&self) -> gtk_source_sys::GtkSourceNewlineType {
        match *self {
            NewlineType::Lf => gtk_source_sys::GTK_SOURCE_NEWLINE_TYPE_LF,
            NewlineType::Cr => gtk_source_sys::GTK_SOURCE_NEWLINE_TYPE_CR,
            NewlineType::CrLf => gtk_source_sys::GTK_SOURCE_NEWLINE_TYPE_CR_LF,
            NewlineType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<gtk_source_sys::GtkSourceNewlineType> for NewlineType {
    fn from_glib(value: gtk_source_sys::GtkSourceNewlineType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => NewlineType::Lf,
            1 => NewlineType::Cr,
            2 => NewlineType::CrLf,
            value => NewlineType::__Unknown(value),
        }
    }
}

impl StaticType for NewlineType {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_source_sys::gtk_source_newline_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for NewlineType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for NewlineType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for NewlineType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum SmartHomeEndType {
    Disabled,
    Before,
    After,
    Always,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for SmartHomeEndType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SmartHomeEndType::{}", match *self {
            SmartHomeEndType::Disabled => "Disabled",
            SmartHomeEndType::Before => "Before",
            SmartHomeEndType::After => "After",
            SmartHomeEndType::Always => "Always",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl ToGlib for SmartHomeEndType {
    type GlibType = gtk_source_sys::GtkSourceSmartHomeEndType;

    fn to_glib(&self) -> gtk_source_sys::GtkSourceSmartHomeEndType {
        match *self {
            SmartHomeEndType::Disabled => gtk_source_sys::GTK_SOURCE_SMART_HOME_END_DISABLED,
            SmartHomeEndType::Before => gtk_source_sys::GTK_SOURCE_SMART_HOME_END_BEFORE,
            SmartHomeEndType::After => gtk_source_sys::GTK_SOURCE_SMART_HOME_END_AFTER,
            SmartHomeEndType::Always => gtk_source_sys::GTK_SOURCE_SMART_HOME_END_ALWAYS,
            SmartHomeEndType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<gtk_source_sys::GtkSourceSmartHomeEndType> for SmartHomeEndType {
    fn from_glib(value: gtk_source_sys::GtkSourceSmartHomeEndType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => SmartHomeEndType::Disabled,
            1 => SmartHomeEndType::Before,
            2 => SmartHomeEndType::After,
            3 => SmartHomeEndType::Always,
            value => SmartHomeEndType::__Unknown(value),
        }
    }
}

impl StaticType for SmartHomeEndType {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_source_sys::gtk_source_smart_home_end_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SmartHomeEndType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SmartHomeEndType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for SmartHomeEndType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}
