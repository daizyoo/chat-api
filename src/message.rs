use std::sync::Mutex;

use actix_web::{
    web::{Data, Json},
    Responder,
};
use serde::{Deserialize, Serialize};
use tracing::info;
use type_utilities_rs::omit;

use crate::{types::Response, user::LoginInfo, DataList, MessageList, RoomId, RoomList, UserList};

#[omit(Message, [room])]
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageInfo {
    room: RoomId,
    text: String,
    user: LoginInfo,
}

pub async fn send_message(
    message_list: Data<Mutex<MessageList>>,
    Json(message): Json<MessageInfo>,
) -> impl Responder {
    let mut list = message_list.lock().unwrap();

    if let Some(vec) = list.0.get_mut(&message.room) {
        vec.push(Message {
            text: message.text,
            user: message.user,
        });
        info!("{:#?}", vec);
        return Response::ok("send message");
    }

    Response::error(format!("not get message_list room_id: {}", message.room))
}

#[derive(Debug, Deserialize)]
pub struct GetMessages {
    room_id: RoomId,
    user: LoginInfo,
}

/// # Example
///```ts
/// req -> { id: ID = room id }
/// res -> { data: [...] }
/// ```
pub async fn get_message(
    message_list: Data<Mutex<MessageList>>,
    user_list: Data<Mutex<UserList>>,
    room_list: Data<Mutex<RoomList>>,
    Json(get): Json<GetMessages>,
) -> impl Responder {
    let rooms = room_list.lock().unwrap();
    if let Some(room) = rooms.find(&get.room_id) {
        let users = user_list.lock().unwrap();

        // room memberの中にリクエストしてきたユーザーがいるか
        if room
            .members()
            .iter()
            .find(|u| u.id() == get.user.id())
            .is_some()
        {
            // そのユーザーが存在するか
            if let Some(user) = users.find(get.user.id()) {
                // パスワードがあっていたらメッセージを返す
                if user.password() == get.user.password() {
                    let list = message_list.lock().unwrap();
                    if let Some(vec) = list.0.get(&room.id) {
                        info!("get: {:#?}", vec);
                        Response::ok(vec)
                    } else {
                        Response::error(format!("not found messages: {}", get.room_id))
                    }
                } else {
                    Response::error(format!("no match password: {}", get.user.password()))
                }
            } else {
                Response::error(format!("not found request user: {}", get.user.id()))
            }
        } else {
            Response::error(format!("are you room member?: {}", get.room_id))
        }
    } else {
        Response::error(format!("not found room id: {}", get.room_id))
    }
}
