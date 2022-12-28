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

macro_rules! define_integer_vector128_struct_with_primitive {
    ($t: ident, $primitive: ident) => (
        //Basic setup of the new struct
        #[derive(Copy, Clone, Debug)]
        #[repr(align(16))]
        pub struct $t {
            vector: x86_64::__m128i
        }

        //Traits
        /*impl Vector128 for $t {
            type AssociatedPrimitive = $primitive;

            #[inline(always)]
            fn new_zeroed() -> $t {
                return Self {
                    vector: unsafe { x86_64::_mm_setzero_si128() }
                };
            }

            //TODO
        }
        */

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
                    vector: unsafe { x86_64::_mm_and_si128(self.vector, rhs.vector) }
                };
            }
        }

        impl BitAndAssign for $t {
            #[inline(always)]
            fn bitand_assign(self: &mut Self, rhs: Self) {
                self.vector = unsafe { x86_64::_mm_and_si128(self.vector, rhs.vector) };
            }
        }

        impl BitOr for $t {
            type Output = Self;

            #[inline(always)]
            fn bitor(self: Self, rhs: Self) -> Self {
                return Self {
                    vector: unsafe { x86_64::_mm_or_si128(self.vector, rhs.vector) }
                };
            }
        }

        impl BitOrAssign for $t {
            #[inline(always)]
            fn bitor_assign(self: &mut Self, rhs: Self) {
                self.vector = unsafe { x86_64::_mm_or_si128(self.vector, rhs.vector) };
            }
        }

        impl BitXor for $t {
            type Output = Self;

            #[inline(always)]
            fn bitxor(self: Self, rhs: Self) -> Self {
                return Self {
                    vector: unsafe { x86_64::_mm_xor_si128(self.vector, rhs.vector) }
                };
            }
        }

        impl BitXorAssign for $t {
            #[inline(always)]
            fn bitxor_assign(self: &mut Self, rhs: Self) {
                self.vector = unsafe { x86_64::_mm_xor_si128(self.vector, rhs.vector) };
            }
        }
    )
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

    ////FIXME don't do this; movemask dosn't work on all primitive types (only U8, F32, F64)

    //fn cmpeq(self: Self, rhs: Self) -> Self;
    //TODO others like the above
}

pub trait FloatVector128:
    Vector128 + Div + DivAssign + Mul + MulAssign
{
    //TODO
    //TODO sqrt, rsqrt, etc.
    fn movemask(self: Self) -> i32;

}

