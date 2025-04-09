use config::{Config, File};
use once_cell::sync::Lazy;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    // #[serde(default)]
    #[serde(skip)]
    pub environment: Environment,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub host: String,
    pub port: u16,
    pub database_name: String,
    pub require_ssl: bool,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub host: String,
    pub port: u16,
    pub base_url: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub api_version: u16,
}

impl Default for Settings {
    fn default() -> Self {
        dotenv::dotenv().ok();
        let environment = Environment::default();

        let base_path = std::env::current_dir().expect("Failed to determine the current dirctory");
        let configuration_directory = base_path.join("configuration");
        let settings = Config::builder()
            .add_source(File::from(configuration_directory.join("base")).required(true))
            .add_source(
                File::from(configuration_directory.join(environment.as_str())).required(true),
            )
            // Add in settings from environment variables (with a prefix of APP and '__' as separator)
            // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
            .add_source(config::Environment::with_prefix("app").separator("__"))
            .build()
            .expect("Failed to parse configuration");

        settings.try_deserialize().expect("Invalid settings")
    }
}

impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = match self.require_ssl {
            true => PgSslMode::Require,
            false => {
                // Try an encrypted connection, fallback to unencrypted if it fails
                PgSslMode::Prefer
            }
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        let environment: Environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "local".into())
            .try_into()
            .expect("Failed to parse APP_ENVIRONMENT");

        environment
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}

pub static CONFIGURATION: Lazy<Settings> = Lazy::new(Settings::default);
