use crate::schema::messages;

use chrono;

#[derive(Insertable, Debug, PartialEq)]
#[table_name = "messages"]
pub struct NewMessage {
    user_id: i32,
    channel_id: i32,
    discord_id: String,
    content: String,
}

impl NewMessage {
    pub fn new(content: &str, discord_id: &str, channel_id: i32, user_id: i32) -> NewMessage {
        NewMessage {
            channel_id,
            user_id,
            discord_id: String::from(discord_id),
            content: String::from(content),
        }
    }
}

#[derive(Identifiable, Queryable, Associations, Debug, PartialEq)]
#[table_name = "messages"]
pub struct Message {
    pub user_id: i32,
    pub channel_id: i32,
    pub id: i32,
    pub discord_id: String,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
}
