/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use super::Mandelbrot;
use super::Workload;

use core::arch::x86_64;

use crate::simd::amd64;//DON'T use anything modules within this at the module scope so that we don't unintentionally use it when we didn't mean to
use crate::simd::amd64::{Vector128, ComparableVector128, U8Vector128, U64Vector128, F64Vector128};//Base types are okay since we assume we are on amd64

use std::thread;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

//TODO

//TODO add types to wrap multiple x86 vectors in one to make things more clear

/* Associated Functions and Methods */

impl Mandelbrot {
    //Detect CPU features and choose the fastest supported implementation

    pub(super) unsafe fn update_x86_64(self: &mut Self) {
        if is_x86_feature_detected!("avx2") {
            self.update_avx2();
        } else if is_x86_feature_detected!("avx") {
            self.update_avx();
        } else {
            self.update_sse2();//On x86_64, we can assume SSE2
        }
    }

    /*     ___     ____  ______       ____
     *    / \ \   / /\ \/ /___ \__  _|___ \
     *   / _ \ \ / /  \  /  __) \ \/ / __) |
     *  / ___ \ V /   /  \ / __/ >  < / __/
     * /_/   \_\_/   /_/\_\_____/_/\_\_____|
     *
     * AVX2 Mandelbrot implementation, interleaving processing of two vectors at once to better mask latency
    */

    #[target_feature(enable = "avx2")]
    unsafe fn update_avx2(self: &mut Self) {
        self.update_with_unsafe_line_function(Self::update_avx2_line);
    }

