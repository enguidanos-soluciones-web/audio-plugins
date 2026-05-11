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
    dsp::bs2b::Bs2bCoefficients,
    gui::{app::state::AppState, colors},
    parameters::{Parameter, Range, angle::Angle, bs2b_low_shelf::Bs2bLowShelf, cutoff::Cutoff, xfeed::XFeed, xfeed_slope::XFeedSlope},
};
use anyrender::PaintScene as _;
use blitz_dom::{Widget, node::ComputedStyles};
use dioxus::prelude::*;
use dioxus_native_dom::CustomWidgetAttr;
use std::f64::consts::PI;
use vello::{
    kurbo::{Affine, BezPath, Circle, Line, Point, Rect, Stroke},
    peniko::{BrushRef, Fill},
};

const DISPLAY_SR: f64 = 44100.0;

/// Mini-curve frequency range — wider than needed, but keeps the shape readable.
const MINI_F_MIN: f64 = 80.0;
const MINI_F_MAX: f64 = 12000.0;
const MINI_DB_MIN: f64 = -20.0;
const MINI_DB_MAX: f64 = 4.0;
const MINI_N: usize = 48;

/// Top-down spatial diagram of the bs2b crossfeed setup.
///
/// Three parameters are encoded visually on top of the spatial diagram:
///
/// - **XFeed Slope (Q)** → dash pattern on cross paths: low Q = long dashes
///   (gentle rolloff, more frequencies pass), high Q = short dashes (sharp cutoff).
/// - **XFeed Shelf (dB)** → alpha of direct paths: 0 dB = fully opaque,
///   −12 dB = dim (less signal on the direct path at low frequencies).
/// - **Mini curves at cups** → LP shape (sky) and shelf shape (neutral) drawn
///   outside each ear cup to show what arrives at each ear from each path.
pub struct HeadViewWidget {
    angle: f64,
    xfeed: f64,
    slope: f64,
    shelf: f64,
    cutoff: f64,
}

impl HeadViewWidget {
    pub fn new() -> Self {
        Self {
            angle: 30.0,
            xfeed: 6.2,
            slope: 0.707,
            shelf: -3.1,
            cutoff: 700.0,
        }
    }

    fn lp_magnitude_db(coeffs: &Bs2bCoefficients, f_hz: f64) -> f64 {
        let w = 2.0 * PI * f_hz / DISPLAY_SR;
        let cos_w = w.cos();
        let sin_w = w.sin();
        let cos_2w = (2.0 * w).cos();
        let sin_2w = (2.0 * w).sin();
        let num_re = coeffs.b0_lp + coeffs.b1_lp * cos_w + coeffs.b2_lp * cos_2w;
        let num_im = -(coeffs.b1_lp * sin_w + coeffs.b2_lp * sin_2w);
        let den_re = 1.0 + coeffs.a1_lp * cos_w + coeffs.a2_lp * cos_2w;
        let den_im = -(coeffs.a1_lp * sin_w + coeffs.a2_lp * sin_2w);
        let mag_sq = (num_re * num_re + num_im * num_im) / (den_re * den_re + den_im * den_im);
        10.0 * mag_sq.log10()
    }

    fn lowshelf_magnitude_db(coeffs: &Bs2bCoefficients, f_hz: f64) -> f64 {
        let w = 2.0 * PI * f_hz / DISPLAY_SR;
        let cos_w = w.cos();
        let sin_w = w.sin();
        let num_re = coeffs.a0_ls + coeffs.a1_ls * cos_w;
        let num_im = -coeffs.a1_ls * sin_w;
        let den_re = 1.0 - coeffs.b1_ls * cos_w;
        let den_im = coeffs.b1_ls * sin_w;
        let mag_sq = (num_re * num_re + num_im * num_im) / (den_re * den_re + den_im * den_im);
        10.0 * mag_sq.log10()
    }

    /// Build a mini frequency-response curve inside the rectangle (x0, y0, w, h).
    fn mini_curve(
        coeffs: &Bs2bCoefficients,
        x0: f64,
        y0: f64,
        w: f64,
        h: f64,
        magnitude_fn: impl Fn(&Bs2bCoefficients, f64) -> f64,
    ) -> BezPath {
        let mut path = BezPath::new();
        let mut pen = false;
        for i in 0..MINI_N {
            let t = i as f64 / (MINI_N - 1) as f64;
            let f = MINI_F_MIN * (MINI_F_MAX / MINI_F_MIN).powf(t);
            let db = magnitude_fn(coeffs, f);
            if db < MINI_DB_MIN || db > MINI_DB_MAX {
                pen = false;
                continue;
            }
            let x = x0 + t * w;
            let y = y0 + h * (1.0 - (db - MINI_DB_MIN) / (MINI_DB_MAX - MINI_DB_MIN));
            if pen {
                path.line_to(Point::new(x, y));
            } else {
                path.move_to(Point::new(x, y));
                pen = true;
            }
        }
        path
    }
}

