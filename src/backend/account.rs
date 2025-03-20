use async_trait::async_trait;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Wss, opt::auth::Root, Surreal};

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    login: String,
    password: String,
}

pub async fn login() -> Result<(), ServerFnError> {
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
}

#[server]
pub async fn signup(login: String, password: String) -> Result<(), ServerFnError> {
    let db =
        Surreal::new::<Wss>("personal-projec-069vchuhbhqijbenh167ov516g.aws-euw1.surreal.cloud")
            .await?;

    db.use_ns("project").use_db("project").await?;

    db.signin(Root {
        username: &std::env::var("DB_USERNAME").unwrap(),
        password: &std::env::var("DB_PASSWORD").unwrap(),
    })
    .await?;

    db.create("user").content(User {
        login
        password
    }).await?;


}

// use log::{error, info};
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
    path::Path,
    sync::Mutex,
};
use uuid::Uuid;

use crate::error::Result;

pub struct Session {
    id: Uuid,
    values: HashMap<String, Value>,
}

#[allow(unused)]
impl Session {
    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get(key)
    }

    pub fn set(&mut self, key: String, value: Value) {
        self.values.insert(key, value);
    }

    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.values.remove(key)
    }
}

impl std::fmt::Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Session: Id - {}; Values - {:?}",
            // It will be printed only when id exists
            self.id,
            self.values
        )
    }
}

#[server]
pub async fn session_start(
    app_handle: tauri::AppHandle,
    cookie_session_id: Option<Uuid>,
) -> Result<Uuid> {
    const SESSION_PATH: &str = "./sessions";

    _ = fs::create_dir(SESSION_PATH);

    let session = app_handle.try_state::<Mutex<Session>>();

    let session_id = match (session, cookie_session_id) {
        // Se non esiste la sessione e non esiste un cookie con l'id della sessione
        // Creiamo una nuova sessione e la salviamo
        (None, None) => {
            let session_id = Uuid::new_v4();

            let session = Mutex::new(Session {
                id: session_id,
                values: HashMap::new(),
            });

            app_handle.manage(session);

            let path = Path::new(SESSION_PATH).join(format!("sess_{session_id}.json"));
            fs::write(path, b"{}").unwrap();

            session_id
        }
        // Esiste la sessione ma non esiste il cookie con l'id della sessione
        // Creiamo una nuova sessione con nuovo id e eliminiamo la vecchia
        (Some(session), None) => {
            let path =
                Path::new(SESSION_PATH).join(format!("sess_{}.json", session.lock().unwrap().id));

            if let Err(e) = std::fs::remove_file(&path) {
                error!(
                    "Failed to remove file: {:?} - Error: {}",
                    path,
                    e.to_string()
                );
            };

            let session_id = Uuid::new_v4();

            let session = Mutex::new(Session {
                id: session_id,
                values: HashMap::new(),
            });

            app_handle.manage(session);

            let path = Path::new(SESSION_PATH).join(format!("sess_{session_id}.json"));
            fs::write(path, b"{}").unwrap();

            session_id
        }
        // Non esiste la sessione ma abbiamo l'id della sessione sul cookie
        // Carichiamo i dati della sessione se esiste il file sennò si crea
        (None, Some(cookie_session_id)) => {
            let session_id = cookie_session_id;

            let path = Path::new(SESSION_PATH).join(format!("sess_{session_id}.json"));

            if let Ok(file) = File::open(path) {
                // Esiste il file di sessione quindi leggiamo tutti i dati

                let file = BufReader::new(file);

                let values = serde_json::from_reader(file)?;

                let session = Mutex::new(Session {
                    id: session_id,
                    values,
                });

                app_handle.manage(session);
            } else {
                // Non esiste il file nel quale leggere i dati di sessione
                // Creiamo una nuova sessione e la salviamo

                let session = Mutex::new(Session {
                    id: session_id,
                    values: HashMap::new(),
                });

                app_handle.manage(session);

                let path = Path::new(SESSION_PATH).join(format!("sess_{session_id}.json"));
                fs::write(path, b"{}").unwrap();
            };

            session_id
        }
        (Some(session), Some(cookie_session_id)) => {
            let session_id = session.lock().unwrap().id;

            if session_id == cookie_session_id {
                return Ok(session_id);
            }

            // L'id del cookie e l'id della sessione sono differenti
            // Creiamo una nuova sessione con nuovo id e eliminiamo la vecchia

            let path = Path::new(SESSION_PATH).join(format!("sess_{session_id}.json"));

            if let Err(e) = std::fs::remove_file(&path) {
                error!(
                    "Failed to remove file: {:?} - Error: {}",
                    path,
                    e.to_string()
                );
            };

            let session = Mutex::new(Session {
                id: session_id,
                values: HashMap::new(),
            });

            app_handle.manage(session);

            let path = Path::new(SESSION_PATH).join(format!("sess_{session_id}.json"));
            fs::write(path, b"{}").unwrap();

            session_id
        }
    };

    info!("Loaded session with id: {session_id}");

    Ok(session_id)
}
