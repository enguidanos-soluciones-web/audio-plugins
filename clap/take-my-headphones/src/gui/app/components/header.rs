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
    gui::app::{components::dropdown::Dropdown, dispatcher::Dispatcher, state::AppState},
    parameters::any::PARAMS_COUNT,
    preset_factory::PRESETS,
    state::GuiRequest,
};
use dioxus::prelude::*;

fn preset_matches(params: &[f64; PARAMS_COUNT], preset_idx: usize) -> bool {
    PRESETS[preset_idx]
        .values
        .iter()
        .zip(params.iter())
        .all(|(a, b)| (a - b).abs() < 1e-9)
}

#[component]
pub fn Header() -> Element {
    let state = consume_context::<Signal<AppState>>();
    let dispatcher = consume_context::<Dispatcher>();

    let mut last_selected = use_signal(|| 0usize);

    let params = state.read().params;
    let idx = *last_selected.read();
    let is_modified = !preset_matches(&params, idx);

    let options: Vec<String> = PRESETS.iter().map(|p| p.name.to_string()).collect();
    let label = if is_modified {
        Some(format!("{} *", PRESETS[idx].name))
    } else {
        None
    };

    rsx! {
        div {
            id: "header",
            class: "flex items-center justify-between px-4 py-4 border-b bg-neutral-900 border-neutral-700",

            div {
                class: "flex items-baseline gap-2",
                span {
                    class: "text-amber-500 uppercase tracking-widest font-bold text-sm whitespace-nowrap",
                    "Take My Headphones"
                }
                span {
                    class: "text-neutral-600 text-xs whitespace-nowrap",
                    "v{env!(\"CARGO_PKG_VERSION\")}"
                }
            }

            span {
                class: "text-neutral-500 text-xs whitespace-nowrap",
                "Created by Cristian A. Enguídanos Nebot"
            }

            div {
                class: "flex items-center gap-2",
                span { class: "text-neutral-500 text-xs uppercase tracking-widest", "Preset" }
                Dropdown {
                    options,
                    selected: Some(idx),
                    label,
                    on_select: move |i: usize| {
                        last_selected.set(i);
                        dispatcher(GuiRequest::LoadPreset(PRESETS[i].load_key));
                    },
                }
            }
        }
    }
}
