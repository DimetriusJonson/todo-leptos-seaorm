use std::time::Duration;

use leptos::prelude::*;
use leptos_meta::{Meta, MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::components::{Outlet, ParentRoute, Route, Router, Routes, RoutingProgress};
use leptos_router::hooks::{use_navigate, use_query_map};
use leptos_router::{StaticSegment, path};

use crate::components::layout::message_banner::MessageBanner;
use crate::components::layout::navbar::Navbar;
use crate::components::ui::button_link::ButtonLink;
use crate::components::ui::button_link::ButtonLinkColor::Primary;
use crate::domain::home::routing::home_page::HomePage;
use crate::domain::task::routing::routes::TaskRoutes;
use crate::domain::task::routing::task_edit_page::TaskEditPage;
use crate::domain::task::routing::task_page::TaskPage;
use crate::domain::user::routing::create_user_page::CreateUserPage;
use crate::domain::user::routing::login_page::LoginPage;
use crate::domain::user::routing::routes::UserRoutes;
use crate::domain::user::user_services::auth_data;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="ru">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <meta name="text-scale" content="scale" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body class="dark:bg-dark-bg dark">
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (is_routing, set_is_routing) = signal(false);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/todo_leptos.css" />

        <Title text="TODO leptos app"/>
        <Meta name="keywords" content="todo, leptos, rust, web application, development" />
        <Meta name="description" content="Пример приложения списка дел (TODO) с использованием языка программирования Rust и фреймворка Leptos." />

        <Router set_is_routing>

            {
                let query_map = use_query_map();
                let auth = move || query_map.with(|m| m.get("auth"));
                let user_resource = Resource::new_blocking(auth, |_s| async move { auth_data().await });

                provide_context(user_resource);
            }

            <div class="progress-container pt-0 mt-0">
                <RoutingProgress is_routing max_time=Duration::from_millis(250) />
            </div>

            <section>
                <div>
                    <main>
                        <MessageBanner />

                        <ErrorBoundary fallback=move |errors| {
                            let navigate = use_navigate();

                            let errors_clear = errors.clone();
                            let on_click = move |_| {
                                errors_clear.set(Errors::default());
                                navigate("/", Default::default());
                            };

                            view! {
                                <section class="container mx-auto pt-8">
                                    <div class="bg-neutral-100 dark:bg-gray-950 p-8 rounded-lg shadow-md shadow-danger block text-center">
                                        <div class="text-5xl font-extrabold text-danger">500</div>
                                        <ul class="text-3xl text-gray-400">
                                            {move || errors.get()
                                                .into_iter()
                                                .map(|(_, error)| view! { <li>{format_error(error)}</li> })
                                                .collect::<Vec<_>>()
                                            }
                                        </ul>
                                        <div class="m-5">
                                            <ButtonLink
                                                color=Primary
                                                href="/".to_owned()
                                                label="Вернутся Домой".to_owned()
                                                on:click=on_click
                                            />
                                        </div>
                                    </div>
                                </section>
                            }
                        }>
                            <Navbar />

                            <Routes transition=true fallback=NotFound>
                                <ParentRoute path=path!("/") view=Outlet>

                                    <ParentRoute path=StaticSegment(UserRoutes::base_segment()) view=Outlet>
                                        <Route path=StaticSegment(UserRoutes::create_segment()) view=CreateUserPage />
                                        <Route path=StaticSegment(UserRoutes::login_segment()) view=LoginPage />
                                    </ParentRoute>

                                    <ParentRoute path=StaticSegment(TaskRoutes::base_segment()) view=Outlet>
                                        <Route path=StaticSegment(TaskRoutes::create_segment()) view=TaskEditPage />
                                        <Route path=path!(":id") view=TaskPage />
                                        <Route path=path!(":id/edit") view=TaskEditPage />
                                    </ParentRoute>

                                    <Route path=path!("") view=HomePage />

                                </ParentRoute>

                                //<Route path=path!("/*any") view=NotFound />

                            </Routes>
                        </ErrorBoundary>

                    </main>
                </div>
            </section>
        </Router>
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <section class="container mx-auto pt-8">
            <div class="bg-neutral-100 dark:bg-gray-950 p-8 rounded-lg shadow-md shadow-danger block text-center">
                <div class="text-5xl font-extrabold text-danger">404</div>
                <ul class="text-3xl text-gray-400">
                    <li>Страница не найдена</li>                
                </ul>
            </div>
        </section>
    }
}

fn format_error(error: Error) -> String {
    let msg = error.to_string();

    if let Some(pos) = msg.find('|') { msg[pos + 1..].to_string() } else { error.to_string() }
}
