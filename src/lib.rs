// SPDX-License-Identifier: MIT

// NOTE: The annotation is MIT-licensed for maximum compatibility with other codebases.
// Usage and validation primarily occurs in the RAWR utility.

extern crate proc_macro;
use proc_macro::TokenStream;

/// Identify an interesting piece of code based upon its type, location, git
/// revision, and various other properties. Has no functionality, other than
/// as a marker for external tools.
///
/// NOTE: This is not yet validated, as the fields have not been stabilized.
///
/// # Arguments
/// * `codebase` - Optional name of codebase in which item resides. Use the
///   first-defined codebase in the configuration if nothing else is defined.
/// * `language` - Optional language if different from the codebase's default.
/// * `kind` - Type of object as per the codebase's objects of interest.
/// * `identifier` - Name of the item being referenced.
/// * `path` - Path to containing file, relative to the codebase's root.
/// * `revision` - Git revision, branch, or tag that the item was last reviewed.
/// * `ignore` - Optional, if `true`, ignore mismatches with the other codebase.
/// * `state` - Optional, workflow state.
///   TODO: Consider enum of Done, InProgress, NotStarted, WontImplement, Other(String)
/// * `notes` - Optional human-friendly notes about the reference.
/// * `hash_alg` - Optional hashing algorithm for contents. Default algorithm is
///   `sha256`.
/// * `salt` - Optional u64 salt for hash. Will look up in the database if not
///   specified, and randomly initialized if not found.
/// * `hash` - Optional hex string of salted hash.
#[proc_macro_attribute]
pub fn rawr(_input: TokenStream, _annotated_item: TokenStream) -> TokenStream {
    TokenStream::new()
}
