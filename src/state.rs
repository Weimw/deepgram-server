use std::collections::HashMap;
use std::sync::Mutex;
use crate::models::audio::Audio;

pub struct AppState {
    pub db: Mutex<HashMap<String, Audio>>,
}