/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use std::ops::IndexMut;

use crate::BaseFractal;
use crate::EscapeTimeFractal;

use core::arch::x86_64;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

#[derive(Debug, Clone)]
pub struct Mandelbrot {
    max_iterations: usize,
    x_samples: usize,
    y_samples: usize,
    min_real: f64,
    min_imag: f64,
    max_real: f64,
    max_imag: f64,
    max_threads: usize,


    iterations: Vec::<usize>,//For cheap resizing in case the user changes x_samples or y_samples
    update_pending: bool,
}

impl Mandelbrot {
    //NOTE: it is okay if min/max real/imag values are flipped, it will just flip the image
    pub fn new(
        max_iterations: usize,
        x_samples: usize, y_samples: usize,
        min_real: f64, max_real: f64,
        min_imag: f64, max_imag: f64
    ) -> Mandelbrot {
        assert!(x_samples != 0, "x_samples must be non-zero");
        assert!(y_samples != 0, "y_samples must be non-zero");

        let mut new_iterations_vec = Vec::<usize>::with_capacity(x_samples * y_samples);
        new_iterations_vec.resize(x_samples * y_samples, 0);

        return Mandelbrot {
            max_iterations: max_iterations,
            x_samples: x_samples,
            y_samples: y_samples,
            min_real: min_real,
            min_imag: min_imag,
            max_real: max_real,
            max_imag: max_imag,
            max_threads: 1,

            iterations: new_iterations_vec,
            update_pending: true
        };
    }

    #[inline(always)]
    fn mandelbrot_iterations_universal(self: &Self, c_real: f64, c_imag: f64) -> usize {//Returns MAX_ITERATIONS if it is bounded
        //println!("mandelbrot iteration with params: {} {}", c_real, c_imag);
        let diverge_threshold: f64 = 2.0;//TODO make this flexible?

        //z_0 = 0
        let mut z_real: f64 = 0.0;
        let mut z_imag: f64 = 0.0;

        //We exit the loop in two cases: if we reach MAX_ITERATIONS (meaning we assume the c value produces a bounded series)
        //or the modulus of the complex number exceeds the diverge_threshold (meaning the c value produces an unbounded series)
        let mut i: usize = 0;
        while (i < self.max_iterations) && (((z_real * z_real) + (z_imag * z_imag)) < (diverge_threshold * diverge_threshold)) {
            //println!("iteration {} starts: z_real {}, z_imag {}", i, z_real, z_imag);
            //z_(n+1) = z_n^2 + c
            let next_z_real = (z_real * z_real) - (z_imag * z_imag) + c_real;
            let next_z_imag = (2.0 * z_real * z_imag) + c_imag;
            z_real = next_z_real;
            z_imag = next_z_imag;
            //println!("iteration {} ends: z_real {}, z_imag {}", i, z_real, z_imag);
            i += 1;
        }
        //println!("mandelbrot ends returning {}", i);
        return i;
    }

    //#[inline(always)]//Can't do this with the second "#[target_feature(enable = "sse2")]"
    #[inline]//But this is okay
    #[target_feature(enable = "sse2")]
    unsafe fn mandelbrot_iterations_sse2(self: &Self, c_real_f: x86_64::__m128d, c_imag_f: x86_64::__m128d) -> x86_64::__m128i {//Returns MAX_ITERATIONS if it is bounded
        let diverge_threshold: f64 = 2.0;//TODO make this flexible?

        unsafe {
            let diverge_threshold_squared_f = x86_64::_mm_set_pd1(diverge_threshold * diverge_threshold);
            let two_f = x86_64::_mm_set_pd1(2.0);
            let one_i = x86_64::_mm_set1_epi64x(1);

            let mut result_i = x86_64::_mm_set1_epi64x(0);

            let mut z_real_f = x86_64::_mm_set_pd1(0.0);
            let mut z_imag_f = x86_64::_mm_set_pd1(0.0);
            for i in 0..self.max_iterations {
                //Calculate some values that are used below
                let z_real_squared_f = x86_64::_mm_mul_pd(z_real_f, z_real_f);
                let z_imag_squared_f = x86_64::_mm_mul_pd(z_imag_f, z_imag_f);

                //Check if the modulus of each z < the diverge value (aka that they haven't diverged)
                //We do this faster by doing (z_real * z_real) + (z_imag * z_imag) < (2 * 2)
                let squared_sum = x86_64::_mm_add_pd(z_real_squared_f, z_imag_squared_f);
                let compare = x86_64::_mm_castpd_si128(x86_64::_mm_cmplt_pd(squared_sum, diverge_threshold_squared_f));

                //If both complex numbers have diverged (entire vector is 0), return
                if x86_64::_mm_movemask_epi8(compare) == 0 {
                    break;
                }

                //Get next entries (For each complex number z, z_(n+1) = z_n^2 + c)
                let temp_z_real_f = x86_64::_mm_add_pd(x86_64::_mm_sub_pd(z_real_squared_f, z_imag_squared_f), c_real_f);
                z_imag_f = x86_64::_mm_add_pd(c_imag_f, x86_64::_mm_mul_pd(two_f, x86_64::_mm_mul_pd(z_real_f, z_imag_f)));
                z_real_f = temp_z_real_f;

                //Increment the corresponding count only if we haven't converged yet
                let incrementor_i = x86_64::_mm_and_si128(compare, one_i);//If a number hasn't converged, we will increment it's count
                result_i = x86_64::_mm_add_epi64(result_i, incrementor_i);
            }

            return result_i;
        }
    }

