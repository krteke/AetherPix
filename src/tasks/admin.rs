use chrono::Local;
use loco_rs::{hash, prelude::*};

use crate::models::users::{users, RegisterParams};

pub struct Admin;
#[async_trait]
impl Task for Admin {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "admin".to_string(),
            detail: "Manage admin user".to_string(),
        }
    }
    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<()> {
        let username = vars
            .cli_arg("username")
            .map_err(|_| Error::string("Argument 'username' is required"))?;
        let password = vars
            .cli_arg("password")
            .map_err(|_| Error::string("Argument 'password' is required"))?;

        let email = vars
            .cli_arg("email")
            .map_err(|_| Error::string("Argument 'email' is required"))?;
        let params = RegisterParams {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        };
        validator::Validate::validate(&params).map_err(|e| Error::Validation(e.into()))?;

        println!("正在创建管理员账号...");
        let password_hash = hash::hash_password(password).map_err(|e| ModelError::Any(e.into()))?;
        let res = users::ActiveModel {
            password: ActiveValue::set(password_hash),
            username: ActiveValue::set(username.to_string()),
            role: ActiveValue::set(users::UserRole::Admin),
            email: ActiveValue::set(email.to_string()),
            email_verification_sent_at: ActiveValue::set(Some(Local::now().into())),
            email_verified_at: ActiveValue::set(Some(Local::now().into())),
            ..Default::default()
        }
        .insert(&app_context.db)
        .await;

        match res {
            Ok(_) => {
                println!("管理员账号创建成功");
            }
            Err(err) => {
                println!("管理员账号创建失败: {}", err);
            }
        }

        app_context.db.close_by_ref().await?;
        Ok(())
    }
}
