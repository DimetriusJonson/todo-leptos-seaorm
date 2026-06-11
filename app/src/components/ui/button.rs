use leptos::{ev::MouseEvent, html, prelude::*};

#[derive(Default)]
pub enum ButtonColor {
    #[default]
    Primary,
    Light,
    Danger,
}

#[derive(Default)]
pub enum ButtonTextSize {
    Sm,
    #[default]
    Md,
}

#[derive(Default)]
pub enum ButtonWidth {
    #[default]
    Auto,
    Md,
}

#[component]
pub fn Button(
    #[prop(optional)] id: i32,
    label: String,
    #[prop(optional)] class_name: String,
    #[prop(optional)] color: ButtonColor,
    #[prop(optional)] text_size: ButtonTextSize,
    #[prop(optional)] button_width: ButtonWidth,
    loading: impl Fn() -> bool + Send + Sync + 'static,
    disabled: impl Fn() -> bool + Send + Sync + 'static,
    on_click: impl FnMut(MouseEvent) + 'static,
) -> impl IntoView {
    let loading_memo = Memo::new(move |_| loading());
    let disabled_memo = Memo::new(move |_| disabled());

    let button_element: NodeRef<html::Button> = NodeRef::new();
    let aria_label = label.to_owned();

    let base_classes = "rounded-3xl cursor-pointer font-medium px-6 py-2 h-10 transition-[background-color,border-color,box-shadow,color] duration-294".to_owned();

    let variant_classes = match color {
        ButtonColor::Primary => "bg-primary hover:bg-primary/80 text-black".to_owned(),
        ButtonColor::Light => {
            "bg-gray-200 dark:hover:bg-gray-50 hover:bg-gray-300 text-black".to_owned()
        }
        ButtonColor::Danger => "bg-danger hover:bg-danger/80 text-black".to_owned(),
    };

    let text_size_classes = match text_size {
        ButtonTextSize::Sm => "text-sm".to_owned(),
        ButtonTextSize::Md => "text-base".to_owned(),
    };

    let button_width_classes = match button_width {
        ButtonWidth::Auto => "w-auto".to_owned(),
        ButtonWidth::Md => "w-32".to_owned(),
    };

    view! {
        <button
            node_ref=button_element
            id={id}
            aria-label={aria_label}
            class=move || format!("{} {} {} {} {} {} {}", base_classes, variant_classes, text_size_classes, button_width_classes, 
                match loading_memo.get() {
                    true => "inline-flex justify-center items-center leading-6 transition ease-in-out duration-150".to_owned(),
                    false => "".to_owned(),
                }, 
                match loading_memo.get() || disabled_memo.get() {
                    true => "cursor-not-allowed brightness-110".to_owned(),
                    false => "".to_owned(),
                }, class_name)
            on:click=on_click
            on:mouseup=move |_| if let Some(button) = button_element.get() { button.blur().unwrap(); }
            disabled=disabled_memo
           >

           <Show
                when=move || loading_memo.get()
                fallback=move || view! { {label.to_owned()} }
            >
                <svg class="animate-spin [animation-duration:500ms] h-5 w-5 text-black" xmlns="http://w3.org" fill="none" viewBox="0 0 24 24">
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                "\u{00A0}"
            </Show>

        </button>
    }
}
