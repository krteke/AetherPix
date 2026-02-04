use loco_rs::prelude::*;

use crate::{models::users::users, views::profile::UserProfileResponse};

async fn user_profile(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    tracing::debug!("Received request for user profile");
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    format::json(UserProfileResponse {
        name: user.username,
        email: user.email,
        api_token: user.api_key,
    })
}

pub fn router() -> Routes {
    Routes::new()
        .prefix("/api/profile")
        .add("/user", get(user_profile))
}
