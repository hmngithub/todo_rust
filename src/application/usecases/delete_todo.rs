use crate::application::{dto::DeleteTodoReq, gateways::TodoRepository};

use super::TodoUsecase;

pub struct DeleteTodo {
    repo: Box<dyn TodoRepository>,
}

impl DeleteTodo {
    pub fn new(repo: Box<dyn TodoRepository>) -> Self {
        Self { repo }
    }
}

impl TodoUsecase<DeleteTodoReq, ()> for DeleteTodo {
    fn execute(&mut self, request: DeleteTodoReq) -> () {
        self.repo.delete_todo(request.id);
    }
}
