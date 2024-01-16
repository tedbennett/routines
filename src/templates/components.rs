use crate::models::routines::Routine;
use maud::{html, Markup, DOCTYPE};
use time::Date;

pub fn navbar(signed_in: bool) -> Markup {
    html! {
        nav .navbar {
            .nav-contents {
                span .nav-title {
                    "Your Routines"
                }
                @if signed_in {
                    a href="/logout" {
                        "Logout"
                    }
                }
            }
        }
    }
}

pub fn create_routine_form() -> Markup {
    html! {
        form .card hx-post="/routine" hx-target="#routine-list" hx-swap="beforeend" {
            span .card-title {
                "Create Routine"
            }
            div .form-body {
                .form-row {
                    input .title-input type="text" placeholder="Routine name" name="title" required;
                    input .color-input type="color" name="color";
                }
                button .create-button type="submit" {
                    "Create"
                }
            }
        }
    }
}

pub fn routine_card(routine: &Routine, entries: &[(Date, bool)]) -> Markup {
    html! {
        div .card {
            span .card-title {
                (routine.title)
            }
            input name="routine_id" value=(routine.id) type="hidden" {}
            div .entry-container {
                    @for entry in entries {
                        (routine_entry(&entry.0, entry.1, &routine.color))
                    }
            }
        }
    }
}

pub fn routine_entry(date: &Date, complete: bool, color: &str) -> Markup {
    let bg_color = if complete { color } else { "#52525b" };
    html! {
        form hx-include="previous [name='routine_id']" hx-swap="outerHTML" {
            div .entry hx-post="/entry" style={"background-color: "(bg_color)} {}
            input type="hidden" name="date" value=(date) {}
        }
    }
}

pub fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8" {}
        meta name="viewport" content="width=device-width, initial-scale=1.0" {}
        link rel="stylesheet" href="/static/index.css" {}
        link rel="preconnect" href="https://fonts.googleapis.com" {}
        link rel="preconnect" href="https://fonts.gstatic.com" crossorigin {}
        link href="https://fonts.googleapis.com/css2?family=Fira+Mono:wght@400;500;700&display=swap" rel="stylesheet" {}
        script src="static/js/htmx@1.9.5.js" {}
        title { (page_title) }
    }
}
