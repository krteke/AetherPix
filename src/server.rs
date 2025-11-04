#[cfg(feature = "server")]
mod handlers;
#[cfg(feature = "server")]
mod middleware;
mod route;

#[cfg(feature = "server")]
pub(crate) use handlers::login_handler;
#[cfg(feature = "server")]
pub(crate) use handlers::verify_token_handler;
#[cfg(feature = "server")]
pub(crate) use middleware::auth_middleware;
#[cfg(feature = "server")]
pub(crate) use middleware::cookie_cleanup_middleware;
pub(crate) use route::client::Route;
#[cfg(feature = "server")]
pub(crate) use route::server::router;