pub trait IntegerVector128://TODO on the I* vectors, Shr will be arithmetic, but it will be logical on the U* vectors
    Vector128 + AsRef<x86_64::__m128i> + AsMut<x86_64::__m128i> + Shl + ShlAssign + Shr + ShrAssign
{
    //TODO

    //TODO perhaps have I8, I16, I32, I64 so that we do arithmetic shifts on that, and regular shifs on the U versions?
    //fn shri<const AMOUNT: i32>(self: Self) -> Self;
    //fn shli<const AMOUNT: i32>(self: Self) -> Self;
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
define_integer_vector128_struct_with_primitive!(I16Vector128, i16);
define_integer_vector128_struct_with_primitive!(I32Vector128, i32);
define_integer_vector128_struct_with_primitive!(I64Vector128, i64);*/

/*define_integer_vector128_struct_with_primitive!(U8Vector128, u8);//TODO this also supports fn movemask(self: Self) -> i32;
define_integer_vector128_struct_with_primitive!(U16Vector128, u16);
define_integer_vector128_struct_with_primitive!(U32Vector128, u32);*/
define_integer_vector128_struct_with_primitive!(U64Vector128, u64);

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

/*
impl std::ops::Add for F64Vector128 {
    type Output = F64Vector128;

    #[inline(always)]
    fn add(self: Self, rhs: F64Vector128) -> F64Vector128 {
        return F64Vector128 {
            vector: unsafe { x86_64::_mm_add_pd(self.vector, rhs.vector) }
        }
    }
}
*/
overload_operator_for!(F64Vector128, Add, add, _mm_add_pd);

impl std::ops::AddAssign for F64Vector128 {
    #[inline(always)]
    fn add_assign(self: &mut Self, rhs: F64Vector128) {
        self.vector = unsafe { x86_64::_mm_add_pd(self.vector, rhs.vector) };
    }
}

impl BitAnd for F64Vector128 {
    type Output = Self;

    #[inline(always)]
    fn bitand(self: Self, rhs: Self) -> Self {
        return Self {
            vector: unsafe { x86_64::_mm_and_pd(self.vector, rhs.vector) }
        };
    }
}

impl BitAndAssign for F64Vector128 {
    #[inline(always)]
    fn bitand_assign(self: &mut Self, rhs: Self) {
        self.vector = unsafe { x86_64::_mm_and_pd(self.vector, rhs.vector) };
    }
}

impl BitOr for F64Vector128 {
    type Output = Self;

    #[inline(always)]
    fn bitor(self: Self, rhs: Self) -> Self {
        return Self {
            vector: unsafe { x86_64::_mm_or_pd(self.vector, rhs.vector) }
        };
    }
}

impl BitOrAssign for F64Vector128 {
    #[inline(always)]
    fn bitor_assign(self: &mut Self, rhs: Self) {
        self.vector = unsafe { x86_64::_mm_or_pd(self.vector, rhs.vector) };
    }
}

impl BitXor for F64Vector128 {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self: Self, rhs: Self) -> Self {
        return Self {
            vector: unsafe { x86_64::_mm_xor_pd(self.vector, rhs.vector) }
        };
    }
}

impl BitXorAssign for F64Vector128 {
    #[inline(always)]
    fn bitxor_assign(self: &mut Self, rhs: Self) {
        self.vector = unsafe { x86_64::_mm_xor_pd(self.vector, rhs.vector) };
    }
}

impl std::ops::Div for F64Vector128 {
    type Output = F64Vector128;

    #[inline(always)]
    fn div(self: Self, rhs: F64Vector128) -> F64Vector128 {
        return F64Vector128 {
            vector: unsafe { x86_64::_mm_div_pd(self.vector, rhs.vector) }
        }
    }
}

impl std::ops::DivAssign for F64Vector128 {
    #[inline(always)]
    fn div_assign(self: &mut Self, rhs: F64Vector128) {
        self.vector = unsafe { x86_64::_mm_div_pd(self.vector, rhs.vector) };
    }
}

impl std::ops::Mul for F64Vector128 {
    type Output = F64Vector128;

    #[inline(always)]
    fn mul(self: Self, rhs: F64Vector128) -> F64Vector128 {
        return F64Vector128 {
            vector: unsafe { x86_64::_mm_mul_pd(self.vector, rhs.vector) }
        }
    }
}

impl std::ops::MulAssign for F64Vector128 {
    #[inline(always)]
    fn mul_assign(self: &mut Self, rhs: F64Vector128) {
        self.vector = unsafe { x86_64::_mm_mul_pd(self.vector, rhs.vector) };
    }
}

impl std::ops::Sub for F64Vector128 {
    type Output = F64Vector128;

    #[inline(always)]
    fn sub(self: Self, rhs: F64Vector128) -> F64Vector128 {
        return F64Vector128 {
            vector: unsafe { x86_64::_mm_sub_pd(self.vector, rhs.vector) }
        }
    }
}

impl std::ops::SubAssign for F64Vector128 {
    #[inline(always)]
    fn sub_assign(self: &mut Self, rhs: F64Vector128) {
        self.vector = unsafe { x86_64::_mm_sub_pd(self.vector, rhs.vector) };
    }
}

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
//TODO
/*
impl U8Vector128 {
    #[inline(always)]
    fn movemask(self: Self) -> i32 {
        return unsafe { x86_64::_mm_movemask_epi8(self.vector) };
    }
}
*/

//U16Vector128
//TODO

//U32Vector128
//TODO

