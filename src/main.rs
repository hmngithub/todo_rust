use todo::{
    infrastructure::{IDGeneratorImpl, TodoRepoInMem},
    presentation::cli::CommandControll,
};

fn main() {
    let repo = Box::new(TodoRepoInMem::new());
    let id_gen = Box::new(IDGeneratorImpl::new());
    let command = CommandControll::new(repo, id_gen);
    command.execute();
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
