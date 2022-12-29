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

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Traits */

pub trait Vector256:
    Copy + Clone + Debug +
    From<x86_64::__m256> + From<x86_64::__m256i> + From<x86_64::__m256d> + Into<x86_64::__m256> + Into<x86_64::__m256i> + Into<x86_64::__m256d> +
    Add + AddAssign + BitAnd + BitAndAssign + BitOr + BitOrAssign + BitXor + BitXorAssign + Sub + SubAssign
{
    type AssociatedPrimitive;
    type AssociatedPrimitiveArray;

    fn new_from_array(array: Self::AssociatedPrimitiveArray) -> Self;
    fn new_broadcasted(scalar: Self::AssociatedPrimitive) -> Self;
    fn new_zeroed() -> Self;
    fn new_uninit() -> MaybeUninit<Self>;

    unsafe fn unaligned_load_from(self: Self, address: *const Self::AssociatedPrimitive);
    unsafe fn unaligned_store_to(self: Self, address: *mut Self::AssociatedPrimitive);
    unsafe fn aligned_load_from(self: Self, address: *const Self::AssociatedPrimitive);
    unsafe fn aligned_store_to(self: Self, address: *mut Self::AssociatedPrimitive);

}

pub trait ComparableVector256: Vector256 {//Mutually exclusive with SSE41Comparable
    //fn cmplt(self: Self, rhs: Self) -> Self;
    //TODO others like the above
}

pub trait FloatVector256: Vector256 + Div + DivAssign + Mul + MulAssign {
    //TODO
    //TODO sqrt, rsqrt, etc.
    //fn movemask(self: Self) -> i32;
}

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

//TODO
