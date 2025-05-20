use std::{io};

pub fn cli() {
    println!("Using uniter cli");

    let mut input_str: String = String::new();

    io::stdin().read_line(&mut input_str).expect("Failed to read line");

    let parts: Vec<&str> = input_str.trim().split_whitespace().collect();

    if parts.len() != 3 {
        eprintln!("Invalid input. Expected format: <convert> <value><input_unit> <output_unit>");
        return;
    }

    let convert = parts[0];

    let mut number_part = String::new();
    let mut unit_part = String::new();

    for c in parts[1].chars() {
        if c.is_digit(10) || c == '.' {
            number_part.push(c);
        } else {
            unit_part.push(c);
        }
    }

    let input: f64 = match number_part.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid number: {}", number_part);
            return;
        }
    };

    let input_type = unit_part;
    let output_type = parts[2];

    println!("convert = {}", convert);
    println!("input = {}", input);
    println!("input_type = {}", input_type);
    println!("output_type = {}", output_type);
}