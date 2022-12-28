/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

fn test() {
    let the_test = U64Vector128::from_workaround(F64Vector128::new_zeroed());
}

/* Imports */

use core::arch::x86_64;
use std::fmt::Debug;
use std::ops::*;

/* Constants */

//TODO

/* Macros */

macro_rules! define_vector128_struct_called {
    ($t: ident) => (
        //Basic setup of the new struct
        #[derive(Copy, Clone, Debug)]
        pub struct $t {
            vector: x86_64::__m128i
        }

        impl Vector128 for $t {}

        //Conversion functions/trait implementations
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

        //FIXME This dosn't work since we conflict with the implementation in core:: (it would be nice if we had specialization)
        /*impl<T: Vector128> From<T> for $t {//This is how we can convert between Vector128s freely
            #[inline(always)]
            fn from(other: T) -> Self {
                unsafe {
                    let raw: x86_64::__m128i = other.into();
                    return Self::from(raw);
                }
            }
        }
        */
        //Workaround (will be removed eventually)
        impl $t {
            pub fn from_workaround(other: impl Vector128) -> Self {
                //Convert to __m128i and back
                let raw: x86_64::__m128i = other.into();
                return Self::from(raw);
            }

            pub fn into_workaround<T: Vector128>(self: Self) -> T {
                //Convert to __m128i and back
                let raw: x86_64::__m128i = self.into();
                return T::from(raw);
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
                unsafe {
                    self.vector = unsafe { x86_64::_mm_and_si128(self.vector.into(), rhs.vector.into()) };
                }
            }
        }

        //TODO add other universal traits
    )
}

/* Static Variables */

//TODO

/* Traits */

pub trait Vector128: Copy + Clone + Debug + From<x86_64::__m128> + From<x86_64::__m128i> + From<x86_64::__m128d> + Into<x86_64::__m128> + Into<x86_64::__m128i> + Into<x86_64::__m128d> + BitAnd + BitAndAssign {//TODO add other universal traits
    #[inline(always)]
    fn new_zeroed() -> Self {
        return Self::from(unsafe { x86_64::_mm_setzero_si128() });
    }

    #[inline(always)]
    unsafe fn unaligned_store_to<T>(self: Self, address: *mut T) {
        x86_64::_mm_storeu_si128(address as *mut x86_64::__m128i, self.into());
    }
}

/* Types */

define_vector128_struct_called!(U64Vector128);
define_vector128_struct_called!(F64Vector128);

/* Associated Functions and Methods */

//TODO

/* Functions */

//TODO
