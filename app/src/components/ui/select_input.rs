use leptos::prelude::*;

pub type SelectOption = (Option<String>, String);

#[component]
pub fn SelectInput(
    name: String,
    #[prop(optional)] value: String,
    #[prop(optional)] class_name: String,
    label: String,
    #[prop(optional)] not_selected_text: String,
    options: Vec<SelectOption>,
    #[prop(into)] on_change: Callback<String>
) -> impl IntoView {
    view! {
        <div class={class_name}>
            <select aria-label={label}
                id = {name.to_owned()}
                class={"border rounded-lg block w-full p-2
            focus:outline-4

            transition-[background-color,border-color,box-shadow,color]
            duration-294

            focus:outline-blue-400/20
            dark:focus:outline-blue-200/20
            bg-white
            border-gray-300
            dark:bg-dark-bg
            dark:text-gray-50 
            dark:border-gray-700 
            hover:ring-gray-400
            hover:border-gray-400
            dark:hover:ring-gray-500
            dark:hover:border-gray-500
            focus:ring-indigo-400 
            focus:border-indigo-400
            active:ring-indigo-400 
            active:border-indigo-400 
            dark:active:ring-indigo-400 
            dark:active:border-indigo-400 
            dark:focus:ring-indigo-400 
            dark:focus:border-indigo-400

            disabled:text-weak
            disabled:bg-disabled-bg
            disabled:dark:border-bg-dark-bg
            disabled:border-bg-white
            disabled:placeholder:text-gray-500/30

            "}
                name = {name}
                prop:value = {value.to_owned()}
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    on_change.run(val)
                }
            >
                <option class="dark:bg-dark-bg" value={""}>{not_selected_text}</option>

                {
                    options.into_iter()
                    .map(|option| view! { 
                        <option class="dark:bg-dark-bg" value={option.0.to_owned()} selected={option.0 == Some(value.to_owned())}>{option.1}</option>
                    }).collect::<Vec<_>>()
                }

            </select>
        </div>
    }
}
