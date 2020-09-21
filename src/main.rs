extern crate turtle;

use std::fs::File;
// use std::io::{Write, Error};

// mod turtle::ast;



fn main() {
    use std::io::*;
    let mut source = String::new();
    match std::env::args().nth(1) {
        Some(filename) => {
            File::open(&filename)
                .expect(&format!("Can't open {}", &filename))
                .read_to_string(&mut source)
                .expect(&format!("Can't read {}", &filename));
        }

        None => {
            stdin()
                .read_to_string(&mut source)
                .expect("Can't read stdin");
        }
    }

    if source.is_empty() {
        println!("No input");
        return;
    }
    turtle::parse_turtle_commands(source);
}
