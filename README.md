# chrome-pwd-dumper-rs
A Windows Google Chrome Password Dumper written in Rust

<p align="center">
  <img width="460" height="300" src="./logo.png">
</p>

## Compatibility
- Test Microsoft Windows 10!

You can make a PR if it works on older versions of Windows

## Motivation
I previously wrote a version in Kotlin and there are quite a number of problems with it.
- Host target requires JRE installed, if no JRE is installed it cannot run
- Java has a noticeably slower startup compared to this rust version. Assuming we are trying to steal password and it took some time for 
the program to load, the user will probably notice it. Rust version is **blazing fast**.
- Jar file is humongous, 10mb. Binary produced by Rust is 2.7mb.

## How to build
> You might need sqlite3.lib to compile
```
// recommended
cargo build --release

// for more optimised builds (If target is not the same as your cpu it might not work!)
cargo rustc --release -- -C target-cpu=native

// Optimise for binary size
// Current config builds with opt-level = 3 for performance
// Change to opt-level = 'z' in Cargo.toml 

```

## How to run
Double click the `exe` after building and it should spit out the passwords in a text file in the same directory

## How fast is Rust version compared to Kotlin?
🚀 Queries, transform and output to file under 92ms on my machine. (i7 3770k @ 4.20Ghz, 16gb RAM @ 2200Mhz)

## Disclaimer
I shall not be responsible for any crimes committed by anyone that uses this software. Use it only for good!

## License
chrome-pwd-dumper-rs is licensed under MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

