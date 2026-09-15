use tower_cookies::cookie::{Cookie, SameSite, time::Duration};

pub const SESSION_DURATION_SECONDS: i64 = 604800;
pub const AUTH_COOKIE_NAME: &str = "session";

pub fn build_auth_cookie(token: String, cookie_secure: bool) -> Cookie<'static> {
    Cookie::build((AUTH_COOKIE_NAME, token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(Duration::seconds(SESSION_DURATION_SECONDS))
        .secure(cookie_secure)
        .build()
}

pub fn build_removal_cookie(cookie_secure: bool) -> Cookie<'static> {
    Cookie::build((AUTH_COOKIE_NAME, ""))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(Duration::ZERO)
        .secure(cookie_secure)
        .build()
}
