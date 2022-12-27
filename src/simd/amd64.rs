/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use core::arch::x86_64;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub(super) struct Nice128I {
    pub(super) vector: x86_64::__m128i
}

pub(super) struct Nice128D {
    pub(super) vector: x86_64::__m128d
}

pub(super) struct Nice256I {
    pub(super) vector: x86_64::__m256i
}

pub(super) struct Nice256D {
    pub(super) vector: x86_64::__m256d
}

/*
pub(super) struct Double128I {
    pub(super) first: x86_64::__m128i,
    pub(super) second: x86_64::__m128i
}

pub(super) struct Double128D {
    pub(super) first: x86_64::__m128d,
    pub(super) second: x86_64::__m128d
}

pub(super) struct Double256I {
    pub(super) first: x86_64::__m256i,
    pub(super) second: x86_64::__m256i
}

pub(super) struct Double256D {
    pub(super) first: x86_64::__m256d,
    pub(super) second: x86_64::__m256d
}
*/

/* Associated Functions and Methods */

impl Nice128I {
    #[inline(always)]
    fn with_u64s_all_set_to(value: u64) -> Nice128I {
        unsafe {
            return Nice128I {
                vector: x86_64::_mm_set1_epi64x(value as i64)
            }
        }
    }

    #[inline(always)]
    fn to_nice128d(self: Self) -> Nice128D {
        unsafe {
            return Nice128D {
                vector: x86_64::_mm_castsi128_pd(self.vector)
            }
        }
    }

    #[inline(always)]
    fn store_u64s_unaligned_to(self: &Self, addr: *mut u64) {
        unsafe {
            x86_64::_mm_storeu_si128(addr as *mut x86_64::__m128i, self.vector);
        }
    }

    //#[inline(always)]
    //fn is_zero_sse2(self: &Self) ->
}

impl std::ops::Add for Nice128I {
    type Output = Nice128I;

    #[inline(always)]
    fn add(self: Self, rhs: Nice128I) -> Nice128I {
        unsafe {
            return Nice128I {
                vector: x86_64::_mm_add_epi64(self.vector, rhs.vector)
            }
        }
    }
}

impl std::ops::AddAssign for Nice128I {
    #[inline(always)]
    fn add_assign(self: &mut Self, rhs: Nice128I) {
        unsafe {
            self.vector = x86_64::_mm_add_epi64(self.vector, rhs.vector);
        }
    }
}

impl std::ops::BitAnd for Nice128I {
    type Output = Nice128I;

    #[inline(always)]
    fn bitand(self: Self, rhs: Nice128I) -> Nice128I {
        unsafe {
            return Nice128I {
                vector: x86_64::_mm_and_si128(self.vector, rhs.vector)
            }
        }
    }
}

impl std::ops::BitAndAssign for Nice128I {
    #[inline(always)]
    fn bitand_assign(self: &mut Self, rhs: Nice128I) {
        unsafe {
            self.vector = x86_64::_mm_and_si128(self.vector, rhs.vector);
        }
    }
}

impl Nice128D {
    #[inline(always)]
    fn with_f64s(value_hi: f64, value_low: f64) -> Nice128D {
        return Nice128D {
            vector: unsafe { x86_64::_mm_set_pd(value_hi, value_low) }
        }
    }

    #[inline(always)]
    fn with_f64s_all_set_to(value: f64) -> Nice128D {
        return Nice128D {
            vector: unsafe { x86_64::_mm_set_pd1(value) }
        }
    }

    #[inline(always)]
    fn to_nice128i(self: Self) -> Nice128I {
        unsafe {
            return Nice128I {
                vector: x86_64::_mm_castpd_si128(self.vector)
            }
        }
    }
}

impl std::ops::Add for Nice128D {
    type Output = Nice128D;

    #[inline(always)]
    fn add(self: Self, rhs: Nice128D) -> Nice128D {
        unsafe {
            return Nice128D {
                vector: x86_64::_mm_add_pd(self.vector, rhs.vector)
            }
        }
    }
}

impl std::ops::Sub for Nice128D {
    type Output = Nice128D;

    #[inline(always)]
    fn sub(self: Self, rhs: Nice128D) -> Nice128D {
        unsafe {
            return Nice128D {
                vector: x86_64::_mm_sub_pd(self.vector, rhs.vector)
            }
        }
    }
}

impl std::ops::Mul for Nice128D {
    type Output = Nice128D;

    #[inline(always)]
    fn mul(self: Self, rhs: Nice128D) -> Nice128D {
        unsafe {
            return Nice128D {
                vector: x86_64::_mm_mul_pd(self.vector, rhs.vector)
            }
        }
    }
}

/*
impl std::ops::Add for Double128I {
    type Output = Double128I;

    #[inline(always)]
    fn add(self: Self, rhs: Double128I) -> Double128I {
        unsafe {
            return Double128I {
                first: x86_64::_mm_add_epi64(self.first, rhs.first),
                second: x86_64::_mm_add_epi64(self.second, rhs.second)
            }
        }
    }
}
*/

/* Functions */

//TODO
