use super::{
    PARAMETER_GESTURE_DOUBLE_CLICK, PARAMETER_GESTURE_DRAG, Parameter, ParameterClickable, ParameterDraggable, ProposedParamChange, Range,
};
use crate::gui::colors;
use crate::gui::helpers::{arc_path, full_circle_path};
use crate::gui::widget::Widget;
use std::f64::consts::PI;
use vello::{
    Scene,
    kurbo::{Affine, Circle, Line, Point, Stroke},
    peniko::Fill,
};

#[derive(Clone, Copy)]
pub struct Center;

impl Parameter<Center, Range> {
    pub const ID: usize = 3;

    pub const fn new() -> Self {
        Self {
            id: Self::ID,
            name: "Center",
            gestures: PARAMETER_GESTURE_DRAG | PARAMETER_GESTURE_DOUBLE_CLICK,
            behave: Range {
                min: -2.0,
                max: 0.0,
                def: -0.27,
            },
            _marker_type: std::marker::PhantomData,
            _marker_behaviour: std::marker::PhantomData,
        }
    }

    pub fn format_value(value: f64) -> String {
        format!("{:.2}", value)
    }

    pub fn as_draggable(&self) -> Option<ParameterDraggable<'_, Center, Range>> {
        if self.gestures & PARAMETER_GESTURE_DRAG != 0 {
            Some(ParameterDraggable::<Center, Range>::new(self))
        } else {
            None
        }
    }

    pub fn as_clickable(&self) -> Option<ParameterClickable<'_, Center, Range>> {
        if self.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 {
            Some(ParameterClickable::<Center, Range>::new(self))
        } else {
            None
        }
    }
}

impl<'a> ParameterDraggable<'a, Center, Range> {
    pub fn new(inner: &'a Parameter<Center, Range>) -> Self {
        Self {
            inner,
            _marker_type: std::marker::PhantomData,
            _marker_behaviour: std::marker::PhantomData,
        }
    }

    pub fn on_drag(&self, start_pos: (f64, f64), start_value: f64, current_pos: (f64, f64)) -> Option<ProposedParamChange> {
        const SENSITIVITY: f64 = 200.0;
        let delta = (start_pos.1 - current_pos.1) / SENSITIVITY;
        let normalized = (start_value + delta).clamp(0.0, 1.0);
        let value = self.inner.behave.min + normalized * (self.inner.behave.max - self.inner.behave.min);
        Some(ProposedParamChange {
            index: self.inner.id,
            value,
        })
    }
}

impl<'a> ParameterClickable<'a, Center, Range> {
    pub fn new(inner: &'a Parameter<Center, Range>) -> Self {
        Self {
            inner,
            _marker_type: std::marker::PhantomData,
            _marker_behaviour: std::marker::PhantomData,
        }
    }

    pub fn on_double_click(&self) -> Option<ProposedParamChange> {
        Some(ProposedParamChange {
            index: self.inner.id,
            value: self.inner.behave.def,
        })
    }
}

impl Widget for Parameter<Center, Range> {
    fn dom_id(&self) -> &'static str {
        "center"
    }
    fn param_id(&self) -> usize {
        self.id
    }

    fn draw(&self, scene: &mut Scene, coordinates: (f64, f64), dimensions: (f64, f64), _cursor: (f64, f64), value: f64) {
        let (x, y) = coordinates;
        let (width, height) = dimensions;
        let normalized = self.normalize(value);

        const KNOB_START: f64 = 3.0 * PI / 4.0;
        const KNOB_SWEEP: f64 = 3.0 * PI / 2.0;

        let cx = x + width / 2.0;
        let cy = y + height / 2.0;
        let r = width.min(height) / 2.0 - 4.0;
        let center = Point::new(cx, cy);

        scene.fill(Fill::NonZero, Affine::IDENTITY, colors::neutral_600, None, &Circle::new(center, r));
        scene.stroke(
            &Stroke::new(2.0),
            Affine::IDENTITY,
            colors::neutral_800,
            None,
            &arc_path(cx, cy, r - 7.0, KNOB_START, KNOB_SWEEP),
        );

        if normalized > 0.001 {
            scene.stroke(
                &Stroke::new(2.0),
                Affine::IDENTITY,
                colors::amber_500,
                None,
                &arc_path(cx, cy, r - 7.0, KNOB_START, normalized * KNOB_SWEEP),
            );
        }

        let angle = KNOB_START + normalized * KNOB_SWEEP;
        let ix = cx + (r - 12.0) * angle.cos();
        let iy = cy + (r - 12.0) * angle.sin();
        scene.stroke(
            &Stroke::new(1.5),
            Affine::IDENTITY,
            colors::white,
            None,
            &Line::new(center, Point::new(ix, iy)),
        );
        scene.stroke(
            &Stroke::new(1.0),
            Affine::IDENTITY,
            colors::neutral_900,
            None,
            &full_circle_path(cx, cy, r),
        );
    }
}
