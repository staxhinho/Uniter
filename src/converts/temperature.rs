use std::io;
use inquire::Select;

pub fn temperature() {
    let options = vec!["Celsius", "Fahrenheit"];

    let answer = Select::new("Input temperature:", options)
        .prompt();

    let mut in_temp_opt = String::new();

    match answer {
        Ok(choice) => match choice {
            "Celsius" => in_temp_opt = "c".to_string(),
            "Fahrenheit" => in_temp_opt = "f".to_string(),
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
        let out_temp = (in_temp_val * 9.0 / 5.0) + 32.0;
        println!("Output temperature: {:.2} °F", out_temp); // 2 decimal places
    } else if in_temp_opt == "f" {
        let out_temp = (in_temp_val - 32.0) * 5.0 / 9.0;
        println!("Output temperature: {:.2} °C", out_temp);
    }
}
