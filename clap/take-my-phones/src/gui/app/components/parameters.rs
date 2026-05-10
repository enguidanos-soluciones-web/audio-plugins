use crate::{
    gestures::drag::ActiveDrag,
    gui::app::{dispatcher::Dispatcher, state::AppState},
    parameters::{Parameter, Range, Select, calibration_mode::CalibrationMode, cutoff::Cutoff, mix::Mix, xfeed::XFeed},
    state::GuiRequest,
};
use dioxus::prelude::*;

#[component]
pub fn Parameters() -> Element {
    let state = consume_context::<Signal<AppState>>();
    let dispatcher = consume_context::<Dispatcher>();

    let mut drag = consume_context::<Signal<Option<ActiveDrag>>>();

    let cutoff_val = Parameter::<Cutoff, Range>::format_value(state.read().params[Parameter::<Cutoff, Range>::ID]);
    let xfeed_val = Parameter::<XFeed, Range>::format_value(state.read().params[Parameter::<XFeed, Range>::ID]);
    let mix_val = Parameter::<Mix, Range>::format_value(state.read().params[Parameter::<Mix, Range>::ID]);
    let cal_value = state.read().params[Parameter::<CalibrationMode, Select>::ID].round() as u8;

    rsx! {
        div {
            class: "flex flex-col",

            div {
                class: "flex-1 flex items-center justify-center gap-10 py-8",

                div {
                    class: "flex flex-col items-center gap-2.5",
                    span { class: "text-amber-500 text-sm", "{cutoff_val} Hz" }
                    div {
                        id: "cutoff",
                        class: "w-20 h-20",
                        onmousedown: {
                            let state = state.clone();
                            let dispatcher = dispatcher.clone();
                            move |e| {
                                dispatcher(GuiRequest::BeginGesture(Parameter::<Cutoff, Range>::ID));
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
                    span { class: "text-amber-500 text-sm", "{xfeed_val} dB" }
                    div {
                        id: "xfeed",
                        class: "w-20 h-20",
                        onmousedown: {
                            let state = state.clone();
                            let dispatcher = dispatcher.clone();
                            move |e| {
                                dispatcher(GuiRequest::BeginGesture(Parameter::<XFeed, Range>::ID));
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
                    span { class: "text-xs font-semibold tracking-widest uppercase text-neutral-400", "Crossfeed" }
                }

                div {
                    class: "flex flex-col items-center gap-2.5",
                    span { class: "text-amber-500 text-sm", "{mix_val} %" }
                    div {
                        id: "mix",
                        class: "w-20 h-20",
                        onmousedown: {
                            let state = state.clone();
                            let dispatcher = dispatcher.clone();
                            move |e| {
                                dispatcher(GuiRequest::BeginGesture(Parameter::<Mix, Range>::ID));
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

                // CalibrationMode — dropdown: one button per option
                div {
                    class: "flex flex-col items-center gap-2.5",
                    span { class: "text-xs font-semibold tracking-widest uppercase text-neutral-400", "Calibration" }
                    div {
                        class: "flex flex-col gap-1",
                        for option in Parameter::<CalibrationMode, Select>::new().behave.options {
                            {
                                let option = *option;
                                let is_active = cal_value == option;
                                let dispatcher = dispatcher.clone();
                                rsx! {
                                    button {
                                        class: if is_active {
                                            "px-3 py-1 text-xs rounded bg-amber-400 text-neutral-900 font-semibold"
                                        } else {
                                            "px-3 py-1 text-xs rounded bg-neutral-700 text-neutral-300 hover:bg-neutral-600"
                                        },
                                        onclick: move |_| dispatcher(GuiRequest::SetParam(
                                            Parameter::<CalibrationMode, Select>::ID,
                                            option as f64,
                                        )),
                                        { CalibrationMode::label(option) }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } // end controls row

        // Calibration guide
        div {
            class: "px-8 py-3 border-t border-neutral-800 text-neutral-500 text-xs leading-relaxed",
            p {
                class: "font-semibold text-neutral-400 mb-1",
                "Calibration guide"
            }
            p { "① Put on headphones. Set Calibration → Continuous. You will hear pink noise in your left ear only — the right ear receives only the crossed signal through bs2b. Adjust Cutoff to control which frequencies bleed to the right, and XFeed to control how much. Find a natural-sounding balance." }
            p { "② Switch to Intermittent. Pink noise alternates between L and R every 500 ms. Adjust XFeed until the bleed from the active side to the silent side feels right for your head anatomy." }
            p { "③ Set Calibration → Off. Your personal bs2b settings are ready." }
        }
    }
}
