use std::sync::Mutex;

use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use tracing::info;

use crate::{
    types::{GetMessages, Message, MessageInfo, Response},
    DataList, MessageList, RoomList, UserList,
};

pub async fn send_message(
    message_list: Data<Mutex<MessageList>>,
    Json(message): Json<MessageInfo>,
) -> HttpResponse {
    let mut list = message_list.lock().unwrap();

    if let Some(vec) = list.0.get_mut(&message.id()) {
        vec.push(Message::from(message));
        info!("{:#?}", vec);
        return Response::ok("send message");
    }

    Response::error(format!("not get message_list room_id: {}", message.id()))
}

pub async fn get_message(
    message_list: Data<Mutex<MessageList>>,
    user_list: Data<Mutex<UserList>>,
    room_list: Data<Mutex<RoomList>>,
    Json(get): Json<GetMessages>,
) -> HttpResponse {
    let rooms = room_list.lock().unwrap();
    if let Some(room) = rooms.find(&get.room_id()) {
        let users = user_list.lock().unwrap();

        // room memberの中にリクエストしてきたユーザーがいるか
        if room
            .members()
            .iter()
            .find(|u| u.id() == get.user_id())
            .is_some()
        {
            // そのユーザーが存在するか
            if let Some(user) = users.find(get.user_id()) {
                // パスワードがあっていたらメッセージを返す
                if user.password() == get.user_password() {
                    let list = message_list.lock().unwrap();
                    if let Some(vec) = list.0.get(&room.id()) {
                        Response::ok(vec)
                    } else {
                        Response::error(format!("not found messages: {}", get.room_id()))
                    }
                } else {
                    Response::error(format!("no match password: {}", get.user_password()))
                }
            } else {
                Response::error(format!("not found request user: {}", get.user_id()))
            }
        } else {
            Response::error(format!("are you room member?: {}", get.room_id()))
        }
    } else {
        Response::error(format!("not found room id: {}", get.room_id()))
    }
}
