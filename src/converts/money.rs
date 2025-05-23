use std::io;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ApiResponse {
   rates: HashMap<String, f64>,
}

pub fn money() {
    dialog();
}

pub fn money_logic(input: f64, input_type: &str, output_type: &str, decimals: i64) {
    let input_string = input.to_string();
    let input_str: &str = &input_string;
    let decimals_str = decimals.to_string();

    let url: String = "https://api.fxratesapi.com/latest?currencies=".to_owned() + &output_type.to_ascii_uppercase().trim() + "&base=" + &input_type.to_ascii_uppercase().trim() + "&places=" + &decimals_str.trim() + "&amount=" + &input_str.trim();

    let _ = request(url);
}

fn dialog() {
    let mut in_iso = String::new();
    println!("Type input ISO(3/4 letter abreviation):");
    io::stdin().read_line(&mut in_iso).expect("Failed to read line");
    in_iso.make_ascii_uppercase();

    let mut in_amount = String::new();
    println!("Type input quantity:");
    io::stdin().read_line(&mut in_amount).expect("Failed to read line");

    let mut out_iso = String::new();
    println!("Type output ISO(3/4 letter abreviation):");
    io::stdin().read_line(&mut out_iso).expect("Failed to read line");
    out_iso.make_ascii_uppercase();

    println!("Enter the decimal value:");
    let mut decimal_str = String::new();
    if io::stdin().read_line(&mut decimal_str).is_err() {
        println!("Failed to read line");
        return;
    }

    let mut decimal: i64 = decimal_str.trim().parse::<i64>().expect("Failed to parse string to i64");

    if decimal > 15 {
        println!("Decimals cannot be greater than 15. Using 15 as the decimal count");
        decimal = 15;
    }

    let decimal_str_2: String = decimal.to_string();

    let url: String = "https://api.fxratesapi.com/latest?currencies=".to_owned() + &out_iso.trim() + "&base=" + &in_iso.trim() + "&places=" + &decimal_str_2.trim() + "&amount=" + &in_amount.trim();

    let _ = request(url);

    crate::converts::aftermenu(money);
}

fn request(url: String) -> Result<(), reqwest::Error> {
    let req = reqwest::blocking::get(url)?
        .json::<ApiResponse>()?;


    if let Some((currency, rate)) = req.rates.iter().next() {
        println!("{}{}", rate, currency);
    } else {
        println!("No rates found.");
    }

    Ok(())
}