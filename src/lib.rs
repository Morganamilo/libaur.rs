//! #libaur
//!
//! A collection of AUR related functions for making AUR stuff easier

#![warn(missing_docs)]

#[cfg(feature = "aur")]
mod aur;
#[cfg(feature = "comments")]
mod comments;
mod error;
#[cfg(feature = "news")]
mod news;
mod split;
mod utils;

#[cfg(feature = "aur")]
pub use aur::*;
#[cfg(feature = "comments")]
pub use comments::*;
pub use error::*;
#[cfg(feature = "news")]
pub use news::*;
pub use split::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
