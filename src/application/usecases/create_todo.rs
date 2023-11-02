use crate::{
    application::{
        dto::CreateTodoReq,
        gateways::{IdGenerator, TodoRepository},
    },
    domain::{enums::Color, Todo},
};

use super::TodoUsecase;

pub struct CreateTodo {
    repo: Box<dyn TodoRepository>,
    id_gen: Box<dyn IdGenerator>,
}

impl CreateTodo {
    pub fn new(repo: Box<dyn TodoRepository>, id_gen: Box<dyn IdGenerator>) -> Self {
        Self { repo, id_gen }
    }
}

impl TodoUsecase<CreateTodoReq, String> for CreateTodo {
    fn execute(&mut self, request: CreateTodoReq) -> String {
        let new_id = self.id_gen.generate_id();

        let new_todo = Todo::new()
            .set_id(new_id.clone())
            .set_title(request.title)
            .set_description(request.description)
            .set_color(request.color.unwrap_or(Color::Blue))
            .set_is_completed(false);

        self.repo.create_todo(new_todo);
        new_id
    }
}
