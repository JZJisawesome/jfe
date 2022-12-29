/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

pub mod base;
pub mod sse3;
pub mod ssse3;
pub mod sse41;
pub mod sse42;
pub mod avx;
pub mod avx2;

pub use base::*;

/* Constants */

//TODO

/* Macros */

macro_rules! overload_operator_for {
    ($t: ident, $ops_trait: ident, $trait_function: ident, $auto_ops_trait: ident, $auto_trait_function: ident, $amd64_intrinsic: ident) => {
        impl $ops_trait for $t {
            type Output = Self;

            #[inline(always)]
            fn $trait_function(self: Self, rhs: Self) -> Self {
                return Self {
                    vector: unsafe { x86_64::$amd64_intrinsic(self.vector, rhs.vector) }
                };
            }
        }

        impl $auto_ops_trait for $t {
            #[inline(always)]
            fn $auto_trait_function(self: &mut Self, rhs: Self) {
                self.vector = unsafe { x86_64::$amd64_intrinsic(self.vector, rhs.vector) };
            }
        }
    }
}
pub(crate) use overload_operator_for;

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
pub(crate) use implement_nicetransmute_for;

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
pub(crate) use implement_cast_from_for;

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
pub(crate) use implement_cast_into_for;

/* Static Variables */

//TODO

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

//TODO
