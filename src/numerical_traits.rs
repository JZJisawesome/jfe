/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use std::ops::*;
use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Octal;
use std::fmt::LowerExp;
use std::fmt::UpperExp;
use std::hash::Hash;
use std::fmt::LowerHex;
use std::fmt::UpperHex;
use std::str::FromStr;
use std::iter::Product;
use std::iter::Sum;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

//TODO include more than just Self; also &Self, &'a Self, etc, and for Shr/Shl/etc, ALL integers
//Also all of the TryFroms

pub trait Integer: Add<Self> + AddAssign<Self> + BitAnd<Self> + BitAndAssign<Self> + BitOr<Self> + BitOrAssign<Self> + BitXor<Self> + BitXorAssign<Self> + Clone + Copy + Debug + Default + Display +
               Div<Self> + DivAssign<Self> + Eq + From<Self> + FromStr + Hash + LowerExp + LowerHex + Mul<Self> + MulAssign<Self> + Not + Octal + Ord + PartialEq<Self> + Product<Self> + Rem<Self> +
               RemAssign<Self> + Shl<Self> + ShlAssign<Self> + Shr<Self> + ShrAssign<Self> + Sub<Self> + SubAssign<Self> + Sum<Self> + ToString + UpperExp + UpperHex + Sized
{}

impl Integer for u8 {}
impl Integer for u16 {}
impl Integer for u32 {}
impl Integer for u64 {}
impl Integer for u128 {}

impl Integer for i8 {}
impl Integer for i16 {}
impl Integer for i32 {}
impl Integer for i64 {}
impl Integer for i128 {}

/* Associated Functions and Methods */

//TODO

/* Functions */

//TODO
