/* ini.rs
 * By: John Jekel
 * Copyright (C) 2022-2023 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * INI Parsing
 *
*/

/* Imports */

use std::collections::HashMap;
use std::path::Path;
use std::fs::read;
use crate::FractalBox;
use crate::escape_time::EscapeTimeFractal;
use crate::escape_time;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub type SectionName = String;

pub type Key = String;

#[derive(Debug)]
pub enum Value {
    Integer(i128),
    Float(f64),
    Text(String)
}

pub type Section = HashMap<Key, Value>;
pub type Sections = Vec<(SectionName, Section)>;

/* Associated Functions and Methods */

//TODO

/* Functions */

pub fn section_to_fractal(fractal_ini_section: &Section) -> Result<FractalBox, ()> {
    //TODO optimize this function

    let type_string;
    let subtype_string;

    if let Some(Value::Text(string)) = fractal_ini_section.get("type") {
        type_string = string.as_str();
    } else {
        return Err(());
    }

    if let Some(Value::Text(string)) = fractal_ini_section.get("subtype") {
        subtype_string = string.as_str();
    } else {
        return Err(());
    }

    match type_string {
        "ifs" => { todo!(); },
        "strange_attractor" => { todo!(); },
        "escape_time" => {
            let mut escape_time_fractal_box: Box<dyn EscapeTimeFractal>;
            match subtype_string {
                "mandelbrot" => {
                    escape_time_fractal_box = Box::new(escape_time::mandelbrot::Mandelbrot::new(
                        1,
                        1,
                        1,
                        0.0, 0.0,
                        0.0, 0.0
                    ));
                },
                "burning_ship" => {
                    escape_time_fractal_box = Box::new(escape_time::burning_ship::BurningShip::new(
                        1,
                        1,
                        1,
                        0.0, 0.0,
                        0.0, 0.0
                    ));
                }
                _ => { return Err(()); },
            }

            if let Some(Value::Integer(int)) = fractal_ini_section.get("max_iterations") {
                //assert!(int < max usize);//TODO proper error handling
                escape_time_fractal_box.set_max_iterations(*int as usize);
            } else {
                return Err(());
            }

            if let Some(Value::Integer(int)) = fractal_ini_section.get("x_samples") {
                //assert!(int < max usize);//TODO proper error handling
                escape_time_fractal_box.set_x_samples(*int as usize);
            } else {
                return Err(());
            }

            if let Some(Value::Integer(int)) = fractal_ini_section.get("y_samples") {
                //assert!(int < max usize);//TODO proper error handling
                escape_time_fractal_box.set_y_samples(*int as usize);
            } else {
                return Err(());
            }

            if let Some(Value::Float(float)) = fractal_ini_section.get("min_x") {
                //TODO return error rather than panic if it is greater than max_x
                escape_time_fractal_box.set_min_x(*float);
            } else {
                return Err(());
            }

            if let Some(Value::Float(float)) = fractal_ini_section.get("max_x") {
                //TODO return error rather than panic if it is less than min_x
                escape_time_fractal_box.set_max_x(*float);
            } else {
                return Err(());
            }

            if let Some(Value::Float(float)) = fractal_ini_section.get("min_y") {
                //TODO return error rather than panic if it is greater than max_y
                escape_time_fractal_box.set_min_y(*float);
            } else {
                return Err(());
            }

            if let Some(Value::Float(float)) = fractal_ini_section.get("max_y") {
                //TODO return error rather than panic if it is less than min_y
                escape_time_fractal_box.set_max_y(*float);
            } else {
                return Err(());
            }

            return Ok(FractalBox::EscapeTime(escape_time_fractal_box));
        },
        "random" => { todo!(); },
        "finite_subdivision" => { todo!(); },
        _ => { return Err(()); },
    }
}

