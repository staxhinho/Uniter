#![allow(unused_assignments)]
use inquire::Select;
mod converts;
mod cli;

fn main() {
    println!("Uniter -- The best unit converter!");

    let answer = Select::new("Choose an option:", vec!["CLI", "Interface","Exit"])
        .prompt();

    match answer {
        Ok(choice) => match choice {
            "CLI" => cli::cli(),
            "Interface" => convert_select(),
            "Exit" => std::process::exit(0),
            _ => println!("Unknown option selected."),
        },
        Err(err) => println!("There was an error: {}", err),
    }
}

pub fn convert_select() {
    let options = vec!["Temperature", "Weight", "Length", "Money", "Back", "Exit"];

    let answer = Select::new("Choose an option:", options)
        .prompt();

    match answer {
        Ok(choice) => match choice {
            "Temperature" => converts::temperature(),
            "Weight" => converts::weight(),
            "Length" => converts::length(),
            "Money" => converts::money(),
            "Back" => main(),
            "Exit" => std::process::exit(0),
            _ => println!("Unknown option selected."),
        },
        Err(err) => println!("There was an error: {}", err),
    }
}