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

macro_rules! overload_operator_for {
    ($t: ident, $ops_trait: ident, $trait_function: ident, $amd64_intrinsic: ident) => {
        impl $ops_trait for $t {
            type Output = Self;

            #[inline(always)]
            fn $trait_function(self: Self, rhs: Self) -> Self {
                return Self {
                    vector: unsafe { x86_64::$amd64_intrinsic(self.vector, rhs.vector) }
                };
            }
        }
    }
}

macro_rules! overload_autoassignment_operator_for {
    ($t: ident, $ops_trait: ident, $trait_function: ident, $amd64_intrinsic: ident) => {
        impl $ops_trait for $t {
            #[inline(always)]
            fn $trait_function(self: &mut Self, rhs: Self) {
                self.vector = unsafe { x86_64::$amd64_intrinsic(self.vector, rhs.vector) };
            }
        }
    }
}

macro_rules! implement_nicetransmute_for {
    ($t: ident, $amd64_intrinsic_t: ident) => {
        impl From<x86_64::$amd64_intrinsic_t> for $t {
            #[inline(always)]
            fn from(raw: x86_64::$amd64_intrinsic_t) -> $t {
                return $t {
                    vector: raw
                };
            }
        }

        impl Into<x86_64::$amd64_intrinsic_t> for $t {
            #[inline(always)]
            fn into(self: Self) -> x86_64::$amd64_intrinsic_t {
                return self.vector;
            }
        }

        impl AsRef<x86_64::$amd64_intrinsic_t> for $t {
            #[inline(always)]
            fn as_ref(self: &Self) -> &x86_64::$amd64_intrinsic_t {
                return &self.vector;
            }
        }

        impl AsMut<x86_64::$amd64_intrinsic_t> for $t {
            #[inline(always)]
            fn as_mut(self: &mut Self) -> &mut x86_64::$amd64_intrinsic_t {
                return &mut self.vector;
            }
        }
    }
}

macro_rules! implement_cast_from_for {
    ($t: ident, $amd64_intrinsic_t: ident, $amd64_intrinsic: ident) => {
        impl From<x86_64::$amd64_intrinsic_t> for $t {
            #[inline(always)]
            fn from(raw: x86_64::$amd64_intrinsic_t) -> $t {
                return $t {
                    vector: unsafe { x86_64::$amd64_intrinsic(raw) }
                };
            }
        }
    }
}

macro_rules! implement_cast_into_for {
    ($t: ident, $amd64_intrinsic_t: ident, $amd64_intrinsic: ident) => {
        impl Into<x86_64::$amd64_intrinsic_t> for $t {
            #[inline(always)]
            fn into(self: Self) -> x86_64::$amd64_intrinsic_t {
                return unsafe { x86_64::$amd64_intrinsic(self.vector) };
            }
        }
    }
}

macro_rules! define_integervector128_struct_with_primitive {
    ($t: ident, $primitive: ident) => {
        //Basic setup of the new struct
        #[derive(Copy, Clone, Debug)]
        #[repr(align(16))]
        pub struct $t {
            vector: x86_64::__m128i
        }

        impl IntegerVector128 for $t {}

        implement_cast_from_for!($t, __m128, _mm_castps_si128);
        implement_cast_from_for!($t, __m128d, _mm_castpd_si128);
        implement_cast_into_for!($t, __m128, _mm_castsi128_ps);
        implement_cast_into_for!($t, __m128d, _mm_castsi128_pd);
        implement_nicetransmute_for!($t, __m128i);

        overload_operator_for!($t, BitAnd, bitand, _mm_and_si128);
        overload_autoassignment_operator_for!($t, BitAndAssign, bitand_assign, _mm_and_si128);
        overload_operator_for!($t, BitOr, bitor, _mm_or_si128);
        overload_autoassignment_operator_for!($t, BitOrAssign, bitor_assign, _mm_or_si128);
        overload_operator_for!($t, BitXor, bitxor, _mm_xor_si128);
        overload_autoassignment_operator_for!($t, BitXorAssign, bitxor_assign, _mm_xor_si128);
    }
}

macro_rules! common_impl_vector128_function_implementations_for_integervector128 {
    () => {
        #[inline(always)]
        fn new_zeroed() -> Self {
            return Self {
                vector: unsafe { x86_64::_mm_setzero_si128() }
            };
        }

        #[inline(always)]
        fn new_uninit() -> MaybeUninit<Self> {
            return MaybeUninit::new(Self {
                vector: unsafe { x86_64::_mm_undefined_si128() }
            });
        }

        #[inline(always)]
        unsafe fn unaligned_load_from(self: Self, address: *const Self::AssociatedPrimitive) {
            todo!()
        }

        #[inline(always)]
        unsafe fn unaligned_store_to(self: Self, address: *mut Self::AssociatedPrimitive) {
            unsafe { x86_64::_mm_storeu_si128(address as *mut x86_64::__m128i, self.vector); }
        }

        #[inline(always)]
        unsafe fn aligned_load_from(self: Self, address: *const Self::AssociatedPrimitive) {
            todo!()
        }

        #[inline(always)]//TODO boilerplate
        unsafe fn aligned_store_to(self: Self, address: *mut Self::AssociatedPrimitive) {
            unsafe { x86_64::_mm_store_si128(address as *mut x86_64::__m128i, self.vector); }
        }
    }
}

