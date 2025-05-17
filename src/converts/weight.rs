use std::io;
use inquire::Select;

static mut MID_WEIGHT: f64 = 0.0; // Mid temp is an intermediary variable to make conversion easier and it is based on the gram.

pub fn weight() {
    let options = vec!["Metric", "Imperial"];
    let answer = Select::new("Input system:", options)
        .prompt();

    match answer {
        Ok(choice) => match choice {
            "Metric" => in_metric(),
            "Imperial" => in_imperial(),
            _ => {}
        },
        Err(err) => println!("There was an error: {}", err),
    };
}

fn in_metric() {
    let options = vec!["Ton(t)", "Kilogram(kg)", "Gram(g)", "Miligram(mg)", "Microgram(μg)", "Back"];
    let answer = Select::new("Select option:", options)
        .prompt();

    let w_option = match answer {
        Ok(choice) => {
            if choice == "Back" {
                weight();
                return;
            }

            match choice {
                "Ton(t)" => "t".to_string(),
                "Kilogram(kg)" => "kg".to_string(),
                "Gram(g)" => "g".to_string(),
                "Miligram(mg)" => "mg".to_string(),
                "Microgram(μg)" => "μg".to_string(),
                _ => {
                    println!("Unknown option");
                    return;
                }
            }
        },
        Err(err) => {
            println!("There was an error: {}", err);
            return;
        }
    };

    let mut weight_str = String::new();

    if w_option == "t" {
        println!("Input weight: ");
        io::stdin().read_line(&mut weight_str).expect("Failed to read line");
        let weight: f64 = weight_str.trim().parse().expect("Input not an integer");
        unsafe { MID_WEIGHT = weight * 1000000.0 };
        w_output();
    } else if w_option == "kg" {
        println!("Input weight: ");
        io::stdin().read_line(&mut weight_str).expect("Failed to read line");
        let weight: f64 = weight_str.trim().parse().expect("Input not an integer");
        unsafe { MID_WEIGHT = weight * 1000.0 };
        w_output();
    } else if w_option == "g" {
        println!("Input weight: ");
        io::stdin().read_line(&mut weight_str).expect("Failed to read line");
        let weight: f64 = weight_str.trim().parse().expect("Input not an integer");
        unsafe { MID_WEIGHT = weight };
        w_output();
    } else if w_option == "mg" {
        println!("Input weight: ");
        io::stdin().read_line(&mut weight_str).expect("Failed to read line");
        let weight: f64 = weight_str.trim().parse().expect("Input not an integer");
        unsafe { MID_WEIGHT = weight / 1000.0 };
        w_output();
    } else if w_option == "μg" {
        println!("Input weight: ");
        io::stdin().read_line(&mut weight_str).expect("Failed to read line");
        let weight: f64 = weight_str.trim().parse().expect("Input not an integer");
        unsafe { MID_WEIGHT = weight / 1000000.0 };
        w_output();
    } 
}

fn in_imperial() {
    let options = vec!["British Ton(l.t)", "American Ton(sh.t)", "Pound(lb)", "Ounce(oz)", "Back"];
    let answer = Select::new("Select option:", options)
        .prompt();

    let w_option = match answer {
        Ok(choice) => {
            if choice == "Back" {
                weight();
                return;
            }

            match choice {
                "British Ton(l.t)" => "lt".to_string(),
                "American Ton(sh.t)" => "sht".to_string(),
                "Pound(lb)" => "lb".to_string(),
                "Ounce(oz)" => "oz".to_string(),
                _ => {
                    println!("Unknown option");
                    return;
                }
            }
        },
        Err(err) => {
            println!("There was an error: {}", err);
            return;
        }
    };

    let mut weight_str = String::new();

    if w_option == "lt" {
        println!("Input weight: ");
        io::stdin().read_line(&mut weight_str).expect("Failed to read line");
        let weight: f64 = weight_str.trim().parse().expect("Input not an integer");
        unsafe { MID_WEIGHT = weight * 1016046.9088 };
        w_output();
    } else if w_option == "sht" {
        println!("Input weight: ");
        io::stdin().read_line(&mut weight_str).expect("Failed to read line");
        let weight: f64 = weight_str.trim().parse().expect("Input not an integer");
        unsafe { MID_WEIGHT = weight * 907184.74 };
        w_output();
    } else if w_option == "lb" {
        println!("Input weight: ");
        io::stdin().read_line(&mut weight_str).expect("Failed to read line");
        let weight: f64 = weight_str.trim().parse().expect("Input not an integer");
        unsafe { MID_WEIGHT = weight * 453.59237 };
        w_output();
    } else if w_option == "oz" {
        println!("Input weight: ");
        io::stdin().read_line(&mut weight_str).expect("Failed to read line");
        let weight: f64 = weight_str.trim().parse().expect("Input not an integer");
        unsafe { MID_WEIGHT = weight * 28.349523125 };
        w_output();
    }
}

