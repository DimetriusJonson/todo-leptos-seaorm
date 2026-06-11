use std::time::Duration;

use leptos::prelude::*;
use web_sys::HtmlElement;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
enum MessageBannerState {
    InShow,
    InHide,
    #[default]
    None,
}

#[derive(Debug, Clone, Default)]
struct MessageBannerItem {
    pub id: String,
    pub msg: String,
    pub kind: String,
    pub state: MessageBannerState,
}

#[derive(Clone, Copy)]
pub struct Messages(RwSignal<Vec<MessageBannerItem>>);

#[component]
pub fn MessageBanner() -> impl IntoView {
    let messages = Messages(RwSignal::new(Vec::<MessageBannerItem>::new()));
    provide_context(messages);

    view! {

        <div
            class="text-center py-3"
            style:position="fixed"
            style:left="0"
            style:bottom="1.5rem"
            style:width="100%"
            style:z-index="1000"
        >
            {
                move || messages.0.get().into_iter()
                .map(|msg| {
                    let msg_state = msg.state.clone();

                    view! {
                        <p id={format!("m_{}", msg.id)}
                            class="mb-4"
                            class:messagebanner=move || msg_state == MessageBannerState::InHide
                            class:messagebanner-show=move || msg.state == MessageBannerState::InShow
                            on:transitionend= move |event| {
                                let id_str = event_target::<HtmlElement>(&event).id().to_string();
                                if let Some(pos) = id_str.find('_') {
                                    let id = &id_str[pos + 1..];
                                    let mut need_delete = false;
                                    for msg in messages.0.write().iter_mut() {
                                        if msg.id == id {
                                            match msg.state {
                                                MessageBannerState::InShow => msg.state = MessageBannerState::None,
                                                MessageBannerState::InHide => need_delete = true,
                                                MessageBannerState::None => (),
                                            }
                                            break;
                                        }
                                    }

                                    if need_delete {
                                        let mut new_list = messages.0.get();
                                        new_list.retain(|m| m.id != msg.id);
                                        messages.0.set(new_list);
                                    }
                                }
                            }
                        >
                            <span class={format!("inline-flex items-center justify-center px-2.5 py-1 text-base rounded text-black space-x-2 {}", msg_style(&msg))}>
                                <span class="pr-2">{msg.msg.to_owned()}</span>
                                <button
                                    aria-label="x"
                                    class="items-center justify-right size-4 rounded-full bg-black/20 hover:bg-black/30 text-white text-xs cursor-pointer"
                                    id={format!("m_{}", msg.id)}
                                    on:click={move |event| {
                                        let id_str = event_target::<HtmlElement>(&event).id().to_string();
                                        if let Some(pos) = id_str.find('_') {
                                            set_message_state(MessageBannerState::InHide, &id_str[pos + 1..], messages);
                                        }
                                    }}
                                >"\u{00D7}"</button>
                            </span>
                        </p>
                    }
                }).collect::<Vec<_>>()}
        </div>
    }
}

pub fn show_info(msg: String, messages: Messages) {
    show_message(msg, "INFO".to_string(), Duration::from_millis(5000), messages);
}

pub fn show_error(msg: String, messages: Messages) {
    show_message(msg, "ERROR".to_string(), Duration::from_millis(30000), messages);
}

pub fn show_server_error(err: ServerFnError, messages: Messages) {
    let msg = match err {
        ServerFnError::ServerError(msg) => msg,
        _ => err.to_string(),
    };

    show_message(msg, "ERROR".to_string(), Duration::from_millis(30000), messages);
}

fn show_message(msg: String, kind: String, active_time: Duration, messages: Messages) {
    use uuid::Uuid;

    let id = Uuid::new_v4().to_string();
    messages.0.write().push(MessageBannerItem {
        id: id.to_owned(),
        msg,
        kind,
        state: MessageBannerState::InHide,
    });

    let cloned_id = id.to_owned();
    set_timeout(
        move || set_message_state(MessageBannerState::InHide, &cloned_id, messages),
        active_time,
    );

    set_timeout(
        move || set_message_state(MessageBannerState::InShow, &id, messages),
        Duration::from_millis(50),
    );
}

fn set_message_state(state: MessageBannerState, id: &str, messages: Messages) {
    for msg in messages.0.write().iter_mut() {
        if msg.id == id {
            msg.state = state;
            break;
        }
    }
}

fn msg_style(msg: &MessageBannerItem) -> String {
    if msg.kind == "INFO" {
        return "bg-primary".to_owned();
    }

    "bg-danger".to_owned()
}
