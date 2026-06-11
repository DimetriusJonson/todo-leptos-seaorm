use leptos::prelude::*;

#[component]
pub fn MainTitle(
    title: impl Fn() -> String + Send + Sync + 'static,
    #[prop(optional)] class_name: String,
) -> impl IntoView {
    view! {
        <h1 class={format!("text-3xl font-monospace font-extrabold text-gray-800 dark:text-gray-200 tracking-tight pb-5 {}", class_name)}>
            {title}
        </h1>
    }
}
