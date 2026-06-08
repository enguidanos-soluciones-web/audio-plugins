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
    dsp::nam,
    helper::DecibelConversion,
    parameters::{Parameter, Range, blend::Blend, input_gain::InputGain, output_gain::OutputGain, tone::Tone},
    state::AudioThreadState,
};

fn apply_pending_model_update(audio_thread: &mut AudioThreadState) {
    if let Some(update) = audio_thread.model_updates.pop() {
        audio_thread.nam_model = Some(update.model);
        audio_thread.nam_loudness_correction = update.loudness_correction;
        audio_thread.dc_filter.reset();
        audio_thread.lowpass_filter.reset();
    }
}

pub fn render_audio_f64(audio_thread: &mut AudioThreadState, input: *const f64, output: *mut f64, nframes: usize) {
    audio_thread.assert_audio_thread();
    apply_pending_model_update(audio_thread);

    let snapshot = audio_thread.param_snapshot.load();

    let input_gain = DecibelConversion::Amplitude.to_linear(snapshot.values[Parameter::<InputGain, Range>::ID]);
    let output_gain = DecibelConversion::Amplitude.to_linear(snapshot.values[Parameter::<OutputGain, Range>::ID]);
    let blend = snapshot.values[Parameter::<Blend, Range>::ID];

    let input_slice = unsafe { std::slice::from_raw_parts(input, nframes) };
    let output_slice = unsafe { std::slice::from_raw_parts_mut(output, nframes) };

    // 1. Apply input gain
    for i in 0..nframes {
        audio_thread.input_buf[i] = input_slice[i] * input_gain;
    }

    if let Some(nam_model) = audio_thread.nam_model.as_mut() {
        // 2. Process with NAM
        nam::ffi::process(
            nam_model.pin_mut(),
            &audio_thread.input_buf[..nframes],
            &mut audio_thread.output_buf[..nframes],
        );

        let tone = snapshot.values[Parameter::<Tone, Range>::ID];
        let cutoff = Parameter::<Tone, Range>::to_hertz(tone);
        audio_thread.lowpass_filter.set_cutoff(cutoff, audio_thread.sample_rate);

        // 3. DC filter, loudness correction, output gain, dry/wet blend, then tone lowpass.
        // dry: raw input signal — no input gain, no NAM, no loudness correction.
        // wet: NAM → DC filter → loudness correction → output gain.
        // The tone lowpass is applied to the blended result so it shapes both paths equally.
        for i in 0..nframes {
            let dc_filtered = audio_thread.dc_filter.process_sample(audio_thread.output_buf[i]);
            let wet = dc_filtered * audio_thread.nam_loudness_correction * output_gain;
            let blended = Parameter::<Blend, Range>::mix(input_slice[i], wet, blend);
            output_slice[i] = audio_thread.lowpass_filter.process_sample(blended);
        }
    } else {
        output_slice.copy_from_slice(input_slice);
    }
}

pub fn render_audio_f32(audio_thread: &mut AudioThreadState, input: *const f32, output: *mut f32, nframes: usize) {
    audio_thread.assert_audio_thread();
    apply_pending_model_update(audio_thread);

    let snapshot = audio_thread.param_snapshot.load();

    let input_gain = DecibelConversion::Amplitude.to_linear(snapshot.values[Parameter::<InputGain, Range>::ID]);
    let output_gain = DecibelConversion::Amplitude.to_linear(snapshot.values[Parameter::<OutputGain, Range>::ID]);
    let blend = snapshot.values[Parameter::<Blend, Range>::ID];

    let input_slice = unsafe { std::slice::from_raw_parts(input, nframes) };
    let output_slice = unsafe { std::slice::from_raw_parts_mut(output, nframes) };

    // 1. Apply input gain
    for i in 0..nframes {
        audio_thread.input_buf[i] = input_slice[i] as f64 * input_gain;
    }

    if let Some(nam_model) = audio_thread.nam_model.as_mut() {
        // 2. Process with NAM
        nam::ffi::process(
            nam_model.pin_mut(),
            &audio_thread.input_buf[..nframes],
            &mut audio_thread.output_buf[..nframes],
        );

        let tone = snapshot.values[Parameter::<Tone, Range>::ID];
        let tone_cutoff = Parameter::<Tone, Range>::to_hertz(tone);
        audio_thread.lowpass_filter.set_cutoff(tone_cutoff, audio_thread.sample_rate);

        // 3. DC filter, loudness correction, output gain, dry/wet blend, then tone lowpass.
        // dry: raw input signal — no input gain, no NAM, no loudness correction.
        // wet: NAM → DC filter → loudness correction → output gain.
        // The tone lowpass is applied to the blended result so it shapes both paths equally.
        for i in 0..nframes {
            let dc_filtered = audio_thread.dc_filter.process_sample(audio_thread.output_buf[i]);
            let wet = dc_filtered * audio_thread.nam_loudness_correction * output_gain;
            let blended = Parameter::<Blend, Range>::mix(input_slice[i] as f64, wet, blend);
            output_slice[i] = audio_thread.lowpass_filter.process_sample(blended) as f32;
        }
    } else {
        output_slice.copy_from_slice(input_slice);
    }
}
