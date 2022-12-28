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

/* Constants */

//TODO

/* Macros */

macro_rules! define_integer_vector128_struct_with_primitive {
    ($t: ident, $primitive: ident) => (
        //Basic setup of the new struct
        #[derive(Copy, Clone, Debug)]
        pub struct $t {
            vector: x86_64::__m128i
        }

        //Traits
        impl Vector128 for $t {
            type AssociatedPrimitive = $primitive;

            #[inline(always)]
            fn new_zeroed() -> $t {
                return Self {
                    vector: unsafe { x86_64::_mm_setzero_si128() }
                };
            }

            //TODO
        }

        impl IntegerVector128 for $t {
            //TODO
        }

        //Conversion functions
        impl From<x86_64::__m128> for $t {
            #[inline(always)]
            fn from(raw: x86_64::__m128) -> $t {
                return $t {
                    vector: unsafe { x86_64::_mm_castps_si128(raw) }
                };
            }
        }

        impl From<x86_64::__m128i> for $t {
            #[inline(always)]
            fn from(raw: x86_64::__m128i) -> $t {
                return $t {
                    vector: raw
                };
            }
        }

        impl From<x86_64::__m128d> for $t {
            #[inline(always)]
            fn from(raw: x86_64::__m128d) -> $t {
                return $t {
                    vector: unsafe { x86_64::_mm_castpd_si128(raw) }
                };
            }
        }

        impl Into<x86_64::__m128> for $t {
            #[inline(always)]
            fn into(self: Self) -> x86_64::__m128 {
                return unsafe { x86_64::_mm_castsi128_ps(self.vector) };
            }
        }

        impl Into<x86_64::__m128i> for $t {
            #[inline(always)]
            fn into(self: Self) -> x86_64::__m128i {
                return self.vector;
            }
        }

        impl Into<x86_64::__m128d> for $t {
            #[inline(always)]
            fn into(self: Self) -> x86_64::__m128d {
                return unsafe { x86_64::_mm_castsi128_pd(self.vector) };
            }
        }

        impl AsRef<x86_64::__m128i> for $t {
            #[inline(always)]
            fn as_ref(self: &Self) -> &x86_64::__m128i {
                return &self.vector;
            }
        }

        impl AsMut<x86_64::__m128i> for $t {
            #[inline(always)]
            fn as_mut(self: &mut Self) -> &mut x86_64::__m128i {
                return &mut self.vector;
            }
        }

        //Operator overloading
        impl BitAnd for $t {
            type Output = Self;

            #[inline(always)]
            fn bitand(self: Self, rhs: Self) -> Self {
                return Self {
                    vector: unsafe { x86_64::_mm_and_si128(self.vector.into(), rhs.vector.into()) }
                };
            }
        }

        impl BitAndAssign for $t {
            #[inline(always)]
            fn bitand_assign(self: &mut Self, rhs: Self) {
                self.vector = unsafe { x86_64::_mm_and_si128(self.vector.into(), rhs.vector.into()) };
            }
        }

        impl BitOr for $t {
            type Output = Self;

            #[inline(always)]
            fn bitor(self: Self, rhs: Self) -> Self {
                return Self {
                    vector: unsafe { x86_64::_mm_or_si128(self.vector.into(), rhs.vector.into()) }
                };
            }
        }

        impl BitOrAssign for $t {
            #[inline(always)]
            fn bitor_assign(self: &mut Self, rhs: Self) {
                self.vector = unsafe { x86_64::_mm_or_si128(self.vector.into(), rhs.vector.into()) };
            }
        }

        impl BitXor for $t {
            type Output = Self;

            #[inline(always)]
            fn bitxor(self: Self, rhs: Self) -> Self {
                return Self {
                    vector: unsafe { x86_64::_mm_xor_si128(self.vector.into(), rhs.vector.into()) }
                };
            }
        }

        impl BitXorAssign for $t {
            #[inline(always)]
            fn bitxor_assign(self: &mut Self, rhs: Self) {
                self.vector = unsafe { x86_64::_mm_xor_si128(self.vector.into(), rhs.vector.into()) };
            }
        }
    )
}

/* Static Variables */

//TODO

/* Traits */

