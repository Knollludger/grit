use std::env;
use std::fs;

// intended functions atm:
//      - read_file -> read in entire file.
//      - write_file -> write an entire file.
//      - read_line -> read in line from a file.
//      - write_line -> read in entire file.
//      - make_temp_dir -> make a temporary directory. Potentially copy in files.



fn read_file(filename){
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    return contents;
}