use actix_web::web;
use crate::handlers::audio;

pub fn audio_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/post", web::post().to(audio::post_file))
        .route("/download", web::get().to(audio::get_file))
        .route("/list", web::get().to(audio::getall_file_metadata))
        .route("/info", web::get().to(audio::get_file_metadata));
}