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

use std::sync::Arc;

use crate::clap::clap_host_t;

/// Wraps the CLAP host pointer and exposes a safe `notify()` method.
///
/// The raw pointer is valid for the entire plugin lifetime — the host
/// guarantees this. `request_callback` is thread-safe per the CLAP spec,
/// so `Send + Sync` are sound.
pub struct HostNotifier(*const clap_host_t);

// SAFETY: CLAP spec guarantees `request_callback` may be called from any thread.
// The host pointer is valid for the plugin lifetime.
unsafe impl Send for HostNotifier {}
unsafe impl Sync for HostNotifier {}

impl HostNotifier {
    pub fn new(host: *const clap_host_t) -> Arc<Self> {
        Arc::new(Self(host))
    }

    pub fn notify(&self) {
        unsafe {
            if let Some(request_callback) = (*self.0).request_callback {
                request_callback(self.0);
            }
        }
    }
}
