use std::collections::HashMap;

use leptos::prelude::*;
use validator::Validate;

use crate::common::validate_helper::{
    ui_build_common_error, ui_build_validation_errors, validate_form, validation_errors_to_map,
};
use crate::components::layout::message_banner::{Messages, show_info};
use crate::components::ui::button::{Button, ButtonWidth};
use crate::components::ui::main_title::MainTitle;
use crate::components::ui::text_with_error::TextWithError;
use crate::domain::user::model::create_user_params::CreateUserParams;
use crate::domain::user::user_services::CreateUser;

#[component]
pub fn CreateUserPage() -> impl IntoView {
    let create_user = ServerAction::<CreateUser>::new();

    let messages = use_context::<Messages>().expect("Cant get messages context!");

    let (errors, set_validation_errors) = signal(HashMap::<String, Vec<String>>::new());

    let validation_errors: Signal<HashMap<String, Vec<String>>> = Signal::derive(move || {
        let mut result = errors.get();
        result.extend(create_user.value().with(ui_build_validation_errors));
        result
    });

    let common_error = move || ui_build_common_error(validation_errors);

    Effect::new(move |_| match create_user.value().get() {
        Some(Ok(user)) => {
            show_info(format!("Создан пользователь {}", &user.username.unwrap()), messages);
            create_user.clear();
        },
        _ => (),
    });

    view! {
        <div class="container mx-auto p-4">
            <MainTitle title=|| "Создать пользователя".to_owned() />
            <ActionForm action=create_user
                on:submit:capture=move |event| {
                    if let Ok(params) = CreateUser::from_event(&event) {
                        if let Err(validation_errors) = params.validate() {
                            set_validation_errors.set(validation_errors_to_map(validation_errors));
                            event.prevent_default();
                        }
                    } else {
                        event.prevent_default();
                    }
                }
                on:input=move |event| {
                        validate_form(event, set_validation_errors, CreateUserParams::default());
                        create_user.clear();
                    }
            >
                <input name="params[version]" type="hidden" value={move || create_user.version().get()} />

                <div class="text-danger text-bold py-2">{common_error}</div>

                <fieldset disabled=create_user.pending()>
                    <div class="mb-4">
                        <TextWithError
                            input_type="text".to_owned()
                            name="params[name]".to_owned()
                            placeholder="Имя пользователя".to_owned()
                            validation_errors
                        />
                    </div>

                    <div class="mb-4">
                        <TextWithError input_type="password".to_owned() name="params[password]".to_owned()
                            placeholder="Пароль".to_owned()
                            validation_errors
                        />
                    </div>

                    <div class="flex flex-wrap items-center gap-2">
                        <Button
                            label="Создать".to_owned()
                            button_width=ButtonWidth::Md
                            loading=move || create_user.pending().get()
                            on_click=move |_| {}
                            disabled=move || create_user.pending().get()
                        />
                    </div>
                </fieldset>
            </ActionForm>
        </div>
    }
}
