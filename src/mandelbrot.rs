/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

mod universal;
mod amd64;

use std::ops::IndexMut;

use crate::BaseFractal;
use crate::EscapeTimeFractal;

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
