use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::layout::message_banner::{Messages, show_info, show_server_error};
use crate::components::ui::button::Button;
use crate::components::ui::button::ButtonColor::Danger;
use crate::components::ui::button_link::ButtonLink;
use crate::components::ui::button_link::ButtonLinkColor::Light;
use crate::domain::task::model::task::Task;
use crate::domain::task::routing::routes::TaskRoutes;
use crate::domain::task::task_services::{DeleteTask, get_task};

#[component]
pub fn TaskPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.read().get("id").unwrap_or_default();

    let task_resource =
        Resource::new_blocking(id, async move |id| get_task(id.parse().unwrap_or(0)).await);

    view! {
        <div class="container mx-auto p-4">
            // Message
            <div class="rounded-md bg-neutral-100 dark:bg-neutral-800 text-sm overflow-hidden border border-neutral-700">
                // Message Header
                <div class="flex items-center justify-between bg-neutral-800 dark:bg-neutral-700 px-5 py-3 font-bold text-white dark:text-neutral-300">
                    <p>{"Сделать"}</p>
                </div>

                // Message Body
                <div class="border-t border-neutral-700 px-5 py-4 text-neutral-800 dark:text-neutral-100">

                    <Transition
                        fallback=move || view! { <TaskDetails task=Task {title: Some("...".to_owned()), description: Some("...".to_owned()), ..Task::default()} /> }
                        >
                        {move || task_resource.get().map(|data| {
                            data.map(|task| view! { <TaskDetails task /> })
                        })}
                    </Transition>

                </div>
            </div>
        </div>

    }
}

#[component]
pub fn TaskDetails(task: Task) -> impl IntoView {
    let delete_task = ServerAction::<DeleteTask>::new();

    let messages = use_context::<Messages>().expect("Cant get messages context!");

    Effect::new(move |_| match delete_task.value().get() {
        Some(res) => match res {
            Ok(_) => {
                show_info("Задача удалена!".to_owned(), messages);
                delete_task.clear();
            }
            Err(err) => show_server_error(err, messages),
        },
        None => (),
    });

    view! {
        // media
        <div class="flex items-start gap-4 font-monospace font-bold">
            // media-left
            <div class="text-4xl pt-2">
                {
                    if task.completed_at.is_some() {
                        view! {<span >{"✅"}</span>}.into_any()
                    } else {
                        view! {<span >{"❌"}</span>}.into_any()
                    }
                }
            </div>
            <div>
                <p class="text-2xl">
                    {task.title.to_owned()}
                </p>
                <p class="text-base text-gray-700 dark:text-gray-300">
                    {task.priority_name()}
                </p>
            </div>
        </div>

        <div class="prose pt-4 text-gray-700 dark:text-gray-300">
            <p>{task.description.to_owned()}</p>
        </div>

        <div class="flex flex-wrap items-center gap-2 pt-4">
            <ButtonLink
                color=Light
                href={TaskRoutes::edit_url(task.id.unwrap_or_default())}
                label="Изменить".to_owned()
            />

            <ActionForm action=delete_task>
                <input type="hidden" name="id" value={task.id.unwrap_or_default()} />
                <Button
                    color=Danger
                    label="Удалить".to_owned()
                    loading=move || delete_task.pending().get()
                    disabled=move || delete_task.pending().get()
                    on_click=move |_| {}
                />
            </ActionForm>

        </div>

    }
}
