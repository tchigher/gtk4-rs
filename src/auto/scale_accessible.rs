// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Accessible;
use RangeAccessible;
use WidgetAccessible;
use atk;
use ffi;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct ScaleAccessible(Object<ffi::GtkScaleAccessible, ffi::GtkScaleAccessibleClass, ScaleAccessibleClass>) @extends RangeAccessible, WidgetAccessible, Accessible, atk::Object;

    match fn {
        get_type => || ffi::gtk_scale_accessible_get_type(),
    }
}

impl ScaleAccessible {}

pub const NONE_SCALE_ACCESSIBLE: Option<&ScaleAccessible> = None;

impl fmt::Display for ScaleAccessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScaleAccessible")
    }
}
