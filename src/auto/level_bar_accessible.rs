// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Accessible;
use WidgetAccessible;
use atk;
use ffi;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct LevelBarAccessible(Object<ffi::GtkLevelBarAccessible, ffi::GtkLevelBarAccessibleClass, LevelBarAccessibleClass>) @extends WidgetAccessible, Accessible, atk::Object;

    match fn {
        get_type => || ffi::gtk_level_bar_accessible_get_type(),
    }
}

impl LevelBarAccessible {}

pub const NONE_LEVEL_BAR_ACCESSIBLE: Option<&LevelBarAccessible> = None;

impl fmt::Display for LevelBarAccessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LevelBarAccessible")
    }
}
