use crate::{application::dto::ReadTodosReq, domain::Todo};

pub trait TodoRepository {
    fn create_todo(&mut self, todo: Todo);
    fn delete_todo(&mut self, id: String);
    fn update_todo(&mut self, todo: Todo);
    fn read_todo(&mut self, id: String) -> Option<Todo>;
    fn read_todos(&mut self, req: ReadTodosReq) -> Vec<Todo>;
}
