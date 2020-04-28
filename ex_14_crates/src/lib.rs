//! # Art
//!
//! A library to mixing the colors

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {

    // The primary colors
    pub enum PrimaryColor {
        Red,
        Yelow,
        Blue
    }

    //The secondary colors
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    ///Combines two primary colors to get the secondary one

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}

