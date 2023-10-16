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
    pub message: Option<Message>,
    // pub edited_message: Option<Message>,
    // pub channel_post: Option<Message>,
    // pub edited_channel_post: Option<Message>,
    // pub inline_query: Option<InlineQuery>,
    // pub chosen_inline_result: Option<ChosenInlineResult>,
    // pub callback_query: Option<CallbackQuery>,
    // pub shipping_query: Option<ShippingQuery>,
    // pub pre_checkout_query: Option<PreCheckoutQuery>,
    // pub poll: Option<Poll>,
    // pub poll_answer: Option<PollAnswer>,
    // pub my_chat_member: Option<ChatMemberUpdated>,
    // pub chat_member: Option<ChatMemberUpdated>,
    // pub chat_join_request: Option<ChatJoinRequest>,
}


impl Update {
    pub fn get_effective_user(&self) -> &User {
        self.message.as_ref().unwrap().from.as_ref().unwrap()
    }

    pub fn get_effective_message(&self) -> &Message {
        self.message.as_ref().unwrap()
    }

    pub fn get_effective_text(&self) -> &String {
        self.message.as_ref().unwrap().text.as_ref().unwrap()
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message_id: i64,
    pub message_thread_id: Option<i64>,
    pub from: Option<User>,
    // pub sender_chat: Option<Box<Chat>>,
    // pub date: i64,
    // pub chat: Box<Chat>,
    // pub forward_from: Option<User>,
    // pub forward_from_chat: Option<Box<Chat>>,
    // pub forward_from_message_id: Option<i64>,
    // pub forward_signature: Option<String>,
    // pub forward_sender_name: Option<String>,
    // pub forward_date: Option<i64>,
    // pub is_topic_message: Option<bool>,
    // pub is_automatic_forward: Option<bool>,
    // pub reply_to_message: Option<Box<Message>>,
    // pub via_bot: Option<User>,
    // pub edit_date: Option<i64>,
    // pub has_protected_content: Option<bool>,
    // pub media_group_id: Option<String>,
    // pub author_signature: Option<String>,
    pub text: Option<String>,
    // pub entities: Option<Vec<MessageEntity>>,
    // pub animation: Option<Animation>,
    // pub audio: Option<Audio>,
    // pub document: Option<Document>,
    // pub photo: Option<Vec<PhotoSize>>,
    // pub sticker: Option<Sticker>,
    // pub story: Option<Story>,
    // pub video: Option<Video>,
    // pub video_note: Option<VideoNote>,
    // pub voice: Option<Voice>,
    // pub caption: Option<String>,
    // pub caption_entities: Option<Vec<MessageEntity>>,
    // pub has_media_spoiler: Option<bool>,
    // pub contact: Option<Contact>,
    // pub dice: Option<Dice>,
    // pub game: Option<Game>,
    // pub poll: Option<Poll>,
    // pub venue: Option<Venue>,
    // pub location: Option<Location>,
    // pub new_chat_members: Option<Vec<User>>,
    // pub left_chat_member: Option<User>,
    // pub new_chat_title: Option<String>,
    // pub new_chat_photo: Option<Vec<PhotoSize>>,
    // pub delete_chat_photo: Option<bool>,
    // pub group_chat_created: Option<bool>,
    // pub supergroup_chat_created: Option<bool>,
    // pub channel_chat_created: Option<bool>,
    // pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
    // pub migrate_to_chat_id: Option<i64>,
    // pub migrate_from_chat_id: Option<i64>,
    // pub pinned_message: Option<Box<Message>>,
    // pub invoice: Option<Invoice>,
    // pub successful_payment: Option<SuccessfulPayment>,
    // pub user_shared: Option<UserShared>,
    // pub chat_shared: Option<ChatShared>,
    // pub connected_website: Option<String>,
    // pub write_access_allowed: Option<WriteAccessAllowed>,
    // pub passport_data: Option<PassportData>,
    // pub proximity_alert_triggered: Option<ProximityAlertTriggered>,
    // pub forum_topic_created: Option<ForumTopicCreated>,
    // pub forum_topic_edited: Option<ForumTopicEdited>,
    // pub forum_topic_closed: Option<ForumTopicClosed>,
    // pub forum_topic_reopened: Option<ForumTopicReopened>,
    // pub general_forum_topic_hidden: Option<GeneralForumTopicHidden>,
    // pub general_forum_topic_unhidden: Option<GeneralForumTopicUnhidden>,
    // pub video_chat_scheduled: Option<VideoChatScheduled>,
    // pub video_chat_started: Option<VideoChatStarted>,
    // pub video_chat_ended: Option<VideoChatEnded>,
    // pub video_chat_participants_invited: Option<VideoChatParticipantsInvited>,
    // pub web_app_data: Option<WebAppData>,
    // pub reply_markup: Option<InlineKeyboardMarkup>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,
    pub query: String,
    pub offset: String,
    pub chat_type: Option<String>,
    pub location: Option<Location>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,
    pub location: Option<Location>,
    pub inline_message_id: Option<String>,
    pub query: String 
}


#[derive(Serialize, Deserialize, Debug)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ShippingQuery {
    pub id: String,
    pub from: User,
    pub invoce_payload: String,
    pub shipping_address: ShippingAddress
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PreCheckoutQuery {
    pub id: String,
    pub from: User,
    pub currency: String,
    pub total_amount: i64,
    pub invoce_payload: Option<String>,
    pub order_info: Option<OrderInfo>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Poll {
    pub id: String,
    pub question: String,
    pub options: Vec<PollOption>,
    pub total_voter_count: i64,
    pub is_closed: bool,
    pub is_anonymous: bool,
    #[serde(rename = "type")]
    pub _type: String,
    pub allows_multiple_answers: bool,
    pub correct_option_id: Option<i64>,
    pub explanation: Option<String>,
    pub explanation_entities: Vec<MessageEntity>,
    pub open_period: Option<i64>,
    pub close_date: Option<i64>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PollAnswer {
    pub poll_id: String,
    pub voter_chat: Option<Chat>,
    pub user: Option<User>,
    pub option_ids: Vec<i64>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMemberUpdated {
    pub chat: Chat,
    pub from: User,
    pub date: i64,
    pub old_chat_member: ChatMember,
    pub new_chat_member: ChatMember,
    pub invite_link: Option<ChatInviteLink>,
    pub via_chat_folder_invite_link: Option<bool>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ChatJoinRequest {
    pub chat: Chat,
    pub user: User,
    pub user_chat_id: i64,
    pub date: i64,
    pub bip: Option<String>,
    pub invite_link: Option<ChatInviteLink>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<i64>,
    pub heading: Option<i64>,
    pub proximity_alert_radius: Option<i64>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ShippingAddress {
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct OrderInfo {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<ShippingAddress>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PollOption {
    pub text: String,
    pub voter_count: i64
}


#[derive(Serialize, Deserialize, Debug)]
pub struct MessageEntity {
    #[serde(rename = "type")]
    pub _type: String,
    pub offset: i64,
    pub length: i64,
    pub url: Option<String>,
    pub user: Option<User>,
    pub language: Option<String>,
    pub custom_emoji_id: Option<String>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    pub id: i64,
    #[serde(rename = "type")]
    pub _type: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_forum: Option<bool>,
    pub photo: Option<ChatPhoto>,
    pub active_username: Option<Vec<String>>,
    pub emoji_status_custom_emoji_id: Option<String>,
    pub emoji_status_expiration_date: Option<i64>,
    pub bio: Option<String>,
    pub has_private_forwards: Option<bool>,
    pub has_restricted_voice_and_video_messages: Option<bool>,
    pub join_to_send_messages: Option<bool>,
    pub join_by_request: Option<bool>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub pinned_message: Option<Message>,
    pub permissions: Option<ChatPermissions>,
    pub slow_mode_delay: Option<i64>,
    pub message_auto_delete_time: Option<i64>,
    pub has_aggressive_anti_spam_enabled: Option<bool>,
    pub has_hidden_members: Option<bool>,
    pub has_protected_content: Option<bool>,
    pub sticker_set_name: Option<String>,
    pub can_set_sticker_set: Option<bool>,
    pub linked_chat_id: Option<i64>,
    pub location: Option<ChatLocation>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMember {}


#[derive(Serialize, Deserialize, Debug)]
pub struct ChatInviteLink {}


#[derive(Serialize, Deserialize, Debug)]
pub struct ChatPhoto {}


#[derive(Serialize, Deserialize, Debug)]
pub struct ChatPermissions {}


#[derive(Serialize, Deserialize, Debug)]
pub struct ChatLocation {}


#[derive(Serialize, Deserialize, Debug)]
pub struct Animation {}
#[derive(Serialize, Deserialize, Debug)]
pub struct Audio {}
#[derive(Serialize, Deserialize, Debug)]
pub struct Document {}
#[derive(Serialize, Deserialize, Debug)]
pub struct PhotoSize {}
#[derive(Serialize, Deserialize, Debug)]
pub struct Sticker {}
#[derive(Serialize, Deserialize, Debug)]
pub struct Story {}
#[derive(Serialize, Deserialize, Debug)]
pub struct Video {}
#[derive(Serialize, Deserialize, Debug)]
pub struct VideoNote {}
#[derive(Serialize, Deserialize, Debug)]
pub struct Voice {}
#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {}
#[derive(Serialize, Deserialize, Debug)]
pub struct Dice {}
#[derive(Serialize, Deserialize, Debug)]
pub struct Game {}
#[derive(Serialize, Deserialize, Debug)]
pub struct Venue {}
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageAutoDeleteTimerChanged {}
#[derive(Serialize, Deserialize, Debug)]
pub struct Invoice {}
#[derive(Serialize, Deserialize, Debug)]
pub struct SuccessfulPayment {}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserShared {}
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatShared {}
#[derive(Serialize, Deserialize, Debug)]
pub struct WriteAccessAllowed {}
#[derive(Serialize, Deserialize, Debug)]
pub struct PassportData {}
#[derive(Serialize, Deserialize, Debug)]
pub struct ProximityAlertTriggered {}
#[derive(Serialize, Deserialize, Debug)]
pub struct ForumTopicCreated {}
#[derive(Serialize, Deserialize, Debug)]
pub struct ForumTopicEdited {}
#[derive(Serialize, Deserialize, Debug)]
pub struct ForumTopicClosed {}
#[derive(Serialize, Deserialize, Debug)]
pub struct ForumTopicReopened {}
#[derive(Serialize, Deserialize, Debug)]
pub struct GeneralForumTopicHidden {}
#[derive(Serialize, Deserialize, Debug)]
pub struct GeneralForumTopicUnhidden {}
#[derive(Serialize, Deserialize, Debug)]
pub struct VideoChatScheduled {}
#[derive(Serialize, Deserialize, Debug)]
pub struct VideoChatStarted {}
#[derive(Serialize, Deserialize, Debug)]
pub struct VideoChatEnded {}
#[derive(Serialize, Deserialize, Debug)]
pub struct VideoChatParticipantsInvited {}
#[derive(Serialize, Deserialize, Debug)]
pub struct WebAppData {}
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineKeyboardMarkup {}