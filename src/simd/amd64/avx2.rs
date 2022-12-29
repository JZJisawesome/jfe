/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use core::arch::x86_64;
use std::fmt::Debug;
use std::ops::*;
use std::mem::MaybeUninit;

use super::avx::Vector256;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Traits */

pub trait IntegerVector256: Vector256 + AsRef<x86_64::__m256i> + AsMut<x86_64::__m256i> {}//Markerish trait

//TODO on the I* vectors, Shr will be arithmetic, but it will be logical on the U* vectors
pub trait ShiftableVector256: IntegerVector256 + Shl + ShlAssign + Shr + ShrAssign {
    //TODO

    //TODO perhaps have I8, I16, I32, I64 so that we do arithmetic shifts on that, and regular shifs on the U versions?
    //fn shri<const AMOUNT: i32>(self: Self) -> Self;
    //fn shli<const AMOUNT: i32>(self: Self) -> Self;
}

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

//TODO