fn w_output() {
    let options = vec!["Metric", "Imperial"];
    let answer = Select::new("Output system:", options)
        .prompt();

    match answer {
        Ok(choice) => match choice {
            "Metric" => out_metric(),
            "Imperial" => out_imperial(),
            _ => {}
        },
        Err(err) => println!("There was an error: {}", err),
    };
}

fn out_metric() {
    let options = vec!["Ton(t)", "Kilogram(kg)", "Gram(g)", "Miligram(mg)", "Microgram(μg)", "Back"];
    let answer = Select::new("Select option:", options)
        .prompt();

    let w_option = match answer {
        Ok(choice) => {
            if choice == "Back" {
                weight();
                return;
            }

            match choice {
                "Ton(t)" => "t".to_string(),
                "Kilogram(kg)" => "kg".to_string(),
                "Gram(g)" => "g".to_string(),
                "Miligram(mg)" => "mg".to_string(),
                "Microgram(μg)" => "μg".to_string(),
                _ => {
                    println!("Unknown option");
                    return;
                }
            }
        },
        Err(err) => {
            println!("There was an error: {}", err);
            return;
        }
    };

    let mut out_weight: f64 = 0.0;

    if w_option == "t" {
        unsafe { out_weight = MID_WEIGHT / 1000000.0 };
        println!("Output weight: {}", out_weight);
        aftermenu();
    } else if w_option == "kg" {
        unsafe { out_weight = MID_WEIGHT / 1000.0 };
        println!("Output weight: {}", out_weight);
        aftermenu();
    } else if w_option == "g" {
        unsafe { out_weight = MID_WEIGHT };
        println!("Output weight: {}", out_weight);
        aftermenu();
    } else if w_option == "mg" {
        unsafe { out_weight = MID_WEIGHT * 1000.0 };
        println!("Output weight: {}", out_weight);
        aftermenu();
    } else if w_option == "μg" {
        unsafe { out_weight = MID_WEIGHT * 1000000.0 };
        println!("Output weight: {}", out_weight);
        aftermenu();
    } 
}

fn out_imperial() {
    let options = vec!["British Ton(l.t)", "American Ton(sh.t)", "Pound(lb)", "Ounce(oz)", "Back"];
    let answer = Select::new("Select option:", options)
        .prompt();

    let w_option = match answer {
        Ok(choice) => {
            if choice == "Back" {
                weight();
                return;
            }

            match choice {
                "British Ton(l.t)" => "lt".to_string(),
                "American Ton(sh.t)" => "sht".to_string(),
                "Pound(lb)" => "lb".to_string(),
                "Ounce(oz)" => "oz".to_string(),
                _ => {
                    println!("Unknown option");
                    return;
                }
            }
        },
        Err(err) => {
            println!("There was an error: {}", err);
            return;
        }
    };

    let mut out_weight: f64 = 0.0;

    if w_option == "lt" {
        unsafe { out_weight = MID_WEIGHT / 1016046.9088 };
        println!("Output weight: {}", out_weight);
        aftermenu();
    } else if w_option == "sht" {
        unsafe { out_weight = MID_WEIGHT / 907184.74 };
        println!("Output weight: {}", out_weight);
        aftermenu();
    } else if w_option == "lb" {
        unsafe { out_weight = MID_WEIGHT / 453.59237 };
        println!("Output weight: {}", out_weight);
        aftermenu();
    } else if w_option == "oz" {
        unsafe { out_weight = MID_WEIGHT / 28.349523125 };
        println!("Output weight: {}", out_weight);
        aftermenu();
    }
}

fn aftermenu() {
    let options = vec!["Continue", "Back", "Exit"];

    let answer = Select::new("", options)
        .prompt();
    
    match answer {
        Ok(choice) => match choice {
            "Continue" => weight(),
            "Back" => crate::main_select(),
            "Exit" => std::process::exit(0),
            _ => {
                println!("Unknown option selected.");
                return;
            },
        },
        Err(err) => {
            println!("There was an error: {}", err);
            return;
        }
    }
}