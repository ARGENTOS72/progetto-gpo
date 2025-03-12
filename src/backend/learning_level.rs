use serde_json::Value;
use surrealdb::{engine::any, Object, RecordIdKey};
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::Ws,
    opt::{auth::Root, Resource},
    RecordId, Surreal,
};

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct Level {
    id: RecordId,
    nome: String,
    quesiti: Vec<Value>,
    difficolta: i32,
    argomento: RecordId,
}

#[tokio::main]
pub async fn get_level() -> surrealdb::Result<()> {
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
    db.use_ns("Rustng")
        .use_db("gpo")
        .await
        .map_err(|e| {
            eprintln!("Failed to select namespace/database: {:?}", e);
            e
        })?;

    println!("Selected namespace and database.");

    // Authenticate
    db.signin(Root {
        username: "cum",
        password: "riri",
    })
    .await
    .map_err(|e| {
        eprintln!("Failed to authenticate: {:?}", e);
        e
    })?;

    println!("Authenticated successfully.");
/*
    // Execute the query
    let mut result = db.query("SELECT * FROM level")
        .await
        .map_err(|e| {
            eprintln!("Failed to execute query: {:?}", e);
            e
        })?;

    println!("Query executed successfully.");

    // Deserialize the result
    let levels: Vec<Level> = result.take(0).map_err(|e| {
        eprintln!("Failed to deserialize query result: {:?}", e);
        e
    })?;
*/
	let levels: Vec<Level> = db.select("level").await.map_err(|e| {
		eprintln!("Failed to execute query: {:?}", e);
		e
	})?;
    println!("Deserialized levels: {:?}", levels.get(0).unwrap().quesiti.get(0).unwrap().as_object().unwrap().get_key_value("type").unwrap().1);

    Ok(())
}