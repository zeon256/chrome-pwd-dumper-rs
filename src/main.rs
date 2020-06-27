#[macro_use]
extern crate serde;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate structopt;

use crate::args::Opt;
use crate::dumper::{Dumper, DumperError};
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;
use structopt::StructOpt;
use std::path::{PathBuf, Path};

mod args;
mod decryption_core;
mod dumper;
mod models;

#[rustfmt::skip]
lazy_static! {
    static ref BROWSERS: Mutex<HashMap<&'static str, Dumper>> = {
        let mut hm = HashMap::new();
        hm.insert("edge", Dumper::new("Edge", "Microsoft"));
        hm.insert("chromium", Dumper::new("", "Chromium"));
        hm.insert("7star", Dumper::new("7Star", "7Star"));
        hm.insert("amigo", Dumper::new("", "Amigo"));
        hm.insert("brave", Dumper::new("Brave-Browser", "BraveSoftware"));
        hm.insert("centbrowser", Dumper::new("", "CentBrowser"));
        hm.insert("chedot", Dumper::new("", "Chedot"));
        hm.insert("chrome_canary", Dumper::new("Chrome SxS", "Google"));
        hm.insert("coccoc", Dumper::new("Browser", "CocCoc"));
        hm.insert("dragon", Dumper::new("Dragon", "Comodo"));
        hm.insert("elements-browser", Dumper::new("", "Elements Browser"));
        hm.insert("epic-privacy-browser",Dumper::new("", "Epic Privacy Browser"));
        hm.insert("chrome", Dumper::new("Chrome", "Google"));
        hm.insert("kometa", Dumper::new("", "Kometa"));
        hm.insert("orbitum", Dumper::new("", "Orbitum"));
        hm.insert("sputnik", Dumper::new("Sputnik", "Sputnik"));
        hm.insert("torch", Dumper::new("", "Torch"));
        hm.insert("ucozmedia", Dumper::new("Uran", "uCozMedia"));
        hm.insert("vivaldi", Dumper::new("", "Vivaldi"));
        hm.insert("atom-mailru", Dumper::new("Atom", "Mail.Ru"));

        Mutex::new(hm)
    };
}

fn main() -> Result<(), DumperError> {
    let opt: Opt = Opt::from_args();
    fs::remove_dir_all("./.tmp");
    fs::create_dir("./.tmp")?;

    let browsers = &mut *BROWSERS.lock().unwrap();
    // check if all browsers selected
    let data = if opt.browsers[0].eq("all") {
        browsers
            .iter_mut()
            .map(|dumper| {
                let res = dumper.1.dump();
                (dumper, res)
            })
            .filter(|v| v.1.is_ok())
            .map(|v| ((v.0).1).clone())
            .collect::<Vec<_>>()
    } else {
        opt.browsers
            .iter()
            .filter_map(|v| browsers.remove(v.as_str()))
            .map(|mut v| {
                let res = v.dump();
                (v, res)
            })
            .filter(|v| v.1.is_ok())
            .map(|v| v.0)
            .collect::<Vec<_>>()
    };

    if opt.print {
        println!("{:#?}", data);
    }

    let (buf, file_name) = if opt.format.eq("json") {
        (serde_json::to_string_pretty(data.as_slice())
            .map_err(|e| DumperError::JsonError(e))?, opt.file_name.as_path())
    } else {
        (format!("{:#?}", data), Path::new("dump.txt"))
    };

    fs::write(file_name, buf.as_bytes()).map_err(|_| DumperError::IoError);
    fs::remove_dir_all("./.tmp");
    Ok(())
}
