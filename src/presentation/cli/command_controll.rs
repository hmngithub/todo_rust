use clap::{Arg, Command};

use crate::{
    application::{
        dto::{CreateTodoReq, DeleteTodoReq, ReadTodosReq, UpdateTodoReq},
        gateways::{IdGenerator, TodoRepository},
        usecases::{CreateTodo, DeleteTodo, ReadTodos, TodoUsecase, UpdateTodo},
    },
    domain::enums::Color,
};

use super::PrintDataColor;

pub struct CommandControll {
    repo: Box<dyn TodoRepository>,
    id_gen: Box<dyn IdGenerator>,
}

impl CommandControll {
    pub fn new(repo: Box<dyn TodoRepository>, id_gen: Box<dyn IdGenerator>) -> Self {
        Self { repo, id_gen }
    }
}

impl CommandControll {
    pub fn execute(self) {
        let create_title = Arg::new("ctitle").short('t').long("title").required(true);
        let create_description = Arg::new("cdescription")
            .short('d')
            .long("description")
            .required(true);
        let update_title = Arg::new("utitle").short('t').long("title");
        let update_description = Arg::new("udescription").short('d').long("description");
        let id = Arg::new("id").short('i').long("id").required(true);
        let color = Arg::new("color").short('c').long("color");
        let is_complete = Arg::new("is_complete").short('k').long("complete");

        let create_sub = Command::new("create")
            .override_usage("todo create [option]")
            .about("you can create todo")
            .arg(create_title)
            .arg(create_description)
            .arg(color.clone())
            .arg_required_else_help(true);

        let delete_sub = Command::new("delete")
            .about("you can delete todo")
            .arg(id.clone())
            .arg_required_else_help(true);
        let update_sub = Command::new("update")
            .about("you can update todo")
            .arg(id)
            .arg(update_title.clone())
            .arg(update_description.clone())
            .arg(color.clone())
            .arg(is_complete.clone())
            .arg_required_else_help(true);
        let all = Arg::new("all").short('a').long("all");

        let show_sub = Command::new("show")
            .about("you can show todo")
            .arg(all)
            .arg(color)
            .arg(is_complete)
            .arg_required_else_help(true);

        let command = Command::new("todo")
            .override_usage("todo [COMMAND]")
            .version("v1.0.0")
            .about("todo is a cli app   todo ")
            .subcommand(create_sub)
            .subcommand(delete_sub)
            .subcommand(update_sub)
            .subcommand(show_sub)
            .author("Homayoun mohammadi")
            .arg_required_else_help(true)
            .disable_help_subcommand(true)
            .get_matches();

        if command.subcommand_matches("create").is_some() {
            let crc = command.subcommand_matches("create").unwrap();
            let tit = crc.get_one::<String>("title").unwrap();
            let des = crc.get_one::<String>("description").unwrap();
            let col: Option<Color> = match crc.get_one::<String>("description") {
                Some(vl) => Some(Color::from_str(vl.to_string())),
                None => None,
            };
            let request = CreateTodoReq {
                title: tit.to_string(),
                description: des.to_string(),
                color: col,
            };
            let mut usecases = CreateTodo::new(self.repo, self.id_gen);
            let todo_id = usecases.execute(request);
            println!("todo created id : {}", todo_id);
        } else if command.subcommand_matches("update").is_some() {
            let crc = command.subcommand_matches("update").unwrap();
            let tit = crc.get_one::<String>("utitle").to_owned();
            let des = crc.get_one::<String>("udescription");
            let col: Option<Color> = match crc.get_one::<String>("color") {
                Some(vl) => Some(Color::from_str(vl.to_string())),
                None => None,
            };
            let i = crc.get_one::<String>("id").unwrap();
            let complet: Option<bool> = match crc.get_one::<String>("is_complete") {
                Some(vl) => {
                    if vl == "true" {
                        Some(true)
                    } else {
                        Some(false)
                    }
                }
                None => None,
            };
            let request = UpdateTodoReq {
                id: i.to_string(),
                title: tit.cloned(),
                description: des.cloned(),
                color: col,
                is_completed: complet,
            };
            let mut usecases = UpdateTodo::new(self.repo);
            usecases.execute(request);
            println!("todo updated");
        } else if command.subcommand_matches("delete").is_some() {
            let crc = command.subcommand_matches("delete").unwrap();
            let mut delete_todo = DeleteTodo::new(self.repo);
            let request = DeleteTodoReq {
                id: crc.get_one::<String>("id").unwrap().to_string(),
            };
            delete_todo.execute(request);
            println!("todo deleted");
        } else if command.subcommand_matches("show").is_some() {
            let crc = command.subcommand_matches("show").unwrap();

            let a: Option<bool> = match crc.get_one::<String>("all") {
                Some(vl) => {
                    if vl == "true" {
                        Some(true)
                    } else {
                        None
                    }
                }
                None => None,
            };
            let c = match crc.get_one::<String>("color") {
                Some(vl) => Some(Color::from_str(vl.to_string())),
                None => None,
            };
            let k = match crc.get_one::<String>("is_complete") {
                Some(vl) => {
                    if vl == "true" {
                        Some(true)
                    } else {
                        Some(false)
                    }
                }
                None => None,
            };

            let mut usecase = ReadTodos::new(self.repo);

            let request = ReadTodosReq {
                all: a,
                color: c,
                completed: k,
            };
            let result = usecase.execute(request);
            if result.is_empty() {
                println!("no todo find");
                return;
            }
            PrintDataColor::print_with_color(result);
        }
    }
}
