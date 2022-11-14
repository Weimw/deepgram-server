use crate::models::audio::*;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use actix_web::error::HttpError;
use actix_web::http::StatusCode;
use actix_web::web::{Buf, Data};
use uuid::Uuid;
use hound::{WavReader};


pub async fn post_file(
    body: web::Bytes,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    // TODO: gracefully handle empty and malformed wav
    let reader = WavReader::new(
        body.clone().reader()).unwrap();
    let metadata = AudioMeta {
        file_name: Uuid::new_v4().to_string(),
        duration: reader.duration() / reader.spec().sample_rate,
    };
    let file = Audio {
        metadata: metadata.clone(),
        data: body.to_vec(),
    };
    app_state.db
        .lock()
        .unwrap()
        .insert(metadata.file_name.clone(), file);
    HttpResponse::Ok().json(metadata.clone())
}

pub async fn get_file(
    query: web::Query<Name>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let file: Audio = app_state.db
        .lock()
        .unwrap()
        .get(&query.name)
        .unwrap().clone();
    HttpResponse::Ok().content_type("audio/wav").body(file.data)
}

pub async fn getall_file_metadata(
    query: web::Query<Duration>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let filtered_audio = app_state.db
        .lock()
        .unwrap()
        .clone()
        .into_values()
        .filter(|file| {
            match query.maxduration {
                Some(u32) => u32 >= file.metadata.duration,
                None => true
            }
        })
        .map(|file| file.metadata)
        .collect::<Vec<AudioMeta>>();

    HttpResponse::Ok().json(filtered_audio)
}

pub async fn get_file_metadata(
    query: web::Query<Name>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let file: Audio = app_state.db
        .lock()
        .unwrap()
        .get(&query.name)
        .unwrap().clone();
    HttpResponse::Ok().json(file.metadata)
}

