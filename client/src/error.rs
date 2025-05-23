use dioxus::prelude::ServerFnError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::error::Error),
    #[error(transparent)]
    Db(#[from] surrealdb::Error),
    #[error("Error: {0}")]
    Custom(String),
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type Result<T> = std::result::Result<T, Error>;

// impl From<Error> for ServerFnError {
//     fn from(value: Error) -> Self {
//         match value {
//             Error::Io(e) => Self::new(e),
//             Error::Json(e) => match e.classify() {
//                 serde_json::error::Category::Io => Self::new(e),
//                 serde_json::error::Category::Syntax => e.into(),
//                 serde_json::error::Category::Data => e.into(),
//                 serde_json::error::Category::Eof => e.into(),
//             },
//             Error::Db(e) => Self::new(e),
//             Error::Custom(s) => Self::new(s),
//         }
//     }
// }
