/* avx.rs
 * By: John Jekel
 * Copyright (C) 2022-2023 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * AVX abstractions for Rust
 *
*/

/* Imports */

use core::arch::x86_64;
use std::fmt::Debug;
use std::ops::*;
use std::mem::MaybeUninit;

use super::{overload_operator_for, implement_nicetransmute_for, implement_cast_from_for, implement_cast_into_for};

use super::{F32Vector128, F64Vector128};

//TODO add avx2_emulation submodule?

/* Constants */

pub mod compare_ops {
    //pub use x86_64::_CMP*;
    //TODO
}

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Traits */

pub trait Vector256:
    Copy + Clone + Debug +
    From<x86_64::__m256> + From<x86_64::__m256i> + From<x86_64::__m256d> + Into<x86_64::__m256> + Into<x86_64::__m256i> + Into<x86_64::__m256d> +
    From<Self::AssociatedPrimitiveArray> + Into<Self::AssociatedPrimitiveArray> +
    Add + AddAssign + BitAnd + BitAndAssign + BitOr + BitOrAssign + BitXor + BitXorAssign + Sub + SubAssign
{
    type AssociatedHalf;
    type AssociatedPrimitive;
    type AssociatedPrimitiveArray;

    fn new_broadcasted(scalar: Self::AssociatedPrimitive) -> Self;
    fn new_zeroed() -> Self;
    fn new_uninit() -> MaybeUninit<Self>;

    unsafe fn unaligned_load_from(self: Self, address: *const Self::AssociatedPrimitive);
    unsafe fn unaligned_store_to(self: Self, address: *mut Self::AssociatedPrimitive);
    unsafe fn aligned_load_from(self: Self, address: *const Self::AssociatedPrimitive);
    unsafe fn aligned_store_to(self: Self, address: *mut Self::AssociatedPrimitive);

    fn get_low_half(self: Self) -> Self::AssociatedHalf;
    fn get_high_half(self: Self) -> Self::AssociatedHalf;
}
//TODO what about conversion to/from smaller vectors?

pub trait ComparableVector256: Vector256 {//Mutually exclusive with SSE41Comparable
    fn cmp<const COMPARE_OP: i32>(self: Self, rhs: Self) -> Self;
    //TODO others like the above
}

pub trait FloatVector256: Vector256 + Div + DivAssign + Mul + MulAssign {
    //TODO
    //TODO sqrt, rsqrt, etc.
    fn movemask(self: Self) -> i32;
}

/* Types */

#[derive(Copy, Clone, Debug)]
#[repr(align(32))]
pub struct F32Vector256 {//TODO implement this (low priority)
    vector: x86_64::__m256
}

#[derive(Copy, Clone, Debug)]
#[repr(align(32))]
pub struct F64Vector256 {
    vector: x86_64::__m256d
}
//TODO what about conversion to/from smaller vectors?

/* Associated Functions and Methods */

//F32Vector256
//TODO

//F64Vector256
impl F64Vector256 {
    //TODO
}

impl Vector256 for F64Vector256 {
    type AssociatedHalf = F64Vector128;
    type AssociatedPrimitive = f64;
    type AssociatedPrimitiveArray = [f64; 4];

    #[inline(always)]
    fn new_broadcasted(scalar: f64) -> Self {
        return Self {
            vector: unsafe { x86_64::_mm256_set1_pd(scalar) }
        };
    }

    #[inline(always)]
    fn new_zeroed() -> F64Vector256 {
        return Self {
            vector: unsafe { x86_64::_mm256_setzero_pd() }
        };
    }

    #[inline(always)]
    fn new_uninit() -> MaybeUninit<F64Vector256> {
        return MaybeUninit::new(F64Vector256 {
            vector: unsafe { x86_64::_mm256_undefined_pd() }
        });
    }

    #[inline(always)]
    unsafe fn unaligned_load_from(self: Self, address: *const f64) {
        todo!()
    }

    #[inline(always)]
    unsafe fn unaligned_store_to(self: Self, address: *mut f64) {
        todo!()
    }

    #[inline(always)]
    unsafe fn aligned_load_from(self: Self, address: *const f64) {
        todo!()
    }

    #[inline(always)]
    unsafe fn aligned_store_to(self: Self, address: *mut f64) {
        todo!()
    }

    fn get_low_half(self: Self) -> F64Vector128 {
        return F64Vector128::from(unsafe { x86_64::_mm256_castpd256_pd128(self.vector) });
    }

    fn get_high_half(self: Self) -> F64Vector128 {
        return F64Vector128::from(unsafe { x86_64::_mm256_extractf128_pd(self.vector, 1) });
    }

    //TODO
}

impl FloatVector256 for F64Vector256 {
    #[inline(always)]
    fn movemask(self: Self) -> i32 {
        return unsafe { x86_64::_mm256_movemask_pd(self.vector) };
    }

    //TODO
}

impl ComparableVector256 for F64Vector256 {
    fn cmp<const COMPARE_OP: i32>(self: Self, rhs: Self) -> Self {
        return Self {
            vector: unsafe { x86_64::_mm256_cmp_pd(self.vector, rhs.vector, COMPARE_OP) }
        };
    }
}

impl From<F32Vector256> for F64Vector256 {
    #[inline(always)]
    fn from(other: F32Vector256) -> F64Vector256 {
        /*return F64Vector128 {
            vector: other.into()
        };
        */
        todo!();
    }
}

impl From<[f64; 4]> for F64Vector256 {
    #[inline(always)]
    fn from(array: [f64; 4]) -> F64Vector256 {
        return Self {
            vector: unsafe { x86_64::_mm256_set_pd(array[0], array[1], array[2], array[3]) }
        };
    }
}

impl From<F64Vector256> for [f64; 4] {
    #[inline(always)]
    fn from(vector: F64Vector256) -> [f64; 4] {
        todo!();
    }
}

//TODO what about conversion to/from smaller vectors?
implement_cast_from_for!(F64Vector256, __m256, _mm256_castps_pd);
implement_cast_from_for!(F64Vector256, __m256i, _mm256_castsi256_pd);
implement_cast_into_for!(F64Vector256, __m256, _mm256_castpd_ps);
implement_cast_into_for!(F64Vector256, __m256i, _mm256_castpd_si256);
implement_nicetransmute_for!(F64Vector256, __m256d);

overload_operator_for!(F64Vector256, Add, add, AddAssign, add_assign, _mm256_add_pd);
overload_operator_for!(F64Vector256, BitAnd, bitand, BitAndAssign, bitand_assign, _mm256_and_pd);
overload_operator_for!(F64Vector256, BitOr, bitor, BitOrAssign, bitor_assign, _mm256_or_pd);
overload_operator_for!(F64Vector256, BitXor, bitxor, BitXorAssign, bitxor_assign, _mm256_xor_pd);
overload_operator_for!(F64Vector256, Div, div, DivAssign, div_assign, _mm256_div_pd);
overload_operator_for!(F64Vector256, Mul, mul, MulAssign, mul_assign, _mm256_mul_pd);
overload_operator_for!(F64Vector256, Sub, sub, SubAssign, sub_assign, _mm256_sub_pd);

/* Functions */

//TODO
