//! Condition codes for the Cranelift code generator.
//!
//! A condition code here is an enumerated type that determined how to compare two numbers. There
//! are different rules for comparing integers and floating point numbers, so they use different
//! condition codes.

use core::fmt::{self, Display, Formatter};
use core::str::FromStr;
use core::prelude::rust_2024::derive;
use core::marker::Copy;
use core::fmt::Debug;
use core::hash::Hash;
use core::clone::Clone;
use core::cmp::{Eq, PartialEq};

/// Common traits of condition codes.
pub trait CondCode: Copy {
    /// Get the inverse condition code of `self`.
    ///
    /// The inverse condition code produces the opposite result for all comparisons.
    /// That is, `cmp CC, x, y` is true if and only if `cmp CC.inverse(), x, y` is false.
    #[must_use]
    fn inverse(self) -> Self;

    /// Get the reversed condition code for `self`.
    ///
    /// The reversed condition code produces the same result as swapping `x` and `y` in the
    /// comparison. That is, `cmp CC, x, y` is the same as `cmp CC.reverse(), y, x`.
    #[must_use]
    fn reverse(self) -> Self;
}

/// Condition code for comparing integers.
///
/// This condition code is used by the `icmp` instruction to compare integer values. There are
/// separate codes for comparing the integers as signed or unsigned numbers where it makes a
/// difference.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum IntCC {
    /// `==`.
    Equal,
    /// `!=`.
    NotEqual,
    /// Signed `<`.
    SignedLessThan,
    /// Signed `>=`.
    SignedGreaterThanOrEqual,
    /// Signed `>`.
    SignedGreaterThan,
    /// Signed `<=`.
    SignedLessThanOrEqual,
    /// Unsigned `<`.
    UnsignedLessThan,
    /// Unsigned `>=`.
    UnsignedGreaterThanOrEqual,
    /// Unsigned `>`.
    UnsignedGreaterThan,
    /// Unsigned `<=`.
    UnsignedLessThanOrEqual,
    /// Signed Overflow.
    Overflow,
    /// Signed No Overflow.
    NotOverflow,
}

impl CondCode for IntCC {
    fn inverse(self) -> Self {
        use self::IntCC::*;
        match self {
            Equal => NotEqual,
            NotEqual => Equal,
            SignedLessThan => SignedGreaterThanOrEqual,
            SignedGreaterThanOrEqual => SignedLessThan,
            SignedGreaterThan => SignedLessThanOrEqual,
            SignedLessThanOrEqual => SignedGreaterThan,
            UnsignedLessThan => UnsignedGreaterThanOrEqual,
            UnsignedGreaterThanOrEqual => UnsignedLessThan,
            UnsignedGreaterThan => UnsignedLessThanOrEqual,
            UnsignedLessThanOrEqual => UnsignedGreaterThan,
            Overflow => NotOverflow,
            NotOverflow => Overflow,
        }
    }

    fn reverse(self) -> Self {
        use self::IntCC::*;
        match self {
            Equal => Equal,
            NotEqual => NotEqual,
            SignedGreaterThan => SignedLessThan,
            SignedGreaterThanOrEqual => SignedLessThanOrEqual,
            SignedLessThan => SignedGreaterThan,
            SignedLessThanOrEqual => SignedGreaterThanOrEqual,
            UnsignedGreaterThan => UnsignedLessThan,
            UnsignedGreaterThanOrEqual => UnsignedLessThanOrEqual,
            UnsignedLessThan => UnsignedGreaterThan,
            UnsignedLessThanOrEqual => UnsignedGreaterThanOrEqual,
            Overflow => Overflow,
            NotOverflow => NotOverflow,
        }
    }
}

impl Display for IntCC {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use self::IntCC::*;
        f.write_str(match *self {
            Equal => "eq",
            NotEqual => "ne",
            SignedGreaterThan => "sgt",
            SignedGreaterThanOrEqual => "sge",
            SignedLessThan => "slt",
            SignedLessThanOrEqual => "sle",
            UnsignedGreaterThan => "ugt",
            UnsignedGreaterThanOrEqual => "uge",
            UnsignedLessThan => "ult",
            UnsignedLessThanOrEqual => "ule",
            Overflow => "of",
            NotOverflow => "nof",
        })
    }
}

impl FromStr for IntCC {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::IntCC::*;
        match s {
            "eq" => Ok(Equal),
            "ne" => Ok(NotEqual),
            "sge" => Ok(SignedGreaterThanOrEqual),
            "sgt" => Ok(SignedGreaterThan),
            "sle" => Ok(SignedLessThanOrEqual),
            "slt" => Ok(SignedLessThan),
            "uge" => Ok(UnsignedGreaterThanOrEqual),
            "ugt" => Ok(UnsignedGreaterThan),
            "ule" => Ok(UnsignedLessThanOrEqual),
            "ult" => Ok(UnsignedLessThan),
            "of" => Ok(Overflow),
            "nof" => Ok(NotOverflow),
            _ => Err(()),
        }
    }
}

