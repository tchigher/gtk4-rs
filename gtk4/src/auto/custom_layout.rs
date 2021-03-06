// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::LayoutManager;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct CustomLayout(Object<ffi::GtkCustomLayout, ffi::GtkCustomLayoutClass>) @extends LayoutManager;

    match fn {
        get_type => || ffi::gtk_custom_layout_get_type(),
    }
}

impl CustomLayout {
    //pub fn new<P: FnMut(&Widget, &Orientation, i32, i32, i32, i32, i32), Q: FnMut(&Widget, i32, i32, i32)>(request_mode: Option<&mut dyn (FnMut(&Widget) -> SizeRequestMode)>, measure: P, allocate: Q) -> CustomLayout {
    //    unsafe { TODO: call ffi:gtk_custom_layout_new() }
    //}
}

pub const NONE_CUSTOM_LAYOUT: Option<&CustomLayout> = None;

impl fmt::Display for CustomLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CustomLayout")
    }
}
