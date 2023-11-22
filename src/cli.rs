use clap::{arg, ArgMatches, Command};

pub fn args() -> ArgMatches {
    Command::new("cw")
        .version("0.2.0")
        .author("appositum <appositum@pm.me>")
        .about("Print newline, word, and byte counts for each file")
        .args(&[
            arg!(<file> ... "Read input from the specified files"),
            /* TODO: implement these without `conflicts_with_all`.
            The columns should always be printed in this order:
            `lines | words | chars | bytes | max-line-length`
            If no arguments are provided, the default is:
            `lines | words | bytes`

            `cw --max-line-length --chars file.txt` would display:
            `chars | max-line-length`

            `cw --max-line-length --lines --bytes file.txt` would display:
            `lines | bytes | max-line-length` */
            arg!(-l --lines "Print the line count").conflicts_with_all([
                "words",
                "chars",
                "bytes",
                "max-line-length",
            ]),
            arg!(-w --words "Print the word count").conflicts_with_all([
                "lines",
                "chars",
                "bytes",
                "max-line-length",
            ]),
            arg!(-c --chars "Print the character count").conflicts_with_all([
                "lines",
                "words",
                "bytes",
                "max-line-length",
            ]),
            arg!(-b --bytes "Print the byte count").conflicts_with_all([
                "lines",
                "words",
                "chars",
                "max-line-length",
            ]),
            arg!(-L --"max-line-length" "Print the maximum display width")
                .conflicts_with_all(["lines", "words", "chars", "bytes"]),
        ])
        .get_matches()
}
