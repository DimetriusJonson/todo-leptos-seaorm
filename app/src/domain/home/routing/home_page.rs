use leptos::prelude::*;

use crate::domain::task::components::tasks_panel::TasksPanel;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
    <TasksPanel />
    }
}
