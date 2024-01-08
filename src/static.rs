use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::IntoResponse,
};
use include_dir::{include_dir, Dir};

static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");

pub async fn static_router(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');
    let mime_type = mime_guess::from_path(path).first_or_text_plain();
    let Some(file) = STATIC_DIR.get_file(path) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, mime_type.as_ref())],
        file.contents(),
    )
        .into_response()
}
