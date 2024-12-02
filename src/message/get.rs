use super::*;

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
