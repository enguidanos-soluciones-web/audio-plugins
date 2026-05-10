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

use std::f64::consts::PI;
use vello::kurbo::{BezPath, Point};

pub fn arc_path(cx: f64, cy: f64, r: f64, start: f64, sweep: f64) -> BezPath {
    const STEPS: usize = 48;
    let mut path = BezPath::new();
    for i in 0..=STEPS {
        let a = start + (i as f64 / STEPS as f64) * sweep;
        let x = cx + r * a.cos();
        let y = cy + r * a.sin();
        if i == 0 {
            path.move_to(Point::new(x, y));
        } else {
            path.line_to(Point::new(x, y));
        }
    }
    path
}

pub fn full_circle_path(cx: f64, cy: f64, r: f64) -> BezPath {
    arc_path(cx, cy, r, 0.0, 2.0 * PI)
}

