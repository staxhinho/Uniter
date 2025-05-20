use inquire::Select;

pub fn aftermenu(function: fn()) {
    match Select::new("", vec!["Continue", "Back", "Exit"]).prompt() {
        Ok("Continue") => function(),
        Ok("Back") => crate::convert_select(),
        Ok("Exit") => std::process::exit(0),
        Ok(_) => println!("Unknown option selected."),
        Err(err) => println!("There was an error: {}", err),
    }
}