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
use std::marker::PhantomData;

/* Constants */

//TODO

/* Macros */

macro_rules! overload_operator_for {
    ($t: ident, $ops_trait: ident, $trait_function: ident, $auto_ops_trait: ident, $auto_trait_function: ident, $amd64_intrinsic: ident) => {
        impl $ops_trait for $t {
            type Output = Self;

            #[inline(always)]
            fn $trait_function(self: Self, rhs: Self) -> Self {
                return Self {
                    vector: unsafe { x86_64::$amd64_intrinsic(self.vector, rhs.vector) }
                };
            }
        }

        impl $auto_ops_trait for $t {
            #[inline(always)]
            fn $auto_trait_function(self: &mut Self, rhs: Self) {
                self.vector = unsafe { x86_64::$amd64_intrinsic(self.vector, rhs.vector) };
            }
        }
    }
}

/* Static Variables */

//TODO

/* Traits */

pub trait XMM:
    Copy + Clone + Debug +
    From<x86_64::__m128> + From<x86_64::__m128i> + From<x86_64::__m128d> + Into<x86_64::__m128> + Into<x86_64::__m128i> + Into<x86_64::__m128d> +
    Add + AddAssign + BitAnd + BitAndAssign + BitOr + BitOrAssign + BitXor + BitXorAssign + Sub + SubAssign
{
    type ScalarType;
    //type AssociatedPrimitiveArray;

    //fn new_from_array(array: Self::AssociatedPrimitiveArray) -> Self;
    fn new_broadcasted(scalar: Self::ScalarType) -> Self;
    fn new_zeroed() -> Self;
    fn new_uninit() -> MaybeUninit<Self>;

    unsafe fn unaligned_load_from(self: Self, address: *const Self::ScalarType);
    unsafe fn unaligned_store_to(self: Self, address: *mut Self::ScalarType);
    unsafe fn aligned_load_from(self: Self, address: *const Self::ScalarType);
    unsafe fn aligned_store_to(self: Self, address: *mut Self::ScalarType);
}

/* Types */

#[derive(Copy, Clone, Debug)]
#[repr(align(16))]
pub struct XMMS {
    vector: x86_64::__m128
}

#[derive(Copy, Clone, Debug)]
#[repr(align(16))]
pub struct XMMI<ScalarType> {
    vector: x86_64::__m128i,
    _scalar_type: PhantomData<ScalarType>
}

#[derive(Copy, Clone, Debug)]
#[repr(align(16))]
pub struct XMMD {
    vector: x86_64::__m128d
}

/* Associated Functions and Methods */

//XMMS
//TODO

//XMMI
/*
impl<ScalarType, OtherScalarType: !ScalarType> From<XMMI<OtherScalarType>> for &XMMI<ScalarType> {
    #[inline(always)]
    fn from(other: XMMI<OtherScalarType>) -> XMMI<ScalarType> {
        return XMMI<ScalarType> {
            vector: other.vector
        };
    }
}
*/
//TODO

//XMMD
//TODO

/* Functions */

//TODO