pub trait Vector128:
    Copy + Clone + Debug +
    From<x86_64::__m128> + From<x86_64::__m128i> + From<x86_64::__m128d> + Into<x86_64::__m128> + Into<x86_64::__m128i> + Into<x86_64::__m128d> /*+
    Add + AddAssign + BitAnd + BitAndAssign + BitOr + BitOrAssign + BitXor + BitXorAssign + Div + DivAssign + Mul + MulAssign + Sub + SubAssign*/
{
    type AssociatedPrimitive;

    fn new_zeroed() -> Self;

    /*fn new_from_broadcasted(scaler: AssociatedPrimitive) -> Self;
    unsafe fn unaligned_store_to<T>(self: Self, address: *mut T);*/

    //fn cmpeq(self: Self, rhs: Self) -> Self;
    //TODO others like the above
}

pub trait IntegerVector128:
    Vector128 + AsRef<x86_64::__m128i> + AsMut<x86_64::__m128i> /*+ Shl + ShlAssign + Shr + ShrAssign*/
{
    //TODO
}

/* Types */

#[derive(Copy, Clone, Debug)]
pub struct F32Vector128 {//TODO implement this (low priority)//TODO this will be Vector128 + AsRef<x86_64::__m128> + AsMut<x86_64::__m128>
    vector: x86_64::__m128
}

#[derive(Copy, Clone, Debug)]
pub struct F64Vector128 {//TODO this will be Vector128 + AsRef<x86_64::__m128d> + AsMut<x86_64::__m128d>
    vector: x86_64::__m128d
}

//TODO will still need to implement add, subtract, multiply, divide, shl, shr, etc. for each of these
define_integer_vector128_struct_with_primitive!(U8Vector128, u8);
define_integer_vector128_struct_with_primitive!(U16Vector128, u16);
define_integer_vector128_struct_with_primitive!(U32Vector128, u32);
define_integer_vector128_struct_with_primitive!(U64Vector128, u64);

/* Associated Functions and Methods */

//F32Vector128
//TODO

//F64Vector128
impl Vector128 for F64Vector128 {
    type AssociatedPrimitive = f64;

    #[inline(always)]
    fn new_zeroed() -> F64Vector128 {
        return Self {
            vector: unsafe { x86_64::_mm_setzero_pd() }
        };
    }

    //TODO
}

impl From<x86_64::__m128> for F64Vector128 {
    #[inline(always)]
    fn from(raw: x86_64::__m128) -> F64Vector128 {
        return F64Vector128 {
            vector: unsafe { x86_64::_mm_castps_pd(raw) }
        };
    }
}

impl From<x86_64::__m128i> for F64Vector128 {
    #[inline(always)]
    fn from(raw: x86_64::__m128i) -> F64Vector128 {
        return F64Vector128 {
            vector: unsafe { x86_64::_mm_castsi128_pd(raw) }
        };
    }
}

impl From<x86_64::__m128d> for F64Vector128 {
    #[inline(always)]
    fn from(raw: x86_64::__m128d) -> F64Vector128 {
        return F64Vector128 {
            vector: raw
        };
    }
}

impl Into<x86_64::__m128> for F64Vector128 {
    #[inline(always)]
    fn into(self: Self) -> x86_64::__m128 {
        return unsafe { x86_64::_mm_castpd_ps(self.vector) };
    }
}

impl Into<x86_64::__m128i> for F64Vector128 {
    #[inline(always)]
    fn into(self: Self) -> x86_64::__m128i {
        return unsafe { x86_64::_mm_castpd_si128(self.vector) };
    }
}

impl Into<x86_64::__m128d> for F64Vector128 {
    #[inline(always)]
    fn into(self: Self) -> x86_64::__m128d {
        return self.vector;
    }
}

impl AsRef<x86_64::__m128d> for F64Vector128 {
    #[inline(always)]
    fn as_ref(self: &Self) -> &x86_64::__m128d {
        return &self.vector;
    }
}

impl AsMut<x86_64::__m128d> for F64Vector128 {
    #[inline(always)]
    fn as_mut(self: &mut Self) -> &mut x86_64::__m128d {
        return &mut self.vector;
    }
}

/* Functions */

//TODO
