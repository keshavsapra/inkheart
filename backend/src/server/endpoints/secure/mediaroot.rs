use std::io;

use actix_web::{get, web, Responder};

use crate::{
    indexer::utils::hash_string,
    server::{auth::FirebaseUser, endpoints::library_path, state::AppState},
};

#[get("/api/mediaroot")]
pub async fn mediaroot(
    data: web::Data<AppState>,
    _user: FirebaseUser,
) -> io::Result<impl Responder> {
    let config = data.get_config()?;
    let id = hash_string(&config.media_root);

    library_path(data, id, _user).await
}
