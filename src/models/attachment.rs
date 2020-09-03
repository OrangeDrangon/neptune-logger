use super::message::Message;
use crate::schema::attachments;

use chrono;
#[derive(Insertable, Debug, PartialEq)]
#[table_name = "attachments"]
pub struct NewAttachment {
    message_id: i32,
    filename: String,
    binary_data: Vec<u8>,
}

impl NewAttachment {
    pub fn new(message_id: i32, filename: &str, binary_data: Vec<u8>) -> NewAttachment {
        NewAttachment {
            message_id,
            filename: String::from(filename),
            binary_data,
        }
    }
}

#[derive(Identifiable, Queryable, Associations, Debug, PartialEq)]
#[belongs_to(Message)]
#[table_name = "attachments"]
pub struct Attachment {
    pub message_id: i32,
    pub id: i32,
    pub filename: String,
    pub binary_data: Vec<u8>,
    pub created_at: chrono::NaiveDateTime,
}
