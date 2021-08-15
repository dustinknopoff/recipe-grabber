use std::{fs::File, io::Read};

use nytcooking_grabber::get_ld_json;

fn main() {
    let mut args = std::env::args();
    if let Some(val) = args.nth(1) {
        let mut file = File::open(val).expect("File not found.");
        let mut contents_of_file = String::new();
        file.read_to_string(&mut contents_of_file)
            .expect("could not read from file");
        println!("{}", get_ld_json(&contents_of_file))
    }
}
