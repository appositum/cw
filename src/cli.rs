use clap::{arg, Command};

pub fn cmd() -> Command {
    Command::new("cw")
        .version("0.2.1")
        .author("appositum <appositum@pm.me>")
        .about("Print newline, word, and byte counts for each file")
        .args(&[
            arg!(<file> ... "Read input from the specified files"),
            arg!(-l --lines "Print the line count"),
            arg!(-w --words "Print the word count"),
            arg!(-c --chars "Print the character count"),
            arg!(-b --bytes "Print the byte count"),
            arg!(-L --"max-line-length" "Print the maximum display width"),
        ])
}
