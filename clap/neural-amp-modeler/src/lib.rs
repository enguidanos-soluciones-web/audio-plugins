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

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
#![cfg_attr(target_os = "macos", allow(unexpected_cfgs))]

pub mod clap {
    include!(concat!(env!("OUT_DIR"), "/clap.rs"));
}

mod channel;
mod descriptor;
mod dsp;
mod entry;
mod extensions;
mod factory;
mod gestures;
mod gui;
mod helper;
mod host_notifier;
mod parameters;
mod plugin;
mod preset_factory;
mod processors;
mod state;
mod version;
