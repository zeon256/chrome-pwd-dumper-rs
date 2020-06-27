# chrome-pwd-dumper-rs
A Windows Google Chrome Password Dumper written in Rust

<p align="center">
  <img width="500" height="380" src="./logo.png">
</p>

## Compatibility
- Tested Microsoft Windows 10 Education 64-bit (Build 17763)

You can make a PR if it works on older versions of Windows

## Flags
```
chrome-pwd-dumper 0.3.0
Windows Chromium based password dumper that doesn't require admin rights

USAGE:
    chrome-pwd-dumper.exe [FLAGS] [OPTIONS] [--] [FILE]

FLAGS:
    -h, --help       Prints help information
    -p, --print      Print to stdout
    -V, --version    Prints version information

OPTIONS:
    -b <browsers>...         Select a browser. If left blank, program will try to get all the data available. Browsers
                             selection: `edge`, `chromium`, `7star`, `amigo`, `brave`, `centbrowser`, `chedot`,
                             `chrome_canary`, `coccoc`, `dragon`, `elements-browser`, `epic-privacy-browser`, `chrome`,
                             `kometa`, `orbitum`, `sputnik`, `torch`, `ucozmedia`, `vivaldi`, `atom-mailru` [default: all]
    -f, --format <format>    Available format `json`, `txt` [default: json]

ARGS:
    <FILE>    File name of output file [default: dump]
```

## Example Usage
```
.\chrome-pwd-dumper.exe -b edge -f json -p dump2
[
    Dumper {
        app_info: AppInfo {
            name: "Edge",
            author: "Microsoft",
        },
        accounts: [
            DecryptedAccount {
                website: "https://app.houseparty.com/login",
                username_value: "xXxJohnCena420xXx",
                pwd: "xXxJohnCena420xXx",
            },
            DecryptedAccount {
                website: "https://accounts.google.com/signin/v2/challenge/password/empty",
                username_value: "xXxJohnCena420xXx",
                pwd: "xXxJohnCena420xXx",
            },
        ],
    },
]
Dumped!
```

## How to build
Since this software can be used maliciously, #justbuildyourselflol
```
// recommended
cargo build --release

// for more optimised builds (If target is not the same as your cpu it might not work!)
cargo rustc --release -- -C target-cpu=native

// Optimise for binary size
// Current config builds with opt-level = 3 for performance
// Change to opt-level = 'z' in Cargo.toml 

```

## License
chrome-pwd-dumper-rs is licensed under MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
