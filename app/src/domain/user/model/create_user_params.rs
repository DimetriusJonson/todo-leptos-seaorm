use crate::domain::user::model::user::is_valid_username;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
pub struct CreateUserParams {
    pub version: usize,

    #[validate(required, custom(function = "is_valid_username"))]
    pub name: Option<String>,
    #[validate(required, length(min = 4))]
    pub password: Option<String>,
}
