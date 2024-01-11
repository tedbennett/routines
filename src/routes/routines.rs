use axum::{extract::State, response::Html, Form};
use serde::Deserialize;

use crate::{
    database::DataLayer, models::users::User, state::AppState, templates::components::routine_card,
};

use super::root::{build_entry_table, NUM_ENTRIES};

#[derive(Deserialize)]
pub struct CreateRoutineRequest {
    title: String,
    color: String,
}

pub async fn create_routine<T: DataLayer>(
    State(state): State<AppState<T>>,
    user: User,
    Form(body): Form<CreateRoutineRequest>,
) -> Html<String> {
    let routine = state
        .db
        .create_routine(&body.title, &body.color, &user.id)
        .await
        .unwrap();
    let entries = build_entry_table(&routine.id, &vec![], NUM_ENTRIES);
    let markup = routine_card(&routine, &entries);
    Html(markup.into_string())
}
