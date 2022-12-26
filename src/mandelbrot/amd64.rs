/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use super::Mandelbrot;

use core::arch::x86_64;

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

    #[cfg(target_arch = "x86_64")]
    #[inline]//But this is okay
    #[target_feature(enable = "avx2")]
    unsafe fn mandelbrot_iterations_avx2(self: &Self, c_real_f: x86_64::__m256d, c_imag_f: x86_64::__m256d) -> x86_64::__m256i {
        let diverge_threshold: f64 = 2.0;//TODO make this flexible?
        let diverge_threshold_squared_f = x86_64::_mm256_set1_pd(diverge_threshold * diverge_threshold);
        let two_f = x86_64::_mm256_set1_pd(2.0);

        let mut result_i = x86_64::_mm256_set1_epi64x(0);

        let mut incrementor_i_as_f = x86_64::_mm256_castsi256_pd(x86_64::_mm256_set1_epi64x(1));//We increment the result counter until we go past the diverge_threshold, then we never do again

        let mut z_real_f = x86_64::_mm256_set1_pd(0.0);
        let mut z_imag_f = x86_64::_mm256_set1_pd(0.0);

        for _ in 0..self.max_iterations {
            //Calculate some values that are used below
            let z_real_squared_f = x86_64::_mm256_mul_pd(z_real_f, z_real_f);
            let z_imag_squared_f = x86_64::_mm256_mul_pd(z_imag_f, z_imag_f);

            //Check if the modulus of each z < the diverge value (aka that they haven't diverged)
            //We do this faster by doing (z_real * z_real) + (z_imag * z_imag) < (2 * 2)
            let squared_sum_f = x86_64::_mm256_add_pd(z_real_squared_f, z_imag_squared_f);
            let compare_i_as_f = x86_64::_mm256_cmp_pd(squared_sum_f, diverge_threshold_squared_f, x86_64::_CMP_LT_OQ);

            //Get next entries (For each complex number z, z_(n+1) = z_n^2 + c)
            //We do this before the diverge check below, instead of after like we used to because it better masks the latency of
            //A) The comparisons performed above to the check below
            //B) The calculation of the next z_imag and z_real to the squaring at the start of the next iteration of the loop
            //Also this dosn't compromise the latency of the squaring at the start of the loop to this since we still have the compare above in-between
            let temp_z_real_f = x86_64::_mm256_add_pd(x86_64::_mm256_sub_pd(z_real_squared_f, z_imag_squared_f), c_real_f);
            z_imag_f = x86_64::_mm256_add_pd(c_imag_f, x86_64::_mm256_mul_pd(two_f, x86_64::_mm256_mul_pd(z_real_f, z_imag_f)));
            z_real_f = temp_z_real_f;

            //If both complex numbers have diverged (entire vector is 0), return
            if x86_64::_mm256_movemask_pd(compare_i_as_f) == 0 {
                break;
            }

            //Increment the corresponding count only if we haven't converged yet
            incrementor_i_as_f = x86_64::_mm256_and_pd(compare_i_as_f, incrementor_i_as_f);
            let incrementor_i = x86_64::_mm256_castpd_si256(incrementor_i_as_f);//If a number hasn't converged, we will increment it's count
            result_i = x86_64::_mm256_add_epi64(result_i, incrementor_i);//Requires AVX2
        }

        return result_i;
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn update_avx2(self: &mut Self) {
        debug_assert!((self.x_samples & 0b11) == 0);//TODO overcome this limitation

        //TODO actually use avx here

        let real_length: f64 = self.max_real - self.min_real;
        let real_step_amount: f64 = real_length / (self.x_samples as f64);
        let imag_length: f64 = self.max_imag - self.min_imag;
        let imag_step_amount: f64 = imag_length / (self.y_samples as f64);

        let iterations_pointer = self.iterations.as_mut_ptr();

        let real_step_amount_vector = x86_64::_mm256_set1_pd(real_step_amount * 4.0);//x4 since we process four at a time
        let imag_step_amount_vector = x86_64::_mm256_set1_pd(imag_step_amount);

        let mut c_real = x86_64::_mm256_set_pd(self.min_real + (real_step_amount * 3.0), self.min_real + (real_step_amount * 2.0), self.min_real + real_step_amount, self.min_real);
        for x in (0..self.x_samples).step_by(4) {
            let mut c_imag = x86_64::_mm256_set1_pd(self.min_imag);
            for y in 0..self.y_samples {
                let result = self.mandelbrot_iterations_avx2(c_real, c_imag);
                let pointer = iterations_pointer.offset((x + (y * self.x_samples)) as isize) as *mut x86_64::__m256i;
                x86_64::_mm256_storeu_si256(pointer, result);
                c_imag = x86_64::_mm256_add_pd(c_imag, imag_step_amount_vector);
            }
            c_real = x86_64::_mm256_add_pd(c_real, real_step_amount_vector);
        }

        self.update_pending = false;
    }

    #[cfg(target_arch = "x86_64")]
    #[inline]//But this is okay
    #[target_feature(enable = "avx")]
    unsafe fn mandelbrot_iterations_avx(self: &Self, c_real_f: x86_64::__m256d, c_imag_f: x86_64::__m256d) -> x86_64::__m256i {
        let diverge_threshold: f64 = 2.0;//TODO make this flexible?
        let diverge_threshold_squared_f = x86_64::_mm256_set1_pd(diverge_threshold * diverge_threshold);
        let two_f = x86_64::_mm256_set1_pd(2.0);

        let mut result_i = x86_64::_mm256_set1_epi64x(0);

        let mut incrementor_i_as_f = x86_64::_mm256_castsi256_pd(x86_64::_mm256_set1_epi64x(1));//We increment the result counter until we go past the diverge_threshold, then we never do again

        let mut z_real_f = x86_64::_mm256_set1_pd(0.0);
        let mut z_imag_f = x86_64::_mm256_set1_pd(0.0);

        for _ in 0..self.max_iterations {
            //Calculate some values that are used below
            let z_real_squared_f = x86_64::_mm256_mul_pd(z_real_f, z_real_f);
            let z_imag_squared_f = x86_64::_mm256_mul_pd(z_imag_f, z_imag_f);

            //Check if the modulus of each z < the diverge value (aka that they haven't diverged)
            //We do this faster by doing (z_real * z_real) + (z_imag * z_imag) < (2 * 2)
            let squared_sum_f = x86_64::_mm256_add_pd(z_real_squared_f, z_imag_squared_f);
            let compare_i_as_f = x86_64::_mm256_cmp_pd(squared_sum_f, diverge_threshold_squared_f, x86_64::_CMP_LT_OQ);

            //Get next entries (For each complex number z, z_(n+1) = z_n^2 + c)
            //We do this before the diverge check below, instead of after like we used to because it better masks the latency of
            //A) The comparisons performed above to the check below
            //B) The calculation of the next z_imag and z_real to the squaring at the start of the next iteration of the loop
            //Also this dosn't compromise the latency of the squaring at the start of the loop to this since we still have the compare above in-between
            let temp_z_real_f = x86_64::_mm256_add_pd(x86_64::_mm256_sub_pd(z_real_squared_f, z_imag_squared_f), c_real_f);
            z_imag_f = x86_64::_mm256_add_pd(c_imag_f, x86_64::_mm256_mul_pd(two_f, x86_64::_mm256_mul_pd(z_real_f, z_imag_f)));
            z_real_f = temp_z_real_f;

            //If both complex numbers have diverged (entire vector is 0), return
            if x86_64::_mm256_movemask_pd(compare_i_as_f) == 0 {
                break;
            }

            //Increment the corresponding count only if we haven't converged yet
            incrementor_i_as_f = x86_64::_mm256_and_pd(compare_i_as_f, incrementor_i_as_f);
            let incrementor_i = x86_64::_mm256_castpd_si256(incrementor_i_as_f);//If a number hasn't converged, we will increment it's count
            //Since we don't have AVX2, we must add the top and bottom separately
            let lower_result_i = x86_64::_mm256_castsi256_si128(result_i);
            let upper_result_i = x86_64::_mm256_extractf128_si256(result_i, 1);
            let lower_incrementor_i = x86_64::_mm256_castsi256_si128(incrementor_i);
            let upper_incrementor_i = x86_64::_mm256_extractf128_si256(incrementor_i, 1);
            let lower_sum_i = x86_64::_mm_add_epi64(lower_result_i, lower_incrementor_i);
            let upper_sum_i = x86_64::_mm_add_epi64(upper_result_i, upper_incrementor_i);
            result_i = x86_64::_mm256_castsi128_si256(lower_sum_i);
            result_i = x86_64::_mm256_insertf128_si256(result_i, upper_sum_i, 1);
        }

        return result_i;
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx")]
    unsafe fn update_avx(self: &mut Self) {
        debug_assert!((self.x_samples & 0b11) == 0);//TODO overcome this limitation

        let real_length: f64 = self.max_real - self.min_real;
        let real_step_amount: f64 = real_length / (self.x_samples as f64);
        let imag_length: f64 = self.max_imag - self.min_imag;
        let imag_step_amount: f64 = imag_length / (self.y_samples as f64);

        let iterations_pointer = self.iterations.as_mut_ptr();

        let real_step_amount_vector = x86_64::_mm256_set1_pd(real_step_amount * 4.0);//x4 since we process four at a time
        let imag_step_amount_vector = x86_64::_mm256_set1_pd(imag_step_amount);

        let mut c_real = x86_64::_mm256_set_pd(self.min_real + (real_step_amount * 3.0), self.min_real + (real_step_amount * 2.0), self.min_real + real_step_amount, self.min_real);
        for x in (0..self.x_samples).step_by(4) {
            let mut c_imag = x86_64::_mm256_set1_pd(self.min_imag);
            for y in 0..self.y_samples {
                let result = self.mandelbrot_iterations_avx(c_real, c_imag);
                let pointer = iterations_pointer.offset((x + (y * self.x_samples)) as isize) as *mut x86_64::__m256i;
                x86_64::_mm256_storeu_si256(pointer, result);
                c_imag = x86_64::_mm256_add_pd(c_imag, imag_step_amount_vector);
            }
            c_real = x86_64::_mm256_add_pd(c_real, real_step_amount_vector);
        }

        self.update_pending = false;
    }

    //#[inline(always)]//Can't do this with the second "#[target_feature(enable = "sse2")]"
    #[cfg(target_arch = "x86_64")]
    #[inline]//But this is okay
    #[target_feature(enable = "sse2")]
    unsafe fn old_mandelbrot_iterations_sse2(self: &Self, c_real_f: x86_64::__m128d, c_imag_f: x86_64::__m128d) -> x86_64::__m128i {//Returns MAX_ITERATIONS if it is bounded
        let diverge_threshold: f64 = 2.0;//TODO make this flexible?

        let diverge_threshold_squared_f = x86_64::_mm_set_pd1(diverge_threshold * diverge_threshold);
        let two_f = x86_64::_mm_set_pd1(2.0);

        let mut result_i = x86_64::_mm_set1_epi64x(0);

        let mut incrementor_i = x86_64::_mm_set1_epi64x(1);//We increment the result counter until we go past the diverge_threshold, then we never do again

        let mut z_real_f = x86_64::_mm_set_pd1(0.0);
        let mut z_imag_f = x86_64::_mm_set_pd1(0.0);
        for _ in 0..self.max_iterations {
            //Calculate some values that are used below
            let z_real_squared_f = x86_64::_mm_mul_pd(z_real_f, z_real_f);
            let z_imag_squared_f = x86_64::_mm_mul_pd(z_imag_f, z_imag_f);

            //Check if the modulus of each z < the diverge value (aka that they haven't diverged)
            //We do this faster by doing (z_real * z_real) + (z_imag * z_imag) < (2 * 2)
            let squared_sum_f = x86_64::_mm_add_pd(z_real_squared_f, z_imag_squared_f);
            let compare_i_as_f = x86_64::_mm_cmplt_pd(squared_sum_f, diverge_threshold_squared_f);
            let compare_i = x86_64::_mm_castpd_si128(compare_i_as_f);

            //Get next entries (For each complex number z, z_(n+1) = z_n^2 + c)
            //We do this before the diverge check below, instead of after like we used to because it better masks the latency of
            //A) The comparisons performed above to the check below
            //B) The calculation of the next z_imag and z_real to the squaring at the start of the next iteration of the loop
            //Also this dosn't compromise the latency of the squaring at the start of the loop to this since we still have the compare above in-between
            let temp_z_real_f = x86_64::_mm_add_pd(x86_64::_mm_sub_pd(z_real_squared_f, z_imag_squared_f), c_real_f);
            z_imag_f = x86_64::_mm_add_pd(c_imag_f, x86_64::_mm_mul_pd(two_f, x86_64::_mm_mul_pd(z_real_f, z_imag_f)));
            z_real_f = temp_z_real_f;

            //If both complex numbers have diverged (entire vector is 0), return
            if x86_64::_mm_movemask_epi8(compare_i) == 0 {
                break;
            }

            //Increment the corresponding count only if we haven't converged yet
            incrementor_i = x86_64::_mm_and_si128(compare_i, incrementor_i);//If a number diverged, never increment it's iteration count again
            result_i = x86_64::_mm_add_epi64(result_i, incrementor_i);
        }

        return result_i;
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    unsafe fn old_update_sse2(self: &mut Self) {
        debug_assert!((self.x_samples & 0b1) == 0);//TODO overcome this limitation
        //TESTING
        //unsafe { self.mandelbrot_iterations_sse2(x86_64::_mm_set_pd(0.3, 1.4), x86_64::_mm_set_pd(1234.45, 3.141592)); }

        let real_length: f64 = self.max_real - self.min_real;
        let real_step_amount: f64 = real_length / (self.x_samples as f64);
        let imag_length: f64 = self.max_imag - self.min_imag;
        let imag_step_amount: f64 = imag_length / (self.y_samples as f64);

        let iterations_pointer = self.iterations.as_mut_ptr();

        let real_step_amount_vector = x86_64::_mm_set_pd1(real_step_amount * 2.0);//x2 since we process two at a time
        let imag_step_amount_vector = x86_64::_mm_set_pd1(imag_step_amount);

        let mut c_real = x86_64::_mm_set_pd(self.min_real + real_step_amount, self.min_real);
        for x in (0..self.x_samples).step_by(2) {
            let mut c_imag = x86_64::_mm_set1_pd(self.min_imag);
            for y in 0..self.y_samples {
                let result = self.old_mandelbrot_iterations_sse2(c_real, c_imag);
                let pointer = iterations_pointer.offset((x + (y * self.x_samples)) as isize) as *mut x86_64::__m128i;
                x86_64::_mm_storeu_si128(pointer, result);
                c_imag = x86_64::_mm_add_pd(c_imag, imag_step_amount_vector);
            }
            c_real = x86_64::_mm_add_pd(c_real, real_step_amount_vector);
        }

        self.update_pending = false;
    }

    //Go faster by doing two vectors at once, so that we can exploit the pipeline better
    //#[inline(always)]//Can't do this with the second "#[target_feature(enable = "sse2")]"
    #[cfg(target_arch = "x86_64")]
    #[inline]//But this is okay
    #[target_feature(enable = "sse2")]
    unsafe fn mandelbrot_iterations_sse2(self: &Self, c_real_f: [x86_64::__m128d; 2], c_imag_f: [x86_64::__m128d; 2]) -> [x86_64::__m128i; 2] {//Returns MAX_ITERATIONS if it is bounded
        let diverge_threshold: f64 = 2.0;//TODO make this flexible?

        let diverge_threshold_squared_f = x86_64::_mm_set_pd1(diverge_threshold * diverge_threshold);
        let two_f = x86_64::_mm_set_pd1(2.0);

        let mut result_i = [x86_64::_mm_set1_epi64x(0); 2];

        let mut incrementor_i = [x86_64::_mm_set1_epi64x(1); 2];//We increment the result counter until we go past the diverge_threshold, then we never do again

        let mut z_real_f = [x86_64::_mm_set_pd1(0.0); 2];
        let mut z_imag_f = [x86_64::_mm_set_pd1(0.0); 2];
        for _ in 0..self.max_iterations {
            //Calculate some values that are used below
            let z_real_squared_f = [x86_64::_mm_mul_pd(z_real_f[0], z_real_f[0]), x86_64::_mm_mul_pd(z_real_f[1], z_real_f[1])];
            let z_imag_squared_f = [x86_64::_mm_mul_pd(z_imag_f[0], z_imag_f[0]), x86_64::_mm_mul_pd(z_imag_f[1], z_imag_f[1])];

            //Check if the modulus of each z < the diverge value (aka that they haven't diverged)
            //We do this faster by doing (z_real * z_real) + (z_imag * z_imag) < (2 * 2)
            let squared_sum_f = [x86_64::_mm_add_pd(z_real_squared_f[0], z_imag_squared_f[0]), x86_64::_mm_add_pd(z_real_squared_f[1], z_imag_squared_f[1])];
            let compare_i_as_f = [x86_64::_mm_cmplt_pd(squared_sum_f[0], diverge_threshold_squared_f), x86_64::_mm_cmplt_pd(squared_sum_f[1], diverge_threshold_squared_f)];
            let compare_i = [x86_64::_mm_castpd_si128(compare_i_as_f[0]), x86_64::_mm_castpd_si128(compare_i_as_f[1])];

            //Get next entries (For each complex number z, z_(n+1) = z_n^2 + c)
            //We do this before the diverge check below, instead of after like we used to because it better masks the latency of
            //A) The comparisons performed above to the check below
            //B) The calculation of the next z_imag and z_real to the squaring at the start of the next iteration of the loop
            //Also this dosn't compromise the latency of the squaring at the start of the loop to this since we still have the compare above in-between
            let temp_z_real_f = [x86_64::_mm_add_pd(x86_64::_mm_sub_pd(z_real_squared_f[0], z_imag_squared_f[0]), c_real_f[0]), x86_64::_mm_add_pd(x86_64::_mm_sub_pd(z_real_squared_f[1], z_imag_squared_f[1]), c_real_f[1])];
            z_imag_f = [x86_64::_mm_add_pd(c_imag_f[0], x86_64::_mm_mul_pd(two_f, x86_64::_mm_mul_pd(z_real_f[0], z_imag_f[0]))), x86_64::_mm_add_pd(c_imag_f[1], x86_64::_mm_mul_pd(two_f, x86_64::_mm_mul_pd(z_real_f[1], z_imag_f[1])))];
            z_real_f = temp_z_real_f;

            //If both complex numbers have diverged (entire vector is 0), return
            if (x86_64::_mm_movemask_epi8(compare_i[0]) == 0) && (x86_64::_mm_movemask_epi8(compare_i[1]) == 0) {
                break;
            }

            //Increment the corresponding count only if we haven't converged yet
            incrementor_i = [x86_64::_mm_and_si128(compare_i[0], incrementor_i[0]), x86_64::_mm_and_si128(compare_i[1], incrementor_i[1])];//If a number diverged, never increment it's iteration count again
            result_i = [x86_64::_mm_add_epi64(result_i[0], incrementor_i[0]), x86_64::_mm_add_epi64(result_i[1], incrementor_i[1])];
        }

        return result_i;
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    unsafe fn update_sse2(self: &mut Self) {
        debug_assert!((self.x_samples & 0b11) == 0);//TODO overcome this limitation
        //TESTING
        //unsafe { self.mandelbrot_iterations_sse2(x86_64::_mm_set_pd(0.3, 1.4), x86_64::_mm_set_pd(1234.45, 3.141592)); }

        let real_length: f64 = self.max_real - self.min_real;
        let real_step_amount: f64 = real_length / (self.x_samples as f64);
        let imag_length: f64 = self.max_imag - self.min_imag;
        let imag_step_amount: f64 = imag_length / (self.y_samples as f64);

        let iterations_pointer = self.iterations.as_mut_ptr();

        let real_step_amount_vector = x86_64::_mm_set_pd1(real_step_amount * 4.0);//x4 since we process four at a time
        let imag_step_amount_vector = x86_64::_mm_set_pd1(imag_step_amount);

        let mut c_real = [x86_64::_mm_set_pd(self.min_real + (real_step_amount * 3.0), self.min_real + (real_step_amount * 2.0)), x86_64::_mm_set_pd(self.min_real + real_step_amount, self.min_real)];
        for x in (0..self.x_samples).step_by(4) {
            let mut c_imag = [x86_64::_mm_set1_pd(self.min_imag), x86_64::_mm_set1_pd(self.min_imag)];
            for y in 0..self.y_samples {
                let result = self.mandelbrot_iterations_sse2(c_real, c_imag);
                let pointer = iterations_pointer.offset((x + (y * self.x_samples)) as isize) as *mut x86_64::__m128i;
                x86_64::_mm_storeu_si128(pointer, result[1]);
                x86_64::_mm_storeu_si128(pointer.offset(1), result[0]);
                c_imag = [x86_64::_mm_add_pd(c_imag[0], imag_step_amount_vector), x86_64::_mm_add_pd(c_imag[1], imag_step_amount_vector)];
            }
            c_real = [x86_64::_mm_add_pd(c_real[0], real_step_amount_vector), x86_64::_mm_add_pd(c_real[1], real_step_amount_vector)];
        }

        self.update_pending = false;
    }

    #[cfg(target_arch = "x86_64")]
    pub(super) unsafe fn update_x86_64(self: &mut Self) {
        if is_x86_feature_detected!("avx2") {
            self.update_avx2();
        } else if is_x86_feature_detected!("avx") {
            self.update_avx();
        } else {
            self.update_sse2();//On x86_64, we can assume SSE2
        }
    }
}

/* Functions */

//TODO

/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    //Floating point operations are tricky, so these will fail, but it won't really affect how pleasing the image is
    /*
    #[test]
    #[cfg(target_arch = "x86_64")]
    fn test_sse2_against_universal() {
        let mut mandelbrot_universal = Mandelbrot::new(
            1024,
            128,
            128,
            -2.3, 0.8,
            -1.1, 1.1
        );
        mandelbrot_universal.update_universal();

        let mut mandelbrot_sse2 = Mandelbrot::new(
            1024,
            128,
            128,
            -2.3, 0.8,
            -1.1, 1.1
        );
        unsafe { mandelbrot_sse2.update_sse2(); }

        for i in 0..(128 * 128) {
            assert_eq!(mandelbrot_universal.iterations[i], mandelbrot_sse2.iterations[i]);
        }
    }

    #[test]
    #[cfg(target_arch = "x86_64")]
    fn test_avx_against_universal() {
        if !is_x86_feature_detected!("avx") {
            panic!("Can't run this test without AVX support");
        }

        let mut mandelbrot_universal = Mandelbrot::new(
            1024,
            128,
            128,
            -2.3, 0.8,
            -1.1, 1.1
        );
        mandelbrot_universal.update_universal();

        let mut mandelbrot_avx = Mandelbrot::new(
            1024,
            128,
            128,
            -2.3, 0.8,
            -1.1, 1.1
        );
        unsafe { mandelbrot_avx.update_avx(); }

        for i in 0..(128 * 128) {
            assert_eq!(mandelbrot_universal.iterations[i], mandelbrot_avx.iterations[i]);
        }
    }
    */
}
/* Benches */

//TODO
#[cfg_attr(feature = "nightly-features-benches", cfg(test))]
#[cfg(feature = "nightly-features-benches")]
mod benches {
    extern crate test;
    use test::Bencher;
    use super::*;

    #[bench]
    fn create_mandelbrot(b: &mut Bencher) {
        b.iter(|| -> Mandelbrot {
            let mandelbrot = Mandelbrot::new(
                1024,
                128,
                128,
                -2.3, 0.8,
                -1.1, 1.1
            );

            return mandelbrot;
        });
    }

    #[bench]
    fn copy_overhead(b: &mut Bencher) {
        let mandelbrot = Mandelbrot::new(
            1024,
            128,
            128,
            -2.3, 0.8,
            -1.1, 1.1
        );

        b.iter(|| -> Mandelbrot {
            let copy = mandelbrot.clone();
            return copy;
        });
    }

    #[bench]
    #[cfg(target_arch = "x86_64")]
    fn update_sse2(b: &mut Bencher) {
        let mandelbrot = Mandelbrot::new(
            1024,
            128,
            128,
            -2.3, 0.8,
            -1.1, 1.1
        );

        b.iter(|| -> Mandelbrot {
            let mut copy = mandelbrot.clone();
            unsafe { copy.update_sse2() };
            return copy;
        });
    }

    #[bench]
    #[cfg(target_arch = "x86_64")]
    fn update_avx(b: &mut Bencher) {
        if is_x86_feature_detected!("avx") {
            let mandelbrot = Mandelbrot::new(
                1024,
                128,
                128,
                -2.3, 0.8,
                -1.1, 1.1
            );

            b.iter(|| -> Mandelbrot {
                let mut copy = mandelbrot.clone();
                unsafe { copy.update_avx() };
                return copy;
            });
        }
    }

    #[bench]
    #[cfg(target_arch = "x86_64")]
    fn update_avx2(b: &mut Bencher) {
        if is_x86_feature_detected!("avx2") {
            let mandelbrot = Mandelbrot::new(
                1024,
                128,
                128,
                -2.3, 0.8,
                -1.1, 1.1
            );

            b.iter(|| -> Mandelbrot {
                let mut copy = mandelbrot.clone();
                unsafe { copy.update_avx2() };
                return copy;
            });
        }
    }

    #[bench]
    fn update_universal(b: &mut Bencher) {
        let mandelbrot = Mandelbrot::new(
            1024,
            128,
            128,
            -2.3, 0.8,
            -1.1, 1.1
        );

        b.iter(|| -> Mandelbrot {
            let mut copy = mandelbrot.clone();
            copy.update_universal();
            return copy;
        });
    }
}
