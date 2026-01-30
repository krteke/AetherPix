use crate::{
    error::{AppError, AppResult},
    mailers::auth::AuthMailer,
    models::{
        _entities::users,
        users::{LoginParams, Model, RegisterParams},
    },
    views::auth::{ApiKeyResponse, CurrentResponse, LoginResponse},
};
use ::cookie::{time::Duration, Cookie};
use axum::http::{header::SET_COOKIE, HeaderValue};
use loco_rs::{config::JWT, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgotParams {
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResetParams {
    pub token: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResendVerificationParams {
    pub email: String,
}

/// Register function creates a new user with the given parameters
#[debug_handler]
async fn register(
    State(ctx): State<AppContext>,
    Json(params): Json<RegisterParams>,
) -> Result<Response> {
    if let Err(e) = validator::Validate::validate(&params) {
        tracing::info!("参数校验失败: {}", e);

        return Err(Error::Validation(e.into()));
    }

    let res = users::Model::create_with_password(&ctx.db, &params).await;

    let user = match res {
        Ok(user) => user,
        Err(err) => {
            tracing::info!(
                message = err.to_string(),
                user_email = &params.email,
                "could not register user.",
            );
            return format::json(());
        }
    };

    let user = user
        .into_active_model()
        .set_email_verification_sent(&ctx.db)
        .await?;

    AuthMailer::send_welcome(&ctx, &user).await?;

    format::json(())
}

/// Verify register user. if the user not verified his email, he can't login to
/// the system.
#[debug_handler]
async fn verify(State(ctx): State<AppContext>, Path(token): Path<String>) -> Result<Response> {
    let Ok(user) = users::Model::find_by_verification_token(&ctx.db, &token).await else {
        return unauthorized("invalid token");
    };

    if user.email_verified_at.is_some() {
        tracing::info!(pid = user.pid.to_string(), "user already verified");
    } else {
        let active_model = user.into_active_model();
        let user = active_model.verified(&ctx.db).await?;
        tracing::info!(pid = user.pid.to_string(), "user verified");
    }

    format::json(())
}

/// In case the user forgot his password  this endpoints generate a forgot token
/// and send email to the user. In case the email not found in our DB, we are
/// returning a valid request for for security reasons (not exposing users DB
/// list).
#[debug_handler]
async fn forgot(
    State(ctx): State<AppContext>,
    Json(params): Json<ForgotParams>,
) -> Result<Response> {
    let Ok(user) = users::Model::find_by_email(&ctx.db, &params.email).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        return format::json(());
    };

    let user = user
        .into_active_model()
        .set_forgot_password_sent(&ctx.db)
        .await?;

    AuthMailer::forgot_password(&ctx, &user).await?;

    format::json(())
}

/// reset user password by the given parameters
#[debug_handler]
async fn reset(State(ctx): State<AppContext>, Json(params): Json<ResetParams>) -> Result<Response> {
    let Ok(user) = users::Model::find_by_reset_token(&ctx.db, &params.token).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        tracing::info!("reset token not found");

        return format::json(());
    };
    user.into_active_model()
        .reset_password(&ctx.db, &params.password)
        .await?;

    format::json(())
}

/// Creates a user login and returns a token
#[debug_handler]
async fn login(
    State(ctx): State<AppContext>,
    Json(params): Json<LoginParams>,
) -> AppResult<impl IntoResponse> {
    let Ok(user) = users::Model::find_by_username(&ctx.db, &params.username).await else {
        tracing::debug!(
            username = params.username,
            "login attempt with non-existent username"
        );
        return Err(AppError::UsernameNotExist);
    };

    if user.email_verified_at.is_none() {
        return Err(AppError::EmailNotVerified);
    }

    let valid = user.verify_password(&params.password);
    if !valid {
        return Err(AppError::WrongCredentials);
    }

    let jwt_secret = ctx.config.get_jwt_config()?;
    let response = insert_jwt_into_cookie(jwt_secret, &user)?;

    Ok(response)
}

#[debug_handler]
async fn current(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(CurrentResponse::new(&user))
}

#[debug_handler]
async fn logout(_: auth::JWT, State(_): State<AppContext>) -> Result<Response> {
    let mut response = format::json(())?;
    let cookie = "auth_token=; Path=/; HttpOnly; Max-Age=0; SameSite=Lax";
    response
        .headers_mut()
        .insert(SET_COOKIE, HeaderValue::from_static(cookie));

    Ok(response)
}

#[debug_handler]
async fn resend_verification_email(
    State(ctx): State<AppContext>,
    Json(params): Json<ResendVerificationParams>,
) -> Result<Response> {
    let Ok(user) = users::Model::find_by_email(&ctx.db, &params.email).await else {
        tracing::info!(
            email = params.email,
            "User not found for resend verification"
        );
        return format::json(());
    };

    if user.email_verified_at.is_some() {
        tracing::info!(
            pid = user.pid.to_string(),
            "User already verified, skipping resend"
        );
        return format::json(());
    }

    let user = user
        .into_active_model()
        .set_email_verification_sent(&ctx.db)
        .await?;

    AuthMailer::send_welcome(&ctx, &user).await?;
    tracing::info!(pid = user.pid.to_string(), "Verification email re-sent");

    format::json(())
}

pub async fn get_api_key(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user_pid = auth.claims.pid;

    let user = users::Model::find_by_pid(&ctx.db, &user_pid)
        .await
        .map_err(|_| Error::Unauthorized("User not found".to_string()))?;

    format::json(ApiKeyResponse { key: user.api_key })
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/auth")
        .add("/register", post(register))
        .add("/verify/{token}", get(verify))
        .add("/login", post(login))
        .add("/forgot", post(forgot))
        .add("/reset", post(reset))
        .add("/current", get(current))
        .add("/logout", post(logout))
        .add("/resend-verification-mail", post(resend_verification_email))
        .add("/api-key", get(get_api_key))
}

fn insert_jwt_into_cookie(jwt: &JWT, user: &Model) -> Result<Response> {
    let token = user
        .generate_jwt(&jwt.secret, jwt.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;

    let mut response = format::json(LoginResponse::new(user))?;
    let cookie = Cookie::build(("auth_token", token))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(cookie::SameSite::Lax)
        .max_age(Duration::seconds(jwt.expiration as i64))
        .build();

    let Ok(header_value) = HeaderValue::from_str(&cookie.to_string()) else {
        tracing::warn!("Failed to create header value from cookie");

        return unauthorized("Internal server error");
    };

    response.headers_mut().insert(SET_COOKIE, header_value);

    Ok(response)
}
