use super::*;

pub async fn login(user_list: Data<Mutex<UserList>>, Json(login): Json<LoginInfo>) -> HttpResponse {
    let user_list = user_list.lock().unwrap();
    // ログインするユーザーが存在するか
    if let Some(user) = user_list.find(login.id()) {
        // パスワードの確認
        if user.password() == login.password() {
            info!("login: {:?}", user);
            Response::ok(LoginInfo::new(
                user.id().to_string(),
                user.password().to_string(),
            ))
        } else {
            Response::error("not match password")
        }
    } else {
        Response::error("not found user")
    }
}
