// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(clippy::let_unit_value)]
#![allow(clippy::new_without_default)]
#![allow(clippy::type_complexity)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::derive_hash_xor_eq)]
#![allow(clippy::needless_doctest_main)]
#![allow(clippy::too_many_arguments)]
#![allow(deprecated)]

pub use cairo;
pub use ffi;
pub use gdk_pixbuf;
pub use gio;
pub use glib;
pub use pango;

// GDK 4 has no runtime to initialize
macro_rules! assert_initialized_main_thread {
    () => {};
}

// No-op
macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(clippy::match_same_arms)]
#[allow(clippy::let_and_return)]
#[allow(clippy::many_single_char_names)]
#[allow(clippy::wrong_self_convention)]
#[allow(clippy::cognitive_complexity)]
#[allow(clippy::clone_on_copy)]
#[allow(clippy::cast_ptr_alignment)]
#[allow(unused_imports)]
mod auto;

mod alias;
pub mod prelude;
pub mod subclass;

mod cairo_interaction;
mod clipboard;
mod content_deserializer;
mod draw_context;
mod drop;
mod functions;
mod keymap_key;
mod popup_layout;
mod rectangle;
mod rgba;
mod surface;
mod time_coord;
mod toplevel_size;

pub use self::auto::functions::*;
pub use auto::*;

pub use alias::*;
pub use functions::*;

pub use keymap_key::KeymapKey;
pub use popup_layout::PopupLayoutExtManual;
pub use rectangle::Rectangle;
pub use rgba::{RgbaParseError, RGBA};
pub use surface::SurfaceExtManual;
pub use time_coord::TimeCoord;
pub use toplevel_size::ToplevelSize;
