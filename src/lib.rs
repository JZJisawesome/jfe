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
//pub mod fractal_names;
use std::any::Any;//Overhead of Any is minimal compared to slow speed of updating fractals
use std::fmt::Debug;

use std::ops::Index;

use crate::numerical_traits::Integer;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

#[derive(Debug)]
pub enum FractalBox {
    IFS(Box<dyn IFSFractal>),
    StrangeAttractor(Box<dyn StrangeAttractorFractal>),
    EscapeTime(Box<dyn EscapeTimeFractal>),
    Random(Box<dyn RandomFractal>),
    FiniteSubdivision(Box<dyn FiniteSubdivisionFractal>)
}

pub trait BaseFractal: Debug {//Common between all fractal types
    //Getters
    fn get_max_threads(self: &Self) -> usize;//0 means all available

    //Setters
    fn set_max_threads(self: &mut Self, max_threads: usize);//0 means all available

    //Update the fractal when settings were changed
    fn update(self: &mut Self);
}

pub trait IFSFractal: BaseFractal {
    //TODO
}

pub trait StrangeAttractorFractal: BaseFractal {
    //TODO
}

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

pub trait RandomFractal: BaseFractal {
    //TODO
}

pub trait FiniteSubdivisionFractal: BaseFractal {
    //TODO
}
/*
pub trait AnyBaseFractal : Any + BaseFractal {}//Overhead of Any is minimal compared to slow speed of updating fractals

//pub type Fractal = IterationCountFractal;
//pub trait IterationCountFractal {//Fractals that provide iteration counts only
pub trait BaseFractal {//Fractals that provide dots only (the bare minimum)
    //Constructor
    /*fn new(
        iterations: usize,
        x_dots: usize, y_dots: usize,
        min_real: f64, max_real: f64,
        min_imag: f64, max_imag: f64
    ) -> Self;
    */

    //Getters
    fn get_iterations(self: &Self) -> usize;//Meaning depends on the fractal
    fn get_x_dots(self: &Self) -> usize;
    fn get_y_dots(self: &Self) -> usize;
    fn get_min_real(self: &Self) -> f64;
    fn get_max_real(self: &Self) -> f64;
    fn get_min_imag(self: &Self) -> f64;
    fn get_max_imag(self: &Self) -> f64;

    //Setters
    fn set_iterations(self: &mut Self, max_iterations: usize);//Meaning depends on the fractal
    fn set_x_dots(self: &mut Self, x_dots: usize);
    fn set_y_dots(self: &mut Self, y_dots: usize);
    fn set_min_real(self: &mut Self, min_real: f64);
    fn set_max_real(self: &mut Self, max_real: f64);
    fn set_min_imag(self: &mut Self, min_imag: f64);
    fn set_max_imag(self: &mut Self, max_imag: f64);

    //The meaning of what a dot is depends on the fractal

    //Access Dots Storage
    fn dots_ref(self: &Self) -> Option::<&[usize]>;//Returns None if update() wasn't called since the last change to arguments/since construction

    //Update Dots
    fn update_dots(self: &mut Self);
}

pub trait FractalKeepingLastValue: BaseFractal {
    //TODO this also keeps the last computed value of Z for use with certain colouring algorithms
}

/*pub enum NewOldFractal {
    Double(dyn FractalType<f64>),
    Float(dyn FractalType<f32>)
}
*/

pub trait OldFractalType<FloatType> {
    //Constructor
    fn new(
        iterations: usize,
        x_dots: usize, y_dots: usize,
        min_real: FloatType, max_real: FloatType,
        min_imag: FloatType, max_imag: FloatType
    ) -> Self;

    //Getters
    fn get_iterations(self: &Self) -> usize;//Meaning depends on the fractal
    fn get_x_dots(self: &Self) -> usize;
    fn get_y_dots(self: &Self) -> usize;
    fn get_min_real(self: &Self) -> FloatType;
    fn get_max_real(self: &Self) -> FloatType;
    fn get_min_imag(self: &Self) -> FloatType;
    fn get_max_imag(self: &Self) -> FloatType;

