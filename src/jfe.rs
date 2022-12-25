

fn main() {
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

    let value = jfe::ini::parse_ini_file(std::path::Path::new("test.ini"));

    println!("{:?}", value);

    let fractal_box = jfe::ini::section_to_fractal(&(value.unwrap()[0].1));

    println!("{:?}", fractal_box);
}
