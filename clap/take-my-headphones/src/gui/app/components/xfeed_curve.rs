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
    parameters::{Parameter, Range, bs2b_low_shelf::Bs2bLowShelf, cutoff::Cutoff, xfeed::XFeed, xfeed_slope::XFeedSlope},
};
use anyrender::PaintScene as _;
use blitz_dom::{Widget, node::ComputedStyles};
use dioxus::prelude::*;
use dioxus_native_dom::CustomWidgetAttr;
use std::f64::consts::PI;
use vello::{
    kurbo::{Affine, BezPath, Line, Point, Rect, Stroke},
    peniko::{BrushRef, Fill},
};

/// Fixed sample rate used for display-only frequency response calculation.
/// The actual audio engine uses the real session sample rate; this value only
/// affects the visual shape of the curves in the widget.
const DISPLAY_SR: f64 = 44100.0;

/// Number of frequency samples used to build each curve path.
/// 512 points gives a smooth curve at typical widget widths (300–800 px).
const N_POINTS: usize = 512;

/// Lowest frequency shown on the x-axis (Hz).
const F_MIN: f64 = 20.0;

/// Highest frequency shown on the x-axis (Hz).
const F_MAX: f64 = 20000.0;

/// Bottom of the dB scale (y-axis minimum).
const DB_MIN: f64 = -24.0;

/// Top of the dB scale (y-axis maximum).
const DB_MAX: f64 = 6.0;

/// Frequency response visualiser for the bs2b crossfeed filters.
///
/// Draws three curves on a log-frequency / linear-dB grid:
///
/// - **Combined (sky, primary)** — complex sum `H_ls(f) + H_lp(f)`: the actual
///   tonal effect heard by each ear for a mono input signal.
/// - **LP (amber, dim)** — the biquad lowpass on the crossed path. Reference
///   for how Cutoff and Slope shape the head-shadow simulation.
/// - **Low shelf (neutral, dim)** — the first-order IIR shelf on the direct
///   path. Reference for the low-frequency attenuation.
///
/// A vertical marker at the Cutoff frequency ties all curves visually.
pub struct XFeedCurveWidget {
    cutoff: f64,
    xfeed: f64,
    slope: f64,
    shelf: f64,
}

impl XFeedCurveWidget {
    pub fn new() -> Self {
        Self {
            cutoff: 700.0,
            xfeed: 4.5,
            slope: 0.707,
            shelf: -2.25,
        }
    }

    /// Map a frequency in Hz to an x pixel coordinate (log scale).
    fn freq_to_x(f: f64, width: f64) -> f64 {
        width * (f / F_MIN).log2() / (F_MAX / F_MIN).log2()
    }

    /// Map a dB value to a y pixel coordinate (linear scale, 0 = top).
    fn db_to_y(db: f64, height: f64) -> f64 {
        height * (1.0 - (db - DB_MIN) / (DB_MAX - DB_MIN))
    }

    /// Magnitude in dB of the biquad LP (crossed path) at `f_hz`.
    ///
    /// Evaluates `H(e^jω)` at `ω = 2π·f/fs` using Direct Form II transposed
    /// coefficients. The result already includes the `Gd` gain baked into the
    /// feedforward coefficients, so DC gain equals `Gd` in dB.
    fn lp_magnitude_db(coeffs: &Bs2bCoefficients, f_hz: f64) -> f64 {
        let w = 2.0 * PI * f_hz / DISPLAY_SR;
        let cos_w = w.cos();
        let sin_w = w.sin();
        let cos_2w = (2.0 * w).cos();
        let sin_2w = (2.0 * w).sin();

        // H(e^jω) = (b0 + b1·e^−jω + b2·e^−2jω) / (1 + a1·e^−jω + a2·e^−2jω)
        let num_re = coeffs.b0_lp + coeffs.b1_lp * cos_w + coeffs.b2_lp * cos_2w;
        let num_im = -(coeffs.b1_lp * sin_w + coeffs.b2_lp * sin_2w);
        let den_re = 1.0 + coeffs.a1_lp * cos_w + coeffs.a2_lp * cos_2w;
        let den_im = -(coeffs.a1_lp * sin_w + coeffs.a2_lp * sin_2w);

        let mag_sq = (num_re * num_re + num_im * num_im) / (den_re * den_re + den_im * den_im);
        10.0 * mag_sq.log10()
    }