/* Static Variables */

//TODO

/* Traits */

pub trait Vector128:
    Copy + Clone + Debug +
    From<x86_64::__m128> + From<x86_64::__m128i> + From<x86_64::__m128d> + Into<x86_64::__m128> + Into<x86_64::__m128i> + Into<x86_64::__m128d> +
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

pub trait Comparable: Vector128 {//Mutually exclusive with SSE41Comparable
    //fn cmpeq(self: Self, rhs: Self) -> Self;
    //TODO others like the above
}

pub trait FloatVector128: Vector128 + Div + DivAssign + Mul + MulAssign {
    //TODO
    //TODO sqrt, rsqrt, etc.
    fn movemask(self: Self) -> i32;
}

pub trait IntegerVector128: Vector128 + AsRef<x86_64::__m128i> + AsMut<x86_64::__m128i> {}//Markerish trait

//TODO on the I* vectors, Shr will be arithmetic, but it will be logical on the U* vectors
//NOTE: U8 and I8 aren't shiftable
pub trait Shiftable: IntegerVector128 + Shl + ShlAssign + Shr + ShrAssign {
    //TODO

    //TODO perhaps have I8, I16, I32, I64 so that we do arithmetic shifts on that, and regular shifs on the U versions?
    //fn shri<const AMOUNT: i32>(self: Self) -> Self;
    //fn shli<const AMOUNT: i32>(self: Self) -> Self;
}

//These feature traits below are to help protect the user from using microarch features unintentionally

//Only U64Vector128 implements this trait, so we can guarantee IntegerVector128 + Shiftable
pub trait SSE41Comparable: IntegerVector128 + Shiftable {//Mutually exclusive with Comparable
    //fn cmpeq(self: Self, rhs: Self) -> Self;
    //TODO others like the above
    //Should have the same things as Comparable
}

pub trait SSE41CommonFloatFeatures: FloatVector128 {
    //TODO These are addsub, blend, blendv, ceil, dp, floor, hadd, hsub, round
}



/* Types */

#[derive(Copy, Clone, Debug)]
#[repr(align(16))]
pub struct F32Vector128 {//TODO implement this (low priority)//TODO this will be Vector128 + AsRef<x86_64::__m128> + AsMut<x86_64::__m128>
    vector: x86_64::__m128
}

#[derive(Copy, Clone, Debug)]
#[repr(align(16))]
pub struct F64Vector128 {//TODO this will be Vector128 + AsRef<x86_64::__m128d> + AsMut<x86_64::__m128d>
    vector: x86_64::__m128d
}

//TODO will still need to implement add, subtract, multiply, divide, shl, shr, etc. for each of these
/*define_integer_vector128_struct_with_primitive!(I8Vector128, i8);//TODO this also supports fn movemask(self: Self) -> i32;
define_integervector128_struct_with_primitive!(I16Vector128, i16);
define_integervector128_struct_with_primitive!(I32Vector128, i32);
define_integervector128_struct_with_primitive!(I64Vector128, i64);*/

define_integervector128_struct_with_primitive!(U8Vector128, u8);
/*
define_integervector128_struct_with_primitive!(U16Vector128, u16);
define_integervector128_struct_with_primitive!(U32Vector128, u32);*/
define_integervector128_struct_with_primitive!(U64Vector128, u64);

/* Associated Functions and Methods */

//F32Vector128
//TODO

//F64Vector128
impl F64Vector128 {
    //TODO
}

impl Vector128 for F64Vector128 {
    type AssociatedPrimitive = f64;
    type AssociatedPrimitiveArray = [f64; 2];

    #[inline(always)]
    fn new_from_array(array: [f64; 2]) -> Self {
        return Self {
            vector: unsafe { x86_64::_mm_set_pd(array[1], array[0]) }
        };
    }

    #[inline(always)]
    fn new_broadcasted(scalar: f64) -> Self {
        return Self {
            vector: unsafe { x86_64::_mm_set1_pd(scalar) }
        };
    }

    #[inline(always)]
    fn new_zeroed() -> F64Vector128 {
        return Self {
            vector: unsafe { x86_64::_mm_setzero_pd() }
        };
    }

