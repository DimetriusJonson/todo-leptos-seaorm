use std::collections::HashMap;

use crate::{
    common::validate_helper::{extract_form_field_name, ui_extract_field_errors},
    components::ui::select_input::SelectInput,
};
use leptos::prelude::*;

pub type SelectOption = (Option<String>, String);

#[component]
pub fn SelectWithError(
    name: String,
    label: String,
    #[prop(optional)] value: String,
    #[prop(optional)] not_selected_text: String,
    options: Vec<SelectOption>,
    #[prop(into)] on_change: Callback<String>,
    validation_errors: Signal<HashMap<String, Vec<String>>>,
) -> impl IntoView {
    view! {

        <div class="flex items-center">
            <label class="block text-base text-gray-700 dark:text-gray-50 mx-2" for={name.to_owned()}>{label.to_owned()}</label>
            <SelectInput
                name={name.to_owned()}
                label={label}
                value=value
                options={options}
                not_selected_text=not_selected_text
                on_change=on_change
            />

        {
            let field_name = extract_form_field_name(name.to_owned());
            move || {
                let errors = ui_extract_field_errors(&field_name, validation_errors);
                errors.map(|list| list.into_iter().map(|msg| 
                    view!{ <p class="mt-1 px-2 text-xs text-pink-700 dark:text-danger">{msg}</p>}).collect::<Vec<_>>())
            }
        }
        </div>
    }
}
