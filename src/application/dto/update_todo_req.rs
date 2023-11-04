use crate::domain::enums::Color;

pub struct UpdateTodoReq {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub color: Option<Color>,
    pub is_completed: Option<bool>,
}
