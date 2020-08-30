use argh::FromArgs;
use std::ffi::OsString;

#[derive(Debug, FromArgs)]
/// Windows Chromium based password dumper that doesn't require admin rights
pub struct Opt {
    /// select a browser. If left blank, program will try to get all the data available.
    /// Browsers selection: `edge`, `chromium`, `7star`, `amigo`, `brave`, `centbrowser`, `chedot`, `chrome_canary`,
    /// `coccoc`, `dragon`, `elements-browser`, `epic-privacy-browser`, `chrome`, `kometa`, `orbitum`, `sputnik`,
    /// `torch`, `ucozmedia`, `vivaldi`, `atom-mailru`
    #[argh(option, short = 'b')]
    pub browsers: Vec<String>,

    /// available format `json`, `txt`
    #[argh(switch)]
    pub json: bool,

    /// file name of output file
    #[argh(option)]
    pub file_name: OsString,

    /// print to stdout
    #[argh(switch)]
    pub print: bool,
}
