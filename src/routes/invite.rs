use axum::extract::State;

use crate::{database::DataLayer, models::users::User, state::AppState};

pub async fn create_invite<T: for<'a> DataLayer<'a>>(
    user: User,
    State(state): State<AppState<T>>,
) -> String {
    let token = state.db.create_invite(&user.id).await.unwrap();
    format!("{}?invite={}", state.env.client_url, token.to_string())
}
