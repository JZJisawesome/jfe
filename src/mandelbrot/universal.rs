/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use super::Mandelbrot;
use super::Workload;

use std::thread;

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
    pub(super) fn update_universal(self: &mut Self) {
        self.update_with_line_function(Self::update_universal_line);
    }

    #[inline(always)]
    fn mandelbrot_iterations_universal(max_iterations: usize, c_real: f64, c_imag: f64) -> usize {//Returns MAX_ITERATIONS if it is bounded
        let diverge_threshold: f64 = 2.0;//TODO make this flexible?

        //z_0 = 0
        let mut z_real: f64 = 0.0;
        let mut z_imag: f64 = 0.0;

        //We exit the loop in two cases: if we reach MAX_ITERATIONS (meaning we assume the c value produces a bounded series)
        //or the modulus of the complex number exceeds the diverge_threshold (meaning the c value produces an unbounded series)
        let mut i: usize = 0;
        while (i < max_iterations) && (((z_real * z_real) + (z_imag * z_imag)) < (diverge_threshold * diverge_threshold)) {
            //z_(n+1) = z_n^2 + c
            let next_z_real = (z_real * z_real) - (z_imag * z_imag) + c_real;
            let next_z_imag = (2.0 * z_real * z_imag) + c_imag;
            z_real = next_z_real;
            z_imag = next_z_imag;
            i += 1;
        }
        //println!("mandelbrot ends returning {}", i);
        return i;
    }

    fn update_universal_line(max_iterations: usize, starting_c_real: f64, real_step_amount: f64, workload: Workload) {
        for (line_slice, c_imag) in workload {
            let mut c_real: f64 = starting_c_real;
            for x in 0..line_slice.len() {
                line_slice[x] = Self::mandelbrot_iterations_universal(max_iterations, c_real, c_imag);
                c_real += real_step_amount;
            }
        }
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
    fn update_universal(b: &mut Bencher) {
        use crate::BaseFractal;
        let mut mandelbrot = Mandelbrot::new(
            1024,
            512,
            512,
            -2.3, 0.8,
            -1.1, 1.1
        );
        mandelbrot.set_max_threads(1);

        b.iter(|| -> Mandelbrot {
            let mut copy = mandelbrot.clone();
            copy.update_universal();
            return copy;
        });
    }

    #[bench]
    fn update_universal_mt(b: &mut Bencher) {
        use crate::BaseFractal;
        let mut mandelbrot = Mandelbrot::new(
            1024,
            512,
            512,
            -2.3, 0.8,
            -1.1, 1.1
        );
        mandelbrot.set_max_threads(std::thread::available_parallelism().expect("Couldn't determine num of host threads").get());//It will just fail otherwise

        b.iter(|| -> Mandelbrot {
            let mut copy = mandelbrot.clone();
            copy.update_universal();
            return copy;
        });
    }
}
