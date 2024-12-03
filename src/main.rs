mod friend;
mod message;
mod room;
mod types;
mod user;

use std::env;

use actix_cors::Cors;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key,
    http::header,
    web::{scope, Data},
    App, HttpServer,
};

use sqlx::MySqlPool;
use tracing_subscriber::EnvFilter;

struct Database {
    pub pool: MySqlPool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().expect("Failed to load .env file");

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let store = RedisSessionStore::new(env::var("REDIS_URL")?).await?;
    let key = Key::generate();
    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;
    let db = Data::new(Database { pool });

    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(store.clone(), key.clone())
                    .cookie_name(String::from("session-id"))
                    .build(),
            )
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

#[cfg(test)]
mod test {
    use std::env;

    use actix_session::storage::RedisSessionStore;
    use anyhow::Result;

    use sqlx::MySqlPool;

    use crate::types::{DBUser, QueryUser, UserList};

    async fn connect_redis() -> Result<RedisSessionStore> {
        dotenvy::dotenv()?;
        Ok(RedisSessionStore::new(env::var("REDIS_URL")?).await?)
    }

    pub async fn connect_mysql() -> Result<MySqlPool> {
        dotenvy::dotenv()?;
        Ok(MySqlPool::connect(&env::var("DATABASE_URL")?).await?)
    }

    #[tokio::test]
    async fn mysql_connect_test() -> anyhow::Result<()> {
        let pool = connect_mysql().await?;
        pool.close().await;
        Ok(())
    }

    #[tokio::test]
    async fn redis_connect_test() -> Result<()> {
        connect_redis().await?;
        Ok(())
    }

    #[tokio::test]
    async fn search_user() -> Result<()> {
        let pool = connect_mysql().await?;

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
        let pool = connect_mysql().await?;
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
