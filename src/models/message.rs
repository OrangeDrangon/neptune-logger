use crate::schema::messages;

use chrono;
use serenity::model::channel;

#[derive(Insertable, Debug, PartialEq)]
#[table_name = "messages"]
pub struct NewMessage {
    author: String,
    author_id: String,
    channel: String,
    channel_id: String,
    content: String,
}

impl NewMessage {
    pub fn new(msg: &channel::Message, channel_name: &str) -> NewMessage {
        NewMessage {
            author: format!("{}#{}", msg.author.name, msg.author.discriminator),
            author_id: format!("{}", msg.author.id),
            channel: String::from(channel_name),
            channel_id: format!("{}", msg.channel_id),
            content: msg.content.clone(),
        }
    }
}

#[derive(Identifiable, Queryable, Debug, PartialEq)]
#[table_name = "messages"]
pub struct Message {
    pub id: i32,
    pub author: String,
    pub author_id: String,
    pub channel: String,
    pub channel_id: String,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
}
