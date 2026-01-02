# chrome-pwd-dumper-rs
A Windows Chromium based password dumper written in Rust

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
Usage: chrome-pwd-dumper.exe [-b <browsers...>] [--json] --file-name <file-name> [--print]

Windows Chromium based password dumper that doesn't require admin rights

Options:
  -b, --browsers    select a browser. If left blank, program will try to get all
                    the data available. Browsers selection: `edge`, `chromium`,
                    `7star`, `amigo`, `brave`, `centbrowser`, `chedot`,
                    `chrome_canary`, `coccoc`, `dragon`, `elements-browser`,
                    `epic-privacy-browser`, `chrome`, `kometa`, `orbitum`,
                    `sputnik`, `torch`, `ucozmedia`, `vivaldi`, `atom-mailru`
  --json            available format `json`, `txt`
  --file-name       file name of output file
  --print           print to stdout
  --help            display usage information
```

## Example Usage
```
chrome-pwd-dumper.exe -b edge --json --file-name dump2.txt --print
[
    Dumper {
        app_info: AppInfo {
            name: "Edge",
            author: "Microsoft",
        },
        accounts: [
            DecryptedAccount {
                website: "https://www.mcdelivery.com.sg/",
                username_value: "chromedumper@gmail.com",
                pwd: "xXxChromePwdDumperxXx",
            },
            DecryptedAccount {
                website: "https://www.singpass.gov.sg/",
                username_value: "",
                pwd: "xXxChromePwdDumperxXx",
            },
            DecryptedAccount {
                website: "https://careers.tiktok.com/",
                username_value: "chromedumper@gmail.com",
                pwd: "xXxChromePwdDumperxXx",
            },
        ],
    },
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
