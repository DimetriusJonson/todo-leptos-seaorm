use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
};

use leptos::{reactive::{signal::WriteSignal, traits::Write, wrappers::read::Signal}, tachys::dom::event_target};
use leptos::server_fn::ServerFnError;
use leptos::{leptos_dom::logging::console_log, reactive::traits::Read};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::{Validate, ValidationError, ValidationErrors, ValidationErrorsKind};
use web_sys::{Event, HtmlInputElement};

pub fn ui_build_validation_errors<T>(
    error: &Option<Result<T, ServerFnError>>,
) -> HashMap<String, Vec<String>> {
    if let Some(Err(error)) = error {
        match error {
            ServerFnError::ServerError(msg) => match serde_json::from_str::<Value>(msg) {
                Ok(value) => {
                    if let Some(value_obj) = value.as_object() {
                        let mut map = HashMap::new();
                        for (field_name, field_errors_val) in value_obj.iter() {
                            if let Value::Array(field_errors) = field_errors_val {
                                map.insert(
                                    field_name.to_owned(),
                                    field_errors
                                        .iter()
                                        .map(|v| {
                                            v.get("message").unwrap().as_str().unwrap().to_string()
                                        })
                                        .collect(),
                                );
                            };
                        }
                        return map;
                    }
                }
                Err(_err) => {
                    return HashMap::from([("common_error".to_owned(), vec![msg.to_owned()])]);
                }
            },
            ServerFnError::Registration(msg) => {
                return HashMap::from([("common_error".to_owned(), vec![msg.to_owned()])]);
            }
            ServerFnError::Request(msg) => {
                return HashMap::from([("common_error".to_owned(), vec![msg.to_owned()])]);
            }
            ServerFnError::Response(msg) => {
                return HashMap::from([("common_error".to_owned(), vec![msg.to_owned()])]);
            }
            ServerFnError::MiddlewareError(msg) => {
                return HashMap::from([("common_error".to_owned(), vec![msg.to_owned()])]);
            }
            ServerFnError::Deserialization(msg) => {
                return HashMap::from([("common_error".to_owned(), vec![msg.to_owned()])]);
            }
            ServerFnError::Serialization(msg) => {
                return HashMap::from([("common_error".to_owned(), vec![msg.to_owned()])]);
            }
            ServerFnError::Args(msg) => {
                return HashMap::from([("common_error".to_owned(), vec![msg.to_owned()])]);
            }
            ServerFnError::MissingArg(msg) => {
                return HashMap::from([("common_error".to_owned(), vec![msg.to_owned()])]);
            }
            _ => return HashMap::new(),
        }
    }

    HashMap::new()
}

pub fn ui_build_common_error(errors: Signal<HashMap<String, Vec<String>>>) -> String {
    match errors.read().get("common_error") {
        Some(v) => v.first().unwrap_or(&"".to_owned()).to_owned(),
        None => "".to_owned(),
    }
}

pub fn ui_extract_field_errors(
    name: &str,
    validation_errors: Signal<HashMap<String, Vec<String>>>,
) -> Option<Vec<String>> {
    let all_errors = validation_errors.read();
    all_errors.get(name).map(|c| c.iter().map(|e| e.to_owned()).collect())
}

pub fn transform_validation_errors(validation_errors: ValidationErrors) -> ValidationErrors {
    let mut errors_map = validation_errors.0.clone();
    for (key, kind) in validation_errors.0 {
        match kind {
            ValidationErrorsKind::Struct(validation_errors) => {
                errors_map.insert(
                    key,
                    ValidationErrorsKind::Struct(Box::new(transform_validation_errors(
                        *validation_errors,
                    ))),
                );
            }
            ValidationErrorsKind::List(btree_map) => {
                let mut list_errors_map: BTreeMap<usize, Box<ValidationErrors>> = BTreeMap::new();
                for (i, validation_errors) in btree_map {
                    list_errors_map
                        .insert(i, Box::new(transform_validation_errors(*validation_errors)));
                }

                errors_map.insert(key, ValidationErrorsKind::List(list_errors_map));
            }
            ValidationErrorsKind::Field(validation_errors) => {
                let mut errors: Vec<ValidationError> = Vec::new();
                for field_err in validation_errors {
                    let mut new_field = field_err.clone();
                    new_field.message = transform_error_message(&field_err);
                    errors.push(new_field);
                }
                errors_map.insert(key, ValidationErrorsKind::Field(errors));
            }
        }
    }
    ValidationErrors(errors_map)
}

