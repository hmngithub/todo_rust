use colored::{self, Colorize};

use crate::domain::Todo;

pub struct PrintDataColor;

impl PrintDataColor {
    pub fn new() -> Self {
        Self
    }
}

impl PrintDataColor {
    pub fn print_with_color(todos: Vec<Todo>) {
        for i in todos {
            if i.clone().color.unwrap().is_red() {
                let d = "----------------------------------------".bright_red();
                println!("{}", d);
                println!("{} : {}", "id".bright_red(), i.id.unwrap().bright_red());
                println!(
                    "{} : {}",
                    "title".bright_red(),
                    i.title.unwrap().bright_red()
                );
                println!(
                    "{} : {}",
                    "description".bright_red(),
                    i.description.unwrap().bright_red()
                );
                println!("{} : {}", "color".bright_red(), "red".bright_red());
                println!(
                    "\x1B[41m {} : {} \x1B[0m",
                    "is_complete",
                    i.is_completed.unwrap().to_string()
                );

                println!();
                continue;
            } else if i.clone().color.unwrap().is_blue() {
                let d = "----------------------------------------".bright_blue();
                println!("{}", d);
                println!("{} : {}", "id".bright_blue(), i.id.unwrap().bright_blue());
                println!(
                    "{} : {}",
                    "title".bright_blue(),
                    i.title.unwrap().bright_blue()
                );
                println!(
                    "{} : {}",
                    "description".bright_blue(),
                    i.description.unwrap().bright_blue()
                );
                println!("{} : {}", "color".bright_blue(), "blue".bright_blue());
                println!(
                    "\x1B[44m {} : {} \x1B[0m ",
                    "is_complete",
                    i.is_completed.unwrap().to_string()
                );
                println!();
                continue;
            } else if i.clone().color.unwrap().is_green() {
                let d = "----------------------------------------".bright_green();
                println!("{}", d);
                println!("{} : {}", "id".bright_green(), i.id.unwrap().bright_green());
                println!(
                    "{} : {}",
                    "title".bright_green(),
                    i.title.unwrap().bright_green()
                );
                println!(
                    "{} : {}",
                    "description".bright_green(),
                    i.description.unwrap().bright_green()
                );
                println!("{} : {}", "color".bright_green(), "Greens".bright_green());
                println!(
                    "\x1B[42m {} : {} \x1B[0m",
                    "is_complete",
                    i.is_completed.unwrap().to_string()
                );
                println!();
                continue;
            } else {
                let d = "----------------------------------------";
                println!("{}", d);
                println!("id : {}", i.id.unwrap());
                println!("{} : {}", "title", i.title.unwrap());
                println!("{} : {}", "description", i.description.unwrap());
                println!("{} : {}", "color", "red");
                println!(
                    "\x1B[43m {} : {}",
                    "is_complete",
                    i.is_completed.unwrap().to_string()
                );
                println!();
            }
        }
    }
}
