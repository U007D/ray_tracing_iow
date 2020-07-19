// Safety-critical application lints
#![deny(
    bare_trait_objects,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used,
    clippy::pedantic
)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
#![allow(
    clippy::empty_enum,
    clippy::pub_enum_variant_names,
    clippy::iter_nth_zero,
    clippy::match_bool,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::wildcard_imports
)]
// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]
// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing
// license files and more
//#![warn(clippy::cargo, clippy::restriction, missing_docs, clippy::missing_errors_doc, warnings)]
//#![deny(warnings)]

mod args;
mod color;
pub mod consts;
pub mod error;
mod image;

use error::Result;
use float_cmp::Ulps;
use std::cmp::max;
pub use {args::Args, color::Color, image::Image};

#[allow(clippy::needless_pass_by_value)]
pub fn main(_args: Args) -> Result<()> {
    let img = render_image(Image::new(256, 256)?);
    println!("{}", img.serialize_to_ppm());

    Ok(())
}

fn max_non_zero_index_from_dimension(dimension: u32) -> f64 {
    f64::from(max(1, dimension).saturating_sub(1))
}

fn render_image(mut img: Image) -> Image {
    let w_denom = max_non_zero_index_from_dimension(img.width());
    let h_denom = max_non_zero_index_from_dimension(img.height());
    (0..img.height()).for_each(|y| {
        (0..img.width()).for_each(|x| {
            let r = f64::from(x) / w_denom;
            let g = f64::from(y) / h_denom;
            let b = 0.25_f64;

            let r_trunc = (256.0.prev() * r).trunc();
            let g_trunc = (256.0.prev() * g).trunc();
            let b_trunc = (256.0.prev() * b).trunc();

            img.set_pixel(x, y, Color(r_trunc, g_trunc, b_trunc));
        })
    });
    img
}
