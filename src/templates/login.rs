use maud::{html, Markup};

use crate::templates::components::{header, navbar};

pub enum LoginInvite {
    Invite(String),
    InvalidInvite,
    None,
}

pub fn login(invite: LoginInvite) -> Markup {
    html! {
        (header("Routines"))
        body {
            (navbar(false))
            article .page-container {
                div .login-container {
                    @match invite {
                        LoginInvite::Invite(invite) => {
                            a .login-button href={"auth/google?invite=" (invite)} {
                                "Sign up with Google"
                            }
                        },
                        LoginInvite::InvalidInvite => {
                            h2 .card-title {
                                "Invalid Invite"
                            }
                            hr { }
                            a .login-button href="auth/google" {
                                "Login with Google"
                            }
                        }
                        LoginInvite::None => {
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
}