//U64Vector128
impl Vector128 for U64Vector128 {
    //TODO turn boilerplate into a macro (everything that's common between all IntegerVector128 types)


    type AssociatedPrimitive = u64;
    type AssociatedPrimitiveArray = [u64; 2];

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

    #[inline(always)]//TODO boilerplate
    fn new_zeroed() -> U64Vector128 {
        return U64Vector128 {
            vector: unsafe { x86_64::_mm_setzero_si128() }
        };
    }

    #[inline(always)]//TODO boilerplate
    fn new_uninit() -> MaybeUninit<U64Vector128> {
        return MaybeUninit::new(U64Vector128 {
            vector: unsafe { x86_64::_mm_undefined_si128() }
        });
    }

    #[inline(always)]//TODO boilerplate
    unsafe fn unaligned_load_from(self: Self, address: *const u64) {
        todo!()
    }

    #[inline(always)]//TODO boilerplate
    unsafe fn unaligned_store_to(self: Self, address: *mut u64) {
        unsafe { x86_64::_mm_storeu_si128(address as *mut x86_64::__m128i, self.vector); }
    }

    #[inline(always)]//TODO boilerplate
    unsafe fn aligned_load_from(self: Self, address: *const u64) {
        todo!()
    }

    #[inline(always)]//TODO boilerplate
    unsafe fn aligned_store_to(self: Self, address: *mut u64) {
        unsafe { x86_64::_mm_store_si128(address as *mut x86_64::__m128i, self.vector); }
    }

    //TODO
}

impl std::ops::Add for U64Vector128 {
    type Output = U64Vector128;

    #[inline(always)]
    fn add(self: Self, rhs: U64Vector128) -> U64Vector128 {
        return U64Vector128 {
            vector: unsafe { x86_64::_mm_add_epi64(self.vector, rhs.vector) }
        }
    }
}

impl std::ops::AddAssign for U64Vector128 {
    #[inline(always)]
    fn add_assign(self: &mut Self, rhs: U64Vector128) {
        self.vector = unsafe { x86_64::_mm_add_epi64(self.vector, rhs.vector) };
    }
}

impl std::ops::Sub for U64Vector128 {
    type Output = U64Vector128;

    #[inline(always)]
    fn sub(self: Self, rhs: U64Vector128) -> U64Vector128 {
        return U64Vector128 {
            vector: unsafe { x86_64::_mm_sub_epi64(self.vector, rhs.vector) }
        }
    }
}

impl std::ops::SubAssign for U64Vector128 {
    #[inline(always)]
    fn sub_assign(self: &mut Self, rhs: U64Vector128) {
        self.vector = unsafe { x86_64::_mm_sub_epi64(self.vector, rhs.vector) };
    }
}

impl std::ops::Shl for U64Vector128 {
    type Output = U64Vector128;

    #[inline(always)]
    fn shl(self: Self, rhs: U64Vector128) -> U64Vector128 {
        return U64Vector128 {
            vector: unsafe { x86_64::_mm_sll_epi64(self.vector, rhs.vector) }
        }
    }
}

impl std::ops::ShlAssign for U64Vector128 {
    #[inline(always)]
    fn shl_assign(self: &mut Self, rhs: U64Vector128) {
        self.vector = unsafe { x86_64::_mm_sll_epi64(self.vector, rhs.vector) };
    }
}

impl std::ops::Shr for U64Vector128 {//Logical right shift since this is unsigned
    type Output = U64Vector128;

    #[inline(always)]
    fn shr(self: Self, rhs: U64Vector128) -> U64Vector128 {
        return U64Vector128 {
            vector: unsafe { x86_64::_mm_srl_epi64(self.vector, rhs.vector) }
        }
    }
}

impl std::ops::ShrAssign for U64Vector128 {//Logical right shift since this is unsigned
    #[inline(always)]
    fn shr_assign(self: &mut Self, rhs: U64Vector128) {
        self.vector = unsafe { x86_64::_mm_srl_epi64(self.vector, rhs.vector) };
    }
}

/* Functions */

//TODO
