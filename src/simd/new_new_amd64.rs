/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/


fn test() {
    let the_test1 = U64Vector128::from_workaround(F64Vector128::new_zeroed());
    let the_test2 = F64Vector128::from_workaround(U64Vector128::new_zeroed());
    let the_test3: F64Vector128 = U64Vector128::new_zeroed().into_workaround();
    let the_test4: U64Vector128 = F64Vector128::new_zeroed().into_workaround();

    //This works and avoids the workaround, but can't be used outside of this module...
    let the_test5 = U64Vector128::from(Into::<Raw128>::into(F64Vector128::new_zeroed()));
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
        #[derive(Copy, Clone, Debug)]
        pub struct $t {
            vector: Raw128
        }

        /* BackedByRaw128 implementations for conversion */

        impl BackedByRaw128 for $t {}

        impl From<Raw128> for $t {
            #[inline(always)]
            fn from(raw: Raw128) -> Self {
                unsafe {
                    return Self {
                        vector: raw
                    };
                }
            }
        }

        impl Into<Raw128> for $t {
            #[inline(always)]
            fn into(self: Self) -> Raw128 {
                unsafe {
                    return self.vector;
                }
            }
        }

        /*
        //FIXME This dosn't work since we conflict with the base implementation (it would be nice if we had specialization)
        impl<T: BackedByRaw128> From<T> for $t {//This is how we can convert between Vector128s freely
            #[inline(always)]
            fn from(other: T) -> Self {
                unsafe {
                    return Self::from(other.into());
                }
            }
        }
        */
        //Workaround
        impl $t {
            //TODO figure out how to make these public without exposing BackedByRaw128

            fn from_workaround(other: impl BackedByRaw128) -> Self {
                return Self::from(other.into());//Convert to Raw128 and back
            }

            fn into_workaround<T: BackedByRaw128>(self: Self) -> T {
                return T::from(self.into());//Convert to Raw128 and back
            }
        }

        impl Vector128 for $t {
            #[inline(always)]
            fn new_zeroed() -> Self {
                unsafe {
                    //Even if this vector will be accessed like a floating point one, it is okay since the representation of 0.0 is also all zeroes!
                    return Self::from(Raw128::from(x86_64::_mm_setzero_si128()));
                }
            }
        }

        impl BitAnd for $t {
            type Output = Self;

            #[inline(always)]
            fn bitand(self: Self, rhs: Self) -> Self {
                unsafe {
                    return Self {
                        vector: Raw128::from(x86_64::_mm_and_si128(self.vector.into(), rhs.vector.into()))
                    };
                }
            }
        }

        impl BitAndAssign for $t {
            #[inline(always)]
            fn bitand_assign(self: &mut Self, rhs: Self) {
                unsafe {
                    self.vector = Raw128::from(x86_64::_mm_and_si128(self.vector.into(), rhs.vector.into()));
                }
            }
        }

        //TODO what about conversion to other Vector128 types

        //TODO add other universal traits
    )
}

/* Static Variables */

//TODO

/* Traits */

pub trait Vector128: Copy + Clone + Debug + BitAnd + BitAndAssign {//TODO add other universal traits
    fn new_zeroed() -> Self;
}

trait BackedByRaw128: Copy + Clone + Debug + From<Raw128> + Into<Raw128> {}

/* Types */

#[derive(Copy, Clone, Debug)]
struct Raw128 {//x86 intrinsic types are sorta dumb (since they're all the same registers), so abstract them away
    vector: x86_64::__m128i
}

define_vector128_struct_called!(U64Vector128);
define_vector128_struct_called!(F64Vector128);

/* Associated Functions and Methods */

//Raw128
impl From<x86_64::__m128> for Raw128 {
    #[inline(always)]
    fn from(bare_raw: x86_64::__m128) -> Raw128 {
        unsafe {
            return Raw128 {
                vector: x86_64::_mm_castps_si128(bare_raw)
            };
        }
    }
}

impl From<x86_64::__m128i> for Raw128 {
    #[inline(always)]
    fn from(bare_raw: x86_64::__m128i) -> Raw128 {
        unsafe {
            return Raw128 {
                vector: bare_raw
            };
        }
    }
}

impl From<x86_64::__m128d> for Raw128 {
    #[inline(always)]
    fn from(bare_raw: x86_64::__m128d) -> Raw128 {
        unsafe {
            return Raw128 {
                vector: x86_64::_mm_castpd_si128(bare_raw)
            };
        }
    }
}

impl From<Raw128> for x86_64::__m128 {
    #[inline(always)]
    fn from(wrapper: Raw128) -> x86_64::__m128 {
        unsafe {
            return x86_64::_mm_castsi128_ps(wrapper.vector);
        }
    }
}

impl From<Raw128> for x86_64::__m128i {
    #[inline(always)]
    fn from(wrapper: Raw128) -> x86_64::__m128i {
        unsafe {
            return wrapper.vector;
        }
    }
}

impl From<Raw128> for x86_64::__m128d {
    #[inline(always)]
    fn from(wrapper: Raw128) -> x86_64::__m128d {
        unsafe {
            return x86_64::_mm_castsi128_pd(wrapper.vector);
        }
    }
}

/* Functions */

//TODO
