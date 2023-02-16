/* When coding a crate, add the following lines to your Cargo.toml file. 
 *         [profile.dev]
 *         opt-level = 0
 * 
 *         [profile.release]
 *         opt-level = 3
 * This makes the optimisation level for development minimal (shortens compile times), but makes the level for releases be the maximum because it will only be 
 * compiled once and thus, more optimisation makes code better with no trade-off.
 */

/* To document functions for a cargo package, use three slashes `///` as this notation suports markdown. We can translate this type of comments to HTML by using
 * the `cargo doc` command. Create and open it whith `cargo doc --open`. Documentation comments often have a Panics, Errors and Safety sections, as well as Examples.
 * The style of doc comment `//!` adds documentation to the item that contains the comments rather than to the items following the comments.
 */

//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the given number
/// 
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}