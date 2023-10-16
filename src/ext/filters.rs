use crate::telegram::types::Update;


pub trait Filter {
    fn check_update(&self, update: &Update) -> bool;
}


pub enum MessageFilter {
    Contains(String),
    Regexp(String)
}

impl Filter for MessageFilter {
    fn check_update(&self, update: &Update) -> bool {
        match self {
            Self::Contains(text) => update.message.as_ref().unwrap().text.as_ref().unwrap().contains(text),
            Self::Regexp(text) => true,
        }
    }
}


pub enum CallbackFilter {
    Contains(String),
    All
}

impl Filter for CallbackFilter {
    fn check_update(&self, update: &Update) -> bool {
        match self {
            Self::Contains(text) => true,
            Self::All => true
        }
    }
}


pub enum CommandFilter {
    Command(String),
}

impl Filter for CommandFilter {
    fn check_update(&self, update: &Update) -> bool {
        match self {
            Self::Command(text) => update.message.as_ref().unwrap().text.as_ref().unwrap().starts_with(text)
        }
    }
}