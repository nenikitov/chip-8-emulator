#![feature(slice_flatten)]
#![warn(clippy::pedantic)]
#![allow(
    unused,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::struct_excessive_bools,
    clippy::too_many_lines,
    clippy::unreadable_literal,
    clippy::wildcard_imports,
)]
#![warn(unused_imports)]

mod chip_8;
mod instruction;

pub use crate::chip_8::*;
