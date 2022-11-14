use crate::models::audio::*;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use actix_web::error::HttpError;
use actix_web::web::Buf;
use uuid::Uuid;
use hound::{WavReader};


pub async fn post_file(
    body: web::Bytes,
    app_state: web::Data<AppState>
) -> HttpResponse {
    // TODO: gracefully handle empty and malformed wav
    let reader = WavReader::new(
        body.clone().reader()).unwrap();
    let metadata = AudioMeta {
        file_name: Uuid::new_v4().to_string(),
        duration: reader.duration(),
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
    query: web::Query<String>
) -> HttpResponse {
    println!("{}", &query);
    HttpResponse::Ok().into()
}

pub async fn getall_file_metadata(
    query: web::Query<String>
)-> HttpResponse  {
    println!("{}", &query);
    HttpResponse::Ok().into()
}

pub async fn get_file_metadata(
    query: web::Query<String>
) -> HttpResponse {
    println!("{}", &query);
    HttpResponse::Ok().into()
}

