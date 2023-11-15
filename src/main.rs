use std::env::args;
use std::fs;
use std::path::Path;

use regex::Regex;

fn main() {
    let files: Vec<String> = args().skip(1).collect();
    print_file_count(&files[0]);

    for file in files {
        // TODO
    }
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

fn print_file_count(file_name: &String) {
    let path = Path::new(file_name);

    if path.is_dir() {
        eprintln!("cw: {}: Is a directory", file_name);
        println!("{x}\t{x}\t{x}", x = 0);
    } else {
        let file: Result<String, _> = fs::read_to_string(file_name);

        match file {
            Err(_) => {
                eprintln!("cw: {}: No such file or directory", file_name);
            }
            Ok(f) => {
                println!(
                    "lines\twords\tbytes\t\n{}\t{}\t{}\t{}",
                    count_newlines(&f),
                    count_words(&f),
                    count_bytes(&f),
                    file_name,
                );
            }
        }
    }
}
