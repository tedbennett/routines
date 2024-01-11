use axum::{
    extract::{Query, State},
    response::Html,
};
use serde::Deserialize;
use time::{ext::NumericalDuration, Date, Duration, OffsetDateTime};

use uuid::Uuid;

use crate::{
    database::DataLayer,
    models::{entries::RoutineEntry, users::User},
    state::AppState,
    templates::home::index,
};
use crate::{models::routines::RoutineWithEntries, templates::login::login};

pub const NUM_ENTRIES: i64 = 60;

#[derive(Deserialize)]
pub struct QueryParams {
    invite: Option<String>,
}

pub async fn root<T: DataLayer>(
    user: Option<User>,
    State(state): State<AppState<T>>,
    Query(query): Query<QueryParams>,
) -> Html<String> {
    let Some(user) = user else {
        return Html(login(query.invite).into_string());
    };
    let routines = state.db.get_routines(&user.id).await.unwrap();
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

pub fn build_entry_table(routine: &Uuid, entries: &[RoutineEntry], size: i64) -> Vec<(Date, bool)> {
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