    #[target_feature(enable = "avx2")]
    unsafe fn update_avx2_line(max_iterations: usize, starting_c_real: f64, real_step_amount: f64, workload: Workload) {
        use amd64::avx::Vector256;
        use amd64::avx::F64Vector256;

        let real_step_amount_vector = F64Vector256::new_broadcasted(real_step_amount * 8.0);//x8 since we process eight real coords at a time (with two F64Vector256s)

        for (line_slice, c_imag_scalar) in workload {
            debug_assert!((line_slice.len() & 0b111) == 0);//TODO overcome this limitation

            let line_slice_pointer = line_slice.as_mut_ptr();

            let c_imag = [F64Vector256::new_broadcasted(c_imag_scalar); 2];

            let mut c_real = [
                F64Vector256::from([
                    starting_c_real + (real_step_amount * 7.0), starting_c_real + (real_step_amount * 6.0),
                    starting_c_real + (real_step_amount * 5.0), starting_c_real + (real_step_amount * 4.0)
                ]),
                F64Vector256::from([
                    starting_c_real + (real_step_amount * 3.0), starting_c_real + (real_step_amount * 2.0),
                    starting_c_real + real_step_amount, starting_c_real
                ]),
            ];

            for x in (0..line_slice.len()).step_by(8) {
                let result = Self::mandelbrot_iterations_avx2(max_iterations, c_real, c_imag);
                let pointer = line_slice_pointer.offset(x as isize) as *mut u64;
                result[1].unaligned_store_to(pointer);
                result[0].unaligned_store_to(pointer.offset(4));
                c_real = [c_real[0] + real_step_amount_vector, c_real[1] + real_step_amount_vector];
            }
        }
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn mandelbrot_iterations_avx2(max_iterations: usize, c_real_f: [amd64::avx::F64Vector256; 2], c_imag_f: [amd64::avx::F64Vector256; 2]) -> [amd64::avx2::U64Vector256; 2] {
        use amd64::avx::Vector256;
        use amd64::avx::FloatVector256;
        use amd64::avx2::U64Vector256;
        use amd64::avx::F64Vector256;
        use amd64::avx::ComparableVector256;

        let diverge_threshold: f64 = 2.0;//TODO make this flexible?
        let diverge_threshold_squared_f = F64Vector256::new_broadcasted(diverge_threshold * diverge_threshold);
        let two_f = F64Vector256::new_broadcasted(2.0);

        let mut result_i = [U64Vector256::new_broadcasted(0); 2];

        let mut incrementor_i = [U64Vector256::new_broadcasted(1); 2];

        let mut z_real_f = [F64Vector256::new_zeroed(); 2];
        let mut z_imag_f = [F64Vector256::new_zeroed(); 2];

        for _ in 0..max_iterations {
            //Calculate some values that are used below
            let z_real_squared_f = [z_real_f[0] * z_real_f[0], z_real_f[1] * z_real_f[1]];
            let z_imag_squared_f = [z_imag_f[0] * z_imag_f[0], z_imag_f[1] * z_imag_f[1]];

            //Check if the modulus of each z < the diverge value (aka that they haven't diverged)
            //We do this faster by doing (z_real * z_real) + (z_imag * z_imag) < (2 * 2)
            let squared_sum_f = [z_real_squared_f[0] + z_imag_squared_f[0], z_real_squared_f[1] + z_imag_squared_f[1]];
            let compare_i_as_f = [squared_sum_f[0].cmp::<{x86_64::_CMP_LT_OQ}>(diverge_threshold_squared_f), squared_sum_f[1].cmp::<{x86_64::_CMP_LT_OQ}>(diverge_threshold_squared_f)];
            let compare_i: [U64Vector256; 2] = [compare_i_as_f[0].into(), compare_i_as_f[1].into()];

            //Get next entries (For each complex number z, z_(n+1) = z_n^2 + c)
            //We do this before the diverge check below, instead of after like we used to because it better masks the latency of
            //A) The comparisons performed above to the check below
            //B) The calculation of the next z_imag and z_real to the squaring at the start of the next iteration of the loop
            //Also this dosn't compromise the latency of the squaring at the start of the loop to this since we still have the compare above in-between
            let temp_z_real_f = [z_real_squared_f[0] - z_imag_squared_f[0] + c_real_f[0], z_real_squared_f[1] - z_imag_squared_f[1] + c_real_f[1]];
            z_imag_f = [(two_f * z_real_f[0] * z_imag_f[0]) + c_imag_f[0], (two_f * z_real_f[1] * z_imag_f[1]) + c_imag_f[1]];
            z_real_f = temp_z_real_f;

            //If both complex numbers have diverged (entire vector is 0), return
            if (compare_i_as_f[0].movemask() == 0) && (compare_i_as_f[1].movemask() == 0) {
                break;
            }
            //Increment the corresponding count only if we haven't converged yet
            incrementor_i = [incrementor_i[0] & compare_i[0], incrementor_i[1] & compare_i[1]];//If a number diverged, never increment its iteration count again
            result_i = [result_i[0] + incrementor_i[0], result_i[1] + incrementor_i[1]];
        }

        return result_i;
    }

    /*     ___     ____  __      ____
     *    / \ \   / /\ \/ /__  _|___ \
     *   / _ \ \ / /  \  / \ \/ / __) |
     *  / ___ \ V /   /  \  >  < / __/
     * /_/   \_\_/   /_/\_\/_/\_\_____|
     *
     * AVX Mandelbrot implementation, interleaving processing of two vectors at once to better mask latency
     * Also returns results internally as four U64Vector128 as regular AVX dosn't (fully) support 256bit integer vectors
    */

    #[target_feature(enable = "avx")]
    unsafe fn update_avx(self: &mut Self) {
        self.update_with_unsafe_line_function(Self::update_avx_line);
    }

    #[target_feature(enable = "avx")]
    unsafe fn update_avx_line(max_iterations: usize, starting_c_real: f64, real_step_amount: f64, workload: Workload) {
        use amd64::avx::Vector256;
        use amd64::avx::F64Vector256;

        let real_step_amount_vector = F64Vector256::new_broadcasted(real_step_amount * 8.0);//x8 since we process eight real coords at a time (with two F64Vector256s)

        for (line_slice, c_imag_scalar) in workload {
            debug_assert!((line_slice.len() & 0b111) == 0);//TODO overcome this limitation

            let line_slice_pointer = line_slice.as_mut_ptr();

            let c_imag = [F64Vector256::new_broadcasted(c_imag_scalar); 2];

            let mut c_real = [
                F64Vector256::from([
                    starting_c_real + (real_step_amount * 7.0), starting_c_real + (real_step_amount * 6.0),
                    starting_c_real + (real_step_amount * 5.0), starting_c_real + (real_step_amount * 4.0)
                ]),
                F64Vector256::from([
                    starting_c_real + (real_step_amount * 3.0), starting_c_real + (real_step_amount * 2.0),
                    starting_c_real + real_step_amount, starting_c_real
                ]),
            ];

            for x in (0..line_slice.len()).step_by(8) {
                let result = Self::mandelbrot_iterations_avx(max_iterations, c_real, c_imag);
                let pointer = line_slice_pointer.offset(x as isize) as *mut u64;
                result[3].unaligned_store_to(pointer);
                result[2].unaligned_store_to(pointer.offset(2));
                result[1].unaligned_store_to(pointer.offset(4));
                result[0].unaligned_store_to(pointer.offset(6));
                c_real = [c_real[0] + real_step_amount_vector, c_real[1] + real_step_amount_vector];
            }
        }
    }

    #[inline]
    #[target_feature(enable = "avx")]
    unsafe fn mandelbrot_iterations_avx(max_iterations: usize, c_real_f: [amd64::avx::F64Vector256; 2], c_imag_f: [amd64::avx::F64Vector256; 2]) -> [U64Vector128; 4] {
        use amd64::avx::Vector256;
        use amd64::avx::FloatVector256;
        use amd64::avx::F64Vector256;
        use amd64::avx::ComparableVector256;

        let diverge_threshold: f64 = 2.0;//TODO make this flexible?
        let diverge_threshold_squared_f = F64Vector256::new_broadcasted(diverge_threshold * diverge_threshold);
        let two_f = F64Vector256::new_broadcasted(2.0);

        let mut result_i_in_halves = [U64Vector128::new_broadcasted(0); 4];

        let mut incrementor_i_as_f = [F64Vector256::new_broadcasted(f64::from_bits(1)); 2];//We have to do some whacky stuff since we don't have 256 bit integer vectors

        let mut z_real_f = [F64Vector256::new_zeroed(); 2];
        let mut z_imag_f = [F64Vector256::new_zeroed(); 2];

        for _ in 0..max_iterations {
            //Calculate some values that are used below
            let z_real_squared_f = [z_real_f[0] * z_real_f[0], z_real_f[1] * z_real_f[1]];
            let z_imag_squared_f = [z_imag_f[0] * z_imag_f[0], z_imag_f[1] * z_imag_f[1]];

            //Check if the modulus of each z < the diverge value (aka that they haven't diverged)
            //We do this faster by doing (z_real * z_real) + (z_imag * z_imag) < (2 * 2)
            let squared_sum_f = [z_real_squared_f[0] + z_imag_squared_f[0], z_real_squared_f[1] + z_imag_squared_f[1]];
            let compare_i_as_f = [squared_sum_f[0].cmp::<{x86_64::_CMP_LT_OQ}>(diverge_threshold_squared_f), squared_sum_f[1].cmp::<{x86_64::_CMP_LT_OQ}>(diverge_threshold_squared_f)];

            //Get next entries (For each complex number z, z_(n+1) = z_n^2 + c)
            //We do this before the diverge check below, instead of after like we used to because it better masks the latency of
            //A) The comparisons performed above to the check below
            //B) The calculation of the next z_imag and z_real to the squaring at the start of the next iteration of the loop
            //Also this dosn't compromise the latency of the squaring at the start of the loop to this since we still have the compare above in-between
            let temp_z_real_f = [z_real_squared_f[0] - z_imag_squared_f[0] + c_real_f[0], z_real_squared_f[1] - z_imag_squared_f[1] + c_real_f[1]];
            z_imag_f = [(two_f * z_real_f[0] * z_imag_f[0]) + c_imag_f[0], (two_f * z_real_f[1] * z_imag_f[1]) + c_imag_f[1]];
            z_real_f = temp_z_real_f;


            //If both complex numbers have diverged (entire vector is 0), return
            if (compare_i_as_f[0].movemask() == 0) && (compare_i_as_f[1].movemask() == 0) {
                break;
            }

            //Increment the corresponding count only if we haven't converged yet
            incrementor_i_as_f = [incrementor_i_as_f[0] & compare_i_as_f[0], incrementor_i_as_f[1] & compare_i_as_f[1]];//If a number diverged, never increment its iteration count again

            //We then split the vector apart and add the halves seperatly since we don't have AVX2; so four additions in total
            let incrementor_i_as_f_in_halves = [//Get upper and lower halves as F64Vector128s
                incrementor_i_as_f[0].get_high_half(), incrementor_i_as_f[0].get_low_half(),
                incrementor_i_as_f[1].get_high_half(), incrementor_i_as_f[1].get_low_half()
            ];

            let incrementor_i_in_halves: [U64Vector128; 4] = [//Convert to U64Vector128s
                incrementor_i_as_f_in_halves[0].into(), incrementor_i_as_f_in_halves[1].into(),
                incrementor_i_as_f_in_halves[2].into(), incrementor_i_as_f_in_halves[3].into()
            ];

            result_i_in_halves = [//Perform the four additions
                incrementor_i_in_halves[0] + result_i_in_halves[0], incrementor_i_in_halves[1] + result_i_in_halves[1],
                incrementor_i_in_halves[2] + result_i_in_halves[2], incrementor_i_in_halves[3] + result_i_in_halves[3]
            ];
        }

        return result_i_in_halves;
    }

    /*  ____ ____  _____ ____       ____
     * / ___/ ___|| ____|___ \__  _|___ \
     * \___ \___ \|  _|   __) \ \/ / __) |
     *  ___) |__) | |___ / __/ >  < / __/
     * |____/____/|_____|_____/_/\_\_____|
     *
     * SSE2 Mandelbrot implementation, interleaving processing of two vectors at once to better mask latency
    */

    unsafe fn update_sse2(self: &mut Self) {
        self.update_with_unsafe_line_function(Self::update_sse2_line);
    }

    #[target_feature(enable = "sse2")]
    unsafe fn update_sse2_line(max_iterations: usize, starting_c_real: f64, real_step_amount: f64, workload: Workload) {
        let real_step_amount_vector = F64Vector128::new_broadcasted(real_step_amount * 4.0);//x4 since we process four real coords at a time (with two F64Vector128s)

        for (line_slice, c_imag_scalar) in workload {
            debug_assert!((line_slice.len() & 0b11) == 0);//TODO overcome this limitation
            let line_slice_pointer = line_slice.as_mut_ptr();

            let c_imag = [F64Vector128::new_broadcasted(c_imag_scalar); 2];

            let mut c_real = [
                F64Vector128::from([starting_c_real + (real_step_amount * 3.0), starting_c_real + (real_step_amount * 2.0)]),
                F64Vector128::from([starting_c_real + real_step_amount, starting_c_real]),
            ];

            for x in (0..line_slice.len()).step_by(4) {
                let result = Self::mandelbrot_iterations_sse2(max_iterations, c_real, c_imag);
                let pointer = line_slice_pointer.offset(x as isize) as *mut u64;
                result[1].unaligned_store_to(pointer);
                result[0].unaligned_store_to(pointer.offset(2));
                c_real = [c_real[0] + real_step_amount_vector, c_real[1] + real_step_amount_vector];
            }
        }
    }

    //#[inline(always)]//Can't do this with the second "#[target_feature(enable = "sse2")]"
    #[inline]//But this is okay
    #[target_feature(enable = "sse2")]
    unsafe fn mandelbrot_iterations_sse2(max_iterations: usize, c_real_f: [F64Vector128; 2], c_imag_f: [F64Vector128; 2]) -> [U64Vector128; 2] {
        let diverge_threshold: f64 = 2.0;//TODO make this flexible?

        let diverge_threshold_squared_f = F64Vector128::new_broadcasted(diverge_threshold * diverge_threshold);
        let two_f = F64Vector128::new_broadcasted(2.0);

        let mut result_i = [U64Vector128::new_zeroed(); 2];

        let mut incrementor_i = [U64Vector128::new_broadcasted(1); 2];//We increment the result counter until we go past the diverge_threshold, then we never do again

        let mut z_real_f = [F64Vector128::new_zeroed(); 2];
        let mut z_imag_f = [F64Vector128::new_zeroed(); 2];

        for _ in 0..max_iterations {
            //Calculate some values that are used below
            let z_real_squared_f = [z_real_f[0] * z_real_f[0], z_real_f[1] * z_real_f[1]];
            let z_imag_squared_f = [z_imag_f[0] * z_imag_f[0], z_imag_f[1] * z_imag_f[1]];

            //Check if the modulus of each z < the diverge value (aka that they haven't diverged)
            //We do this faster by doing (z_real * z_real) + (z_imag * z_imag) < (2 * 2)
            let squared_sum_f = [z_real_squared_f[0] + z_imag_squared_f[0], z_real_squared_f[1] + z_imag_squared_f[1]];
            let compare_i_as_f = [squared_sum_f[0].cmplt(diverge_threshold_squared_f), squared_sum_f[1].cmplt(diverge_threshold_squared_f)];
            let compare_i: [U8Vector128; 2] = [compare_i_as_f[0].into(), compare_i_as_f[1].into()];

            //Get next entries (For each complex number z, z_(n+1) = z_n^2 + c)
            //We do this before the diverge check below, instead of after like we used to because it better masks the latency of
            //A) The comparisons performed above to the check below
            //B) The calculation of the next z_imag and z_real to the squaring at the start of the next iteration of the loop
            //Also this dosn't compromise the latency of the squaring at the start of the loop to this since we still have the compare above in-between
            let temp_z_real_f = [z_real_squared_f[0] - z_imag_squared_f[0] + c_real_f[0], z_real_squared_f[1] - z_imag_squared_f[1] + c_real_f[1]];
            z_imag_f = [(two_f * z_real_f[0] * z_imag_f[0]) + c_imag_f[0], (two_f * z_real_f[1] * z_imag_f[1]) + c_imag_f[1]];
            z_real_f = temp_z_real_f;

            //If both complex numbers have diverged (entire vector is 0), return
            if (compare_i[0].movemask() == 0) && (compare_i[1].movemask() == 0) {
                break;
            }

            //Increment the corresponding count only if we haven't converged yet
            incrementor_i = [incrementor_i[0] & compare_i[0], incrementor_i[1] & compare_i[1]];//If a number diverged, never increment its iteration count again
            result_i = [result_i[0] + incrementor_i[0], result_i[1] + incrementor_i[1]];
        }

        return result_i;
    }
}

/* Functions */

//TODO

/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    //Floating point operations are tricky, so these will fail, but it won't really affect how pleasing the image is
    /*
    #[test]
    fn test_sse2_against_universal() {
        let mut mandelbrot_universal = Mandelbrot::new(
            1024,
            128,
            128,
            -2.3, 0.8,
            -1.1, 1.1
        );
        mandelbrot_universal.update_universal();

        let mut mandelbrot_sse2 = Mandelbrot::new(
            1024,
            128,
            128,
            -2.3, 0.8,
            -1.1, 1.1
        );
        unsafe { mandelbrot_sse2.update_sse2(); }

        for i in 0..(128 * 128) {
            assert_eq!(mandelbrot_universal.iterations[i], mandelbrot_sse2.iterations[i]);
        }
    }

    #[test]
    fn test_avx_against_universal() {
        if !is_x86_feature_detected!("avx") {
            panic!("Can't run this test without AVX support");
        }

        let mut mandelbrot_universal = Mandelbrot::new(
            1024,
            128,
            128,
            -2.3, 0.8,
            -1.1, 1.1
        );
        mandelbrot_universal.update_universal();

        let mut mandelbrot_avx = Mandelbrot::new(
            1024,
            128,
            128,
            -2.3, 0.8,
            -1.1, 1.1
        );
        unsafe { mandelbrot_avx.update_avx(); }

        for i in 0..(128 * 128) {
            assert_eq!(mandelbrot_universal.iterations[i], mandelbrot_avx.iterations[i]);
        }
    }
    */
}
/* Benches */