pub fn parse_ini_file(path: &Path) -> Result<Sections, ()> {
    let file_contents_result = read(path);
    let file_contents;
    if let Ok(contents) = file_contents_result {
        file_contents = contents;
    } else {
        return Err(());
    }

    let mut global_index: usize = 0;

    //Skip past all lines before the first section
    loop {
        //Skip past leading whitespace
        loop {
            if global_index == file_contents.len() {
                return Ok(Sections::new());//There were no sections (this isn't an error, it just means we return an empty Sections)
            }

            let character = file_contents[global_index];
            if !character.is_ascii_whitespace() {
                break;
            }

            global_index += 1;
        }
        //At this point, global_index now points to the first non-whitespace character

        let first_non_whitespace_character = file_contents[global_index];
        if first_non_whitespace_character == b';' {//A comment
            //Skip to the end of the line
            loop {
                global_index += 1;
                if global_index == file_contents.len() {
                    return Ok(Sections::new());//There were no sections (this isn't an error, it just means we return an empty Sections)
                }

                let character = file_contents[global_index];
                if (character == b'\n') || (character == b'\r') {//End of the line (support all platforms)
                    break;//Go back to the start of the initial line-skipping loop
                }
            }
        } else if first_non_whitespace_character == b'[' {//Start of a new section
            break;//Go onto the next part; parsing the first section's name
        } else {//Invalid syntax
            return Err(());
        }
    }
    //At this point, global_index is pointing to the [ of a section

    let mut sections = Sections::new();

    //The main loop handles a section, then restarts
    loop {
        //Parse the section's name
        let mut new_section_name_buffer = Vec::<u8>::with_capacity(32);
        global_index += 1;
        while global_index < file_contents.len() {
            let character = file_contents[global_index];
            if (character == b'\n') || (character == b'\r') {//Line ended without the section's name being closed (support all platforms)
                return Err(());
            } else if character == b']' {//End of section's name!
                break;
            }

            new_section_name_buffer.push(character);
            global_index += 1;
        }
        if new_section_name_buffer.len() == 0 {//The section's name was empty (or the file ended after the first [)
            return Err(());
        }
        let new_section_name;
        if let Ok(string_from_vec) = String::from_utf8(new_section_name_buffer) {//Dosn't copy the contents of the vector (fast)
            new_section_name = string_from_vec;
        } else {
            return Err(());
        }
        //At this point, new_section_name contains the section's name and global_index points to the trailing ]

        let mut new_section = Section::new();

        //Ensure there is no garbage after the section name and skip to the next line
        global_index += 1;
        while global_index < file_contents.len() {
            let character = file_contents[global_index];
            if (character == b'\n') || (character == b'\r') {//Line ended (support all platforms)
                break;
            } else if !character.is_ascii_whitespace() {//Garbage after the section name
                return Err(());
            }

            global_index += 1;
        }
        if global_index == file_contents.len() {//File ended immediately after the section's name
            //It is an empty section, but valid
            sections.push((new_section_name, new_section));
            return Ok(sections);
        }
        //At this point, global index points to some whitespace at the end of the section line

        //Parse keys and values in the section
        loop {
            //Skip past leading whitespace
            loop {
                let character = file_contents[global_index];
                if !character.is_ascii_whitespace() {
                    break;
                }

                global_index += 1;
                if global_index == file_contents.len() {//End of the file; push the current section name and current section and return
                    //It is an empty section, but valid
                    sections.push((new_section_name, new_section));
                    return Ok(sections);
                }
            }
            //At this point, global_index now points to the first non-whitespace character

            //Determine what to do based on the character
            let first_non_whitespace_character = file_contents[global_index];
            if first_non_whitespace_character == b';' {//A comment
                //Skip to the end of the line
                loop {
                    global_index += 1;
                    if global_index == file_contents.len() {//End of the file; push the current section name and current section and return
                        //It is an empty section, but valid
                        sections.push((new_section_name, new_section));
                        return Ok(sections);
                    }

                    let character = file_contents[global_index];
                    if (character == b'\n') || (character == b'\r') {//End of the line (support all platforms)
                        break;//Go back to the start of the initial line-skipping loop
                    }
                }
            } else if first_non_whitespace_character == b'[' {//Start of a new section
                //Push the current section and name
                sections.push((new_section_name, new_section));
                break;//Restart the main loop, parsing the next section's name
            } else {//This is a key-value line
                let mut new_key_buffer = Vec::<u8>::with_capacity(32);
                new_key_buffer.push(first_non_whitespace_character);
                global_index += 1;
                while global_index < file_contents.len() {
                    let character = file_contents[global_index];
                    if (character == b'\n') || (character == b'\r') {//Line ended without an "=" or a value (support all platforms)
                        return Err(());
                    } else if character.is_ascii_whitespace() || (character == b'=') {//End of the key!
                        break;
                    }

                    new_key_buffer.push(character);
                    global_index += 1;
                }
                if new_key_buffer.len() == 0 {//The file ended without an "=" or a value after the key
                    return Err(());
                }
                let new_key;
                if let Ok(string_from_vec) = String::from_utf8(new_key_buffer) {//Dosn't copy the contents of the vector (fast)
                    new_key = string_from_vec;
                } else {
                    return Err(());
                }
                //At this point, global_index points to the character after the key

                //Skip past whitespace and ensure there is an equal sign
                loop {
                    let character = file_contents[global_index];
                    if character == b'=' {
                        break;//Found the equals sign!
                    } else if !character.is_ascii_whitespace() || (character == b'\n') || (character == b'\r') {//Never found = before the value or the end of the line
                        return Err(());
                    }

                    global_index += 1;

                    if global_index == file_contents.len() {
                        return Err(());//The file ended without an "=" or a value after the key
                    }
                }
                //At this point, global_index now points to the equals sign

                //Skip past whitespace after the equals sign
                loop {
                    global_index += 1;
                    if global_index == file_contents.len() {
                        return Err(());//The file ended without a value after the key and the =
                    }

                    let character = file_contents[global_index];
                    if (character == b'\n') || (character == b'\r') {//Never found the value before the end of the line
                        return Err(());
                    } else if !character.is_ascii_whitespace() {//Start of the value!
                        break;
                    }
                }
                //At this point, global_index now points to the first character of the value

                //Get the raw value as a string
                let mut new_raw_value_buffer = Vec::<u8>::with_capacity(32);
                while global_index < file_contents.len() {
                    let character = file_contents[global_index];
                    if (character == b'\n') || (character == b'\r') {
                        break;
                    }

                    new_raw_value_buffer.push(character);
                    global_index += 1;
                }
                debug_assert!(new_raw_value_buffer.len() != 0);
                let new_raw_value;
                if let Ok(string_from_vec) = String::from_utf8(new_raw_value_buffer) {//Dosn't copy the contents of the vector (fast)
                    new_raw_value = string_from_vec;
                } else {
                    return Err(());
                }
                //At this point, global_index now points to whitespace at the end of the line, or it is at the end of the file

                //Try to parse the raw value string as an integer, then as a float, and then as a string
                let new_value;
                if let Ok(parsed_int) = new_raw_value.parse::<i128>() {
                    new_value = Value::Integer(parsed_int);
                } else if let Ok(parsed_float) = new_raw_value.parse::<f64>() {
                    new_value = Value::Float(parsed_float);
                } else {
                    new_value = Value::Text(new_raw_value);
                }

                //Add the new key-value pair to the section
                new_section.insert(new_key, new_value);

                if global_index == file_contents.len() {//The entire file, not just the line, ended
                    //Push the current section and name, and return it since there's nothing left
                    sections.push((new_section_name, new_section));
                    return Ok(sections);
                }//Else keep looking for key-value pairs
            }
        }
    }
}
