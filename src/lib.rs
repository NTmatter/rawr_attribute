// SPDX-License-Identifier: MIT

// NOTE: The annotation is MIT-licensed for maximum compatibility with other codebases.
// Usage and validation primarily occurs in the RAWR utility.

extern crate proc_macro;
use proc_macro::TokenStream;

/// Identify an interesting piece of code based upon its type, location, git
/// revision, and various other properties. Has no functionality, other than
/// as a marker for external tools.
///
/// Provides no functionality, and expands to nothing.
///
/// NOTE: This is not yet validated, as the fields have not been stabilized.
///
/// # Arguments
/// * `upstream` - Optional name of codebase in which item resides. Use the
///   first-defined codebase in the configuration if not specified.
/// * `rev` - Git revision, branch, or tag that the item was last reviewed.
/// * `file` - Path to containing file, relative to the upstream's root.
/// * `kind` - Type of object as per the codebase's objects of interest.
/// * `ident` - Name of the item being referenced.
/// * `state` - Optional, workflow state. The built-in workflow uses `DONE` and `TODO`.
/// * `action` - Optional, for implementation planning.
/// * `notes` - Optional, human-friendly notes about the reference.
/// * `ignore` - Optional, if `true`, ignore mismatches with the upstream codebase.
#[proc_macro_attribute]
pub fn rawr(_input: TokenStream, _annotated_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// Derive macro to allow use of rawr attribute inside enums and structs.
#[proc_macro_derive(Rawr, attributes(rawr))]
pub fn derive_helper_attr(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}
