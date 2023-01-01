/* escape_time.rs
 * By: John Jekel
 * Copyright (C) 2022-2023 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Escape Time Fractals
 *
*/

/* Imports */

pub mod burning_ship;
pub mod mandelbrot;
pub mod standard_newton;
pub mod quadratic_julia;

use crate::BaseFractal;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub trait EscapeTimeFractal: BaseFractal {
    //Getters
    fn get_max_iterations(self: &Self) -> usize;
    fn get_x_samples(self: &Self) -> usize;
    fn get_y_samples(self: &Self) -> usize;
    fn get_min_x(self: &Self) -> f64;//For the x axis or the real axis
    fn get_max_x(self: &Self) -> f64;//For the x axis or the real axis
    fn get_min_y(self: &Self) -> f64;//For the y axis or the imaginary axis
    fn get_max_y(self: &Self) -> f64;//For the y axis or the imaginary axis

    //Setters
    fn set_max_iterations(self: &mut Self, max_iterations: usize);
    fn set_x_samples(self: &mut Self, x_samples: usize);
    fn set_y_samples(self: &mut Self, y_samples: usize);
    fn set_min_x(self: &mut Self, min_x: f64);//For the x axis or the real axis
    fn set_max_x(self: &mut Self, max_x: f64);//For the x axis or the real axis
    fn set_min_y(self: &mut Self, min_y: f64);//For the y axis or the imaginary axis
    fn set_max_y(self: &mut Self, max_y: f64);//For the y axis or the imaginary axis

    //Access Samples Storage
    fn samples_ref(self: &Self) -> Option::<&[usize]>;//Returns None if update() wasn't called since the last change to arguments/since construction
}

/* Associated Functions and Methods */

//TODO

/* Functions */

//TODO
