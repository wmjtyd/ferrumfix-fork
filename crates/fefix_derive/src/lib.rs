//! Derive macros for FerrumFIX.

#![deny(missing_debug_implementations, clippy::useless_conversion)]

mod derive_fix_value;

use proc_macro::TokenStream;

/// A *derive macro* for the `FixValue` trait on `enum`'s.
#[proc_macro_derive(FixValue, attributes(fefix))]
pub fn derive_fix_value(input: TokenStream) -> TokenStream {
    derive_fix_value::derive_fix_value(input)
}
