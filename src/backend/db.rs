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
use dioxus::{hooks::use_context_provider, prelude::*};
use surrealdb::{
    engine::remote::ws::{Client, Wss},
    opt::auth::Root,
    Surreal,
};

#[derive(Clone)]
pub struct DB(Surreal<Client>);

#[server]
pub async fn load_db() -> Result<(), ServerFnError> {
    let db =
        Surreal::new::<Wss>("personal-projec-069vchuhbhqijbenh167ov516g.aws-euw1.surreal.cloud")
            .await
            .unwrap();

    db.use_ns("project").use_db("project").await.unwrap();

    db.signin(Root {
        username: &std::env::var("DB_USERNAME").unwrap(),
        password: &std::env::var("DB_PASSWORD").unwrap(),
    })
    .await
    .unwrap();

    use_context_provider(|| DB(db));

    Ok(())
}
