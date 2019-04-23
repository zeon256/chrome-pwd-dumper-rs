use crate::dumper::{close_chrome, dump_to_file};

mod dumper;
mod models;

fn main() {
    close_chrome();
    dump_to_file();
}