// #[cfg(feature = "server")]

//     pub static DB: Surreal<surrealdb::engine::remote::ws::Client> = {
//         let db =
//             Surreal::new::<Wss>("personal-projec-069vchuhbhqijbenh167ov516g.aws-euw1.surreal.cloud")
//                 .await
//                 .unwrap();

//         db.use_ns("project").use_db("project").await.unwrap();

//         db.signin(Root {
//             username: &std::env::var("DB_USERNAME").unwrap(),
//             password: &std::env::var("DB_PASSWORD").unwrap(),
//         })
//         .await
//         .unwrap();

//         db
//     };
// }

// use dioxus::prelude::*;
// use surrealdb::{engine::remote::ws::Wss, opt::auth::Root, Surreal};

// #[server]
// pub async fn test() -> Result<(), ServerFnError> {
//     DB.create("test").await.unwrap();

//     OK(())
// }

#![allow(unused)]
use async_once::AsyncOnce;
use dioxus::{hooks::use_context_provider, prelude::*};
use lazy_static::lazy_static;
use surrealdb::engine::any::Any;
use surrealdb::{engine::any, Object, RecordIdKey};
use surrealdb::{
    engine::remote::ws::{Client, Wss},
    opt::auth::Root,
    Surreal,
};
use tokio::sync::Mutex;
use tokio::sync::OnceCell;

// Dovrebbe funzionare (preso da tutorial/documentazione)
// In caso commentare
// lazy_static! {
//     static ref DB: AsyncOnce<Surreal<Client>> = {
//         AsyncOnce::new(async {
//             let db =
//                 Surreal::new::<Wss>("personal-projec-069vchuhbhqijbenh167ov516g.aws-euw1.surreal.cloud")
//                     .await
//                     .unwrap();

//             db.use_ns("project").use_db("project").await.unwrap();

//             db.signin(Root {
//                 username: &std::env::var("DB_USERNAME").unwrap(),
//                 password: &std::env::var("DB_PASSWORD").unwrap(),
//             })
//             .await
//             .unwrap();

//             db
//         })
//     };
// }

static DB_INSTANCE: OnceCell<Surreal<Any>> = OnceCell::const_new();

/// Returns the singleton DB instance (initializes on first call)
pub async fn get_db() -> &'static Surreal<Any> {
    DB_INSTANCE
        .get_or_init(|| async {
            // Connect to SurrealDB
            // let address = "localhost:8000"
            //     .parse::<SocketAddr>()
            //     .expect("Invalid DB address");

            // let db = Surreal::new::<Ws>(address)
            //     .await
            //     .expect("Failed to connect to DB");

            let db = any::connect(
                "wss://cute-reindeer-06adakj6hlsod2g654g8t2cs9c.aws-euw1.surreal.cloud",
            )
            .await
            .map_err(|e| {
                eprintln!("Failed to connect to SurrealDB: {:?}", e);
                e
            })
            .unwrap();
            db.use_ns("Rustng")
                .use_db("gpo")
                .await
                .map_err(|e| {
                    eprintln!("Failed to select namespace/database: {:?}", e);
                    e
                })
                .unwrap();
            // Authenticate and configure namespace/database
            db.signin(Root {
                username: "skibidicos",
                password: "skibigus",
            })
            .await
            .map_err(|e| {
                eprintln!("Failed to authenticate: {:?}", e);
                e
            })
            .unwrap();

            db
        })
        .await
}

// #[server]
// pub async fn load_db() -> Result<(), ServerFnError> {
//     let db =
//         Surreal::new::<Wss>("personal-projec-069vchuhbhqijbenh167ov516g.aws-euw1.surreal.cloud")
//             .await
//             .unwrap();

//     db.use_ns("project").use_db("project").await.unwrap();

//     db.signin(Root {
//         username: &std::env::var("DB_USERNAME").unwrap(),
//         password: &std::env::var("DB_PASSWORD").unwrap(),
//     })
//     .await
//     .unwrap();

//     Ok(())
// }
