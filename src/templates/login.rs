use maud::{html, Markup};

use crate::templates::components::{header, navbar};

pub fn login(invite: Option<String>) -> Markup {
    html! {
        (header("Routines"))
        body {
            (navbar(false))
            article .page-container {
                div .login-container {
                    @if let Some(invite) = invite {
                        a .login-button href={"auth/google?invite=" (invite)} {
                            "Login with Google"
                        }
                    } @else {
                        h2 .card-title {
                            "Not currently accepting new sign ups"
                        }
                        p .callout {
                            "An invite link is required to sign up"
                        }
                        hr { }
                        a .login-button href="auth/google" {
                            "Login with Google"
                        }
                    }
                }
            }
        }
    }
}
