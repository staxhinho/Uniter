use std::io;
use inquire::Select;

pub fn weight() {
    dialog();
}

pub fn weight_logic(input: f64, input_type: &str, output_type: &str, decimals: i64) -> f64 {
    let mut mid_weight: f64 = 0.0; // Mid weight is an intermediary variable to make conversion easier and it is based on the gram.

    if input_type == "t" {
        mid_weight = input * 1000000.0;
    } else if input_type == "kg" {
        mid_weight = input * 1000.0;
    } else if input_type == "g" {
        mid_weight = input;
    } else if input_type == "mg" {
        mid_weight = input / 1000.0;
    } else if input_type == "μg" {
        mid_weight = input / 1000000.0;
    } else if input_type == "l.t" {
        mid_weight = input * 1016046.9088;
    } else if input_type == "sh.t" {
        mid_weight = input * 907184.74;
    } else if input_type == "lb" {
        mid_weight = input * 453.59237;
    } else if input_type == "oz" {
        mid_weight = input * 28.349523125;
    } else {
        println!("Input type nonexistent.");
        crate::cli();
    }

    let mut output_raw: f64 = 0.0;

    if output_type == "t" {
        output_raw = mid_weight / 1000000.0;
    } else if output_type == "kg" {
        output_raw = mid_weight / 1000.0;
    } else if output_type == "g" {
        output_raw = mid_weight;
    } else if output_type == "mg" {
        output_raw = mid_weight * 1000.0
    } else if output_type == "μg" {
        output_raw = mid_weight * 1000000.0;
    } else if output_type == "l.t" {
        output_raw = mid_weight / 1016046.9088;
    } else if output_type == "sh.t" {
        output_raw = mid_weight / 907184.74;
    } else if output_type == "lb" {
        output_raw = mid_weight / 453.59237
    } else if output_type == "oz" {
        output_raw = mid_weight / 28.349523125;
    } else {
        println!("Output type nonexistent.");
        crate::cli();
    }

    let output: f64 = crate::converts::round(output_raw, decimals);

    output
}

fn dialog() {
    fn select_unit(prompt_text: &str) -> Option<&'static str> {
        let options = vec!["Ton(t)", "Kilogram(kg)", "Gram(g)", "Miligram(mg)", "Microgram(μg)", "British Ton(l.t)", "American Ton(sh.t)", "Pound(lb)", "Ounce(oz)"];
        Select::new(prompt_text, options).prompt().ok().and_then(|choice| match choice {
            "Ton(t)" => Some("t"),
            "Kilogram(kg)" => Some("kg"),
            "Gram(g)" => Some("g"),
            "Miligram(mg)" => Some("mg"),
            "Microgram(μg)" => Some("μg"),
            "British Ton(l.t)" => Some("l.t"),
            "American Ton(sh.t)" => Some("sh.t"),
            "Pound(lb)" => Some("lb"), 
            "Ounce(oz)" => Some("oz"),
            _ => {
                println!("Unknown option selected.");
                None
            }
        })
    }

    let input_type = match select_unit("Input weight:") {
        Some(unit) => unit,
        None => return,
    };

    println!("Enter the weight value:");
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

    let output_type = match select_unit("Output weight:") {
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

    let output = weight_logic(input, input_type, output_type, decimal);
    println!("Output weight: {}{}", output, output_type.to_ascii_lowercase());
    crate::converts::aftermenu(weight);
}