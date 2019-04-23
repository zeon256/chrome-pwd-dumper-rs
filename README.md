# chrome-pwd-dumper-rs
A chrome password dumper written in Rust

## Motivation
I previously wrote a version in Kotlin and there quite a number of problems with it.
- Host target requires JRE installed, if no JRE is installed it cannot run
- Java has a noticibly slower startup compared to this rust version. Assuming we are trying to steal password and it took some time for the program to load, the user will probably notice it
- Jar file is humongous, 10mb. Rust version is 10x smaller even with sqlite3.dll linked into it
