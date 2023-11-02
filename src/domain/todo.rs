use self::enums::Color;
pub mod enums;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Todo {
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub color: Option<Color>,
    pub is_completed: bool,
}

impl Todo {
    pub fn new() -> Self {
        Self {
            id: None,
            title: None,
            description: None,
            color: None,
            is_completed: false,
        }
    }
}

impl Todo {
    pub fn set_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn set_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn set_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn set_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn set_is_completed(mut self, is_completed: bool) -> Self {
        self.is_completed = is_completed;
        self
    }
}
