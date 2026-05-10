use crate::{
    parameters::{Parameter, Range, cutoff::Cutoff, mix::Mix, xfeed::XFeed},
    state::AudioThreadState,
};

pub fn render_audio_f64(
    audio_thread: &mut AudioThreadState,
    in_l: *const f64,
    in_r: *const f64,
    out_l: *mut f64,
    out_r: *mut f64,
    nframes: usize,
) {
    audio_thread.assert_audio_thread();

    let snapshot = audio_thread.param_snapshot.load();
    let cutoff = snapshot.values[Parameter::<Cutoff, Range>::ID];
    let feed = snapshot.values[Parameter::<XFeed, Range>::ID];
    let mix = snapshot.values[Parameter::<Mix, Range>::ID];

    audio_thread.bs2b.update_coeffs(cutoff, feed, audio_thread.sample_rate);

    let in_l = unsafe { std::slice::from_raw_parts(in_l, nframes) };
    let in_r = unsafe { std::slice::from_raw_parts(in_r, nframes) };
    let out_l = unsafe { std::slice::from_raw_parts_mut(out_l, nframes) };
    let out_r = unsafe { std::slice::from_raw_parts_mut(out_r, nframes) };

    for i in 0..nframes {
        let (wet_l, wet_r) = audio_thread.bs2b.process(in_l[i], in_r[i]);
        out_l[i] = mix * wet_l + (1.0 - mix) * in_l[i];
        out_r[i] = mix * wet_r + (1.0 - mix) * in_r[i];
    }
}

pub fn render_audio_f32(
    audio_thread: &mut AudioThreadState,
    in_l: *const f32,
    in_r: *const f32,
    out_l: *mut f32,
    out_r: *mut f32,
    nframes: usize,
) {
    audio_thread.assert_audio_thread();

    let snapshot = audio_thread.param_snapshot.load();
    let cutoff = snapshot.values[Parameter::<Cutoff, Range>::ID];
    let feed = snapshot.values[Parameter::<XFeed, Range>::ID];
    let mix = snapshot.values[Parameter::<Mix, Range>::ID];

    audio_thread.bs2b.update_coeffs(cutoff, feed, audio_thread.sample_rate);

    let in_l = unsafe { std::slice::from_raw_parts(in_l, nframes) };
    let in_r = unsafe { std::slice::from_raw_parts(in_r, nframes) };
    let out_l = unsafe { std::slice::from_raw_parts_mut(out_l, nframes) };
    let out_r = unsafe { std::slice::from_raw_parts_mut(out_r, nframes) };

    for i in 0..nframes {
        let (wet_l, wet_r) = audio_thread.bs2b.process(in_l[i] as f64, in_r[i] as f64);
        out_l[i] = (mix * wet_l + (1.0 - mix) * in_l[i] as f64) as f32;
        out_r[i] = (mix * wet_r + (1.0 - mix) * in_r[i] as f64) as f32;
    }
}