    /// Combined mono response at `f_hz`: `H_ls(f) + H_lp(f)`.
    ///
    /// For a mono input (L=R=x) both paths sum at each output ear, so the
    /// perceived tonal effect is the complex sum of the two transfer functions.
    fn combined_magnitude_db(coeffs: &Bs2bCoefficients, f_hz: f64) -> f64 {
        let w = 2.0 * PI * f_hz / DISPLAY_SR;
        let cos_w = w.cos();
        let sin_w = w.sin();
        let cos_2w = (2.0 * w).cos();
        let sin_2w = (2.0 * w).sin();

        // H_lp — biquad (crossed path)
        let lp_num_re = coeffs.b0_lp + coeffs.b1_lp * cos_w + coeffs.b2_lp * cos_2w;
        let lp_num_im = -(coeffs.b1_lp * sin_w + coeffs.b2_lp * sin_2w);
        let lp_den_re = 1.0 + coeffs.a1_lp * cos_w + coeffs.a2_lp * cos_2w;
        let lp_den_im = -(coeffs.a1_lp * sin_w + coeffs.a2_lp * sin_2w);
        let lp_den_sq = lp_den_re * lp_den_re + lp_den_im * lp_den_im;
        let lp_re = (lp_num_re * lp_den_re + lp_num_im * lp_den_im) / lp_den_sq;
        let lp_im = (lp_num_im * lp_den_re - lp_num_re * lp_den_im) / lp_den_sq;

        // H_ls — first-order shelf (direct path)
        let ls_num_re = coeffs.a0_ls + coeffs.a1_ls * cos_w;
        let ls_num_im = -coeffs.a1_ls * sin_w;
        let ls_den_re = 1.0 - coeffs.b1_ls * cos_w;
        let ls_den_im = coeffs.b1_ls * sin_w;
        let ls_den_sq = ls_den_re * ls_den_re + ls_den_im * ls_den_im;
        let ls_re = (ls_num_re * ls_den_re + ls_num_im * ls_den_im) / ls_den_sq;
        let ls_im = (ls_num_im * ls_den_re - ls_num_re * ls_den_im) / ls_den_sq;

        // Sum in complex domain, then convert to dB
        let re = ls_re + lp_re;
        let im = ls_im + lp_im;
        10.0 * (re * re + im * im).log10()
    }

    /// Magnitude in dB of the low shelf (direct path) at `f_hz`.
    ///
    /// Transfer function: `H(z) = (a0_ls + a1_ls·z⁻¹) / (1 − b1_ls·z⁻¹)`
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

    /// Build a `BezPath` by sampling a magnitude function at `N_POINTS` log-spaced frequencies.
    ///
    /// Points outside `[DB_MIN, DB_MAX]` lift the pen — the curve stops rather
    /// than hugging the display edge. This prevents a flat line at the bottom
    /// of the LP curve after its rolloff exceeds the visible range.
    fn build_curve(coeffs: &Bs2bCoefficients, width: f64, height: f64, magnitude_fn: impl Fn(&Bs2bCoefficients, f64) -> f64) -> BezPath {
        let mut path = BezPath::new();
        let mut pen_down = false;
        for i in 0..N_POINTS {
            let t = i as f64 / (N_POINTS - 1) as f64;
            // Log-spaced frequency: F_MIN · (F_MAX/F_MIN)^t
            let f = F_MIN * (F_MAX / F_MIN).powf(t);
            let db = magnitude_fn(coeffs, f);
            if db < DB_MIN || db > DB_MAX {
                pen_down = false;
                continue;
            }
            let x = t * width;
            let y = Self::db_to_y(db, height);
            if pen_down {
                path.line_to(Point::new(x, y));
            } else {
                path.move_to(Point::new(x, y));
                pen_down = true;
            }
        }
        path
    }
}

