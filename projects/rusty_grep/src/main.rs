use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

struct Config {
    pattern: String,
    files: Vec<String>,
    case_insensitive: bool,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(format!("Usage: {} pattern file (or files [-i])", args[0]));
        }

        let mut case_insensitive: bool = false;
        let mut non_flag_args: Vec<String> = Vec::new();

        for arg in &args[1..] {
            if arg == "-i" {
                case_insensitive = true;
            } else {
                non_flag_args.push(arg.clone());
            }
        }
        if non_flag_args.is_empty() {
            return Err("Error: Pattern not provided.".to_string());
        }

        let pattern: String = non_flag_args[0].clone();

        if non_flag_args.len() < 2 {
            return Err("Error, Non input files provided.".to_string());
        }
        let files: Vec<String> = non_flag_args[1..].to_vec();

        Ok(Config {
            pattern,
            files,
            case_insensitive,
        })
    }
}

fn grep(reader: &mut BufReader<File>, pattern: &str, case_insensitive: bool) {
    let mut line: String = String::new();
    while let Ok(bytes_read) = reader.read_line(&mut line) {
        if bytes_read == 0 {
            break;
        }
        let matched;

        if case_insensitive {
            matched = line.to_lowercase().contains(&pattern.to_lowercase());
        } else {
            matched = line.contains(pattern)
        }

        if matched {
            print!("{}", line);
        }
        line.clear();
    }
}

fn grep_file(file: &str, config: &Config) {
    match File::open(file) {
        Ok(f) => {
            let mut reader = BufReader::new(f);
            grep(&mut reader, &config.pattern, config.case_insensitive);
        }
        Err(e) => {
            eprintln!("Could not open file: '{}': {}", file, e);
        }
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args)?;

    for file in &config.files {
        grep_file(file, &config);
    }

    Ok(())
}
