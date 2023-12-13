//! A collection of useful functions and types for solving [Advent of Code][aoc] puzzles.
//!
//! # Design choices
//!
//! ## Error Handling
//!
//! Errors in this crate are generally not handled. In most of the cases when something unexpected
//! happens, the library will panic. There is simply no need for elaborate error handling in
//! [Advent of Code][aoc]. In fact, you want quite the opposite. If there is an error, it most
//! likely indicates a bug in your code, in which case you probably want the code to fail instantly
//! and tell you what is wrong.
//!
//! ## Simplicity vs. Performance
//!
//! Whenever there is a tradeoff between simplicity and performance, this crate chooses simplicity
//! over performance. Performance does not generally matter too much in [Advent of Code][aoc]. On
//! the other hand, not having to deal with an unnecessarily complex API may prevent a bunch of
//! headaches.
//!
//! [aoc]: https://adventofcode.com

#![warn(clippy::dbg_macro)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::use_debug)]
#![warn(missing_docs)]

mod grid;

pub use grid::Grid;
