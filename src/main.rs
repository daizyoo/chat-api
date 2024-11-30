mod friend;
mod message;
mod room;
mod types;
mod user;

use std::{collections::HashMap, sync::Mutex};

use actix_cors::Cors;
use actix_web::{
    http::header,
    web::{post, scope, Data},
    App, HttpServer,
};

use tracing_subscriber::EnvFilter;

use types::{Message, Room, RoomId, User, UserId, UserInfo};

#[derive(Debug)]
struct RoomList(Vec<Room>);

#[derive(Debug)]
struct MessageList(HashMap<RoomId, Vec<Message>>);

#[derive(Debug)]
struct FriendList(HashMap<UserId, Vec<UserInfo>>);

#[derive(Debug)]
struct UserList(Vec<User>);

trait DataList {
    type Data;
    type ID;
    /// idの要素が存在するか
    fn exist(&self, id: &Self::ID) -> bool;
    /// idの要素を探す
    fn find(&self, id: &Self::ID) -> Option<&Self::Data>;
    /// iterにする
    fn iter(&self) -> std::slice::Iter<'_, Self::Data>;
}

impl UserList {
    fn serach(&self, user: UserInfo) -> impl Iterator<Item = &User> {
        self.0
            .iter()
            .filter(move |&u| u.id().contains(user.id()) || u.name().contains(user.name()))
    }
}

impl DataList for UserList {
    type Data = User;
    type ID = UserId;

    fn exist(&self, id: &Self::ID) -> bool {
        self.0.iter().any(|u| u.id() == id)
    }
    fn find(&self, id: &Self::ID) -> Option<&Self::Data> {
        self.iter().find(|user| user.id() == id)
    }
    fn iter(&self) -> std::slice::Iter<'_, Self::Data> {
        self.0.iter()
    }
}

impl DataList for RoomList {
    type Data = Room;
    type ID = RoomId;

    fn exist(&self, id: &Self::ID) -> bool {
        self.0.iter().any(|room| room.id() == id)
    }
    fn find(&self, id: &Self::ID) -> Option<&Self::Data> {
        self.iter().find(|room| room.id() == id)
    }
    fn iter(&self) -> std::slice::Iter<'_, Self::Data> {
        self.0.iter()
    }
}

