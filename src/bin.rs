use std::{fs::{self}};

use recipe_grabber::get_ld_json;

fn main() {
    let mut args = std::env::args();
    if let Some(val) = args.nth(1) {
        let contents_of_file = fs::
            read_to_string(val)
            .expect("could not read from file");
        println!("{}", get_ld_json(&contents_of_file))
    }
}
