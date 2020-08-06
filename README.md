# chrome-pwd-dumper-rs
A Windows Chromium based password dumper written in rust

<p align="center">
  <img width="500" height="380" src="./logo.png">
</p>

## Supported browsers
- Microsoft Edge
- Google Chrome
- Chromium
- 7star
- Amigo
- Brave
- Cent Browser
- Chedot
- Chrome Canary
- Coccoc Browser
- Comodo Dragon
- Elements Browser
- Epic Privacy Browser
- Kometa
- Orbitum
- Sputnik
- Torch Browser
- uCozMedia Uran
- Vivaldi
- Mail.Ru Atom

## Compatibility
- Tested Microsoft Windows 10 Education 64-bit (Build 17763)
- Tested Microsoft Windows 7 SP1 64-bit (Build 7601)

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
.\chrome-pwd-dumper.exe -b edge chromium -f json -p dump2
[
    Dumper {
        app_info: AppInfo {
            name: "Edge",
            author: "Microsoft",
        },
        accounts: [
            DecryptedAccount {
                website: "https://app.houseparty.com/login",
                username_value: "xXxChromeSlayer420xXx",
                pwd: "xXxChromeSlayer420xXx",
            },
            DecryptedAccount {
                website: "https://accounts.google.com/signin/v2/challenge/password/empty",
                username_value: "xXxChromeSlayer420xXx",
                pwd: "xXxChromeSlayer420xXx",
            },
        ],
    },
    Dumper {
        app_info: AppInfo {
            name: "User Data",
            author: "Chromium",
        },
        accounts: [
            DecryptedAccount {
                website: "https://app.houseparty.com/login",
                username_value: "xXxChromeSlayer420xXx",
                pwd: "xXxChromeSlayer420xXx",
            },
        ],
    }
]
```

## How to build
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
