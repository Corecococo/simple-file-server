use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Deserialize, Debug,Clone)]
pub struct AppSettings {
    pub admin_user:UserSetting ,
    pub upload_user:UserSetting ,
    pub target_dir: String,
}

#[derive(Deserialize, Debug,Clone)]
pub struct UserSetting {
    pub role: String,
    pub pwd: String,
}

pub fn load_app_config() -> Result<AppSettings, ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::with_name("config.yml"))
        .build()?;
    settings.try_deserialize::<AppSettings>()
}

#[cfg(test)]
mod tests {
    use claim::assert_ok;

    #[test]
    fn test_load_app_config() {
        let settings = super::load_app_config();
        println!("settings: {:?}", settings);
        assert_ok!(settings);
    }
}
