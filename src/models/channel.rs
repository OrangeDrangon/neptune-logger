use crate::schema::channels;

use chrono;

#[derive(Insertable, Debug, PartialEq)]
#[table_name = "channels"]
pub struct NewChannel {
    discord_id: String,
    name: Option<String>,
}

impl NewChannel {
    pub fn new(discord_id: &str, name: Option<String>) -> NewChannel {
        NewChannel {
            discord_id: String::from(discord_id),
            name,
        }
    }
}

#[derive(Queryable, Identifiable, Associations, Debug, PartialEq)]
#[table_name = "channels"]
pub struct Channel {
    pub id: i32,
    pub discord_id: String,
    pub name: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}
