use crate::app_config::UserSetting;
use secrecy::{ExposeSecret, SecretBox};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    role: String,
    pwd: String,
}

impl User {
    pub fn parse(role: String, pwd: String) -> Self {
        User { role, pwd }
    }

    pub fn is_admin(&self, user_settings: &UserSetting) -> bool {
        self.role == "admin" && self.pwd == user_settings.pwd
    }

    pub fn is_upload(&self, user_settings: &UserSetting) -> bool {
        self.role == "upload" && self.pwd == user_settings.pwd
    }
}

impl From<UserSetting> for User {
    fn from(user_setting: UserSetting) -> Self {
        User {
            role: user_setting.role,
            pwd: user_setting.pwd,
        }
    }
}
