use std::sync::Mutex;

use actix_web::{
    web::{Data, Json},
    Responder,
};
use tracing::info;

use crate::{
    types::{Response, UserInfo},
    user::LoginInfo,
    CreateRoom, DataList, FriendList, MessageList, Room, RoomId, RoomList, UserList,
};

pub async fn create(
    friend_list: Data<Mutex<FriendList>>,
    user_list: Data<Mutex<UserList>>,
    message_list: Data<Mutex<MessageList>>,
    room_list: Data<Mutex<RoomList>>,
    Json(new_room): Json<CreateRoom>,
) -> impl Responder {
    info!("create");
    {
        // メンバーがフレンドかどうか
        let friends = friend_list.lock().unwrap();
        if let Some(friends) = friends.0.get(new_room.user.id()) {
            if !friends
                .iter()
                .filter(|u| new_room.members().iter().any(|user_id| u.id() == user_id))
                .count()
                == new_room.members.len()
            {
                return Response::error("フレンドではないユーザーが含まれています");
            }
        }
    }
    let mut members: Vec<UserInfo>;
    {
        let users = user_list.lock().unwrap();
        if users
            .iter()
            .find(|u| u.password() == new_room.user.password())
            .is_none()
        {
            return Response::ok("ユーザーのパスワードが間違っているのでRoomを作成できません");
        }
        // memberが全て存在するuserか
        if !new_room.members().iter().all(|id| users.exist(id)) {
            return Response::error(format!("存在しないユーザー: {:?}", new_room.members()));
        }
        members = new_room
            .members()
            .iter()
            .filter_map(|id| users.find(id))
            .map(|user| UserInfo::from(user))
            .collect();

        members.push(UserInfo::from(users.find(new_room.user.id()).unwrap()));
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
    let mut ids = rooms.iter().map(|r| *r.id()).collect::<Vec<RoomId>>();
    ids.sort();
    let id = ids.last().unwrap_or(&0) + 1;

    // 新しく作ったroomのMessageListを作成
    messages.0.insert(id, Vec::new());
    // RoomListに新しいroomを追加
    rooms.0.push(Room::new(id, members));

    Response::ok("create new room")
}

// TODO: アルゴリズムの改善
pub async fn get_rooms(
    user_list: Data<Mutex<UserList>>,
    room_list: Data<Mutex<RoomList>>,
    Json(get_user): Json<LoginInfo>,
) -> impl Responder {
    info!("get rooms: {:?}", get_user);
    let users = user_list.lock().unwrap();
    if let Some(get_user) = users.find(get_user.id()) {
        let rooms = room_list.lock().unwrap();
        info!("{:?}", rooms.0);

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
                    return Response::ok(rooms);
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
