use std::env;

pub fn get_fname() -> String {
    env::args().nth(1).unwrap()
}
