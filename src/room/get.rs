use super::*;

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
