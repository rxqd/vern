use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let code_lines = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            if code_lines.is_empty() {
                println!("EOF  null");
                return;
            }

            scan_code(&code_lines);
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

fn scan_code(code_lines: &str) {
    let mut parsed_lines: Vec<String> = code_lines.split('\n').map(scan_code_line).collect();

    parsed_lines.push("EOF  null".to_owned());

    for line in parsed_lines {
        println!("{line}");
    }
}

fn scan_code_line(line: &str) -> String {
    let mut result = String::new();

    for c in line.chars() {
        match c {
            '(' => result.push_str("LEFT_PAREN ( null"),
            ')' => result.push_str("RIGHT_PAREN ) null"),
            '+' => result.push_str("PLUS + null"),
            '-' => result.push_str("MINUS - null"),
            '*' => result.push_str("MULTIPLY * null"),
            '/' => result.push_str("DIVIDE / null"),
            '=' => result.push_str("EQUAL = null"),
            ' ' | '\t' => {}
            _ => result.push_str(&format!("UNKNOWN {} null ", c)),
        }

        result.push('\n');
    }

    result.trim().to_owned()
}
