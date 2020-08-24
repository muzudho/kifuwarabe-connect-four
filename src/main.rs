mod log;
mod look_and_model;
mod position;

use crate::log::LogExt;
use casual_logger::{Level, Log, Table};

fn main() {
    Log::set_level(Level::Debug);
    println!("Hello, world!");
}

fn file_to_num(file: char) -> u8 {
    match file {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        _ => panic!(Log::print_fatal_t(
            "(Err.32) Invalid file.",
            Table::default().char("file", file)
        )),
    }
}
