use std::collections::HashMap;

use leptos::prelude::*;

use crate::common::validate_helper::{extract_form_field_name, ui_extract_field_errors};

#[component]
pub fn TextWithError(
    name: String,
    placeholder: String,
    input_type: String,
    #[prop(optional)] value: String,
    validation_errors: Signal<HashMap<String, Vec<String>>>,
) -> impl IntoView {
    let field_name = extract_form_field_name(name.to_owned());

    view! {
        <div class="w-full">
            <input
                aria-invalid={move || (validation_errors.read().get(&field_name).map_or(false, |v|!v.is_empty())).to_string()}
                class={"w-full px-4 py-2 rounded-md shadow-inner 
            text-gray-700 
            placeholder-gray-400 
            dark:text-gray-50 

            autofill:bg-blue-300/20  
            dark:autofill:bg-gray-50

            focus:outline-4
            border

            transition-[background-color,border-color,box-shadow,color]
            duration-294
            
            hover:ring-gray-400
            hover:border-gray-400
            dark:hover:ring-gray-500
            dark:hover:border-gray-500
            bg-white
            dark:bg-dark-bg
            border-gray-300
            dark:border-gray-700
            focus:ring-indigo-400 
            focus:border-indigo-400
            active:ring-indigo-400 
            active:border-indigo-400 
            dark:active:ring-indigo-400 
            dark:active:border-indigo-400 
            dark:focus:ring-indigo-400 
            dark:focus:border-indigo-400
            focus:outline-blue-200/20
            aria-invalid:border-danger
            aria-invalid:dark:border-danger
            aria-invalid:dark:bg-danger-bg
            aria-invalid:focus:ring-danger 
            aria-invalid:focus:border-danger
            aria-invalid:active:ring-danger 
            aria-invalid:active:border-danger
            aria-invalid:focus:outline-danger/20
            aria-invalid:dark:focus:ring-danger 
            aria-invalid:dark:focus:border-danger 
            aria-invalid:dark:active:ring-danger 
            aria-invalid:dark:active:border-danger"}
                type=input_type
                id=name.to_owned()
                name=name.to_owned()
                value=value
                placeholder=placeholder
            />
        </div>

        {
            let field_name = extract_form_field_name(name.to_owned());
            move || {
                let errors = ui_extract_field_errors(&field_name, validation_errors);
                errors.map(|list| list.into_iter().map(|msg| 
                    view!{ <p class="mt-1 text-xs text-pink-700 dark:text-danger">{msg}</p>}).collect::<Vec<_>>())
            }
        }


    }
}
