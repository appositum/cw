use cw::{self, cli, Error, File};

fn main() {
    let matches = cli::cmd().get_matches();

    let mut input_files = matches.get_many::<String>("file").unwrap();

    if input_files.len() == 1 {
        let (file, result) = cw::parse_file(&input_files.next().unwrap());

        if let Some(Error::FileNotFound(e)) = result {
            eprintln!("{}", e);
            return;
        }

        if let Some(Error::IsDirectory(e)) = result {
            eprintln!("{}", e);
        }

        let max_width = if file.bytes > 0 {
            // `x.ilog10() + 1` == number of digits in `x`
            file.bytes.ilog10() + 1
        } else {
            1
        };

        cw::print(file, max_width as usize);
        return;
    }

    let mut total_newlines = 0;
    let mut total_words = 0;
    let mut total_chars = 0;
    let mut total_bytes = 0;
    let mut max_line_length = 0;

    let files: Vec<(File, Option<Error>)> = input_files
        .map(|f| {
            let (file, result) = cw::parse_file(&f);

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

        if let Some(Error::FileNotFound(e)) = result {
            eprintln!("{}", e);
            continue;
        }

        if let Some(Error::IsDirectory(e)) = result {
            eprintln!("{}", e);
        }

        cw::print(file, max_width as usize);
    }

    let total = File {
        name: "total".to_string(),
        newlines: total_newlines,
        words: total_words,
        chars: total_chars,
        bytes: total_bytes,
        max_line_length,
    };

    cw::print(total, max_width as usize);
}
