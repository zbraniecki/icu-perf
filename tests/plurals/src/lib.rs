extern crate alloc;

#[cfg(feature = "icu4c")]
pub mod icu4c;

#[cfg(feature = "icu4x")]
pub mod icu4x;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Category {
    Zero,
    One,
    Two,
    Few,
    Many,
    Other,
}
