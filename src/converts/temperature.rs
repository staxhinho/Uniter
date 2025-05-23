use std::io;
use inquire::Select;

pub fn temperature() {
    dialog();
}

pub fn temperature_logic(input: f64, input_type: &str, output_type: &str, decimals: i64) -> f64 {
    let mut mid_temp: f64 = 0.0; // Mid temp is an intermediary variable to make conversion easier and it is based on celsius.

    if input_type == "c" {
        mid_temp = input;
    } else if input_type == "f" {
        mid_temp = (input - 32.0) * 5.0 / 9.0;
    } else if input_type  == "k" {
        mid_temp = input - 273.15;
    }

    let mut output_raw: f64 = 0.0;

    if output_type == "c" {
        output_raw = mid_temp;
    } else if output_type == "f" {
        output_raw = (mid_temp * 9.0 / 5.0) + 32.0;
    } else if output_type == "k" {
        output_raw = mid_temp + 273.15;
    }

    let output: f64 = crate::converts::round(output_raw, decimals);

    output
}

fn dialog() {
    fn select_unit(prompt_text: &str) -> Option<&'static str> {
        let options = vec!["Celsius", "Fahrenheit", "Kelvin"];
        Select::new(prompt_text, options).prompt().ok().and_then(|choice| match choice {
            "Celsius" => Some("c"),
            "Fahrenheit" => Some("f"),
            "Kelvin" => Some("k"),
            _ => {
                println!("Unknown option selected.");
                None
            }
        })
    }

    let input_type = match select_unit("Input temperature:") {
        Some(unit) => unit,
        None => return,
    };

    println!("Enter the temperature value:");
    let mut in_temp_str = String::new();
    if io::stdin().read_line(&mut in_temp_str).is_err() {
        println!("Failed to read line");
        return;
    }

    let input: f64 = match in_temp_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    let output_type = match select_unit("Output temperature:") {
        Some(unit) => unit,
        None => return,
    };

    println!("Enter the decimal value:");
    let mut decimal_str = String::new();
    if io::stdin().read_line(&mut decimal_str).is_err() {
        println!("Failed to read line");
        return;
    }

    let decimal: i64 = decimal_str.trim().parse::<i64>().expect("Failed to parse string to i64");

    let output = temperature_logic(input, input_type, output_type, decimal);
    println!("Output temperature: {}°{}", output, output_type.to_ascii_uppercase());
    crate::converts::aftermenu(temperature);
}