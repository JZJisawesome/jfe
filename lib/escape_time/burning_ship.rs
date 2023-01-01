/* burning_ship.rs
 * By: John Jekel
 * Copyright (C) 2022-2023 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Burning Ship Escape Time Fractal
 *
*/

/* Imports */

use std::ops::IndexMut;

use crate::BaseFractal;
use super::EscapeTimeFractal;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

#[derive(Debug)]
pub struct BurningShip {
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

impl BurningShip {
    //NOTE: it is okay if min/max real/imag values are flipped, it will just flip the image
    pub fn new(
        max_iterations: usize,
        x_samples: usize, y_samples: usize,
        min_real: f64, max_real: f64,
        min_imag: f64, max_imag: f64
    ) -> BurningShip {
        assert!(max_iterations > 0, "Must at least iterate once");
        assert!(x_samples != 0, "x_samples must be non-zero");
        assert!(y_samples != 0, "y_samples must be non-zero");

        let mut new_iterations_vec = Vec::<usize>::with_capacity(x_samples * y_samples);
        new_iterations_vec.resize(x_samples * y_samples, 0);

        return BurningShip {
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

    fn burning_ship_iterations(self: &Self, c_real: f64, c_imag: f64) -> usize {//Returns MAX_ITERATIONS if it is bounded
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
            //z_(n+1) = (abs(Re(z_n)) + i(abs(Im(z_n))))^2 + c
            let next_z_real = (z_real * z_real) - (z_imag * z_imag) + c_real;
            let next_z_imag = (2.0 * z_real.abs() * z_imag.abs()) + c_imag;
            z_real = next_z_real;
            z_imag = next_z_imag;
            //println!("iteration {} ends: z_real {}, z_imag {}", i, z_real, z_imag);
            i += 1;
        }
        //println!("mandelbrot ends returning {}", i);
        return i;
    }

    #[inline(always)]
    fn at(self: &mut Self, x: usize, y: usize) -> &mut usize {//unchecked for speed in release builds
        debug_assert!(x < self.x_samples);
        debug_assert!(y < self.y_samples);
        return self.iterations.index_mut(x + (y * self.x_samples));
    }
}

impl BaseFractal for BurningShip {
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
        let real_length: f64 = self.max_real - self.min_real;
        let real_step_amount: f64 = real_length / (self.x_samples as f64);
        let imag_length: f64 = self.max_imag - self.min_imag;
        let imag_step_amount: f64 = imag_length / (self.y_samples as f64);

        let mut c_real: f64 = self.min_real;
        for x in 0..self.x_samples {
            let mut c_imag: f64 = self.min_imag;
            for y in 0..self.y_samples {
                *self.at(x, y) = self.burning_ship_iterations(c_real, c_imag);
                c_imag += imag_step_amount;
            }
            c_real += real_step_amount;
        }
        self.update_pending = false;
    }
}

impl EscapeTimeFractal for BurningShip {
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
        assert!(max_iterations > 0, "Must at least iterate once");
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