    #[target_feature(enable = "sse2")]
    unsafe fn update_sse2(self: &mut Self) {
        assert!((self.x_samples & 0b1) == 0);//TODO overcome this limitation
        //TESTING
        //unsafe { self.mandelbrot_iterations_sse2(x86_64::_mm_set_pd(0.3, 1.4), x86_64::_mm_set_pd(1234.45, 3.141592)); }

        let real_length: f64 = self.max_real - self.min_real;
        let real_step_amount: f64 = real_length / (self.x_samples as f64);
        let imag_length: f64 = self.max_imag - self.min_imag;
        let imag_step_amount: f64 = imag_length / (self.y_samples as f64);

        let iterations_pointer = self.iterations.as_mut_ptr();

        let real_step_amount_vector = x86_64::_mm_set_pd1(real_step_amount * 2.0);//x2 since we process two at a time
        let imag_step_amount_vector = x86_64::_mm_set_pd1(imag_step_amount);//x2 since we process two at a time

        let mut c_real = x86_64::_mm_set_pd(self.min_real, self.min_real + real_step_amount);
        for x in (0..self.x_samples).step_by(2) {
            let mut c_imag = x86_64::_mm_set_pd(self.min_imag, self.min_imag + imag_step_amount);
            for y in 0..self.y_samples {
                let result = self.mandelbrot_iterations_sse2(c_real, c_imag);
                let pointer = iterations_pointer.offset((x + (y * self.x_samples)) as isize) as *mut x86_64::__m128i;
                x86_64::_mm_storeu_si128(pointer, result);
                c_imag = x86_64::_mm_add_pd(c_imag, imag_step_amount_vector);
            }
            c_real = x86_64::_mm_add_pd(c_real, real_step_amount_vector);
        }

        self.update_pending = false;
    }

    unsafe fn update_x86_64(self: &mut Self) {
        if is_x86_feature_detected!("avx") {//TODO
            self.update_sse2();//todo!();
        } else if is_x86_feature_detected!("sse4.2") {//TODO
            self.update_sse2();//todo!();
        } else {//On x86_64, we can assume SSE2
            self.update_sse2();
        }
    }

    fn update_universal(self: &mut Self) {
        let real_length: f64 = self.max_real - self.min_real;
        let real_step_amount: f64 = real_length / (self.x_samples as f64);
        let imag_length: f64 = self.max_imag - self.min_imag;
        let imag_step_amount: f64 = imag_length / (self.y_samples as f64);

        let mut c_real: f64 = self.min_real;
        for x in 0..self.x_samples {
            let mut c_imag: f64 = self.min_imag;
            for y in 0..self.y_samples {
                *self.at(x, y) = self.mandelbrot_iterations_universal(c_real, c_imag);
                c_imag += imag_step_amount;
            }
            c_real += real_step_amount;
        }
        self.update_pending = false;
    }

    #[inline(always)]
    fn at(self: &mut Self, x: usize, y: usize) -> &mut usize {//unchecked for speed in release builds
        debug_assert!(x < self.x_samples);
        debug_assert!(y < self.y_samples);
        return self.iterations.index_mut(x + (y * self.x_samples));
    }
}

impl BaseFractal for Mandelbrot {
    //Getters
    fn get_max_threads(self: &Self) -> usize {
        return self.max_threads;
    }