    //Setters
    fn set_iterations(self: &mut Self, max_iterations: usize);//Meaning depends on the fractal
    fn set_x_dots(self: &mut Self, x_dots: usize);
    fn set_y_dots(self: &mut Self, y_dots: usize);
    fn set_min_real(self: &mut Self, min_real: FloatType);
    fn set_max_real(self: &mut Self, max_real: FloatType);
    fn set_min_imag(self: &mut Self, min_imag: FloatType);
    fn set_max_imag(self: &mut Self, max_imag: FloatType);

    //The meaning of what a dot is depends on the fractal

    //Access Dots Storage
    fn dots_ref(self: &Self) -> Option::<&[usize]>;//Returns None if update() wasn't called since the last change to arguments/since construction

    //Update Dots
    fn update_dots(self: &mut Self);
}

//TODO create a CustomFunctionFractal that allows the user to define custom (but slower) fractals

pub trait OldFractal {//Fractals that provide iteration counts only
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
*/
//pub trait FractalKeepingLastValue: Fractal {
    //TODO this also keeps the last computed value of Z for use with certain colouring algorithms
//}

/* Associated Functions and Methods */

/*impl FractalBox {
    fn as_base_mut(self: &mut Self) -> &mut dyn BaseFractal {
        match self {
            FractalBox::IFS(IFSFractalBox) => { return IFSFractalBox.as_mut(); },
            FractalBox::StrangeAttractor(StrangeAttractorFractalBox) => { return StrangeAttractorFractalBox.as_mut(); },
            /*FractalBox::IFS(IFSFractalBox) => { return IFSFractalBox.get_max_threads(); },
            FractalBox::IFS(IFSFractalBox) => { return IFSFractalBox.get_max_threads(); },
            FractalBox::IFS(IFSFractalBox) => { return IFSFractalBox.get_max_threads(); },*/
        }
    }

}
*/

impl BaseFractal for FractalBox {
    //Getters
    fn get_max_threads(self: &Self) -> usize {
        match self {
            FractalBox::IFS(ifs_fractal_box) => { return ifs_fractal_box.get_max_threads(); },
            FractalBox::StrangeAttractor(strange_attractor_box) => { return strange_attractor_box.get_max_threads(); },
            FractalBox::EscapeTime(escape_time_fractal_box) => { return escape_time_fractal_box.get_max_threads(); },
            FractalBox::Random(random_fractal_box) => { return random_fractal_box.get_max_threads(); },
            FractalBox::FiniteSubdivision(finite_subdivision_fractal_box) => { return finite_subdivision_fractal_box.get_max_threads(); },
        }
    }

    //Setters
    fn set_max_threads(self: &mut Self, max_threads: usize) {
        match self {
            FractalBox::IFS(ifs_fractal_box) => { return ifs_fractal_box.set_max_threads(max_threads); },
            FractalBox::StrangeAttractor(strange_attractor_box) => { return strange_attractor_box.set_max_threads(max_threads); },
            FractalBox::EscapeTime(escape_time_fractal_box) => { return escape_time_fractal_box.set_max_threads(max_threads); },
            FractalBox::Random(random_fractal_box) => { return random_fractal_box.set_max_threads(max_threads); },
            FractalBox::FiniteSubdivision(finite_subdivision_fractal_box) => { return finite_subdivision_fractal_box.set_max_threads(max_threads); },
        }
    }

    //Update Samples
    fn update(self: &mut Self) {
        match self {
            FractalBox::IFS(ifs_fractal_box) => { return ifs_fractal_box.update(); },
            FractalBox::StrangeAttractor(strange_attractor_box) => { return strange_attractor_box.update(); },
            FractalBox::EscapeTime(escape_time_fractal_box) => { return escape_time_fractal_box.update(); },
            FractalBox::Random(random_fractal_box) => { return random_fractal_box.update(); },
            FractalBox::FiniteSubdivision(finite_subdivision_fractal_box) => { return finite_subdivision_fractal_box.update(); },
        }
    }
}

/* Functions */

//TODO
