// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Accessible;
use ContainerAccessible;
use WidgetAccessible;
use atk;
use ffi;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct TextViewAccessible(Object<ffi::GtkTextViewAccessible, ffi::GtkTextViewAccessibleClass, TextViewAccessibleClass>) @extends ContainerAccessible, WidgetAccessible, Accessible, atk::Object;

    match fn {
        get_type => || ffi::gtk_text_view_accessible_get_type(),
    }
}

impl TextViewAccessible {}

pub const NONE_TEXT_VIEW_ACCESSIBLE: Option<&TextViewAccessible> = None;

impl fmt::Display for TextViewAccessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TextViewAccessible")
    }
}