pub fn transform_error_message(field_err: &ValidationError) -> Option<Cow<'static, str>> {
    if field_err.message.is_some() {
        return field_err.message.to_owned();
    }

    let params = field_err.params.to_owned();
    let min = params.get("min");
    let max = params.get("max");

    match (field_err.code.as_ref(), min, max) {
        ("required", ..) => Some(Cow::Borrowed("Обязательно для заполнения")),
        ("length", Some(min), Some(max)) => {
            Some(Cow::Owned(format!("Длина от {} до {} символов", min, max)))
        }
        ("length", Some(min), None) => Some(Cow::Owned(format!("Длина минимум {} символа", min))),
        ("length", None, Some(max)) => Some(Cow::Owned(format!("Длина максимум {} символа", max))),
        _ => field_err.message.to_owned(),
    }
}

pub fn validate_field_value<T>(field_name: String, value: String, form_data: T) -> Vec<String>
where
    T: Validate + Clone + Default + Serialize + for<'a> Deserialize<'a> + 'static,
{
    let default_value = serde_json::to_value(form_data.clone()).unwrap();
    let mut default_map: HashMap<String, serde_json::Value> =
        serde_json::from_value(default_value).unwrap();

    default_map.insert(field_name.to_owned(), serde_json::Value::String(value));

    let mut errors = Vec::new();
    if let Ok(json_value) = serde_json::to_value(default_map) {
        match serde_json::from_value::<T>(json_value) {
            Ok(entity) => {
                if let Err(validation_errors) = entity.validate()
                    && let Some(errors_kind) = validation_errors.0.get(field_name.as_str())
                {
                    errors = validation_errors_kind_to_list(errors_kind);
                }
            }
            Err(err) => console_log(&format!("from_value error={}", err)),
        }
    }
    errors
}

pub fn extract_form_field_name(name: String) -> String {
    match (name.find('['), name.find(']')) {
        (Some(pos_start), Some(pos_end)) => name[pos_start + 1..pos_end].to_owned(),
        _ => "".to_owned(),
    }
}

pub fn validation_errors_to_map(
    validation_errors: ValidationErrors,
) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::new();

    for (field_name, errors_kind) in validation_errors.0 {
        let errors = validation_errors_kind_to_list(&errors_kind);
        result.insert(field_name.into_owned(), errors);
    }

    result
}

pub fn validation_errors_kind_to_list(errors_kind: &ValidationErrorsKind) -> Vec<String> {
    match errors_kind {
        validator::ValidationErrorsKind::Field(field_errors) => field_errors
            .iter()
            .map(|e| transform_error_message(e).unwrap_or(Cow::Borrowed("error")).into_owned())
            .collect::<Vec<String>>(),
        _ => Vec::new(),
    }
}

pub fn validate_form<T>(
    event: Event,
    set_validation_errors: WriteSignal<HashMap<String, Vec<String>>>,
    form_data: T,
) where
    T: Validate + Clone + Default + Serialize + for<'a> Deserialize<'a> + 'static,
{
    let target = event_target::<HtmlInputElement>(&event);
    let field_name = extract_form_field_name(target.name().to_owned());
    set_validation_errors.write().insert(
        field_name.to_owned(),
        validate_field_value(field_name.to_owned(), target.value(), form_data.clone()),
    );
}
