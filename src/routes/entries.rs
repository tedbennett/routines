use axum::{extract::State, response::Html, Form};
use serde::Deserialize;
use time::Date;
use uuid::Uuid;

use crate::{database::DataLayer, state::AppState, templates::routine_entry};

#[derive(Deserialize)]
pub struct ToggleEntryRequest {
    date: Date,
    routine_id: Uuid,
}

pub async fn toggle_entry<T: DataLayer>(
    State(state): State<AppState<T>>,
    Form(body): Form<ToggleEntryRequest>,
) -> Html<String> {
    let complete = state
        .db
        .toggle_entries(&body.date, &body.routine_id)
        .await
        .unwrap();

    let routine = state
        .db
        .get_routine(&body.routine_id)
        .await
        .unwrap()
        .unwrap();

    let markup = routine_entry(&body.date, !complete, &routine.color);
    tracing::info!("{}", markup.clone().into_string());
    Html(markup.into_string())
}
