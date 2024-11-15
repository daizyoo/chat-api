mod message;

use std::{collections::HashMap, sync::Mutex};

use actix_web::{
    web::{post, scope, Data},
    App, HttpServer,
};
use message::Message;
use tracing_subscriber::EnvFilter;

type Id = u32;

struct MessageList(HashMap<Id, Vec<Message>>);

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let messages = Data::new(Mutex::new(MessageList(HashMap::from([(0, Vec::new())]))));

    HttpServer::new(move || {
        App::new()
            .service(scope("/message").route("/send", post().to(message::send_message)))
            .app_data(messages.clone())
    })
    .bind(("127.0.0.1", 3478))?
    .run()
    .await?;

    Ok(())
}
