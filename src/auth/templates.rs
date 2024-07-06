use askama::Template;

use crate::User;

#[derive(Template)]
#[template(path = "auth/login.html")]
pub struct Login;

#[derive(Template)]
#[template(path = "auth/login_dev.html")]
pub struct LoginDev;

#[derive(Template)]
#[template(path = "auth/profile.html")]
pub(super) struct Profile<'a> {
    pub(super) user: &'a User,
}
