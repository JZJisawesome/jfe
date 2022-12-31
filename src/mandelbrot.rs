/* mandelbrot.rs
 * By: John Jekel
 * Copyright (C) 2022-2023 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Mandelbrot Escape Time Fractal
 *
*/

/* Imports */

mod universal;
mod amd64;

use crate::BaseFractal;
use crate::EscapeTimeFractal;

use std::thread;

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

type LineWorkload<'a> = (&'a mut [usize], f64);
type Workload<'a> = Vec::<LineWorkload<'a>>;

/* Associated Functions and Methods */

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

    fn update_with_line_function(self: &mut Self, function: fn(usize, f64, f64, Workload)) {
        unsafe { self.update_with_unsafe_line_function(function); }//Safe since the line is safe
    }

    unsafe fn update_with_unsafe_line_function(self: &mut Self, function: unsafe fn(usize, f64, f64, Workload)) {
        let real_length: f64 = self.max_real - self.min_real;
        let real_step_amount: f64 = real_length / (self.x_samples as f64);//Per line
        let imag_length: f64 = self.max_imag - self.min_imag;
        let imag_step_amount: f64 = imag_length / (self.y_samples as f64);

        debug_assert!(self.max_threads != 0);
        if self.max_threads == 1 {
            let mut st_workload = Workload::with_capacity(self.y_samples);
            //Split into horizontal lines, pushing each to a single Workload
            let mut c_imag: f64 = self.min_imag;
            let mut counter_across_threads = 0;
            for line_slice in self.iterations.chunks_mut(self.x_samples) {
                st_workload.push((line_slice, c_imag));

                c_imag += imag_step_amount;

                counter_across_threads += 1;
                if counter_across_threads == self.max_threads {
                    counter_across_threads = 0;
                }
            }

            //Execute the workload
            function(self.max_iterations, self.min_real, real_step_amount, st_workload);
        } else {
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
                        function(
                            max_iterations_copy, min_real_copy, real_step_amount_copy,
                            workload
                        );
                    });
                }
            });
            self.update_pending = false;
        }
    }
}

impl BaseFractal for Mandelbrot {
    //Getters
    fn get_max_threads(self: &Self) -> usize {
        return self.max_threads;
    }

    //Setters
    fn set_max_threads(self: &mut Self, max_threads: usize) {
        if max_threads != 0 {
            self.max_threads = max_threads;
        } else {
            self.max_threads = std::thread::available_parallelism().expect("Failed to auto-determine the max number of threads to use when calling set_max_threads with 0").get();
        }
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
    fn create_mandelbrot(b: &mut Bencher) {
        b.iter(|| -> Mandelbrot {
            use crate::BaseFractal;
            let mut mandelbrot = Mandelbrot::new(
                1024,
                128,
                128,
                -2.3, 0.8,
                -1.1, 1.1
            );
            mandelbrot.set_max_threads(1);

            return mandelbrot;
        });
    }

    #[bench]
    fn copy_overhead(b: &mut Bencher) {
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
            let copy = mandelbrot.clone();
            return copy;
        });
    }
}
