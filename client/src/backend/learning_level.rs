use serde::{Deserialize, Serialize};
use surrealdb::{engine::any, Object, RecordIdKey};
use surrealdb::{
    engine::remote::ws::Ws,
    opt::{auth::Root, Resource},
    RecordId, Surreal,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Level {
    id: RecordId,
    nome: String,
    quesiti: Vec<Quesito>,
    difficolta: i32,
    unita: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Ris {
    is_correct: bool,
    testo: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub enum TypeQuesito {
    Mul,
    Aperta,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Quesito {
    tipo: TypeQuesito,
    testo: String,
    ris: Option<Vec<Ris>>,
}

#[tokio::main]
pub async fn get_level(difficolta: i32, unita: i32) -> surrealdb::Result<Level> {
    println!("Connecting to SurrealDB...");

    // Open a connection
    let db = any::connect("wss://cute-reindeer-06adakj6hlsod2g654g8t2cs9c.aws-euw1.surreal.cloud")
        .await
        .map_err(|e| {
            eprintln!("Failed to connect to SurrealDB: {:?}", e);
            e
        })?;

    println!("Connected to SurrealDB.");

    // Select a namespace and database
    db.use_ns("Rustng").use_db("gpo").await.map_err(|e| {
        eprintln!("Failed to select namespace/database: {:?}", e);
        e
    })?;

    println!("Selected namespace and database.");

    // Authenticate
    db.signin(Root {
        username: "skibidicos",
        password: "skibigus",
    })
    .await
    .map_err(|e| {
        eprintln!("Failed to authenticate: {:?}", e);
        e
    })?;

    println!("Authenticated successfully.");

    // Execute the query
    let sql = "
            SELECT argomento.unita.numero AS unita, nome, quesiti, difficolta, id FROM level
            WHERE difficolta=$difficolta AND argomento.unita.numero=$unita
            ORDER BY rand()
            LIMIT 1
            FETCH argomento, unita_gloss
        ";
    let mut result = db
        .query(sql)
        .bind(("difficolta", difficolta))
        .bind(("unita", unita))
        .await
        .map_err(|e| {
            eprintln!("Failed to execute query: {:?}", e);
            e
        })?;

    println!("Query executed successfully.");

    // Deserialize the result
    let level: Option<Level> = result.take(0).map_err(|e| {
        eprintln!("Failed to deserialize query result: {:?}", e);
        e
    })?;

    /*
    let levels: Vec<Level> = db.select("level").await.map_err(|e| {
        eprintln!("Failed to execute query: {:?}", e);
        e
    })?;
    */
    println!("Deserialized levels: {:?}", level.clone().unwrap());

    Ok(level.unwrap())
}
