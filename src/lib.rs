#[derive(Debug)]
pub struct File {
    pub name: String,
    pub newlines: usize,
    pub words: usize,
    pub bytes: usize,
}

pub mod count {
    use crate::File;
    use regex::Regex;
    use std::fs;
    use std::path::Path;

    pub fn file_newlines(s: &String) -> usize {
        s.chars().filter(|&c| c == '\n').collect::<Vec<_>>().len()
    }

    pub fn file_words(s: &String) -> usize {
        Regex::new(r"[\s\n]+")
            .unwrap()
            .split(&s.trim())
            .collect::<Vec<&str>>()
            .len()
    }

    pub fn file_bytes(s: &String) -> usize {
        s.bytes().collect::<Vec<_>>().len()
    }

    pub fn file(file_name: &String) -> (File, Option<String>) {
        let path = Path::new(file_name);

        if path.is_dir() {
            return (
                File {
                    name: file_name.to_string(),
                    newlines: 0,
                    words: 0,
                    bytes: 0,
                },
                Some(format!("cw: {}: Is a directory", file_name)),
            );
        }

        let mut newlines = 0;
        let mut words = 0;
        let mut bytes = 0;
        let mut in_word = false;

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
            Ok(content) => {
                for b in content.bytes() {
                    bytes += 1;

                    if b == b'\n' {
                        newlines += 1;
                    }

                    if !in_word {
                        if !b.is_ascii_whitespace() {
                            in_word = true;
                        }
                    } else {
                        if b.is_ascii_whitespace() {
                            in_word = false;
                            words += 1;
                        }
                    }
                }

                (
                    File {
                        name: file_name.to_string(),
                        newlines,
                        words,
                        bytes,
                    },
                    None,
                )
            }
        }
    }
}
