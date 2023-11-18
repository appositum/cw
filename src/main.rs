use std::env::args;
use std::fs;
use std::path::Path;

use regex::Regex;

#[derive(Debug)]
struct File {
    name: String,
    newlines: usize,
    words: usize,
    bytes: usize,
}

fn main() {
    let args: Vec<String> = args().skip(1).collect();

    if args.len() == 0 {
        // TODO:
        // - arg parse with `clap`
        // - read from stdin when file is not specified (ctrl + d should stop stdin read)

        println!("cw: Input file not specified");
        std::process::exit(1);
    } else if args.len() == 1 {
        let (file, res) = parse(&args[0]);

        if let Some(e) = res {
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
            let (file, result) = parse(&f);

            total_newlines += file.newlines;
            total_words += file.words;
            total_bytes += file.bytes;

            (file, result)
        }).collect();

    let max_width = files.iter().max_by_key(|f| {
        let (file, _res) = f;
        file.bytes
    }).unwrap().0.bytes;

    for f in files {
        let (file, result) = f;

        if let Some(e) = result {
            eprintln!("{}", e);
        }

        println!(
            "{:>w$} {:>w$} {:>w$} {:>w$}",
            file.newlines,
            file.words,
            file.bytes,
            file.name,
            w = (max_width.ilog10() + 1) as usize
        );
    }

    println!(
        "{:>w$} {:>w$} {:>w$} total",
        total_newlines,
        total_words,
        total_bytes,
        w = (max_width.ilog10() + 1) as usize
    );
}

fn count_newlines(s: &String) -> usize {
    s.chars().filter(|&c| c == '\n').collect::<Vec<_>>().len()
}

fn count_words(s: &String) -> usize {
    Regex::new(r"[\s\n]+")
        .unwrap()
        .split(&s.trim())
        .collect::<Vec<&str>>()
        .len()
}

fn count_bytes(s: &String) -> usize {
    s.bytes().collect::<Vec<_>>().len()
}

fn parse(file_name: &String) -> (File, Option<String>) {
    let path = Path::new(file_name);

    if path.is_dir() {
        (
            File {
                name: file_name.to_string(),
                newlines: 0,
                words: 0,
                bytes: 0,
            },
            Some(format!("cw: {}: Is a directory", file_name)),
        )
    } else {
        match fs::read_to_string(file_name) {
            Err(_) => (
                File {
                    name: file_name.to_string(),
                    newlines: 0,
                    words: 0,
                    bytes: 0,
                },
                Some(format!("cw: {}: No such file or directory", file_name)),
            ),
            Ok(f) => (
                File {
                    name: file_name.to_string(),
                    newlines: count_newlines(&f),
                    words: count_words(&f),
                    bytes: count_bytes(&f),
                },
                None,
            ),
        }
    }
}
