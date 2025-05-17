use inquire::Select;
mod converts;

fn main() {
    println!("Uniter -- The best unit converter!");

    main_select();
}

pub fn main_select() {
        let options = vec!["Temperature", "Weight", "Length", "Exit"];

    let answer = Select::new("Choose an option:", options)
        .prompt();

    match answer {
        Ok(choice) => match choice {
            "Temperature" => converts::temperature(),
            "Weight" => converts::weight(),
            "Length" => converts::length(),
            "Exit" => std::process::exit(0),
            _ => println!("Unknown option selected."),
        },
        Err(err) => println!("There was an error: {}", err),
    }
}