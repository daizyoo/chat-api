use super::*;

pub async fn get(
    friends: Data<Mutex<FriendList>>,
    users: Data<Mutex<UserList>>,
    Json(user): Json<UserInfo>,
) -> HttpResponse {
    {
        let users = users.lock().unwrap();
        if !users.exist(user.id()) {
            return Response::error("not found user");
        }
    }
    let friends = friends.lock().unwrap();
    if let Some(vec) = friends.0.get(user.id()) {
        Response::ok(vec)
    } else {
        Response::error("not found friend list")
    }
}
