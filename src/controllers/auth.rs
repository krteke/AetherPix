use crate::{
    models::{
        _entities::users,
        users::{LoginParams, RegisterParams},
    },
    views::auth::{CurrentResponse, LoginResponse},
};
use loco_rs::prelude::*;
use regex::Regex;
use std::sync::OnceLock;

pub static USERNAME_RE: OnceLock<Regex> = OnceLock::new();

pub fn get_allow_username_re() -> &'static Regex {
    USERNAME_RE.get_or_init(|| Regex::new(r"^[a-zA-Z0-9_-]+$").expect("Failed to compile regex"))
}

/// Register function creates a new user with the given parameters
#[debug_handler]
async fn register(
    State(ctx): State<AppContext>,
    Json(params): Json<RegisterParams>,
) -> Result<Response> {
    users::Model::create_with_password(&ctx.db, &params)
        .await
        .inspect_err(|e| {
            tracing::info!(
                message = e.to_string(),
                username = &params.username,
                "could not register user",
            );
        })?;

    format::json(())
}

/// Creates a user login and returns a token
#[debug_handler]
async fn login(State(ctx): State<AppContext>, Json(params): Json<LoginParams>) -> Result<Response> {
    let Ok(user) = users::Model::find_by_username(&ctx.db, &params.username).await else {
        tracing::debug!(
            username = params.username,
            "login attempt with non-existent username"
        );
        return unauthorized("Invalid credentials!");
    };

    let valid = user.verify_password(&params.password);

    if !valid {
        return unauthorized("unauthorized!");
    }

    let jwt_secret = ctx.config.get_jwt_config()?;

    let token = user
        .generate_jwt(&jwt_secret.secret, jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;

    format::json(LoginResponse::new(&user, &token))
}

#[debug_handler]
async fn current(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(CurrentResponse::new(&user))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/auth")
        .add("/register", post(register))
        // .add("/verify/{token}", get(verify))
        .add("/login", post(login))
        // .add("/forgot", post(forgot))
        // .add("/reset", post(reset))
        .add("/current", get(current))
}
