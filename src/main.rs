use cw::{count, File};
use std::env::args;

fn main() {
    let args: Vec<String> = args().skip(1).collect();

    if args.len() == 0 {
        // TODO:
        // - arg parse with `clap`
        // - read from stdin when file is not specified (ctrl + d should stop stdin read)

        println!("cw: Input file not specified");
        std::process::exit(1);
    } else if args.len() == 1 {
        let (file, result) = count::file(&args[0]);

        if let Some(e) = result {
            eprintln!("{}", e);
        }

        let max_width = if file.bytes > 0 {
            file.bytes.ilog10() + 1
        } else {
            1
        };

        // `x.ilog10() + 1` == number of digits in `x`
        println!(
            "{:>w$} {:>w$} {:>w$} {:>w$}",
            file.newlines,
            file.words,
            file.bytes,
            file.name,
            w = max_width as usize
        );
        return;
    }

    let mut total_newlines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;

    let files: Vec<(File, Option<String>)> = args
        .iter()
        .map(|f| {
            let (file, result) = count::file(&f);

            total_newlines += file.newlines;
            total_words += file.words;
            total_bytes += file.bytes;

            (file, result)
        })
        .collect();

    let max_width = if total_bytes > 0 {
        total_bytes.ilog10() + 1
    } else {
        1
    };

    for f in files {
        let (file, result) = f;

        if let Some(e) = result {
            eprintln!("{}", e);
        }

        println!(
            "{:>w$} {:>w$} {:>w$} {}",
            file.newlines,
            file.words,
            file.bytes,
            file.name,
            w = max_width as usize
        );
    }

    println!(
        "{:>w$} {:>w$} {:>w$} total",
        total_newlines,
        total_words,
        total_bytes,
        w = max_width as usize
    );
}