    //Setters
    fn set_max_threads(self: &mut Self, max_threads: usize) {
        self.max_threads = max_threads;
    }

    //Update Samples
    fn update(self: &mut Self) {
        if cfg!(target_arch = "x86_64") {
            unsafe { self.update_x86_64(); }
        } else {
            self.update_universal();
        }
    }
}

impl EscapeTimeFractal for Mandelbrot {
    //Getters
    fn get_max_iterations(self: &Self) -> usize {
        return self.max_iterations;
    }

    fn get_x_samples(self: &Self) -> usize {
        return self.x_samples;
    }

    fn get_y_samples(self: &Self) -> usize {
        return self.y_samples;
    }

    fn get_min_x(self: &Self) -> f64 {
        return self.min_real;
    }

    fn get_max_x(self: &Self) -> f64 {
        return self.min_imag;
    }

    fn get_min_y(self: &Self) -> f64 {
        return self.max_real;
    }

    fn get_max_y(self: &Self) -> f64 {
        return self.max_imag;
    }

    //Setters
    //TODO only set update_pending if it changed
    fn set_max_iterations(self: &mut Self, max_iterations: usize) {
        self.max_iterations = max_iterations;
        self.update_pending = true;
    }

    fn set_x_samples(self: &mut Self, x_samples: usize) {
        assert!(x_samples != 0, "x_samples must be non-zero");
        self.x_samples = x_samples;
        self.update_pending = true;
        self.iterations.reserve(x_samples * self.y_samples);
        self.iterations.resize(x_samples * self.y_samples, 0);
    }

    fn set_y_samples(self: &mut Self, y_samples: usize) {
        assert!(y_samples != 0, "y_samples must be non-zero");
        self.y_samples = y_samples;
        self.update_pending = true;
        self.iterations.reserve(self.x_samples * y_samples);
        self.iterations.resize(self.x_samples * y_samples, 0);
    }

    fn set_min_x(self: &mut Self, min_real: f64) {
        self.min_real = min_real;
        self.update_pending = true;
    }

    fn set_max_x(self: &mut Self, max_real: f64) {
        self.max_real = max_real;
        self.update_pending = true;
    }

    fn set_min_y(self: &mut Self, min_imag: f64) {
        self.min_imag = min_imag;
        self.update_pending = true;
    }

    fn set_max_y(self: &mut Self, max_imag: f64) {
        self.max_imag = max_imag;
        self.update_pending = true;
    }

    //Access Samples Storage
    fn samples_ref(self: &Self) -> Option::<&[usize]> {//Returns None if update() wasn't called since the last change to arguments/since construction
        if self.update_pending {
            return None;
        }

        return Some(&self.iterations);
    }
}

/* Associated Functions and Methods */

//TODO

/* Functions */

/* Benches */

//TODO
#[cfg_attr(feature = "nightly-features-benches", cfg(test))]
#[cfg(feature = "nightly-features-benches")]
mod benches {
    extern crate test;
    use test::Bencher;
    use super::*;

    #[bench]
    fn create_mandelbrot(b: &mut Bencher) {
        b.iter(|| -> Mandelbrot {
            let mandelbrot = Mandelbrot::new(
                1024,
                128,
                128,
                -2.3, 0.8,
                -1.1, 1.1
            );

            return mandelbrot;
        });
    }

    #[bench]
    fn copy_overhead(b: &mut Bencher) {
        let mandelbrot = Mandelbrot::new(
            1024,
            128,
            128,
            -2.3, 0.8,
            -1.1, 1.1
        );

        b.iter(|| -> Mandelbrot {
            let copy = mandelbrot.clone();
            return copy;
        });
    }

    #[bench]
    #[cfg(target_arch = "x86_64")]
    #[cfg(target_feature = "sse2")]
    fn update_sse2(b: &mut Bencher) {
        let mandelbrot = Mandelbrot::new(
            1024,
            128,
            128,
            -2.3, 0.8,
            -1.1, 1.1
        );

        b.iter(|| -> Mandelbrot {
            let mut copy = mandelbrot.clone();
            unsafe { copy.update_sse2() };
            return copy;
        });
    }

    #[bench]
    fn update_universal(b: &mut Bencher) {
        let mandelbrot = Mandelbrot::new(
            1024,
            128,
            128,
            -2.3, 0.8,
            -1.1, 1.1
        );

        b.iter(|| -> Mandelbrot {
            let mut copy = mandelbrot.clone();
            copy.update_universal();
            return copy;
        });
    }
}
