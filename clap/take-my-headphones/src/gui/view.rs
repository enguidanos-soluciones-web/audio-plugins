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
    gui::app::{dispatcher::Dispatcher, layout::Layout, state::AppState},
    parameters::any::PARAMS_COUNT,
    state::GUIShared,
};
use anyrender_vello::VelloScenePainter;
use baseview::MouseButton;
use blitz_dom::{Document, DocumentConfig};
use blitz_paint::paint_scene;
use blitz_traits::{
    events::{BlitzPointerEvent, BlitzPointerId, MouseEventButton, MouseEventButtons, PointerCoords, PointerDetails, UiEvent},
    shell::Viewport,
};
use dioxus::prelude::Modifiers;
use dioxus::prelude::{Signal, WritableExt};
use dioxus_core::{ScopeId, VirtualDom};
use dioxus_native_dom::DioxusDocument;
use vello::Scene;

pub struct View {
    pub doc: DioxusDocument,
    pub app_state: Signal<AppState>,
    pub pointer: (f64, f64),
    pub held_buttons: MouseEventButtons,
}

impl View {
    pub fn new(width: f64, height: f64, dispatch: Dispatcher) -> Self {
        let vdom = VirtualDom::new(Layout);

        // Create the signal inside the Dioxus runtime so reactive subscriptions work.
        // Signal writes send SchedulerMsg to the vdom channel — poll() picks these up.
        let app_state: Signal<AppState> = vdom.in_runtime(|| Signal::new_in_scope(AppState::default(), ScopeId::ROOT));

        vdom.provide_root_context(app_state);
        vdom.provide_root_context(dispatch);

        let css = include_str!("app/style/output.css");
        let mut doc = DioxusDocument::new(vdom, DocumentConfig::default());
        doc.create_head_element("style", &[], &Some(css.to_string()));

        {
            let mut inner = doc.inner_mut();
            inner.set_viewport(Viewport {
                window_size: (width as u32, height as u32),
                ..Viewport::default()
            });
        }

        doc.initial_build();

        {
            let mut inner = doc.inner_mut();
            inner.resolve(0.0);
        }

        Self {
            doc,
            app_state,
            pointer: (0.0, 0.0),
            held_buttons: MouseEventButtons::None,
        }
    }

    pub fn set_dimensions(&mut self, width: f64, height: f64) {
        let mut inner = self.doc.inner_mut();
        inner.set_viewport(Viewport {
            window_size: (width as u32, height as u32),
            ..Viewport::default()
        });
        // Mark all nodes dirty so flush_styles_to_layout re-propagates
        // viewport-dependent sizes (h-full, w-full, flex-1) on next resolve().
        let root_id = inner.root_element().id;
        if let Some(n) = inner.get_node(root_id) {
            n.set_dirty_descendants()
        }
    }

    pub fn send_pointer_down(&mut self, x: f64, y: f64, button: MouseButton) {
        self.held_buttons |= mouse_button_mask(button);
        let ui_event = UiEvent::PointerDown(self.make_pointer_event(x, y, mouse_button(button)));
        self.doc.handle_ui_event(ui_event);
    }

    pub fn send_pointer_up(&mut self, x: f64, y: f64, button: MouseButton) {
        self.held_buttons &= !mouse_button_mask(button);
        let ui_event = UiEvent::PointerUp(self.make_pointer_event(x, y, mouse_button(button)));
        self.doc.handle_ui_event(ui_event);
    }

    fn make_pointer_event(&self, x: f64, y: f64, button: MouseEventButton) -> BlitzPointerEvent {
        let coords = PointerCoords {
            page_x: x as f32,
            page_y: y as f32,
            screen_x: x as f32,
            screen_y: y as f32,
            client_x: x as f32,
            client_y: y as f32,
        };

        BlitzPointerEvent {
            id: BlitzPointerId::Mouse,
            is_primary: true,
            coords,
            button,
            buttons: self.held_buttons,
            mods: Modifiers::empty(),
            details: PointerDetails::default(),
        }
    }

    /// Called on every CursorMoved event. Forwards the pointer position to the DOM.
    pub fn send_pointer_move(&mut self, x: f64, y: f64) {
        self.pointer = (x, y);
        let ui_event = UiEvent::PointerMove(self.make_pointer_event(x, y, MouseEventButton::Main));
        self.doc.handle_ui_event(ui_event);
    }

    pub fn render(&mut self, scene: &mut Scene, state: &GUIShared, parameters_values: &[f64; PARAMS_COUNT]) {
        self.update_app_state(state, parameters_values);
        // Signal write (in update_app_state) already queued a SchedulerMsg — poll processes it.
        self.doc.poll(None);

        {
            let mut inner = self.doc.inner_mut();
            inner.resolve(0.0);
        }

        {
            let viewport = self.doc.inner().viewport().clone();
            let mut inner = self.doc.inner_mut();
            let mut painter = VelloScenePainter::new(scene);

            paint_scene(
                &mut painter,
                &mut inner,
                viewport.scale_f64(),
                viewport.window_size.0,
                viewport.window_size.1,
                0,
                0,
            );
        }
    }

    pub fn update_app_state(&mut self, _state: &GUIShared, parameters_values: &[f64; PARAMS_COUNT]) {
        // Write inside the Dioxus runtime so the reactive system tracks the change
        // and sends SchedulerMsg to the vdom channel.
        let mut app_state = self.app_state;

        self.doc.vdom.in_runtime(|| {
            let mut s = app_state.write();
            s.params = *parameters_values;
        });
    }
}

fn mouse_button(button: MouseButton) -> MouseEventButton {
    match button {
        MouseButton::Left => MouseEventButton::Main,
        MouseButton::Middle => MouseEventButton::Auxiliary,
        MouseButton::Right => MouseEventButton::Secondary,
        MouseButton::Back => MouseEventButton::Fourth,
        MouseButton::Forward => MouseEventButton::Fifth,
        MouseButton::Other(_) => MouseEventButton::Main,
    }
}

fn mouse_button_mask(button: MouseButton) -> MouseEventButtons {
    match button {
        MouseButton::Left => MouseEventButtons::Primary,
        MouseButton::Right => MouseEventButtons::Secondary,
        MouseButton::Middle => MouseEventButtons::Auxiliary,
        MouseButton::Back => MouseEventButtons::Fourth,
        MouseButton::Forward => MouseEventButtons::Fifth,
        MouseButton::Other(_) => MouseEventButtons::None,
    }
}
