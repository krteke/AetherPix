use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use axum_extra::extract::cookie::Cookie;
use reqwest::{
    header::{InvalidHeaderValue, SET_COOKIE},
    StatusCode,
};
use time::Duration;

use crate::server::handlers::ACCESS_TOKEN_NAME;

fn clear_cookie_header() -> Result<HeaderValue, InvalidHeaderValue> {
    let expired_cookie = Cookie::build((ACCESS_TOKEN_NAME, ""))
        .path("/")
        .max_age(Duration::seconds(0))
        .http_only(true)
        .secure(true)
        .same_site(axum_extra::extract::cookie::SameSite::Strict)
        .build();

    expired_cookie.to_string().try_into()
}

pub async fn cookie_cleanup_middleware(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;

    if response.status() == StatusCode::UNAUTHORIZED {
        tracing::debug!("Caught 401 response, attempting to clear invalid access_token cookie");

        match clear_cookie_header() {
            Ok(header_value) => {
                response.headers_mut().insert(SET_COOKIE, header_value);
            }
            Err(e) => {
                tracing::error!("Failed to create valid Set-Cookie header: {}", e);
            }
        }
    }

    response
}
