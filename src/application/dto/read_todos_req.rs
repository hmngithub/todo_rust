use crate::domain::enums::Color;

pub struct ReadTodosReq {
    pub all: Option<bool>,
    pub color: Option<Color>,
    pub completed: Option<bool>,
}
