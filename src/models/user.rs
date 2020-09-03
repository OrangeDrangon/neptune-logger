use crate::schema::users;

use chrono;

#[derive(Insertable, Debug, PartialEq)]
#[table_name = "users"]
pub struct NewUser {
    discord_id: String,
}

impl NewUser {
    pub fn new(discord_id: &str) -> NewUser {
        NewUser {
            discord_id: String::from(discord_id),
        }
    }
}

#[derive(Queryable, Identifiable, Debug, PartialEq)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub discord_id: String,
    pub created_at: chrono::NaiveDateTime,
}
