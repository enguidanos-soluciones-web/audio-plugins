use dioxus::prelude::*;

/// https://github.com/DioxusLabs/blitz/issues/119
///
/// Generic dropdown component.
///
/// - `options`   — list of labels (index = value)
/// - `selected`  — currently selected index (None = placeholder shown)
/// - `label`     — override label to display on the button (e.g. "Default *")
/// - `on_select` — called with the chosen index
#[component]
pub fn Dropdown(options: Vec<String>, selected: Option<usize>, label: Option<String>, on_select: EventHandler<usize>) -> Element {
    let mut open = use_signal(|| false);

    let display = label.unwrap_or_else(|| selected.and_then(|i| options.get(i)).cloned().unwrap_or_else(|| "–".to_string()));

    let mut hovered: Signal<Option<usize>> = use_signal(|| None);

    rsx! {
        div {
            class: "relative inline-block",

            button {
                class: "px-3 py-1 text-xs rounded bg-neutral-700 text-neutral-300 hover:bg-neutral-600 flex items-center gap-2 min-w-[110px] justify-between whitespace-nowrap",
                onclick: move |_| {
                    let was_open = *open.read();
                    open.set(!was_open);
                },
                span { "{display}" }
                span { class: "text-neutral-500 text-[10px]", "▾" }
            }

            if *open.read() {
                div {
                    class: "absolute top-full left-0 mt-0.5 z-50 bg-neutral-800 border border-neutral-700 rounded shadow-lg min-w-full",
                    for (i, option) in options.iter().enumerate() {
                        {
                            let option = option.clone();
                            let is_active = selected == Some(i);
                            let is_hovered = *hovered.read() == Some(i);
                            rsx! {
                                button {
                                    class: if is_active {
                                        "block w-full text-left px-3 py-1.5 text-xs text-amber-400 bg-neutral-700/60 whitespace-nowrap"
                                    } else if is_hovered {
                                        "block w-full text-left px-3 py-1.5 text-xs text-neutral-300 bg-neutral-700/40 whitespace-nowrap"
                                    } else {
                                        "block w-full text-left px-3 py-1.5 text-xs text-neutral-300 whitespace-nowrap"
                                    },
                                    onmouseenter: move |_| hovered.set(Some(i)),
                                    onmouseleave: move |_| hovered.set(None),
                                    onclick: move |_| {
                                        on_select.call(i);
                                        open.set(false);
                                    },
                                    "{option}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
