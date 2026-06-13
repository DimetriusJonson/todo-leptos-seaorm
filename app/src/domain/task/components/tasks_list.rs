use leptos::prelude::*;
use leptos::reactive::spawn_local;
use web_sys::{Event, HtmlInputElement};

use crate::components::layout::message_banner::{Messages, show_info, show_server_error};
use crate::components::ui::checkbox::Checkbox;
use crate::domain::task::model::task::{Task, filter_task};
use crate::domain::task::routing::routes::TaskRoutes;
use crate::domain::task::task_services::change_completed_task;

#[component]
pub fn TasksList(filter: ReadSignal<Option<String>>) -> impl IntoView {
    let messages: Messages = use_context::<Messages>().expect("Cant get messages context!");

    let tasks_resource = use_context::<Resource<Result<Vec<Task>, ServerFnError>>>().unwrap();

    let (change_completed_in_progress, set_change_completed_in_progress) = signal(true);
    Effect::new(move |_| {
        set_change_completed_in_progress.set(false);
    });

    let completed_on_change = move |event: Event| {
        event.prevent_default();

        let checkbox = event_target::<HtmlInputElement>(&event);
        let name = checkbox.name();
        let value = checkbox.checked();
        checkbox.set_checked(!value);

        if let Some(index_und) = name.find('_')
            && let Ok(id) = name[index_und + 1..].parse::<i32>()
        {
            spawn_local(async move {
                set_change_completed_in_progress.set(true);
                let res = change_completed_task(id, value).await;
                set_change_completed_in_progress.set(false);
                match res {
                    Ok(saved_task) => {
                        if let Some(Ok(tasks)) = tasks_resource.write().as_mut()
                            && let Some(found_task) = tasks.iter_mut().find(|t| t.id == Some(id))
                        {
                            found_task.completed_at = saved_task.completed_at;
                            show_info(
                                "Задача сохранена.".to_owned(),
                                messages,
                            );
                        }
                    }
                    Err(err) => show_server_error(err, messages)
                }
            });
        }
    };

    view! {
        <table class="table-auto w-full text-left dark:text-gray-200 text-xs md:text-base">
            <thead>
                <tr class="border-b dark:border-gray-600">
                    <th class="px-4 py-4">{"Приоритет"}</th>
                    <th class="px-4">{"Завершена"}</th>
                    <th class="px-4">{"Название"}</th>
                    <th class="px-4 py-4 hidden md:block">{"Описание"}</th>
                </tr>
            </thead>
            <tbody>
            <Transition fallback=move || view! { <tr>
                                <td colSpan="3" class="text-center">
                                    Загрузка...
                                </td>
                            </tr> }>
                {move || tasks_resource.get().map(|data| {
                    let tasks = data.ok().unwrap_or_default();
                    if !tasks.is_empty() {
                        {
                            tasks
                                .into_iter()
                                .filter(|task| filter_task(task, &filter.get()))
                                .map(|task| {
                                    let completed_at = task.completed_at.to_owned();
                                    view! {
                                        <tr class="dark:even:bg-gray-800/30 border-b dark:border-gray-600">
                                            <td class="px-4 py-2">{task.priority_name()}</td>
                                            <td class="px-4 py-2">
                                                <Checkbox
                                                    class_name="is-medium".to_owned()
                                                    name=format!("completed_{}", task.id.unwrap())
                                                    label="Изменить признак завершения".to_owned()
                                                    value=move || task.completed_at.is_some()
                                                    title=move || match completed_at.to_owned() {
                                                        Some(completed_at) => completed_at.to_owned(),
                                                        None => "".to_owned(),
                                                    }
                                                    on:change=completed_on_change
                                                    disabled=move || change_completed_in_progress.get()
                                                />
                                            </td>
                                            <td class="px-4 py-2">
                                                <a class="text-link dark:text-link-dark"
                                                    href=TaskRoutes::details_url(task.id.unwrap())
                                                    aria-label=task.title.to_owned()
                                                >
                                                    {task.title.to_owned()}
                                                </a>
                                            </td>
                                            <td class="px-4 py-2 hidden md:block">{task.description.to_owned()}</td>
                                        </tr>
                                    }
                                }).collect::<Vec<_>>()
                        }.into_any()
                    } else {
                        view! {
                            <tr>
                                <td colSpan="3" class="text-center">
                                    Нет записей
                                </td>
                            </tr>
                        }.into_any()
                    }
                })}
                </Transition>
            </tbody>
        </table>
    }
}
