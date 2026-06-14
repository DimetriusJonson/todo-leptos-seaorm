use leptos::prelude::*;

pub type SelectOption = (Option<String>, String);

#[component]
pub fn CheckboxWithLabel(
    name: String,
    label: String,
    value: bool,
    #[prop(optional)] class_name: String,
    title: impl Fn() -> String + Send + Sync + 'static,
) -> impl IntoView {
    view! {
        <div class="flex relative items-center">
            <input class={format!("peer h-8 w-8 cursor-pointer appearance-none rounded border-2
            border-slate-500
            dark:bg-dark-bg 
            bg-white
            transition-colors 
            checked:border-emerald-500 
            checked:bg-emerald-500 
            checked:hover:border-emerald-600 
            checked:hover:bg-emerald-600 
            focus:outline-none 
            checked:focus:border-emerald-700 
            checked:focus:bg-emerald-700 
            checked:focus:ring-emerald-300 
            focus-visible:outline-none 
            disabled:cursor-not-allowed {}", class_name)} 
            id={name.to_owned()}
            type="checkbox"
            checked=value
            name={name.to_owned()} 
            title=title
            />
            <label class="h-full cursor-pointer pl-2 text-gray-700 dark:text-gray-50 peer-disabled:cursor-not-allowed peer-disabled:text-slate-400" for={name.to_owned()}>{label}</label>
            <svg class="pointer-events-none absolute left-0 top-1 h-8 w-8 -rotate-90 fill-white stroke-white opacity-0 transition-all duration-300 peer-checked:rotate-0 peer-checked:opacity-100 peer-disabled:cursor-not-allowed" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true" role="graphics-symbol">
            <path fill-rule="evenodd" clip-rule="evenodd" d="M12.8116 5.17568C12.9322 5.2882 13 5.44079 13 5.5999C13 5.759 12.9322 5.91159 12.8116 6.02412L7.66416 10.8243C7.5435 10.9368 7.37987 11 7.20925 11C7.03864 11 6.87501 10.9368 6.75435 10.8243L4.18062 8.42422C4.06341 8.31105 3.99856 8.15948 4.00002 8.00216C4.00149 7.84483 4.06916 7.69434 4.18846 7.58309C4.30775 7.47184 4.46913 7.40874 4.63784 7.40737C4.80655 7.406 4.96908 7.46648 5.09043 7.57578L7.20925 9.55167L11.9018 5.17568C12.0225 5.06319 12.1861 5 12.3567 5C12.5273 5 12.691 5.06319 12.8116 5.17568Z" />
            </svg>
        </div>

    }
}
