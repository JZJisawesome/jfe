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

use super::{overload_operator_for, implement_nicetransmute_for, implement_cast_from_for, implement_cast_into_for};

use super::avx::{Vector256, FloatVector256};

use super::{U64Vector128};//TODO others

/* Constants */

//TODO

/* Macros */

macro_rules! define_integervector256_struct {
    ($t: ident) => {
        //Basic setup of the new struct
        #[derive(Copy, Clone, Debug)]
        #[repr(align(16))]
        pub struct $t {
            vector: x86_64::__m256i
        }

        impl IntegerVector256 for $t {}

        implement_cast_from_for!($t, __m256, _mm256_castps_si256);
        implement_cast_from_for!($t, __m256d, _mm256_castpd_si256);
        implement_cast_into_for!($t, __m256, _mm256_castsi256_ps);
        implement_cast_into_for!($t, __m256d, _mm256_castsi256_pd);
        implement_nicetransmute_for!($t, __m256i);

        impl<OtherT: FloatVector256> From<OtherT> for $t {
            #[inline(always)]
            fn from(other: OtherT) -> $t {
                return $t {
                    vector: other.into()
                };
            }
        }

        //TODO handle From other IntegerVector128 types

        impl<OtherT: IntegerVector256> BitAnd<OtherT> for $t {
            type Output = Self;

            #[inline(always)]
            fn bitand(self: Self, rhs: OtherT) -> Self {
                return Self {
                    vector: unsafe { x86_64::_mm256_and_si256(self.vector, rhs.into()) }
                };
            }
        }

        impl<OtherT: IntegerVector256> BitAndAssign<OtherT> for $t {
            #[inline(always)]
            fn bitand_assign(self: &mut Self, rhs: OtherT) {
                self.vector = unsafe { x86_64::_mm256_and_si256(self.vector, rhs.into()) };
            }
        }

        impl<OtherT: IntegerVector256> BitOr<OtherT> for $t {
            type Output = Self;

            #[inline(always)]
            fn bitor(self: Self, rhs: OtherT) -> Self {
                return Self {
                    vector: unsafe { x86_64::_mm256_or_si256(self.vector, rhs.into()) }
                };
            }
        }

        impl<OtherT: IntegerVector256> BitOrAssign<OtherT> for $t {
            #[inline(always)]
            fn bitor_assign(self: &mut Self, rhs: OtherT) {
                self.vector = unsafe { x86_64::_mm256_or_si256(self.vector, rhs.into()) };
            }
        }

        impl<OtherT: IntegerVector256> BitXor<OtherT> for $t {
            type Output = Self;

            #[inline(always)]
            fn bitxor(self: Self, rhs: OtherT) -> Self {
                return Self {
                    vector: unsafe { x86_64::_mm256_xor_si256(self.vector, rhs.into()) }
                };
            }
        }

        impl<OtherT: IntegerVector256> BitXorAssign<OtherT> for $t {
            #[inline(always)]
            fn bitxor_assign(self: &mut Self, rhs: OtherT) {
                self.vector = unsafe { x86_64::_mm256_xor_si256(self.vector, rhs.into()) };
            }
        }
    }
}

macro_rules! common_impl_vector256_function_implementations_for_integervector256 {
    () => {
        #[inline(always)]
        fn new_zeroed() -> Self {
            return Self {
                vector: unsafe { x86_64::_mm256_setzero_si256() }
            };
        }

        #[inline(always)]
        fn new_uninit() -> MaybeUninit<Self> {
            return MaybeUninit::new(Self {
                vector: unsafe { x86_64::_mm256_undefined_si256() }
            });
        }

        #[inline(always)]
        unsafe fn unaligned_load_from(self: Self, address: *const Self::AssociatedPrimitive) {
            todo!()
        }

        #[inline(always)]
        unsafe fn unaligned_store_to(self: Self, address: *mut Self::AssociatedPrimitive) {
            x86_64::_mm256_storeu_si256(address as *mut x86_64::__m256i, self.vector);
        }

        #[inline(always)]
        unsafe fn aligned_load_from(self: Self, address: *const Self::AssociatedPrimitive) {
            todo!()
        }

        #[inline(always)]//TODO boilerplate
        unsafe fn aligned_store_to(self: Self, address: *mut Self::AssociatedPrimitive) {
            x86_64::_mm256_store_si256(address as *mut x86_64::__m256i, self.vector);
        }

        fn get_low_half(self: Self) -> Self::AssociatedHalf {
            return Self::AssociatedHalf::from(unsafe { x86_64::_mm256_castsi256_si128(self.vector) });
        }

        fn get_high_half(self: Self) -> Self::AssociatedHalf {
            return Self::AssociatedHalf::from(unsafe { x86_64::_mm256_extracti128_si256(self.vector, 1) });
        }
    }
}

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

//TODO will still need to implement add, subtract, multiply, divide, shl, shr, etc. for each of these
/*define_integer_vector128_struct!(I8Vector128);//TODO this also supports fn movemask(self: Self) -> i32;
define_integervector256_struct!(I16Vector256);
define_integervector256_struct!(I32Vector256);
define_integervector256_struct!(I64Vector256);*/

/*define_integervector256_struct!(U8Vector256);//TODO this also supports fn movemask(self: Self) -> i32;
define_integervector256_struct!(U16Vector256);
define_integervector256_struct!(U32Vector256);*/
define_integervector256_struct!(U64Vector256);

/* Associated Functions and Methods */

//TODO others

//U64Vector256
impl Vector256 for U64Vector256 {
    type AssociatedHalf = U64Vector128;
    type AssociatedPrimitive = u64;
    type AssociatedPrimitiveArray = [u64; 4];

    common_impl_vector256_function_implementations_for_integervector256!();

    #[inline(always)]
    fn new_broadcasted(scalar: u64) -> U64Vector256 {
        return U64Vector256 {
            vector: unsafe { x86_64::_mm256_set1_epi64x(scalar as i64) }
        }
    }

    //TODO
}

impl From<[u64; 4]> for U64Vector256 {
    #[inline(always)]
    fn from(array: [u64; 4]) -> U64Vector256 {
        return U64Vector256 {
            vector: unsafe { x86_64::_mm256_set_epi64x(array[0] as i64, array[1] as i64, array[2] as i64, array[3] as i64) }
        };
    }
}

impl From<U64Vector256> for [u64; 4] {
    #[inline(always)]
    fn from(vector: U64Vector256) -> [u64; 4] {
        todo!();
    }
}

impl ShiftableVector256 for U64Vector256 {
    //TODO
}

overload_operator_for!(U64Vector256, Add, add, AddAssign, add_assign, _mm256_add_epi64);
overload_operator_for!(U64Vector256, Sub, sub, SubAssign, sub_assign, _mm256_sub_epi64);
overload_operator_for!(U64Vector256, Shl, shl, ShlAssign, shl_assign, _mm256_sllv_epi64);
overload_operator_for!(U64Vector256, Shr, shr, ShrAssign, shr_assign, _mm256_srlv_epi64);//Logical right shift since this is unsigned

/* Functions */

//TODO
