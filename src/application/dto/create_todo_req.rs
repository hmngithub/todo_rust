use crate::domain::enums::Color;

pub struct CreateTodoReq {
    pub title: String,
    pub description: String,
    pub color: Option<Color>,
}
