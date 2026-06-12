use std::collections::HashMap;

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use validator::Validate;

use crate::common::validate_helper::{
    ui_build_common_error, ui_build_validation_errors, validate_form, validation_errors_to_map,
};
use crate::components::layout::message_banner::{Messages, show_info};
use crate::components::ui::button::{Button, ButtonWidth};
use crate::components::ui::button_link::ButtonLink;
use crate::components::ui::checkbox_with_label::CheckboxWithLabel;
use crate::components::ui::main_title::MainTitle;
use crate::components::ui::select_with_error::SelectWithError;
use crate::components::ui::text_area::TextArea;
use crate::components::ui::text_with_error::TextWithError;
use crate::domain::home::routing::routes::HomeRoutes;
use crate::domain::task::model::task::Task;
use crate::domain::task::routing::routes::TaskRoutes;
use crate::domain::task::task_services::{UpdateOrCreateTask, get_priorities, get_task};

#[component]
pub fn TaskEditPage() -> impl IntoView {
    let params = use_params_map();

    let task_resource = Resource::new_blocking(
        move || params.read().get("id"),
        async move |id| { 
            match id {
                Some(id) => get_task(id.parse().expect("Failed parse id")).await,
                None => Ok(Task::default()),
            }
        }
    );
    let priorities_resource = OnceResource::new(get_priorities());

    view! {
        <div class="container mx-auto p-4">
            <MainTitle title=move || match params.read().get("id") {
                Some(_) => "Редактировать задачу".to_owned(),
                None => "Создать задачу".to_owned(),
            } />
            <Transition fallback=move || view! { <TaskEditForm task={Task::default()} priorities={None} disabled=true /> }>
                {move || Suspend::new(async move {
                    let priorities = priorities_resource.await.ok();
                    if params.read().get("id").is_some() {
                        task_resource.get().map(|data| data.map(|task| view! { <TaskEditForm task priorities disabled=false /> }))
                    } else {
                        Some(Ok(view! { <TaskEditForm task={Task::default()} priorities disabled=false /> }))
                    }
                })}
            </Transition>
        </div>
    }
}

#[component]
pub fn TaskEditForm(
    task: Task,
    priorities: Option<Vec<(Option<String>, String)>>,
    disabled: bool,
) -> impl IntoView {
    let update_or_create_task = ServerAction::<UpdateOrCreateTask>::new();

    let (errors, set_validation_errors) = signal(HashMap::<String, Vec<String>>::new());

    let validation_errors: Signal<HashMap<String, Vec<String>>> = Signal::derive(move || {
        let mut result = errors.get();
        result.extend(update_or_create_task.value().with(ui_build_validation_errors));
        result
    });
    let common_error = move || ui_build_common_error(validation_errors);

    let messages = use_context::<Messages>().expect("Cant get messages context!");

    Effect::new(move |_| {
        if let Some(Ok(_)) = update_or_create_task.value().get() {
            show_info("Задача сохранена!".to_owned(), messages);
            update_or_create_task.clear();
        }
    });

    view! {
        <ActionForm action=update_or_create_task
            on:submit:capture=move |event| {
                if let Ok(params) = UpdateOrCreateTask::from_event(&event) {
                    if let Err(validation_errors) = params.validate() {
                        set_validation_errors.set(validation_errors_to_map(validation_errors));
                        event.prevent_default();
                    }
                } else {
                    event.prevent_default();
                }
            }
            on:input=move |event| {
                    validate_form(event, set_validation_errors, Task::default());
                    update_or_create_task.clear();
                }
        >
            <input type="hidden" name="task[id]" value=task.id />

            <div class="text-danger text-bold py-2">{common_error}</div>

            <fieldset disabled={move || { disabled || update_or_create_task.pending().get()}}>
                <div class="flex flex-col md:flex-row justify-between items-center mb-4">
                    <div class="flex items-center justify-start gap-4">
                        <div class="flex items-center justify-center">
                            <SelectWithError
                                name="task[priority]".to_owned()
                                label="Приоритет:".to_owned()
                                validation_errors
                                options=priorities.unwrap_or_default()
                                not_selected_text="Не выбран".to_owned()
                                value=task.priority.unwrap_or_default()
                                on_change=|_| {}
                            />
                        </div>
                    </div>

                    <div class="flex items-center justify-end gap-4">
                        <div class="flex items-center justify-center shrink-0">
                            <CheckboxWithLabel
                                name="task[completed_at]".to_owned()
                                value=task.completed_at.is_some()
                                label="Завершена".to_owned()
                            />
                        </div>
                    </div>

                </div>
                <div class="mb-4">
                    <TextWithError
                        input_type="text".to_owned()
                        name="task[title]".to_owned()
                        placeholder="Название".to_owned()
                        validation_errors
                        value=task.title.unwrap_or_default()
                    />
                </div>

                <div>
                    <TextArea
                        name="task[description]".to_owned()
                        placeholder="Описание".to_owned()
                        value=task.description.unwrap_or_default()
                        on_change=|_| {}
                    />
                </div>

                <div class="flex flex-wrap items-center gap-2 pt-2">
                    <Button
                        label="Сохранить".to_owned()
                        button_width=ButtonWidth::Md
                        loading=move || update_or_create_task.pending().get()
                        on_click=move |_| {}
                        disabled=move || update_or_create_task.pending().get()
                    />
                    <ButtonLink
                        label="Отмена".to_owned()
                        href=match task.id {
                            Some(id) => TaskRoutes::details_url(id),
                            None => HomeRoutes::base_url().to_owned(),
                        }.to_owned()
                    />
                </div>

            </fieldset>
        </ActionForm>


    }
}
