use crate::domain::enums::Color;

pub struct UpdateTodoReq {
    pub id: String,
    pub title: String,
    pub description: String,
    pub color: Option<Color>,
    pub is_completed: Option<bool>,
}
