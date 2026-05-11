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

use crate::gui::colors;
use crate::gui::helpers::{arc_path, full_circle_path};
use crate::{
    gestures::drag::ActiveDrag,
    gui::app::{dispatcher::Dispatcher, state::AppState},
    parameters::{Parameter, Range, gain::Gain},
    state::GuiRequest,
};
use anyrender::PaintScene as _;
use blitz_dom::{Widget, node::ComputedStyles};
use dioxus::prelude::*;
use dioxus_native_dom::CustomWidgetAttr;
use std::f64::consts::PI;
use vello::{
    kurbo::{Affine, Circle, Line, Point, Stroke},
    peniko::{BrushRef, Fill},
};

pub struct ParamGainWidget {
    normalized: f64,
}

impl ParamGainWidget {
    fn new() -> Self {
        Self { normalized: 0.0 }
    }
}

impl Widget for ParamGainWidget {
    fn attribute_changed(&mut self, name: &str, _old_value: Option<&str>, new_value: Option<&str>) {
        if name == "value" {
            if let Some(v) = new_value.and_then(|s| s.parse().ok()) {
                self.normalized = v;
            }
        }
    }

    fn paint(
        &mut self,
        _render_ctx: &mut dyn anyrender::RenderContext,
        _styles: &ComputedStyles,
        width: u32,
        height: u32,
        scale: f64,
    ) -> anyrender::Scene {
        /// `normalized` ∈ [0, 1]. `width`/`height` are physical pixels. `scale` is DPI factor.
        const KNOB_START: f64 = 3.0 * PI / 4.0;
        const KNOB_SWEEP: f64 = 3.0 * PI / 2.0;

        let mut scene = anyrender::Scene::new();

        let cx = width as f64 / 2.0;
        let cy = height as f64 / 2.0;
        let unit = (width.min(height)) as f64;
        let r = unit / 2.0 - 4.0 * scale;
        let track_r = r - 7.0 * scale;
        let indicator_r = r - 12.0 * scale;
        let stroke_2 = Stroke::new(2.0 * scale);
        let stroke_1_5 = Stroke::new(1.5 * scale);
        let stroke_1 = Stroke::new(1.0 * scale);
        let center = Point::new(cx, cy);

        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            BrushRef::from(colors::neutral_600),
            None,
            &Circle::new(center, r),
        );
        scene.stroke(
            &stroke_2,
            Affine::IDENTITY,
            BrushRef::from(colors::neutral_800),
            None,
            &arc_path(cx, cy, track_r, KNOB_START, KNOB_SWEEP),
        );

        if self.normalized > 0.001 {
            scene.stroke(
                &stroke_2,
                Affine::IDENTITY,
                BrushRef::from(colors::amber_500),
                None,
                &arc_path(cx, cy, track_r, KNOB_START, self.normalized * KNOB_SWEEP),
            );
        }

        let angle = KNOB_START + self.normalized * KNOB_SWEEP;
        let ix = cx + indicator_r * angle.cos();
        let iy = cy + indicator_r * angle.sin();
        scene.stroke(
            &stroke_1_5,
            Affine::IDENTITY,
            BrushRef::from(colors::white),
            None,
            &Line::new(center, Point::new(ix, iy)),
        );
        scene.stroke(
            &stroke_1,
            Affine::IDENTITY,
            BrushRef::from(colors::neutral_900),
            None,
            &full_circle_path(cx, cy, r),
        );

        scene
    }
}

#[component]
pub fn ParamGainKnob() -> Element {
    let state = consume_context::<Signal<AppState>>();
    let dispatcher = consume_context::<Dispatcher>();
    let mut drag = consume_context::<Signal<Option<ActiveDrag>>>();

    let normalized = Parameter::<Gain, Range>::new().normalize(state.read().params[Parameter::<Gain, Range>::ID]);

    let custom_widget = use_memo(|| CustomWidgetAttr::new(ParamGainWidget::new()));
    let dispatcher_cloned = dispatcher.clone();

    rsx! {
        div {
            class: "w-16 h-16",
            onmousedown: move |e| {
                dispatcher(GuiRequest::BeginGesture(Parameter::<Gain, Range>::ID));
                let coords = e.data().client_coordinates();
                let raw = state.read().params[Parameter::<Gain, Range>::ID];
                drag.set(ActiveDrag::from_index(Parameter::<Gain, Range>::ID, coords.x, coords.y, raw));
            },
            ondoubleclick: move |_| dispatcher_cloned(GuiRequest::ResetParam(Parameter::<Gain, Range>::ID)),
            object {
                style: "display: block; width: 100%; height: 100%;",
                "data": custom_widget,
                "value": "{normalized}",
            }
        }
    }
}
