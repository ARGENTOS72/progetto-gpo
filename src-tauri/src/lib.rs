// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
mod error;

use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{prelude::FromRow, Pool, Sqlite};
use std::{
    collections::HashMap,
    fs::{read_to_string, remove_file, File},
    io::{BufReader, Read},
    path::Path,
    sync::Mutex,
};
use tauri::State;
use uuid::Uuid;

use self::error::Result;

// #[derive(Debug, Deserialize, Serialize)]
// struct GlossaryArgument {
//     title: String,
//     content: String,
// }

// #[tauri::command]
// fn read_from_file(file_name: &str) -> Result<GlossaryArgument> {
//     let path = Path::new("./glossary/").join(file_name);

//     let file = File::open(path)?;
//     let reader = BufReader::new(file);

//     let main_argument: GlossaryArgument = serde_json::from_reader(reader)?;

//     Ok(main_argument)
// }

use once_cell::sync::Lazy;

static NAVBAR: Lazy<String> = Lazy::new(|| read_to_string("./helpers/navbar.html").unwrap());

#[tauri::command]
fn get_navbar() -> &'static str {
    &*NAVBAR
}

struct Session {
    id: Option<Uuid>,
    values: SessionValues,
}

#[derive(Deserialize, Serialize)]
struct SessionValues {
    #[serde(flatten)]
    values: HashMap<String, Value>,
}

impl SessionValues {
    fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
}

#[derive(Serialize)]
struct SerializeUser {
    name: String,
}

impl From<Value> for SerializeUser {
    fn from(value: Value) -> Self {
        Self {
            name: value.as_str().unwrap().to_string(),
        }
    }
}

impl From<&Value> for SerializeUser {
    fn from(value: &Value) -> Self {
        Self {
            name: value.as_str().unwrap().to_string(),
        }
    }
}

#[tauri::command]
fn get_user_session(session: State<'_, Session>) -> Result<SerializeUser> {
    let data = read_to_string(format!("sess_{}.json", session.id.unwrap()))?;
    let values: SessionValues = serde_json::from_str(&data)?;
    let user: SerializeUser = values.values.get("user_name").unwrap().into();

    Ok(user)
}

#[derive(Deserialize)]
struct UserDeserialize {
    email: String,
    password: String,
}

#[derive(Debug, FromRow)]
struct UserDb {
    id: i32,
    email: String,
    password: String,
}

#[tauri::command]
async fn login(app_state: State<'_, AppState>, user: UserDeserialize) -> Result<()> {
    let sql = "SELECT * FROM users WHERE email = ?1 AND password = ?2";

    let result: UserDb = sqlx::query_as(sql)
        .bind(user.email)
        .bind(user.password)
        .fetch_one(&app_state.db)
        .await
        .unwrap();

    println!("{:?}", result);

    Ok(())
}

fn save_state(session: State<'_, Session>) -> Result<()> {
    let serialized = serde_json::to_string(&session.values)?;
    std::fs::write(format!("sess_{}.json", session.id.unwrap()), serialized)?;
    Ok(())
}

type Db = Pool<Sqlite>;

struct AppState {
    db: Db,
}

async fn setup_db(app: &tauri::App) -> Db {
    use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions};
    use tauri::Manager;

    let mut path = app.path().app_data_dir().expect("failed to get data_dir");

    match std::fs::create_dir_all(path.clone()) {
        Ok(_) => {}
        Err(err) => {
            panic!("error creating directory {}", err);
        }
    };

    path.push("db.sqlite");

    remove_file(&path).unwrap();

    Sqlite::create_database(
        format!(
            "sqlite:{}",
            path.to_str().expect("path should be something")
        )
        .as_str(),
    )
    .await
    .expect("failed to create database");

    let db = SqlitePoolOptions::new()
        .connect(path.to_str().unwrap())
        .await
        .unwrap();

    sqlx::migrate!("./migrations").run(&db).await.unwrap();

    db
}

#[tauri::command]
fn start_session(
    session: State<'_, Mutex<Session>>,
    cookie_session_id: Option<Uuid>,
) -> Result<Uuid> {
    // Siccome è multithread, tecnicamente, prendiamo "controllo" del valore (Semafori)
    let mut session = session.lock().unwrap();

    // Se abbiamo già la sessione non la ricarichiamo
    if let (Some(session_id), Some(cookie_session_id)) = (session.id, cookie_session_id) {
        if session_id == cookie_session_id {
            return Ok(session_id);
        }
    }

    println!("{:?}", cookie_session_id);

    // Riceve id sessione, se vuoto ne crea uno
    session.id = Some(cookie_session_id.unwrap_or(Uuid::new_v4()));

    let session_id = session.id.unwrap();

    // Percorso del file di sessione
    let path = Path::new(".\\sessions").join(format!("sess_{session_id}.json"));
    let file = File::open(&path);
    println!("{}", path.display());

    // Controlliamo se il file esiste o no
    if let Ok(mut file) = file {
        // Se esiste leggiamo
        let mut data = String::new();

        file.read_to_string(&mut data)?;
        let values: SessionValues = serde_json::from_str(&data)?;

        session.values = values;
    } else {
        // Se non esiste creaiamo (vuoto)
        let serialized = serde_json::to_string(&session.values)?;
        std::fs::write(path, serialized)?;
    }

    Ok(session_id)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            use tauri::Manager;

            app.manage(Mutex::new(Session {
                id: None,
                values: SessionValues::new(),
            }));

            tauri::async_runtime::block_on(async move {
                let db = setup_db(&app).await;

                app.manage(AppState { db });
            });

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_navbar, start_session, login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
