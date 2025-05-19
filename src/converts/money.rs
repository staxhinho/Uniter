use std::io;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ApiResponse {
   rates: HashMap<String, f64>,
}

pub fn money() {
    let mut in_iso = String::new();
    println!("Type input ISO(3/4 letter abreviation):");
    io::stdin().read_line(&mut in_iso).expect("Failed to read line");
    in_iso.make_ascii_uppercase();

    let mut in_amount = String::new();
    println!("Type input quantity:");
    io::stdin().read_line(&mut in_amount).expect("Failed to read line");

    let mut places = String::new();
    println!("Type input decimal places(1-15):");
    io::stdin().read_line(&mut places).expect("Failed to read line");

    let mut out_iso = String::new();
    println!("Type output ISO(3/4 letter abreviation):");
    io::stdin().read_line(&mut out_iso).expect("Failed to read line");
    out_iso.make_ascii_uppercase();

    let url: String = "https://api.fxratesapi.com/latest?currencies=".to_owned() + &out_iso.trim() + "&base=" + &in_iso.trim() + "&places=" + &places.trim() + "&amount=" + &in_amount.trim();

    let _ = request(url);
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