impl Widget for XFeedCurveWidget {
    fn attribute_changed(&mut self, name: &str, _old_value: Option<&str>, new_value: Option<&str>) {
        let v = new_value.and_then(|s| s.parse().ok()).unwrap_or(0.0);
        match name {
            "cutoff" => self.cutoff = v,
            "xfeed" => self.xfeed = v,
            "slope" => self.slope = v,
            "shelf" => self.shelf = v,
            _ => {}
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
        let mut scene = anyrender::Scene::new();
        let w = width as f64;
        let h = height as f64;
        let stroke_grid = Stroke::new(1.0 * scale);
        let stroke_ref = Stroke::new(0.75 * scale); // individual filter reference lines
        let stroke_combined = Stroke::new(1.75 * scale); // combined mono response
        let stroke_cutoff = Stroke::new(1.0 * scale);

        // Background
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            BrushRef::from(colors::neutral_950),
            None,
            &Rect::new(0.0, 0.0, w, h),
        );

        let coeffs = Bs2bCoefficients::compute(self.cutoff, self.xfeed, self.slope, self.shelf, DISPLAY_SR);

        // Horizontal grid — dB reference lines at -18, -12, -6, 0
        for &db in &[-18.0_f64, -12.0, -6.0, 0.0] {
            let y = Self::db_to_y(db, h);
            scene.stroke(
                &stroke_grid,
                Affine::IDENTITY,
                BrushRef::from(colors::neutral_800),
                None,
                &Line::new((0.0, y), (w, y)),
            );
        }

        // Vertical grid — one line per decade-subdivision: 50, 100, 200, 500, 1k, 2k, 5k, 10k
        for &f in &[50.0_f64, 100.0, 200.0, 500.0, 1_000.0, 2_000.0, 5_000.0, 10_000.0] {
            let x = Self::freq_to_x(f, w);
            scene.stroke(
                &stroke_grid,
                Affine::IDENTITY,
                BrushRef::from(colors::neutral_800),
                None,
                &Line::new((x, 0.0), (x, h)),
            );
        }

        // Cutoff frequency marker — amber_500 (same as parameter labels)
        let cutoff_x = Self::freq_to_x(self.cutoff, w);
        scene.stroke(
            &stroke_cutoff,
            Affine::IDENTITY,
            BrushRef::from(colors::amber_500),
            None,
            &Line::new((cutoff_x, 0.0), (cutoff_x, h)),
        );

        // Low shelf reference (direct path) — neutral_400
        let ls_path = Self::build_curve(&coeffs, w, h, Self::lowshelf_magnitude_db);
        scene.stroke(&stroke_ref, Affine::IDENTITY, BrushRef::from(colors::neutral_400), None, &ls_path);

        // LP biquad reference (crossed path) — neutral_400
        let lp_path = Self::build_curve(&coeffs, w, h, Self::lp_magnitude_db);
        scene.stroke(&stroke_ref, Affine::IDENTITY, BrushRef::from(colors::neutral_400), None, &lp_path);

        // Combined mono response (H_ls + H_lp) — sky, drawn on top as primary
        let combined_path = Self::build_curve(&coeffs, w, h, Self::combined_magnitude_db);
        scene.stroke(
            &stroke_combined,
            Affine::IDENTITY,
            BrushRef::from(colors::sky_400),
            None,
            &combined_path,
        );

        scene
    }
}

#[component]
pub fn XFeedCurve() -> Element {
    let state = consume_context::<Signal<AppState>>();

    let cutoff = state.read().params[Parameter::<Cutoff, Range>::ID];
    let xfeed = state.read().params[Parameter::<XFeed, Range>::ID];
    let slope = state.read().params[Parameter::<XFeedSlope, Range>::ID];
    let shelf = state.read().params[Parameter::<Bs2bLowShelf, Range>::ID];

    let custom_widget = use_memo(|| CustomWidgetAttr::new(XFeedCurveWidget::new()));

    rsx! {
        div {
            class: "w-full h-full",
            object {
                style: "display: block; width: 100%; height: 100%;",
                "data": custom_widget,
                "cutoff": "{cutoff}",
                "xfeed": "{xfeed}",
                "slope": "{slope}",
                "shelf": "{shelf}",
            }
        }
    }
}
