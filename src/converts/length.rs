use std::io;
use inquire::Select;

pub fn length() {
    dialog();
}


pub fn length_logic(input: f64, input_type: &str, output_type: &str, decimals: i64) -> f64 {
    let mut mid_length: f64 = 0.0; // Mid length is an intermediary variable to make conversion easier and it is based on the meter.

    if input_type == "km" {
        mid_length = input * 1000.0;
    } else if input_type == "m" {
        mid_length = input;
    } else if input_type == "dm" {
        mid_length = input / 10.0;
    } else if input_type == "cm" {
        mid_length = input / 100.0;
    } else if input_type == "mm" {
        mid_length = input / 1000.0;
    } else if input_type == "μm" {
        mid_length = input / 1000000.0
    } else if input_type == "hm" {
        mid_length = input / 1000000000.0
    } else if input_type == "mni" {
        mid_length = input * 1852.0;
    } else if input_type == "mi" {
        mid_length = input * 1609.344;
    } else if input_type == "jd" {
        mid_length = input * 0.9144;
    } else if input_type == "ft" {
        mid_length = input * 0.3048;
    } else if input_type == "in" {
        mid_length = input * 0.0254;
    } else if input_type == "ld" {
        mid_length = input * 384400000.0;
    } else if input_type == "au" {
        mid_length = input * 149597870700.0;
    } else if input_type == "ly" {
        mid_length = input * 9460700000000000.0;
    }

    let mut output_raw: f64 = 0.0;

    if output_type == "km" {
        output_raw = mid_length / 1000.0;
    } else if output_type == "m" {
        output_raw = mid_length;
    } else if output_type == "dm" {
        output_raw = mid_length * 10.0;
    } else if output_type == "cm" {
        output_raw = mid_length * 100.0;
    } else if output_type == "mm" {
        output_raw = mid_length * 1000.0;
    } else if output_type == "μm" {
        output_raw = mid_length * 1000000.0;
    } else if output_type == "hm" {
        output_raw = mid_length * 1000000000.0;
    } else if output_type == "mni" {
        output_raw = mid_length / 1852.0;
    } else if output_type == "mi" {
        output_raw = mid_length / 1609.344;
    } else if output_type == "jd" {
        output_raw = mid_length / 0.9144;
    } else if output_type == "ft" {
        output_raw = mid_length / 0.3048;
    } else if output_type == "in" {
        output_raw = mid_length / 0.0254;
    } else if output_type == "ld" {
        output_raw = mid_length / 384400000.0;
    } else if output_type == "au" {
        output_raw = mid_length / 149597870700.0;
    } else if output_type == "ly" {
        output_raw = mid_length / 9460700000000000.0;
    }

    let output: f64 = crate::converts::round(output_raw, decimals);
    
    output
}

fn dialog() {
    fn select_unit(prompt_text: &str) -> Option<&'static str> {
        let options = vec!["Kilometer(km)", "Meter(m)", "Decimeter(dm)", "Centimeter(cm)", "Milimeter(mm)", "Micrometer(μm)", "Nanometer(hm)", "Nautic Mile(mni)", "Mile(mi)", "Yard(jd)", "Foot(ft)", "Inch(in)", "Lunar Distance(ld)", "Astronomical Unit(au)", "Light Year(ly)"];
        Select::new(prompt_text, options).prompt().ok().and_then(|choice| match choice {
            "Kilometer(km)" => Some("km"),
            "Meter(m)" => Some("m"),
            "Decimeter(dm)" => Some("dm"),
            "Centimeter(cm)" => Some("cm"),
            "Milimeter(mm)" => Some("mm"),
            "Micrometer(μm)" => Some("μm"),
            "Nanometer(hm)" => Some("hm"),
            "Nautic Mile(mni)" => Some("mni"),
            "Mile(mi)" => Some("mi"),
            "Yard(jd)" => Some("jd"),
            "Foot(ft)" => Some("ft"),
            "Inch(in)" => Some("in"), 
            "Lunar Distance(ld)" => Some("ld"),
            "Astronomical Unit(au)" => Some("au"),
            "Light Year(ly)" => Some("ly"),
            _ => {
                println!("Unknown option selected.");
                None
            }
        })
    }

    let input_type = match select_unit("Input length:") {
        Some(unit) => unit,
        None => return,
    };

    println!("Enter the length value:");
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

    let output_type = match select_unit("Output length:") {
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

    let output = length_logic(input, input_type, output_type, decimal);
    println!("Output length: {}{}", output, output_type.to_ascii_lowercase());
    crate::converts::aftermenu(length);
}