impl Widget for HeadViewWidget {
    fn attribute_changed(&mut self, name: &str, _old: Option<&str>, new_value: Option<&str>) {
        let v = new_value.and_then(|s| s.parse().ok()).unwrap_or(0.0);
        match name {
            "angle" => self.angle = v,
            "xfeed" => self.xfeed = v,
            "slope" => self.slope = v,
            "shelf" => self.shelf = v,
            "cutoff" => self.cutoff = v,
            _ => {}
        }
    }

    fn paint(
        &mut self,
        _ctx: &mut dyn anyrender::RenderContext,
        _styles: &ComputedStyles,
        width: u32,
        height: u32,
        scale: f64,
    ) -> anyrender::Scene {
        let mut scene = anyrender::Scene::new();
        let w = width as f64;
        let h = height as f64;

        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            BrushRef::from(colors::neutral_950),
            None,
            &Rect::new(0.0, 0.0, w, h),
        );

        let cx = w * 0.5;
        let cy = h * 0.64;
        let r_head = (w.min(h) * 0.20).min(h * 0.26);
        let r_cup = r_head * 0.22;
        let l_cup = Point::new(cx - r_head * 1.08, cy);
        let r_cup_pt = Point::new(cx + r_head * 1.08, cy);

        let spk_dist = r_head * 2.5;
        let angle_rad = self.angle * PI / 180.0;
        let xfeed_t = ((self.xfeed - 1.0) / 14.0).clamp(0.0, 1.0);

        let l_spk = Point::new(cx - angle_rad.sin() * spk_dist, cy - angle_rad.cos() * spk_dist);
        let r_spk = Point::new(cx + angle_rad.sin() * spk_dist, cy - angle_rad.cos() * spk_dist);

        // ── Slope → dash pattern: high Q = short dashes (sharp cutoff) ──
        let slope_t = ((self.slope - 0.1) / 1.9_f64).clamp(0.0, 1.0);
        let dash_len = (12.0 - slope_t * 9.0) * scale; // 12px at Q=0.1 → 3px at Q=2.0
        let gap_len = 3.5 * scale;
        let cross_w = (0.6 + xfeed_t * 1.6) * scale;
        let stroke_cross = Stroke::new(cross_w).with_dashes(0.0, [dash_len, gap_len]);

        // ── Shelf → direct path alpha ──
        let shelf_t = ((self.shelf + 12.0) / 12.0_f64).clamp(0.0, 1.0);
        let direct_alpha = (0.25 + shelf_t * 0.75) as f32;
        let direct_color = colors::neutral_400.multiply_alpha(direct_alpha);
        let stroke_direct = Stroke::new(1.0 * scale);
        let stroke_thin = Stroke::new(1.0 * scale);

        // ── Cross paths (dashed, behind head) ──
        scene.stroke(
            &stroke_cross,
            Affine::IDENTITY,
            BrushRef::from(colors::sky_400),
            None,
            &Line::new(l_spk, r_cup_pt),
        );
        scene.stroke(
            &stroke_cross,
            Affine::IDENTITY,
            BrushRef::from(colors::sky_400),
            None,
            &Line::new(r_spk, l_cup),
        );

        // ── Direct paths (variable alpha) ──
        scene.stroke(
            &stroke_direct,
            Affine::IDENTITY,
            BrushRef::from(direct_color),
            None,
            &Line::new(l_spk, l_cup),
        );
        scene.stroke(
            &stroke_direct,
            Affine::IDENTITY,
            BrushRef::from(direct_color),
            None,
            &Line::new(r_spk, r_cup_pt),
        );

