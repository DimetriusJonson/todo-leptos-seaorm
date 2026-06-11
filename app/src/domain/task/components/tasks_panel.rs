use leptos::prelude::*;
use leptos_router::components::Form;
use leptos_router::hooks::use_query_map;

use crate::components::ui::button::Button;
use crate::components::ui::button::ButtonColor::Light;
use crate::components::ui::button_link::ButtonLinkWidth::Auto;
use crate::components::ui::button_link::{ButtonLink, ButtonLinkColor};
use crate::components::ui::select_input::SelectInput;
use crate::domain::task::components::tasks_list::TasksList;
use crate::domain::task::model::task::sort_task;
use crate::domain::task::routing::routes::TaskRoutes;
use crate::domain::task::task_services::{get_filter_options, get_sort_options, get_tasks};
use crate::domain::user::model::user::User;

#[component]
pub fn TasksPanel() -> impl IntoView {
    let (filter, set_filter) = signal(Some("".to_owned()));

    let user_resource = use_context::<Resource<Result<User, ServerFnError>>>().unwrap();

    let query_map = use_query_map();
    let url_params =
        move || query_map.with(|m| (m.get("auth"), m.get("filter"), m.get("sort_kind")));

    let tasks_resource =
        Resource::new_blocking(url_params, move |params| get_tasks(params.1, params.2));

    provide_context(tasks_resource);

    let filter_options_resource = OnceResource::new(get_filter_options());
    let sort_options_resource = OnceResource::new(get_sort_options());

    view! {
        <div class="container mx-auto pt-5 dark:text-gray-50 text-xs md:text-base">
            <Form method="GET" action="">
                <div class="flex justify-between pb-4">
                    <Transition>
                        {move || Suspend::new(async move {
                            let filter_options = filter_options_resource.await.ok();
                            let sort_options = sort_options_resource.await.ok();
                            view! {
                                    <span class="flex space-x-4">
                                        <SelectInput
                                            name="filter".to_owned()
                                            label="Фильтр".to_owned()
                                            not_selected_text="Фильтр".to_owned()
                                            options=filter_options.unwrap()
                                            on_change=move |value: String| {
                                                set_filter
                                                    .set(if value.is_empty() { None } else { Some(value.to_owned()) });
                                            }
                                        />
                                        <SelectInput
                                            name="sort_kind".to_owned()
                                            label="Сортировка".to_owned()
                                            not_selected_text="Сортировка".to_owned()
                                            options=sort_options.unwrap()
                                            on_change=move |value: String| {
                                                let sort_kind = if value.is_empty() { None } else { Some(value) };
                                                if let Some(data) = tasks_resource.write().as_mut() && let Ok(tasks) = data {
                                                    tasks.sort_by(|task1, task2| sort_task(task1, task2, &sort_kind));
                                                    }
                                            }
                                        />

                                        <noscript>
                                            <Button
                                                color=Light
                                                class_name="text-xs md:text-base".to_owned()
                                                label="Ok".to_owned()
                                                loading=move || false
                                                on_click=move |_| {}
                                                disabled=move || false
                                            />
                                        </noscript>
                                    </span>

                                    {move || user_resource.get().map(|data| {
                                        let user = data.ok().unwrap_or_default();
                                        if user.username.is_some() {
                                            view! {
                                                <ButtonLink
                                                    color=ButtonLinkColor::Light
                                                    button_width=Auto
                                                    class_name="mr-2 text-xs md:text-base".to_owned()
                                                    href=TaskRoutes::create_url().to_owned()
                                                    label="+".to_owned()
                                                />
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }
                                    })}


                            }.into_any()
                        })}
                    </Transition>

                </div>
            </Form>

            <TasksList filter />

        </div>
    }
}
