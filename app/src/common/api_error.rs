use std::borrow::Cow;
use std::error::Error;
use std::fmt::Display;

use leptos::server_fn::error::ServerFnErrorErr;
use validator::{ValidationError, ValidationErrors};

use crate::common::validate_helper::transform_validation_errors;

#[derive(Debug, PartialEq)]
pub enum ApiError {
    UnAuthorized(String),
    NotFound(String),
    Validation(ValidationErrors),
    Network(String),
    Db(String),
    ServerFn(ServerFnErrorErr),
}

impl ApiError {
    pub fn validation_field(
        field: &'static str,
        code: &'static str,
        message: &'static str,
    ) -> Self {
        let mut errors = ValidationErrors::new();
        errors.add(field, ValidationError::new(code).with_message(Cow::Borrowed(message)));
        Self::Validation(errors)
    }

    pub fn validation(validation_errors: ValidationErrors) -> Self {
        Self::Validation(transform_validation_errors(validation_errors))
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnAuthorized(msg) => write!(f, "Пользователь не авторизован. {}", msg),
            Self::NotFound(msg) => write!(f, "{}", msg),
            Self::Db(msg) => write!(f, "{}", msg),
            Self::Network(msg) => write!(f, "Ошибка запроса: {}.", msg),
            Self::Validation(errors) => {
                write!(f, "{}", serde_json::to_string(&errors).expect("Failed serialize error!"))
            }
            Self::ServerFn(err) => write!(f, "Ошибка сервера: {}", err),
        }
    }
}

impl Error for ApiError {}
