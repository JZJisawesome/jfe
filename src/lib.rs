/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

mod mandelbrot;

use std::ops::Index;

pub use mandelbrot::Mandelbrot;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

//TODO create a CustomFunctionFractal that allows the user to define custom (but slower) fractals

pub trait Fractal {//Fractals that provide iteration counts only
    type FractalFloatType;

    //Constructor
    fn new(
        max_iterations: usize,
        x_pixels: usize, y_pixels: usize,
        min_real: Self::FractalFloatType, max_real: Self::FractalFloatType,
        min_imag: Self::FractalFloatType, max_imag: Self::FractalFloatType
    ) -> Self;

    //Getters
    fn get_max_iterations(self: &Self) -> usize;
    fn get_x_pixels(self: &Self) -> usize;
    fn get_y_pixels(self: &Self) -> usize;
    fn get_min_real(self: &Self) -> Self::FractalFloatType;
    fn get_max_real(self: &Self) -> Self::FractalFloatType;
    fn get_min_imag(self: &Self) -> Self::FractalFloatType;
    fn get_max_imag(self: &Self) -> Self::FractalFloatType;
    fn get_max_threads(self: &Self) -> usize;

    //Setters
    fn set_max_iterations(self: &mut Self, max_iterations: usize);
    fn set_x_pixels(self: &mut Self, x_pixels: usize);
    fn set_y_pixels(self: &mut Self, y_pixels: usize);
    fn set_min_real(self: &mut Self, min_real: Self::FractalFloatType);
    fn set_max_real(self: &mut Self, max_real: Self::FractalFloatType);
    fn set_min_imag(self: &mut Self, min_imag: Self::FractalFloatType);
    fn set_max_imag(self: &mut Self, max_imag: Self::FractalFloatType);
    fn set_max_threads(self: &mut Self, max_threads: usize);

    //Access Iteration Storage
    fn iterations_ref(self: &Self) -> Option::<&[usize]>;//Returns None if update_iterations() wasn't called since the last change to arguments/since construction

    //Update Iterations
    fn update_iterations(self: &mut Self);
}

/*impl<T> Index<usize> for T where T: Fractal {
    type Output = usize;

    fn index(self: &Self, index: usize) -> &Self::Output {
        return self.iterations_ref().index(index);
    }
}
*/

pub trait FractalKeepingLastValue: Fractal {
    //TODO this also keeps the last computed value of Z for use with certain colouring algorithms
}

/* Associated Functions and Methods */

//TODO

/* Functions */

//TODO
