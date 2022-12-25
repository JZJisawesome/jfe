/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use crate::BaseFractal;
use crate::EscapeTimeFractal;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

#[derive(Debug)]
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
    pub fn new(
        max_iterations: usize,
        x_samples: usize, y_samples: usize,
        min_real: f64, max_real: f64,
        min_imag: f64, max_imag: f64
    ) -> Mandelbrot {
        assert!(max_iterations > 0, "Must at least iterate once");
        assert!(x_samples != 0, "x_samples must be non-zero");
        assert!(y_samples != 0, "y_samples must be non-zero");
        assert!(min_real < max_real, "min_real must be < max_real");
        assert!(min_imag < max_imag, "min_imag must be < max_imag");

        let mut new_iterations_vec = Vec::<usize>::with_capacity(x_samples * y_samples);
        new_iterations_vec.resize(x_samples * y_samples, 0);

        let mut new_mandelbrot = Mandelbrot {
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
        return new_mandelbrot;
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
        self.update_pending = false;
        todo!();
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
        assert!(max_iterations > 0, "Must at least iterate once");
        self.max_iterations = max_iterations;
        self.update_pending = true;
    }

    fn set_x_samples(self: &mut Self, x_samples: usize) {
        assert!(x_samples != 0, "x_samples must be non-zero");
        self.x_samples = x_samples;
        self.update_pending = true;
    }

    fn set_y_samples(self: &mut Self, y_samples: usize) {
        assert!(y_samples != 0, "y_samples must be non-zero");
        self.y_samples = y_samples;
        self.update_pending = true;
    }

    fn set_min_x(self: &mut Self, min_real: f64) {
        assert!(min_real < self.max_real, "min_real must be < max_real");
        self.min_real = min_real;
        self.update_pending = true;
    }

    fn set_max_x(self: &mut Self, max_real: f64) {
        assert!(self.min_real < max_real, "min_real must be < max_real");
        self.max_real = max_real;
        self.update_pending = true;
    }

    fn set_min_y(self: &mut Self, min_imag: f64) {
        assert!(min_imag < self.max_imag, "min_imag must be < max_imag");
        self.min_imag = min_imag;
        self.update_pending = true;
    }

    fn set_max_y(self: &mut Self, max_imag: f64) {
        assert!(self.min_imag < max_imag, "min_imag must be < max_imag");
        self.max_imag = max_imag;
        self.update_pending = true;
    }

    //Access Samples Storage
    fn samples_ref(self: &Self) -> Option::<&[usize]> {//Returns None if update() wasn't called since the last change to arguments/since construction
        if self.update_pending {
            return None;
        }

        todo!();
    }
}

/*pub struct AccurateMandelbrotFractal {
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

*/

/* Associated Functions and Methods */

//TODO

/* Functions */

//TODO
