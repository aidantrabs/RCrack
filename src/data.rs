use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    pub hash: String,
}