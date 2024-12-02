use super::*;

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