    #[inline(always)]
    fn new_uninit() -> MaybeUninit<F64Vector128> {
        return MaybeUninit::new(F64Vector128 {
            vector: unsafe { x86_64::_mm_undefined_pd() }
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


    //TODO
}

impl FloatVector128 for F64Vector128 {
    #[inline(always)]
    fn movemask(self: Self) -> i32 {
        todo!();
    }

    //TODO
}

implement_cast_from_for!(F64Vector128, __m128, _mm_castps_pd);
implement_cast_from_for!(F64Vector128, __m128i, _mm_castsi128_pd);
implement_cast_into_for!(F64Vector128, __m128, _mm_castpd_ps);
implement_cast_into_for!(F64Vector128, __m128i, _mm_castpd_si128);
implement_nicetransmute_for!(F64Vector128, __m128d);

overload_operator_for!(F64Vector128, Add, add, _mm_add_pd);
overload_autoassignment_operator_for!(F64Vector128, AddAssign, add_assign, _mm_add_pd);
overload_operator_for!(F64Vector128, BitAnd, bitand, _mm_and_pd);
overload_autoassignment_operator_for!(F64Vector128, BitAndAssign, bitand_assign, _mm_and_pd);
overload_operator_for!(F64Vector128, BitOr, bitor, _mm_or_pd);
overload_autoassignment_operator_for!(F64Vector128, BitOrAssign, bitor_assign, _mm_or_pd);
overload_operator_for!(F64Vector128, BitXor, bitxor, _mm_xor_pd);
overload_autoassignment_operator_for!(F64Vector128, BitXorAssign, bitxor_assign, _mm_xor_pd);
overload_operator_for!(F64Vector128, Div, div, _mm_div_pd);
overload_autoassignment_operator_for!(F64Vector128, DivAssign, div_assign, _mm_div_pd);
overload_operator_for!(F64Vector128, Mul, mul, _mm_mul_pd);
overload_autoassignment_operator_for!(F64Vector128, MulAssign, mul_assign, _mm_mul_pd);
overload_operator_for!(F64Vector128, Sub, sub, _mm_div_pd);
overload_autoassignment_operator_for!(F64Vector128, SubAssign, sub_assign, _mm_sub_pd);

//I8Vector128
//TODO
/*
impl I8Vector128 {
    #[inline(always)]
    fn movemask(self: Self) -> i32 {
        return unsafe { x86_64::_mm_movemask_epi8(self.vector) };
    }
}
*/

//I16Vector128
//TODO

//I32Vector128
//TODO

//I64Vector128
//TODO

//U8Vector128
impl U8Vector128 {
    #[inline(always)]
    fn movemask(self: Self) -> i32 {
        return unsafe { x86_64::_mm_movemask_epi8(self.vector) };
    }
}

impl Vector128 for U8Vector128 {
    type AssociatedPrimitive = u8;
    type AssociatedPrimitiveArray = [u8; 16];

    common_impl_vector128_function_implementations_for_integervector128!();

    #[inline(always)]
    fn new_from_array(array: [u8; 16]) -> Self {
        return Self {
            vector: unsafe {
                x86_64::_mm_set_epi8(
                    array[15] as i8, array[14] as i8, array[13] as i8, array[12] as i8,
                    array[11] as i8, array[10] as i8, array[9] as i8, array[8] as i8,
                    array[7] as i8, array[6] as i8, array[5] as i8, array[4] as i8,
                    array[3] as i8, array[2] as i8, array[1] as i8, array[0] as i8
                )
            }
        };
    }

    #[inline(always)]
    fn new_broadcasted(scalar: u8) -> U8Vector128 {
        return U8Vector128 {
            vector: unsafe { x86_64::_mm_set1_epi8(scalar as i8) }
        }
    }

    //TODO
}

overload_operator_for!(U8Vector128, Add, add, _mm_add_epi8);
overload_autoassignment_operator_for!(U8Vector128, AddAssign, add_assign, _mm_add_epi8);
overload_operator_for!(U8Vector128, Sub, sub, _mm_sub_epi8);
overload_autoassignment_operator_for!(U8Vector128, SubAssign, sub_assign, _mm_sub_epi8);

//U16Vector128
//TODO

//U32Vector128
//TODO

//U64Vector128
impl Vector128 for U64Vector128 {
    type AssociatedPrimitive = u64;
    type AssociatedPrimitiveArray = [u64; 2];

    common_impl_vector128_function_implementations_for_integervector128!();

    #[inline(always)]
    fn new_from_array(array: [u64; 2]) -> Self {
        return Self {
            vector: unsafe { x86_64::_mm_set_epi64x(array[1] as i64, array[0] as i64) }
        };
    }

    #[inline(always)]
    fn new_broadcasted(scalar: u64) -> U64Vector128 {
        return U64Vector128 {
            vector: unsafe { x86_64::_mm_set1_epi64x(scalar as i64) }
        }
    }

    //TODO
}

impl Shiftable for U64Vector128 {
    //TODO
}

overload_operator_for!(U64Vector128, Add, add, _mm_add_epi64);
overload_autoassignment_operator_for!(U64Vector128, AddAssign, add_assign, _mm_add_epi64);
overload_operator_for!(U64Vector128, Sub, sub, _mm_sub_epi64);
overload_autoassignment_operator_for!(U64Vector128, SubAssign, sub_assign, _mm_sub_epi64);
overload_operator_for!(U64Vector128, Shl, shl, _mm_sll_epi64);
overload_autoassignment_operator_for!(U64Vector128, ShlAssign, shl_assign, _mm_sll_epi64);
overload_operator_for!(U64Vector128, Shr, shr, _mm_srl_epi64);//Logical right shift since this is unsigned
overload_autoassignment_operator_for!(U64Vector128, ShrAssign, shr_assign, _mm_srl_epi64);//Logical right shift since this is unsigned

/* Functions */

//TODO
