use chrono::{Duration, Utc};
use poem::web::cookie::SameSite;
use poem::EndpointExt;
use poem::{
    get, handler,
    listener::TcpListener,
    middleware::CookieJarManager,
    web::cookie::{Cookie, CookieJar},
    Route, Server,
};

use std::ops::Add;

const COOKIE_NAME: &str = "cookie-name";

#[handler]
fn hello(cookie_jar: &CookieJar) -> String {
    let cookie = cookie_jar.get(COOKIE_NAME);
    let cookie_value = match cookie {
        Some(cookie) => cookie.value_str().to_string(),
        None => "no cookie set".to_owned(),
    };
    format!("cookie value: {}", cookie_value)
}

#[handler]
fn hello_set(cookie_jar: &CookieJar) -> String {
    let mut new_cookie = Cookie::new_with_str(COOKIE_NAME, "my_cookie_value");
    new_cookie.set_path("/");
    new_cookie.set_domain(".example.local");
    new_cookie.set_http_only(true);
    new_cookie.set_expires(Utc::now().add(Duration::days(30)));
    new_cookie.set_same_site(SameSite::Strict);

    cookie_jar.add(new_cookie);

    format!("cookie set")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/hello", get(hello))
        .at("/hello_set", get(hello_set))
        .with(CookieJarManager::new());
    Server::new(TcpListener::bind("localhost:3000"))
        .run(app)
        .await
}
