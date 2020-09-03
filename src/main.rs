#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::r2d2;
use serenity::{async_trait, framework::StandardFramework, model::channel, prelude::*};
use std::env;

mod models;
mod schema;

struct Handler {
    pool: diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>,
}

impl Handler {
    pub fn new(
        pool: diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>,
    ) -> Handler {
        Handler { pool }
    }

    fn get_connection(&self) -> r2d2::PooledConnection<r2d2::ConnectionManager<PgConnection>> {
        self.pool.get().unwrap()
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: channel::Message) {
        let channel_name = msg
            .channel_id
            .name(context.cache)
            .await
            .map(|chan_name| format!("{}", chan_name));

        let channel_record = {
            use schema::channels::dsl::*;
            let conn = self.get_connection();
            match (channels
                .filter(discord_id.eq(format!("{}", msg.channel_id)))
                .first::<models::channel::Channel>(&conn))
            {
                Ok(record) => record,
                Err(_) => diesel::insert_into(channels)
                    .values(models::channel::NewChannel::new(
                        &format!("{}", msg.channel_id),
                        channel_name,
                    ))
                    .get_result::<models::channel::Channel>(&conn)
                    .unwrap(),
            }
        };

        println!("{:?}", channel_record);

        let user_record = {
            use schema::users::dsl::*;
            let conn = self.get_connection();
            match (users
                .filter(discord_id.eq(format!("{}", msg.author.id)))
                .first::<models::user::User>(&conn))
            {
                Ok(record) => record,
                Err(_) => diesel::insert_into(users)
                    .values(models::user::NewUser::new(&format!("{}", msg.author.id)))
                    .get_result::<models::user::User>(&conn)
                    .unwrap(),
            }
        };

        println!("{:?}", user_record);

        let identity_record = {
            use schema::identities::dsl::*;
            let conn = self.get_connection();
            let curr_discriminator = format!("{}", msg.author.discriminator);
            let curr_nickname = msg.member.map_or(None, |mem| mem.nick);
            match (identities
                .filter(name.eq(&msg.author.name))
                .filter(discriminator.eq(&curr_discriminator))
                .filter(nickname.eq(&curr_nickname))
                .first::<models::identity::Identity>(&conn))
            {
                Ok(record) => record,
                Err(_) => diesel::insert_into(identities)
                    .values(models::identity::NewIdentity::new(
                        user_record.id,
                        &msg.author.name,
                        &curr_discriminator,
                        curr_nickname,
                    ))
                    .get_result::<models::identity::Identity>(&conn)
                    .unwrap(),
            }
        };

        println!("{:?}", identity_record);

        let message_record = {
            use schema::messages::dsl::*;
            let connection = self.get_connection();
            let to_insert = models::message::NewMessage::new(
                &msg.content,
                &format!("{}", msg.id),
                channel_record.id,
                user_record.id,
            );
            diesel::insert_into(messages)
                .values(&to_insert)
                .get_result::<models::message::Message>(&connection)
                .expect("Error saving message")
        };

        println!("{:?}", message_record);

        let attachment_records = if (msg.attachments.len() > 0) {
            use schema::attachments;
            let connection = self.get_connection();
            let mut to_insert: Vec<models::attachment::NewAttachment> = vec![];
            for attatchment in msg.attachments {
                let binary_data = attatchment.download().await.unwrap();
                to_insert.push(models::attachment::NewAttachment::new(
                    message_record.id,
                    &attatchment.filename,
                    binary_data,
                ))
            }
            diesel::insert_into(attachments::table)
                .values(&to_insert)
                .get_results::<models::attachment::Attachment>(&connection)
                .unwrap()
        } else {
            vec![]
        };
        println!("{:?}", attachment_records);
    }
}

#[tokio::main]
async fn main() {
    let manager = r2d2::ConnectionManager::<PgConnection>::new(
        env::var("DATABASE_URL").expect("Please provide a DATABASE_URL enviroment variable"),
    );
    let pool = r2d2::Builder::new()
        .max_size(15)
        .build(manager)
        .expect("Error initializing connection pool");

    let handler = Handler::new(pool);

    let framework = StandardFramework::new().configure(|c| c.prefix("~")); // set the bot's prefix to "~"

    // Login with a bot token from the environment
    let token =
        env::var("DISCORD_TOKEN").expect("Please provide a DISCORD_TOKEN enviroment variable");
    let mut client = Client::new(token)
        .event_handler(handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    println!("Starting bot...");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
