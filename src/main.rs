extern crate clap;
extern crate rayon;
extern crate serde;
extern crate serde_json;
use clap::{App, Arg};

use crate::dumper::{close_chrome, dump};

mod dumper;
mod models;

fn main() {
    let matches = App::new("chrome-pwd-dumper-rs")
        .version("0.2.0")
        .author("Budi Syahiddin, <budisyahiddin@gmail.com>")
        .about("Windows Google Chrome Password dumper that doesn't require admin rights")
        .arg(
            Arg::with_name("filename")
                .short("n")
                .long("filename")
                .help("Sets the filename of the text file. Defaults to `dump` if no nothing is provided")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("dump_to_file")
                .short("d")
                .long("dump")
                .help("Do you want to dump to a file")
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .help("Choose your preferred format. Only `json` and `txt` are allowed. If not provided, `txt` is picked")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("print")
                .short("p")
                .long("print")
                .help("Prints dump to stdout. Format is the same as the one provided with `-f`")
                .takes_value(false)
                .required(false),
        )
        .get_matches();

    let filename = matches.value_of("filename").unwrap_or("dump");

    let is_dumping_to_file = matches.is_present("dump_to_file");
    let is_printing_to_stdout = matches.is_present("print");
    let format = matches.value_of("format").unwrap_or("txt").to_uppercase();

    close_chrome();
    dump(filename, format, is_printing_to_stdout, is_dumping_to_file);
}
