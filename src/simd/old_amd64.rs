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

#[derive(Copy, Clone, Debug)]
pub struct Nice128I {
    pub vector: x86_64::__m128i
}

#[derive(Copy, Clone, Debug)]
pub struct Nice128D {
    pub vector: x86_64::__m128d
}

#[derive(Copy, Clone, Debug)]
pub struct Nice256I {
    pub vector: x86_64::__m256i
}

#[derive(Copy, Clone, Debug)]
pub struct Nice256D {
    pub vector: x86_64::__m256d
}

/*
#[derive(Copy, Clone, Debug)]
pub struct Multi128I<const N: usize> {
    pub vectors: [x86_64::__m128i; N]
}

#[derive(Copy, Clone, Debug)]
pub struct Multi128D<const N: usize> {
    pub vectors: [x86_64::__m128d; N]
}
*/

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
    pub fn with_u64s_all_set_to(value: u64) -> Nice128I {
        unsafe {
            return Nice128I {
                vector: x86_64::_mm_set1_epi64x(value as i64)
            }
        }
    }

    #[inline(always)]
    pub fn new_zeroed() -> Nice128I {
        unsafe {
            return Nice128I {
                vector: x86_64::_mm_setzero_si128()
            }
        }
    }

    #[inline(always)]
    pub fn to_nice128d(self: Self) -> Nice128D {
        unsafe {
            return Nice128D {
                vector: x86_64::_mm_castsi128_pd(self.vector)
            }
        }
    }

    #[inline(always)]
    pub fn store_u64s_unaligned_to(self: &Self, addr: *mut u64) {
        unsafe {
            x86_64::_mm_storeu_si128(addr as *mut x86_64::__m128i, self.vector);
        }
    }

    #[inline(always)]
    pub fn create_mask_from_msbs_of_u8s(self: Self) -> i32 {
        unsafe {
            return x86_64::_mm_movemask_epi8(self.vector);
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
    pub fn with_f64s(value_hi: f64, value_low: f64) -> Nice128D {
        return Nice128D {
            vector: unsafe { x86_64::_mm_set_pd(value_hi, value_low) }
        }
    }

    #[inline(always)]
    pub fn new_zeroed() -> Nice128D {
        unsafe {
            return Nice128D {
                vector: x86_64::_mm_setzero_pd()
            }
        }
    }

    #[inline(always)]
    pub fn with_f64s_all_set_to(value: f64) -> Nice128D {
        return Nice128D {
            vector: unsafe { x86_64::_mm_set_pd1(value) }
        }
    }

    #[inline(always)]
    pub fn to_nice128i(self: Self) -> Nice128I {
        unsafe {
            return Nice128I {
                vector: x86_64::_mm_castpd_si128(self.vector)
            }
        }
    }

    #[inline(always)]
    pub fn set_u64s_in_nice128i_if_less_than(self: Self, rhs: Nice128D) -> Nice128I {
        unsafe {
            return Nice128I {
                vector: x86_64::_mm_castpd_si128(x86_64::_mm_cmplt_pd(self.vector, rhs.vector))
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

impl Nice256I {
    #[inline(always)]
    pub fn with_u64s_all_set_to(value: u64) -> Nice256I {
        unsafe {
            return Nice256I {
                vector: x86_64::_mm256_set1_epi64x(value as i64)
            }
        }
    }

    #[inline(always)]
    pub fn new_zeroed() -> Nice256I {
        unsafe {
            return Nice256I {
                vector: x86_64::_mm256_setzero_si256()
            }
        }
    }

    #[inline(always)]
    pub fn to_nice256d(self: Self) -> Nice256D {
        unsafe {
            return Nice256D {
                vector: x86_64::_mm256_castsi256_pd(self.vector)
            }
        }
    }

    #[inline(always)]
    pub fn store_u64s_unaligned_to(self: &Self, addr: *mut u64) {
        unsafe {
            x86_64::_mm256_storeu_si256(addr as *mut x86_64::__m256i, self.vector);
        }
    }

    /*#[inline(always)]
    pub fn create_mask_from_msbs_of_u8s(self: Self) -> i32 {
        unsafe {
            return x86_64::_mm256_movemask_pd(self.vector);
        }
    }
    */

    //TODO use addition functions to make it clear if AVX2 is used or not

    pub fn add_u64s_without_avx2(lhs: Nice256I, rhs: Nice256I) -> Nice256I {
        unsafe {
            //We have to add the upper and lower parts separately

            //Split them apart
            let lower_lhs = x86_64::_mm256_castsi256_si128(lhs.vector);
            let upper_lhs = x86_64::_mm256_extractf128_si256(lhs.vector, 1);
            let lower_rhs = x86_64::_mm256_castsi256_si128(rhs.vector);
            let upper_rhs = x86_64::_mm256_extractf128_si256(rhs.vector, 1);

            //Add the halves
            let lower_sum = x86_64::_mm_add_epi64(lower_lhs, lower_rhs);
            let upper_sum = x86_64::_mm_add_epi64(upper_lhs, upper_rhs);

            //Put them back together
            let mut result = x86_64::_mm256_castsi128_si256(lower_sum);
            result = x86_64::_mm256_insertf128_si256(result, upper_sum, 1);

            return Nice256I {
                vector: result
            }
        }
    }

    pub fn add_u64s_with_avx2(lhs: Nice256I, rhs: Nice256I) -> Nice256I {
        unsafe {
            return Nice256I {
                vector: x86_64::_mm256_add_epi64(lhs.vector, rhs.vector)
            }
        }
    }

    //#[inline(always)]
    //fn is_zero_sse2(self: &Self) ->
}
/*

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
    pub fn with_f64s(value_hi: f64, value_low: f64) -> Nice128D {
        return Nice128D {
            vector: unsafe { x86_64::_mm_set_pd(value_hi, value_low) }
        }
    }

    #[inline(always)]
    pub fn new_zeroed() -> Nice128D {
        unsafe {
            return Nice128D {
                vector: x86_64::_mm_setzero_pd()
            }
        }
    }

    #[inline(always)]
    pub fn with_f64s_all_set_to(value: f64) -> Nice128D {
        return Nice128D {
            vector: unsafe { x86_64::_mm_set_pd1(value) }
        }
    }

    #[inline(always)]
    pub fn to_nice128i(self: Self) -> Nice128I {
        unsafe {
            return Nice128I {
                vector: x86_64::_mm_castpd_si128(self.vector)
            }
        }
    }

    #[inline(always)]
    pub fn set_u64s_in_nice128i_if_less_than(self: Self, rhs: Nice128D) -> Nice128I {
        unsafe {
            return Nice128I {
                vector: x86_64::_mm_castpd_si128(x86_64::_mm_cmplt_pd(self.vector, rhs.vector))
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

*/




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
/*
impl<const N: usize> Multi128I<N> {
    #[inline(always)]
    pub fn with_u64s_all_set_to(value: u64) -> Multi128I<N> {
        unsafe {
            return Multi128I {
                vectors: [x86_64::_mm_set1_epi64x(value as i64); N]
            }
        }
    }

    #[inline(always)]
    pub fn new_zeroed() -> Multi128I<N> {
        unsafe {
            return Multi128I {
                vectors: [x86_64::_mm_setzero_si128(); N]
            }
        }
    }

    #[inline(always)]
    pub fn to_128d(self: Self) -> Multi128D<N> {
        unsafe {
            //TODO ensure the performance of this is okay
            return Multi128D::<N> {
                vectors: self.vectors.map(| v | x86_64::_mm_castsi128_pd(v))
            }
        }
    }

    #[inline(always)]
    pub fn store_u64s_unaligned_to(self: &Self, addr: *mut u64) {
        unsafe {
            //TODO ensure the performance of this is okay
            let pointer = addr as *mut x86_64::__m128i;
            for offset in 0..N {
                x86_64::_mm_storeu_si128(pointer.offset(offset as isize), self.vectors[offset]);
            }
        }
    }

    #[inline(always)]
    pub fn create_mask_from_msbs_of_u8s(self: Self) -> [i32; N] {
        unsafe {
            //TODO ensure the performance of this is okay
            return self.vectors.map(| v | x86_64::_mm_movemask_epi8(v));
        }
    }

    //#[inline(always)]
    //fn is_zero_sse2(self: &Self) ->
}


impl<const N: usize> std::ops::Add for Multi128I<N> {
    type Output = Multi128I<N>;

    #[inline(always)]
    fn add(self: Self, rhs: Multi128I<N>) -> Multi128I<N> {
        unsafe {
            //TODO ensure the performance of this is okay
            let mut new_vectors: [x86_64::__m128i; N] = self.vectors;//TODO avoid cost of initializing this
            for i in 0..N {
                new_vectors[i] = x86_64::_mm_add_epi64(self.vectors[i], rhs.vectors[i]);
            }

            return Multi128I::<N> {
                vectors: new_vectors
            }
        }
    }
}


impl<const N: usize> std::ops::AddAssign for Multi128I<N> {
    #[inline(always)]
    fn add_assign(self: &mut Self, rhs: Multi128I<N>) {
        unsafe {
            //TODO ensure the performance of this is okay
            for i in 0..N {
                self.vectors[i] = x86_64::_mm_add_epi64(self.vectors[i], rhs.vectors[i]);
            }
        }
    }
}

impl<const N: usize> std::ops::BitAnd for Multi128I<N> {
    type Output = Multi128I<N>;

    #[inline(always)]
    fn bitand(self: Self, rhs: Multi128I<N>) -> Multi128I<N> {
        unsafe {
            //TODO ensure the performance of this is okay
            let mut new_vectors: [x86_64::__m128i; N] = self.vectors;//TODO avoid cost of initializing this
            for i in 0..N {
                new_vectors[i] = x86_64::_mm_and_si128(self.vectors[i], rhs.vectors[i]);
            }

            return Multi128I::<N> {
                vectors: new_vectors
            }
        }
    }
}

impl<const N: usize> std::ops::BitAndAssign for Multi128I<N> {
    #[inline(always)]
    fn bitand_assign(self: &mut Self, rhs: Multi128I<N>) {
        unsafe {
            //TODO ensure the performance of this is okay
            for i in 0..N {
                self.vectors[i] = x86_64::_mm_and_si128(self.vectors[i], rhs.vectors[i]);
            }
        }
    }
}

impl<const N: usize> Multi128D<N> {
    /*#[inline(always)]
    pub fn with_f64s(values_high: [f64; N], values_low: [f64; N]) -> Multi128D<N> {//Since we can't do [f64; N * 2]
        unsafe {
            //TODO ensure the performance of this is okay
            let mut new_vectors: [x86_64::__m128d; N] = [];//TODO avoid cost of initializing this
            for i in 0..N {
                new_vectors[i] = x86_64::_mm_add_epi64(self.vectors[i], rhs.vectors[i]);
            }
            //return Multi128D {
            //   vector: unsafe { x86_64::_mm_set_pd(value_hi, value_low) }
            //}
        }
    }*/

    #[inline(always)]
    pub fn with_f64s_all_set_to(value: f64) -> Multi128D<N> {
        unsafe {
            return Multi128D::<N> {
                vectors: [x86_64::_mm_set_pd1(value); N]
            }
        }
    }

    #[inline(always)]
    pub fn new_zeroed() -> Multi128D<N> {
        unsafe {
            return Multi128D::<N> {
                vectors: [x86_64::_mm_setzero_pd(); N]
            }
        }
    }

    #[inline(always)]
    pub fn to_nice128i(self: Self) -> Nice128I {
        unsafe {
            return Nice128I {
                vector: x86_64::_mm_castpd_si128(self.vector)
            }
        }
    }
/*
    #[inline(always)]
    pub fn set_u64s_in_nice128i_if_less_than(self: Self, rhs: Nice128D) -> Nice128I {
        unsafe {
            return Nice128I {
                vector: x86_64::_mm_castpd_si128(x86_64::_mm_cmplt_pd(self.vector, rhs.vector))
            }
        }
    }
    */
}
*/

/*
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
*/

/* Functions */

//TODO