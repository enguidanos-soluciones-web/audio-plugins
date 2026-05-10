use crate::{
    dsp::nam,
    helper::{DecibelConversion, db_to_linear},
    parameters::{Parameter, Range, blend::Blend, input_gain::InputGain, output_gain::OutputGain, tone::Tone},
    state::AudioThreadState,
};

pub fn render_audio_f64(audio_thread: &mut AudioThreadState, input: *const f64, output: *mut f64, nframes: usize) {
    audio_thread.assert_audio_thread();

    let snapshot = audio_thread.param_snapshot.load();

    let input_gain = db_to_linear(snapshot.values[Parameter::<InputGain, Range>::ID], DecibelConversion::Amplitude);
    let output_gain = db_to_linear(snapshot.values[Parameter::<OutputGain, Range>::ID], DecibelConversion::Amplitude);

    let blend = snapshot.values[Parameter::<Blend, Range>::ID];

    let input_slice = unsafe { std::slice::from_raw_parts(input, nframes) };
    let output_slice = unsafe { std::slice::from_raw_parts_mut(output, nframes) };

    for i in 0..nframes {
        audio_thread.input_buf[i] = input_slice[i] * input_gain;
    }

    for i in 0..nframes {
        let dc_filtered = audio_thread.dc_filter.process_sample(audio_thread.output_buf[i]);

        let wet = dc_filtered * output_gain;
        let dry = input_slice[i];

        output_slice[i] = Parameter::<Blend, Range>::mix(dry, wet, blend);
    }
}

pub fn render_audio_f32(audio_thread: &mut AudioThreadState, input: *const f32, output: *mut f32, nframes: usize) {
    audio_thread.assert_audio_thread();
    apply_pending_model_update(audio_thread);

    let snapshot = audio_thread.param_snapshot.load();

    let input_gain = db_to_linear(snapshot.values[Parameter::<InputGain, Range>::ID], DecibelConversion::Amplitude);
    let output_gain = db_to_linear(snapshot.values[Parameter::<OutputGain, Range>::ID], DecibelConversion::Amplitude);
    let blend = snapshot.values[Parameter::<Blend, Range>::ID];

    let input_slice = unsafe { std::slice::from_raw_parts(input, nframes) };
    let output_slice = unsafe { std::slice::from_raw_parts_mut(output, nframes) };

    for i in 0..nframes {
        audio_thread.input_buf[i] = input_slice[i] as f64 * input_gain;
    }

    for i in 0..nframes {
        let dc_filtered = audio_thread.dc_filter.process_sample(audio_thread.output_buf[i]);

        let wet = dc_filtered * output_gain;
        let dry = input_slice[i];

        output_slice[i] = Parameter::<Blend, Range>::mix(dry as f64, wet, blend) as f32;
    }
}
