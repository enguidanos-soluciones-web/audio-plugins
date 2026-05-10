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
    gui::app::{
        components::{header::Header, parameters::Parameters},
        dispatcher::Dispatcher,
    },
    state::GuiRequest,
};
use dioxus::prelude::*;
use std::time::{Duration, Instant};

const DRAG_THROTTLE: Duration = Duration::from_millis(4); // ~250fps

#[component]
pub fn Layout() -> Element {
    let dispatcher = consume_context::<Dispatcher>();

    let mut drag: Signal<Option<ActiveDrag>> = use_signal(|| None);
    let mut drag_last_dispatch: Signal<Option<Instant>> = use_signal(|| None);

    provide_context(drag);

    rsx! {
        div {
            class: "flex flex-col h-full w-full bg-neutral-900",
            onmousemove: {
                let dispatcher = dispatcher.clone();
                move |e| {
                    if drag.read().is_none() {
                        return;
                    }

                    if !e.data().held_buttons().contains(dioxus::html::input_data::MouseButton::Primary) {
                        if let Some(active) = drag.read().as_ref() {
                            dispatcher(GuiRequest::EndGesture(active.param_id()));
                        }
                        drag.set(None);
                        drag_last_dispatch.set(None);
                        return;
                    }

                    let now = Instant::now();
                    let should_dispatch = drag_last_dispatch
                        .read()
                        .map_or(true, |t| now.duration_since(t) >= DRAG_THROTTLE);

                    if should_dispatch {
                        let coords = e.data().client_coordinates();
                        if let Some(change) = drag.read().as_ref().and_then(|a| a.on_drag(coords.x, coords.y)) {
                            dispatcher(GuiRequest::SetParam(change.index, change.value));
                            drag_last_dispatch.set(Some(now));
                        }
                    }
                }
            },
            onmouseup: move |_| {
                if let Some(active) = drag.read().as_ref() {
                    dispatcher(GuiRequest::EndGesture(active.param_id()));
                }
                drag.set(None);
                drag_last_dispatch.set(None);
            },
            Header {}
            Parameters {}
        }
    }
}
