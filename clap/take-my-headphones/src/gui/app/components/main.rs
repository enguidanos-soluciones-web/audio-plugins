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
    gui::app::{
        components::{
            dropdown::Dropdown, head_view::HeadView, param_angle_knob::ParamAngleKnob, param_bs2b_low_shelf_knob::ParamBs2bLowShelfKnob,
            param_center_knob::ParamCenterKnob, param_cutoff_knob::ParamCutoffKnob, param_gain_knob::ParamGainKnob,
            param_xfeed_knob::ParamXFeedKnob, param_xfeed_slope_knob::ParamXFeedSlopeKnob, xfeed_curve::XFeedCurve,
        },
        dispatcher::Dispatcher,
        state::AppState,
    },
    parameters::{
        Parameter, Range, Select, angle::Angle, bs2b_low_shelf::Bs2bLowShelf, calibration_mode::CalibrationMode, center::Center,
        cutoff::Cutoff, gain::Gain, lrswap::LRSwap, phase::Phase, solo::Solo, xfeed::XFeed, xfeed_slope::XFeedSlope,
    },
    state::GuiRequest,
};
use dioxus::prelude::*;

#[component]
pub fn Main() -> Element {
    let state = consume_context::<Signal<AppState>>();
    let dispatcher = consume_context::<Dispatcher>();

    let cutoff_val = Parameter::<Cutoff, Range>::format_value(state.read().params[Parameter::<Cutoff, Range>::ID]);
    let xfeed_val = Parameter::<XFeed, Range>::format_value(state.read().params[Parameter::<XFeed, Range>::ID]);
    let xfeed_slope_val = Parameter::<XFeedSlope, Range>::format_value(state.read().params[Parameter::<XFeedSlope, Range>::ID]);
    let shelf_val = Parameter::<Bs2bLowShelf, Range>::format_value(state.read().params[Parameter::<Bs2bLowShelf, Range>::ID]);
    let angle_val = Parameter::<Angle, Range>::format_value(state.read().params[Parameter::<Angle, Range>::ID]);
    let center_val = Parameter::<Center, Range>::format_value(state.read().params[Parameter::<Center, Range>::ID]);
    let gain_val = Parameter::<Gain, Range>::format_value(state.read().params[Parameter::<Gain, Range>::ID]);
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
        .map(|&v| LRSwap::from(v).to_string())
        .collect();
    let solo_options: Vec<String> = Parameter::<Solo, Select>::new()
        .behave
        .options
        .iter()
        .map(|&v| Solo::from(v).to_string())
        .collect();
    let phase_options: Vec<String> = Parameter::<Phase, Select>::new()
        .behave
        .options
        .iter()
        .map(|&v| Phase::from(v).to_string())
        .collect();

    let mut show_guide = use_signal(|| false);

    rsx! {
        div {
            class: "flex flex-col gap-2",
            // Knobs
            div {
                class: "flex-1 flex items-center justify-center gap-10 py-6",

                div {
                    class: "flex flex-col items-center gap-2.5",
                    span { class: "text-amber-500 text-sm", "{cutoff_val} Hz" }
                    ParamCutoffKnob {}
                    span { class: "text-xs font-semibold tracking-widest uppercase text-neutral-400", "Cutoff" }
                }

                div {
                    class: "flex flex-col items-center gap-2.5",
                    span { class: "text-amber-500 text-sm", "{xfeed_val} dB" }
                    ParamXFeedKnob {}
                    span { class: "text-xs font-semibold tracking-widest uppercase text-neutral-400", "Xfeed" }
                }

                div {
                    class: "flex flex-col items-center gap-2.5",
                    span { class: "text-amber-500 text-sm", "{angle_val}°" }
                    ParamAngleKnob {}
                    span { class: "text-xs font-semibold tracking-widest uppercase text-neutral-400", "Angle" }
                }

                div {
                    class: "flex flex-col items-center gap-2.5",
                    span { class: "text-amber-500 text-sm", "{center_val} dB" }
                    ParamCenterKnob {}
                    span { class: "text-xs font-semibold tracking-widest uppercase text-neutral-400", "Center" }
                }

                div {
                    class: "flex flex-col items-center gap-2.5",
                    span { class: "text-amber-500 text-sm", "{gain_val} dB" }
                    ParamGainKnob {}
                    span { class: "text-xs font-semibold tracking-widest uppercase text-neutral-400", "Gain" }
                }
            }

            // Slope + Shelf knobs (left) | XFeedCurve (right)
            div {
                class: "flex items-stretch gap-0 px-8 py-2",

                // Left: Slope and Shelf controls (stacked, smaller)
                div {
                    class: "flex flex-col gap-3 items-center pr-6 border-r border-neutral-800",
                    div {
                        class: "flex flex-col items-center gap-1 text-center",
                        span { class: "text-[10px] font-semibold tracking-widest uppercase text-neutral-400", "XFeed Slope" }
                        ParamXFeedSlopeKnob {}
                        span { class: "text-amber-500 text-xs", "Q {xfeed_slope_val}" }
                    }
                    div {
                        class: "flex flex-col items-center gap-1 text-center",
                        span { class: "text-[10px] font-semibold tracking-widest uppercase text-neutral-400", "XFeed Shelf" }
                        ParamBs2bLowShelfKnob {}
                        span { class: "text-amber-500 text-xs", "{shelf_val}" }
                    }
                }

                // Right: head view + frequency response side by side
                div {
                    class: "flex-1 min-w-0 flex gap-6",
                    div {
                        class: "flex-1",
                        HeadView {}
                    }
                    div {
                        class: "flex-1",
                        XFeedCurve {}
                    }
                }
            }

            // Utility dropdowns
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
                class: "flex justify-between items-center gap-5 p-5",

                // Calibration dropdown — left-aligned above Cutoff/XFeed
                div {
                    class: "flex items-center gap-2",
                    span { class: "text-neutral-500 text-xs uppercase tracking-widest", "Calibration" }
                    Dropdown {
                        options: cal_options,
                        selected: Some(cal_value),
                        label: None,
                        open_up: true,
                        on_select: {
                            let dispatcher = dispatcher.clone();
                            move |i: usize| dispatcher(GuiRequest::SetParam(
                                Parameter::<CalibrationMode, Select>::ID,
                                i as f64,
                            ))
                        },
                    }
                }

                // Calibration guide button
                div {
                    class: "flex justify-end px-8 pb-3",
                    button {
                        class: "text-neutral-500 hover:text-neutral-300 text-xs uppercase tracking-widest border border-neutral-700 hover:border-neutral-500 rounded px-3 py-1 transition-colors",
                        onclick: move |_| show_guide.set(true),
                        "Calibration guide"
                    }
                }
            }
        }

        // Modal overlay
        if *show_guide.read() {
            div {
                class: "fixed inset-0 flex items-center justify-center z-50",
                // Backdrop
                div {
                    class: "absolute inset-0 bg-black/70",
                    onclick: move |_| show_guide.set(false),
                }
                // Panel
                div {
                    class: "relative z-10 bg-neutral-900 border border-neutral-700 rounded-lg max-w-xl w-full mx-6 p-6 text-neutral-400 text-xs leading-relaxed space-y-3",
                    div {
                        class: "flex items-center justify-between mb-3",
                        span { class: "font-semibold text-neutral-200 text-sm uppercase tracking-widest", "Calibration guide" }
                        button {
                            class: "text-neutral-500 hover:text-neutral-200 text-base leading-none",
                            onclick: move |_| show_guide.set(false),
                            "✕"
                        }
                    }
                    p { "① Calibration → Continuous. Set Solo → R: both ears hear the crossfeed path in isolation — the direct signal is removed. Adjust Cutoff until the crossed tone is warm and natural (too high = nasal and bright; too low = muddy and dark). Then refine XFeed Slope (Q): low Q broadens the rolloff for a gentler head shadow; high Q sharpens it and adds a slight resonance peak at Cutoff. Q 0.707 (Butterworth) is the neutral starting point. The LP curve (sky) in the frequency graph and the dashed cross-path in the head diagram update in real time." }
                    p { "② Still in Continuous. Set Solo → Off. Raise XFeed until the image opens outside your head — enough bleed to feel like speakers, not so much that it sounds phasey or artificial. Then tune XFeed Shelf: it attenuates the low frequencies of the direct path (the neutral curve in the graph). Lower values reduce bass heaviness and add perceived width; 0 dB leaves the direct path flat. The opacity of the direct-path lines in the head diagram reflects the shelf depth. The default −3 dB reproduces the original bs2b 1:2 ratio." }
                    p { "③ Calibration → Intermittent. Pink noise alternates L and R every 500 ms. Solo → Off. Adjust Angle until each burst feels like it originates at a natural speaker position outside your head. Too small collapses the image inward; too large pushes it unnaturally wide. The speaker icons in the head diagram move with the knob. The Angle knob maps linearly to ITD delay (0° = 0 μs, 75° = 635 μs)." }
                    p { "④ Calibration → Off. Play music. Adjust Center if the phantom center (vocals) sounds too dominant — lower values push it outward via M/S attenuation. Use Gain to compensate any perceived level reduction from the processing." }
                }
            }
        }
    }
}
