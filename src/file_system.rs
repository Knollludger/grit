use std::env;
use std::fs;

fn read_file(filename){
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    return contents;
}