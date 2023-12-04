use std::env;
use std::fs::read_to_string;

fn get_fname() -> String {
    env::args().nth(1).unwrap()
}

pub fn read_file() -> String {
    read_to_string(get_fname()).unwrap()
}
