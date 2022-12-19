/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use crate::Fractal;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub struct AccurateMandelbrotFractal {
    x_pixels: usize,
    y_pixels: usize,
    min_real: f64,
    min_imag: f64,
    max_real: f64,
    max_imag: f64,
    iterations: Vec::<usize>//For cheap resizing in case the user changes x_pixels or y_pixels
}

impl AccurateMandelbrotFractal {
    fn get_max_threads(self: &Self) -> usize {
        todo!();
    }

    fn set_max_threads(self: &mut Self, max_threads: usize) {
        todo!();
    }
}

impl Fractal for AccurateMandelbrotFractal {
    type FractalFloatType = f64;//High precision, but slow...

    //Constructor
    fn new(
        iterations: usize,
        x_pixels: usize, y_pixels: usize,
        min_real: Self::FractalFloatType, max_real: Self::FractalFloatType,
        min_imag: Self::FractalFloatType, max_imag: Self::FractalFloatType
    ) -> Self {
        todo!();
    }

    //Getters
    fn get_iterations(self: &Self) -> usize {
        todo!();
    }

    fn get_x_pixels(self: &Self) -> usize {
        todo!();
    }

    fn get_y_pixels(self: &Self) -> usize {
        todo!();
    }

    fn get_min_real(self: &Self) -> Self::FractalFloatType {
        todo!();
    }

    fn get_max_real(self: &Self) -> Self::FractalFloatType {
        todo!();
    }

    fn get_min_imag(self: &Self) -> Self::FractalFloatType {
        todo!();
    }

    fn get_max_imag(self: &Self) -> Self::FractalFloatType {
        todo!();
    }


    //Setters
    fn set_iterations(self: &mut Self, max_iterations: usize) {
        todo!();
    }

    fn set_x_pixels(self: &mut Self, x_pixels: usize) {
        todo!();
    }

    fn set_y_pixels(self: &mut Self, y_pixels: usize) {
        todo!();
    }

    fn set_min_real(self: &mut Self, min_real: Self::FractalFloatType) {
        todo!();
    }

    fn set_max_real(self: &mut Self, max_real: Self::FractalFloatType) {
        todo!();
    }

    fn set_min_imag(self: &mut Self, min_imag: Self::FractalFloatType) {
        todo!();
    }

    fn set_max_imag(self: &mut Self, max_imag: Self::FractalFloatType) {
        todo!();
    }


    //Access Iteration Storage
    fn iterations_ref(self: &Self) -> Option::<&[usize]> {//Returns None if update_iterations() wasn't called since the last change to arguments/since construction
        todo!();
    }

    //Update Iterations
    fn update_iterations(self: &mut Self) {
        todo!();
    }

}

/* Associated Functions and Methods */

//TODO

/* Functions */

//TODO
