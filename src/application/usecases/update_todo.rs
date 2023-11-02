use crate::{
    application::{dto::UpdateTodoReq, gateways::TodoRepository},
    domain::{enums::Color, Todo},
};

use super::TodoUsecase;

pub struct UpdateTodo {
    repo: Box<dyn TodoRepository>,
}

impl UpdateTodo {
    pub fn new(repo: Box<dyn TodoRepository>) -> Self {
        Self { repo }
    }
}

impl TodoUsecase<UpdateTodoReq, ()> for UpdateTodo {
    fn execute(&mut self, request: UpdateTodoReq) -> () {
        let new_todo = Todo::new()
            .set_id(request.id)
            .set_title(request.title)
            .set_description(request.description)
            .set_color(request.color.unwrap_or(Color::Blue))
            .set_is_completed(false);

        self.repo.update_todo(new_todo)
    }
}
