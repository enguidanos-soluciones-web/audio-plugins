use crate::{
    dsp::itd::ItdDelay,
    parameters::{Parameter, Range, Select, angle::Angle, calibration_mode::CalibrationMode, center::Center, cutoff::Cutoff, lrswap::LRSwap, phase::Phase, solo::Solo, xfeed::XFeed},
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
    let calibration_mode = snapshot.values[Parameter::<CalibrationMode, Select>::ID].round() as u8;
    let center_gain = 10f64.powf(snapshot.values[Parameter::<Center, Range>::ID] / 20.0);
    let delay_samples = ItdDelay::angle_to_delay_samples(
        snapshot.values[Parameter::<Angle, Range>::ID],
        audio_thread.sample_rate,
    );
    let lrswap = snapshot.values[Parameter::<LRSwap, Select>::ID].round() as u8;
    let solo   = snapshot.values[Parameter::<Solo,   Select>::ID].round() as u8;
    let phase  = snapshot.values[Parameter::<Phase,  Select>::ID].round() as u8;

    audio_thread.bs2b.update_coeffs(cutoff, xfeed, audio_thread.sample_rate);

    let in_l = unsafe { std::slice::from_raw_parts(in_l, nframes) };
    let in_r = unsafe { std::slice::from_raw_parts(in_r, nframes) };
    let out_l = unsafe { std::slice::from_raw_parts_mut(out_l, nframes) };
    let out_r = unsafe { std::slice::from_raw_parts_mut(out_r, nframes) };

    let half = audio_thread.cal.half_period;

    for i in 0..nframes {
        let (raw_l, raw_r) = match calibration_mode {
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
                let cal_phase = audio_thread.cal.phase;
                audio_thread.cal.phase = (cal_phase + 1) % (half * 2);
                let n = audio_thread.cal.pink_noise.next();
                if cal_phase < half { (n, 0.0) } else { (0.0, n) }
            }

            _ => (in_l[i], in_r[i]),
        };

        // 1. L/R swap
        let (src_l, src_r) = if lrswap == LRSwap::ON { (raw_r, raw_l) } else { (raw_l, raw_r) };

        // 2. Phase inversion
        let src_l = if phase == Phase::L { -src_l } else { src_l };
        let src_r = if phase == Phase::R { -src_r } else { src_r };

        // 3. ITD delay for the crossed path + bs2b
        let (src_l_delayed, src_r_delayed) = audio_thread.itd.process(src_l, src_r, delay_samples);
        let (bs2b_l, bs2b_r) = audio_thread.bs2b.process_with_itd(src_l, src_r, src_l_delayed, src_r_delayed);

        // 4. M/S center attenuation (SPL Phonitor 3 Center knob)
        let mid  = (bs2b_l + bs2b_r) * 0.5 * center_gain;
        let side = (bs2b_l - bs2b_r) * 0.5;
        let wet_l = mid + side;
        let wet_r = mid - side;

        // 5. Solo (post-matrix, copy processed channel to both ears)
        let (wet_l, wet_r) = match solo {
            Solo::L => (wet_l, wet_l),
            Solo::R => (wet_r, wet_r),
            _ => (wet_l, wet_r),
        };

        out_l[i] = wet_l;
        out_r[i] = wet_r;
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
    let calibration_mode = snapshot.values[Parameter::<CalibrationMode, Select>::ID].round() as u8;
    let center_gain = 10f64.powf(snapshot.values[Parameter::<Center, Range>::ID] / 20.0);
    let delay_samples = ItdDelay::angle_to_delay_samples(
        snapshot.values[Parameter::<Angle, Range>::ID],
        audio_thread.sample_rate,
    );
    let lrswap = snapshot.values[Parameter::<LRSwap, Select>::ID].round() as u8;
    let solo   = snapshot.values[Parameter::<Solo,   Select>::ID].round() as u8;
    let phase  = snapshot.values[Parameter::<Phase,  Select>::ID].round() as u8;

    audio_thread.bs2b.update_coeffs(cutoff, xfeed, audio_thread.sample_rate);

    let in_l = unsafe { std::slice::from_raw_parts(in_l, nframes) };
    let in_r = unsafe { std::slice::from_raw_parts(in_r, nframes) };
    let out_l = unsafe { std::slice::from_raw_parts_mut(out_l, nframes) };
    let out_r = unsafe { std::slice::from_raw_parts_mut(out_r, nframes) };

    let half = audio_thread.cal.half_period;

    for i in 0..nframes {
        let (raw_l, raw_r) = match calibration_mode {
            CalibrationMode::OFF => (in_l[i] as f64, in_r[i] as f64),

            CalibrationMode::CONTINUOUS => {
                let n = audio_thread.cal.pink_noise.next();
                (n, 0.0)
            }

            CalibrationMode::INTERMITTENT => {
                let cal_phase = audio_thread.cal.phase;
                audio_thread.cal.phase = (cal_phase + 1) % (half * 2);
                let n = audio_thread.cal.pink_noise.next();
                if cal_phase < half { (n, 0.0) } else { (0.0, n) }
            }

            _ => (in_l[i] as f64, in_r[i] as f64),
        };

        let (src_l, src_r) = if lrswap == LRSwap::ON { (raw_r, raw_l) } else { (raw_l, raw_r) };
        let src_l = if phase == Phase::L { -src_l } else { src_l };
        let src_r = if phase == Phase::R { -src_r } else { src_r };

        let (src_l_delayed, src_r_delayed) = audio_thread.itd.process(src_l, src_r, delay_samples);
        let (bs2b_l, bs2b_r) = audio_thread.bs2b.process_with_itd(src_l, src_r, src_l_delayed, src_r_delayed);

        let mid  = (bs2b_l + bs2b_r) * 0.5 * center_gain;
        let side = (bs2b_l - bs2b_r) * 0.5;
        let wet_l = mid + side;
        let wet_r = mid - side;

        let (wet_l, wet_r) = match solo {
            Solo::L => (wet_l, wet_l),
            Solo::R => (wet_r, wet_r),
            _ => (wet_l, wet_r),
        };

        out_l[i] = wet_l as f32;
        out_r[i] = wet_r as f32;
    }
}
