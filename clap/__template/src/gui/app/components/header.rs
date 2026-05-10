use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            id: "header",
            class: "flex items-center justify-between px-4 py-4 border-b bg-neutral-900 border-neutral-700",
            span {
                class: "text-amber-500 uppercase tracking-widest font-bold text-sm whitespace-nowrap",
                "Take My Phones"
            }
        }
    }
}
