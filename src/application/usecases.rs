mod create_todo;
mod delete_todo;
mod read_todo;
mod read_todos;
mod update_todo;
pub use create_todo::CreateTodo;
pub use delete_todo::DeleteTodo;
pub use read_todo::ReadTodo;
pub use read_todos::ReadTodos;
pub use update_todo::UpdateTodo;

pub trait TodoUsecase<Request, Response> {
    fn execute(&mut self, request: Request) -> Response;
}
