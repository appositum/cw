use cw::{self, cli, File};

fn main() {
    let matches = cli::args();

    let mut input_files = matches.get_many::<String>("file").unwrap();

    if input_files.len() == 1 {
        let (file, result) = cw::count(&input_files.next().unwrap());

        if let Some(e) = result {
            eprintln!("{}", e);
        }

        let max_width = if file.bytes > 0 {
            // `x.ilog10() + 1` == number of digits in `x`
            file.bytes.ilog10() + 1
        } else {
            1
        };

        if matches.get_flag("lines") {
            println!("{:?}", matches.get_flag("lines"));
            println!(
                "{:>w$} {}",
                file.newlines,
                file.name,
                w = max_width as usize
            );
        } else if matches.get_flag("words") {
            println!("{:>w$} {}", file.words, file.name, w = max_width as usize);
        } else if matches.get_flag("chars") {
            println!("{:>w$} {}", file.chars, file.name, w = max_width as usize);
        } else if matches.get_flag("bytes") {
            println!("{:>w$} {}", file.bytes, file.name, w = max_width as usize);
        } else if matches.get_flag("max-line-length") {
            println!("{:>w$} {}", file.max_line_length, file.name, w = max_width as usize);
        } else {
            println!(
                "{:>w$} {:>w$} {:>w$} {}",
                file.newlines,
                file.words,
                file.bytes,
                file.name,
                w = max_width as usize
            );
        }

        return;
    }

    let mut total_newlines = 0;
    let mut total_words = 0;
    let mut total_chars = 0;
    let mut total_bytes = 0;
    let mut max_line_length = 0;

    let files: Vec<(File, Option<String>)> = input_files
        .map(|f| {
            let (file, result) = cw::count(&f);

            max_line_length = if file.max_line_length > max_line_length {
                file.max_line_length
            } else {
                    max_line_length
            };

            total_newlines += file.newlines;
            total_words += file.words;
            total_chars += file.chars;
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

        if matches.get_flag("lines") {
            println!(
                "{:>w$} {}",
                file.newlines,
                file.name,
                w = max_width as usize
            );
        } else if matches.get_flag("words") {
            println!("{:>w$} {}", file.words, file.name, w = max_width as usize);
        } else if matches.get_flag("chars") {
            println!("{:>w$} {}", file.chars, file.name, w = max_width as usize);
        } else if matches.get_flag("bytes") {
            println!("{:>w$} {}", file.bytes, file.name, w = max_width as usize);
        } else if matches.get_flag("max-line-length") {
            println!("{:>w$} {}", file.max_line_length, file.name, w = max_width as usize);
        } else {
            println!(
                "{:>w$} {:>w$} {:>w$} {}",
                file.newlines,
                file.words,
                file.bytes,
                file.name,
                w = max_width as usize
            );
        }
    }

    if matches.get_flag("lines") {
        println!("{:>w$} total", total_newlines, w = max_width as usize);
    } else if matches.get_flag("words") {
        println!("{:>w$} total", total_words, w = max_width as usize);
    } else if matches.get_flag("chars") {
        println!("{:>w$} total", total_chars, w = max_width as usize);
    } else if matches.get_flag("bytes") {
        println!("{:>w$} total", total_bytes, w = max_width as usize);
    } else if matches.get_flag("max-line-length") {
        println!("{:>w$} total", max_line_length, w = max_width as usize);
    } else {
        println!(
            "{:>w$} {:>w$} {:>w$} total",
            total_newlines,
            total_words,
            total_bytes,
            w = max_width as usize
        );
    }
}
