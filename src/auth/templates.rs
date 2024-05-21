use askama::Template;

#[derive(Template)]
#[template(path = "auth/login.html")]
pub struct Login;

#[derive(Template)]
#[template(path = "auth/logout.html")]
pub struct Logout;
