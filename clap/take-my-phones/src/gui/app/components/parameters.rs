use crate::{
    gestures::drag::ActiveDrag,
    gui::app::{dispatcher::Dispatcher, state::AppState},
    parameters::{Parameter, Range, cutoff::Cutoff, mix::Mix, xfeed::XFeed},
    state::GuiRequest,
};
use dioxus::prelude::*;

#[component]
pub fn Parameters() -> Element {
    let state = consume_context::<Signal<AppState>>();
    let dispatcher = consume_context::<Dispatcher>();

    let mut drag = consume_context::<Signal<Option<ActiveDrag>>>();

    let cutoff_val = Parameter::<Cutoff, Range>::format_value(state.read().params[Parameter::<Cutoff, Range>::ID]);
    let feed_val = Parameter::<XFeed, Range>::format_value(state.read().params[Parameter::<XFeed, Range>::ID]);
    let mix_val = Parameter::<Mix, Range>::format_value(state.read().params[Parameter::<Mix, Range>::ID]);

    rsx! {
        div {
            class: "flex-1 flex items-center justify-center gap-10",

            div {
                class: "flex flex-col items-center gap-2.5",
                span { class: "text-amber-500 text-sm", "{cutoff_val}" }
                div {
                    id: "cutoff",
                    class: "w-20 h-20",
                    onmousedown: {
                        let state = state.clone();
                        move |e| {
                            let coords = e.data().client_coordinates();
                            let raw = state.read().params[Parameter::<Cutoff, Range>::ID];
                            drag.set(ActiveDrag::from_index(Parameter::<Cutoff, Range>::ID, coords.x, coords.y, raw));
                        }
                    },
                    ondoubleclick: {
                        let dispatcher = dispatcher.clone();
                        move |_| dispatcher(GuiRequest::ResetParam(Parameter::<Cutoff, Range>::ID))
                    },
                }
                span { class: "text-xs font-semibold tracking-widest uppercase text-neutral-400", "Cutoff" }
            }

            div {
                class: "flex flex-col items-center gap-2.5",
                span { class: "text-amber-500 text-sm", "{feed_val}" }
                div {
                    id: "feed",
                    class: "w-20 h-20",
                    onmousedown: {
                        let state = state.clone();
                        move |e| {
                            let coords = e.data().client_coordinates();
                            let raw = state.read().params[Parameter::<XFeed, Range>::ID];
                            drag.set(ActiveDrag::from_index(Parameter::<XFeed, Range>::ID, coords.x, coords.y, raw));
                        }
                    },
                    ondoubleclick: {
                        let dispatcher = dispatcher.clone();
                        move |_| dispatcher(GuiRequest::ResetParam(Parameter::<XFeed, Range>::ID))
                    },
                }
                span { class: "text-xs font-semibold tracking-widest uppercase text-neutral-400", "XFeed" }
            }

            div {
                class: "flex flex-col items-center gap-2.5",
                span { class: "text-amber-500 text-sm", "{mix_val}" }
                div {
                    id: "mix",
                    class: "w-20 h-20",
                    onmousedown: {
                        let state = state.clone();
                        move |e| {
                            let coords = e.data().client_coordinates();
                            let raw = state.read().params[Parameter::<Mix, Range>::ID];
                            drag.set(ActiveDrag::from_index(Parameter::<Mix, Range>::ID, coords.x, coords.y, raw));
                        }
                    },
                    ondoubleclick: {
                        let dispatcher = dispatcher.clone();
                        move |_| dispatcher(GuiRequest::ResetParam(Parameter::<Mix, Range>::ID))
                    },
                }
                span { class: "text-xs font-semibold tracking-widest uppercase text-neutral-400", "Mix" }
            }
        }
    }
}
