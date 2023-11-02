use crate::{
    application::{
        dto::ReadTodosReq,
        usecases::{ReadTodos, TodoUsecase},
    },
    domain::enums::Color,
    infrastructure::TodoRepoInMem,
};
use colored::{self, Colorize};

pub struct PrintColor;

impl PrintColor {
    pub fn new() -> Self {
        Self
    }
}

impl PrintColor {
    pub fn print_with_color(&mut self) {
        let repo = Box::new(TodoRepoInMem::new());
        let mut usecase = ReadTodos::new(repo);

        let request = ReadTodosReq {
            all: None,
            color: Some(Color::Red),
            completed: Some(true),
        };

        for i in usecase.execute(request) {
            if i.clone().color.unwrap().is_red() {
                let d = "----------------------------------------".bright_red();
                println!("{}", d);
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
                    i.is_completed.to_string()
                );

                println!();
                continue;
            } else if i.clone().color.unwrap().is_blue() {
                let d = "----------------------------------------".bright_blue();
                println!("{}", d);
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
                    i.is_completed.to_string()
                );
                println!();
                continue;
            } else if i.clone().color.unwrap().is_green() {
                let d = "----------------------------------------".bright_green();
                println!("{}", d);
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
                    i.is_completed.to_string()
                );
                println!();
                continue;
            } else {
                let d = "----------------------------------------";
                println!("{}", d);
                println!("{} : {}", "title", i.title.unwrap());
                println!("{} : {}", "description", i.description.unwrap());
                println!("{} : {}", "color", "red");
                println!(
                    "\x1B[43m {} : {}",
                    "is_complete",
                    i.is_completed.to_string()
                );
                println!();
            }
        }
    }
}
