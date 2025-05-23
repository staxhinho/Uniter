use cli::cli;
use inquire::Select;
mod converts;
mod cli;

fn main() {
    println!("Uniter -- The best unit converter!");
    
    cli();

    /*let answer = Select::new("Choose an option:", vec!["CLI", "Interface","Exit"])
        .prompt();

    match answer {
        Ok(choice) => match choice {
            "CLI" => cli::cli(),
            "Interface" => convert_select(),
            "Exit" => std::process::exit(0),
            _ => println!("Unknown option selected."),
        },
        Err(err) => println!("There was an error: {}", err),
    } */
}

pub fn convert_select() {
    let options = vec!["Temperature", "Weight", "Length", "Money", "Time", "Volume", "Back", "Exit"];

    let answer = Select::new("Choose an option:", options)
        .prompt();

    match answer {
        Ok(choice) => match choice {
            "Temperature" => converts::temperature(),
            "Weight" => converts::weight(),
            "Length" => converts::length(),
            "Money" => converts::money(),
            "Time" => converts::time(),
            "Volume" => converts::volume(),
            "Back" => main(),
            "Exit" => std::process::exit(0),
            _ => println!("Unknown option selected."),
        },
        Err(err) => println!("There was an error: {}", err),
    }
}