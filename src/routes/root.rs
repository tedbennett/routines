use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use maud::{html, Markup, DOCTYPE};
use serde::Serialize;

use crate::{
    database::Database,
    models::routines::{Routine, RoutineDataLayer},
    AppState,
};

#[derive(Serialize)]
pub struct IndexGlobals {
    routines: Vec<Routine>,
}

pub async fn root(State(state): State<AppState<Database>>) -> impl IntoResponse {
    let routines = state.db.get_routines(&state.user_id).await.unwrap();
    let markup = index(&routines);
    Html(markup.into_string())
}

fn index(routines: &[Routine]) -> Markup {
    html! {
        (header("Routines"))
        body {
            (navbar())
            div .page-container {
                div .routine-card-list {

                }

            }
            (create_routine_form())
        }
    }
}

fn navbar() -> Markup {
    html! {
        nav .navbar {
            span .nav-title {
                "Your Routines"
            }
        }
    }
}

fn create_routine_form() -> Markup {
    html! {
        form .card hx-post="/routine" hx-target="#routine-list" hx-swap="beforeend" {
            span .card-title {
                "Create Routine"
            }
            div .form-body {
                input .color-input type="color" name="color";
                input .title-input type="text" name="title";
                button type="submit" {
                    "Create"
                }
            }
        }
    }
}

fn routine_card(routine: &Routine) -> Markup {
    html! {
        span { "Hi!" }
    }
}

fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8" {}
        meta name="viewport" content="width=device-width, initial-scale=1.0" {}
        link rel="stylesheet" href="/static/index.css" {}
        script src="static/js/htmx@1.9.5.js" {}
        title { (page_title) }
    }
}
