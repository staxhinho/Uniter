use std::{io};
use regex::Regex;
use crate::{converts, main};

pub fn cli() {
    let mut input_str: String = String::new();

    io::stdin().read_line(&mut input_str).expect("Failed to read line");

    let parts: Vec<&str> = input_str.trim().split_whitespace().collect();

    if parts.len() == 1 {
        match parts[0] {
            "back" => main(),
            "exit" => std::process::exit(0),
            _ => eprintln!("Unknown command: {}", parts[0]),
        }
    }

    if parts.len() != 3 {
        eprintln!("Invalid input. Expected format: <convert> <value><input_unit> <output_unit>");
        return;
    } 

    let convert = parts[0];
    let input_with_unit = parts[1];
    let output_type = parts[2];

    let re = Regex::new(r"^([0-9]*\.?[0-9]+)([a-zA-Z\./]+)$").unwrap();

    let caps = match re.captures(input_with_unit) {
        Some(c) => c,
        None => {
            eprintln!("Invalid input format: {}", input_with_unit);
            return;
        }
    };

    let input: f64 = match caps[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid number: {}", &caps[1]);
            return;
        }
    };

    let input_type = &caps[2];

    if convert == "t" {
        let output = converts::temperature_logic(input, input_type, output_type);
        println!("{}°{}", output, output_type.to_ascii_uppercase());
        cli();
    } else if convert == "l" {
        let output = converts::length_logic(input, input_type, output_type);
        println!("{}{}", output, output_type.to_ascii_lowercase());
        cli();
    } else if convert == "m" {
        converts::money_logic(input, input_type, output_type);
        cli();
    } else if convert == "w" {
        let output = converts::weight_logic(input, input_type, output_type);
        println!("{}{}", output, output_type.to_ascii_lowercase());
        cli();
    } else if convert == "ti" {
        let output = converts::time_logic(input, input_type, output_type);
        println!("{}{}", output, output_type.to_ascii_lowercase());
        cli();
    } else if convert == "v" {
        let output = converts::volume_logic(input, input_type, output_type);
        println!("{}{}", output, output_type.to_ascii_lowercase());
        cli();
    }
}