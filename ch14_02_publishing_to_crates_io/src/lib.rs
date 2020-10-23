//! # ch 14.02 - publishing to crates.io
//!
//! using crates.io to document, test, and publish code

// re-export for public use
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

/// Returns one as unsigned 8 byte int
///
/// # Examples
///
/// ```
/// use self::ch14_02_publishing_to_crates_io::one;
///
/// let unsigned_int = one();
///
/// assert_eq!(1, unsigned_int);
/// ```
pub fn one() -> u8 {
    1
}

pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(Debug)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Green // fixme - whoops
    }
}