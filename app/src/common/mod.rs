pub mod api_error;
pub mod app_state;
pub mod security_context;
pub mod validate_helper;

#[cfg(feature = "ssr")]
use sea_orm::DatabaseConnection;

#[cfg(feature = "ssr")]
pub type DbPool = DatabaseConnection;
