#![allow(unused_assignments)]
use inquire::Select;
mod converts;
mod cli;

fn main() {
    println!("Uniter -- The best unit converter!");

    cli::cli();
    convert_select();
}

pub fn convert_select() {
    let options = vec!["Temperature", "Weight", "Length", "Money", "Exit"];

    let answer = Select::new("Choose an option:", options)
        .prompt();

    match answer {
        Ok(choice) => match choice {
            "Temperature" => converts::temperature(),
            "Weight" => converts::weight(),
            "Length" => converts::length(),
            "Money" => converts::money(),
            "Exit" => std::process::exit(0),
            _ => println!("Unknown option selected."),
        },
        Err(err) => println!("There was an error: {}", err),
    }
}