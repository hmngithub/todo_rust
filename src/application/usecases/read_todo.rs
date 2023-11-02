use crate::{
    application::{dto::ReadTodoReq, gateways::TodoRepository},
    domain::Todo,
};

use super::TodoUsecase;

pub struct ReadTodo {
    repo: Box<dyn TodoRepository>,
}

impl ReadTodo {
    pub fn new(repo: Box<dyn TodoRepository>) -> Self {
        Self { repo }
    }
}

impl TodoUsecase<ReadTodoReq, Option<Todo>> for ReadTodo {
    fn execute(&mut self, request: ReadTodoReq) -> Option<Todo> {
        self.repo.read_todo(request.id)
    }
}