impl RoomList {
    fn new_id(&self) -> u32 {
        let mut ids = self.iter().map(|r| *r.id()).collect::<Vec<RoomId>>();
        ids.sort();
        ids.last().unwrap_or(&0) + 1
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let _user = vec![
        User::new(
            "daiki".to_string(),
            "@daiki".to_string(),
            "daiki".to_string(),
        ),
        User::new(
            "kouta".to_string(),
            "@kouta".to_string(),
            "kouta".to_string(),
        ),
    ];

    // let rooms = Data::new(Mutex::new(RoomList(vec![Room::new(
    //     1,
    //     vec![UserInfo::from(&user[0]), UserInfo::from(&user[1])],
    // )])));
    // let messages = Data::new(Mutex::new(MessageList(HashMap::from([(1, Vec::new())]))));
    // let friends = Data::new(Mutex::new(FriendList(HashMap::from([
    //     (String::from(user[0].id()), vec![UserInfo::from(&user[1])]),
    //     (String::from(user[1].id()), vec![UserInfo::from(&user[0])]),
    // ]))));
    // let users = Data::new(Mutex::new(UserList(user)));
    let rooms = Data::new(Mutex::new(RoomList(Vec::new())));
    let messages = Data::new(Mutex::new(MessageList(HashMap::new())));
    let friends = Data::new(Mutex::new(FriendList(HashMap::new())));
    let users = Data::new(Mutex::new(UserList(Vec::new())));

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_origin("https://c3d9-42-125-172-148.ngrok-free.app")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(
                scope("/message")
                    .route("/send", post().to(message::send_message))
                    .route("/get", post().to(message::get_message)),
            )
            .service(
                scope("/room")
                    .route("/create", post().to(room::create))
                    .route("/get", post().to(room::get_rooms)),
            )
            .service(
                scope("/user")
                    .route("/create", post().to(user::create))
                    .route("/login", post().to(user::login))
                    .route("/search", post().to(user::search))
                    .service(user::info),
            )
            .service(
                scope("/friend")
                    .route("/add", post().to(friend::add))
                    .route("/search", post().to(friend::get))
                    .route("/delete", post().to(friend::delete)),
            )
            .app_data(rooms.clone())
            .app_data(messages.clone())
            .app_data(friends.clone())
            .app_data(users.clone())
    })
    .bind(("127.0.0.1", 3478))?
    .run()
    .await?;

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::*;
    use actix_web::{body::MessageBody, test, web::Json, HttpResponse};
    use types::{GetMessages, LoginInfo, MessageInfo};

    type DataM<T> = Data<Mutex<T>>;

    fn data_mutex<T>(data: T) -> DataM<T> {
        Data::new(Mutex::new(data))
    }

    fn message_list<const N: usize>(map: [(u32, Vec<Message>); N]) -> DataM<MessageList> {
        data_mutex(MessageList(HashMap::from(map)))
    }
    fn room_list(vec: Vec<Room>) -> DataM<RoomList> {
        data_mutex(RoomList(vec))
    }
    fn friend_list<const N: usize>(map: [(UserId, Vec<UserInfo>); N]) -> DataM<FriendList> {
        data_mutex(FriendList(HashMap::from(map)))
    }
    fn user_list(vec: Vec<User>) -> DataM<UserList> {
        data_mutex(UserList(vec))
    }

    fn user(name: &str, id: &str, pass: &str) -> User {
        User::new(name.to_string(), id.to_string(), pass.to_string())
    }
    fn info_user(name: &str, id: &str) -> UserInfo {
        UserInfo::new(name.to_string(), id.to_string())
    }
    fn login_user(id: &str, pass: &str) -> LoginInfo {
        LoginInfo::new(id.to_string(), pass.to_string())
    }

    fn default_rooms() -> Vec<Room> {
        vec![Room::new(
            0,
            vec![info_user("daiki", "@daiki"), info_user("kouta", "@kouta")],
        )]
    }

    fn default_data() -> (
        DataM<MessageList>,
        DataM<RoomList>,
        DataM<UserList>,
        DataM<FriendList>,
    ) {
        (
            message_list([(0, vec![])]),
            room_list(default_rooms()),
            user_list(vec![
                user("daiki", "@daiki", "daiki"),
                user("kouta", "@kouta", "kouta"),
            ]),
            friend_list([(String::from("@daiki"), vec![])]),
        )
    }

    fn body_string(res: HttpResponse) -> String {
        let bytes = res.into_body().try_into_bytes().unwrap().to_vec();
        String::from_utf8(bytes).unwrap()
    }
    fn print_data<T: std::fmt::Debug>(data: &DataM<T>) -> std::sync::MutexGuard<'_, T> {
        data.lock().unwrap()
    }

    #[test]
    async fn message_get() {
        let (msgs, rooms, users, _) = default_data();
        let json = Json(GetMessages::new(
            0,
            LoginInfo::new(String::from("@daiki"), String::from("daiki")),
        ));
        println!("{:#?}", print_data(&msgs).0);
        let res = message::get_message(msgs, users, rooms, json).await;
        let body = body_string(res);
        println!("{}", body)
    }
    #[test]
    async fn message_send() {
        let (msgs, _, _, _) = default_data();
        let msgsc = msgs.clone();
        let res = message::send_message(
            msgsc,
            Json(MessageInfo::new(
                0,
                String::from("test message"),
                LoginInfo::new("@daiki".to_string(), "daiki".to_string()),
            )),
        )
        .await;

        let body = body_string(res);
        println!("{}", body);
        println!("{:#?}", print_data(&msgs));
    }

    #[test]
    async fn room_create() {}
    #[test]
    async fn room_get() {}

    #[test]
    async fn user_create() {}
    #[test]
    async fn user_search() {}
    #[test]
    async fn user_login() {}

    #[test]
    async fn friend_add() {}
    #[test]
    async fn friend_get() {}
    #[test]
    async fn friend_delete() {}
}
