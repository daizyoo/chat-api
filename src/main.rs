mod friend;
mod message;
mod room;
mod types;
mod user;

use std::env;

use actix_cors::Cors;
use actix_web::{
    http::header,
    web::{scope, Data},
    App, HttpServer,
};

use serde_json::Value;
use sqlx::MySqlPool;
use tracing_subscriber::EnvFilter;

use types::{Room, UserId};

struct Database {
    pub pool: MySqlPool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().expect("Failed to load .env file");

    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;
    let db = Data::new(Database { pool });

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(scope("/message").configure(message::message_service_config))
            .service(scope("/room").configure(room::room_service_config))
            .service(scope("/user").configure(user::user_service_config))
            .service(scope("/friend").configure(friend::friend_service_config))
            .app_data(db.clone())
    })
    .bind(("127.0.0.1", 3478))?
    .run()
    .await?;

    Ok(())
}

#[derive(Debug)]
struct DBUser {
    id: String,
    name: String,
    password: String,
    friends: UserList,
}

#[derive(Debug)]
struct QueryUser {
    id: String,
    name: String,
    password: String,
    friends: Value,
}

#[derive(Debug)]
struct UserList {
    list: Vec<String>,
}

impl From<UserList> for Value {
    fn from(friends: UserList) -> Self {
        serde_json::json!({ "list": friends.list })
    }
}

/// mysqlに保存されたJson形式の {"list": [...]}をFriendsに変換する
impl From<Value> for UserList {
    fn from(value: Value) -> Self {
        Self {
            list: value
                .as_object()
                .unwrap()
                .get("list")
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap().to_string())
                .collect::<Vec<String>>(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use anyhow::Result;

    use sqlx::MySqlPool;

    use crate::{DBUser, QueryUser, UserList};

    pub async fn connect() -> Result<MySqlPool> {
        dotenvy::dotenv()?;
        Ok(MySqlPool::connect(&env::var("DATABASE_URL")?).await?)
    }

    #[tokio::test]
    async fn mysql_connect() -> anyhow::Result<()> {
        let pool = connect().await?;
        pool.close().await;
        Ok(())
    }

    #[tokio::test]
    async fn search_user() -> Result<()> {
        let pool = connect().await?;

        let user_name = String::from("name");

        let users = sqlx::query_as!(
            DBUser,
            "SELECT * FROM users WHERE id = ? OR name = ?",
            user_name,
            user_name
        )
        .fetch_all(&pool)
        .await?;

        println!("{:#?}", users);

        Ok(())
    }

    #[tokio::test]
    async fn create_user() -> Result<()> {
        let pool = connect().await?;
        let user = QueryUser {
            id: "id".to_string(),
            name: "test".to_string(),
            password: "password".to_string(),
            friends: UserList { list: vec![] }.into(),
        };
        sqlx::query!(
            "INSERT INTO users (id, name, password, friends) VALUES (?, ?, ?, ?)",
            user.id,
            user.name,
            user.password,
            user.friends
        )
        .execute(&pool)
        .await?;
        Ok(())
    }
}
