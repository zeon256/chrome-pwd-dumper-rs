#[macro_use]
extern crate serde;

use crate::args::Opt;
use crate::dumper::{Dumper, DumperError};
use std::collections::HashMap;
use std::fs;
use lazy_static::lazy_static;

mod args;
mod decryption_core;
mod dumper;
mod models;

pub type DumperResult<T> = Result<T, DumperError>;

#[rustfmt::skip]
lazy_static! {
    pub static ref BROWSERS: HashMap<&'static str, Dumper> = {
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

        hm
    };
}

fn main() -> DumperResult<()> {
    let mut opt: Opt = argh::from_env();

    // error can be ignored
    fs::remove_dir_all("./.tmp");
    fs::create_dir("./.tmp")?;

    let browsers = &mut BROWSERS.clone();

    if opt.browsers.is_empty() {
        return Err(DumperError::BrowserNotFound)
    }

    if opt.browsers[0].eq("all") {
        opt.browsers.clear();
        opt.browsers = browsers.keys().map(|v| v.to_string()).collect::<Vec<_>>();
    }

    let data = opt
        .browsers
        .into_iter()
        .filter_map(|v| browsers.get(v.as_str()).cloned())
        .map(|mut v| v.dump().map(|_| v))
        .filter_map(|v| v.ok())
        .collect::<Vec<_>>();

    if opt.print {
        println!("{:#?}", data);
    }

    let mut path = opt.file_name;

    let buf = if opt.json {
        path.push(".json");
        serde_json::to_string_pretty(data.as_slice()).map_err(|e| DumperError::JsonError(e))?
    } else {
        path.push(".txt");
        format!("{:#?}", data)
    };

    fs::write(path, buf.as_bytes())?;
    fs::remove_dir_all("./.tmp")?;
    Ok(())
}
