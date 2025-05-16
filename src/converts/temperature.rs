use std::io;
use inquire::Select;

pub fn temperature() {
    let options = vec!["Celsius", "Fahrenheit", "Kelvin"];

    let answer = Select::new("Input temperature:", options)
        .prompt();

    let in_temp_opt = match answer {
        Ok(choice) => match choice {
            "Celsius" => "c".to_string(),
            "Fahrenheit" => "f".to_string(),
            "Kelvin" => "k".to_string(),
            _ => {
                println!("Unknown option selected.");
                return;
            },
        },
        Err(err) => {
            println!("There was an error: {}", err);
            return;
        }
    };

    println!("Enter the temperature value:");
    let mut in_temp_str = String::new();

    io::stdin()
        .read_line(&mut in_temp_str)
        .expect("Failed to read line");

    let in_temp_str = in_temp_str.trim(); // remove newline
    let in_temp_val: f64 = match in_temp_str.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    if in_temp_opt == "c" {

        let out_options = vec!["Fahrenheit", "Kelvin"];
        let out_answer = Select::new("Output temperature:", out_options)
            .prompt();

        let out_temp_opt = match out_answer {
            Ok(choice) => match choice {
                "Fahrenheit" => "f".to_string(),
                "Kelvin" => "k".to_string(),
                _ => {
                    println!("Unknown option selected.");
                    return;
                },
            },
            Err(err) => {
                println!("There was an error: {}", err);
                return;
            }
        };

        if out_temp_opt == "f" {
            let out_temp = (in_temp_val * 9.0 / 5.0) + 32.0;
            let formatted = format!("{:.10}", out_temp).trim_end_matches('0').trim_end_matches('.').to_string();
            println!("Output temperature: {:.2} °F", formatted);
            aftermenu();
        } else if out_temp_opt == "k" {
            let out_temp = in_temp_val + 273.15;
            let formatted = format!("{:.10}", out_temp).trim_end_matches('0').trim_end_matches('.').to_string();
            println!("Output temperature: {:.2} °K", formatted);
            aftermenu();
        }

    } else if in_temp_opt == "f" {
        let out_options = vec!["Celsius", "Kelvin"];
        let out_answer = Select::new("Output temperature:", out_options)
            .prompt();

        let out_temp_opt = match out_answer {
            Ok(choice) => match choice {
                "Celsius" => "c".to_string(),
                "Kelvin" => "k".to_string(),
                _ => {
                    println!("Unknown option selected.");
                    return;
                },
            },
            Err(err) => {
                println!("There was an error: {}", err);
                return;
            }
        };

        if out_temp_opt == "c" {
            let out_temp = (in_temp_val - 32.0) * 5.0 / 9.0;
            let formatted = format!("{:.10}", out_temp).trim_end_matches('0').trim_end_matches('.').to_string();
            println!("Output temperature: {:.2} °C", formatted);
            aftermenu();
        } else if out_temp_opt == "k" {
            let out_temp = ((in_temp_val - 32.0) * 5.0 / 9.0) + 273.15;
            let formatted = format!("{:.10}", out_temp).trim_end_matches('0').trim_end_matches('.').to_string();
            println!("Output temperature: {:.2} °K", formatted);
            aftermenu();
        }

    } else if in_temp_opt == "k" {

        let out_options = vec!["Celsius", "Fahrenheit"];
        let out_answer = Select::new("Output temperature:", out_options)
            .prompt();

        let out_temp_opt = match out_answer {
            Ok(choice) => match choice {
                "Celsius" => "c".to_string(),
                "Fahrenheit" => "f".to_string(),
                _ => {
                    println!("Unknown option selected.");
                    return;
                },
            },
            Err(err) => {
                println!("There was an error: {}", err);
                return;
            }
        };

        if out_temp_opt == "c" {
            let out_temp = in_temp_val - 273.15;
            let formatted = format!("{:.10}", out_temp).trim_end_matches('0').trim_end_matches('.').to_string();
            println!("Output temperature: {:.2} °C", formatted);
            aftermenu();
        } else if out_temp_opt == "f" {
            let out_temp = ((in_temp_val - 273.15) * 9.0 / 5.0) + 32.0;
            let formatted = format!("{:.10}", out_temp).trim_end_matches('0').trim_end_matches('.').to_string();
            println!("Output temperature: {} °F", formatted);
            aftermenu();
        }
    }

}

fn aftermenu() {
    let options = vec!["Continue", "Exit"];

    let answer = Select::new("", options)
        .prompt();
    
    match answer {
        Ok(choice) => match choice {
            "Continue" => temperature(),
            "Exit" => std::process::exit(0),
            _ => {
                println!("Unknown option selected.");
                return;
            },
        },
        Err(err) => {
            println!("There was an error: {}", err);
            return;
        }
    }
}