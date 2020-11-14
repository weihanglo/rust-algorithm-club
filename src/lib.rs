//! ![](https://rust-algo.club/logo.svg)
//!
//! # Welcome to Rust algorithm club!

#![warn(missing_docs)]
#![warn(dead_code)]
#![deny(deprecated)]
#![deny(nonstandard_style)]
#![doc(html_logo_url = "https://rust-algo.club/favicon.png")]

pub mod collections;
pub mod searching;
pub mod sorting;

mod levenshtein_distance;
pub use levenshtein_distance::{levenshtein_distance, levenshtein_distance_naive};

mod hamming_distance;
pub use hamming_distance::{hamming_distance, hamming_distance_str};
