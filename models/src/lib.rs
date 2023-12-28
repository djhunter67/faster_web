use std::fmt::{self, Display, Formatter};

pub struct Person {
    pub name: String,
    pub age: u8,
    pub email: String,
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}, Age: {}", self.name, self.age)
    }
}

pub struct History {
    pub title: &'static str,
    pub history: Vec<String>,
}

impl Display for History {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Title: {}, History: {:?}", self.title, self.history[0])
    }
}

pub struct FastingInfo {
    pub notes: String,
    pub weight_time: String,
    pub weight: f32,
    pub time_since_last_meal: String,
}
