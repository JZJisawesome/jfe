/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use super::Mandelbrot;

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
    fn mandelbrot_iterations_universal(self: &Self, c_real: f64, c_imag: f64) -> usize {//Returns MAX_ITERATIONS if it is bounded
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
        let real_length: f64 = self.max_real - self.min_real;
        let real_step_amount: f64 = real_length / (self.x_samples as f64);
        let imag_length: f64 = self.max_imag - self.min_imag;
        let imag_step_amount: f64 = imag_length / (self.y_samples as f64);

        let mut c_real: f64 = self.min_real;
        for x in 0..self.x_samples {
            let mut c_imag: f64 = self.min_imag;
            for y in 0..self.y_samples {
                *self.at(x, y) = self.mandelbrot_iterations_universal(c_real, c_imag);
                c_imag += imag_step_amount;
            }
            c_real += real_step_amount;
        }
        self.update_pending = false;
    }
}

/* Functions */

//TODO
