use std::sync::Mutex;

use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use tracing::info;

use crate::{
    types::{CreateRoom, LoginInfo, Response, UserInfo},
    DataList, FriendList, MessageList, Room, RoomList, UserList,
};

pub async fn create(
    friend_list: Data<Mutex<FriendList>>,
    user_list: Data<Mutex<UserList>>,
    message_list: Data<Mutex<MessageList>>,
    room_list: Data<Mutex<RoomList>>,
    Json(new_room): Json<CreateRoom>,
) -> HttpResponse {
    {
        // メンバーがフレンドかどうか
        let friends = friend_list.lock().unwrap();
        if let Some(friends) = friends.0.get(new_room.user_id()) {
            if !new_room
                .members()
                .iter()
                .filter(|&m| new_room.user_id() != m)
                .all(|m| friends.iter().any(|f| m == f.id()))
            {
                return Response::error(format!(
                    "フレンドではないユーザーが含まれています: {:?}",
                    new_room.members()
                ));
            }
        }
    }
    let members: Vec<UserInfo>;
    {
        let users = user_list.lock().unwrap();
        let Some(user) = users.find(new_room.user_id()) else {
            return Response::error("not found create request user");
        };
        if new_room.user_password() != user.password() {
            return Response::error("ユーザーのパスワードが間違っているのでRoomを作成できません");
        }

        // memberが全て存在するuserか
        if !new_room.members().iter().all(|id| users.exist(id)) {
            return Response::error(format!("存在しないユーザー: {:?}", new_room.members()));
        }
        members = new_room
            .members()
            .iter()
            .map(|id| UserInfo::from(users.find(id).unwrap()))
            .collect();
    }
    let mut rooms = room_list.lock().unwrap();
    // roomの存在確認
    if rooms
        .iter()
        .find(|room| {
            new_room
                .members()
                .iter()
                .all(|user_id| room.members().iter().find(|u| u.id() == user_id).is_some())
        })
        .is_some()
    {
        return Response::error("すでに存在するRoomです");
    }

    let mut messages = message_list.lock().unwrap();

    // 新しいidを作成
    let id = rooms.new_id();

    // 新しく作ったroomのMessageListを作成
    messages.0.insert(id, Vec::new());
    // RoomListに新しいroomを追加
    rooms.0.push(Room::new(id, members));

    info!("create");
    Response::ok("create new room")
}

pub async fn get_rooms(
    user_list: Data<Mutex<UserList>>,
    room_list: Data<Mutex<RoomList>>,
    Json(get_user): Json<LoginInfo>,
) -> HttpResponse {
    info!("get rooms: {:?}", get_user);
    let users = user_list.lock().unwrap();
    if let Some(get_user) = users.find(get_user.id()) {
        let rooms = room_list.lock().unwrap();

        let rooms: Vec<&Room> = rooms
            .iter()
            .filter(|r| r.members().iter().any(|u| u.id() == get_user.id()))
            .collect();
        if rooms.len() == 0 {
            return Response::ok(Vec::<Room>::new());
        }
        if let Some(u) = rooms[0].members().iter().find(|u| u.id() == get_user.id()) {
            if let Some(u) = users.find(u.id()) {
                if u.password() == get_user.password() {
                    return Response::ok(
                        rooms
                            .iter()
                            .map(|r| {
                                Room::from((
                                    r.id(),
                                    r.members()
                                        .iter()
                                        .filter(|u| u.id() != get_user.id())
                                        .map(|u| u.clone())
                                        .collect::<Vec<UserInfo>>(),
                                ))
                            })
                            .collect::<Vec<Room>>(),
                    );
                } else {
                    return Response::error("not match user password");
                }
            } else {
                Response::error(format!("not found user id: {}", u.id()))
            }
        } else {
            Response::error(format!("not found user id: {}", get_user.id()))
        }
    } else {
        Response::error(format!("not found user id: {}", get_user.id()))
    }
}
