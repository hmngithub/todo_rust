use crate::{
    application::{dto::ReadTodosReq, gateways::TodoRepository},
    domain::Todo,
};

use super::TodoUsecase;

pub struct ReadTodos {
    repo: Box<dyn TodoRepository>,
}

impl ReadTodos {
    pub fn new(repo: Box<dyn TodoRepository>) -> Self {
        Self { repo }
    }
}

impl TodoUsecase<ReadTodosReq, Vec<Todo>> for ReadTodos {
    fn execute(&mut self, request: ReadTodosReq) -> Vec<Todo> {
        self.repo.read_todos(request)
    }
}
