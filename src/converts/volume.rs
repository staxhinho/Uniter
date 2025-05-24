use std::io;
use inquire::Select;

pub fn volume() {
    dialog();
}

pub fn volume_logic(input: f64, input_type: &str, output_type: &str, decimals: i64) -> f64 {
    let mut mid_volume: f64 = 0.0; // Mid volume is an intermediary variable to make conversion easier and it is based on cubic meters.

    if input_type == "m3" {
        mid_volume = input;
    } else if input_type == "dm3" {
        mid_volume = input / 1000.0;
    } else if input_type == "cm3" {
        mid_volume = input / 1_000_000.0;
    } else if input_type == "mm3" {
        mid_volume = input / 1_000_000_000.0;
    } else if input_type == "hl" {
        mid_volume = input / 10.0;
    } else if input_type == "l" {
        mid_volume = input / 1000.0;
    } else if input_type == "dl" {
        mid_volume = input / 10_000.0;
    } else if input_type == "cl" {
        mid_volume = input / 100_000.0;
    } else if input_type == "ml" {
        mid_volume = input / 1_000_000.0;
    } else if input_type == "ft3" {
        mid_volume = input * 0.0283168;
    } else if input_type == "in3" {
        mid_volume = input * 0.0000163871;
    } else if input_type == "yd3" {
        mid_volume = input * 0.764555;
    } else if input_type == "ac3" {
        mid_volume = input * 1233.48;
    } else if input_type == "gal" {
        mid_volume = input * 0.00378541;
    } else if input_type == "bbl" {
        mid_volume = input * 0.158987;
    } else {
        println!("Input type nonexistent.");
        crate::cli();
    }

    let mut output_raw: f64 = 0.0;

    if output_type == "m3" {
        output_raw = mid_volume;
    } else if output_type == "dm3" {
        output_raw = mid_volume * 1000.0;
    } else if output_type == "cm3" {
        output_raw = mid_volume * 1_000_000.0;
    } else if output_type == "mm3" {
        output_raw = mid_volume * 1_000_000_000.0;
    } else if output_type == "hl" {
        output_raw = mid_volume * 10.0;
    } else if output_type == "l" {
        output_raw = mid_volume * 1000.0;
    } else if output_type == "dl" {
        output_raw = mid_volume * 10_000.0;
    } else if output_type == "cl" {
        output_raw = mid_volume * 100_000.0;
    } else if output_type == "ml" {
        output_raw = mid_volume * 1_000_000.0;
    } else if output_type == "ft3" {
        output_raw = mid_volume / 0.0283168;
    } else if output_type == "in3" {
        output_raw = mid_volume / 0.0000163871;
    } else if output_type == "yd3" {
        output_raw = mid_volume / 0.764555;
    } else if output_type == "ac3" {
        output_raw = mid_volume / 1233.48;
    } else if output_type == "gal" {
        output_raw = mid_volume / 0.00378541;
    } else if output_type == "bbl" {
        output_raw = mid_volume / 0.158987;
    } else {
        println!("Output type nonexistent.");
        crate::cli();
    }

    let output: f64 = crate::converts::round(output_raw, decimals);

    output
}

fn dialog() {
    fn select_unit(prompt_text: &str) -> Option<&'static str> {
        let options = vec![
            "Cubic meter(m3)", "Cubic decimeter(dm3)", "Cubic centimeter(cm3)", "Cubic millimeter(mm3)",
            "Hectoliter(hl)", "Liter(l)", "Deciliter(dl)", "Centiliter(cl)", "Milliliter(ml)",
            "Cubic feet(ft3)", "Cubic inch(in3)", "Cubic yard(yd3)", "Cubic acre(ac3)",
            "Gallon(gal)", "Barrel(bbl)"
        ];
        Select::new(prompt_text, options).prompt().ok().and_then(|choice| match choice {
            "Cubic meter(m3)" => Some("m3"),
            "Cubic decimeter(dm3)" => Some("dm3"),
            "Cubic centimeter(cm3)" => Some("cm3"),
            "Cubic millimeter(mm3)" => Some("mm3"),
            "Hectoliter(hl)" => Some("hl"),
            "Liter(l)" => Some("l"),
            "Deciliter(dl)" => Some("dl"),
            "Centiliter(cl)" => Some("cl"),
            "Milliliter(ml)" => Some("ml"),
            "Cubic feet(ft3)" => Some("ft3"),
            "Cubic inch(in3)" => Some("in3"),
            "Cubic yard(yd3)" => Some("yd3"),
            "Cubic acre(ac3)" => Some("ac3"),
            "Gallon(gal)" => Some("gal"),
            "Barrel(bbl)" => Some("bbl"),
            _ => {
                println!("Unknown option selected.");
                None
            }
        })
    }

    let input_type = match select_unit("Input volume:") {
        Some(unit) => unit,
        None => return,
    };

    println!("Enter the volume value:");
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

    let output_type = match select_unit("Output volume:") {
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

    let output = volume_logic(input, input_type, output_type, decimal);
    println!("Output volume: {}{}", output, output_type.to_ascii_lowercase());
    crate::converts::aftermenu(volume);
}