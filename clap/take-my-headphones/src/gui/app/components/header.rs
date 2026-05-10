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

            span {
                class: "text-amber-500 uppercase tracking-widest font-bold text-sm whitespace-nowrap",
                "Take My Headphones"
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
