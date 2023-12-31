pub mod cli;

use clap::ArgAction;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub newlines: usize,
    pub words: usize,
    pub chars: usize,
    pub bytes: usize,
    pub max_line_length: usize,
}

pub enum Error {
    IsDirectory(String),
    FileNotFound(String),
}

pub fn parse_content(file_name: String, content: String) -> File {
    let mut newlines = 0;
    let mut words = 0;
    let mut chars = 0;
    let mut bytes = 0;

    let mut in_word = false;

    let mut max_line_length = 0;
    let mut max_line_length_tmp = 0;

    for c in content.chars() {
        max_line_length = if max_line_length_tmp > max_line_length {
            max_line_length_tmp
        } else {
            max_line_length
        };

        chars += 1;
        bytes += c.len_utf8();

        if c == '\n' {
            newlines += 1;
            max_line_length_tmp = 0;
        } else {
            max_line_length_tmp += 1;
        }

        if !in_word {
            if !c.is_whitespace() {
                in_word = true;
            }
        } else {
            if c.is_whitespace() {
                in_word = false;
                words += 1;
            }
        }
    }

    if in_word {
        words += 1;
    };

    File {
        name: file_name.to_string(),
        newlines,
        words,
        chars,
        bytes,
        max_line_length,
    }
}

pub fn parse_file(file_name: &String) -> (File, Option<Error>) {
    let path = Path::new(file_name);

    if path.is_dir() {
        return (
            File {
                name: file_name.to_string(),
                newlines: 0,
                words: 0,
                chars: 0,
                bytes: 0,
                max_line_length: 0,
            },
            Some(Error::IsDirectory(format!(
                "cw: {}: Is a directory",
                file_name
            ))),
        );
    }

    match fs::read_to_string(file_name) {
        Err(_) => (
            File {
                name: file_name.to_string(),
                newlines: 0,
                words: 0,
                chars: 0,
                bytes: 0,
                max_line_length: 0,
            },
            Some(Error::FileNotFound(format!(
                "cw: {}: No such file or directory",
                file_name
            ))),
        ),
        Ok(content) => {
            (parse_content(file_name.to_string(), content), None)
        }
    }
}

pub fn print(file: File, width: usize) {
    let matches = cli::cmd().get_matches();

    // grab all arguments that don't take values,
    // then filter only the ones that were used in the command
    // simpler way to do this????
    let no_flags: bool = cli::cmd()
        .get_arguments()
        .filter(|arg| {
            if let ArgAction::SetTrue = arg.get_action() {
                true
            } else {
                false
            }
        })
        .all(|arg| !matches.get_flag(arg.get_id().as_str()));

    if no_flags {
        println!(
            "{:>w$} {:>w$} {:>w$} {}",
            file.newlines,
            file.words,
            file.bytes,
            file.name,
            w = width as usize
        );
    } else {
        if matches.get_flag("lines") {
            print!("{:>w$} ", file.newlines, w = width as usize);
        }

        if matches.get_flag("words") {
            print!("{:>w$} ", file.words, w = width as usize);
        }

        if matches.get_flag("chars") {
            print!("{:>w$} ", file.chars, w = width as usize);
        }

        if matches.get_flag("bytes") {
            print!("{:>w$} ", file.bytes, w = width as usize);
        }

        if matches.get_flag("max-line-length") {
            print!("{:>w$} ", file.max_line_length, w = width as usize);
        }

        println!("{}", file.name);
    }

}
