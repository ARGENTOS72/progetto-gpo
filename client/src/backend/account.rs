use crate::backend::db::get_db;
use crate::error::{Error, Result};
use dioxus::Ok;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Write};
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

    let ress: Option<User> = res.take(0).unwrap()?;

    if let Some(user) = ress.clone() {
        USER.set(Some(user.clone())).unwrap();
        write_pwd_on_file(hashedPSW, user.login.clone().unwrap());
    }

    Ok(ress.unwrap_or_default())
}

pub async fn try_login() -> Result<()> {
    let db = get_db().await;
    let user: User = USER.get().unwrap_or(&Some(_get_psswrd_from_file().await.as_ref().unwrap().clone())).as_ref().unwrap().clone();
    
        
    if let Some(pwd) = &user.password {
        let mut res = db
        .query("SELECT login, password FROM user WHERE login = $login AND password = $password")
        .bind(("login", user.login.clone().unwrap()))
        .bind(("password", user.password.clone().unwrap()))
        .await?;

        let ress: Option<User> = res.take(0).unwrap();
        return if let Some(user) = ress.clone() {
            USER.set(Some(user.clone())).unwrap();
            write_pwd_on_file(user.password.clone().unwrap(), user.login.clone().unwrap());
            Ok(())
        } else {
            Err(Error::Custom("failed automatic login".to_string()))
        }
    }

        Err(Error::Custom("failed automatic login".to_string()))
    //da finirre

}


async fn _get_psswrd_from_file() -> Result<User> {
    let mut buf = String::new();
    let mut file = OpenOptions::new()
        .read(true)
        .create(true)
        .open("/cookie/login.txt") //serve anche il login quindi va fatto conjson o boh explode nn lo so
        .map_err(|e| Error::Custom(format!("File error: {}", e)))?;

    file.read_to_string(&mut buf)
        .map_err(|e| Error::Custom(format!("Read error: {}", e)))?;

    let u:User= serde_json::from_str(&buf)?;

    Ok(u)
}

async fn write_pwd_on_file(psw: String, lg: String) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("/cookie/login.txt") //serve anche il login quindi va fatto conjson o boh explode nn lo so
        .map_err(|e| Error::Custom(format!("File error: {}", e)))?;

    let u:User= User {login: Some(lg.to_owned()), password: Some(psw.to_owned())};

    let buf= serde_json::to_string(&u)?;

    file.write_all(buf.as_bytes())
    .map_err(|e| Error::Custom(format!("Write error: {}", e)))?;

    Ok(())
}
