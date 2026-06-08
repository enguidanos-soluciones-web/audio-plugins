// Copyright (C) 2026 Cristian A. Enguídanos Nebot
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::{
    gestures::drag::ActiveDrag,
    gui::app::{dispatcher::Dispatcher, state::AppState},
    parameters::{Parameter, Range, blend::Blend, input_gain::InputGain, output_gain::OutputGain, tone::Tone},
    state::GuiRequest,
};
use dioxus::prelude::*;

#[component]
pub fn Parameters() -> Element {
    let state = consume_context::<Signal<AppState>>();
    let dispatcher = consume_context::<Dispatcher>();

    let mut drag = consume_context::<Signal<Option<ActiveDrag>>>();

    let input_db = format!("{:.1} db", state.read().params[Parameter::<InputGain, Range>::ID]);
    let output_db = format!("{:.1} db", state.read().params[Parameter::<OutputGain, Range>::ID]);
    let tone_val = format!("{:.1}", state.read().params[Parameter::<Tone, Range>::ID] * 5.0);
    let blend_val = format!("{:.0}%", state.read().params[Parameter::<Blend, Range>::ID] * 100.0);

    rsx! {
        div {
            class: "flex-1 flex items-center justify-center gap-10",

                div {
                    class: "flex flex-col items-center gap-2.5",
                    span { id: "blend-val", class: "text-amber-500 text-sm", "{blend_val}" }
                    div {
                        id: "blend",
                        class: "w-20 h-20",
                        onmousedown: {
                            let state = state.clone();
                            let dispatcher = dispatcher.clone();
                            move |e| {
                                let coords = e.data().client_coordinates();
                                let raw = state.read().params[Parameter::<Blend, Range>::ID];
                                drag.set(ActiveDrag::from_index(Parameter::<Blend, Range>::ID, coords.x, coords.y, raw));
                                dispatcher(GuiRequest::BeginGesture(Parameter::<Blend, Range>::ID));
                            }
                        },
                        ondoubleclick: {
                            let dispatcher = dispatcher.clone();
                            move |_| dispatcher(GuiRequest::ResetParam(Parameter::<Blend, Range>::ID))
                        },
                    }
                    span { class: "text-xs font-semibold tracking-widest uppercase text-neutral-400", "Blend" }
                }

                div {
                    class: "flex flex-col items-center gap-2.5",
                    span { id: "input-gain-db", class: "text-amber-500 text-sm", "{input_db}" }
                    div {
                        id: "input-gain",
                        class: "w-20 h-20",
                        onmousedown: {
                            let state = state.clone();
                            let dispatcher = dispatcher.clone();
                            move |e| {
                                let coords = e.data().client_coordinates();
                                let raw = state.read().params[Parameter::<InputGain, Range>::ID];
                                drag.set(ActiveDrag::from_index(Parameter::<InputGain, Range>::ID, coords.x, coords.y, raw));
                                dispatcher(GuiRequest::BeginGesture(Parameter::<InputGain, Range>::ID));
                            }
                        },
                        ondoubleclick: {
                            let dispatcher = dispatcher.clone();
                            move |_| dispatcher(GuiRequest::ResetParam(Parameter::<InputGain, Range>::ID))
                        },
                    }
                    span { class: "text-xs font-semibold tracking-widest uppercase text-neutral-400", "Gain" }
                }

                div {
                    class: "flex flex-col items-center gap-2.5",
                    span { id: "tone-val", class: "text-amber-500 text-sm", "{tone_val}" }
                    div {
                        id: "tone",
                        class: "w-20 h-20",
                        onmousedown: {
                            let state = state.clone();
                            let dispatcher = dispatcher.clone();
                            move |e| {
                                let coords = e.data().client_coordinates();
                                let raw = state.read().params[Parameter::<Tone, Range>::ID];
                                drag.set(ActiveDrag::from_index(Parameter::<Tone, Range>::ID, coords.x, coords.y, raw));
                                dispatcher(GuiRequest::BeginGesture(Parameter::<Tone, Range>::ID));
                            }
                        },
                        ondoubleclick: {
                            let dispatcher = dispatcher.clone();
                            move |_| dispatcher(GuiRequest::ResetParam(Parameter::<Tone, Range>::ID))
                        },
                    }
                    span { class: "text-xs font-semibold tracking-widest uppercase text-neutral-400", "Tone" }
                }

                div {
                    class: "flex flex-col items-center gap-2.5",
                    span { id: "output-gain-db", class: "text-amber-500 text-sm", "{output_db}" }
                    div {
                        id: "output-gain",
                        class: "w-20 h-20",
                        onmousedown: {
                            let state = state.clone();
                            let dispatcher = dispatcher.clone();
                            move |e| {
                                let coords = e.data().client_coordinates();
                                let raw = state.read().params[Parameter::<OutputGain, Range>::ID];
                                drag.set(ActiveDrag::from_index(Parameter::<OutputGain, Range>::ID, coords.x, coords.y, raw));
                                dispatcher(GuiRequest::BeginGesture(Parameter::<OutputGain, Range>::ID));
                            }
                        },
                        ondoubleclick: {
                            let dispatcher = dispatcher.clone();
                            move |_| dispatcher(GuiRequest::ResetParam(Parameter::<OutputGain, Range>::ID))
                        },
                    }
                    span { class: "text-xs font-semibold tracking-widest uppercase text-neutral-400", "Master" }
                }
        }
    }
}
