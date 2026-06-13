use leptos::prelude::*;

pub type SelectOption = (Option<String>, String);

#[component]
pub fn TextArea(
    name: String,
    #[prop(optional)] value: String,
    placeholder: String,
    #[prop(into)] on_change: Callback<String>,
) -> impl IntoView {
    view! {
        <textarea
            class="textblock w-full px-3 py-2 rounded-md shadow-inner
        focus:outline-4
        border
        text-gray-700 
        placeholder-gray-400 
        hover:ring-gray-400
        hover:border-gray-400
        bg-white
        border-gray-300
        focus:ring-indigo-400 
        focus:border-indigo-400
        active:ring-indigo-400 
        active:border-indigo-400 
        focus:outline-blue-200/20

        transition-[background-color,border-color,box-shadow,color]
        duration-294

        dark:text-gray-50 
        dark:placeholder-gray-400 
        dark:hover:ring-gray-500
        dark:hover:border-gray-500
        dark:bg-dark-bg
        dark:border-gray-700
        dark:active:border-indigo-400 
        dark:focus:ring-indigo-400 
        dark:focus:border-indigo-400
        dark:active:ring-indigo-400 
        
        disabled:text-weak
        disabled:bg-disabled-bg
        disabled:dark:border-bg-dark-bg
        disabled:border-bg-white
        disabled:placeholder:text-gray-500/30
        "
            rows="4"
            cols="50".to_ascii_lowercase()
            name=name
            placeholder=placeholder
            on:change=move |ev| {
                let val = event_target_value(&ev);
                on_change.run(val)
            }
        >{value}</textarea>
    }
}
