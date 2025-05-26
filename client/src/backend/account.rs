use crate::backend::db::get_db;
use crate::error::{Error, Result};
use bcrypt::hash;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use surrealdb::Error as SurrealError;
use tokio::sync::OnceCell;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct User {
    login: Option<String>,
    password: Option<String>,
}

// Use OnceCell with RwLock for thread-safe access
static USER: OnceCell<Option<User>> = OnceCell::const_new();

// Removed redundant #[tokio::main] - you should only have one tokio runtime
pub async fn login(login: String, password: String) -> surrealdb::Result<User> {
    let db = get_db().await;

    let mut resPSW = db
        .query("RETURN crypto::bcrypt::generate($psw)")
        .bind(("psw", password))
        .await?;
    let ressPSW: Option<String> = resPSW.take(0).unwrap();
    let hashedPSW: String = ressPSW.unwrap_or("".to_string().clone());

    // RETURN crypto::bcrypt::generate("this is a strong password");
    let mut res = db
        .query("SELECT login, password FROM user WHERE login = $login AND password = $password")
        .bind(("login", login))
        .bind(("password", hashedPSW.clone()))
        .await?;

    let ress: Option<User> = res.take(0).unwrap();

    if let Some(user) = ress.clone() {
        USER.set(Some(user.clone())).unwrap();
        write_pwd_on_file(hashedPSW);
    }

    Ok(ress.unwrap_or_default())
}

pub async fn try_login() -> Result<()> {
    let user = USER
        .get()
        .ok_or(Error::Custom("User not initialized".to_string()))?
        .as_ref()
        .ok_or(Error::Custom("No user logged in".to_string()))?;

    if let Some(pwd) = &user.password {
        _validate_psswd(pwd.clone()).await?;
    }
    //da finirre

    Ok(())
}

async fn _validate_psswd(pass: String) -> Result<()> {
    // Add actual validation logic
    let db = get_db().await;

    let mut resPSW = db
        .query("RETURN crypto::bcrypt::compare($hash, $pass);") //query a caso da rimpiazzare (serveil login also)
        .bind(("hash", hash))
        .bind(("pass", pass))
        .await?;
    let ressPSW: Option<String> = resPSW.take(0).unwrap();
    let hashedPSW: String = ressPSW.unwrap_or("".to_string().clone());
    //da finirre
    Ok(())
}

async fn _get_psswrd_from_file() -> Result<String> {
    let mut buf = String::new();
    let mut file = OpenOptions::new()
        .read(true)
        .create(true)
        .open("/cookie/login.txt") //serve anche il login quindi va fatto conjson o boh explode nn lo so
        .map_err(|e| Error::Custom(format!("File error: {}", e)))?;

    file.read_to_string(&mut buf)
        .map_err(|e| Error::Custom(format!("Read error: {}", e)))?;

    Ok(buf)
}

async fn write_pwd_on_file(psw: String) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("/cookie/login.txt") //serve anche il login quindi va fatto conjson o boh explode nn lo so
        .map_err(|e| Error::Custom(format!("File error: {}", e)))?;

    file.write_all(psw.as_bytes())
        .map_err(|e| Error::Custom(format!("Write error: {}", e)))?;

    Ok(())
}
