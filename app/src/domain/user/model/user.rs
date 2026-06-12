use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use validator::ValidationError;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct User{
    pub id: Option<i32>,
    pub username: Option<String>,
    pub token: Option<String>,
    pub password: Option<String>,
}

pub fn is_valid_username(username: &str) -> Result<(), ValidationError> {
    let len = username.len();
    // 1. Length constraint (e.g., 3 to 16 characters)
    if !(3..=16).contains(&len) {
        let mut error = ValidationError::new("length");
        error.add_param::<i32>(Cow::Borrowed("min"), &3);
        error.add_param::<i32>(Cow::Borrowed("max"), &16);
        return Err(error);
    }

    let mut chars = username.chars();

    // 2. First character must be a letter
    if let Some(first) = chars.next() {
        if !first.is_ascii_alphabetic() {
            return Err(ValidationError::new("wrong_first_letter")
                .with_message(Cow::Borrowed("Первый символ должен быть буквой")));
        }
    } else {
        return Err(ValidationError::new("required")); // Empty string
    }

    // 3. Remaining characters must be alphanumeric or underscore
    if !chars.all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(ValidationError::new("wrong_other_symbols").with_message(Cow::Borrowed(
            "Символы кроме первого должны быть цифрами, буквами или знаком подчеркивания",
        )));
    }

    Ok(())
}
