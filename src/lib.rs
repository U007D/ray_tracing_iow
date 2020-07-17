// Safety-critical application lints
#![deny(
    bare_trait_objects,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used,
    clippy::pedantic
)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
#![allow(
    clippy::empty_enum,
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
pub mod consts;
pub mod error;
mod image;

pub use args::Args;
use error::Result;
use float_cmp::Ulps;
pub use image::Image;
use interpol::println;
use std::cmp::max;

#[allow(clippy::needless_pass_by_value)]
pub fn main(_args: Args) -> Result<()> {
    println!("P3\n{Image::width()} {Image::height()}\n255");

    let w_denom = f64::from(max(1, u32::from(Image::width()).saturating_sub(1)));
    let h_denom = f64::from(max(1, u32::from(Image::height()).saturating_sub(1)));
    (0..u32::from(Image::height())).for_each(|j| {
        (0..u32::from(Image::width())).for_each(|i| {
            let r = f64::from(i) / w_denom;
            let g = f64::from(j) / h_denom;
            let b = 0.25_f64;

            let ir = (256.0.prev() * r).trunc();
            let ig = (256.0.prev() * g).trunc();
            let ib = (256.0.prev() * b).trunc();
            println!("{ir} {ig} {ib}");
        })
    });

    Ok(())
}
