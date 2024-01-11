use super::components::{create_routine_form, header, navbar, routine_card};
use crate::models::routines::RoutineWithEntries;
use maud::{html, Markup};

pub fn index(routines: &[RoutineWithEntries]) -> Markup {
    html! {
        (header("Routines"))
        body {
            (navbar(true))
            article .page-container {
                div .routine-card-list #routine-list {
                    @for routine in routines {
                        (routine_card(&routine.routine, &routine.entries))
                    }
                }
                (create_routine_form())
            }
        }
    }
}
