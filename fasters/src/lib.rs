//! Fasters is a standard-compliant FIX & FAST (FIX Adapted for STreaming)
//! implementation in pure Rust.
//!
//! FIX and FAST functionality is kept isolated in the
//! [`fasters::fix`](fasters::fix) and [`fasters::fast`](fasters::fast) modules,
//! respectively.

pub mod codegen;
mod dictionary;
#[deprecated]
pub mod internals;
pub mod ir;
pub mod presentation;
pub mod session;
pub mod sofh;

pub use dictionary::Dictionary;
