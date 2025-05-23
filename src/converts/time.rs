use std::io;
use inquire::Select;

pub fn time() {
    dialog();
}

pub fn time_logic(input: f64, input_type: &str, output_type: &str, decimals: i64) -> f64 {
    let mut mid_time: f64 = 0.0; // Mid time is an intermediary variable to make conversion easier and it is based on minutes.

    if input_type == "y" {
        mid_time = input * 525600.0;
    } else if input_type == "wk" {
        mid_time = input * 10080.0;
    } else if input_type == "d" {
        mid_time = input * 1440.0;
    } else if input_type == "h" {
        mid_time = input * 60.0;
    } else if input_type == "min" {
        mid_time = input;
    } else if input_type == "s" {
        mid_time = input / 60.0;
    } else if input_type == "ms" {
        mid_time = input / 60000.0;
    } else if input_type == "μs" {
        mid_time = input / 60000000.0;
    } else if input_type == "ps" {
        mid_time = input / 60000000000000.0;
    }

    let mut output_raw: f64 = 0.0;

    if output_type == "y" {
        output_raw = mid_time / 525600.0;
    } else if output_type == "wk" {
        output_raw = mid_time / 10080.0;
    } else if output_type == "d" {
        output_raw = mid_time / 1440.0;
    } else if output_type == "h" {
        output_raw = mid_time / 60.0;
    } else if output_type == "min" {
        output_raw = mid_time;
    } else if output_type == "s" {
        output_raw = mid_time * 60.0;
    } else if output_type == "ms" {
        output_raw = mid_time * 60000.0;
    } else if output_type == "μs" {
        output_raw = mid_time * 60000000.0;
    } else if output_type == "ps" {
        output_raw = mid_time * 60000000000000.0;
    }

    let output: f64 = crate::converts::round(output_raw, decimals);

    output
}

fn dialog() {
    fn select_unit(prompt_text: &str) -> Option<&'static str> {
        let options = vec!["Year(y)", "Week(wk)", "Day(d)", "Hour(h)", "Minute(min)", "Second(s)", "Milisecond(ms)", "Microsecond(μs)", "Picosecond(ps)"];
        Select::new(prompt_text, options).prompt().ok().and_then(|choice| match choice {
            "Year(y)" => Some("y"),
            "Week(wk)" => Some("wk"),
            "Day(d)" => Some("d"), 
            "Hour(h)" => Some("h"),
            "Minute(min)" => Some("min"),
            "Second(s)" => Some("s"),
            "Milisecond(ms)" => Some("ms"),
            "Microsecond(μs)" => Some("μs"), 
            "Picosecond(ps)" => Some("ps"),
            _ => {
                println!("Unknown option selected.");
                None
            }
        })
    }

    let input_type = match select_unit("Input time:") {
        Some(unit) => unit,
        None => return,
    };

    println!("Enter the time value:");
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

    let output_type = match select_unit("Output time:") {
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

    let output = time_logic(input, input_type, output_type, decimal);
    println!("Output temperature: {}{}", output, output_type.to_ascii_lowercase());
    crate::converts::aftermenu(time);
}