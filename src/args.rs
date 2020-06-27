use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "chrome-pwd-dumper",
    about = "Windows Chromium based password dumper that doesn't require admin rights"
)]
pub struct Opt {
    /// Select a browser. If left blank, program will try to get all the data available.
    /// Browsers selection: `edge`, `chromium`, `7star`, `amigo`, `brave`, `centbrowser`, `chedot`, `chrome_canary`,
    /// `coccoc`, `dragon`, `elements-browser`, `epic-privacy-browser`, `chrome`, `kometa`, `orbitum`, `sputnik`,
    /// `torch`, `ucozmedia`, `vivaldi`, `atom-mailru`
    #[structopt(short, default_value = "all")]
    pub browsers: Vec<String>,

    /// Available format `json`, `txt`
    #[structopt(short, long, default_value = "json")]
    pub format: String,

    /// File name of output file
    #[structopt(name = "FILE", parse(from_os_str), default_value = "dump")]
    pub file_name: PathBuf,

    /// Print to stdout
    #[structopt(short, long)]
    pub print: bool,
}