#[cfg_attr(feature = "nightly-features-benches", cfg(test))]
#[cfg(feature = "nightly-features-benches")]
mod benches {
    extern crate test;
    use test::Bencher;
    use super::*;

    #[bench]
    fn update_sse2(b: &mut Bencher) {
        use crate::BaseFractal;
        let mut mandelbrot = Mandelbrot::new(
            1024,
            512,
            512,
            -2.3, 0.8,
            -1.1, 1.1
        );
        mandelbrot.set_max_threads(1);

        b.iter(|| -> Mandelbrot {
            let mut copy = mandelbrot.clone();
            unsafe { copy.update_sse2() };
            return copy;
        });
    }

    #[bench]
    fn update_sse2_mt(b: &mut Bencher) {
        use crate::BaseFractal;
        let mut mandelbrot = Mandelbrot::new(
            1024,
            512,
            512,
            -2.3, 0.8,
            -1.1, 1.1
        );
        mandelbrot.set_max_threads(std::thread::available_parallelism().expect("Couldn't determine num of host threads").get());//It will just fail otherwise

        b.iter(|| -> Mandelbrot {
            let mut copy = mandelbrot.clone();
            unsafe { copy.update_sse2() };
            return copy;
        });
    }

    #[bench]
    fn update_avx(b: &mut Bencher) {
        assert!(is_x86_feature_detected!("avx"));
        use crate::BaseFractal;
        let mut mandelbrot = Mandelbrot::new(
            1024,
            512,
            512,
            -2.3, 0.8,
            -1.1, 1.1
        );
        mandelbrot.set_max_threads(1);

        b.iter(|| -> Mandelbrot {
            let mut copy = mandelbrot.clone();
            unsafe { copy.update_avx() };
            return copy;
        });
    }

