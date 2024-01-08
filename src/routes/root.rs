use axum::{extract::State, response::Html, Form};
use serde::Deserialize;
use time::{ext::NumericalDuration, Date, Duration, OffsetDateTime};

use uuid::Uuid;

use crate::{
    database::DataLayer,
    models::entries::RoutineEntry,
    templates::{index, routine_card, routine_entry, RoutineWithEntries},
    AppState,
};

const NUM_ENTRIES: i64 = 60;

pub async fn root<T: DataLayer>(State(state): State<AppState<T>>) -> Html<String> {
    let routines = state.db.get_routines(&state.user_id).await.unwrap();
    let ids = routines.iter().map(|r| r.id).collect();
    let all_entries = state.db.get_entries(&ids).await.unwrap();
    let data: Vec<_> = routines
        .into_iter()
        .map(|r| {
            let entries = build_entry_table(&r.id, &all_entries, NUM_ENTRIES);
            RoutineWithEntries {
                routine: r,
                entries,
            }
        })
        .collect();
    let markup = index(&data);
    Html(markup.into_string())
}

#[derive(Deserialize)]
pub struct CreateRoutineRequest {
    title: String,
    color: String,
}

pub async fn create_routine<T: DataLayer>(
    State(state): State<AppState<T>>,
    Form(body): Form<CreateRoutineRequest>,
) -> Html<String> {
    let routine = state
        .db
        .create_routine(&body.title, &body.color, &state.user_id)
        .await
        .unwrap();
    let entries = build_entry_table(&routine.id, &vec![], NUM_ENTRIES);
    let markup = routine_card(&routine, &entries);
    Html(markup.into_string())
}

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

fn build_entry_table(routine: &Uuid, entries: &[RoutineEntry], size: i64) -> Vec<(Date, bool)> {
    let now = OffsetDateTime::now_utc();
    let start = Date::from_calendar_date(now.year(), now.month(), now.day())
        .unwrap()
        .checked_add(-Duration::days(size))
        .unwrap();

    (0..size)
        .into_iter()
        .map(|i| {
            let date = start.checked_add(i.days()).unwrap();
            (
                date,
                entries
                    .iter()
                    .find(|e| e.routine_id == *routine && e.date == date)
                    .is_some(),
            )
        })
        .collect()
}
