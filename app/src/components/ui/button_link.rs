use leptos::prelude::*;

#[derive(Default)]
pub enum ButtonLinkColor {
    Ghost,
    #[default] 
    Light,
     Black, 
     Brown,
     Primary
}

#[derive(Default)]
pub enum ButtonLinkTextSize {
    Sm,
    #[default] 
    Md,
}

#[derive(Default)]
pub enum ButtonLinkWidth {
    Auto,
    #[default] 
    Md,
}

#[component]
pub fn ButtonLink(
    #[prop(optional)] id: i32,
    label: String,
    href: String,
    #[prop(optional)] color: ButtonLinkColor,
    #[prop(optional)] text_size: ButtonLinkTextSize,
    #[prop(optional)] button_width: ButtonLinkWidth,
    #[prop(optional)] class_name: String,
) -> impl IntoView {

    let base_classes = "rounded-3xl font-medium px-6 py-2 text-center transition-[background-color,border-color,box-shadow,color] duration-294".to_owned();

    let variant_classes = match color {
        ButtonLinkColor::Ghost => "text-link dark:text-link-dark".to_owned(),
        ButtonLinkColor::Light => "bg-gray-200 dark:hover:bg-gray-50 hover:bg-gray-300 text-black".to_owned(),
        ButtonLinkColor::Black => "bg-black hover:bg-gray-900 text-white".to_owned(),
        ButtonLinkColor::Brown => "bg-yellow-900/80 hover:bg-yellow-700 text-gray-50".to_owned(),
        ButtonLinkColor::Primary => "bg-primary hover:bg-primary/80 text-black".to_owned(),
    };

    let text_size_classes = match text_size {
        ButtonLinkTextSize::Sm => "text-sm".to_owned(),
        ButtonLinkTextSize::Md => "text-base".to_owned(),
    };

    let button_width_classes = match button_width {
        ButtonLinkWidth::Auto => "w-auto".to_owned(),
        ButtonLinkWidth::Md => "w-32".to_owned(),
    };

    let aria_label = label.to_owned();
    view! {
        <a id=id aria-label=aria_label href=href
            class=format!("{} {} {} {} {}", base_classes, variant_classes, text_size_classes, button_width_classes, class_name)>
            {label}
        </a>
    }
}
