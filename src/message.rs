use std::sync::Mutex;

use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{Id, MessageList};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    text: String,
}

#[derive(Debug, Serialize)]
pub struct Messages<'a> {
    messages: &'a Vec<Message>,
}

#[derive(Debug, Deserialize)]
pub struct Room {
    id: Id,
}

/// # Example
///
///```ts
/// let post = { message: 'text' }
/// return ...
/// ```
pub async fn send_message(
    message_list: Data<Mutex<MessageList>>,
    Json(message): Json<Message>,
) -> impl Responder {
    let mut list = message_list.lock().unwrap();

    if let Some(vec) = list.0.get_mut(&0) {
        vec.push(message);
        println!("{:#?}", vec);
    }

    HttpResponse::Ok()
}

/// # Example
///```ts
/// let post = { id: ID }
/// return { messages: [message...] }
/// ```
pub async fn get_message(
    message_list: Data<Mutex<MessageList>>,
    Json(room): Json<Room>,
) -> impl Responder {
    let list = message_list.lock().unwrap();

    if let Some(vec) = list.0.get(&room.id) {
        info!("get: {:#?}", vec);
        return HttpResponse::Ok().json(Messages { messages: vec });
    }
    panic!()
}
