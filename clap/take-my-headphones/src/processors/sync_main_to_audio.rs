use crate::{
    clap::*,
    state::{AudioThreadState, ParamChange, ParamEvent},
};

pub fn sync_main_to_audio(audio_thread: &mut AudioThreadState, out: *const clap_output_events_t) {
    audio_thread.assert_audio_thread();

    let out_ref = unsafe { out.as_ref_unchecked() };
    let Some(try_push) = out_ref.try_push else { return };

    while let Some(change) = audio_thread.param_changes.pop() {
        match change {
            ParamChange::Value { id, value } => {
                let mut event = unsafe { std::mem::zeroed::<clap_event_param_value_t>() };
                event.header.size = std::mem::size_of::<clap_event_param_value_t>() as u32;
                event.header.time = 0;
                event.header.space_id = CLAP_CORE_EVENT_SPACE_ID;
                event.header.type_ = CLAP_EVENT_PARAM_VALUE as u16;
                event.header.flags = 0;
                event.param_id = id as u32;
                event.cookie = std::ptr::null_mut();
                event.note_id = -1;
                event.port_index = -1;
                event.channel = -1;
                event.key = -1;
                event.value = value;

                if unsafe { try_push(out, &event.header) } {
                    let _ = audio_thread.daw_events.push(ParamEvent::Ack);
                } else {
                    let _ = audio_thread.daw_events.push(ParamEvent::Nack { id });
                }
            }

            ParamChange::GestureBegin { id } => {
                let mut event = unsafe { std::mem::zeroed::<clap_event_param_gesture_t>() };
                event.header.size = std::mem::size_of::<clap_event_param_gesture_t>() as u32;
                event.header.time = 0;
                event.header.space_id = CLAP_CORE_EVENT_SPACE_ID;
                event.header.type_ = CLAP_EVENT_PARAM_GESTURE_BEGIN as u16;
                event.header.flags = 0;
                event.param_id = id as u32;
                unsafe { try_push(out, &event.header) };
            }

            ParamChange::GestureEnd { id } => {
                let mut event = unsafe { std::mem::zeroed::<clap_event_param_gesture_t>() };
                event.header.size = std::mem::size_of::<clap_event_param_gesture_t>() as u32;
                event.header.time = 0;
                event.header.space_id = CLAP_CORE_EVENT_SPACE_ID;
                event.header.type_ = CLAP_EVENT_PARAM_GESTURE_END as u16;
                event.header.flags = 0;
                event.param_id = id as u32;
                unsafe { try_push(out, &event.header) };
            }
        }
    }
}
