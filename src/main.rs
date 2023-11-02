use todo::presentation::cli::PrintColor;

fn main() {
    //let id_gen = Box::new(IDGeneratorImpl::new());

    let mut tt = PrintColor::new();
    tt.print_with_color();

    // let arg: Arg = Arg::new("title").help("subject").index(1);
    // let option1 = Arg::new("option1").short('s').long("option1");
    // let option2 = Arg::new("option2").short('b').long("option2");
    // let sub01 = Command::new("subject").arg(arg).arg(option1).arg(option2);
    // let sub = Command::new("create")
    //     .subcommand(sub01)
    //     .about("salam farmanda");

    // let d = Command::new("todo")
    //     .about("my todo app")
    //     .subcommand(sub)
    //     .get_matches();

    // let sub_command_match = d.subcommand_matches("create").expect("adsbda");

    // let sub009 = sub_command_match
    //     .subcommand_matches("subject")
    //     .expect("sdf");
    // let value1 = sub009.get_one::<String>("option1").expect("msg");
    // let value2 = sub009.get_one::<String>("option2").expect("msg");
    //println!("{value1} {value2}")
}

// use todo::{
//     application::{
//         dto::CreateTodoReq,
//         usecases::{CreateTodo, TodoUsecase},
//     },
//     domain::enums::Color,
//     infrastructure::{IDGeneratorImpl, TodoRepoInMem},
// };

//use clap::CommandFactory;

//fn main() {
// let d = command();
// let repo = Box::new(TodoRepoInMem::new());
// let id_gen = Box::new(IDGeneratorImpl::new());
// let mut create_usecase = CreateTodo::new(repo, id_gen);
// let req = CreateTodoReq {
//     title: String::from("Practice Rust"),
//     description: String::from("Practice Rust Ownership"),
//     color: Some(Color::Red),
// };
// let id = create_usecase.execute(req);
// println!("Successfully created a new todo with id: {id}");
//}
