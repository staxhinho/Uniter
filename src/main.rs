//use std::io;
use inquire::Select;
mod converts;

fn main() {
    println!("Uniter -- The best unit converter!");

    let options = vec!["Temperature", "Weight", "Length"];

    let answer = Select::new("Choose an option:", options)
        .prompt();

    match answer {
        Ok(choice) => match choice {
            "Temperature" => converts::temperature(),
            "Weight" => converts::weight(),
            "Length" => println!("Length conversion not implemented yet."),
            _ => println!("Unknown option selected."),
        },
        Err(err) => println!("There was an error: {}", err),
    }
}

