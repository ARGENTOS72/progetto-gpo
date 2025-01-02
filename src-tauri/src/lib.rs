// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
mod error;

use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io::Read,
    path::Path,
    sync::Mutex,
};
use surrealdb::{engine::remote::ws::Wss, opt::auth::Root, Surreal};
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

impl std::fmt::Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Session: Id - {}; Values - {}",
            // It will be printed only when id exists
            self.id.unwrap(),
            self.values
        )
    }
}

#[derive(Deserialize, Serialize)]
struct SessionValues {
    #[serde(flatten)]
    values: HashMap<String, Value>,
}

impl std::fmt::Display for SessionValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.values)
    }
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

#[derive(Deserialize)]
struct UserDeserialize {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct UserDb {
    id: i32,
    email: String,
    password: String,
}

impl std::fmt::Display for UserDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User: Id - {}; Email - {}", self.id, self.email)
    }
}

#[tauri::command]
async fn login(app_state: State<'_, AppState>, user: UserDeserialize) -> Result<()> {
    let sql = "SELECT * FROM user WHERE email = $email AND password = $password";

    // let mut result = app_state
    //     .db
    //     .query(sql)
    //     .bind(("email", user.email))
    //     .bind(("password", user.password))
    //     .await?;

    // let user: Option<UserDb> = result.take(0)?;

    // if let Some(user) = user {
    //     debug!("Logged in as {}", user);
    // }

    Ok(())
}

fn save_state(session: State<'_, Session>) -> Result<()> {
    let file_path = format!("sess_{}.json", session.id.unwrap());

    debug!("Salvando dati nella sessione: {}", file_path);

    let serialized = serde_json::to_string(&session.values)?;
    std::fs::write(file_path, serialized)?;

    Ok(())
}

type Db = Surreal<surrealdb::engine::remote::ws::Client>;

struct AppState {
    db: Db,
}

async fn setup_db(_: &tauri::App) -> Db {
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

    db
}

#[tauri::command]
fn start_session(
    session: State<'_, Mutex<Session>>,
    cookie_session_id: Option<Uuid>,
) -> Result<Uuid> {
    // Siccome è multithread, tecnicamente, prendiamo "controllo" del valore (Semafori)
    // Dopo l'uso del metodo "lock()" si può usare la variabile come se fosse la struttura originale
    let mut session = session.lock().unwrap();

    // Se abbiamo già la sessione caricata non la ricarichiamo
    // (ritorniamo id perché obbligati [magari sostituire con Option<Uuid>])
    if let (Some(session_id), Some(cookie_session_id)) = (session.id, cookie_session_id) {
        if session_id == cookie_session_id {
            warn!("Ritorno di variabile/dato inutile, possibile sostituire ritorno con 'Option<Uuid>'");

            return Ok(session_id);
        }
    }

    // Riceve id sessione, se non c'è ne crea uno
    session.id = Some(cookie_session_id.unwrap_or(Uuid::new_v4()));

    let session_id = session.id.unwrap();

    // Percorso del file di sessione
    let path = Path::new("./sessions").join(format!("sess_{session_id}.json"));

    // Apre il file di sessione (ritorna Result quindi possibile fare check errori)
    let file = File::open(&path);

    // Controlliamo se il file esiste o no
    if let Ok(mut file) = file {
        // In questo caso esiste
        // Se esiste leggiamo
        let mut data = String::new();

        // Prendo i dati all'interno della sessione e li inserisco all'interno dell'hashmap
        file.read_to_string(&mut data)?;
        let values: SessionValues = serde_json::from_str(&data)?;

        // Inserisco i valori all'interno della struttura
        session.values = values;
    } else {
        // In questo caso non esiste
        // Se non esiste creaiamo il file di session (vuoto)
        let serialized = serde_json::to_string(&session.values)?;
        std::fs::write(path, serialized)?;
    }

    info!("Caricati i dati di sessione file: {}", session);

    Ok(session_id)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv::dotenv().ok();

    env_logger::Builder::new()
        .filter_level(log::LevelFilter::max())
        .filter_module("tokio_tungstenite", log::LevelFilter::Off)
        .filter_module("tungstenite", log::LevelFilter::Off)
        .filter_module("rustls", log::LevelFilter::Off)
        .init();

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
