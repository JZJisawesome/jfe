/* lib.rs
 * By: John Jekel
 * Copyright (C) 2022-2023 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * JFE Library
 *
*/

/* Nightly Features */

//Only enabled if the relevant Cargo feature is
#![cfg_attr(feature = "nightly-features-benches", feature(test))]

/* Imports */

//mod numerical_traits;
pub mod ini;

mod simd;

//pub mod fractal_names;
use std::fmt::Debug;

pub mod escape_time;

//use std::ops::Index;

//use crate::numerical_traits::Integer;

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
    EscapeTime(Box<dyn escape_time::EscapeTimeFractal>),
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

pub trait RandomFractal: BaseFractal {
    //TODO
}

pub trait FiniteSubdivisionFractal: BaseFractal {
    //TODO
}

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