/// Condition code for comparing floating point numbers.
///
/// This condition code is used by the `fcmp` instruction to compare floating point values. Two
/// IEEE floating point values relate in exactly one of four ways:
///
/// 1. `UN` - unordered when either value is NaN.
/// 2. `EQ` - equal numerical value.
/// 3. `LT` - `x` is less than `y`.
/// 4. `GT` - `x` is greater than `y`.
///
/// Note that `0.0` and `-0.0` relate as `EQ` because they both represent the number 0.
///
/// The condition codes described here are used to produce a single boolean value from the
/// comparison. The 14 condition codes here cover every possible combination of the relation above
/// except the impossible `!UN & !EQ & !LT & !GT` and the always true `UN | EQ | LT | GT`.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum FloatCC {
    /// EQ | LT | GT
    Ordered,
    /// UN
    Unordered,

    /// EQ
    Equal,
    /// The C '!=' operator is the inverse of '==': `NotEqual`.
    /// UN | LT | GT
    NotEqual,
    /// LT | GT
    OrderedNotEqual,
    /// UN | EQ
    UnorderedOrEqual,

    /// LT
    LessThan,
    /// LT | EQ
    LessThanOrEqual,
    /// GT
    GreaterThan,
    /// GT | EQ
    GreaterThanOrEqual,

    /// UN | LT
    UnorderedOrLessThan,
    /// UN | LT | EQ
    UnorderedOrLessThanOrEqual,
    /// UN | GT
    UnorderedOrGreaterThan,
    /// UN | GT | EQ
    UnorderedOrGreaterThanOrEqual,
}

impl CondCode for FloatCC {
    fn inverse(self) -> Self {
        use self::FloatCC::*;
        match self {
            Ordered => Unordered,
            Unordered => Ordered,
            Equal => NotEqual,
            NotEqual => Equal,
            OrderedNotEqual => UnorderedOrEqual,
            UnorderedOrEqual => OrderedNotEqual,
            LessThan => UnorderedOrGreaterThanOrEqual,
            LessThanOrEqual => UnorderedOrGreaterThan,
            GreaterThan => UnorderedOrLessThanOrEqual,
            GreaterThanOrEqual => UnorderedOrLessThan,
            UnorderedOrLessThan => GreaterThanOrEqual,
            UnorderedOrLessThanOrEqual => GreaterThan,
            UnorderedOrGreaterThan => LessThanOrEqual,
            UnorderedOrGreaterThanOrEqual => LessThan,
        }
    }
    fn reverse(self) -> Self {
        use self::FloatCC::*;
        match self {
            Ordered => Ordered,
            Unordered => Unordered,
            Equal => Equal,
            NotEqual => NotEqual,
            OrderedNotEqual => OrderedNotEqual,
            UnorderedOrEqual => UnorderedOrEqual,
            LessThan => GreaterThan,
            LessThanOrEqual => GreaterThanOrEqual,
            GreaterThan => LessThan,
            GreaterThanOrEqual => LessThanOrEqual,
            UnorderedOrLessThan => UnorderedOrGreaterThan,
            UnorderedOrLessThanOrEqual => UnorderedOrGreaterThanOrEqual,
            UnorderedOrGreaterThan => UnorderedOrLessThan,
            UnorderedOrGreaterThanOrEqual => UnorderedOrLessThanOrEqual,
        }
    }
}

impl Display for FloatCC {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use self::FloatCC::*;
        f.write_str(match *self {
            Ordered => "ord",
            Unordered => "uno",
            Equal => "eq",
            NotEqual => "ne",
            OrderedNotEqual => "one",
            UnorderedOrEqual => "ueq",
            LessThan => "lt",
            LessThanOrEqual => "le",
            GreaterThan => "gt",
            GreaterThanOrEqual => "ge",
            UnorderedOrLessThan => "ult",
            UnorderedOrLessThanOrEqual => "ule",
            UnorderedOrGreaterThan => "ugt",
            UnorderedOrGreaterThanOrEqual => "uge",
        })
    }
}

impl FromStr for FloatCC {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::FloatCC::*;
        match s {
            "ord" => Ok(Ordered),
            "uno" => Ok(Unordered),
            "eq" => Ok(Equal),
            "ne" => Ok(NotEqual),
            "one" => Ok(OrderedNotEqual),
            "ueq" => Ok(UnorderedOrEqual),
            "lt" => Ok(LessThan),
            "le" => Ok(LessThanOrEqual),
            "gt" => Ok(GreaterThan),
            "ge" => Ok(GreaterThanOrEqual),
            "ult" => Ok(UnorderedOrLessThan),
            "ule" => Ok(UnorderedOrLessThanOrEqual),
            "ugt" => Ok(UnorderedOrGreaterThan),
            "uge" => Ok(UnorderedOrGreaterThanOrEqual),
            _ => Err(()),
        }
    }
}
