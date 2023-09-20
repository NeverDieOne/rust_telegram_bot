use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub is_premium: Option<bool>,
    pub added_to_attachment_menu: Option<bool>,
    pub can_join_groups: Option<bool>,
    pub can_read_all_group_messages: Option<bool>,
    pub supports_inline_queries: Option<bool>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Update {
    pub update_id: u64,
    pub message: Message
}


impl Update {
    pub fn get_effective_user(&self) -> &User {
        self.message.from.as_ref().unwrap()
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message_id: i64,
    pub text: String,
    pub date: i64,
    pub from: Option<User>,
}

