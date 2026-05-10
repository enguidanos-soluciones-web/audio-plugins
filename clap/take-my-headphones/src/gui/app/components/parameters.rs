use crate::{
    gestures::drag::ActiveDrag,
    gui::app::{components::dropdown::Dropdown, dispatcher::Dispatcher, state::AppState},
    parameters::{
        Parameter, Range, Select, angle::Angle, calibration_mode::CalibrationMode, center::Center, cutoff::Cutoff, lrswap::LRSwap,
        phase::Phase, solo::Solo, xfeed::XFeed,
    },
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
    let angle_val = Parameter::<Angle, Range>::format_value(state.read().params[Parameter::<Angle, Range>::ID]);
    let center_val = Parameter::<Center, Range>::format_value(state.read().params[Parameter::<Center, Range>::ID]);
    let cal_value = state.read().params[Parameter::<CalibrationMode, Select>::ID].round() as usize;
    let lrswap_value = state.read().params[Parameter::<LRSwap, Select>::ID].round() as usize;
    let solo_value = state.read().params[Parameter::<Solo, Select>::ID].round() as usize;
    let phase_value = state.read().params[Parameter::<Phase, Select>::ID].round() as usize;

    let cal_options: Vec<String> = Parameter::<CalibrationMode, Select>::new()
        .behave
        .options
        .iter()
        .map(|&v| CalibrationMode::label(v).to_string())
        .collect();
    let lrswap_options: Vec<String> = Parameter::<LRSwap, Select>::new()
        .behave
        .options
        .iter()
        .map(|&v| LRSwap::label(v).to_string())
        .collect();
    let solo_options: Vec<String> = Parameter::<Solo, Select>::new()
        .behave
        .options
        .iter()
        .map(|&v| Solo::label(v).to_string())
        .collect();
    let phase_options: Vec<String> = Parameter::<Phase, Select>::new()
        .behave
        .options
        .iter()
        .map(|&v| Phase::label(v).to_string())
        .collect();

    rsx! {
        div {
            class: "flex flex-col gap-8",

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
                    span { class: "text-amber-500 text-sm", "{angle_val}°" }
                    div {
                        id: "angle",
                        class: "w-20 h-20",
                        onmousedown: {
                            let state = state.clone();
                            let dispatcher = dispatcher.clone();
                            move |e| {
                                dispatcher(GuiRequest::BeginGesture(Parameter::<Angle, Range>::ID));
                                let coords = e.data().client_coordinates();
                                let raw = state.read().params[Parameter::<Angle, Range>::ID];
                                drag.set(ActiveDrag::from_index(Parameter::<Angle, Range>::ID, coords.x, coords.y, raw));
                            }
                        },
                        ondoubleclick: {
                            let dispatcher = dispatcher.clone();
                            move |_| dispatcher(GuiRequest::ResetParam(Parameter::<Angle, Range>::ID))
                        },
                    }
                    span { class: "text-xs font-semibold tracking-widest uppercase text-neutral-400", "Angle" }
                }

                div {
                    class: "flex flex-col items-center gap-2.5",
                    span { class: "text-amber-500 text-sm", "{center_val} dB" }
                    div {
                        id: "center",
                        class: "w-20 h-20",
                        onmousedown: {
                            let state = state.clone();
                            let dispatcher = dispatcher.clone();
                            move |e| {
                                dispatcher(GuiRequest::BeginGesture(Parameter::<Center, Range>::ID));
                                let coords = e.data().client_coordinates();
                                let raw = state.read().params[Parameter::<Center, Range>::ID];
                                drag.set(ActiveDrag::from_index(Parameter::<Center, Range>::ID, coords.x, coords.y, raw));
                            }
                        },
                        ondoubleclick: {
                            let dispatcher = dispatcher.clone();
                            move |_| dispatcher(GuiRequest::ResetParam(Parameter::<Center, Range>::ID))
                        },
                    }
                    span { class: "text-xs font-semibold tracking-widest uppercase text-neutral-400", "Center" }
                }

                div {
                    class: "flex flex-col items-center gap-2.5",
                    span { class: "text-xs font-semibold tracking-widest uppercase text-neutral-400", "Calibration" }
                    Dropdown {
                        options: cal_options,
                        selected: Some(cal_value),
                        label: None,
                        on_select: {
                            let dispatcher = dispatcher.clone();
                            move |i: usize| dispatcher(GuiRequest::SetParam(
                                Parameter::<CalibrationMode, Select>::ID,
                                i as f64,
                            ))
                        },
                    }
                }
            }
        }

        div {
            class: "flex items-center justify-center gap-8 pb-4",

            div {
                class: "flex items-center gap-2",
                span { class: "text-neutral-500 text-xs uppercase tracking-widest", "LR Swap" }
                Dropdown {
                    options: lrswap_options,
                    selected: Some(lrswap_value),
                    label: None,
                    on_select: {
                        let dispatcher = dispatcher.clone();
                        move |i: usize| dispatcher(GuiRequest::SetParam(Parameter::<LRSwap, Select>::ID, i as f64))
                    },
                }
            }

            div {
                class: "flex items-center gap-2",
                span { class: "text-neutral-500 text-xs uppercase tracking-widest", "Solo" }
                Dropdown {
                    options: solo_options,
                    selected: Some(solo_value),
                    label: None,
                    on_select: {
                        let dispatcher = dispatcher.clone();
                        move |i: usize| dispatcher(GuiRequest::SetParam(Parameter::<Solo, Select>::ID, i as f64))
                    },
                }
            }

            div {
                class: "flex items-center gap-2",
                span { class: "text-neutral-500 text-xs uppercase tracking-widest", "Phase" }
                Dropdown {
                    options: phase_options,
                    selected: Some(phase_value),
                    label: None,
                    on_select: {
                        let dispatcher = dispatcher.clone();
                        move |i: usize| dispatcher(GuiRequest::SetParam(Parameter::<Phase, Select>::ID, i as f64))
                    },
                }
            }
        }

        div {
            class: "px-8 py-3 border-t border-neutral-800 text-neutral-500 text-xs leading-relaxed",
            p {
                class: "font-semibold text-neutral-400 mb-1",
                "Calibration guide"
            }
            p { "① Calibration → Continuous. Pink noise plays into the left ear only; the right ear receives only the crossed, LP-filtered signal. Set Solo → R: both ears now hear the crossfeed path in isolation with the direct signal removed. Adjust Cutoff until the tone is warm and natural — too high sounds nasal and bright, too low sounds muddy and dark." }
            p { "② Still in Continuous. Set Solo → Off. Adjust XFeed until the blend between direct and crossed feels spatially open — enough bleed to move the image outside your head, not so much that it sounds artificial." }
            p { "③ Calibration → Intermittent. Pink noise alternates L and R every 500 ms. Solo → Off. Adjust Angle until each burst feels like it originates outside your head at a natural speaker position. Too small collapses the image inward; too large pushes it unnaturally wide. The Angle knob maps linearly to ITD delay (0° = 0 μs, 75° = 635 μs)." }
            p { "④ Calibration → Off. Play music. Adjust Center if the phantom center (vocals) sounds too dominant — lower values push it outward via M/S attenuation. 0 dB = bs2b canonical, no center effect." }
        }
    }
}