        // ── Head fill + outline ──
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            BrushRef::from(colors::neutral_800),
            None,
            &Circle::new(Point::new(cx, cy), r_head),
        );
        scene.stroke(
            &stroke_thin,
            Affine::IDENTITY,
            BrushRef::from(colors::neutral_600),
            None,
            &Circle::new(Point::new(cx, cy), r_head),
        );

        // ── Nose indicator ──
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            BrushRef::from(colors::neutral_500),
            None,
            &Circle::new(Point::new(cx, cy - r_head * 0.72), r_head * 0.10),
        );

        // ── Headphone band ──
        let band_ctrl_y = cy - r_head * 1.35;
        let mut band = BezPath::new();
        band.move_to(Point::new(l_cup.x, l_cup.y - r_cup * 0.8));
        band.quad_to(Point::new(cx, band_ctrl_y), Point::new(r_cup_pt.x, r_cup_pt.y - r_cup * 0.8));
        scene.stroke(
            &Stroke::new(2.5 * scale),
            Affine::IDENTITY,
            BrushRef::from(colors::neutral_500),
            None,
            &band,
        );

        // ── Ear cups ──
        for &pt in &[l_cup, r_cup_pt] {
            scene.fill(
                Fill::NonZero,
                Affine::IDENTITY,
                BrushRef::from(colors::neutral_700),
                None,
                &Circle::new(pt, r_cup),
            );
            scene.stroke(
                &stroke_thin,
                Affine::IDENTITY,
                BrushRef::from(colors::neutral_500),
                None,
                &Circle::new(pt, r_cup),
            );
        }

        // ── Speakers ──
        let r_spk_icon = r_head * 0.11;
        for &pt in &[l_spk, r_spk] {
            scene.fill(
                Fill::NonZero,
                Affine::IDENTITY,
                BrushRef::from(colors::amber_800),
                None,
                &Circle::new(pt, r_spk_icon),
            );
            scene.stroke(
                &stroke_thin,
                Affine::IDENTITY,
                BrushRef::from(colors::amber_500),
                None,
                &Circle::new(pt, r_spk_icon),
            );
        }

        // ── Mini frequency curves at each cup ──
        let coeffs = Bs2bCoefficients::compute(self.cutoff, self.xfeed, self.slope, self.shelf, DISPLAY_SR);
        let mini_w = r_cup * 3.2;
        let mini_h = r_cup * 1.0;
        let mini_gap = r_cup * 0.35;

        // Left cup: curves to the LEFT, stacked (LP on top, shelf below)
        let lx = l_cup.x - r_cup * 1.2 - mini_w;
        let lp_l = Self::mini_curve(
            &coeffs,
            lx,
            l_cup.y - mini_h - mini_gap * 0.5,
            mini_w,
            mini_h,
            Self::lp_magnitude_db,
        );
        let sh_l = Self::mini_curve(&coeffs, lx, l_cup.y + mini_gap * 0.5, mini_w, mini_h, Self::lowshelf_magnitude_db);
        let shelf_color = colors::neutral_400.multiply_alpha(direct_alpha);
        scene.stroke(
            &Stroke::new(0.8 * scale),
            Affine::IDENTITY,
            BrushRef::from(colors::sky_400),
            None,
            &lp_l,
        );
        scene.stroke(
            &Stroke::new(0.8 * scale),
            Affine::IDENTITY,
            BrushRef::from(shelf_color),
            None,
            &sh_l,
        );

        // Right cup: curves to the RIGHT (same curves, symmetric ears)
        let rx = r_cup_pt.x + r_cup * 1.2;
        let lp_r = Self::mini_curve(
            &coeffs,
            rx,
            r_cup_pt.y - mini_h - mini_gap * 0.5,
            mini_w,
            mini_h,
            Self::lp_magnitude_db,
        );
        let sh_r = Self::mini_curve(
            &coeffs,
            rx,
            r_cup_pt.y + mini_gap * 0.5,
            mini_w,
            mini_h,
            Self::lowshelf_magnitude_db,
        );
        scene.stroke(
            &Stroke::new(0.8 * scale),
            Affine::IDENTITY,
            BrushRef::from(colors::sky_400),
            None,
            &lp_r,
        );
        scene.stroke(
            &Stroke::new(0.8 * scale),
            Affine::IDENTITY,
            BrushRef::from(shelf_color),
            None,
            &sh_r,
        );

        scene
    }
}

#[component]
pub fn HeadView() -> Element {
    let state = consume_context::<Signal<AppState>>();

    let angle = state.read().params[Parameter::<Angle, Range>::ID];
    let xfeed = state.read().params[Parameter::<XFeed, Range>::ID];
    let slope = state.read().params[Parameter::<XFeedSlope, Range>::ID];
    let shelf = state.read().params[Parameter::<Bs2bLowShelf, Range>::ID];
    let cutoff = state.read().params[Parameter::<Cutoff, Range>::ID];

    let custom_widget = use_memo(|| CustomWidgetAttr::new(HeadViewWidget::new()));

    rsx! {
        div {
            class: "w-full h-full",
            object {
                style: "display: block; width: 100%; height: 100%;",
                "data": custom_widget,
                "angle":  "{angle}",
                "xfeed":  "{xfeed}",
                "slope":  "{slope}",
                "shelf":  "{shelf}",
                "cutoff": "{cutoff}",
            }
        }
    }
}
