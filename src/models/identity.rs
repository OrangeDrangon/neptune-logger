use crate::schema::identities;

use chrono;

#[derive(Insertable, Debug, PartialEq)]
#[table_name = "identities"]
pub struct NewIdentity {
    user_id: i32,
    name: String,
    discriminator: String,
    nickname: Option<String>,
}

impl NewIdentity {
    pub fn new(user_id: i32, name: &str, discriminator: &str, nickname: Option<String>) -> NewIdentity {
        NewIdentity {
            user_id,
            name: String::from(name),
            discriminator: String::from(discriminator),
            nickname,
        }
    }
}

#[derive(Queryable, Identifiable, Associations, Debug, PartialEq)]
#[table_name = "identities"]
pub struct Identity {
    pub user_id: i32,
    pub id: i32,
    pub name: String,
    pub discriminator: String,
    pub nickname: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}