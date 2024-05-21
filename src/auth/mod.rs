mod backend;
mod sessions;
mod templates;

use axum::{
    response::{IntoResponse, Redirect},
    routing::get,
    Form,
};
use axum_login::{AuthManagerLayer, AuthManagerLayerBuilder};

use crate::AppRouter;
use backend::Credentials;
use sessions::Sessions;
use templates::{Login, Logout};

pub use backend::Backend;
pub type AuthSession = axum_login::AuthSession<Backend>;

pub async fn layer() -> AuthManagerLayer<Backend, Sessions> {
    let session_layer = Sessions::layer().await;
    let backend = Backend::new();

    AuthManagerLayerBuilder::new(backend, session_layer).build()
}

pub fn router() -> AppRouter {
    AppRouter::new()
        .route("/login", get(|| async { Login {} }).post(login))
        .route("/logout", get(|| async { Logout {} }).post(logout))
}

async fn login(mut auth_session: AuthSession, Form(cred): Form<Credentials>) -> impl IntoResponse {
    if let Some(user) = auth_session.authenticate(cred).await.unwrap() {
        auth_session.login(&user).await.unwrap();
    }
    Redirect::to("/")
}

async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
    auth_session.logout().await.unwrap();
    Redirect::to("/")
}
