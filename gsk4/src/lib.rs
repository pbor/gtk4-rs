// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
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
pub use gdk;
pub use glib;
pub use graphene;
pub use pango;

// GSK 4 has no runtime to initialize
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

pub mod prelude;

pub use auto::*;

mod border_node;
mod color_stop;
mod conic_gradient_node;
mod linear_gradient_node;
mod parse_location;
mod radial_gradient_node;
mod repeating_linear_gradient_node;
mod repeating_radial_gradient_node;
mod rounded_rect;
mod shadow;
mod shadow_node;

pub use border_node::BorderNodeManualExt;
pub use color_stop::ColorStop;
pub use parse_location::ParseLocation;
pub use rounded_rect::RoundedRect;
pub use shadow::Shadow;
