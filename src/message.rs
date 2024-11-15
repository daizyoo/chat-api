use std::sync::Mutex;

use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::Deserialize;

use crate::MessageList;

#[derive(Debug, Deserialize)]
pub struct Message {
    text: String,
}

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
