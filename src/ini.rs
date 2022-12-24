/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use std::collections::HashMap;
use std::path::Path;
use std::fs::read;

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
    Number(i128),
    Float(f64),
    Text(String)
}

pub type Section = HashMap<Key, Value>;
pub type Sections = Vec<(SectionName, Section)>;

/* Associated Functions and Methods */

//TODO

/* Functions */

//pub fn map_to_fractal(fractal_ini_map: &Map) -> dyn Fractal

pub fn parse_ini_file(path: &Path) -> Result<Sections, ()> {
    /*let ini_file = File::open(path).ok()?;
    let mut file_size = metadata(path).ok()?.len();
    let mut file_buffer = vec![0; file_size as usize].into_boxed_slice();
    let bytes_read =
    */
    let file_contents_result = read(path);
    let file_contents;
    if let Ok(contents) = file_contents_result {
        file_contents = contents;
    } else {
        return Err(());
    }
    println!("{:?}", file_contents);

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
        let mut new_section_name_buffer = Vec::<u8>::with_capacity(64);
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
        println!("{:?}", new_section_name);

        let new_section = Section::new();

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
            return sections;
        }
        //At this point, global index points to some whitespace at the end of the section line

        //Parse keys and values in the section


        todo!();


        /*while global_index < file_contents.len() {
            if file_contents[global_index] == b'[' {
                global_index += 1;
                break;
            }

            global_index += 1;
        }

        section_name_buffer.truncate(0);
        while global_index < file_contents.len() {
            let character = file_contents[global_index];
            if character == b']' {
                global_index += 1;
                break;
            }

            section_name_buffer.push(character);
            global_index += 1;
        }
        */
        //FIXME need to parse a line at a time
    }

    todo!();
}
