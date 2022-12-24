/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

pub mod mandelbrot;
mod numerical_traits;
pub mod ini;

use std::ops::Index;

use crate::numerical_traits::Integer;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

//TODO create a CustomFunctionFractal that allows the user to define custom (but slower) fractals

pub trait Fractal {//Fractals that provide iteration counts only
    type FractalDotType: Integer;
    type FractalFloatType;

    //Constructor
    fn new(
        iterations: usize,
        x_dots: usize, y_dots: usize,
        min_real: Self::FractalFloatType, max_real: Self::FractalFloatType,
        min_imag: Self::FractalFloatType, max_imag: Self::FractalFloatType
    ) -> Self;

    //Getters
    fn get_iterations(self: &Self) -> usize;//Meaning depends on the fractal
    fn get_x_dots(self: &Self) -> usize;
    fn get_y_dots(self: &Self) -> usize;
    fn get_min_real(self: &Self) -> Self::FractalFloatType;
    fn get_max_real(self: &Self) -> Self::FractalFloatType;
    fn get_min_imag(self: &Self) -> Self::FractalFloatType;
    fn get_max_imag(self: &Self) -> Self::FractalFloatType;

    //Setters
    fn set_iterations(self: &mut Self, max_iterations: usize);//Meaning depends on the fractal
    fn set_x_dots(self: &mut Self, x_dots: usize);
    fn set_y_dots(self: &mut Self, y_dots: usize);
    fn set_min_real(self: &mut Self, min_real: Self::FractalFloatType);
    fn set_max_real(self: &mut Self, max_real: Self::FractalFloatType);
    fn set_min_imag(self: &mut Self, min_imag: Self::FractalFloatType);
    fn set_max_imag(self: &mut Self, max_imag: Self::FractalFloatType);

    //The meaning of what a dot is depends on the fractal

    //Access Dots Storage
    fn dots_ref(self: &Self) -> Option::<&[Self::FractalDotType]>;//Returns None if update() wasn't called since the last change to arguments/since construction

    //Update Dots
    fn update_dots(self: &mut Self);
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
