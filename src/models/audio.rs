// TODO: add defined error

use actix_web::web::{self, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Audio {
    pub metadata: AudioMeta,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioMeta {
    pub file_name: String,
    pub duration: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Query {

}