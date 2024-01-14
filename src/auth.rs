use anyhow::{Context, Result};
use async_session::{Session, SessionStore};

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, Query, State},
    http::{header::SET_COOKIE, HeaderMap},
    response::{IntoResponse, Redirect, Response},
    RequestPartsExt,
};
use axum_extra::{headers, typed_header::TypedHeaderRejectionReason, TypedHeader};
use http::{header, request::Parts};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};

use crate::{
    database::{DataLayer, Database},
    error::ApiResult,
    models::users::{User, UserDataLayer},
    state::{AppState, Env},
};

static COOKIE_NAME: &str = "SESSION";

pub fn oauth_client(env: &Env) -> ApiResult<BasicClient> {
    Ok(BasicClient::new(
        ClientId::new(env.client_id.clone()),
        Some(ClientSecret::new(env.client_secret.clone())),
        AuthUrl::new(env.auth_url.clone())
            .context("failed to create new authorization server URL")?,
        Some(
            TokenUrl::new(env.token_url.clone())
                .context("failed to create new token endpoint URL")?,
        ),
    )
    .set_redirect_uri(
        RedirectUrl::new(env.redirect_url.clone())
            .context("failed to create new redirection URL")?,
    ))
}

pub async fn google_auth(State(client): State<BasicClient>) -> impl IntoResponse {
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .url();

    // Redirect to Google's oauth service
    Redirect::to(&auth_url.to_string())
}

// Google response
#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub sub: String, // The ID
    pub email: String,
    pub name: String,
}

// Valid user session required. If there is none, redirect to the auth page
pub async fn protected(user: User) -> impl IntoResponse {
    format!(
        "Welcome to the protected area :)\nHere's your info:\n{:?}",
        user.id
    )
}

pub async fn logout<T: for<'a> DataLayer<'a>>(
    State(store): State<DBSessionStore<T>>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> ApiResult<impl IntoResponse> {
    let cookie = cookies
        .get(COOKIE_NAME)
        .context("unexpected error getting cookie name")?;

    let session = match store
        .load_session(cookie.to_string())
        .await
        .context("failed to load session")?
    {
        Some(s) => s,
        // No session active, just redirect
        None => return Ok(Redirect::to("/")),
    };

    store
        .destroy_session(session)
        .await
        .context("failed to destroy session")?;

    Ok(Redirect::to("/"))
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AuthRequest {
    code: String,
    state: String,
}

pub async fn login_authorized(
    Query(query): Query<AuthRequest>,
    State(state): State<AppState<Database>>,
) -> ApiResult<impl IntoResponse> {
    // Get an auth token
    let token = state
        .oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .context("failed in sending request request to authorization server")?;
    let user_data: UserResponse = state
        .http_client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .unwrap()
        .json::<UserResponse>()
        .await
        .unwrap();
    let user = state.db.upsert_user(&user_data).await?;
    // Create a new session filled with user data
    let mut session = Session::new();
    session
        .insert("user", &user)
        .context("failed in inserting serialized value into session")?;

    // Store session and get corresponding cookie
    let cookie = state
        .session_store
        .store_session(session)
        .await
        .context("failed to store session")?
        .context("unexpected error retrieving cookie value")?;

    // Build the cookie
    let cookie = format!("{COOKIE_NAME}={cookie}; SameSite=Lax; Path=/");

    // Set cookie
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        cookie.parse().context("failed to parse cookie")?,
    );

    Ok((headers, Redirect::to("/")))
}

pub struct AuthRedirect;

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary("/auth/google").into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    DBSessionStore<Database>: FromRef<S>,
    S: Send + Sync,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = AuthRedirect;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let store = DBSessionStore::from_ref(state);

        let cookies = parts
            .extract::<TypedHeader<headers::Cookie>>()
            .await
            .map_err(|e| match *e.name() {
                header::COOKIE => match e.reason() {
                    TypedHeaderRejectionReason::Missing => AuthRedirect,
                    _ => panic!("unexpected error getting Cookie header(s): {e}"),
                },
                _ => panic!("unexpected error getting cookies: {e}"),
            })?;
        let session_cookie = cookies.get(COOKIE_NAME).ok_or(AuthRedirect)?;

        let session = store
            .load_session(session_cookie.to_string())
            .await
            .unwrap()
            .ok_or(AuthRedirect)?;

        let user = session.get::<User>("user").ok_or(AuthRedirect)?;

        Ok(user)
    }
}

#[derive(Clone, Debug)]
pub struct DBSessionStore<T: for<'a> DataLayer<'a>> {
    pub db: T,
}

#[async_trait]
impl<T: for<'a> DataLayer<'a>> SessionStore for DBSessionStore<T> {
    async fn load_session(
        &self,
        cookie_value: String,
    ) -> Result<Option<Session>, async_session::Error> {
        let id = Session::id_from_cookie_value(&cookie_value)?;
        let Some(session_str) = self.db.get_session(&id).await? else {
            return Ok(None);
        };
        serde_json::from_str(&session_str).context("")
    }

    async fn store_session(&self, session: Session) -> Result<Option<String>> {
        let id = session.id();
        let serialized_session = serde_json::to_string(&session)?;

        self.db
            .insert_session(
                id,
                &serialized_session,
                session.expiry().map(|t| t.to_string()),
            )
            .await?;

        Ok(session.into_cookie_value())
    }

    async fn destroy_session(&self, session: Session) -> Result<()> {
        let id = session.id();
        self.db.delete_session(id).await?;
        Ok(())
    }

    async fn clear_store(&self) -> Result<()> {
        self.db.delete_all_sessions().await
    }
}
