use crate::{
    application::{dto::UpdateTodoReq, gateways::TodoRepository},
    domain::Todo,
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
        let new_todo = Todo {
            id: Some(request.id),
            title: request.title,
            description: request.description,
            color: request.color,
            is_completed: request.is_completed,
        };

        self.repo.update_todo(new_todo)
    }
}
