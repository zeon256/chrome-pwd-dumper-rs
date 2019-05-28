# chrome-pwd-dumper-rs
A chrome password dumper written in Rust

## Compatibility
- Microsoft Windows only!

## Motivation
I previously wrote a version in Kotlin and there are quite a number of problems with it.
- Host target requires JRE installed, if no JRE is installed it cannot run
- Java has a noticeably slower startup compared to this rust version. Assuming we are trying to steal password and it took some time for the program to load, the user will probably notice it. Rust version is **blazing fast**.
- Jar file is humongous, 10mb. Rust version is 10x smaller even with sqlite3.dll linked into it

## How to build
> You might need sqlite3.lib
```
// recommended
cargo build --release

// for more optimised builds (If target is not the same as your cpu it might not work!)
cargo rustc --release -- -C target-cpu=native
```

## How to run
Double click the `exe` after building and it should spit out the passwords in a text file in the same directory

## Disclaimer
I shall not be responsible for any crimes committed by anyone that uses this software. Use it only for good!

## License
chrome-pwd-dumper-rs is licensed under MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

