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
    clap::*,
    state::{AudioThreadState, ParamEvent},
};

pub fn handle_clap_event(audio_thread: &mut AudioThreadState, event: *const clap_event_header_t) {
    audio_thread.assert_audio_thread();

    let event_ref = unsafe { event.as_ref_unchecked() };
    if event_ref.space_id != CLAP_CORE_EVENT_SPACE_ID {
        return;
    }

    if event_ref.type_ as u32 == CLAP_EVENT_PARAM_VALUE as u32 {
        let value_event = unsafe { (event as *const clap_event_param_value_t).as_ref_unchecked() };
        let id = value_event.param_id as usize;
        let value = value_event.value;

        let _ = audio_thread.daw_events.push(ParamEvent::Automation { id, value });

        // Request host call on_main_thread to update the snapshot
        unsafe {
            if let Some(request_callback) = (*audio_thread.host).request_callback {
                request_callback(audio_thread.host);
            }
        }
    }
}
