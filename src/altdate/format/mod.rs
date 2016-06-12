// This module and its submodules where heavily influenced by rust-chrono by Kang Seonghoon


/// Fixed-format item types.
///
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Fixed {
    LongWeekdayName,
}


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Item<'a> {
    /// A literally printed text
    Literal(&'a str),
    /// Whitespace
    Space(&'a str),
    Fixed(Fixed),
    /// Issues a formatting error. Used to signal an invalid format string.
    Error,
}

macro_rules! lit { ($x:expr) => (Item::Literal($x)) }
macro_rules! sp { ($x:expr) => (Item::Space($x)) }
macro_rules! fix { ($x:ident) => (Item::Fixed(Fixed::$x)) }

pub mod ddate;
