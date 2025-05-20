use std::io;
use inquire::Select;

pub fn length() {
    dialog();
    /* let options = vec!["Metric", "Imperial", "Space"];
    let answer = Select::new("Input system:", options)
        .prompt();

    match answer {
        Ok(choice) => match choice {
            "Metric" => in_metric(),
            "Imperial" => in_imperial(),
            "Space" => in_space(),
            _ => {}
        },
        Err(err) => println!("There was an error: {}", err),
    }; */
}

/* fn in_metric() {
    let options = vec!["Kilometer(km)", "Meter(m)", "Decimeter(dm)", "Centimeter(cm)", "Milimeter(mm)", "Micrometer(μm)", "Nanometer(hm)", "Back"];
    let answer = Select::new("Select option:", options)
        .prompt();

    let mut length_str = String::new();
    io::stdin().read_line(&mut length_str).expect("Failed to read line");
    let length_f: f64 = length_str.trim().parse().expect("Input not a integer");

    match answer {
        Ok(choice) => {
            if choice == "Back" {
                length();
                return;
            }

            match choice {
                "Kilometer(km)" => { unsafe { MID_LENGHT = length_f * 1000.0 } l_output(); },
                "Meter(m)" => { unsafe { MID_LENGHT = length_f } l_output(); },
                "Decimeter(dm)" => { unsafe { MID_LENGHT = length_f / 10.0 } l_output(); },
                "Centimeter(cm)" => { unsafe { MID_LENGHT = length_f / 100.0 } l_output(); },
                "Milimeter(mm)" => { unsafe { MID_LENGHT = length_f / 1000.0 } l_output(); },
                "Micrometer(μm)" => { unsafe { MID_LENGHT = length_f / 1000000.0  } l_output(); },
                "Nanometer(hm)" => { unsafe { MID_LENGHT = length_f / 1000000000.0 } l_output(); },
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
    }
}

fn in_imperial() {
    let options = vec!["Nautic Mile(mni)", "Mile(mi)", "Yard(jd)", "Foot(ft)", "Inch(in)", "Back"];
    let answer = Select::new("Select option:", options)
        .prompt();

    let mut length_str = String::new();
    io::stdin().read_line(&mut length_str).expect("Failed to read line");
    let length_f: f64 = length_str.trim().parse().expect("Input not a integer");

    match answer {
        Ok(choice) => {
            if choice == "Back" {
                length();
                return;
            }

            match choice {
                "Nautic Mile(mni)" => { unsafe { MID_LENGHT = length_f * 1852.0 } l_output();},
                "Mile(mi)" => { unsafe { MID_LENGHT = length_f * 1609.344} l_output(); },
                "Yard(jd)" => { unsafe { MID_LENGHT = length_f * 0.9144 } l_output(); },
                "Foot(ft)" => { unsafe { MID_LENGHT = length_f * 0.3048 } l_output(); },
                "Inch(in)" => { unsafe { MID_LENGHT = length_f * 0.0254 } l_output(); },
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
    }
}

fn in_space() {
    let options = vec!["Lunar Distance(ld)", "Astronomical Unit(au)", "Light Year(ly)", "Back"];
    let answer = Select::new("Select option:", options)
        .prompt();

    let mut length_str = String::new();
    io::stdin().read_line(&mut length_str).expect("Failed to read line");
    let length_f: f64 = length_str.trim().parse().expect("Input not a integer");

    match answer {
        Ok(choice) => {
            if choice == "Back" {
                length();
                return;
            }

            match choice {
                "Lunar Distance(ld)" => { unsafe { MID_LENGHT = length_f * 384400000.0 } l_output(); },
                "Astronomical Unit(au)" => { unsafe { MID_LENGHT = length_f * 149597870700.0 } l_output(); },
                "Light Year(ly)" => { unsafe { MID_LENGHT = length_f * 9460700000000000.0 } l_output(); },
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
    }
}

fn l_output() {
    let options = vec!["Metric", "Imperial", "Space"];
    let answer = Select::new("Output system:", options)
        .prompt();

    match answer {
        Ok(choice) => match choice {
            "Metric" => out_metric(),
            "Imperial" => out_imperial(),
            "Space" => out_space(),
            _ => {}
        },
        Err(err) => println!("There was an error: {}", err),
    };
}

fn out_metric() {
    let options = vec!["Kilometer(km)", "Meter(m)", "Decimeter(dm)", "Centimeter(cm)", "Milimeter(mm)", "Micrometer(μm)", "Nanometer(hm)", "Back"];
    let answer = Select::new("Select option:", options)
        .prompt();

    let mut out_length:f64 = 0.0;

    match answer {
        Ok(choice) => {
            if choice == "Back" {
                length();
                return;
            }

            match choice {
                "Kilometer(km)" => {
                    unsafe { out_length = MID_LENGHT / 1000.0; };
                    println!("{}km", out_length);
                    aftermenu();
                },
                "Meter(m)" => {
                    unsafe { out_length = MID_LENGHT };
                    println!("{}m", out_length);
                    aftermenu();
                },
                "Decimeter(dm)" => {
                    unsafe { out_length = MID_LENGHT * 10.0 };
                    println!("{}dm", out_length);
                    aftermenu();
                },
                "Centimeter(cm)" => {
                    unsafe { out_length = MID_LENGHT * 100.0 };
                    println!("{}cm", out_length);
                    aftermenu();
                },
                "Milimeter(mm)" => {
                    unsafe { out_length = MID_LENGHT * 1000.0 };
                    println!("{}mm", out_length);
                    aftermenu();
                },
                "Micrometer(μm)" => {
                    unsafe { out_length = MID_LENGHT * 1000000.0};
                    println!("{}μm", out_length);
                    aftermenu();
                },
                "Nanometer(hm)" => {
                    unsafe { out_length = MID_LENGHT * 1000000000.0 };
                    println!("{}hm", out_length);
                    aftermenu();
                    },
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
    }
}

fn out_imperial() {
    let options = vec!["Nautic Mile(mni)", "Mile(mi)", "Yard(jd)", "Foot(ft)", "Inch(in)", "Back"];
    let answer = Select::new("Select option:", options)
        .prompt();

    let mut out_length:f64 = 0.0;

    match answer {
        Ok(choice) => {
            if choice == "Back" {
                length();
                return;
            }

            match choice {
                "Nautic Mile(mni)" => {
                    unsafe { out_length = MID_LENGHT / 1852.0; };
                    println!("{}mni", out_length);
                    aftermenu();
                },
                "Mile(mi)" => {
                    unsafe { out_length = MID_LENGHT / 1609.344 };
                    println!("{}mi", out_length);
                    aftermenu();
                },
                "Yard(jd)" => {
                    unsafe { out_length = MID_LENGHT / 0.9144 };
                    println!("{}jd", out_length);
                    aftermenu();
                },
                "Foot(ft)" => {
                    unsafe { out_length = MID_LENGHT / 0.3048 };
                    println!("{}ft", out_length);
                    aftermenu();
                },
                "Inch(in)" => {
                    unsafe { out_length = MID_LENGHT / 0.0254 };
                    println!("{}in", out_length);
                    aftermenu();
                },
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
    }
}

fn out_space() {
    let options = vec!["Lunar Distance(ld)", "Astronomical Unit(au)", "Light Year(ly)", "Back"];
    let answer = Select::new("Select option:", options)
        .prompt();

    let mut out_length:f64 = 0.0;

    match answer {
        Ok(choice) => {
            if choice == "Back" {
                length();
                return;
            }

            match choice {
                "Lunar Distance(ld)" => {
                    unsafe { out_length = MID_LENGHT / 384400000.0; };
                    println!("{}ld", out_length);
                    aftermenu();
                },
                "Astronomical Unit(au)" => {
                    unsafe { out_length = MID_LENGHT / 149597870700.0 };
                    println!("{}au", out_length);
                    aftermenu();
                },
                "Light Year(ly)" => {
                    unsafe { out_length = MID_LENGHT / 9460700000000000.0 };
                    println!("{}ly", out_length);
                    aftermenu();
                },
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
    }
}

fn aftermenu() {
    let options = vec!["Continue", "Back", "Exit"];

    let answer = Select::new("", options)
        .prompt();
    
    match answer {
        Ok(choice) => match choice {
            "Continue" => length(),
            "Back" => crate::convert_select(),
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
} */

pub fn length_logic(input: f64, input_type: &str, output_type: &str) -> f64 {
    static mut mid_length: f64 = 0.0; // Mid length is an intermediary variable to make conversion easier and it is based on the meter.

    //if 

    let mut output: f64 = 0.0;

    output
}

fn dialog() {

}