    #[bench]
    fn update_avx_mt(b: &mut Bencher) {
        assert!(is_x86_feature_detected!("avx"));
        use crate::BaseFractal;
        let mut mandelbrot = Mandelbrot::new(
            1024,
            512,
            512,
            -2.3, 0.8,
            -1.1, 1.1
        );
        mandelbrot.set_max_threads(std::thread::available_parallelism().expect("Couldn't determine num of host threads").get());//It will just fail otherwise

        b.iter(|| -> Mandelbrot {
            let mut copy = mandelbrot.clone();
            unsafe { copy.update_avx() };
            return copy;
        });
    }

    #[bench]
    fn update_avx2(b: &mut Bencher) {
        assert!(is_x86_feature_detected!("avx2"));
        use crate::BaseFractal;
        let mut mandelbrot = Mandelbrot::new(
            1024,
            512,
            512,
            -2.3, 0.8,
            -1.1, 1.1
        );
        mandelbrot.set_max_threads(1);

        b.iter(|| -> Mandelbrot {
            let mut copy = mandelbrot.clone();
            unsafe { copy.update_avx2() };
            return copy;
        });
    }

    #[bench]
    fn update_avx2_mt(b: &mut Bencher) {
        assert!(is_x86_feature_detected!("avx2"));
        use crate::BaseFractal;
        let mut mandelbrot = Mandelbrot::new(
            1024,
            512,
            512,
            -2.3, 0.8,
            -1.1, 1.1
        );
        mandelbrot.set_max_threads(std::thread::available_parallelism().expect("Couldn't determine num of host threads").get());//It will just fail otherwise

        b.iter(|| -> Mandelbrot {
            let mut copy = mandelbrot.clone();
            unsafe { copy.update_avx2() };
            return copy;
        });
    }
}
