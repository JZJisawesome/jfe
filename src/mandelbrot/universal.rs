/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use super::Mandelbrot;

use std::thread;
//use std::sync::mpsc;
use std::thread::JoinHandle;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

//TODO

/* Associated Functions and Methods */

impl Mandelbrot {
    #[inline(always)]
    fn mandelbrot_iterations_universal(max_iterations: usize, c_real: f64, c_imag: f64) -> usize {//Returns MAX_ITERATIONS if it is bounded
        //println!("mandelbrot iteration with params: {} {}", c_real, c_imag);
        let diverge_threshold: f64 = 2.0;//TODO make this flexible?

        //z_0 = 0
        let mut z_real: f64 = 0.0;
        let mut z_imag: f64 = 0.0;

        //We exit the loop in two cases: if we reach MAX_ITERATIONS (meaning we assume the c value produces a bounded series)
        //or the modulus of the complex number exceeds the diverge_threshold (meaning the c value produces an unbounded series)
        let mut i: usize = 0;
        while (i < max_iterations) && (((z_real * z_real) + (z_imag * z_imag)) < (diverge_threshold * diverge_threshold)) {
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

    pub(super) fn update_universal(self: &mut Self) {
        if self.max_threads == 1 {
            self.update_universal_st();
        } else {
            self.update_universal_mt();
        }
    }

    fn update_universal_st(self: &mut Self) {
        let real_length: f64 = self.max_real - self.min_real;
        let real_step_amount: f64 = real_length / (self.x_samples as f64);
        let imag_length: f64 = self.max_imag - self.min_imag;
        let imag_step_amount: f64 = imag_length / (self.y_samples as f64);

        let mut c_real: f64 = self.min_real;
        for x in 0..self.x_samples {
            let mut c_imag: f64 = self.min_imag;
            for y in 0..self.y_samples {
                *self.at(x, y) = Self::mandelbrot_iterations_universal(self.max_iterations, c_real, c_imag);
                c_imag += imag_step_amount;
            }
            c_real += real_step_amount;
        }
        self.update_pending = false;
    }

    fn update_universal_mt(self: &mut Self) {
        let real_length: f64 = self.max_real - self.min_real;
        let real_step_amount: f64 = real_length / (self.x_samples as f64);//Per line
        let imag_length: f64 = self.max_imag - self.min_imag;
        let imag_step_amount: f64 = imag_length / (self.y_samples as f64);

        type LineWorkload<'a> = (&'a mut [usize], f64);
        type Workload<'a> = Vec::<LineWorkload<'a>>;

        let mut workloads = Vec::<Workload>::with_capacity(self.max_threads);
        workloads.resize_with(self.max_threads, || { Vec::<LineWorkload>::new() });

        //Distribute work by splitting into lines
        //Split into horizontal lines
        let mut c_imag: f64 = self.min_imag;
        let mut counter_across_threads = 0;
        for line_slice in self.iterations.chunks_mut(self.x_samples) {
            workloads[counter_across_threads].push((line_slice, c_imag));

            c_imag += imag_step_amount;

            counter_across_threads += 1;
            if counter_across_threads == self.max_threads {
                counter_across_threads = 0;
            }
        }

        //Create threads and join them at the end of the scope
        debug_assert!(workloads.len() == self.max_threads);
        thread::scope(|s| {
            while let Some(workload) = workloads.pop() {
                let max_iterations_copy = self.max_iterations;
                let min_real_copy = self.min_real;
                let real_step_amount_copy = real_step_amount;

                s.spawn(move || {
                    Self::update_universal_mt_thread(
                        max_iterations_copy, min_real_copy, real_step_amount_copy,
                        workload
                    );
                });
            }
        });
        self.update_pending = false;
    }

    fn update_universal_mt_thread(max_iterations: usize, starting_c_real: f64, real_step_amount: f64, workload: Vec::<(&mut [usize], f64)>) {
        for (line_slice, c_imag) in workload {
            let mut c_real: f64 = starting_c_real;
            for x in 0..line_slice.len() {
                line_slice[x] = Self::mandelbrot_iterations_universal(max_iterations, c_real, c_imag);
                c_real += real_step_amount;
            }
        }
    }
}

/* Functions */

//TODO

/* Benches */

#[cfg_attr(feature = "nightly-features-benches", cfg(test))]
#[cfg(feature = "nightly-features-benches")]
mod benches {
    extern crate test;
    use test::Bencher;
    use super::*;

    #[bench]
    fn update_universal(b: &mut Bencher) {
        use crate::BaseFractal;
        let mut mandelbrot = Mandelbrot::new(
            1024,
            128,
            128,
            -2.3, 0.8,
            -1.1, 1.1
        );
        mandelbrot.set_max_threads(1);

        b.iter(|| -> Mandelbrot {
            let mut copy = mandelbrot.clone();
            copy.update_universal();
            return copy;
        });
    }

    #[bench]
    fn update_universal_threaded(b: &mut Bencher) {
        use crate::BaseFractal;
        let mut mandelbrot = Mandelbrot::new(
            1024,
            128,
            128,
            -2.3, 0.8,
            -1.1, 1.1
        );
        mandelbrot.set_max_threads(std::thread::available_parallelism().expect("Couldn't determine num of host threads").get());//It will just fail otherwise

        b.iter(|| -> Mandelbrot {
            let mut copy = mandelbrot.clone();
            copy.update_universal();
            return copy;
        });
    }
}
