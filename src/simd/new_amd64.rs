/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use core::arch::x86_64;

use std::fmt::Debug;

use std::convert::From;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

#[derive(Copy, Clone, Debug)]
pub struct U64Vector128 {
    pub vector: x86_64::__m128i
}

#[derive(Copy, Clone, Debug)]
pub struct F64Vector128 {
    pub vector: x86_64::__m128d
}

#[derive(Copy, Clone, Debug)]
pub struct U64Vector256 {
    pub vector: x86_64::__m256i
}

#[derive(Copy, Clone, Debug)]
pub struct F64Vector256 {
    pub vector: x86_64::__m256d
}

/* Traits */

pub trait Vector128: Copy + Clone + Debug + From<x86_64::__m128> + From<x86_64::__m128i> + From<x86_64::__m128d> {
    /*fn as_m128(self: Self) -> x86_64::__m128;
    fn as_m128i(self: Self) -> x86_64::__m128i;
    fn as_m128d(self: Self) -> x86_64::__m128d;
    */
    //TODO
}

pub trait Vector256: Copy + Clone + Debug {
    //TODO
}

/* Associated Functions and Methods */

//Vector128
//FIXME not allowed to do this which sucks!
//See https://github.com/rust-lang/rfcs/issues/1124
/*impl<T: Vector128> std::ops::BitAnd<T> for T {
{
    type Output = T;

    #[inline(always)]
    fn bitand(self: Self, rhs: T) -> T {
        unsafe {
            return T::from(x86_64::_mm_and_si128(self.into(), rhs.into()));
        }
    }
}
*/
//TODO do a derive macro instead?
macro_rules! vector128_impl_bitand_for {
    ($t: ident) => (
        impl std::ops::BitAnd for $t {
            type Output = Self;

            #[inline(always)]
            fn bitand(self: Self, rhs: Self) -> Self {
                unsafe {
                    return Self::from(x86_64::_mm_and_si128(self.into(), self.into()));
                }
            }
        }
    )
}
vector128_impl_bitand_for!(U64Vector128);


//U64Vector128
impl Vector128 for U64Vector128 {
    //TODO
}

/*impl From<x86_64::__m128> for U64Vector128 {
    #[inline(always)]
    fn from(raw: x86_64::__m128) -> U64Vector128 {
        unsafe {
            return U64Vector128 {
                vector: x86_64::_mm_castps_si128(raw)
            };
        }
    }
}

impl From<x86_64::__m128i> for U64Vector128 {
    #[inline(always)]
    fn from(raw: x86_64::__m128i) -> U64Vector128 {
        unsafe {
            return U64Vector128 {
                vector: raw
            };
        }
    }
}

impl From<x86_64::__m128d> for U64Vector128 {
    #[inline(always)]
    fn from(raw: x86_64::__m128d) -> U64Vector128 {
        unsafe {
            return U64Vector128 {
                vector: x86_64::_mm_castpd_si128(raw)
            };
        }
    }
}

impl From<U64Vector128> for x86_64::__m128 {
    #[inline(always)]
    fn from(wrapper: U64Vector128) -> x86_64::__m128 {
        unsafe {
            return x86_64::_mm_castsi128_ps(wrapper.vector);
        }
    }
}

impl From<U64Vector128> for x86_64::__m128i {
    #[inline(always)]
    fn from(wrapper: U64Vector128) -> x86_64::__m128i {
        unsafe {
            return wrapper.vector;
        }
    }
}

impl From<U64Vector128> for x86_64::__m128d {
    #[inline(always)]
    fn from(wrapper: U64Vector128) -> x86_64::__m128d {
        unsafe {
            return x86_64::_mm_castsi128_pd(wrapper.vector);
        }
    }
}
*/

/*
impl From<x86_64::__m128i> for U64Vector128 {
    #[inline(always)]
    fn from(raw: x86_64::__m128i) -> U64Vector128 {
        unsafe {
            return U64Vector128 {
                vector: raw
            };
        }
    }
}

impl From<x86_64::__m128d> for U64Vector128 {
    #[inline(always)]
    fn from(raw: x86_64::__m128d) -> U64Vector128 {
        unsafe {
            return U64Vector128 {
                vector: x86_64::_mm_castpd_si128(raw)
            };
        }
    }
}
*/

/*
impl AsRef<x86_64::__m128> for U64Vector128 {
    #[inline(always)]
    fn as_ref(self: &Self) -> &x86_64::__m128 {
        unsafe {
            return &x86_64::_mm_castsi128_ps(self.vector);
        }
    }
}

impl AsRef<x86_64::__m128i> for U64Vector128 {
    #[inline(always)]
    fn as_ref(self: &Self) -> &x86_64::__m128i {
        unsafe {
            return &self.vector;
        }
    }
}

impl AsRef<x86_64::__m128d> for U64Vector128 {
    #[inline(always)]
    fn as_ref(self: &Self) -> &x86_64::__m128d {
        unsafe {
            return &x86_64::_mm_castsi128_pd(self.vector);
        }
    }
}
*/

/*
impl AsRef<x86_64::__m128i> for U64Vector128 {
    fn from(raw: x86_64::__m128i) -> U64Vector128 {
        unsafe {
            return U64Vector128 {
                vector: raw
            };
        }
    }
}

impl AsRef<x86_64::__m128d> for U64Vector128 {
    fn from(raw: x86_64::__m128d) -> U64Vector128 {
        unsafe {
            return U64Vector128 {
                vector: x86_64::_mm_castpd_si128(raw)
            };
        }
    }
}*/

/* Functions */

//TODO
