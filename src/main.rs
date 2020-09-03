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
        let channel_name = match (msg.channel_id.name(context.cache).await) {
            Some(name) => name,
            None => {
                println!("Error getting channel name");
                return;
            }
        };
        let message_record = {
            use schema::messages;
            let connection = self.get_connection();
            let to_insert = models::message::NewMessage::new(&msg, &channel_name);
            diesel::insert_into(messages::table)
                .values(&to_insert)
                .get_result::<models::message::Message>(&connection)
                .expect("Error saving message")
        };
        println!("{:?}", message_record);

        let attachment_records = {
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
