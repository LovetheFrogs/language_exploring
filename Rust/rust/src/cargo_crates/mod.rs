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

/* To make it easier for users of your crate, you should create a public API with `pub use`. This simplifies the imports and can change an import like
 *          use my_crate::some_module::another_module::UsefulType;
 * to something easier to locate like
 *          use my_crate::UsefulType;
 * The example below shows how you can simplify the import in a crate named arts.
 */

/*
//! Art
//! 
//! A library for modeling artistic concepts.
*/

/* This will simplify imports for users of the public API, allowing them to call the commands
 *          use art::mix
 *          use art::PrimaryColor;
 */
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors acording to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::cargo_crates::kinds::*;

    use super::kinds::{PrimaryColor, SecondaryColor};

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        SecondaryColor::Green
    }
}

/* Chapter 14.3-Cargo Workspaces can be found in folder add, located in languague_exploring/Rust/add */