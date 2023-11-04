#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    pub fn from_str(value: String) -> Self {
        if value.to_lowercase() == "red".to_string() {
            return Color::Red;
        } else if value.to_lowercase() == "blue".to_string() {
            return Color::Blue;
        } else {
            return Color::Green;
        }
    }
}

impl Color {
    pub fn is_red(&self) -> bool {
        match self {
            Color::Red => true,
            _ => false,
        }
    }

    pub fn is_blue(&self) -> bool {
        match self {
            Color::Blue => true,
            _ => false,
        }
    }

    pub fn is_green(&self) -> bool {
        match self {
            Color::Green => true,
            _ => false,
        }
    }
}
