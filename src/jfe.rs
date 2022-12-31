/* jfe.rs
 * By: John Jekel
 * Copyright (C) 2022-2023 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * JFE Command-line Interfact
 *
*/

use jfe::FractalBox::EscapeTime;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Expected 1 argument (path to ini file)");
        return;
    }

    if let Ok(sections) = jfe::ini::parse_ini_file(std::path::Path::new(&args[1])) {
        for (section_name, section) in sections {
            println!("Processing the {} section", section_name);
            if let Ok(fractal_box) = jfe::ini::section_to_fractal(&section) {
                match fractal_box {
                    EscapeTime(mut fractal) => {
                        fractal.set_max_threads(std::thread::available_parallelism().unwrap().get());
                        fractal.update();
                        //TODO what if it is not 255 max iterations?

                        let mut tga_image_data = Vec::<u8>::with_capacity(fractal.get_x_samples() * fractal.get_y_samples() * 3);

                        for y in 0..fractal.get_y_samples() {
                            for x in 0..fractal.get_x_samples() {
                                let value = fractal.samples_ref().unwrap()[x + (y * fractal.get_x_samples())];
                                tga_image_data.push(value as u8);
                                tga_image_data.push(value as u8);
                                tga_image_data.push(value as u8);
                            }
                        }

                        let tga_file_vec = create_tga_vec(fractal.get_x_samples() as u16, fractal.get_y_samples() as u16, 24, &tga_image_data);
                        std::fs::write("/tmp/".to_string() + &section_name + ".tga", &tga_file_vec).unwrap();
                    }
                    _ => { todo!(); }
                }
            } else {
                println!("Failed to parse the {} section", section_name);
            }
        }
    } else {
        println!("Failed to parse ini");
    }

    /*eprintln!("Creating new Mandelbrot");
    let mandelbrot = Mandelbrot::<255>::new(75, 30, -2.3, -1.1, 0.8, 1.1);

    for y in 0..30 {
        for x in 0..75 {
            if mandelbrot[(x, y)] == 255 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    */

    /*
    let value = jfe::ini::parse_ini_file(std::path::Path::new("test.ini"));

    println!("{:?}", value);

    let mut fractal_box = jfe::ini::section_to_fractal(&(value.unwrap()[0].1)).unwrap();

    println!("{:?}", fractal_box);

    fractal_box.update();

    if let EscapeTime(escape_time_fractal) = fractal_box {
         for y in 0..escape_time_fractal.get_y_samples() {
            for x in 0..escape_time_fractal.get_x_samples() {
                if escape_time_fractal.samples_ref().unwrap()[x + (y * escape_time_fractal.get_x_samples())] == escape_time_fractal.get_max_iterations() {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
            println!();
        }

        let mut tga_image_data = Vec::<u8>::with_capacity(escape_time_fractal.get_x_samples() * escape_time_fractal.get_y_samples() * 3);
        for y in 0..escape_time_fractal.get_y_samples() {
            for x in 0..escape_time_fractal.get_x_samples() {
                let value = escape_time_fractal.samples_ref().unwrap()[x + (y * escape_time_fractal.get_x_samples())];
                tga_image_data.push(value as u8);
                tga_image_data.push(value as u8);
                tga_image_data.push(value as u8);
            }
        }

        let tga_file_vec = create_tga_vec(escape_time_fractal.get_x_samples() as u16, escape_time_fractal.get_y_samples() as u16, 24, &tga_image_data);
        std::fs::write("test_image.tga", &tga_file_vec);
    }
    */
}

fn create_tga_vec(x_pixels: u16, y_pixels: u16, bpp: u8, image_data: &[u8]) -> Vec::<u8> {
    let mut new_vec = Vec::<u8>::with_capacity(image_data.len() + 18);
    new_vec.push(0u8);//ID Length (unused)
    new_vec.push(0u8);//Colour map type (no colour map)
    new_vec.push(2u8);//Image type (uncompressed true-colour)

    //Colour map (5 bytes, unused)
    new_vec.push(0u8);//First entry index low byte (unused)
    new_vec.push(0u8);//First entry index high byte (unused)
    new_vec.push(0u8);//Colour map length low byte (unused)
    new_vec.push(0u8);//Colour map length high byte (unused)
    new_vec.push(0u8);//Colour map entry size (unused)

    //Image specification (10 bytes)
    new_vec.push(0u8);//X origin low byte (coordinates of lower left corner of image)
    new_vec.push(0u8);//X origin high byte (coordinates of lower left corner of image)
    new_vec.push(0u8);//Y origin low byte (coordinates of lower left corner of image)
    new_vec.push(0u8);//Y origin high byte (coordinates of lower left corner of image)
    new_vec.push((x_pixels & 0xFF) as u8);       //Image width low byte
    new_vec.push(((x_pixels >> 8) & 0xFF) as u8);//Image width high byte
    new_vec.push((y_pixels & 0xFF) as u8);       //Image height low byte
    new_vec.push(((y_pixels >> 8) & 0xFF) as u8);//Image height high byte
    new_vec.push(bpp);//Pixel depth
    new_vec.push(0u8);//Image descriptor

    new_vec.extend_from_slice(image_data);
    return new_vec;
}
