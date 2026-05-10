use crate::{
    parameters::{Parameter, Range, Select, calibration_mode::CalibrationMode, cutoff::Cutoff, mix::Mix, xfeed::XFeed},
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
    let xfeed = snapshot.values[Parameter::<XFeed, Range>::ID];
    let mix = snapshot.values[Parameter::<Mix, Range>::ID];
    let calibration_mode = snapshot.values[Parameter::<CalibrationMode, Select>::ID].round() as u8;

    audio_thread.bs2b.update_coeffs(cutoff, xfeed, audio_thread.sample_rate);

    let in_l = unsafe { std::slice::from_raw_parts(in_l, nframes) };
    let in_r = unsafe { std::slice::from_raw_parts(in_r, nframes) };
    let out_l = unsafe { std::slice::from_raw_parts_mut(out_l, nframes) };
    let out_r = unsafe { std::slice::from_raw_parts_mut(out_r, nframes) };

    let half = audio_thread.cal.half_period;

    for i in 0..nframes {
        let (src_l, src_r) = match calibration_mode {
            // Calibration off — pass incoming audio through bs2b
            CalibrationMode::OFF => (in_l[i], in_r[i]),

            // L-only pink noise — direct signal in left ear, crossed LP in right ear.
            // Adjust Cutoff: controls which frequencies appear in the right (crossed) ear.
            // Adjust XFeed: controls how loud the crossed signal is in the right ear.
            CalibrationMode::CONTINUOUS => {
                let n = audio_thread.cal.pink_noise.next();
                (n, 0.0)
            }

            // Intermittent alternating mono pink noise — 500ms L, then 500ms R, repeat.
            // Good for tuning XFeed: hear how much bleed crosses from the active side.
            CalibrationMode::INTERMITTENT => {
                let phase = audio_thread.cal.phase;
                audio_thread.cal.phase = (phase + 1) % (half * 2);
                let n = audio_thread.cal.pink_noise.next();
                if phase < half { (n, 0.0) } else { (0.0, n) }
            }

            _ => (in_l[i], in_r[i]),
        };

        let (wet_l, wet_r) = audio_thread.bs2b.process(src_l, src_r);
        out_l[i] = mix * wet_l + (1.0 - mix) * src_l;
        out_r[i] = mix * wet_r + (1.0 - mix) * src_r;
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
    let xfeed = snapshot.values[Parameter::<XFeed, Range>::ID];
    let mix = snapshot.values[Parameter::<Mix, Range>::ID];
    let calibration_mode = snapshot.values[Parameter::<CalibrationMode, Select>::ID].round() as u8;

    audio_thread.bs2b.update_coeffs(cutoff, xfeed, audio_thread.sample_rate);

    let in_l = unsafe { std::slice::from_raw_parts(in_l, nframes) };
    let in_r = unsafe { std::slice::from_raw_parts(in_r, nframes) };
    let out_l = unsafe { std::slice::from_raw_parts_mut(out_l, nframes) };
    let out_r = unsafe { std::slice::from_raw_parts_mut(out_r, nframes) };

    let half = audio_thread.cal.half_period;

    for i in 0..nframes {
        let (src_l, src_r) = match calibration_mode {
            CalibrationMode::OFF => (in_l[i] as f64, in_r[i] as f64),

            CalibrationMode::CONTINUOUS => {
                let n = audio_thread.cal.pink_noise.next();
                (n, 0.0)
            }

            CalibrationMode::INTERMITTENT => {
                let phase = audio_thread.cal.phase;
                audio_thread.cal.phase = (phase + 1) % (half * 2);
                let n = audio_thread.cal.pink_noise.next();
                if phase < half { (n, 0.0) } else { (0.0, n) }
            }

            _ => (in_l[i] as f64, in_r[i] as f64),
        };

        let (wet_l, wet_r) = audio_thread.bs2b.process(src_l, src_r);
        out_l[i] = (mix * wet_l + (1.0 - mix) * src_l) as f32;
        out_r[i] = (mix * wet_r + (1.0 - mix) * src_r) as f32;
    }
}
