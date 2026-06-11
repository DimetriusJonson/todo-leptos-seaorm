use leptos::prelude::*;

use crate::components::layout::message_banner::{Messages, show_info, show_server_error};
use crate::components::ui::button::Button;
use crate::components::ui::button::ButtonColor::Light;
use crate::components::ui::button_link::ButtonLink;
use crate::components::ui::button_link::ButtonLinkColor::{Black, Brown};
use crate::components::ui::button_link::ButtonLinkWidth::Auto;
use crate::domain::home::routing::routes::HomeRoutes;
use crate::domain::user::model::user::User;
use crate::domain::user::routing::routes::UserRoutes;
use crate::domain::user::user_services::Logout;

#[component]
pub fn Navbar() -> impl IntoView {
    let (nav_links_active, set_nav_links_active) = signal(false);

    view! {
        <style lang="css">
            r#"
                @media screen and (max-width: 1024px) and (scripting: none) {
                    #mobile-menu {
                        display: block;
                    }
                    #desktop-menu {
                        display: none;
                    }
                }
            "#
        </style>


        <nav class="w-full relative bg-primary">

            <div class="px-1 py-1 sm:px-2 lg:px-4">
                <div class="flex justify-between h-14">
                    // Brand / Logo Area
                    <div class="shrink-0 flex items-center">
                        <a href=HomeRoutes::base_url() class="text-4xl font-extrabold text-gray-800 pr-2 font-mono">TODO</a>
                        <ButtonLink label="Пользователи".to_owned() href="/users".to_owned() color=Black button_width=Auto/>
                    </div>

                    // Right-side Menu Wrapper (Flexbox items)
                    <div class="flex items-center">
                        // Mobile Burger Button
                        <button
                            id="mobile-menu-button"
                            type="button"
                            class="md:hidden inline-flex items-center justify-center p-2 rounded-md text-gray-500 hover:text-gray-700 hover:bg-yellow-700/20 bg-yellow-500 focus:outline-none"
                            aria-controls="mobile-menu"
                            aria-expanded=nav_links_active
                            on:click=move |_event| set_nav_links_active.set(!nav_links_active.get())
                        >
                            <span class="sr-only">Open main menu</span>
                            // Burger Icon SVG
                            <svg
                                class="h-6 width-6"
                                xmlns="http://w3.org"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M4 6h16M4 12h16M4 18h16"
                                />
                            </svg>
                        </button>

                        // Desktop Navigation Links (Hidden on mobile)
                        <div id="desktop-menu" class="hidden md:flex md:space-x-4 md:items-center">
                            <UserButtons />
                        </div>
                    </div>
                </div>
            </div>

            // Mobile Menu Dropdown (Controlled via JavaScript)
            <div
                id="mobile-menu"
                class=move || {format!("md:hidden bg-primary left-0 right-0 top-fu z-50 {}", match nav_links_active.get(){
                    true => "",
                    false => "hidden",
                })}
            >
                <div class="py-4">
                    <UserButtons />
                </div>
            </div>
        </nav>

    }
}


#[component]
fn UserButtons() -> impl IntoView {
    let logout = ServerAction::<Logout>::new();
    let messages = use_context::<Messages>().expect("Cant get messages context!");
    let user_resource = use_context::<Resource<Result<User, ServerFnError>>>().unwrap();

    Effect::new(move |_| {
        if let Some(res) = logout.value().get() {
            match res {
                Ok(_) => {
                    show_info("Вы вышли!".to_owned(), messages);
                    logout.clear();
                }
                Err(err) => show_server_error(err, messages),
            }
        }
    });
    

    view! {

        <Transition>
            {move || user_resource.get().map(|data| {
                let user = data.ok().unwrap_or_default();
                if let Some(user_name)=user.username {
                        view! {
                            <div class="flex items-center pl-2 py-2">
                                <ActionForm action=logout>
                                    <Button
                                        color=Light
                                        class_name="ml-2".to_owned()
                                        label={format!("Выйти {}", user_name)}
                                        loading=move || logout.pending().get()
                                        disabled=move || logout.pending().get()
                                        on_click=move |_| {}
                                    />
                                </ActionForm>
                            </div>
                        }.into_any()
                } else {
                        view! {
                            <div class="flex items-center pl-2 py-2">
                                    <ButtonLink
                                        label="Создать пользователя".to_owned()
                                        href=UserRoutes::create_url().to_owned()
                                        color=Brown
                                        button_width=Auto
                                    />
                                </div>
                                <div class="flex items-center pl-2 py-2">
                                    <ButtonLink
                                        label="Войти".to_owned()
                                        href=UserRoutes::login_url().to_owned()
                                    />
                                </div>
                        }.into_any()
                }
            })}
        </Transition>
    }

}